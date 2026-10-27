@echo off
title Build the Piano Logger installer
cd /d "%~dp0"
echo.
echo  Building Piano-Logger-Setup.exe (Windows 10 and 11, 64-bit and 32-bit)
echo  This needs Node.js and an internet connection, and takes a few minutes.
echo.
where npm >nul 2>nul
if errorlevel 1 (
  echo  Node.js is not installed. Opening https://nodejs.org - install the LTS version,
  echo  then double-click this file again.
  start "" https://nodejs.org
  pause
  exit /b 1
)
if exist "..\index.html" (
  echo  Using the newest page from ..\index.html
  copy /y "..\index.html" "page\piano-logger.html" >nul
)
call npm install
if errorlevel 1 goto failed
call npm run dist
if errorlevel 1 goto failed
echo.
echo  Done! Your installer is dist\Piano-Logger-Setup.exe
explorer dist
pause
exit /b 0
:failed
echo.
echo  Something went wrong - see the messages above.
pause
exit /b 1
