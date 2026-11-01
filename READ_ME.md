# 🎹 Piano Lifetime Logger

A single-file, browser-based piano MIDI visualizer and lifetime key logger. Plug in a MIDI keyboard, play with your computer keyboard, or click the on-screen piano — every note you play is saved permanently on your own device, with rich statistics, charts, and practice tools to help you grow as a player.

**Live demo:** [https://kosandra0.github.io/Piano_Logger/](https://kosandra0.github.io/Piano_Logger/)

**Windows app:** [Download Piano-Logger-Setup.exe](https://github.com/kosandra0/Piano_Logger/releases/latest/download/Piano-Logger-Setup.exe) (Windows 10 & 11, 64-bit)

**Current version:** 7.2.2 — MIDI Keyboard Bridge (play Roblox with your MIDI keyboard), system tray, 26 calm roll backgrounds, glass menus. See [CHANGELOG.md](CHANGELOG.md) for everything that changed.

## ✨ Features

- **Full 88-key piano** with falling notes, glowing key animations, and multiple built-in sound engines (synth, grand piano samples, electric piano, organ, strings, music box)
- **Three ways to play:** a real MIDI keyboard (via Web MIDI), your computer keyboard (two playable rows, remappable), or click/tap the on-screen keys
- **Lifetime logging** — every key press is saved forever on your device, including note, velocity, duration, timestamp, and input source
- **🎮 MIDI Keyboard Bridge** *(Windows app)* — play Roblox Virtual Piano and other games with your MIDI keyboard: notes become computer-keyboard keys (full 88-key Roblox layout included), sent only to the game window you choose and only while it is in front. Mapping studio with learn mode, profiles, octave/transpose, split keyboard, sustain pedal, a live monitor, and an Emergency stop (**Ctrl+Alt+End**). On the website it shows a preview of the keys only.
- **🗕 System tray** *(Windows app)* — optionally keep Piano Logger running in the tray when you close the window: it keeps logging with very little memory and no graphics (the bridge does not press keys while in the tray)
- **26 calm animated roll backgrounds** (rain, snow, blossoms, a moonlit lake, lanterns, aurora-like light and more) with a background studio to preview and customise them
- **Dashboards & analytics** — lifetime stats, per-key breakdowns, speed graphs, dynamics and velocity analysis, hand-balance insights, and a "Year in Review" image
- **Chord & key detection**, **practice mode** with `.mid` files, **ear training**, **recorder & MIDI export**, **metronome with timing analysis**
- **Sound routing** to an external synth or VST host through a virtual MIDI cable (e.g. loopMIDI)
- **Full data ownership** — export your whole log as CSV or JSON, backups that also carry your settings, themes and profile, restore, automatic daily backups (Windows app)
- **Theme Studio** — 182 themes with live preview, favourites, your own accent colour, glass/blur/glow/motion dials; frosted-glass drop-down menus in every theme

## 💻 System requirements

Tested on: Windows 10 22H2 · Intel Pentium G3240 (2 cores, 3.1 GHz, 2014) · 8 GB RAM · NVIDIA GeForce GT 710 — it runs, but with animated themes and backgrounds it is not perfectly smooth on this PC (about 17–20 frames per second while playing chords on the High preset).

| | Minimum | Recommended (smoothest, Ultra quality) |
|---|---|---|
| **Operating system** | Windows 10 or Windows 11 (64-bit) for the app; any desktop OS with Chrome/Edge for the website | Windows 11 (64-bit) |
| **Website** | Chrome, Edge, Opera or Brave on a desktop computer (Web MIDI). Safari cannot use MIDI keyboards. | Latest Chrome or Edge |
| **Processor** | 2 cores, about 3 GHz (e.g. Intel Pentium G3240 — tested) | 4 cores or more (Intel Core i5 / AMD Ryzen 5 from 2018 or newer) |
| **Memory (RAM)** | 4 GB (8 GB tested) | 8 GB or more |
| **Graphics** | Any DirectX 11 graphics, including built-in graphics (GeForce GT 710 — tested). Use the **Low** or **Medium** graphics preset. | A dedicated or modern built-in GPU (e.g. GeForce GTX 1050 / Intel Iris Xe or better) for **High / Ultra** |
| **Screen** | 1280 × 720 | 1920 × 1080 or higher |
| **Disk space** | 10 MB for the app, plus about **0.2 MB per 1,000 key presses** (335,000 presses ≈ 60 MB) | 1 GB free for years of history and backups |
| **Other** | WebView2 runtime (built into Windows 11 and up-to-date Windows 10; if it is missing the installer downloads it — internet needed) · a MIDI keyboard for MIDI features | A USB MIDI keyboard; a sustain pedal for the pedal features |

There is no upper limit: faster computers, 4K screens and high-refresh monitors are all supported. The *Recommended* column is an estimate for perfectly smooth animation on the highest settings; only the *tested* machine above was actually measured. The website and app use the same page, so the same requirements apply to both (the Windows app additionally needs Windows 10/11 64-bit).

## 🔒 Privacy

This app has **no server and no accounts**. All data — your key presses, recordings, settings, and practice scores — is stored only on your own device. Nothing is uploaded anywhere. If you clear your browser data, your log history is lost unless you've made a backup, so regular backups are recommended (the Windows app makes one every day).

## 🚀 How to use

1. Open the [live link](https://kosandra0.github.io/Piano_Logger/) in a desktop browser — **Chrome, Edge, Brave, or Opera** work best (Safari does not support Web MIDI).
2. Click anywhere on the page once to enable sound.
3. Plug in a MIDI keyboard and press **Connect MIDI**, or tick **Play with PC keyboard**, or click the on-screen piano.
4. Explore the navigation — Overview, History, Analysis, Studio, Practice, MIDI Bridge and the Theme Studio.
5. Click **?** for the in-app guide, or press **Ctrl+K** to search any action.

## 🖥 Windows app

The Windows app shows the same page in its own window and adds: real key presses for the **MIDI Keyboard Bridge**, an optional **system tray** mode, your own SoundFont (.sf2) instruments, automatic daily backups into `Documents\Piano Logger Backups`, always on top, start with Windows, sound that keeps playing when minimised, and no MIDI permission pop-ups.

- Download `Piano-Logger-Setup.exe` from the [latest release](https://github.com/kosandra0/Piano_Logger/releases/latest) and run it. Windows may say "Windows protected your PC" because the app is not signed by a company: click **More info → Run anyway**.
- The app keeps its own data. To bring your history (and settings) from the website or an older app: **Data → Create backup file** there, then **Data → Restore** in the new app.
- Built with [Tauri 2](https://tauri.app/) (source in [`Piano-Logger-tauri/`](Piano-Logger-tauri/), about 4 MB installed). It installs next to the older Electron app (source in [`desktop/`](desktop/)); it does not replace it.
- **MIDI Bridge and games:** Piano Logger only presses ordinary keys, like typing — no game files, memory or anti-cheat are touched. Keep an English (US) keyboard layout active in the game; symbols and digits assume a US layout. Keep the Piano Logger window open (it may be minimised) while you play: in the system tray the bridge is off.

## 💻 Running it locally

Since this is a single self-contained HTML file, you can also download `index.html` and open it directly in your browser, or serve it locally for a more stable MIDI-permission experience:

```bash
python -m http.server 8000
```

Then visit `http://localhost:8000/index.html`.

## 📄 License

Free to use, modify, and share.
