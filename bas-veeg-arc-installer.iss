[Setup]
AppId={{A5B3C2D1-E4F5-6789-ABCD-EF0123456789}}
AppName=Bas Veeg Arc
AppVersion=1.0.0
AppPublisher=Dutch School Games
AppPublisherURL=https://example.com
AppSupportURL=https://example.com/support
AppUpdatesURL=https://example.com/updates
DefaultDirName={autopf64}\BasVeegArc
DefaultGroupName=Bas Veeg Arc
AllowNoIcons=yes
LicenseFile=..\LICENSE
OutputDir=..\dist\windows
OutputBaseFilename=bas-veeg-arc-setup
SetupIconFile=..\dist\windows\icons\installer.ico
ArchitecturesInstallIn64BitMode=x64
ArchitecturesAllowed=x64
UninstallDisplayIcon={app}\bas-veeg-arc.exe
Compression=lzma
SolidCompression=yes
WizardStyle=modern
UsePreviousAppDir=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\dist\windows\bas-veeg-arc.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\dist\windows\assets\*"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\LICENSE"; DestName: "LICENSE.txt"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Bas Veeg Arc"; Filename: "{app}\bas-veeg-arc.exe"; IconFilename: "{app}\assets\icons\app.ico"; Check: FileExists(ExpandConstant('{app}\assets\icons\app.ico'))
Name: "{group}\{cm:UninstallProgram,Bas Veeg Arc}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Bas Veeg Arc"; Filename: "{app}\bas-veeg-arc.exe"; IconFilename: "{app}\assets\icons\app.ico"; Tasks: desktopicon; Check: FileExists(ExpandConstant('{app}\assets\icons\app.ico'))

[Run]
Filename: "{app}\bas-veeg-arc.exe"; Description: "{cm:LaunchProgram,Bas Veeg Arc}"; Flags: nowait postinstall skipifsilent
