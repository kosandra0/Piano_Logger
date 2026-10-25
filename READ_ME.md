# 🎹 Piano Lifetime Logger

A single-file, browser-based piano MIDI visualizer and lifetime key logger. Plug in a MIDI keyboard, play with your computer keyboard, or click the on-screen piano — every note you play is saved permanently in your own browser, with rich statistics, charts, and practice tools to help you grow as a player.

**Live demo:** [https://kosandra0.github.io/Piano_Logger/](https://kosandra0.github.io/Piano_Logger/)

## ✨ Features

- **Full 88-key piano** with falling notes, glowing key animations, and multiple built-in sound engines (synth, grand piano samples, electric piano, organ, strings, music box)
- **Three ways to play:** a real MIDI keyboard (via Web MIDI), your computer keyboard (two playable rows, remappable), or click/tap the on-screen keys
- **Lifetime logging** — every key press is saved forever in your browser's IndexedDB, including note, velocity, duration, timestamp, and input source
- **Dashboards & analytics** — lifetime stats, per-key breakdowns, speed graphs, dynamics and velocity analysis, hand-balance insights, and a "Year in Review" shareable image
- **Chord & key detection** — live chord recognition, chord progression tracking (in both chord names and Roman numerals), and session-by-session key detection
- **Practice mode** — load any `.mid` file or built-in demo song, choose a hand, pick a mode (Play Along / Wait for Me / Watch & Listen), and get a real-time accuracy score
- **Ear training** — interval and chord recognition games with accuracy tracking
- **Recorder & MIDI export** — record your playing, scrub/trim/loop a take, and export it as a standard `.mid` file
- **Metronome with timing analysis** — see exactly how early or late you are, with visual feedback
- **Sound routing** — send everything you play to an external synth or VST host via a virtual MIDI cable (e.g. loopMIDI + Keyzone Classic)
- **Full data ownership** — export your entire log as CSV or JSON, create backups, restore from backup, or auto-backup to a local folder
- **Deep customization** — 7 visual themes, custom accent colors, background images, compact/high-contrast/calm modes, draggable and collapsible sections, and a command palette (`Ctrl+K`) for quick access to every feature

## 🔒 Privacy

This app has **no server and no accounts**. All data — your key presses, recordings, settings, and practice scores — is stored only in your own browser (via IndexedDB and localStorage), on your own device. Nothing is uploaded anywhere. If you clear your browser data, your log history is lost unless you've made a backup, so regular backups (Export & Backup tab) are recommended.

## 🚀 How to use

1. Open the [live link](https://kosandra0.github.io/Piano_Logger/) in a modern desktop browser — **Chrome, Edge, Brave, or Opera** work best (Safari does not support Web MIDI).
2. Click anywhere on the page once to enable sound.
3. Plug in a MIDI keyboard and press **Connect MIDI**, or tick **Play with PC keyboard** to use your computer keys, or just click the on-screen piano.
4. Explore the tabs — Dashboard, History, Analysis, Studio, and Practice — to see your stats and tools grow as you play.
5. Click the **?** button in the top right anytime for a full in-app guide, or press **Ctrl+K** to search any action.

## 💻 Running it locally

Since this is a single self-contained HTML file, you can also download `index.html` and open it directly in your browser, or serve it locally for a more stable MIDI-permission experience:

```bash
python -m http.server 8000
```

Then visit `http://localhost:8000/index.html`.

## 📄 License

Free to use, modify, and share.
