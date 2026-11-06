@echo off
setlocal enabledelayedexpansion
title DirectOffice - Setup and Launcher

echo ===============================================================================
echo                      DIRECTOFFICE - SETUP & LAUNCHER
echo ===============================================================================
echo.

:: 1. Check prerequisites
echo [1/4] Checking prerequisites...
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Rust / Cargo not found in PATH. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

where npm >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Node.js / npm not found in PATH. Please install Node.js from https://nodejs.org/
    pause
    exit /b 1
)
echo [OK] Rust and Node.js detected.
echo.

:: 2. Setup Desktop Frontend
echo [2/4] Setting up Desktop Frontend dependencies...
cd /d "%~dp0Desktop\Frontend"
if not exist "node_modules" (
    echo Installing npm dependencies...
    call npm install
    if %ERRORLEVEL% NEQ 0 (
        echo [ERROR] npm install failed.
        pause
        exit /b 1
    )
) else (
    echo [OK] node_modules found.
)
echo.

:: 3. Interactive Menu
:MENU
cls
echo ===============================================================================
echo                      DIRECTOFFICE - LAUNCH MENU
echo ===============================================================================
echo.
echo  [1] Start Server Backend (Rust / Axum on port 8080)
echo  [2] Start Desktop App in Development Mode (Hot Reload)
echo  [3] Build Release Desktop Standalone (.exe)
echo  [4] Run Automated Unit Tests (Server + Auth)
echo  [5] Launch Both Server and Desktop App
echo  [0] Exit
echo.
set /p CHOICE="Select option [0-5]: "

if "%CHOICE%"=="1" goto START_SERVER
if "%CHOICE%"=="2" goto START_DESKTOP_DEV
if "%CHOICE%"=="3" goto BUILD_RELEASE
if "%CHOICE%"=="4" goto RUN_TESTS
if "%CHOICE%"=="5" goto START_BOTH
if "%CHOICE%"=="0" exit /b 0

echo Invalid choice. Try again.
timeout /t 2 >nul
goto MENU

:START_SERVER
cls
echo Starting DirectOffice Server Backend...
cd /d "%~dp0Server\Backend"
cargo run
pause
goto MENU

:START_DESKTOP_DEV
cls
echo Starting DirectOffice Desktop in Development Mode...
cd /d "%~dp0Desktop\Frontend"
call npm run tauri dev
pause
goto MENU

:BUILD_RELEASE
cls
echo Building standalone release desktop application...
cd /d "%~dp0Desktop\Frontend"
call npm run tauri build
if %ERRORLEVEL% EQU 0 (
    echo.
    echo [SUCCESS] Release build complete!
    echo Binary located at: %~dp0target\release\
) else (
    echo.
    echo [ERROR] Build failed.
)
pause
goto MENU

:RUN_TESTS
cls
echo Running Server unit tests...
cd /d "%~dp0Server\Backend"
cargo test -- --nocapture
echo.
echo Checking Desktop Backend...
cd /d "%~dp0Desktop\Backend"
cargo check
pause
goto MENU

:START_BOTH
cls
echo Launching Server Backend in separate window...
start "DirectOffice - Server Backend" cmd /k "cd /d "%~dp0Server\Backend" && cargo run"

timeout /t 3 >nul

echo Launching Desktop App in separate window...
start "DirectOffice - Desktop App" cmd /k "cd /d "%~dp0Desktop\Frontend" && npm run tauri dev"

echo Both processes launched.
pause
goto MENU

