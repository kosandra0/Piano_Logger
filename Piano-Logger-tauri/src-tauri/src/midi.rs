// v9.2.4: native MIDI (Windows winmm).
// Why: the browser engine inside the app (WebView2) opens EVERY MIDI input as soon as a page asks for Web MIDI, and
// MIDIPort.close() in the page never gives a port back to Windows. So the app no longer uses Web MIDI at all
// (bridge.js replaces navigator.requestMIDIAccess) and only the ports the user selected are opened, here.
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex, OnceLock,
    },
    time::Instant,
};
use windows::Win32::Media::Audio::*;

const MIM_DATA: u32 = 0x3C3;

#[derive(Serialize)]
pub struct Ports {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

/// One short MIDI message as it arrived: which open input, the packed bytes, and when.
pub struct Msg {
    pub slot: usize,
    pub packed: u32,
    pub at: Instant,
}

static TX: OnceLock<Mutex<Sender<Msg>>> = OnceLock::new();

#[derive(Default)]
struct State {
    inputs: HashMap<String, (usize, usize)>, // name -> (slot, handle)
    names: Vec<String>,                      // slot -> name
    outputs: HashMap<String, usize>,         // name -> handle
}

#[derive(Default)]
pub struct Midi(Mutex<State>);

/// Two ports may have the same name: the second one becomes "name #2", so every port can be chosen on its own.
fn unique(names: Vec<String>) -> Vec<String> {
    let mut seen: HashMap<String, u32> = HashMap::new();
    names
        .into_iter()
        .map(|n| {
            let c = seen.entry(n.clone()).or_insert(0);
            *c += 1;
            if *c == 1 {
                n
            } else {
                format!("{n} #{c}")
            }
        })
        .collect()
}
fn wide(name: &[u16]) -> String {
    let end = name.iter().position(|&c| c == 0).unwrap_or(name.len());
    String::from_utf16_lossy(&name[..end])
}
fn input_names() -> Vec<String> {
    unique(
        (0..unsafe { midiInGetNumDevs() })
            .map(|i| {
                let mut c = MIDIINCAPSW::default();
                if unsafe { midiInGetDevCapsW(i as usize, &mut c, std::mem::size_of::<MIDIINCAPSW>() as u32) } != 0 {
                    return format!("MIDI input {}", i + 1);
                }
                let name = c.szPname; // copied out: the structure is packed
                wide(&name)
            })
            .collect(),
    )
}
fn output_names() -> Vec<String> {
    unique(
        (0..unsafe { midiOutGetNumDevs() })
            .map(|i| {
                let mut c = MIDIOUTCAPSW::default();
                if unsafe { midiOutGetDevCapsW(i as usize, &mut c, std::mem::size_of::<MIDIOUTCAPSW>() as u32) } != 0 {
                    return format!("MIDI output {}", i + 1);
                }
                let name = c.szPname;
                wide(&name)
            })
            .collect(),
    )
}

// Runs on a Windows MIDI thread: only hand the message over (calling MIDI functions here can deadlock).
unsafe extern "system" fn on_input(_h: HMIDIIN, msg: u32, slot: usize, data: usize, _time: usize) {
    if msg != MIM_DATA {
        return;
    }
    let packed = data as u32;
    if packed & 0xFF >= 0xF8 {
        return; // clock / active sensing: the page ignores these, so they are not even passed on
    }
    if let Some(tx) = TX.get() {
        if let Ok(tx) = tx.lock() {
            let _ = tx.send(Msg { slot, packed, at: Instant::now() });
        }
    }
}

/// Number of bytes of a short message, from its status byte.
pub fn length(status: u8) -> usize {
    match status {
        0xC0..=0xDF | 0xF1 | 0xF3 => 2,
        0xF4..=0xFF => 1,
        _ => 3,
    }
}
pub fn unpack(packed: u32) -> Vec<u8> {
    let b = packed.to_le_bytes();
    b[..length(b[0])].to_vec()
}
pub fn pack(bytes: &[u8]) -> Option<u32> {
    if bytes.is_empty() || bytes.len() > 3 || bytes[0] < 0x80 || bytes[0] == 0xF0 {
        return None;
    }
    let mut b = [0u8; 4];
    b[..bytes.len()].copy_from_slice(bytes);
    Some(u32::from_le_bytes(b))
}

impl Midi {
    /// The channel incoming messages arrive on (set up once, at start).
    pub fn receiver() -> Receiver<Msg> {
        let (tx, rx) = channel();
        let _ = TX.set(Mutex::new(tx));
        rx
    }
    pub fn list(&self) -> Ports {
        Ports { inputs: input_names(), outputs: output_names() }
    }
    pub fn name_of(&self, slot: usize) -> Option<String> {
        self.0.lock().unwrap().names.get(slot).cloned()
    }
    /// Open exactly these inputs and close every other one. Returns the names that could not be opened.
    pub fn sync(&self, wanted: &[String]) -> Vec<String> {
        let present = input_names();
        // take the handles to close out of the state first: closing must not happen while the lock is held
        let closing: Vec<usize> = {
            let mut st = self.0.lock().unwrap();
            let gone: Vec<String> = st.inputs.keys().filter(|n| !wanted.contains(n) || !present.contains(n)).cloned().collect();
            gone.iter().filter_map(|n| st.inputs.remove(n)).map(|(_, h)| h).collect()
        };
        for h in closing {
            close_input(h);
        }
        let mut failed = Vec::new();
        for name in wanted {
            let Some(dev) = present.iter().position(|n| n == name) else { continue };
            let slot = {
                let mut st = self.0.lock().unwrap();
                if st.inputs.contains_key(name) {
                    continue;
                }
                match st.names.iter().position(|n| n == name) {
                    Some(s) => s,
                    None => {
                        st.names.push(name.clone());
                        st.names.len() - 1
                    }
                }
            };
            let mut h = HMIDIIN::default();
            let callback = on_input as *const () as usize;
            if unsafe { midiInOpen(&mut h, dev as u32, Some(callback), Some(slot), CALLBACK_FUNCTION) } != 0 {
                failed.push(name.clone());
                continue;
            }
            if unsafe { midiInStart(h) } != 0 {
                unsafe { midiInClose(h) };
                failed.push(name.clone());
                continue;
            }
            self.0.lock().unwrap().inputs.insert(name.clone(), (slot, h.0 as usize));
        }
        failed
    }
    /// Send one short message; the output is opened the first time something is sent to it.
    pub fn send(&self, name: &str, bytes: &[u8]) -> Result<(), String> {
        let packed = pack(bytes).ok_or("not a short MIDI message")?;
        let mut st = self.0.lock().unwrap();
        let h = match st.outputs.get(name) {
            Some(h) => *h,
            None => {
                let dev = output_names().iter().position(|n| n == name).ok_or("MIDI output not found")?;
                let mut h = HMIDIOUT::default();
                if unsafe { midiOutOpen(&mut h, dev as u32, None, None, CALLBACK_NULL) } != 0 {
                    return Err(format!("{name} is in use by another program"));
                }
                st.outputs.insert(name.to_string(), h.0 as usize);
                h.0 as usize
            }
        };
        if unsafe { midiOutShortMsg(HMIDIOUT(h as *mut _), packed) } != 0 {
            // the device went away: forget the handle so it is opened again next time
            st.outputs.remove(name);
            unsafe { midiOutClose(HMIDIOUT(h as *mut _)) };
            return Err(format!("could not send to {name}"));
        }
        Ok(())
    }
    pub fn close_output(&self, name: &str) {
        if let Some(h) = self.0.lock().unwrap().outputs.remove(name) {
            unsafe { midiOutClose(HMIDIOUT(h as *mut _)) };
        }
    }
    pub fn close_all(&self) {
        let (ins, outs) = {
            let mut st = self.0.lock().unwrap();
            let ins: Vec<usize> = st.inputs.drain().map(|(_, (_, h))| h).collect();
            let outs: Vec<usize> = st.outputs.drain().map(|(_, h)| h).collect();
            (ins, outs)
        };
        ins.into_iter().for_each(close_input);
        for h in outs {
            unsafe { midiOutClose(HMIDIOUT(h as *mut _)) };
        }
    }
}
fn close_input(h: usize) {
    let h = HMIDIIN(h as *mut _);
    unsafe {
        midiInStop(h);
        midiInReset(h);
        midiInClose(h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn messages_pack_and_unpack() {
        for m in [vec![0x90u8, 60, 100], vec![0x80, 60, 0], vec![0xC0, 17], vec![0xB0, 64, 127]] {
            assert_eq!(unpack(pack(&m).unwrap()), m);
        }
        assert!(pack(&[]).is_none() && pack(&[0x10, 1, 1]).is_none() && pack(&[0xF0, 1, 2]).is_none() && pack(&[0x90, 1, 2, 3]).is_none());
    }
    #[test]
    fn equal_names_are_numbered() {
        assert_eq!(unique(vec!["A".into(), "B".into(), "A".into()]), vec!["A", "B", "A #2"]);
    }
}
