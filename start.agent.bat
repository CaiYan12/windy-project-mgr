@echo off
rem ============================================================
rem  Windy Project Manager - agent launcher (Tauri CLI workaround)
rem
rem  Why: under the LobsterAI runtime, `node` resolves to LobsterAI.exe,
rem  so the Tauri CLI cannot detect its runtime from argv[0] and injects
rem  the exe path as a subcommand:
rem    error: unrecognized subcommand 'D:\Program Files\LobsterAI\LobsterAI.exe'
rem  This wrapper always runs tauri.js with a real Node.js binary.
rem
rem  Usage:
rem    start.agent.bat         start dev environment (Vite + Tauri)
rem    start.agent.bat build   production build
rem
rem  Stop with Ctrl+C in this window. Nothing is left resident.
rem ============================================================
setlocal
cd /d "%~dp0"

set "TAURI_JS=node_modules\@tauri-apps\cli\tauri.js"
if not exist "%TAURI_JS%" set "TAURI_JS=node_modules\.pnpm\@tauri-apps+cli@2.11.4\node_modules\@tauri-apps\cli\tauri.js"
if not exist "%TAURI_JS%" (
  echo [windy] Tauri CLI entry not found: %TAURI_JS%
  echo [windy] Run "pnpm install" first.
  exit /b 1
)

set "NODE_EXE="
if exist "C:\Program Files\nodejs\node.exe" set "NODE_EXE=C:\Program Files\nodejs\node.exe"
if not defined NODE_EXE (
  for /f "delims=" %%I in ('where node 2^>nul') do (
    echo %%I | findstr /I /V "LobsterAI" >nul && (
      set "NODE_EXE=%%I"
      goto :have_node
    )
  )
)
:have_node
if not defined NODE_EXE (
  echo [windy] No real Node.js found. Install Node.js or set NODE_EXE manually.
  exit /b 1
)

set "ACTION=%~1"
if /I "%ACTION%"=="" set "ACTION=dev"
if /I "%ACTION%"=="dev" goto dev
if /I "%ACTION%"=="build" goto build

echo Usage: start.agent.bat [dev^|build]
exit /b 1

:dev
echo [windy] dev ^(node: "%NODE_EXE%"^)
"%NODE_EXE%" "%TAURI_JS%" dev
exit /b %errorlevel%

:build
echo [windy] build ^(node: "%NODE_EXE%"^)
"%NODE_EXE%" "%TAURI_JS%" build
exit /b %errorlevel%
