@echo off
setlocal
cd /d "%~dp0"

where pwsh.exe >nul 2>nul
if errorlevel 1 (
    echo [build] PowerShell 7 ^(pwsh.exe^) is required.
    exit /b 1
)

pwsh.exe -NoLogo -NoProfile -NonInteractive -File "%~dp0build.ps1"
exit /b %errorlevel%
