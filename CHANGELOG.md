# Changelog — Piano Logger

Versions of the Piano Logger page (`APP_VERSION` in `piano-logger.html`). The same history is shown in the app under **What's new**.
Desktop app versions are listed separately at the end.

## 9.2.4 — 2026-11-28 · Only your chosen MIDI port is used, and lighter on your computer

### Windows app: MIDI ports
- Only the MIDI port you choose is opened. The app now talks to Windows MIDI itself; before, the browser engine inside it opened every port and never released it, which disturbed other programs.
- Your choice is remembered after a restart. While that port is unplugged, no other port is opened instead.
- Fixed: "Connected" pop-ups repeating thousands of times (seen in an unreleased 9.2.4 build).
- The status line shows only the port that is being listened to.
- "All MIDI inputs" still opens every port. Other programs still list all ports, because Windows lists them.

### Lighter on your computer
- Lite effects now removes all blur and stops the endless animations. Endless animations also pause while the window is behind another one. Less redrawing while nothing is played.
- Measured on one PC (2 logical processors, empty profile, idle, share of one processor core): Lite effects off 62 % in front and 26 % behind; Lite effects on 19 % and 18 %. Memory about 550 MB and 440 MB. Measured before the last small redraw change.

### Cloud sync
- Fixed: "Sync failed, will retry". The 9.2.3 web page carried an unfinished upload of other data (sung notes, Sheet Music, takes, voice recordings, personal notes, profile picture, quests, gestures, large records) that the online service cannot accept yet. It is switched off; those items are not uploaded, and the preview says so. Files it had already placed in storage are left there.

### Web browser
- A web browser still opens every MIDI port; the port selector there only chooses which notes are counted.

## 9.2.3 — 2026-11-24 · Every key press, profile and voice history in the cloud, restorable on a new computer (installer published)

Status: the Windows installer 9.2.3 is published as release `app-v9.2.3` (SHA-256 `407483D5D310EE3739F2A7DE0BE7694BFE6B7969DA039E69FBC5AA089EA9224E`). It was tested on an isolated copy of the owner's data and installed on the owner's computer. Nothing was uploaded to the owner's account yet: that needs the owner's authenticator code.

### What is new
- **Every key press (optional toggle).** Approved uploads send each day's key presses as compressed files. Isolated test: 940,452 presses in 27 day files, checksums matched.
- **Profile, notes and voice history (optional toggle).** Profile, journal, routines, badges, best scores and voice session records. Covered by the isolated full-sync test (7 of 7 checks passed).
- **Restore from the cloud.** Restores into a fresh profile: 940,452 presses, 27 days, 67 voice sessions (restore test, 6 of 6 checks passed).
- **Exact preview.** Counts and storage sizes per category, read from the same data the upload uses.

### Not in this release
- Sung notes, Sheet Music, saved takes, voice recordings (sound and details), voice settings, personal notes, profile picture, quests and gestures are not uploaded by 9.2.3. They are listed in the preview as not in this version. See the Unreleased section for the page that adds them.
- Themes, graphics and layout stay on the computer by design.

## 9.2.2 — 2026-11-24 · First-upload preview for Cloud sync (nothing uploads until you approve)

Status: the page, the Windows shell (version 9.2.2) and their tests are built and tested on an isolated copy of the owner's data. The installer is built but **has not been installed**, and nothing is published or pushed.

### What is new
- **First-upload preview.** In Cloud sync, "Make a preview" reads the press log on this computer (read-only) and shows the exact number of days, daily totals and practice sessions that would be uploaded, the presses counted, an estimated upload size and the number of requests.
- **Compared with the copy online.** For this computer: days unchanged, changed or new, and practice sessions already online. Days also recorded by your other computers are flagged as a possible double count.
- **Approval gate.** Nothing uploads until you tick the box and press "Approve and upload". Approval re-reads the press log and refuses if anything changed since the preview.
- **Sync is off by default.** A new profile starts with "Sync this device" OFF (it used to start ON).
- **Voice history, settings and Every key press are not uploaded in this release.** Their switches are shown as "not in this release yet" and cannot be ticked.

### Not changed
- Key presses, the local press log, backups and the sync engine's upload code (daily totals and practice sessions only).

## 9.2.1 — 2026-11-23 · Two-step sign-in for Cloud sync

Status: the page, the phone page, the Windows installer (`installers/release-9.2.1-2026-11-23/`) and their tests are built and tested; the installer has **not been installed** and nothing is published or deployed (this line is replaced by the release date when the owner approves publishing). The matching server rule (`cloud-sync/proposed/0005_mfa_enforcement.sql`) is **proposed and not applied**. 9.2.0 was never released, so 9.2.1 is the first public version with Cloud sync. The Rust shell and its security rules are unchanged from the 9.2.0 build; only the page inside the app changed.

### What is new
- **Two-step sign-in (authenticator app).** If your account has an authenticator app, Piano Logger asks for its 6-digit code after any sign-in (password, Google or a password-reset link) before anything of your account is shown or uploaded; Cloud sync cannot start until the code is checked. *Cloud sync → Account security* shows whether it is on, lets you add an authenticator (QR code or typed key) and remove one (you must type REMOVE). Piano Logger never removes an authenticator by itself. Five wrong codes lock the code box for 30 seconds.
- **Password reset with two-step sign-in.** On the Piano Logger Cloud page the link from the reset e-mail asks for the authenticator code first, and only then lets you choose a new password.
- **Steadier sign-in.** A slow or stuck connection now ends with a clear message and an automatic retry instead of waiting for ever. After signing in the message says whether Cloud sync is on or off for this computer.

### Tests
- Two-step sign-in: fake service 44/44; real service with disposable accounts 25/25; real page 31/31 (+4/4 for an account without an authenticator); phone page 18/18; compiled shell (isolated copy) 33/33 at 9.2.0. Earlier suites re-run: sync page 47/47, phone page 40/40, core 97/97.

## 9.2.0 — 2026-11-23 · Optional Cloud sync: your practice on your phone, and a safe copy if you change computer

Status: the page, the cloud service, the phone page, their tests and the Windows installer (`installers/release-9.2.0-2026-11-23/`) are built and tested; the installer has **not been installed or run on a normal profile yet** and nothing is published (this line is replaced by the release date when the owner approves publishing). Needs the Windows app 9.2.0 (the older app's security rules do not allow the connection; it shows "update the app").

### What is new
- **Cloud sync (optional).** Left menu → *Cloud sync*. Sign in with an email and password or with Google. Your total for each day and your practice sessions are saved online. Nothing is sent unless you sign in; signed-out users make no network request at all (checked by a test).
- **Honest status.** A small light under the left menu: *Synced*, *Syncing* (with "x of y days" for the first upload), *x days waiting*, *Offline*, *Sync failed, will retry*, *Sign in to sync*, or *Update the app to use cloud sync*. *Synced* appears only after the copy online was read back and matched the local one.
- **Offline first.** Playing is never delayed by the cloud: syncing only reads your history in small steps and waits while you play. It resumes from its last confirmed step after a restart or a lost connection, retries with growing pauses, and never uploads the same day twice unless it changed.
- **Phone page.** *Piano Logger Cloud* on the official website: sign in with the same account to see total time and key presses, today / this week / this month / this year, streaks, a practice calendar, sessions and devices. Designed for phones first (bottom tab bar, large touch targets), works on computers too. The numbers use the app's own rules (tested for exact agreement, including Monday-based weeks and the streak rule).
- **Optional extras (Cloud sync window):** *Profile, notes and voice history* (name and look, journal, routines, badges, best scores, voice sessions, never the sound; at most every 10 minutes) and *Every key press* (one compressed file per day, about 1 MB online per 100,000 presses; off unless you switch it on).
- **Restore from the cloud** (new or reinstalled computer): asks for a backup first, then adds your history with the app's own duplicate-safe import. This computer takes over the old device's identity so totals are not counted twice. **Download my cloud data** saves a file that *Data & backup → Restore* reads (tested).
- **Devices and account.** See, rename and disconnect devices; *Sign out*, *Sign out everywhere*; *Delete my cloud data* (also removes the press files and switches syncing off here) and *Delete my account*, both only after typing DELETE. Your history on this computer is never changed by any of these.
- **Windows app:** the app's security rules now allow the cloud sync service, and Google sign-in opens your normal browser and listens on `127.0.0.1` for the return trip (one answer, 3 minutes, plain token characters only).

- **E-mail links work.** The link in a confirmation e-mail signs you in to Piano Logger Cloud ("Your email is confirmed"); the link in a password-reset e-mail opens "Choose a new password" (at least 10 characters). Expired or used links say so in plain words and a made-up link is refused by the service. The sign-in token is removed from the address bar at once. (Both need the project's e-mail delivery to be set up; the page side is tested with real tokens.)

### Checked before release (exact, run on 2026-11-23)
Security rules 81/81 · sync engine and network client 97/97 · raw-press files 12/12 · app page against the live service 47/47 · Google sign-in page flow (with a stand-in for Google) 25/25 · restore on a new computer 32/32 · dashboard statistics equal the app's own 16/16 · dashboard files and policy 8/8 · dashboard in Chrome at phone and desktop sizes 40/40 · account isolation and e-mail links 21/21 · Windows shell reader of the Google return trip 5/5 · **the real compiled Windows app (isolated copy) 25/25**: capability flag, security rules allow the cloud service and still block other websites, the Google return listener (hostile input, cancel, replacement), sign-in and a real sync from inside the app. **Not yet run:** a real Google sign-in, real e-mail delivery, real phones, MIDI recording offline in the real app, restore on a clean real-app profile.

### Safety decisions
- Nothing is deleted online by itself: when days or voice sessions disappear from this computer, the window asks whether to delete them online too.
- A computer whose history belongs to one account is not given to a different account by accident (a clear choice is required).
- Backup files never contain the sign-in, the device number or the sync switches.
- Every write goes through server functions that check who you are and that the device is not disconnected; the database refuses anything else (direct table writes are not allowed at all). One person can never read another person's rows.
- A page update cannot make an old app connect: the app must carry the new security rules (`caps.cloudSync`).

### Honest limits
- Two computers used at the same time each keep their own key presses; the phone page adds their totals. Restore is for a new or reinstalled computer, not for merging two live ones.
- A session that runs past midnight is shown as one session per day in the cloud.
- Sheet Music, voice *recordings* (sound), themes and graphics are not part of Cloud sync.
- A real Google round trip and e-mail confirmation delivery depend on the project's settings (see the master record); the Google page code is tested with a stand-in for Google.
- Free hosting tier: the online project can pause after about a week without use and has no automatic backups; the *Download my cloud data* button is the user's own backup.

## 9.1.0 — 2026-11-18 · Light and dark sheet music, 3D hands, piano videos and a bigger heatmap

Released 2026-11-18 as the Windows app `app-v9.1.0` and on the website (the earlier "not published" wording was correct until the owner approved publishing on 2026-11-18).

### Sheet music and PDF
- **Light or Dark score** (Sheet Music → Score → Appearance, and MIDI → PDF → Appearance). Dark is a real charcoal page with ivory notes in the saved PDF (not a viewer setting); Light is white paper with black notes. The choice is saved for Sheet Music only and never changes the app theme. Staff lines, chord names and the footer have their own softer tones in both looks.
- **Rests are engraved properly:** every rest starts on a multiple of its own length and never crosses a beat; no dotted rests between beats. *Cause of the old look (verified):* v9.0.0 allowed dotted rests to start off the beat, which produced strings like “𝄿· 𝄿 𝄿· 𝄿” (the owner's reference page 4).
- **Combine empty bars** (on by default): bars that are silent in every staff are written as one bar with the number of bars above it (the silent lead-in of a recording becomes one “2” bar). No bar is removed; bar numbers and timing stay exact. Turn it off to show every bar.
- **Balanced pages:** systems are shared out evenly between pages, and a slightly tighter line spacing (never under 88 %) is used if it saves a whole page. The owner's 15-bar reference went from 4 pages (last page two lines) to 3 balanced pages (5 / 5 / 4 lines).
- **Chord names never touch the music:** each name is placed above the highest note, stem or beam under it, and names on one line never overlap.
- Score and PDF controls grouped into tidy cards (Reading & layout, Show, Appearance, View / Paper).

### Falling notes
- Redesigned scene: softly lit rounded keys, pressed keys move down and take the hand colour, a gentle light above each sounding key, rounded note bars with a lit leading edge, octave guides, a progress line.
- **Your own hand colours:** right-hand and left-hand colour pickers, five presets (Crimson / Periwinkle, Teal / Coral, Gold / Violet, Ice / Amber, Mint / Rose), colour by hand or by track, dark or light background, glow on/off, a contrast warning, Reset. Saved in `pll.visualizer` (separate from the theme and the recording).
- **Hands (second pass, after the owner's reference video):** two life-size, see-through 3D hands (palm, thumb and four fingers with three joints each; a hand spans about 4 white keys) seen from a gentle top-down camera, drawn as one shape with a single clean outline, fading out towards the wrist, with finger numbers on the pressed keys. Styles: Hologram (glowing outline, light fill, soft scan lines), Glass, Silhouette, Sketch and Soft; own colour, see-through amount and size. Smooth motion: each finger eases from its resting place onto its key and back (smootherstep), presses in ~60 ms, and the hand glides between positions only after its notes are released. Positions are a pure function of playback time, so pause, seek, speed and loop stay in sync. **Fingering is a transparent rule-based guess**; a MIDI file does not record the performer's fingers. Quality Automatic / High / Low; Hands off keeps the 2D view.
- **Six visualizer themes** (Aurora, Neon, Blocks, Ember, Midnight glass, Paper), each with its own notes (soft, outlined pill, flat block, hatched, glass), keyboard (classic, dark, warm) and suggested hand style, chosen from picture cards drawn by the real renderer in one ⚙ Settings drawer (themes, colours, hands, which hand plays, display). Panels slide in gently and stay still with reduced motion.
- Toolbar in groups: Playback (with loop), View (keys, look-ahead, full screen), ⚙ Settings, ✋ Hands on/off, Export video. Toolbar text is only rewritten when it changes (steadier frames).

### New: export a piano video (MP4)
- **Export video…** in the falling-notes view records the scene (notes, keyboard, 3D hands) with the app's own sound into a real **MP4: H.264 video + AAC sound**, checked at run time (WebView2 / Chrome 126+). Quality 240p / 360p / 480p / 720p / 1080p; shape 16:9, 9:16 or 1:1 (re-composed, never stretched); 30 or 60 fps; optional title and performer. **The video has its own look settings with a live preview** (theme, colours, hands; remembered for the next video, never changing the screen). Size estimate shown first; recordings over ~550 MB are refused.
- Recording is real time, from the start to the end (plus 1.2 s for the last notes to ring out). If the music normally goes to an external MIDI device, the video uses the built-in instrument and says so. Cancel stops at once and saves nothing. The app checks the finished file has a picture track (and a sound track when asked) before offering to save it, and records again by itself (up to two times) if the video encoder started late.
- If a computer cannot record MP4, the file is saved honestly as **.webm** (never a WebM renamed to .mp4).
- Desktop app: the native Save dialog now also accepts `.mp4` / `.webm` (Rust `music_pick` kinds `mp4` and `webm`; write limit raised from 200 MB to 600 MB).

### Third pass (owner review, 2026-11-18)
- **Keyboard size** in Falling notes: Notes used, 44, 61, 88 or 108 keys; the change glides (smootherstep, ~0.5 s) and the keys, notes and hands scale together (more keys → smaller hands).
- **Hand motion:** a wave through the fingers (each finger starts towards its key 12 ms after the previous one; the press stays on the note time), the hand lifts a little while travelling and breathes slowly at rest.
- **Press effects:** soft light, ring and sparks where notes land and a bright landing edge on the falling note — pure functions of time, so they are identical on screen and in the video; off in Low quality and Lite effects.
- **Frame-exact MP4 export** (WebCodecs): *bug (VERIFIED on the owner's 720p file):* the real-time recorder dropped sound and picture on a slow PC (26.1 s of sound, 27.6 s of picture at 18.9 fps for a ~31 s recording), so the sound ran ahead and ended early. Now the music plays once while an AudioWorklet captures every sample with its audio-clock frame, then every video frame is drawn for its exact time and encoded with an exact timestamp, and a built-in MP4 writer joins H.264 and AAC. Tested: sound within ~10 ms of each note with no drift, picture frame-exact, both tracks the same length, 60 fps exports contain exactly 60.00 frames per second. The real-time recorder remains only as a fallback for WebViews without WebCodecs.
- **Export dialog** sits on a dimmed, blurred backdrop over the whole visualizer (it used to overlap the toolbar); **drop-down menus** open inside full-screen views; the **instrument list** and the **Sheet Music playback bar** are solid (they let the page show through).
- **Sight-reading course** reorganised: 5 levels, 7 modules, 48 lessons (was 29; old lesson ids and progress kept), a new minor-keys module (the generator now writes natural minor melodies with harmonic-minor dominant chords), 2/4, 12/8, more keys, intervals, longer pieces and an exam-style test; a **Practice** item per module and a **Mixed review** that write a new sheet every time from the lessons already passed; **Dark sheet**; a tidier lesson screen.

### Fourth pass (owner review, 2026-11-18)
- **MIDI → Markdown** (new tab beside MIDI → PDF): a plain-text .md in simple words — summary (key, tempo, length, chords used), a "how to read this" legend, a bar-by-bar table with each hand's note names and the chord written *under the exact notes* where it is played, the chord progression, and the music described in words. Simple / Detailed / Complete; letters, Do Re Mi or no octave numbers; # b or ♯ ♭; copy or save as .md (native Save dialog in the app).
- **Smarter chord engine** (PDF and Markdown): notes weighted by length and beat position, key-aware choice between close candidates, beat-by-beat timeline with hysteresis (no flicker), triads, sevenths, sus, add9, sixths, diminished, augmented, power chords ("A power chord"), and slash chords (C/E).
- **Firm panels:** the Settings drawer, Export video window and instrument list use their own solid, blurred style, so a skin's see-through .glass rules can no longer make them transparent. The export window blurs the scene behind it.
- **Keyboard sizes and hands:** key proportions never stretch; 44 keys is centred at a natural size; hand size follows the key width (not too thin at 108, not flat at 44).
- **16 visualizer themes** (6 neon dark, 5 calm, 5 classic), more calm note-colour presets, 8 hand-colour swatches, custom colour per hand.
- **Four lifelike 3D hand models** — Realistic, Pianist, Sculpture, Anatomical — with a skin-tone choice.
- **Fix which hand played which notes** (Falling notes → Settings): drag a box over notes and mark them Left or Right; applies to hands, colours, score and PDF; saved per piece on this computer; resettable.
- **Video export in the background:** a bottom-right card with three steps, then the Save dialog; faster (CPU-canvas encode, lighter hand drawing, cached background) while sound, picture and frame rate stay exact.
- **Start-up and exit screens** show the running version and a "what is new" line; Settings → Start-up & exit chooses 7 dark or 7 light themes (or Automatic), with previews.
- Tests: new `tests/v910/pass4.test.js`; `viz`, `video` and `practice` tests updated (16 themes; the easiest practice lesson has only about ten possible sheets, so a repeat is allowed).

### Fifth pass (owner review of the installed app, 2026-11-18)
- **Fixed: exported videos had no hands** even with "Show hands" on. Cause: the video draws the hands on a half-size layer, and the hand's box was cut to the half-size layer using full-size numbers, so anything in the lower half of the picture (where the hands are) was cut to nothing. Hands now appear in every video for all nine hand styles; a new test counts the hand pixels in the video picture so this cannot return unnoticed.
- **Fixed: dark outlines on the lifelike hands came out white** (a colour-reading helper had lost its backslashes).
- **Panels now match the app's drop-down menus:** the Settings drawer, Export video window, instrument list and the toolbar use the same frosted glass (see-through, 28 px blur, the app's own colours, 14 px corners). Instrument rows are 9 px rounded, the row under the pointer or the chosen one gets the colour gradient, and the chosen one a tick, like the other drop-downs. Works in light themes, High contrast and No-blur modes.

### Statistics and the whole app
- **Practice activity heatmap:** 52 weeks by default (26 as an option), day squares sized from the card width (14–22 px; the grid scrolls sideways on a narrow window instead of shrinking), month and weekday labels, today marked, a summary (total presses, active days, best day). Same daily totals and dates as before.
- **Comfort pass:** the smallest labels across the app (Overview stat labels, Profile achievements and XP, table headings, helper text) are lifted to 12–12.5 px; more room in table rows; one consistent focus ring for keyboard users; form fields at least 32 px high. Measured with an audit script: tiny labels (<11 px) on Overview 57 → 39, Statistics 27 → 15, History 9 → 0; short form fields on Practice 7 → 0.

### Limitations (honest)
- The hands are stylised and the fingering is a heuristic. Tested: single notes land within 12 % of a key width; on a very wide 7-key chord the thumb can be up to 42 % of a key width off.
- Video export plays the music once in real time to record its sound (so it takes at least as long as the music) and keeps the recording in memory until it is saved (hence the 550 MB guard). Frame rate depends on the computer: 14–21 fps in the software-only test browser, about 18 fps measured in the desktop app on this PC at 240p (target 30).
- Not tested on the owner's ST50 or GT 710 hardware in this session (simulated MIDI only).
- The comfort pass lifted small text and spacing across the app but did not redesign individual pages.

## 9.0.0 — 2026-11-18 · Sheet Music: score, PDF, falling notes and a sight-reading course

Released 2026-11-18 as the Windows app `app-v9.0.0` and on the website (corrected 2026-11-18 in 9.1.0: this line said "not published").

### New: Sheet Music workspace (menu → Create → Sheet Music)
- Five sections: **Library**, **Recorder**, **Score**, **MIDI → PDF** and **Sight reading**. A transport bar (Play/Pause, Stop, position, speed 50–150%, loop, sound, falling notes) stays at the top while a piece is open.
- **Recorder** (moved from Studio, same take editor with timeline, loop, trim, saved takes and “Practice this take”):
  - **Save MIDI…** opens a Save dialog to choose the folder and file name. In the Windows app this is a native dialog; in Chrome/Edge the browser's file picker; other browsers fall back to the Downloads folder and say so. Cancelling keeps the take. File names are cleaned (no `\ / : * ? " < > |`, at most 120 characters, Unicode kept).
  - Takes now keep the sustain pedal (CC64) and program changes, and note times come from the MIDI event's own timestamp, so chord notes stay together even when the page is busy.
  - **Import MIDI…**, **Open as a score**, **Save to library**, **Falling notes** and **Convert to PDF** for the take.
- **MIDI files:** a new reader and writer. Reads formats 0 and 1 (and 2), running status, note-on with velocity 0, tempo changes, time and key signatures, program changes, controllers, track names; damaged or cut-off files open as far as they can be read with a message; random bytes are refused with a clear message. Writes format 1 (a tempo/time/key track plus one track per part) with programs and the sustain pedal. A written file reads back with the same notes, velocities and timing.
- **Playback:** the file's own instruments (General MIDI programs) or **one instrument for all**, chosen from a searchable, grouped list of every sound the app can play (built-in sounds, grand pianos, the 128 General MIDI instruments, your SoundFonts and Synth Lab sounds). Choosing one plays a short preview. The sustain pedal in a file holds notes as it should. Percussion (channel 10) is not played. Stop, pause, seek, speed changes, leaving the page and closing the app release every note. **Playing a file never adds key presses to your statistics.**
- **Score:** a grand staff drawn as sharp vector graphics: treble and bass clefs, brace, key and time signatures, bar lines and numbers, ledger lines, sharps/flats/naturals (remembered within a bar), notes from whole to 32nd, dots, ties across bar lines, beams, flags, rests, triplets, optional chord names (C, Am, G7, …) and note names. Controls: rhythm grid (automatic, quarter … 32nd, triplets, finest), tempo for free recordings, hand split, layout (grand staff, one staff per track, a single track), key (automatic or chosen), sharps or flats, zoom/fit width, follow playback. The note being played is highlighted. These settings change only how the music is written, never the sound or the MIDI file.
- **MIDI → PDF:** a real vector PDF (text stays text) with as many pages as needed; A4 or Letter, portrait or landscape, small/normal/large staff, title and composer line, page numbers and title on every page; whole systems never split across pages. Page previews (first six pages).
- **Falling notes:** a full-window view with a keyboard at the bottom and falling bars that land on their keys at the right moment, in hand or track colours, with play/pause, seek, speed, key range and look-ahead. It shares one clock with the sound and the score cursor. Esc closes it.
- **Sight reading:** a course in six stages (first contact with the staff, rhythm, moving around the keyboard, two hands, intermediate, advanced) with 29 lessons. Each lesson explains one idea, shows a sheet and asks you to play it on your MIDI keyboard. Every sheet is newly written by rules (key, scale, chord progression, a two-bar motif repeated with variation, phrase endings on the home note, allowed rhythms and range) and checked so it is valid and not a recent repeat. Modes: **at my own pace** (waits for each note or chord) and **with the beat** (count-in, timing window, timing offset for slow devices). Kind, specific hints (“That note is one step too high…”), a hint button that lights the key, Listen, Easier, Harder, Same again, New exercise, First phrase only, Keep this sheet, and a summary with what to review. Progress is saved.
- **Storage:** a new separate database `PianoLoggerSheetMusic` (library and learning progress) and the setting `pll.sheetMusic`. The lifetime log and its database are not changed. Library and progress can be exported and imported as one `.json` file (importing adds, never replaces).

### Moved and improved
- The **calendar heatmap** moved from Studio to the top of **Statistics** as “Practice activity”, with a legend and a read-out of the day under the pointer, tapped, or chosen with the arrow keys. Same data and colours as before.
- The small **staff** moved from Studio to Sheet Music → Recorder (“Live staff”) and only draws while it is visible (it used to redraw for every note even when hidden).
- **Version chip:** a filled inside in the theme's own colours with a calm accent edge instead of the pulsing rainbow ring with a black centre; the edge turns slowly only when motion is allowed (not in Calm, Lite, reduced motion or animations off); high-contrast themes get a plain chip.

## 8.1.2 — 2026-11-13 · 34 new Carbon & Steel themes and gently animated backgrounds

### Themes
- **New collection: Carbon & Steel (34 themes)** in the style of Carbon Black: Carbon Noir, Carbon Crimson, Carbon Cobalt, Carbon Emerald, Carbon Amber, Carbon Violet, Carbon Teal, Carbon Rose, Gunmetal, Anthracite, Jet Black, Charcoal Smoke, Shadow Steel, Midnight Carbon, Obsidian Dots, Graphite Pulse, Titanium Night, Smoked Chrome, Cold Iron, Rust Steel, Ember Carbon, Lime Carbon, Ultraviolet Carbon, Ice Carbon, Bronze Carbon, Sapphire Carbon, Blood Moon Carbon, Nightshade, Carbon Ghost, Black Pearl, and four light themes (Carbon White, Ceramic Grey, Frost Steel, Pearl Carbon).
- Ten new background patterns (`cb-drift`, `cb-pulse`, `cb-twin`, `cb-hex`, `cb-dust`, `cb-plus`, `cb-ring`, `cb-weave`, `cb-sparse`, `cb-micro`): dots, a honeycomb of dots, tiny rings, crosses, dust, a carbon twill and a few bright spots. They move very little (slow drift, breathing, or two layers twinkling in turn), use only transform and opacity, and stop in Calm mode, Lite effects, Reduced motion and Page animations Off.
- Carbon Black now uses the same slowly drifting dots (same look as before, now with a little life).
- Theme Studio now shows 144 new themes in 12 collections, plus the classic ones (216 in all).

### Piano roll
- The white stars in the Meteor shower, Synthwave grid and Fireflies backgrounds now twinkle softly; each star has its own cycle of 3 to 10 seconds.

## 8.1.1 — 2026-11-13 · Profile picture in the header, faster note capture, pitch window

### Start-up and exit screens (desktop app)
- **Fix:** when the app opened it briefly showed the old, unstyled page ("Piano Lifetime Logger", Lv 1 Beginner) before the real layout was built. The cause is that the layout is built by the page's script after the page markup is first drawn. A start-up screen now covers the page until it is ready.
- **New start-up screen:** an animated logo, version and progress bar for about 4 seconds (counted from when the page starts loading).
- **New exit screen:** closing the app shows an animated "Exiting Piano Logger" screen for about 2.5 seconds, then the app closes. It is not shown when *keep running in the tray* is on (closing then only hides the window). If the screen cannot be shown, the app closes itself after 4.5 seconds; closing a second time closes it at once.
- The website does not show these screens. Reduced-motion settings stop the animations.

### Profile and statistics
- Your profile picture now shows beside your level in the top-right chip, instead of the rank emblem. The emblem still shows when no picture is chosen.
- The numbers in **Sung Note Statistics** now slide when they grow, like the key-press numbers.

### Voice Studio (desktop app)
- **Accumulated Notes keep up with your singing.** A note appears in the list as soon as it is steady (it used to appear only after it ended). Notes as short as 70 ms are kept (before: 200 ms), and the same note sung again after a new syllable or a pause of 80 ms or more counts as a new note.
- **Fewer wrong notes.** A pitch must move clearly past the halfway point between two notes before it becomes the neighbouring note, short dropouts no longer restart a note, and a short wobble inside one breath (under 120 ms, no new syllable) is not counted as a note.
- The note tiles have a soft pulsing glow (off in Calm mode, Lite effects, Page animations Off and reduced motion).
- **New setting: pitch window.** Choose how many seconds the pitch chart shows, from 1 to 60 (default 6), in the drop-down above the chart. It is saved in the voice settings (`win` in `pll.voice`) and also sets the span of the Approximate notes strip.
- The Sung Note Statistics use the same 70 ms rule (their gate is shared with the list), so very short sung notes are now counted.

### Graphics
- The **Cursor light** size and strength controls now exist only in the desktop app. The website always uses the default light (520 px, 100%), even if other values were saved earlier. Saved values are not deleted.

## 8.1.0 — 2026-11-13 · Cursor light, quality fixes and Lite effects

### Cursor light (website and desktop)
- A soft multi-colour light now follows your mouse, behind the panels, on every page. It uses your theme colours, moves smoothly, and never blocks clicks, the piano or MIDI.
- **Settings → Graphics → Cursor light:** choose the light's **size** (200–900 px) and **strength** (0–150%). 0% turns it off; **Reset light** restores the defaults (520 px, 100%). Both values are kept in the existing graphics settings (new optional keys `ambSize` and `ambStr`); no existing data or setting is changed.
- It follows the quality preset: off on Low, gentle on Medium, fuller on High and Ultra (Custom follows Glow strength). It is also off with Lite effects, Calm mode, Motion Off or Reduced, high contrast, and the system's reduce-motion setting (unless you chose to play animations anyway). Changing quality applies it at once, no restart.
- The soft light inside each panel is stronger on High and Ultra.

### Quality and motion fixes
- **Lite effects** (the switch in the piano toolbar) overrides every quality preset: flat panels, no glass blur, no glow, no cursor light. Choosing Medium, High or Ultra now switches it off and tells you; before, Ultra looked unchanged while Lite effects was on. Low leaves it as it was.
- Settings → Graphics shows a warning with a **Turn off Lite effects** button while it is on.
- Choosing Low no longer loses your **Expressive** motion setting. It is remembered (new optional key `motionPref` in the existing Theme Studio settings) and comes back when you choose Medium, High or Ultra, instead of becoming Balanced. Picking a Motion level yourself clears the memory.

## 8.0.2 — 2026-11-11 · Website backup Save dialog

- On the website, **Create backup file** opens the Save dialog straight away, when you click it. Cancelling saves nothing. The Windows app is unchanged from 8.0.1.

## 8.0.1 — 2026-11-11 · Backup save location

### Backups (website and desktop)
- **Create backup file** now asks where to save the file (a Save dialog). Before, the file was downloaded straight to the Downloads folder.
  - Windows app: the native Save dialog; the file is written to the folder you choose, in pieces, so large backups work.
  - Chrome and Edge on the website: the browser's Save dialog.
  - Other browsers keep the normal download.
  - If you cancel the dialog, nothing is saved and the backup is not counted.
- Automatic backups are unchanged: they still go to the backups folder chosen in the app.

## 8.0.0 — 2026-11-11 · Sung note statistics, living backgrounds, voice tools and profile pictures

Released 2026-11-11 (GitHub release app-v8.0.0). Items marked *desktop* are in the Windows app only.

### Sung Note Statistics (desktop only)

- *desktop* **Total notes sung** card in Statistics → Voice Statistics, for the period you choose (all time, today, last 7 or 30 days, a month, a year or a custom range). It counts stable detected notes, not microphone frames.
- *desktop* A **Sung Note Statistics** section above Key Statistics: summary cards (total and distinct notes, most sung note, total and average note length, singing sessions with notes, lowest and highest note); charts for the most sung notes, pitch classes, octaves and activity over time (day, week or month); and a sortable table with one row per note (count, share, total, average and longest duration, average frequency and cents, average confidence, sessions, first and last time).
- *desktop* Each stable note is counted once while Voice Studio is in the Singing style (speaking never counts). It uses the same detection and acceptance rules as Approximate Notes and Accumulated Notes (at least 200 ms, confidence 0.6), so the three always agree. Note names are the nearest piano note to the measured pitch, an estimate and not what was meant.
- *desktop* Kept in its own local database (PianoLoggerSungNotes). Only note information is stored, never sound, and nothing is uploaded. Included in backups (older backups still restore and leave the data as it is; restoring twice never duplicates). Clear Notes in Voice Studio never removes it.
- The website does not run, store or show any of this; its "Download app" panel describes the desktop features.

### Statistics and look (website and desktop)

- The page is now called **Statistics**, with two clearly separate sections: Voice Statistics and Key Statistics.
- The version is shown in a small chip under the Piano Logger name (the "Aurora Glass Studio" line is removed). The chip has a dark centre and a slowly turning, pulsing rainbow border (website and desktop).
- Sung Note Statistics: more space above Key Statistics under the "How to read this" note.
- Pressed black keys show their note label inside the key with clearer contrast; white keys are unchanged.
- Theme Studio favourite stars sit clear of the card text.

### Font library (desktop only)

- *desktop* Settings → Appearance & layout → Font: 88 bundled open-licence font families (sans, serif, display, rounded, handwriting, monospace, futuristic and pixel) plus 12 Windows system fonts, with search, category filter, live preview and Apply / Reset font. The chosen font applies app-wide and is kept after restart; it is included in backups. Font files are bundled in the app, so nothing is downloaded from the website. The website is unchanged.

### Backgrounds (website and desktop)
- Cloud drift, rain, raindrops on glass, snow, cherry blossom, meteor shower, synthwave grid, ocean waves, fireflies and nebula move with more natural variation and no fixed repeating cycle.
- Every Roll background fades in when chosen. Saved settings (intensity, speed, palette, opacity, density, quality) are unchanged. Reduced motion still stops the animation.

### Fixes (website and desktop)
- Voice Studio noise reduction also works when the page is opened as a file in the browser (it had no effect there).
- History and Statistics no longer keep "Live activity" highlighted in the sidebar.
- What's New showed "Invalid Date" for a build whose release date was not set yet; it now shows the text as written.

### Voice Studio (desktop)
- Noise reduction: Off, Light, Balanced and Strong, with a gate hold control.
- Accumulated Notes: stable notes are kept in order until cleared, with an on/off switch, and JSON, text and Markdown export (note information only). The Save dialog lets you choose the folder.
- Five-band EQ, a compressor, five presets (Natural, Clean, Warm, Clear Speech, Studio Vocal) and a reset to recommended settings. Presets change nothing until pressed.
- Output protection (limiter and soft clipper) is always on. A timing check shows the audio clock difference; it does not resynchronise the audio yet.

### Profile and quests (desktop)
- 50 built-in profile pictures, and your own PNG or JPG picture with cropping.
- Hourly quests, and category icons on the quest-period tabs.

### Offline fonts (desktop)
- The Windows app no longer asks Google Fonts for anything. The weights its own typography used (Plus Jakarta Sans, JetBrains Mono, Inter, Manrope, Space Grotesk) are built in as files, and the app's security policy no longer allows the Google font hosts. The website keeps using Google Fonts.
- 165 font files are built into the app in total; the font licences are listed in `font-licenses/INDEX.md`.

## 7.4.1 — 2026-11-04 · MIDI octave & Roblox sustain fixes

### MIDI keyboard
- **Notes outside the old 88-key range are detected.** The page accepts every MIDI note 0–127. The keyboard now shows C0 (MIDI 12) to C9 (MIDI 120), which covers the Electro FM ST-50 in octave -2 and +2. Before, notes below A0 and above C8 were ignored.
- Notes outside the drawn keyboard (MIDI 0–11 and 121–127) are logged and counted, but not drawn.
- Statistics show "Keys used / 109" and a 109-key chart.

### MIDI Bridge
- **Roblox sustain (CC64):** a repeated sustain-pedal-down message no longer presses Space again. Space is released when the pedal is lifted.
- Key mappings, profiles and the Roblox Space sustain setting are unchanged.

### Data
- No change to the recording database, settings or backups.

### Testing
- Simulated MIDI only (no hardware, no Roblox): the new MIDI range and sustain test 27/27, the existing simulated-MIDI suite 10/10.
- Not tested: a real Electro FM ST-50 and Roblox.

### Desktop app 7.4.1
- Page 7.4.1 bundled. No change to the app's Rust code.

## 7.4.0 — 2026-11-03 · Voice activity tracking & analytics

### Voice Studio
- **Speaking and Singing sessions are tracked and saved.** A session starts when the microphone and the analysis are running (after you press Start) and ends when you stop. The mode comes from the existing **Voice style** setting (Singing / humming or Speaking). If you change the style while a session runs, that session ends and a new one starts.
- Each session saves its start and end time, the **session duration** (time the microphone was watched; muted/paused time is not counted) and the **active vocal duration** (time a voice was detected). Silence is not counted as speaking or singing.
- Activity that crosses midnight is split between the two calendar days (local time).
- A live session box in Voice Studio (mode, start time, session time, active time, state) and a summary when you stop.
- If the app closes unexpectedly, the session is kept as "interrupted" at its last saved point (saved every 5 seconds); the time after that is never counted.

### History
- New **Voice activity** view (switch next to *Piano presses*): sessions grouped by date with start/end time, session and active duration, paused time and status; Speaking / Singing filter, period filter, pages. The piano history is unchanged.

### Statistics
- **Voice statistics** at the top of *Key statistics*: totals and averages for the chosen period (today, yesterday, 7/30 days, this/last month, this year, all time, custom), session counts, personal bests (longest sessions, most active day / week / month), daily / weekly / monthly charts and a session-length chart.

### Dashboard
- New **Voice activity** section with 9 cards (lifetime, today, speaking, singing, sessions, average, longest, most active day) and a 14-day chart. Zero sessions show clean empty states.

### Data & backup
- Voice sessions are stored in their own database (`PianoLoggerVoiceActivity`); the piano database is not changed.
- Backups include the voice sessions (placed before the presses, next to the settings); restoring merges them by session id without duplicates. Backups from 7.3.5 and older restore normally (voice history starts empty), and 7.3.5 can still restore a 7.4.0 backup (it skips the voice part).

### Privacy
- Voice activity is measured on this computer from the existing analysis (level, pitch, noise). **No sound is saved, nothing is uploaded, and there is no speech-to-text.** Only times and counts are stored.

### Limits
- The speaking and singing time is an **estimate** from level and pitch, not speech recognition: loud background noise above the Voice Studio noise setting can be counted as speech. Sessions shorter than one second are not saved.
- Tested automatically with a controlled clock and a simulated microphone (audio file). Testing with a physical microphone has not been done yet.

### Desktop app 7.4.0
- Page 7.4.0 bundled. No change to the app's Rust code.

## 7.3.5 — 2026-11-03 · Release history & maintenance update

### Release history
- Corrected the release-history ordering: newest releases now appear first (7.3.5, 7.3.4, 7.3.3, …).
- Cause: the list was sorted before the newest entries were added, so entries added later kept the wrong order. The list is now sorted each time it opens.
- The newest release is selected and marked "Current" when What's New opens.
- Website and desktop release-history displays use the same code.

### Branding maintenance
- Kept the new Piano Logger waveform emblem introduced in 7.3.4.
- Kept the website and desktop branding consistent.
- Kept the corrected sidebar branding.
- Kept the updated Windows application icon assets and website favicons.

### Windows maintenance
- Investigated the remaining Windows taskbar icon issue. Cause: the old icon belongs to the legacy Electron app (`Piano Logger 6.1 (Electron)`), not the current Tauri app. Manual steps are in the development record.

### Commits in this history
- `188797b`: initial logo replacement.
- `c7ebdeb`: sidebar branding correction.

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
| 9.2.0 | 2026-11-23 | Tauri 2.12.5 (WebView2) | Page 9.2.0 (Cloud sync). **The shell changed:** the Content-Security-Policy `connect-src` allows the cloud sync service; new commands `oauth_begin` / `oauth_cancel` (Google sign-in return on `127.0.0.1`, new `oauth.rs`); `window.pianoApp.caps` and `.cloud`. An older app cannot use Cloud sync and is told to update. Built locally (installer 6,092,332 bytes) and tested as an isolated copy (25 checks of the shell, below); not installed over the owner's app and not published. |
| 9.2.2 | 2026-11-24 | Tauri 2.12.5 (WebView2) | Page 9.2.2 (first-upload preview; sync off by default; voice, settings and key presses not uploaded yet). **The shell is unchanged** from 9.2.0/9.2.1 (same security rules and commands); version field 9.2.2. Built locally and tested as an isolated copy of the owner's data; not installed over the owner's app and not published. |
| 9.2.1 | 2026-11-23 | Tauri 2.12.5 (WebView2) | Page 9.2.1 (two-step sign-in with an authenticator app). **The shell is unchanged** from 9.2.0 (same security rules and commands); only the bundled page changed. Built locally and tested as an isolated copy; not installed over the owner's app and not published. |
| 9.1.0 | 2026-11-18 | Tauri 2.12.5 (WebView2) | Page 9.1.0 (Light/Dark score, 3D hands, MP4 export, larger heatmap). `music_pick` also accepts `mp4` / `webm` (native Save dialog for videos); `music_write` limit 600 MB. Released 2026-11-18 (`app-v9.1.0`). |
| 9.0.0 | 2026-11-18 | Tauri 2.12.5 (WebView2) | Page 9.0.0 (Sheet Music). New Rust commands `music_pick` / `music_write`: a native Save dialog for `.mid` and `.pdf` files (the page only suggests a checked file name; the file is written only to the path you pick). Released 2026-11-18 (`app-v9.0.0`; corrected in 9.1.0, this row said "not published"). |
| 7.4.1 | 2026-11-04 | Tauri 2.12.5 (WebView2) | MIDI octave range (MIDI 0–127) and Roblox sustain (CC64) fixes (page 7.4.1). No change to the app's Rust code. Built locally, not published yet. |
| 7.4.0 | 2026-11-03 | Tauri 2.12.5 (WebView2) | Voice activity tracking and analytics (page 7.4.0). No change to the app's Rust code. Built locally, not published yet. |
| 7.3.5 | 2026-11-03 | Tauri 2.12.5 (WebView2) | Release-history ordering fixed; branding kept. Built locally, not published yet. |
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
