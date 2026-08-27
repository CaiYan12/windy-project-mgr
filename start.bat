@echo off
rem ============================================================
rem  Windy Project Manager - launch entry (dev convenience)
rem  Manual start/stop: run it, then press Ctrl+C or close the
rem  window to stop. Nothing is auto-started or kept resident.
rem
rem  Usage:
rem    start.bat         start dev environment (Vite dev server
rem                      + Tauri app window)   = pnpm tauri dev
rem    start.bat build   production build       = pnpm tauri build
rem ============================================================
setlocal
cd /d "%~dp0"

if /I "%~1"=="" goto dev
if /I "%~1"=="build" goto build
if /I "%~1"=="dev" goto dev
goto usage

:usage
echo Usage: start.bat [dev^|build]
echo   (no arg)   start dev environment
echo   build      production build
exit /b 1

:dev
echo [windy] Starting dev environment (Vite dev server + Tauri app)...
echo [windy] Stop it with Ctrl+C in this window.
pnpm tauri dev
exit /b %errorlevel%

:build
echo [windy] Building production bundle...
pnpm tauri build
exit /b %errorlevel%