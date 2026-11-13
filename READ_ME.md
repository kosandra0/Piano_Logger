# 🎹 Piano Lifetime Logger

A single-file, browser-based piano MIDI visualizer and lifetime key logger. Plug in a MIDI keyboard, play with your computer keyboard, or click the on-screen piano — every note you play is saved permanently on your own device, with rich statistics, charts, and practice tools to help you grow as a player.

**Live demo:** [https://kosandra0.github.io/Piano_Logger/](https://kosandra0.github.io/Piano_Logger/)

**Windows app:** [Download Piano-Logger-Setup.exe](https://github.com/kosandra0/Piano_Logger/releases/latest/download/Piano-Logger-Setup.exe) (Windows 10 & 11, 64-bit)

**Current version:** 8.1.2 — 🎨 **34 new Carbon & Steel themes** in the style of Carbon Black (216 themes in all), with softly animated background dots, and twinkling stars in the Meteor shower, Synthwave grid and Fireflies roll backgrounds. Version 8.1.1 added: 🖼 your profile picture now shows beside your level; 🎬 the desktop app has an animated start-up screen (which also fixes the old layout that flashed at launch) and an “Exiting Piano Logger” screen; 🎤 Accumulated Notes keep up with fast singing and pulse with a soft glow; the pitch chart length is your choice (1–60 s). Version 8.1.0 added 🌈 **Cursor light**: a soft colour light follows your mouse (its size and strength are set in the desktop app, Settings → Graphics → Cursor light). Also in 8.1.0: Lite effects no longer blocks the quality presets, and Expressive motion is kept when you change quality. Earlier version (7.4.0) — 📊 **Voice activity tracking** (how long you speak or sing, with history, statistics and a dashboard section; only times are saved). 🎙 **Voice Studio**: sing, hum or speak into a microphone and hear your own voice with effects, or turn your voice into one of 10 instruments (and, optionally, play it on your MIDI keyboard). Voice → instrument keeps up with fast singing and speaking; floating "island" controls that shrink and grow smoothly; sections slide into place when you move them; a recording player. See [CHANGELOG.md](CHANGELOG.md) for everything that changed.

**Jump to:** [Features](#-features) · [System requirements](#-system-requirements) · [Privacy](#-privacy) · [How to use](#-how-to-use) · [Voice Studio](#-voice-studio) · [Windows app](#-windows-app)

## ✨ Features

- **Full 88-key piano** with falling notes, glowing key animations, and multiple built-in sound engines (synth, grand piano samples, electric piano, organ, strings, music box)
- **Three ways to play:** a real MIDI keyboard (via Web MIDI), your computer keyboard (two playable rows, remappable), or click/tap the on-screen keys
- **Lifetime logging** — every key press is saved forever on your device, including note, velocity, duration, timestamp, and input source
- **🎙 Voice Studio** *(new in 7.3)* — **Voice passthrough** (hear your own voice live with reverb, echo, noise gate and noise suppression) or **Voice → instrument** (a real-time pitch tracker turns the melody you sing, hum or speak into Grand Piano, Electric Piano, Violin, Cello, Flute, Clarinet, Trumpet, Synth Lead, Synth Pad or Organ). Optional: also play the notes on your MIDI keyboard. Recordings with a full player (play/pause, ±5 s, seek bar) and `.wav` export. Your voice never adds to your lifetime key-press count.
- **📊 Voice activity tracking** *(new in 7.4.0)* — Voice Studio now saves how long you speak or sing: each session (Speaking or Singing, from the *Voice style* setting) keeps its start and end time, the time the microphone was watched and the time a voice was detected. See them in **History → Voice activity**, **Key statistics → Voice statistics** and the **Dashboard**. Sessions are part of your full backups. Only times are saved, never sound; the detected time is an estimate.
- **🎮 MIDI Keyboard Bridge** *(Windows app)* — play Roblox Virtual Piano and other games with your MIDI keyboard: notes become computer-keyboard keys (full 88-key Roblox layout included), sent only to the game window you choose and only while it is in front. Mapping studio with learn mode, profiles, octave/transpose, split keyboard, sustain pedal, a live monitor, and an Emergency stop (**Ctrl+Alt+End**). On the website it shows a preview of the keys only.
- **Floating islands** — while Voice Studio or the MIDI Bridge is running, a small dark-glass control "island" stays in the corner (status, live microphone meter, Stop); click its icon to shrink it
- **🗕 System tray** *(Windows app)* — optionally keep Piano Logger running in the tray when you close the window: it keeps logging with very little memory and no graphics (the bridge does not press keys while in the tray)
- **26 calm animated roll backgrounds** (rain, snow, blossoms, a moonlit lake, lanterns, aurora-like light and more) with a background studio to preview and customise them
- **Dashboards & analytics** — lifetime stats, per-key breakdowns, speed graphs, dynamics and velocity analysis, hand-balance insights, and a "Year in Review" image
- **Chord & key detection**, **practice mode** with `.mid` files, **ear training**, **recorder & MIDI export**, **metronome with timing analysis**
- **Sound routing** to an external synth or VST host through a virtual MIDI cable (e.g. loopMIDI)
- **Full data ownership** — export your whole log as CSV or JSON, backups that also carry your settings, themes and profile, restore, automatic daily backups (Windows app)
- **Cursor light** *(new in 8.1.0)* — a soft multi-colour light follows the mouse behind the panels; in the desktop app choose its size (200–900 px) and strength (0–150%, 0 = off) in Settings → Graphics (the website uses the default light)
- **Theme Studio** — 216 themes (including the 34 Carbon & Steel themes) with live preview, favourites, your own accent colour, glass/blur/glow/motion dials; frosted-glass drop-down menus in every theme

## 💻 System requirements

Tested on: Windows 10 22H2 · Intel Pentium G3240 (2 cores, 3.1 GHz, 2014) · 8 GB RAM · NVIDIA GeForce GT 710 · ST50 USB MIDI keyboard · USB microphone. It runs, but with animated themes and backgrounds it is not perfectly smooth on this PC (about 17–20 frames per second while playing chords on the High preset).

| | 🟢 Minimum (it runs) | 🔵 Recommended (smooth, High/Ultra quality) | 🟣 Maximum (the most Piano Logger can make use of) |
|---|---|---|---|
| **Operating system** | Windows 10 (64-bit) for the app; any desktop OS with Chrome/Edge for the website | Windows 10 22H2 or Windows 11 (64-bit), up to date | Latest Windows 11 (64-bit). There is no 32-bit or ARM build of the app; macOS/Linux/phones: website only |
| **Browser (website)** | Chrome, Edge, Opera or Brave on a desktop computer (Web MIDI). Safari cannot use MIDI keyboards | Latest Chrome or Edge | Latest Chrome or Edge — every feature works there; the Windows app adds the extras listed below |
| **Processor (CPU)** | 2 cores, about 3 GHz (Intel Pentium G3240 — tested) | 4 cores or more (Intel Core i5 / AMD Ryzen 5 from 2018 or newer) | Any modern CPU. Piano Logger mainly uses 1–2 cores, so more than about 8 cores brings no further gain *(estimate)* |
| **Memory (RAM)** | 4 GB (8 GB tested) | 8 GB | 16 GB — more brings no further gain *(estimate)* |
| **Graphics (GPU)** | Any DirectX 11 graphics, including built-in graphics (GeForce GT 710 — tested). Use the **Low** or **Medium** graphics preset | A dedicated or modern built-in GPU (e.g. GeForce GTX 1050 / Intel Iris Xe or better) for **High / Ultra** | Any faster GPU. **Ultra** is the highest preset; beyond a mid-range GPU it does not get any smoother *(estimate)* |
| **Screen** | 1280 × 720 (the layout also works in windows down to about 420 px wide) | 1920 × 1080 | 4K screens and high-refresh (120/144 Hz and more) monitors are supported; there is no upper limit |
| **Disk space** | 10 MB for the app, plus about **0.2 MB per 1,000 key presses** (350,000 presses ≈ 60 MB) | 3 GB free — the Windows app keeps the 30 newest daily backups, and each backup is about 0.25 MB per 1,000 presses (≈ 85 MB at 350,000 presses) | No limit: history is never trimmed. Plan about **8 MB per 1,000 presses** in total if you keep all 30 daily backups (≈ 2.7 GB at 350,000 presses) |
| **MIDI keyboard** | None needed (computer keyboard or on-screen piano) | Any USB MIDI keyboard | An 88-key USB MIDI keyboard with a sustain pedal (every key, velocity, pedal and the MIDI Bridge's full 88-key Roblox layout are used) |
| **Voice Studio audio** | Any microphone (built-in, webcam or USB) and speakers | A USB microphone **and headphones** (prevents feedback, lowest delay) | A good USB/audio-interface microphone, headphones, and a keyboard with an **AUX IN** jack to hear Piano Logger's instruments through the keyboard's own speakers (needs an audio cable) |
| **Internet** | Only to open the website, to download the app, and once per sampled instrument | — | — (the Windows app works fully offline after the first download of each sampled instrument) |
| **Other** | WebView2 runtime (built into Windows 11 and up-to-date Windows 10; if it is missing the installer downloads it) | — | — |

Notes: only the *tested* machine above was actually measured; the *Recommended* and *Maximum* columns are estimates for smooth animation on the highest settings. Disk figures were measured on a real log of about 350,000 presses. The website and the app use the same page, so the same requirements apply to both (the Windows app additionally needs Windows 10/11 64-bit).

## 🔒 Privacy

This app has **no server and no accounts**. All data — your key presses, voice recordings, settings, and practice scores — is stored only on your own device. Nothing is uploaded anywhere. The microphone is used only while Voice Studio is started and is closed when you press Stop. If you clear your browser data, your log history is lost unless you've made a backup, so regular backups are recommended (the Windows app makes one every day).

Voice activity tracking (7.4.0) saves only numbers — when a session started and ended, how long the microphone was watched and how long a voice was detected. It saves no audio, no text and nothing is uploaded. These sessions are included in full backups.

## 🚀 How to use

1. Open the [live link](https://kosandra0.github.io/Piano_Logger/) in a desktop browser — **Chrome, Edge, Brave, or Opera** work best (Safari does not support Web MIDI).
2. Click anywhere on the page once to enable sound.
3. Plug in a MIDI keyboard and press **Connect MIDI**, or tick **Play with PC keyboard**, or click the on-screen piano.
4. Explore the navigation — Overview, History, Analysis, Studio, Practice, Voice Studio, MIDI Bridge and the Theme Studio.
5. Click **?** for the in-app guide, or press **Ctrl+K** to search any action.

## 🎙 Voice Studio

Open it from the navigation (**Create → Voice Studio**).

1. **Devices** tab: choose your microphone and where the sound should come out. Use **headphones** if you can — speakers can feed back into the microphone (Voice Studio mutes itself if it hears a howl).
2. Choose a mode: **Voice passthrough** (your own voice with effects) or **Voice → instrument**, then pick an instrument card (▶ previews it).
3. Press **Start** and sing, hum or speak. **Stop** closes the microphone straight away.
4. *Optional:* **Also play on a MIDI keyboard** sends the notes to your keyboard (e.g. over USB), which plays them with its own sounds. In **Voice tracking**, **Response → ⚡ Fast** follows quick singing and speaking; **Balanced** and **Steady** are calmer but slower.
5. **Recordings** tab: record, play back, rename, save as `.wav`, delete. Recordings are kept separately and are not part of your key-press backups.

Good to know: Voice → instrument follows the *melody* of your voice — it does not turn words into music. A USB MIDI cable carries notes, not sound: to hear Piano Logger's own instruments (or your voice) through a keyboard's speakers, connect the PC's audio output to the keyboard's **AUX IN** with an audio cable.

## 🖥 Windows app

The Windows app shows the same page in its own window and adds: real key presses for the **MIDI Keyboard Bridge**, an optional **system tray** mode, your own SoundFont (.sf2) instruments, automatic daily backups into `Documents\Piano Logger Backups`, always on top, start with Windows, sound that keeps playing when minimised, and no MIDI or microphone permission pop-ups.

- Download `Piano-Logger-Setup.exe` from the [latest release](https://github.com/kosandra0/Piano_Logger/releases/latest) and run it. Windows may say "Windows protected your PC" because the app is not signed by a company: click **More info → Run anyway**.
- **Updating:** run the new `Piano-Logger-Setup.exe`; your history, settings, themes and recordings are kept. If the installer offers **"Uninstall before installing"**, never tick **"Delete the application data"** in the uninstaller — that would erase your history. Make a backup first (**Data → Create backup file**) to be safe.
- The app keeps its own data. To bring your history (and settings) from the website or an older app: **Data → Create backup file** there, then **Data → Restore** in the new app.
- Built with [Tauri 2](https://tauri.app/) (source in [`Piano-Logger-tauri/`](Piano-Logger-tauri/), about 4 MB installed). It installs next to the older Electron app (source in [`desktop/`](desktop/)); it does not replace it.
- **Voice Studio in older apps:** the website page updates installed apps automatically, but the microphone permission is part of the app itself — install app **7.3.0 or newer** to use Voice Studio in the Windows app.
- **MIDI Bridge and games:** Piano Logger only presses ordinary keys, like typing — no game files, memory or anti-cheat are touched. Keep an English (US) keyboard layout active in the game; symbols and digits assume a US layout. Keep the Piano Logger window open (it may be minimised) while you play: in the system tray the bridge is off.

## 💻 Running it locally

Since this is a single self-contained HTML file, you can also download `index.html` and open it directly in your browser, or serve it locally for a more stable MIDI-permission experience:

```bash
python -m http.server 8000
```

Then visit `http://localhost:8000/index.html`. (Browsers allow the microphone for Voice Studio on `https://` pages and on `localhost`.)

## 📄 License

Free to use, modify, and share.
