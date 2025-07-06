use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, Write},
    path::Path,
    process::{Command, Stdio},
};

use colored::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rustyline::{error::ReadlineError, Editor};
use serde::{Deserialize, Serialize};
use rustyline::history::DefaultHistory;


const DATA_FILE: &str = "data.json";

#[derive(Serialize, Deserialize, Debug, Default)]
struct Memory {
    username: Option<String>,
    knowledge: HashMap<String, String>,
}

impl Memory {
    fn load() -> Self {
        if Path::new(DATA_FILE).exists() {
            fs::read_to_string(DATA_FILE)
                .ok()
                .and_then(|c| serde_json::from_str(&c).ok())
                .unwrap_or_default()
        } else {
            Memory::default()
        }
    }

    fn save(&self) {
        let json = serde_json::to_string_pretty(self).expect("Serialize failed");
        fs::write(DATA_FILE, json).expect("Write failed");
    }

    fn pretty_print(&self) {
        println!("{}", "\n— Current Knowledge —".green().bold());
        for (k, v) in &self.knowledge {
            println!("{} ➞ {}", k.yellow().bold(), v.cyan());
        }
    }

    fn reset(&mut self) {
        self.username = None;
        self.knowledge.clear();
        self.save();
        println!("{}", "⚠️  Memory reset complete.".red().bold());
    }

    fn teach_from_file(&mut self, file: &str) {
        if let Ok(f) = fs::File::open(file) {
            let reader = io::BufReader::new(f);
            for line in reader.lines().flatten() {
                if let Some((k, v)) = line.split_once('=') {
                    self.knowledge.insert(k.trim().to_string(), v.trim().to_string());
                }
            }
            self.save();
            println!(
                "{} entries loaded from {}",
                self.knowledge.len().to_string().green().bold(),
                file.cyan()
            );
        } else {
            println!("{}", format!("❌ Could not open file: {}", file).red().bold());
        }
    }
}



fn speak(text: &str) {
    if cfg!(target_os = "linux") {
        let espeak = Command::new("espeak")
            .arg(text)
            .arg("--stdout")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to run espeak");

        let _ = Command::new("aplay")
            .stdin(espeak.stdout.unwrap())
             .stdout(Stdio::null()) // silence "Playing WAVE..."
             .stderr(Stdio::null())
            .spawn();
    } else if cfg!(target_os = "macos") {
        let _ = Command::new("say").arg(text).spawn();
    } else if cfg!(target_os = "windows") {
        let _ = Command::new("PowerShell")
            .args([
                "-Command",
                &format!(
                    "Add-Type -AssemblyName System.Speech; \
                     (New-Object System.Speech.Synthesis.SpeechSynthesizer).Speak('{}');",
                    text
                ),
            ])
            .spawn();
    }
}



fn export_csv(memory: &Memory, filename: &str) {
    let mut wtr = csv::Writer::from_path(filename).expect("Failed to open CSV");
    for (k, v) in &memory.knowledge {
        wtr.write_record(&[k, v]).unwrap();
    }
    wtr.flush().unwrap();
    println!("✅ Exported to {}", filename.green());
}

fn import_csv(memory: &mut Memory, filename: &str) {
    let mut rdr = csv::Reader::from_path(filename).expect("Could not open file");
    for result in rdr.records() {
        if let Ok(record) = result {
            if record.len() >= 2 {
                memory
                    .knowledge
                    .insert(record[0].to_string(), record[1].to_string());
            }
        }
    }
    memory.save();
    println!("✅ Imported from {}", filename.green());
}

fn export_markdown(memory: &Memory, filename: &str) {
    let mut output = String::new();
    output.push_str("# Knowledge Base\n\n");
    for (k, v) in &memory.knowledge {
        output.push_str(&format!("- **{}** → {}\n", k, v));
    }
    fs::write(filename, output).expect("Failed to write markdown");
    println!("✅ Exported to {}", filename.green());
}

fn main() {
    let mut rl = Editor::<(), DefaultHistory>::new().expect("Failed to init readline");
    let mut memory = Memory::load();

    if let Some(name) = &memory.username {
        println!("{} {}", "👋 Welcome back,".green(), name.bold().cyan());
    } else {
        println!("{}", "Hello! What's your name?".yellow());
        let name = read_line();
        memory.username = Some(name.trim().to_string());
        memory.save();
    }

    println!("\n{}", "✨ Available Commands:".blue().bold());
    println!("  {} → {}", "teaching mode".cyan().underline(), "Teach me manually");
    println!("  {} → {}", "teach_file <filename>".cyan().underline(), "Teach from file (key=value)");
    println!("  {} → {}", "show".cyan().underline(), "Show learned data");
    println!("  {} → {}", "reset".cyan().underline(), "Forget everything");
    println!("  {} → {}", "export_csv file.csv".cyan(), "Export to CSV");
    println!("  {} → {}", "import_csv file.csv".cyan(), "Import from CSV");
    println!("  {} → {}", "export_md file.md".cyan(), "Export to Markdown");
    println!("  {} → {}", "exit".cyan().underline(), "Exit the assistant");
    println!("Plus: enter math like {}", "2 + 5 * 3".magenta());

    let matcher = SkimMatcherV2::default();

    loop {
        let readline = rl.readline(&format!("{}", "> ".bold().blue()));
        match readline {
            Ok(line) => {
                let input = line.trim();
                let _ = rl.add_history_entry(input);

                match input {
                    "exit" => break,
                    "teaching mode" => enter_teaching_mode(&mut memory),
                    s if s.starts_with("teach_file ") => {
                        let f = s["teach_file ".len()..].trim();
                        memory.teach_from_file(f);
                    }
                    "show" => memory.pretty_print(),
                    "reset" => memory.reset(),
                    s if s.starts_with("export_csv ") => {
                        let f = s["export_csv ".len()..].trim();
                        export_csv(&memory, f);
                    }
                    s if s.starts_with("import_csv ") => {
                        let f = s["import_csv ".len()..].trim();
                        import_csv(&mut memory, f);
                    }
                    s if s.starts_with("export_md ") => {
                        let f = s["export_md ".len()..].trim();
                        export_markdown(&memory, f);
                    }
                    other => {
                        if let Ok(res) = meval::eval_str(other) {
                            println!("🧮 {} {}", "Answer:".green().bold(), res.to_string().yellow());
                        } else if let Some(resp) = fuzzy_lookup(&memory.knowledge, other, &matcher)
                        {
                            println!("🤖 {} {}", "Response:".cyan(), resp);
                            speak(resp);
                        } else {
                            println!("{}", "🤷 I don't know about that yet.".red());
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) | Err(_) => break,
        }
    }

    memory.save();
    println!("{}", "👋 Goodbye!".purple().bold());
}

fn enter_teaching_mode(memory: &mut Memory) {
    println!("{}", "🧠 Teaching mode. Type 'done' to exit.".green().bold());
    loop {
        print!("{}", "Key: ".yellow().bold());
        io::stdout().flush().unwrap();
        let key = read_line().trim().to_string();
        if key.eq_ignore_ascii_case("done") {
            break;
        }

        print!("{}", "Value: ".cyan().bold());
        io::stdout().flush().unwrap();
        let value = read_line().trim().to_string();
        if value.eq_ignore_ascii_case("done") {
            break;
        }

        memory.knowledge.insert(key.clone(), value.clone());
        memory.save();
        println!("✅ Learned: {} → {}", key.magenta(), value.green());
    }

    println!("{}", "✅ Exiting teaching mode.".green());
}

fn fuzzy_lookup<'a>(
    map: &'a HashMap<String, String>,
    q: &str,
    matcher: &SkimMatcherV2,
) -> Option<&'a String> {
    map.keys()
        .filter_map(|k| matcher.fuzzy_match(k, q).map(|s| (s, k)))
        .max_by_key(|(score, _)| *score)
        .map(|(_, best_key)| &map[best_key])
}

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).unwrap();
    s
}
