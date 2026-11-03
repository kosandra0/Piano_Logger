# Changelog — Piano Logger

Versions of the Piano Logger page (`APP_VERSION` in `piano-logger.html`). The same history is shown in the app under **What's new**.
Desktop app versions are listed separately at the end.

## 7.3.4 — 2026-11-03 · Branding & maintenance update

### Branding
- Introduced the new Piano Logger waveform emblem.
- Replaced the old musical-note branding in the application sidebar.
- Updated the Windows application icon assets.
- Updated the website logo.
- Updated the website favicon and Apple touch icon.
- Corrected the dynamically generated sidebar branding.
- Improved consistency between the website and the desktop application branding.

### Maintenance
- Version updated to 7.3.4 everywhere (page, sidebar label, desktop app files). No change to piano, MIDI, Voice Studio or stored data.
- Development record: commit `188797b` (initial logo replacement) and `c7ebdeb` (sidebar branding correction).

## 7.3.3 — 2026-11-02 · Smooth motion: islands and sections

### UI & UX (owner's notes on G2 and G3 in `TESTING-7.3-voice.md`)
- **Floating islands shrink and grow with an animation.** Clicking the round icon: the title, status, meter and buttons fade away (soft blur) one after another, then the capsule closes up to the icon + ■ Stop. Clicking again: the capsule opens, a soft glow passes over it and the icon glows, and the details fade back in one after another. A fast double click cannot leave it half-open.
- **Sections slide while you drag them.** Dragging a section by its ⠿ handle now moves it live: the other sections slide up or down to make room (instead of only a line, then a jump when you let go). The section you drop glows for a moment where it lands. Esc, or letting go outside the window, slides everything back. The ↑ ↓ keys on the handle and the ↑ ↓ buttons in *Appearance & layout* slide too.
- All of this follows the motion settings: with Calm mode, animations off, or Windows animations off (unless "Play animations even when Windows animations are off" is on), islands and sections change instantly as before.

### Bug fixes
- While sections were moved, Chrome's scroll anchoring scrolled the page by a whole section, so the page jumped and the section under the mouse changed (found by the new test). Scroll anchoring is now off while a section moves.
- Voice Studio's mode and instrument cards ignored "Play animations even when Windows animations are off" (wrong class name in the 7.3 style rule).

### Desktop app 7.3.3
- No change to the app's code. Page 7.3.3 bundled.

### Tested by hand (Piano Logger's owner, installed app 7.3.3)
- Island shrink/grow animation (Voice Studio and MIDI Bridge), sections sliding while dragging, drop glow, Esc to cancel: all passed. Owner: "so far everything perfect".

## 7.3.2 — 2026-11-02 · Voice → instrument keeps up with you · floating islands · section dragging fixed

### Voice Studio — speed (owner's hands-on report C4 / Fix 3: "lag… does not match my speed")
- Measured with a fast synthetic phrase (19 notes, 3 runs each, same conditions): **7.3.1** caught 17/19 sung notes and 12–13/19 spoken ones, with notes starting **154–196 ms** late and still sounding **~180–236 ms** after the voice stopped; three quick syllables on one pitch became one note. **7.3.2 (Fast)** catches 19/19 in both styles, notes start **about 25–65 ms** after the voice and stop **about 45–90 ms** after it, and every syllable is a note.
- How: analysis window 64 → 40 ms and analysed every 8 ms instead of 16 ms; notes are decided on the fast median pitch (the heavier smoothing is only used for gliding); a note starts once three frames agree; "Note stability" default 60 → 30 ms (speech ×1.4 instead of ×2); a dip-and-rise in loudness marks a new syllable and strikes the note again; the note stops ~40 ms after the voice.
- **Response** choice (Voice tracking tab): ⚡ Fast (default) · Balanced · Steady. Settings saved by 7.3/7.3.1 get Fast; the stability/smoothing sliders move to Fast's values only if they were still at the old defaults.
- **Sustain** now only lengthens the fade of Piano Logger's own sound; it no longer keeps the note (or the keyboard's MIDI note) on after you stop.
- Octave check in the pitch detector (if the dip at twice the period is much deeper, the real period is taken) — fewer wrong high notes at note starts.
- **▶ Preview also plays on the MIDI keyboard** when "Also play on a MIDI keyboard" is on (owner's request C6); the keyboard echo is never logged.

### UI & UX (owner's Fix 1)
- The floating Voice Studio and MIDI Bridge bars are **islands**: dark glass capsules with a rotating glow border in the theme's colours, an icon badge with a pulse while live (click = make the island small: badge + Stop), a small title above the status, round buttons, and a live microphone meter on the Voice Studio island. They stack in one corner without overlapping, and **notifications move up so they never cover them** (before, the MIDI Bridge bar could be covered).

### Bug fixes
- **Desktop app: "Drag to move this section" never worked** (owner's Fix 2) — every drag ended with a "not allowed" cursor. Tauri replaces WebView2's drag-and-drop handler on Windows; the app now turns that off (`disable_drag_drop_handler`), so normal page drag-and-drop works. Needs the Windows app 7.3.2.

### Desktop app 7.3.2
- `.disable_drag_drop_handler()` on the main window (the app never used Tauri's file-drop events). Page 7.3.2 bundled.

## 7.3.1 — 2026-11-02 · Voice Studio: a real recording player, Stop that stops

### Bug fixes
- **Voice Studio → Studio: "■ Stop" did nothing after Start** until you switched to another tab and back (reported by the owner). The button's action was decided when the tab was drawn — before Start — so it called Start again. It now checks the current state when you click.
- **Recordings: you could not see which recording was playing.** The row's button changed only on the next redraw, which happened before the recording had loaded.
- The Studio buttons' labels were rewritten 30 times a second; they now change only when their text changes.

### New features
- **Recording player:** the playing recording is highlighted with a moving equaliser; ▶ Play / ❚❚ Pause / ▶ Resume (continues where you paused) / ↺ Play again at the end; ⏪ 5 s and 5 s ⏩; ■ Stop; a bar that fills as it plays, with the current time and the length — click or drag it to jump anywhere (also while paused). Playing another recording switches the player to it.
- A paused player keeps its place while the Voice Studio window is open; with the window closed, the audio output is released after 15 s.

### Desktop app 7.3.1
- Page 7.3.1 bundled. No change to the app's code.

## 7.3 — 2026-11-01 · Voice Studio

### New features
- **Voice Studio** (navigation → Create → Voice Studio). Speak or sing into a microphone and hear it live, in one of two modes:
  - **Voice passthrough** — your own voice on the chosen output, with input gain, output volume, reverb (room size), echo (time, repeats), a noise gate and optional background-noise suppression / echo cancellation, and a "bypass all effects" switch. Each mode keeps its own effect settings.
  - **Voice → instrument** — a real-time pitch tracker turns the melody you sing, hum or speak into notes played by one of 10 instruments: Grand Piano, Electric Piano, Violin, Cello, Flute, Clarinet, Trumpet, Synth Lead, Synth Pad, Organ. Built-in synthesiser (offline, follows the voice continuously) or sampled General MIDI instruments (FluidR3 SoundFont, downloaded once per instrument).
- **Pitch tracking** (YIN) runs on the audio thread (AudioWorklet), about 60 times a second. A note sounds only when the sound is loud enough (noise threshold), clearly pitched (voice-detection confidence), not hissy (zero-crossing rate: “s”, “f”, breaths) and stable for the note-stability time; short gaps are bridged and single-frame octave jumps ignored. Silence and background noise make no notes.
- Controls: separate pitch sensitivity for **singing** and **speaking**, pitch smoothing, note stability, voice-detection confidence, noise threshold, octave (±3), transpose (±12), sustain, per-instrument volume / attack / release, whole notes or "follow exact pitch" (glide), louder-voice-louder-note, monitor your own voice with an adjustable monitoring delay.
- **Instrument selector**: cards with icon, name, ▶ preview, selection state and volume. Switching fades out and in (no click, no restart).
- **Devices & routing**: choose the microphone, the output device and the audio buffer (smallest … largest); lists audio devices and MIDI devices separately; test sound; a connection indicator that says whether the chosen output reaches the ST50 (USB audio / cable confirmed by you / this PC's output / not confirmed). Starting on an unconfirmed output asks first instead of silently playing elsewhere.
- **Optional: also play on a MIDI keyboard** — sends the detected notes (and the instrument as a General MIDI program) to a MIDI output such as the ST50, so the keyboard plays them with its own sounds over the USB cable. Off by default. Notes a keyboard echoes back are never logged as piano presses.
- **Recordings**: record what you hear or only the microphone; play back, rename, save as `.wav` or the original `.webm`, delete after confirmation. Stored in their own database `PianoLoggerVoiceStudio`, never in the piano log or its backups. 30-minute limit per recording.
- Live waveform, input / output level meters with peak markers and clipping lights, the detected note with cents, a 6-second pitch display.

### Safety
- The microphone opens only when you press Start; Stop, Emergency stop, the tray and closing Piano Logger close it and the audio device.
- Mute microphone, mute output, a limiter, a fixed volume ceiling and an input gain limit of +12 dB. Feedback (a steady howl) mutes the output automatically; 3 seconds of overload turn the volume down. A device that is unplugged stops Voice Studio with a message instead of switching to another speaker.

### Honest limits
- The ST50 is a USB **MIDI** keyboard: on this PC Windows lists it only as MIDI ports (`ST-50`), with no speaker or microphone device. Its USB cable carries notes, not sound. Your voice (or Piano Logger's instruments) through the ST50's speakers needs an audio cable from the PC's output to the keyboard's audio input, if the model has one. The "Also play on a MIDI keyboard" option works over USB but uses the keyboard's own sounds.
- Pitch tracking follows the melody of the voice; it does not turn words into music.

### Desktop app 7.3.0
- The microphone is allowed for the app's own page (like MIDI); the page only uses it after Start in Voice Studio. The page's Content-Security-Policy allows `blob:` scripts so the audio-thread module can load.

## 7.2.2 — 2026-11-01 · Glass drop-down menus

### UI & UX
- Every drop-down menu opens as a **frosted-glass menu** in the theme's colours (macOS style) instead of the dark, square list Windows draws. Mouse and keyboard (Space / Enter / Alt+↓ / F4 to open, arrows, Home / End, Page Up / Down, type to jump, Enter to choose, Esc to close); long lists scroll inside the window and open upwards when there is no room below. The real drop-down keeps the value, so every setting works as before. Phones and tablets keep their own pickers. High-contrast and "no blur" modes get a solid menu.
- The website's **Download for Windows** window now leads with the MIDI Keyboard Bridge (play Roblox and other games with your MIDI keyboard) and the system-tray option.

### Bug fixes
- MIDI Bridge → Musical controls: changing two settings one after the other (e.g. octave shift, then transpose) kept only the first change.

### Tested by hand (Piano Logger's owner, ST50 + Roblox, 7.2.1)
- Correct notes on every key and after the ST50's octave buttons; the 88-key layout incl. the underlined Ctrl keys; sustain via Space; Go to Roblox; Ctrl+Alt+End; system tray (logging continues, no keys sent, about 0 % CPU); latency median 9.2 ms / 95 % 52 ms.

## 7.2.1 — 2026-11-01 · MIDI Bridge fixes from real ST50 + Roblox testing

### Bug fixes
- **Wrong or missing notes in Roblox.** The tester's Roblox profile had three keys changed by accident in the Mapping studio (C4 = `s`, C#4 = `f`, D4 = no key), so C#4 played E5 and D4 played nothing. A Roblox / Virtual Piano profile that differs from the standard Roblox piano now shows exactly which notes differ, with **Restore the standard Roblox keys** (only those notes change; other settings stay).
- **Keys held forever with the keyboard's sustain on.** With *Hold notes until released*, every key stayed pressed in the game until sustain was switched off. The Roblox profile now uses **Press the sustain key: Space** (the game's *SpaceSustain*); a profile still set to hold notes offers **Use Roblox sustain (Space)**.
- **"Open" on the bridge chip did not go to Roblox** — it opened Piano Logger's own bridge settings. The chip now has **Go to Roblox** (brings the game window to the front; says so if Roblox is not running) and a separate **Bridge** button. Pressing **Start** in Live mode also switches to the game.

### UI & UX
- Overview: shows whether your keyboard's sustain is on and what it does in the game.
- A warning when octave shift or transpose is on, with **Play exactly what I press**.
- Mapping studio: every change (typed key, learn, fill, restore) can be undone once; learn mode stops by itself when you leave the window, so stray key presses cannot change keys.

### Desktop app 7.2.1
- New command `bridge_focus_target` (`pianoApp.keys.focusTarget`): finds a visible window of the target program and brings it to the front (never Piano Logger itself; does nothing for an "any window" target).

## 7.2 — 2026-11-01 · System tray, 88-key Roblox piano, tidier bridge controls

### New features
- **Keep running in the system tray** (Windows app 7.2): choose what the close button does — *Quit Piano Logger* (default) or *Keep running in the system tray*. Settings → Appearance & layout, or App features → Window. Alt+F4 and closing from the taskbar follow the same choice.
  - In the tray Piano Logger keeps logging what you play and updating your statistics. Nothing is drawn (WebView2 is told it is hidden and to use less memory). Measured on this PC: about **0.07 CPU-seconds per second and 84 MB** in the tray, against 0.51 and 566 MB with the window open and an animated background.
  - Click the tray icon (or start Piano Logger again) to bring the window back; right-click it for *Open* / *Quit*.
  - The **MIDI Bridge never presses keys while Piano Logger is in the tray**: it switches off when the window hides, cannot be started there, and the app refuses key presses then as well. Keep the window open (it may be minimised) to play Roblox or web pianos.
- **88-key Roblox / Virtual Piano profile** (A0–C8). The 15 notes below C2 (`1 2 3 4 5 6 7 8 9 0 q w e r t`) and the 12 above C7 (`y u i o p a s d f g h j`) — the underlined keys on the Roblox piano — are sent with **Ctrl** held. An unchanged 61-key profile is upgraded automatically; a profile you changed is left exactly as it was and an 88-key profile is added next to it. New *Fill: Virtual Piano (88 keys)* button.
- Safety: **Ctrl mappings are never sent to web browsers** (Chrome, Edge, Firefox, Opera, Brave, Vivaldi) because Ctrl+W, Ctrl+R, Ctrl+P… would close, reload or print the page; those notes are skipped there and listed in the Monitor.

### UI & UX
- MIDI Bridge → Musical controls: text fields and drop-downs no longer run outside their boxes (checked automatically at 1440×900 and 1000×760). Clearer names: “Minimum velocity (softer notes are skipped)”, sustain pedal “Off / Press the sustain key / Hold notes until released”, with a short explanation.
- Monitor → Diagnostics: long values wrap inside the table.
- Mapping studio: Ctrl / Shift keys are shown as ⌃ / ⇧ so they fit on the piano keys.

### Compatibility
- The tray option needs the 7.2 Windows app; the website and older apps do not show it. New app commands: `app_close_to_tray`, `app_quit`; `app_info` reports `closeToTray`. Uses Tauri's tray support and the `webview2-com` crate Tauri already uses.

## 7.1 — 2026-11-01 · MIDI Keyboard Bridge, 26 calm roll backgrounds, smoother playing

### New features
- **MIDI Keyboard Bridge** (navigation → MIDI Bridge): play Roblox Virtual Piano, web pianos and other games with your MIDI keyboard. Notes become computer-keyboard keys using a mapping profile. It uses the MIDI connection Piano Logger already has (no second connection), and logging keeps working exactly as before.
  - Windows app: real key presses (Windows `SendInput`, scan codes) go only to the window you choose — for example `RobloxPlayerBeta.exe` — and only while it is in front; never into Piano Logger itself. This is checked in the page and again in the app's Windows code. The website shows a preview of the keys instead.
  - Profiles: Roblox / Virtual Piano (61 keys, C2–C7), PC keys (2 rows) and Custom. Create, rename, duplicate, delete (two clicks), import (adds, never replaces) and export (versioned JSON).
  - Mapping studio with all 88 keys: click a key and type or press the computer key, or play a note and then press a key (learn mode). Search, shared-key and invalid-key warnings, quick fill.
  - Musical controls per profile: octave shift ±3, transpose ±24, note range, ignored notes, softest velocity, MIDI channel, split keyboard (lower part with another profile and octave), sustain pedal (hold a key, or keep keys down while pressed), soft pedal key and other controllers, retrigger.
  - Safety: notes on the same key are counted so no key gets stuck; every key is released when the target loses focus, the MIDI keyboard is unplugged, you press **Emergency stop** (also on a floating chip while the bridge runs), press **Ctrl+Alt+End** anywhere in Windows, or close the app. The bridge always starts switched off.
  - Monitor: every note, key, filter decision and refusal, plus the measured delay from the keyboard's MIDI timestamp to Windows accepting the key.
- **26 calm, animated piano-roll backgrounds**: misty morning haze, slow cloud drift, gentle rainfall, raindrops on glass, feather-light snowfall, cherry blossom drift, autumn leaves, dandelion seeds, sunlit dust motes, moonlit lake ripples, underwater caustics, bioluminescent plankton, deep-sea currents, soft coral light rays, slow gradient tides, flowing silk ribbons, ink in water, watercolour bloom, dreamy bokeh, floating light orbs, floating paper lanterns, moving cloud shadows, distant mountain mist, desert twilight haze, frosted glass reflections, gentle meadow breeze. All earlier backgrounds are still there.
- **Background studio** (Settings → Graphics → Browse & customise): categories, search, favourites, thumbnails, one live preview, and per-background speed, brightness, opacity, amount, colours (8 palettes, theme colours or the effect's own) and direction where it makes sense, with Reset to default. The backgrounds follow the quality preset, Lite mode, reduced motion and Motion Off (a still picture is shown).

### Performance
Measured in Chrome on this PC (GeForce GT 710), Aurora Glass theme, glow notes, animated roll background, 3-note chords every 150 ms for 8 s:
- Aurora background: **8.6–11.9 fps before → 16.9–20.2 fps after**; Synthwave grid: 10.4 → 17.4–20.7 fps.
- The music staff was redrawn for every note even when hidden; now once per frame and only when visible.
- The overview header forced a page layout for every note; now it updates ~8× a second and the note "pop" no longer forces layout.
- Rolling counters change once per frame (a chord rolls once), the navigation's live bars use transforms instead of heights, and small charts reuse their canvas instead of re-allocating it.
- While you play (and 2.5 s after) the decorative background animations pause and, below Ultra quality, the glass panels skip their background blur; dense note runs skip a costly glow shadow.

### Data safety
- Nothing in this update changes logged presses, statistics, badges, profiles or backups, and no database is changed. Bridge and background settings are stored separately (`pll.bridge`, `pll.rollfx`) and are included in full backups.

### Known limitations
- The website, the Electron app and Windows apps older than 7.1 show the bridge as a **preview only** (they say so); real key presses need the 7.1 (or newer) Windows app.
- Keyboard layout: mappings are stored as US English characters. The app sends each key's Windows key code (letters A–Z follow the letter printed on the key, so they work on every Latin layout), but digits and symbols use US positions:
  - **UK**: `@` (Shift+2) types `"`.
  - **German (QWERTZ)**: `@` → `"`, `^` → `&`, `*` → `(`, `(` → `)`, and the punctuation keys `; [ ] ' / - = \` differ.
  - **French (AZERTY)**: the top row types `& é " ' ( - è _ ç à` without Shift, so the Virtual Piano digits `1–0` and the symbols `! @ $ % ^ * (` all type something else (17 of the 61 Virtual Piano keys).
  - With a non-Latin layout active in the target window (e.g. Hindi/Devanagari), text fields receive that script; games that read key codes are not affected.
  Keep an English (US) layout active in the game window. Not tested on a real non-US layout (this PC has US, English (India) and Hindi installed, which all place these keys like US).
- Learn mode records letters by the letter printed on the key (so QWERTZ/AZERTY letters are learned correctly) and other keys by their US position.

## 7.0.4 — 2026-10-31 · Animations even with Windows animations off

### New features
- New option "Play animations even when Windows animations are switched off" (Settings → Motion & themes, and the Theme Studio's Motion section). When "Show animations in Windows" is off, Windows asks every app to reduce motion and Piano Logger follows it, so the animated piano-roll backgrounds (Aurora, Meteor shower, Synthwave grid, Ocean waves, Fireflies) stood still. With this option they, and the other animations, play anyway. Off by default; saved with your theme settings and backups.

### UI & UX
- Choosing an animated piano-roll background while animations are blocked now explains why it does not move and where to change it.

### Compatibility
- Page-only update: the installed Windows app downloads it from the website by itself and uses it from the next start.

## 7.0.3 — 2026-10-31 · Rounded window corners you can adjust

### New features
- Windows app: choose how rounded the window corners are, from 0 (square) to 24 pixels, in the Theme Studio or Settings → Motion & themes. Saved with your theme settings (and in backups).

### Bug fixes
- Windows app: the window corners were not actually rounded and the bottom corners were square, because the browser engine does not round the full-window background through the page's outer element. The page now scrolls inside its body, which is rounded, so all four corners are rounded (square while maximized).

### Compatibility
- Page-only update: the installed 7.0.2 Windows app downloads it from the website by itself and uses it from the next start.
## 7.0.2 — 2026-10-31 · Glass title bar with window controls

### New features
- Windows app: the window has no Windows frame any more. A glass title bar with macOS-style controls takes its place: red closes, yellow minimizes, green maximizes / restores.
- Drag the title bar to move the window, double-click it to maximize or restore. Every edge and corner still resizes the window. The window has rounded corners with a fine light edge on Windows 10 and 11 (square while maximized).
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
| 7.3.4 | 2026-11-03 | Tauri 2.12.5 (WebView2) | Branding & maintenance: new logo in the sidebar and app icons. Page 7.3.4. Built locally, not published yet. |
| 7.3.3 | 2026-11-02 | Tauri 2.12.5 (WebView2) | No code change. Page 7.3.3 (animated islands, sliding sections). Built locally, Published 2026-11-02 as `app-v7.3.3`. |
| 7.3.2 | 2026-11-02 | Tauri 2.12.5 (WebView2) | HTML5 drag and drop in the page works (`disable_drag_drop_handler`). Page 7.3.2. Published 2026-11-02 as `app-v7.3.2`. |
| 7.3.1 | 2026-11-02 | Tauri 2.12.5 (WebView2) | Page 7.3.1 (recording player, Stop fix). No change to the app's code. Built locally, not published. |
| 7.3.0 | 2026-11-01 | Tauri 2.12.5 (WebView2) | Voice Studio support: `PermissionKind::Microphone` allowed for the app's own page (other sites still get WebView2's prompt); CSP `script-src` adds `blob:` for the AudioWorklet. Page 7.3. Built locally, not published. |
| 7.2.2 | 2026-11-01 | Tauri 2.12.5 (WebView2) | Page 7.2.2 (glass drop-down menus). No change to the app itself. |
| 7.2.1 | 2026-11-01 | Tauri 2.12.5 (WebView2) | "Go to Roblox": `bridge_focus_target` brings the target program's window to the front. Page 7.2.1. |
| 7.2.0 | 2026-11-01 | Tauri 2.12.5 (WebView2) | Optional close-to-tray: the window hides, a tray icon (Open / Quit) appears, WebView2 is set invisible with a low memory target, and the bridge refuses key presses (`app-in-tray`). Alt+F4 / taskbar close follow the setting; a second start brings the window back. Page 7.2. |
| 7.1.0 | 2026-11-01 | Tauri 2.12.5 (WebView2) | MIDI Keyboard Bridge back end (`src-tauri/src/bridge.rs`): real key presses with `SendInput` (scan codes) only while the chosen target window is in front and never into Piano Logger; tracks every key it holds and releases them all on Emergency stop, the global **Ctrl+Alt+End** hotkey, window close and app exit. New commands bridge_send / bridge_release_all / bridge_foreground / bridge_status (`pianoApp.keys`). Uses the `windows` crate version Tauri already uses, so no new dependency. Page 7.1. |
| 7.0.2 | 2026-10-31 | Tauri 2.12.5 (WebView2) | Frameless, transparent window with rounded corners and the page's glass title bar (close, minimize, maximize/restore, drag, double-click); new window commands win_state / win_minimize / win_toggle_max / win_close / win_drag. Page 7.0.2. |
| 7.0.1 | 2026-10-31 | Tauri 2.12.5 (WebView2) | Fixed: "Start with Windows" used the same Windows start-up entry name as the Electron app, so with both apps installed one replaced the other. The new app now uses its own name (an entry 7.0.0 left behind for itself is cleaned up). Page unchanged (7.0). |
| 7.0.0 | 2026-10-31 | Tauri 2.12.5 (WebView2) | New desktop target in `Piano-Logger-tauri/`. Same page and the same `window.pianoApp` features as the Electron app (SoundFonts, backups, always on top, start with Windows, updates of the page from the website). MIDI is allowed without a pop-up. The page is served with a Content-Security-Policy (no frames or plugins); remote websites open in the browser. 64-bit only. |
| 6.1.0 | 2026-10-27 | Electron 43 | Backups of any size, more memory, crash note. Unchanged by 7.0 (`Piano-Logger-app-6.1-files/`). |
| 6.0.0 | 2026-10-27 | Electron | First Windows app (`Piano-Logger-desktop-app/`). |
