# 🎹 Piano Lifetime Logger

A single-file, browser-based piano MIDI visualizer and lifetime key logger. Plug in a MIDI keyboard, play with your computer keyboard, or click the on-screen piano — every note you play is saved permanently in your own browser, with rich statistics, charts, and practice tools to help you grow as a player.

**Live demo:** [https://kosandra0.github.io/Piano_Logger/](https://kosandra0.github.io/Piano_Logger/)

**Windows app:** [Download Piano-Logger-Setup.exe](https://github.com/kosandra0/Piano_Logger/releases/latest/download/Piano-Logger-Setup.exe) (Windows 10 & 11, 64-bit)

**Current version:** 7.0 — Aurora Glass Studio. See [CHANGELOG.md](CHANGELOG.md) for everything that changed.

## ✨ Features

- **Full 88-key piano** with falling notes, glowing key animations, and multiple built-in sound engines (synth, grand piano samples, electric piano, organ, strings, music box)
- **Three ways to play:** a real MIDI keyboard (via Web MIDI), your computer keyboard (two playable rows, remappable), or click/tap the on-screen keys
- **Lifetime logging** — every key press is saved forever in your browser's IndexedDB, including note, velocity, duration, timestamp, and input source
- **Dashboards & analytics** — an overview of your connection and progress, lifetime stats, per-key breakdowns, speed graphs, dynamics and velocity analysis, hand-balance insights, and a "Year in Review" shareable image
- **Chord & key detection** — live chord recognition, chord progression tracking (in both chord names and Roman numerals), and session-by-session key detection
- **Practice mode** — load any `.mid` file or built-in demo song, choose a hand, pick a mode (Play Along / Wait for Me / Watch & Listen), and get a real-time accuracy score
- **Ear training** — interval and chord recognition games with accuracy tracking
- **Recorder & MIDI export** — record your playing, scrub/trim/loop a take, and export it as a standard `.mid` file
- **Metronome with timing analysis** — see exactly how early or late you are, with visual feedback
- **Sound routing** — send everything you play to an external synth or VST host via a virtual MIDI cable (e.g. loopMIDI + Keyzone Classic)
- **MIDI device centre** — connection status, every input and output, listen to one keyboard or all, permissions
- **Full data ownership** — export your entire log as CSV or JSON, create backups (which also carry your settings, themes and profile), restore from backup, or auto-backup to a local folder
- **Theme Studio** — 182 themes (110 new design languages in 11 collections plus 72 classic ones) with live preview, search, favourites, your own accent colour and dials for glass, blur, glow and motion
- **Deep customization** — custom accent colors, background images, compact/high-contrast/calm modes, motion levels (Off / Reduced / Balanced / Expressive), draggable and collapsible sections, and a command palette (`Ctrl+K`) for quick access to every feature

## 🔒 Privacy

This app has **no server and no accounts**. All data — your key presses, recordings, settings, and practice scores — is stored only in your own browser (via IndexedDB and localStorage), on your own device. Nothing is uploaded anywhere. If you clear your browser data, your log history is lost unless you've made a backup, so regular backups (Export & Backup tab) are recommended.

## 🚀 How to use

1. Open the [live link](https://kosandra0.github.io/Piano_Logger/) in a modern desktop browser — **Chrome, Edge, Brave, or Opera** work best (Safari does not support Web MIDI).
2. Click anywhere on the page once to enable sound.
3. Plug in a MIDI keyboard and press **Connect MIDI**, or tick **Play with PC keyboard** to use your computer keys, or just click the on-screen piano.
4. Explore the navigation — Overview, History, Analysis, Studio, Practice and the Theme Studio — to see your stats and tools grow as you play.
5. Click the **?** button in the top right anytime for a full in-app guide, or press **Ctrl+K** to search any action.

## 🖥 Windows app

The Windows app shows the same page in its own window and adds: your own SoundFont (.sf2) instruments, automatic daily backups into `Documents\Piano Logger Backups`, always on top, start with Windows, sound that keeps playing when minimised, and no MIDI permission pop-ups.

- Download `Piano-Logger-Setup.exe` from the [latest release](https://github.com/kosandra0/Piano_Logger/releases/latest) and run it. Windows may say "Windows protected your PC" because the app is not signed by a company: click **More info → Run anyway**.
- The app keeps its own data. To bring your history (and settings) from the website or an older app: **Data → Create backup file** there, then **Data → Restore** in the new app.
- Version 7.0 is built with [Tauri 2](https://tauri.app/) (source in [`Piano-Logger-tauri/`](Piano-Logger-tauri/), about 2 MB). It installs next to the older Electron app (source in [`desktop/`](desktop/)), it does not replace it.

## 💻 Running it locally

Since this is a single self-contained HTML file, you can also download `index.html` and open it directly in your browser, or serve it locally for a more stable MIDI-permission experience:

```bash
python -m http.server 8000
```

Then visit `http://localhost:8000/index.html`.

## 📄 License

Free to use, modify, and share.
