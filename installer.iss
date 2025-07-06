[Setup]
AppName=Minimal AI
AppVersion=0.1
DefaultDirName={pf}\Minimal AI
DefaultGroupName=Minimal AI
OutputBaseFilename=MinimalAIInstaller
Compression=lzma
SolidCompression=yes

[Files]
Source: "target\release\minimal_ai.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Minimal AI";