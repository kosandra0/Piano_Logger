# 🎹 Piano Lifetime Logger

A single-file, browser-based piano MIDI visualizer and lifetime key logger. Plug in a MIDI keyboard, play with your computer keyboard, or click the on-screen piano — every note you play is saved permanently on your own device, with rich statistics, charts, and practice tools to help you grow as a player.

**Live demo:** [https://kosandra0.github.io/Piano_Logger/](https://kosandra0.github.io/Piano_Logger/)

**Windows app:** [Download Piano-Logger-Setup.exe](https://github.com/kosandra0/Piano_Logger/releases/latest/download/Piano-Logger-Setup.exe) (Windows 10 & 11, 64-bit)

## 🆕 What's new in 9.1.0 (released 2026-11-18)

Full list with causes and limits: [CHANGELOG.md](CHANGELOG.md#910--2026-11-18--light-and-dark-sheet-music-3d-hands-piano-videos-and-a-bigger-heatmap).

- **🌗 Light or Dark sheet music:** pick the look above the score; the PDF you save uses it (charcoal pages with light notes, or white pages). Cleaner rests, chord names that never touch the notes, silent opening bars combined into one, and evenly filled pages.
- **✋ 3D hands:** two life-size jointed hands play the notes on the falling-notes keyboard in time with the sound, with finger numbers on the pressed keys (the fingering is a best guess from the notes). Five see-through styles (Hologram, Glass, Silhouette, Sketch, Soft) and four lifelike models (Realistic, Pianist, Sculpture, Anatomical) with a skin-tone choice. Choose your own left- and right-hand colours, size and see-through amount. Motion is smooth: a wave through the fingers, a small lift while travelling, a slow "breathing" at rest, and soft light, rings and sparks where notes land.
- **🎬 Export a piano video:** save the falling notes, keyboard and hands with the sound as a real MP4 (H.264 + AAC; 240p to 1080p; landscape, vertical or square; 30 or 60 fps; optional title and performer). Sound and picture are frame-exact (no drift, both tracks the same length). The video has its own look settings with a live preview. It is made **in the background** with a small progress card (three steps) while you keep using the app, then the Save dialog appears.
- **📝 MIDI → Markdown:** a plain-words .md of any piece — note names for each hand, the chord written under the exact notes, summary and chord progression (Simple, Detailed or Complete; letters or Do Re Mi). A smarter chord engine (triads, sevenths, sus, add9, sixths, diminished, augmented, power chords and slash chords) also improves the PDF chord names.
- **🛠 Fix which hand:** in Falling notes → Settings, drag a box over notes and mark them Left or Right. It changes the hands, colours, score and PDF, is remembered per piece, and can be reset.
- **🎨 16 visualizer themes** (6 neon, 5 calm, 5 classic) in one Settings panel, peaceful note-colour presets and hand-colour swatches. Keyboard sizes: notes used, 44, 61, 88 or 108 keys, changing smoothly.
- **🪟 One frosted-glass look:** the Settings drawer, Export window, instrument list and toolbar match the app's drop-down menus (see-through, blurred, rounded, with a colour-gradient row and a tick on the chosen item). Works in light themes, High contrast and No-blur.
- **🚀 Start-up and exit screens** show the version and a "what is new" line; Settings → Start-up & exit has 7 dark and 7 light themes (or follow Windows).
- **🎓 Sight-reading course** grown from 29 to 48 lessons in 5 levels and 7 modules, with a minor-keys module, practice sheets and a mixed review (your old progress is kept).
- **📅 Bigger practice heatmap** (52 weeks, or 26) and easier-to-read small labels across the app.

**🛠 Fixes in 9.1.0**
- **Videos now show the hands.** A late-stage bug cut the hands out of exported videos (the video's lighter hand layer was clipped with full-size numbers, so hands in the lower half of the picture vanished). Fixed for all nine hand styles, with a new test that counts hand pixels in the video picture.
- **Lifelike-hand outlines** were white instead of dark; fixed.
- **Rests** are written properly (no off-beat dotted rests), the PDF no longer wastes a last page, and chord names no longer touch the notes.
- **Videos no longer drift out of sync** on a slow PC (the old real-time recorder dropped sound and picture); the new exporter writes every frame and sample with an exact time.
- **Menus and panels** open inside full-screen views, and the export window no longer overlaps the toolbar.

**Honest limits:** the hands are stylised and the fingering is a rule-based guess; a video takes at least as long as the music to make; PDF text uses Western European letters; not tested on real ST50 / GT 710 hardware (MIDI was simulated).

## 🆕 What's new in 9.0.0 (released 2026-11-18)

- **🎼 Sheet Music** (menu → Create): record or import MIDI files, read them as a proper grand-staff score (chord names and note names optional), export a multi-page vector **PDF**, save `.mid` files where you choose, play them back with their own instruments or any sound the app has, and watch them as **falling notes**.
- **🎓 Sight reading:** a 29-lesson course from the first notes on the staff to two-hand reading. Every practice sheet is new and musical, and your MIDI keyboard is checked note by note, at your own pace or with the beat, with friendly hints.
- The calendar heatmap moved to **Statistics → Practice activity**; the version chip got a cleaner look.

## 🆕 What's new in 8.0.1 (released 2026-11-11)

- **Backup save location:** "Create backup file" now asks where to save the backup (a Save dialog). In the Windows app the file goes to the folder you choose; in Chrome and Edge the browser shows its Save dialog. Cancelling saves nothing.

## 🆕 What's new in 8.0.0 (released 2026-11-11)

Download the Windows app from the [latest release](https://github.com/kosandra0/Piano_Logger/releases/latest). The website version is at the live demo link above. Full history: [CHANGELOG.md](CHANGELOG.md).

**🎤 Sung Note Statistics** *(Windows app)*
- Each stable note you sing or hum in Voice Studio (Singing style) is counted **once**, however long you hold it. Speaking never counts.
- The number updates **live** while you sing. You don't need to stop Voice Studio to see it.
- **Statistics → Voice Statistics** shows a **Total notes sung** card, and a **Sung Note Statistics** section above Key Statistics with summary cards (total and distinct notes, most sung note, note lengths, sessions, lowest and highest note), four charts (most sung notes, pitch classes, octaves, activity over time by day, week or month) and a sortable table with one row per note.
- Note names are the nearest piano note to your pitch. They are an estimate, not what you meant to sing.
- Stored only on your PC: note information only, never sound, and nothing is uploaded. Included in backups; Clear Notes does not remove these statistics.

**🎨 Look and statistics** *(website and Windows app)*
- The page is now called **Statistics**, with separate Voice Statistics and Key Statistics sections.
- The version chip under the Piano Logger name has a dark centre and a slowly turning, pulsing rainbow border.
- Black keys show their note label with clearer contrast. Theme Studio favourite stars no longer overlap the card text.

**🔤 Offline font library** *(Windows app)*
- 88 bundled open-licence font families, plus Windows system fonts, with search, a category filter and live preview. The chosen font applies app-wide and survives a restart.
- The Windows app no longer asks Google Fonts for anything, so text looks the same without an internet connection. The website still uses Google Fonts.

**📖 Guidance** *(website and Windows app)*
- What's New, Help, the complete guide and a tour step describe the new features.
- The "Download app" panel lists the Windows-only features.

**🛠 Fixes**
- The up-down crop slider in the picture tool moves again. When a picture has no room to move on that axis, the zoom is raised automatically.
- The sidebar no longer keeps "Live activity" highlighted on History and Statistics.
- Voice Studio noise reduction works when the page is opened as a file in a browser.
- What's New no longer shows "Invalid Date" for a release date that is not set.
- More space above Key Statistics under the Sung Note help text.

**Jump to:** [Features](#-features) · [System requirements](#-system-requirements) · [Privacy](#-privacy) · [How to use](#-how-to-use) · [Voice Studio](#-voice-studio) · [Windows app](#-windows-app)

## ✨ Features

- **Full 88-key piano** with falling notes, glowing key animations, and multiple built-in sound engines (synth, grand piano samples, electric piano, organ, strings, music box)
- **Three ways to play:** a real MIDI keyboard (via Web MIDI), your computer keyboard (two playable rows, remappable), or click/tap the on-screen keys
- **Lifetime logging** — every key press is saved forever on your device, including note, velocity, duration, timestamp, and input source
- **🎙 Voice Studio** *(new in 7.3)* — **Voice passthrough** (hear your own voice live with reverb, echo, noise gate and noise suppression) or **Voice → instrument** (a real-time pitch tracker turns the melody you sing, hum or speak into Grand Piano, Electric Piano, Violin, Cello, Flute, Clarinet, Trumpet, Synth Lead, Synth Pad or Organ). Optional: also play the notes on your MIDI keyboard. Recordings with a full player (play/pause, ±5 s, seek bar) and `.wav` export. Your voice never adds to your lifetime key-press count.
- **📊 Voice activity tracking** *(new in 7.4.0)* — Voice Studio now saves how long you speak or sing: each session (Speaking or Singing, from the *Voice style* setting) keeps its start and end time, the time the microphone was watched and the time a voice was detected. See them in **History → Voice activity**, **Key statistics → Voice statistics** and the **Dashboard**. Sessions are part of your full backups. Only times are saved, never sound; the detected time is an estimate.
- **🎤 Sung Note Statistics** *(new in 8.0.0, Windows app)* — while Voice Studio is in the Singing style, each stable note is counted once as the nearest piano note to your pitch (an estimate, not what you meant to sing). **Statistics → Voice Statistics** shows a **Total notes sung** card and a **Sung Note Statistics** section: summary cards, charts for the most sung notes, pitch classes, octaves and activity over time, and a sortable table (count, share, durations, average frequency, cents, confidence, sessions, first and last time). Separate from the piano Key Statistics, included in backups, never removed by Clear Notes. Only note information is saved, on your PC; no sound and nothing is uploaded
- **🎮 MIDI Keyboard Bridge** *(Windows app)* — play Roblox Virtual Piano and other games with your MIDI keyboard: notes become computer-keyboard keys (full 88-key Roblox layout included), sent only to the game window you choose and only while it is in front. Mapping studio with learn mode, profiles, octave/transpose, split keyboard, sustain pedal, a live monitor, and an Emergency stop (**Ctrl+Alt+End**). On the website it shows a preview of the keys only.
- **Floating islands** — while Voice Studio or the MIDI Bridge is running, a small dark-glass control "island" stays in the corner (status, live microphone meter, Stop); click its icon to shrink it
- **🗕 System tray** *(Windows app)* — optionally keep Piano Logger running in the tray when you close the window: it keeps logging with very little memory and no graphics (the bridge does not press keys while in the tray)
- **26 calm animated roll backgrounds** (rain, snow, blossoms, a moonlit lake, lanterns, aurora-like light and more) with a background studio to preview and customise them
- **Dashboards & analytics** — lifetime stats, per-key breakdowns, speed graphs, dynamics and velocity analysis, hand-balance insights, and a "Year in Review" image
- **🎼 Sheet Music** *(new in 9.0.0)* — MIDI recorder with Save As, MIDI import/export, a readable grand-staff score, multi-page PDF export, a falling-note visualizer for any file, and a sight-reading course with generated exercises checked through your MIDI keyboard
- **Chord & key detection**, **practice mode** with `.mid` files, **ear training**, **metronome with timing analysis**
- **Sound routing** to an external synth or VST host through a virtual MIDI cable (e.g. loopMIDI)
- **Full data ownership** — export your whole log as CSV or JSON, backups that also carry your settings, themes and profile, restore, automatic daily backups (Windows app)
- **Theme Studio** — 182 themes with live preview, favourites, your own accent colour, glass/blur/glow/motion dials; frosted-glass drop-down menus in every theme

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
