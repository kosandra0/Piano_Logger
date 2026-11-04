# Piano Logger — Windows app (Tauri 2)

This folder turns the Piano Logger page into a Windows app with **Tauri 2** (Rust + the WebView2 that ships with Windows).
It is a newer, much smaller replacement for the Electron app in `../Piano-Logger-desktop-app` and `../Piano-Logger-app-6.1-files`,
which are left unchanged (as is the existing `../Piano-Logger-Setup.exe`).

The website does not need anything from here: `piano-logger.html` works on its own in a browser.
The app shows that same page and adds the extras through `window.pianoApp` (see `src-tauri/src/bridge.js`),
the same API the Electron app's `preload.js` provides, so the page code is identical in all three places.

## What is inside

| Path | What it is |
|---|---|
| `package.json` | version, and the `dev` / `build` scripts (Tauri CLI 2.12) |
| `scripts/copy-page.mjs` | copies the newest page (`../index.html` or `../piano-logger.html`) to `web/index.html` before every build |
| `web/` | the page inside the app (generated; do not edit) |
| `src-tauri/tauri.conf.json` | name, version 7.4.1, identifier, installer settings |
| `src-tauri/capabilities/default.json` | no plugin permissions are given to the page (least privilege) |
| `src-tauri/src/lib.rs` | the app: window, page serving and updates, SoundFonts, backups, always on top, start with Windows, MIDI + microphone permission, system tray |
| `src-tauri/src/bridge.js` | `window.pianoApp` for the page |
| `src-tauri/src/bridge.rs` | MIDI Keyboard Bridge: key presses with `SendInput`, target-window check, Ctrl+Alt+End hotkey, `bridge_focus_target` (Go to Roblox) |
| `.github/workflows/build-windows-tauri.yml` | builds the installer on GitHub and publishes it as `Piano-Logger-Setup.exe` |

## Build it on your PC

Needs Node.js (LTS), Rust (`rustup`, stable, MSVC) and the "Desktop development with C++" Build Tools.

```
npm install
npm run build        # web page copied in, release build, NSIS installer
```

The installer is written to `src-tauri/target/release/bundle/nsis/Piano Logger_7.4.1_x64-setup.exe`,
and the app itself to `src-tauri/target/release/piano-logger.exe`. `npm run dev` starts a debug build.

## How it behaves

- **Page**: served from the app (`http://pianologger.localhost/` inside WebView2). When it starts, the app looks at the website
  (https://kosandra0.github.io/Piano_Logger/) and keeps a newer page for the next start, exactly like the Electron app.
  It is not an installer updater: a new app version still needs a new installer.
- **MIDI**: the app allows Web MIDI (and downloads, the backup-folder picker, clipboard and notifications) for its own page
  without pop-ups. Since 7.3.0 the **microphone** is also allowed for the app's own page (Voice Studio only opens it after Start);
  camera and location are not allowed automatically. The CSP allows `blob:` scripts so Voice Studio's AudioWorklet module can load.
- **Security**: the page is served with a Content-Security-Policy (no frames, plugins or unknown script origins);
  websites open in your normal browser; the page's commands only reach the app's own folders and check every file name.
- **Start with Windows**: a value named `com.kosandra0.pianologger.desktop` in the current user's Run key only (never machine-wide), separate from the Electron app's entry.
- **Window**: no Windows frame; the page draws a glass title bar with close, minimize and maximize/restore controls (bridge.js `window`). Drag it to move, double-click to maximize; Tauri keeps every edge resizable.
- **Drag and drop**: since 7.3.2 Tauri's own file-drop handler is turned off (`disable_drag_drop_handler`), so the page's
  HTML5 drag and drop works on Windows (moving sections by their ⠿ handle). The app never used Tauri drop events.
- **System tray** (7.2): if "close to tray" is on, closing the window hides it and keeps logging in the tray (no graphics,
  WebView2 told to use less memory via `webview2-com`); the MIDI Bridge does not press keys there. Click the tray icon or start
  Piano Logger again to bring the window back.
- **Background**: sound, the visualizer and logging keep running while the window is minimised (background throttling off).
- **MIDI Keyboard Bridge** (7.1, `src-tauri/src/bridge.rs`, bridge.js `keys`): the page decides which keys go down and up;
  the app presses them with `SendInput` (scan codes) only when the target window (program name and/or title) is in front and
  is not Piano Logger. It remembers every key it holds and releases them all on Emergency stop, the global **Ctrl+Alt+End**
  hotkey, closing the window and exit. No game memory, injection or anti-cheat tricks — ordinary key presses only.
- **Where things are kept**
  - data and settings: `%APPDATA%\com.kosandra0.pianologger` (SoundFonts in `SoundFonts\`)
  - the browser storage with your key-press log: `%LOCALAPPDATA%\com.kosandra0.pianologger\EBWebView`
  - daily backups: `Documents\Piano Logger Backups` (the same folder as the Electron app; the newest 30 are kept)
- **Bringing your history from the Electron app or the website**: the Tauri app has its own storage. On the website (or in the
  Electron app) use *Data → Create backup file*, then in this app *Data → Restore*.

## Differences from the Electron app

- 64-bit Windows only (the Electron installer also contained a 32-bit build).
- If WebView2 is missing (rare on Windows 10/11), the installer downloads it.
- The window does not reload itself after a WebView2 crash; reopen the app.
