PIANO LOGGER - WINDOWS APP (source)
===================================

This folder turns the Piano Logger page into a Windows app with extra features:
SoundFont (.sf2) instruments, daily automatic backups, always on top, start with
Windows, sound that keeps playing when minimised, no MIDI pop-ups, offline use,
and updates fetched from https://kosandra0.github.io/Piano_Logger/

What is inside
  main.js              the app itself (window, files, backups, updates)
  preload.js           the few functions the page is allowed to use
  page/                the Piano Logger page that the app shows
  build/icon.png       the app icon
  package.json         name, version and installer settings
  BUILD-INSTALLER.bat  builds the installer on your PC

Make the installer - EASIEST: let GitHub build it (nothing to install)
  1. Upload the "desktop" folder and the ".github" folder to the top of your
     Piano_Logger repository (GitHub -> Add file -> Upload files, drag both folders in,
     Commit changes).
  2. Open the repository's "Actions" tab -> "Build Windows app" -> "Run workflow".
  3. After about 5-10 minutes the installer is published under "Releases" as
     Piano-Logger-Setup.exe. The website's "Download app" button downloads it from there.
  To publish a new app version later: change "version" in desktop/package.json
  (for example 6.0.1) and run the workflow again.

Make the installer - on your own PC
  1. Install Node.js (LTS) from https://nodejs.org
  2. Double-click BUILD-INSTALLER.bat. When it finishes, the installer is in
     dist\Piano-Logger-Setup.exe. Upload it to a GitHub release with exactly that name.

Updating the page inside the app
  The app checks the website each time it starts and downloads a newer page by itself
  (it compares APP_VERSION). So after you update the website, people get it on their
  next restart, without a new installer. When building, the newest index.html from the
  repository is put inside the app automatically.

Where the app keeps things (on the user's PC)
  Data and settings:   %APPDATA%\Piano Logger
  SoundFonts:          %APPDATA%\Piano Logger\SoundFonts
  Daily backups:       Documents\Piano Logger Backups
