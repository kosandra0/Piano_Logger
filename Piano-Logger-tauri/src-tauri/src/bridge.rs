// MIDI Keyboard Bridge — the part that touches Windows.
// The page decides WHICH keys go down and up (mapping, chords, sustain, reference counting); this module only:
//   · presses / releases real keys with SendInput (scan codes, which games such as Roblox read reliably),
//   · refuses to send unless the chosen target window is the foreground window (and never into Piano Logger itself),
//   · remembers every key it is holding, so all of them can be released at once (Emergency Stop, exit, hotkey).
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Mutex, time::Instant};

#[derive(Default)]
pub struct Bridge {
    pub held: Mutex<HashSet<u16>>, // virtual-key codes the bridge has pressed and not yet released
    pub hotkey_ok: Mutex<bool>,
    pub in_tray: std::sync::atomic::AtomicBool, // the window is hidden in the system tray: never press keys then
}

#[derive(Deserialize)]
pub struct KeyEv {
    pub vk: u16,
    pub down: bool,
}

#[derive(Deserialize, Clone, Default)]
pub struct Target {
    #[serde(default)]
    pub exe: String, // e.g. RobloxPlayerBeta.exe (case-insensitive); empty = any window except Piano Logger
    #[serde(default)]
    pub title: String, // optional: the window title must contain this (case-insensitive)
}

#[derive(Serialize, Default)]
pub struct Fg {
    pub exe: String,
    pub title: String,
    pub pid: u32,
    pub is_self: bool,
    pub matches: bool,
}

#[derive(Serialize)]
pub struct SendResult {
    pub sent: bool,
    pub reason: String,
    pub micros: u64,
    pub held: usize,
}

#[cfg(windows)]
mod win {
    use super::*;
    use windows::core::PWSTR;
    use windows::Win32::Foundation::{CloseHandle, HWND};
    use windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION};
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        MapVirtualKeyW, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE,
        MAPVK_VK_TO_VSC, VIRTUAL_KEY,
    };
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId};

    pub fn foreground(target: &Target) -> Fg {
        let h: HWND = unsafe { GetForegroundWindow() };
        if h.0.is_null() {
            return Fg::default();
        }
        describe(h, target)
    }

    // program name, title and "is this the target?" for one window
    fn describe(h: HWND, target: &Target) -> Fg {
        unsafe {
            let mut buf = [0u16; 512];
            let n = GetWindowTextW(h, &mut buf);
            let title = String::from_utf16_lossy(&buf[..n.max(0) as usize]);
            let mut pid = 0u32;
            GetWindowThreadProcessId(h, Some(&mut pid));
            let mut exe = String::new();
            if let Ok(ph) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                let mut pb = [0u16; 1024];
                let mut len = pb.len() as u32;
                if QueryFullProcessImageNameW(ph, PROCESS_NAME_WIN32, PWSTR(pb.as_mut_ptr()), &mut len).is_ok() {
                    let full = String::from_utf16_lossy(&pb[..len as usize]);
                    exe = full.rsplit(['\\', '/']).next().unwrap_or("").to_string();
                }
                let _ = CloseHandle(ph);
            }
            let is_self = pid == std::process::id();
            let exe_ok = target.exe.trim().is_empty() || exe.eq_ignore_ascii_case(target.exe.trim());
            let title_ok = target.title.trim().is_empty() || title.to_lowercase().contains(&target.title.trim().to_lowercase());
            let matches = !is_self && pid != 0 && exe_ok && title_ok;
            Fg { exe, title, pid, is_self, matches }
        }
    }

    /// 7.2.1: bring the target program's window to the front ("Go to Roblox"). Piano Logger is in front when the user clicks
    /// the button, so Windows allows it. Returns the window found, or an empty Fg when the program is not running.
    pub fn focus_target(target: &Target) -> Fg {
        use windows::core::BOOL;
        use windows::Win32::Foundation::LPARAM;
        use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, IsIconic, IsWindowVisible, SetForegroundWindow, ShowWindow, SW_RESTORE};
        struct Find<'a> {
            target: &'a Target,
            hit: Option<(HWND, Fg)>,
        }
        unsafe extern "system" fn each(h: HWND, l: LPARAM) -> BOOL {
            let f = &mut *(l.0 as *mut Find);
            if !IsWindowVisible(h).as_bool() {
                return BOOL(1);
            }
            let fg = describe(h, f.target);
            if fg.matches && !fg.title.is_empty() {
                f.hit = Some((h, fg));
                return BOOL(0);
            }
            BOOL(1)
        }
        if target.exe.trim().is_empty() && target.title.trim().is_empty() {
            return Fg::default(); // "any window" has nothing to switch to
        }
        let mut f = Find { target, hit: None };
        unsafe {
            let _ = EnumWindows(Some(each), LPARAM(&mut f as *mut Find as isize));
            if let Some((h, fg)) = f.hit {
                if IsIconic(h).as_bool() {
                    let _ = ShowWindow(h, SW_RESTORE);
                }
                let _ = SetForegroundWindow(h);
                return fg;
            }
        }
        Fg::default()
    }

    fn extended(vk: u16) -> bool {
        // arrows, Insert/Delete, Home/End, Page Up/Down, right-hand Ctrl/Alt, Num Lock, keypad divide
        matches!(vk, 0x21..=0x28 | 0x2D | 0x2E | 0xA3 | 0xA5 | 0x90 | 0x6F)
    }

    pub fn send(evs: &[KeyEv]) -> u32 {
        let inputs: Vec<INPUT> = evs
            .iter()
            .map(|e| {
                let scan = unsafe { MapVirtualKeyW(e.vk as u32, MAPVK_VK_TO_VSC) } as u16;
                let mut flags = KEYEVENTF_SCANCODE;
                if extended(e.vk) {
                    flags |= KEYEVENTF_EXTENDEDKEY;
                }
                if !e.down {
                    flags |= KEYEVENTF_KEYUP;
                }
                INPUT {
                    r#type: INPUT_KEYBOARD,
                    Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(e.vk), wScan: scan, dwFlags: KEYBD_EVENT_FLAGS(flags.0), time: 0, dwExtraInfo: 0 } },
                }
            })
            .collect();
        if inputs.is_empty() {
            return 0;
        }
        unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) }
    }
}

impl Bridge {
    pub fn focus_target(&self, target: &Target) -> Fg {
        #[cfg(windows)]
        return win::focus_target(target);
        #[cfg(not(windows))]
        {
            let _ = target;
            Fg::default()
        }
    }

    pub fn foreground(&self, target: &Target) -> Fg {
        #[cfg(windows)]
        return win::foreground(target);
        #[cfg(not(windows))]
        {
            let _ = target;
            Fg::default()
        }
    }

    /// Sends a batch of key events (one SendInput call, so a chord goes down together), only into the target window.
    pub fn send(&self, evs: &[KeyEv], target: &Target) -> SendResult {
        let t0 = Instant::now();
        let fg = self.foreground(target);
        let mut held = self.held.lock().unwrap();
        // key-ups are always allowed (releasing can never hurt); key-downs only into the focused target
        let any_down = evs.iter().any(|e| e.down);
        if any_down && self.in_tray.load(std::sync::atomic::Ordering::SeqCst) {
            return SendResult { sent: false, reason: "app-in-tray".into(), micros: t0.elapsed().as_micros() as u64, held: held.len() };
        }
        if any_down && !fg.matches {
            let reason = if fg.is_self { "piano-logger-focused" } else { "target-not-focused" };
            return SendResult { sent: false, reason: reason.into(), micros: t0.elapsed().as_micros() as u64, held: held.len() };
        }
        #[cfg(windows)]
        let n = win::send(evs);
        #[cfg(not(windows))]
        let n = 0u32;
        if n as usize == evs.len() {
            for e in evs {
                if e.down { held.insert(e.vk); } else { held.remove(&e.vk); }
            }
            SendResult { sent: true, reason: String::new(), micros: t0.elapsed().as_micros() as u64, held: held.len() }
        } else {
            SendResult { sent: false, reason: format!("windows-refused ({} of {} events)", n, evs.len()), micros: t0.elapsed().as_micros() as u64, held: held.len() }
        }
    }

    /// Releases every key the bridge is holding (Emergency Stop, disable, device lost, focus lost, exit).
    pub fn release_all(&self) -> usize {
        let mut held = self.held.lock().unwrap();
        let evs: Vec<KeyEv> = held.iter().map(|&vk| KeyEv { vk, down: false }).collect();
        #[cfg(windows)]
        if !evs.is_empty() {
            win::send(&evs);
        }
        let n = held.len();
        held.clear();
        n
    }
}

/// Ctrl + Alt + End anywhere in Windows: release every bridge key and tell the page to disarm.
#[cfg(windows)]
pub fn start_emergency_hotkey<F: Fn() + Send + 'static>(on_hotkey: F) -> bool {
    use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT};
    use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || unsafe {
        let ok = RegisterHotKey(None, 0x504C, MOD_CONTROL | MOD_ALT | MOD_NOREPEAT, 0x23 /* VK_END */).is_ok();
        let _ = tx.send(ok);
        if !ok {
            return;
        }
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            if msg.message == WM_HOTKEY {
                on_hotkey();
            }
        }
    });
    rx.recv().unwrap_or(false)
}
#[cfg(not(windows))]
pub fn start_emergency_hotkey<F: Fn() + Send + 'static>(_on_hotkey: F) -> bool {
    false
}
