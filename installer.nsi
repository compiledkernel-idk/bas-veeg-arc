; NSIS Installer Script for Bas Veeg Arc
; Requires NSIS (Nullsoft Scriptable Install System)
; Download from: https://nsis.sourceforge.io/

;--------------------------------
; Includes
!include "MUI2.nsh"
!include "FileFunc.nsh"

;--------------------------------
; General Configuration
Name "Bas Veeg Arc"
OutFile "Bas_Veeg_Arc_Installer.exe"
Unicode True

; Default installation directory
InstallDir "$PROGRAMFILES64\Bas Veeg Arc"

; Get installation folder from registry if available
InstallDirRegKey HKLM "Software\BasVeegArc" "Install_Dir"

; Request application privileges for Windows Vista and higher
RequestExecutionLevel admin

;--------------------------------
; Interface Settings
!define MUI_ABORTWARNING
!define MUI_ICON "app.ico"
!define MUI_UNICON "app.ico"
!define MUI_WELCOMEFINISHPAGE_BITMAP "app.ico"

;--------------------------------
; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

;--------------------------------
; Languages
!insertmacro MUI_LANGUAGE "English"

;--------------------------------
; Version Information
VIProductVersion "1.0.0.0"
VIAddVersionKey "ProductName" "Bas Veeg Arc"
VIAddVersionKey "FileDescription" "Bas Veeg Arc - School Fighting Game"
VIAddVersionKey "FileVersion" "1.0.0"
VIAddVersionKey "ProductVersion" "1.0.0"
VIAddVersionKey "CompanyName" "BAS VEEG ARC"
VIAddVersionKey "LegalCopyright" "Copyright © 2025"

;--------------------------------
; Installer Sections

Section "Bas Veeg Arc" SecMain
    SectionIn RO ; Read-only - always installed

    ; Set output path to the installation directory
    SetOutPath "$INSTDIR"

    ; Copy files
    File "dist\Bas Veeg Arc.exe"
    File "app.ico"

    ; Store installation folder
    WriteRegStr HKLM "Software\BasVeegArc" "Install_Dir" "$INSTDIR"

    ; Create uninstaller
    WriteUninstaller "$INSTDIR\Uninstall.exe"

    ; Create start menu shortcuts
    CreateDirectory "$SMPROGRAMS\Bas Veeg Arc"
    CreateShortcut "$SMPROGRAMS\Bas Veeg Arc\Bas Veeg Arc.lnk" "$INSTDIR\Bas Veeg Arc.exe" "" "$INSTDIR\app.ico" 0
    CreateShortcut "$SMPROGRAMS\Bas Veeg Arc\Uninstall.lnk" "$INSTDIR\Uninstall.exe"

    ; Create desktop shortcut
    CreateShortcut "$DESKTOP\Bas Veeg Arc.lnk" "$INSTDIR\Bas Veeg Arc.exe" "" "$INSTDIR\app.ico" 0

    ; Add to Add/Remove Programs
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "DisplayName" "Bas Veeg Arc"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "QuietUninstallString" "$\"$INSTDIR\Uninstall.exe$\" /S"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "InstallLocation" "$INSTDIR"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "DisplayIcon" "$INSTDIR\app.ico"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "Publisher" "BAS VEEG ARC"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "DisplayVersion" "1.0.0"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "NoRepair" 1

    ; Calculate installed size
    ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
    IntFmt $0 "0x%08X" $0
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc" "EstimatedSize" "$0"
SectionEnd

;--------------------------------
; Uninstaller Section

Section "Uninstall"
    ; Remove files
    Delete "$INSTDIR\Bas Veeg Arc.exe"
    Delete "$INSTDIR\app.ico"
    Delete "$INSTDIR\Uninstall.exe"

    ; Remove shortcuts
    Delete "$SMPROGRAMS\Bas Veeg Arc\*.*"
    Delete "$DESKTOP\Bas Veeg Arc.lnk"
    RMDir "$SMPROGRAMS\Bas Veeg Arc"

    ; Remove installation directory
    RMDir "$INSTDIR"

    ; Remove registry keys
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\BasVeegArc"
    DeleteRegKey HKLM "Software\BasVeegArc"
SectionEnd
