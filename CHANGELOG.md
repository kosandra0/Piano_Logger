# Changelog — Piano Logger

Versions of the Piano Logger page (`APP_VERSION` in `piano-logger.html`). The same history is shown in the app under **What's new**.
Desktop app versions are listed separately at the end.

## 7.0.2 — 2026-10-31 · Glass title bar with window controls

### New features
- Windows app: the window has no Windows frame any more. A glass title bar with macOS-style controls takes its place: red closes, yellow minimizes, green maximizes / restores.
- Drag the title bar to move the window, double-click it to maximize or restore. Every edge and corner still resizes the window; Windows 11 keeps its rounded corners and shadow.
- Website: the same title bar with a green control that switches full screen on and off (a web page is not allowed to close or minimize the browser window, so those controls are not shown).

### UI & UX
- The control symbols appear when you point at the controls or reach them with Tab; the lights turn grey while the window is in the background.
- The title bar shows the part of Piano Logger you are in. Hover and press animations stop with Motion Off or the system's reduce-motion setting.

### Compatibility
- The Electron app and the 7.0.0 / 7.0.1 Windows apps keep their normal Windows frame and show no extra bar.
## 7.0 — 2026-10-31 · Aurora Glass Studio: a complete redesign

### New features
- **Theme Studio**: 110 new themes in 11 collections (Glass & Liquid, Futuristic & Sci-Fi, Natural & Soothing, Luxury & Premium, Minimal & Professional, Retro & Digital, Vibrant & Colourful, Monochrome, Cinematic & Atmospheric, Artistic & Creative, Pastel & Soft), plus all 72 classic themes (182 in total, 188 with the Living Themes DLC).
- Each new theme is its own design language: shape (sharp, soft, round, pill), surface (liquid glass, frost, glass, solid, paper, outline, neon, brutalist, CRT, velvet, holographic), typeface, background artwork and motion style. Text and accent contrast are checked when a theme is built.
- Search, collections, a light / dark filter, favourites, recently used, live preview of the whole app before applying, "Surprise me" and Reset to default.
- Your own accent colour, and dials for glass opacity, background blur, reflections, border brightness, glow and background intensity.
- Motion levels: Off, Reduced, Balanced, Expressive (Settings → Motion & themes, or the Theme Studio). The system's reduce-motion setting always wins.
- MIDI device centre: connection status, every input and output with its state, listen to one keyboard or all, MIDI out, permission and environment.
- Overview at the top of the Dashboard: connection, lifetime / today / session presses, fastest speed this session, the notes just played, quick actions.

### UI & UX
- Navigation rail with every area of the app, an animated active marker, a live keyboard visualiser and the MIDI status. It folds to icons, and opens from a menu button on smaller screens.
- The top bar shows where you are; tabs have a sliding indicator; panels fade in from a soft blur.
- One consistent line-icon set in the top bar and navigation.
- What's new is now a version timeline with an About card.

### Visual effects
- Layered glass: tinted surfaces, top reflections, inner light, fine noise, gradient and holographic borders.
- Animated backgrounds per theme: aurora, nebula, synth grid, prism, rain, petals, confetti, circuit, blueprint, art-deco fans, ensō and more.
- A soft light follows the pointer over panels; ripples on buttons; numbers count up; the newest note pops in. Expressive motion adds magnetic buttons, card tilt and light sweeps.

### Performance
- Only the active theme's tokens and background are written (one small style rule), so the theme library costs nothing until a theme is picked.
- Animations pause while the window is hidden and stop completely with Motion Off, Lite effects or the system's reduce-motion setting.

### Bug fixes
- The 55 generated themes (Vaporwave, Nebula, Glacier…) were reset to Aurora Classic after every reload. The chosen theme is now kept.
- The "Updated to …" message always described version 6.1.
- Pressing Connect MIDI again while connected could make every key press arrive (and be logged) twice, because the previous MIDI connection kept its note handlers. The old connection is now released first. (Found with a simulated MIDI device; see the release audit.)

- Animated piano-roll backgrounds (Aurora, Meteor shower, Synthwave grid, Ocean waves, Fireflies) only moved for 8 seconds after a key press. They now keep moving while the piano roll is on screen and the window is visible (idle frames are capped at 30 fps; Motion Off, Lite effects and reduce-motion still stop them).
- With the Classic note style the chosen piano-roll background was ignored. Classic bars are now drawn by the visualizer, so every background works with them.
- Long option names in Settings dropdowns pushed the dropdowns out of their boxes and made the Settings window scroll sideways.

### Backups
- Backups (and "Download all as JSON") now also contain settings, themes, Theme Studio dials / favourites, graphics, sound, layout, profile, quests, badges, DLC choices, the background image and the pitch-bend / mod-wheel log, in a `settings` section placed before `logs`.
- Restore has a new "Also restore settings, themes & profile" option (on by default). It also accepts a settings-only file such as the recovery "extras" file.
- Older backups restore exactly as before; older Piano Logger versions can still restore the presses from new backups.
- Add-ons are deliberately not included (they are code).

### Compatibility
- The website still works on its own (Chrome, Edge, Brave, Opera); nothing in it needs the desktop app.
- New Windows desktop app built with Tauri 2 (`Piano-Logger-tauri/`), next to the existing Electron app, showing this same page.
- The "Download app" window describes the new app: 64-bit Windows 10 and 11 (the Tauri build has no 32-bit version).
- Users who still had the old default theme (Aurora Classic) are moved once to the new default, Aurora Glass. Aurora Classic stays in the Classic collection.

## 6.1 — 2026-10-27 · Big backups, fast periods & a new calendar
- Restore backups of any size (even several GB), read piece by piece with a progress bar, time left and a Stop button.
- Exports, backup files, the backup folder and the app's daily backup are built piece by piece too.
- Custom periods load much faster (each day is summed up once and saved).
- Much faster with a long history (History, speed statistics, Profile, Analysis).
- New themed calendar for every date box, with activity bars, quick ranges and keyboard support.
- Repeated messages show a counter instead of stacking up.
- Fixes: What's new opened scrolled to the bottom; repeated guide section numbers; boxes cut off on laptop screens; routine descriptions cut off.
- When saved data cannot be opened, the page says why and offers "Try again" (and "Repair storage" in the app).

## 6.0 — 2026-10-27 · Windows app, focus mode & rolling numbers
- "Download app" button: a free Windows app with extra features.
- App: SoundFont (.sf2) instruments, automatic daily backups, always on top, start with Windows, background audio, no MIDI pop-ups, offline use, updates from the website.
- Fixes: focus mode layout; rolling numbers on PCs with Windows animation effects off.

## 5.1 — 2026-10-26 · Shorter sustain, no more piling up
- Notes held by the sustain pedal fade within about 1.5 s; replaying a key stops its older copy; at most 16 notes ring at once.
- Sustain fade setting: Natural, Short, Long, Hold.

## 5.0 — 2026-10-26 · Sessions, natural sustain, quest boards, XP boosts & DLC
- Dashboard Period picker for the current, previous or any past session.
- Natural sustain fade; 5 daily quests and weekly to yearly quest boards; XP boosts; free DLC packs that unlock with your level.

## 4.0 — 2026-10-26 · Themes, graphics, sounds & practice tools
- 55 more themes (72 in total), Graphics settings, richer visualizer, smoother page animations.
- Sound library (128 sampled instruments and more), daily quests, practice routines, scale & chord explorer, player card, practice journal, update log.

## 3.1 — 2026-10-26 · Live XP & rolling numbers
## 3.0 — 2026-10-26 · Profile, levels & themes
## 2.1 — 2026-10-25 · Your local time
## 2.0 — 2026-10-25 · Practice, analysis & sound
## 1.0 — 2026-10-24 · First version

---

## Desktop app

| App | Date | Built with | Notes |
|---|---|---|---|
| 7.0.2 | 2026-10-31 | Tauri 2.12.5 (WebView2) | Frameless window with the page's glass title bar (close, minimize, maximize/restore, drag, double-click); new window commands win_state / win_minimize / win_toggle_max / win_close / win_drag. Page 7.0.2. |
| 7.0.1 | 2026-10-31 | Tauri 2.12.5 (WebView2) | Fixed: "Start with Windows" used the same Windows start-up entry name as the Electron app, so with both apps installed one replaced the other. The new app now uses its own name (an entry 7.0.0 left behind for itself is cleaned up). Page unchanged (7.0). |
| 7.0.0 | 2026-10-31 | Tauri 2.12.5 (WebView2) | New desktop target in `Piano-Logger-tauri/`. Same page and the same `window.pianoApp` features as the Electron app (SoundFonts, backups, always on top, start with Windows, updates of the page from the website). MIDI is allowed without a pop-up. The page is served with a Content-Security-Policy (no frames or plugins); remote websites open in the browser. 64-bit only. |
| 6.1.0 | 2026-10-27 | Electron 43 | Backups of any size, more memory, crash note. Unchanged by 7.0 (`Piano-Logger-app-6.1-files/`). |
| 6.0.0 | 2026-10-27 | Electron | First Windows app (`Piano-Logger-desktop-app/`). |
