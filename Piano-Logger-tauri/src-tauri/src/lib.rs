// Piano Logger — Windows desktop app (Tauri 2).
// It shows the same page as the website, plus the things only an installed app can do:
// SoundFont instruments, backups into a folder, always on top, start with Windows,
// sound that keeps playing in the background, and a newer page fetched from the website.
// The page reaches these through window.pianoApp (bridge.js), the same API the Electron app gives it.
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{
    http,
    ipc::{InvokeBody, Request, Response},
    webview::NewWindowResponse,
    AppHandle, Manager, State, Url, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

const SITE: &str = "https://kosandra0.github.io/Piano_Logger/"; // the website the app updates its page from
const SCHEME: &str = "pianologger";
const BRIDGE: &str = include_str!("bridge.js");
const KEEP_BACKUPS: usize = 30;

/* ---------- state ---------- */
#[derive(Default, Serialize, Deserialize)]
struct Settings {
    #[serde(default)]
    on_top: bool,
}

struct Paths {
    data: PathBuf,
    sf: PathBuf,
    page: PathBuf,
    settings: PathBuf,
    backups: PathBuf,
}

struct Served {
    version: String,
    source: &'static str,
    html: Vec<u8>,
}

struct Backup {
    file: PathBuf,
    part: PathBuf,
    fh: fs::File,
}

struct AppState {
    p: Paths,
    settings: Mutex<Settings>,
    served: Mutex<Served>,
    bk: Mutex<HashMap<String, Backup>>,
}

type Res<T> = Result<T, String>;
fn err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

/* ---------- small helpers ---------- */
fn ver_of(html: &str) -> String {
    let key = "const APP_VERSION";
    let Some(i) = html.find(key) else { return "0".into() };
    let rest = &html[i + key.len()..];
    let Some(q) = rest.find(['\'', '"']) else { return "0".into() };
    let v: String = rest[q + 1..].chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    if v.is_empty() { "0".into() } else { v }
}
// is version a newer than version b?
fn newer(a: &str, b: &str) -> bool {
    let x: Vec<u64> = a.split('.').map(|n| n.parse().unwrap_or(0)).collect();
    let y: Vec<u64> = b.split('.').map(|n| n.parse().unwrap_or(0)).collect();
    for i in 0..x.len().max(y.len()) {
        let (p, q) = (x.get(i).copied().unwrap_or(0), y.get(i).copied().unwrap_or(0));
        if p != q {
            return p > q;
        }
    }
    false
}
fn looks_like_page(html: &str) -> bool {
    html.len() > 100_000
        && html.contains("const APP_VERSION")
        && html.contains("PianoLifetimeLogger")
        && html.trim_end().to_ascii_lowercase().ends_with("</html>")
}
fn write_atomic(f: &Path, data: &[u8]) -> std::io::Result<()> {
    let tmp = f.with_extension("tmp");
    fs::write(&tmp, data)?;
    fs::rename(&tmp, f)
}
fn now_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
}
fn save_settings(st: &AppState) {
    if let Ok(s) = st.settings.lock() {
        if let Ok(j) = serde_json::to_vec(&*s) {
            let _ = write_atomic(&st.p.settings, &j);
        }
    }
}
// a SoundFont name from the page: only a plain file name ending in .sf2
fn sf_name(f: &str) -> Res<String> {
    let n = Path::new(f).file_name().and_then(|n| n.to_str()).ok_or("Not a .sf2 file")?;
    if n != f || !n.to_ascii_lowercase().ends_with(".sf2") {
        return Err("Not a .sf2 file".into());
    }
    Ok(n.to_string())
}
// a backup name from the page: piano-log-backup-<letters, digits, - or _>.json
fn bk_name(n: &str) -> Res<()> {
    let mid = n.strip_prefix("piano-log-backup-").and_then(|r| r.strip_suffix(".json")).ok_or("Bad backup name")?;
    if mid.is_empty() || !mid.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err("Bad backup name".into());
    }
    Ok(())
}
fn backups_sorted(dir: &Path) -> Vec<String> {
    let mut all: Vec<String> = fs::read_dir(dir)
        .map(|rd| rd.filter_map(|e| e.ok()).filter_map(|e| e.file_name().into_string().ok()).filter(|n| bk_name(n).is_ok()).collect())
        .unwrap_or_default();
    all.sort();
    all
}
fn prune_backups(dir: &Path) {
    let all = backups_sorted(dir);
    if all.len() > KEEP_BACKUPS {
        for n in &all[..all.len() - KEEP_BACKUPS] {
            let _ = fs::remove_file(dir.join(n));
        }
    }
}
// name.ext, or "name (2).ext" … when that file already exists
fn unique_path(dir: &Path, name: &str) -> PathBuf {
    let first = dir.join(name);
    if !first.exists() {
        return first;
    }
    let p = Path::new(name);
    let stem = p.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let ext = p.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    (2..1000).map(|i| dir.join(format!("{stem} ({i}){ext}"))).find(|f| !f.exists()).unwrap_or(first)
}
fn is_sf2(f: &Path) -> bool {
    let mut b = [0u8; 12];
    fs::File::open(f).and_then(|mut h| h.read_exact(&mut b)).is_ok() && &b[0..4] == b"RIFF" && &b[8..12] == b"sfbk"
}

/* ---------- "Start with Windows": a value in the current user's Run key (never machine-wide, even when run as administrator) ---------- */
#[cfg(windows)]
mod startup {
    use windows_registry::{Type, CURRENT_USER};
    const RUN: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
    const APPROVED: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run";
    const NAME: &str = "com.kosandra0.pianologger"; // its own name, so it never touches the Electron app's entry
    pub fn is_enabled() -> bool {
        CURRENT_USER.open(RUN).and_then(|k| k.get_string(NAME)).is_ok()
    }
    pub fn set(on: bool) -> Result<bool, String> {
        let run = CURRENT_USER.create(RUN).map_err(|e| e.to_string())?;
        if on {
            let exe = std::env::current_exe().map_err(|e| e.to_string())?;
            run.set_string(NAME, format!("\"{}\"", exe.display())).map_err(|e| e.to_string())?;
            // switched off earlier in Task Manager → mark it as allowed again
            if let Ok(k) = CURRENT_USER.options().write().open(APPROVED) {
                let _ = k.set_bytes(NAME, Type::Bytes, &[2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            }
        } else {
            let _ = run.remove_value(NAME);
            if let Ok(k) = CURRENT_USER.options().write().open(APPROVED) {
                let _ = k.remove_value(NAME);
            }
        }
        Ok(is_enabled())
    }
}
#[cfg(not(windows))]
mod startup {
    pub fn is_enabled() -> bool {
        false
    }
    pub fn set(_on: bool) -> Result<bool, String> {
        Err("Start with Windows is only available on Windows".into())
    }
}

/* ---------- which copy of the page to show: the one inside the app, or a newer one downloaded from the website ---------- */
fn pick_page(app: &AppHandle, page_cache: &Path) -> Served {
    let bundled = app.asset_resolver().get("index.html".into()).map(|a| a.bytes).unwrap_or_default();
    let mut best = Served { version: ver_of(&String::from_utf8_lossy(&bundled)), source: "bundled", html: bundled };
    if let Ok(html) = fs::read_to_string(page_cache) {
        let v = ver_of(&html);
        if looks_like_page(&html) && newer(&v, &best.version) {
            best = Served { version: v, source: "website", html: html.into_bytes() };
        }
    }
    best
}

/* ---------- commands the page can call (see bridge.js) ---------- */
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Info {
    app_version: String,
    page_version: String,
    page_source: String,
    runtime: &'static str,
    tauri: &'static str,
    arch: &'static str,
    os: &'static str,
    data_dir: String,
    backup_dir: String,
    sf_dir: String,
    on_top: bool,
    startup: bool,
    site: &'static str,
    last_crash: Option<()>,
}
#[tauri::command]
fn app_info(app: AppHandle, st: State<AppState>) -> Info {
    let served = st.served.lock().unwrap();
    Info {
        app_version: app.package_info().version.to_string(),
        page_version: served.version.clone(),
        page_source: served.source.to_string(),
        runtime: "tauri",
        tauri: tauri::VERSION,
        arch: match std::env::consts::ARCH { "x86_64" => "x64", "x86" => "ia32", "aarch64" => "arm64", a => a },
        os: std::env::consts::OS,
        data_dir: st.p.data.display().to_string(),
        backup_dir: st.p.backups.display().to_string(),
        sf_dir: st.p.sf.display().to_string(),
        on_top: st.settings.lock().map(|s| s.on_top).unwrap_or(false),
        startup: startup::is_enabled(),
        site: SITE,
        last_crash: None,
    }
}

#[derive(Serialize)]
struct SfFile {
    file: String,
    size: u64,
    mtime: u128,
}
fn sf_scan(dir: &Path) -> Res<Vec<SfFile>> {
    fs::create_dir_all(dir).map_err(err)?;
    let mut out = vec![];
    for e in fs::read_dir(dir).map_err(err)?.flatten() {
        let Ok(n) = e.file_name().into_string() else { continue };
        if !n.to_ascii_lowercase().ends_with(".sf2") {
            continue;
        }
        if let Ok(m) = e.metadata() {
            if m.is_file() {
                let mtime = m.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_millis()).unwrap_or(0);
                out.push(SfFile { file: n, size: m.len(), mtime });
            }
        }
    }
    out.sort_by(|a, b| a.file.to_lowercase().cmp(&b.file.to_lowercase()));
    Ok(out)
}
#[tauri::command]
fn sf_list(st: State<AppState>) -> Res<Vec<SfFile>> {
    sf_scan(&st.p.sf)
}
#[derive(Serialize)]
struct SfAdded {
    added: Vec<String>,
    skipped: Vec<String>,
    list: Vec<SfFile>,
}
#[tauri::command]
async fn sf_add(app: AppHandle, st: State<'_, AppState>) -> Res<SfAdded> {
    let picked = app.dialog().file().set_title("Add SoundFont instruments").add_filter("SoundFont (.sf2)", &["sf2"]).blocking_pick_files();
    let (mut added, mut skipped) = (vec![], vec![]);
    if let Some(files) = picked {
        fs::create_dir_all(&st.p.sf).map_err(err)?;
        for f in files {
            let Ok(p) = f.into_path() else { continue };
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("?").to_string();
            match sf_name(&name) {
                Ok(n) if is_sf2(&p) && fs::copy(&p, st.p.sf.join(&n)).is_ok() => added.push(n),
                _ => skipped.push(name),
            }
        }
    }
    Ok(SfAdded { added, skipped, list: sf_scan(&st.p.sf)? })
}
#[tauri::command]
async fn sf_read(file: String, st: State<'_, AppState>) -> Res<Response> {
    Ok(Response::new(fs::read(st.p.sf.join(sf_name(&file)?)).map_err(err)?))
}
#[tauri::command]
fn sf_remove(file: String, st: State<AppState>) -> Res<Vec<SfFile>> {
    let p = st.p.sf.join(sf_name(&file)?);
    if trash::delete(&p).is_err() {
        // the Recycle Bin is not available (for example on some network drives): delete it
        fs::remove_file(&p).map_err(err)?;
    }
    sf_scan(&st.p.sf)
}
#[tauri::command]
fn sf_open(app: AppHandle, st: State<AppState>) -> Res<()> {
    fs::create_dir_all(&st.p.sf).map_err(err)?;
    app.opener().open_path(st.p.sf.display().to_string(), None::<&str>).map_err(err)
}

#[tauri::command]
fn bk_save(name: String, text: String, st: State<AppState>) -> Res<String> {
    bk_name(&name)?;
    fs::create_dir_all(&st.p.backups).map_err(err)?;
    let f = st.p.backups.join(&name);
    write_atomic(&f, text.as_bytes()).map_err(err)?;
    prune_backups(&st.p.backups);
    Ok(f.display().to_string())
}
// big backups arrive in pieces: begin → write, write, … → end
#[tauri::command]
fn bk_begin(name: String, st: State<AppState>) -> Res<String> {
    bk_name(&name)?;
    fs::create_dir_all(&st.p.backups).map_err(err)?;
    let file = st.p.backups.join(&name);
    let part = st.p.backups.join(format!("{name}.part"));
    let fh = fs::File::create(&part).map_err(err)?;
    let id = format!("{:x}{:x}", now_ms(), std::process::id() ^ (now_ms() as u32));
    st.bk.lock().unwrap().insert(id.clone(), Backup { file, part, fh });
    Ok(id)
}
#[tauri::command]
fn bk_write(request: Request<'_>, st: State<AppState>) -> Res<bool> {
    let id = request.headers().get("x-backup-id").and_then(|v| v.to_str().ok()).ok_or("Missing backup id")?.to_string();
    let InvokeBody::Raw(bytes) = request.body() else { return Err("Expected raw bytes".into()) };
    let mut map = st.bk.lock().unwrap();
    let b = map.get_mut(&id).ok_or("This backup was already closed")?;
    b.fh.write_all(bytes).map_err(err)?;
    Ok(true)
}
#[tauri::command]
fn bk_end(id: String, st: State<AppState>) -> Res<String> {
    let b = st.bk.lock().unwrap().remove(&id).ok_or("This backup was already closed")?;
    b.fh.sync_all().map_err(err)?;
    drop(b.fh);
    fs::rename(&b.part, &b.file).map_err(err)?;
    prune_backups(&st.p.backups);
    Ok(b.file.display().to_string())
}
#[tauri::command]
fn bk_abort(id: String, st: State<AppState>) -> bool {
    let Some(b) = st.bk.lock().unwrap().remove(&id) else { return false };
    drop(b.fh);
    let _ = fs::remove_file(&b.part);
    true
}
#[derive(Serialize)]
struct BkList {
    count: usize,
    newest: Option<String>,
    dir: String,
}
#[tauri::command]
fn bk_list(st: State<AppState>) -> BkList {
    let all = backups_sorted(&st.p.backups);
    BkList { count: all.len(), newest: all.last().cloned(), dir: st.p.backups.display().to_string() }
}
#[tauri::command]
fn bk_open(app: AppHandle, st: State<AppState>) -> Res<()> {
    fs::create_dir_all(&st.p.backups).map_err(err)?;
    app.opener().open_path(st.p.backups.display().to_string(), None::<&str>).map_err(err)
}

#[tauri::command]
fn win_ontop(on: bool, app: AppHandle, st: State<AppState>) -> Res<bool> {
    if let Some(w) = app.get_webview_window("main") {
        w.set_always_on_top(on).map_err(err)?;
    }
    st.settings.lock().unwrap().on_top = on;
    save_settings(&st);
    Ok(on)
}
#[tauri::command]
fn app_startup(on: bool) -> Res<bool> {
    startup::set(on)
}

// a newer page fetched by bridge.js: checked again here, then kept for the next start
#[derive(Serialize)]
struct UpdResult {
    ok: bool,
    current: String,
    latest: String,
    updated: bool,
    error: Option<String>,
}
#[tauri::command]
fn upd_store(html: String, st: State<AppState>) -> UpdResult {
    let current = st.served.lock().unwrap().version.clone();
    if !looks_like_page(&html) {
        return UpdResult { ok: false, current, latest: String::new(), updated: false, error: Some("The website did not return the Piano Logger page".into()) };
    }
    let latest = ver_of(&html);
    if !newer(&latest, &current) {
        return UpdResult { ok: true, current, latest, updated: false, error: None };
    }
    let res = fs::create_dir_all(st.p.page.parent().unwrap()).and_then(|_| write_atomic(&st.p.page, html.as_bytes()));
    match res {
        Ok(_) => UpdResult { ok: true, current, latest, updated: true, error: None },
        Err(e) => UpdResult { ok: false, current, latest, updated: false, error: Some(e.to_string()) },
    }
}
#[tauri::command]
fn app_restart(app: AppHandle) {
    app.restart();
}
#[tauri::command]
fn open_data_dir(app: AppHandle, st: State<AppState>) -> Res<()> {
    app.opener().open_path(st.p.data.display().to_string(), None::<&str>).map_err(err)
}
// "Repair storage": on the next start the browser-data folder is moved aside (never deleted) and a fresh one is made
#[tauri::command]
fn repair_storage(app: AppHandle, st: State<AppState>) -> Res<()> {
    fs::write(st.p.data.join("repair-storage.flag"), now_ms().to_string()).map_err(err)?;
    app.restart();
}

/* ---------- start ---------- */
// What the page may load. It is one file with inline scripts and styles; it fetches piano samples, fonts and its own
// newer version from these sites only. 'unsafe-eval' is for the page's add-ons feature (new Function on a file the user picks).
// No frames, plugins, forms or <base>: remote websites cannot be embedded in the app window.
const CSP: &str = "default-src 'self'; \
script-src 'self' 'unsafe-inline' 'unsafe-eval'; \
style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; \
font-src 'self' data: https://fonts.gstatic.com; \
img-src 'self' data: blob:; media-src 'self' data: blob:; worker-src 'self' blob:; \
connect-src 'self' ipc: http://ipc.localhost data: blob: https://tonejs.github.io https://gleitz.github.io https://kosandra0.github.io; \
frame-src 'none'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

fn page_response(st: &AppState, path: &str) -> http::Response<Vec<u8>> {
    if path == "/" || path == "/index.html" {
        let html = st.served.lock().unwrap().html.clone();
        return http::Response::builder()
            .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
            .header(http::header::CACHE_CONTROL, "no-store")
            .header("Content-Security-Policy", CSP)
            .body(html)
            .unwrap();
    }
    http::Response::builder().status(404).header(http::header::CONTENT_TYPE, "text/plain").body(b"Not found".to_vec()).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // only one Piano Logger window at a time: a second start brings the first one forward
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .register_uri_scheme_protocol(SCHEME, |ctx, req| {
            let st = ctx.app_handle().state::<AppState>();
            page_response(&st, req.uri().path())
        })
        .invoke_handler(tauri::generate_handler![
            app_info, sf_list, sf_add, sf_read, sf_remove, sf_open, bk_save, bk_begin, bk_write, bk_end, bk_abort, bk_list, bk_open,
            win_ontop, app_startup, upd_store, app_restart, open_data_dir, repair_storage
        ])
        .setup(|app| {
            let h = app.handle().clone();
            let data = app.path().app_data_dir()?;
            fs::create_dir_all(&data)?;
            let docs = app.path().document_dir().unwrap_or_else(|_| data.clone());
            let p = Paths {
                sf: data.join("SoundFonts"),
                page: data.join("page").join("piano-logger.html"),
                settings: data.join("settings.json"),
                backups: docs.join("Piano Logger Backups"),
                data,
            };
            // a repair was asked for: move the old browser data to a safe folder before the window opens
            let flag = p.data.join("repair-storage.flag");
            if flag.exists() {
                if let Ok(local) = app.path().app_local_data_dir() {
                    let web = local.join("EBWebView");
                    if web.exists() {
                        let _ = fs::rename(&web, local.join(format!("EBWebView-old-{}", now_ms())));
                    }
                }
                let _ = fs::remove_file(&flag);
            }
            let settings: Settings = fs::read(&p.settings).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
            let on_top = settings.on_top;
            let served = pick_page(&h, &p.page);
            app.manage(AppState { p, settings: Mutex::new(settings), served: Mutex::new(served), bk: Mutex::new(HashMap::new()) });

            // custom protocols are served as http://<scheme>.localhost on Windows
            let url: Url = if cfg!(windows) { format!("http://{SCHEME}.localhost/") } else { format!("{SCHEME}://localhost/") }.parse()?;
            let origin_host = url.host_str().unwrap_or_default().to_string();
            let perm_host = origin_host.clone();
            let (nav_h, new_h) = (h.clone(), h.clone());
            let is_ours = move |u: &Url| u.host_str() == Some(origin_host.as_str()) || u.scheme() == SCHEME;
            let is_ours2 = is_ours.clone();
            // Tauri's own defaults, plus: sound may start without a click first. Setting these replaces WebView2's standard
            // WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS variable, so its value is added back (used for diagnostics and automated tests).
            let mut args = String::from("--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection --autoplay-policy=no-user-gesture-required");
            if let Ok(extra) = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS") {
                args.push(' ');
                args.push_str(extra.trim());
            }
            WebviewWindowBuilder::new(app, "main", WebviewUrl::CustomProtocol(url))
                .title("Piano Logger")
                .inner_size(1400.0, 900.0)
                .min_inner_size(360.0, 400.0)
                .background_color(tauri::window::Color(5, 8, 22, 255))
                .always_on_top(on_top)
                .initialization_script(BRIDGE)
                // keep sound, the visualizer and logging running while the window is minimised or hidden
                .background_throttling(tauri::utils::config::BackgroundThrottlingPolicy::Disabled)
                .additional_browser_args(&args)
                // MIDI keyboards (and the few things the page uses: downloads, a backup folder, clipboard, notifications)
                // are allowed for the app's own page without a pop-up, like the Electron app did. Anything else
                // (camera, microphone, location…) gets WebView2's normal prompt.
                .on_permission_request(move |wv, kind| {
                    use tauri::webview::{PermissionKind as K, PermissionResponse as P};
                    #[cfg(debug_assertions)]
                    eprintln!("[piano-logger] permission requested: {kind:?}");
                    let ours = wv.url().map(|u| u.host_str() == Some(perm_host.as_str())).unwrap_or(false);
                    match kind {
                        K::Midi | K::Notifications | K::ClipboardRead | K::AutomaticDownloads | K::FileSystemAccess | K::Autoplay if ours => P::Allow,
                        _ => P::Default,
                    }
                })
                // links to websites open in your normal browser
                .on_navigation(move |u| {
                    if is_ours(u) || matches!(u.scheme(), "about" | "blob") {
                        return true;
                    }
                    if matches!(u.scheme(), "http" | "https") {
                        let _ = nav_h.opener().open_url(u.as_str(), None::<&str>);
                    }
                    false
                })
                // exports, backup files and .mid files: saved in Downloads (never over an existing file); the page is told where
                .on_download(move |wv, ev| match ev {
                    tauri::webview::DownloadEvent::Requested { url: _url, destination } => {
                        #[cfg(debug_assertions)]
                        eprintln!("[piano-logger] download requested: {_url} -> {}", destination.display());
                        let dir = wv.app_handle().path().download_dir().unwrap_or_else(|_| std::env::temp_dir());
                        let name = destination.file_name().map(|n| n.to_string_lossy().into_owned()).filter(|n| !n.is_empty()).unwrap_or_else(|| "download".into());
                        *destination = unique_path(&dir, &name);
                        true
                    }
                    tauri::webview::DownloadEvent::Finished { path, success, .. } => {
                        let msg = serde_json::json!({ "success": success, "path": path.map(|p| p.display().to_string()) });
                        let _ = wv.eval(format!("window.dispatchEvent(new CustomEvent('pianoapp-download', {{ detail: {msg} }}))"));
                        true
                    }
                    _ => true,
                })
                // pop-ups: the page's own (print preview, blobs) open; websites go to the browser; anything else (file:, data:…) is refused
                .on_new_window(move |u, _features| {
                    if is_ours2(&u) || matches!(u.scheme(), "about" | "blob") {
                        return NewWindowResponse::Allow;
                    }
                    if matches!(u.scheme(), "http" | "https") {
                        let _ = new_h.opener().open_url(u.as_str(), None::<&str>);
                    }
                    NewWindowResponse::Deny
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Piano Logger");
}
