// Piano Logger — Windows desktop app (Electron).
// It shows the same page as the website, plus a few things only an installed app can do:
// SoundFont instruments, automatic backups into a folder, always-on-top, start with Windows,
// sound that keeps playing in the background, and updates fetched from the website.
'use strict';
const { app, BrowserWindow, protocol, net, ipcMain, dialog, shell, session, Menu } = require('electron');
const path = require('path');
const fs = require('fs');
const fsp = fs.promises;

const SITE = 'https://kosandra0.github.io/Piano_Logger/';   // the website the app updates itself from
const SCHEME = 'pianologger';
const ORIGIN = SCHEME + '://app/';

protocol.registerSchemesAsPrivileged([{ scheme: SCHEME, privileges: { standard: true, secure: true, supportFetchAPI: true, corsEnabled: true, stream: true } }]);
app.setAppUserModelId('com.kosandra0.pianologger');
// the page keeps playing sound and logging notes while the window is minimized or behind other windows
app.commandLine.appendSwitch('disable-renderer-backgrounding');
app.commandLine.appendSwitch('disable-background-timer-throttling');
app.commandLine.appendSwitch('autoplay-policy', 'no-user-gesture-required');

const firstInstance = app.requestSingleInstanceLock();       // only one Piano Logger window at a time
if (!firstInstance) app.quit();

let win = null;
const P = {};                                                   // folders, filled in when the app is ready
let settings = { onTop: false, bounds: null, maximized: false };
let served = { version: '0', source: 'bundled', file: '' };

/* ---------- small helpers ---------- */
const verOf = html => { const m = /const APP_VERSION\s*=\s*['"]([\d.]+)['"]/.exec(html || ''); return m ? m[1] : '0'; };
const newer = (a, b) => {                                       // is version a newer than version b?
  const x = String(a).split('.').map(Number), y = String(b).split('.').map(Number);
  for (let i = 0; i < Math.max(x.length, y.length); i++) { const d = (x[i] || 0) - (y[i] || 0); if (d) return d > 0; }
  return false;
};
const looksLikePage = html => typeof html === 'string' && html.length > 100000 && html.includes('const APP_VERSION') && /<\/html>\s*$/i.test(html.trim()) && html.includes('PianoLifetimeLogger');
async function readJson(f, dflt) { try { return Object.assign({}, dflt, JSON.parse(await fsp.readFile(f, 'utf8'))); } catch (e) { return dflt; } }
async function writeAtomic(f, data) { const tmp = f + '.tmp'; await fsp.writeFile(tmp, data); await fsp.rename(tmp, f); }
const saveSettings = () => writeAtomic(P.settings, JSON.stringify(settings)).catch(() => { });

/* ---------- which copy of the page to show: the one inside the app, or a newer one downloaded from the website ---------- */
async function pickPage() {
  const bundled = path.join(__dirname, 'page', 'piano-logger.html');
  let best = { version: '0', source: 'bundled', file: bundled };
  try { best.version = verOf(await fsp.readFile(bundled, 'utf8')); } catch (e) { }
  try {
    const html = await fsp.readFile(P.cachedPage, 'utf8');
    if (looksLikePage(html) && newer(verOf(html), best.version)) best = { version: verOf(html), source: 'website', file: P.cachedPage };
  } catch (e) { }
  served = best;
  return best;
}
async function checkUpdate() {
  try {
    const r = await net.fetch(SITE + '?t=' + Date.now(), { cache: 'no-store', signal: AbortSignal.timeout(20000) });
    if (!r.ok) return { ok: false, error: 'HTTP ' + r.status, current: served.version };
    const html = await r.text();
    if (!looksLikePage(html)) return { ok: false, error: 'The website did not return the Piano Logger page', current: served.version };
    const v = verOf(html);
    if (!newer(v, served.version)) return { ok: true, current: served.version, latest: v, updated: false };
    await fsp.mkdir(path.dirname(P.cachedPage), { recursive: true });
    await writeAtomic(P.cachedPage, html);
    if (win && !win.isDestroyed()) win.webContents.send('upd:ready', v);
    return { ok: true, current: served.version, latest: v, updated: true };
  } catch (e) { return { ok: false, error: String(e && e.message || e), current: served.version }; }
}

/* ---------- the window ---------- */
function createWindow() {
  const b = settings.bounds || {};
  win = new BrowserWindow({
    width: b.width || 1400, height: b.height || 900, x: b.x, y: b.y, minWidth: 360, minHeight: 400,
    backgroundColor: '#05050f', title: 'Piano Logger', show: false, autoHideMenuBar: true,
    icon: path.join(__dirname, 'build', 'icon.png'),
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'), contextIsolation: true, nodeIntegration: false, sandbox: true,
      backgroundThrottling: false, autoplayPolicy: 'no-user-gesture-required', spellcheck: false
    }
  });
  if (settings.maximized) win.maximize();
  if (settings.onTop) win.setAlwaysOnTop(true, 'floating');
  win.once('ready-to-show', () => win.show());
  win.loadURL(ORIGIN);

  // links to websites open in your normal browser; the page's own pop-ups (print preview) stay in the app
  win.webContents.setWindowOpenHandler(({ url }) => {
    if (/^https?:/i.test(url)) { shell.openExternal(url); return { action: 'deny' }; }
    return { action: 'allow' };
  });
  win.webContents.on('will-navigate', (e, url) => { if (!url.startsWith(ORIGIN)) { e.preventDefault(); if (/^https?:/i.test(url)) shell.openExternal(url); } });
  // F5 / Ctrl+R reload, F11 full screen, Ctrl+Shift+I developer tools
  win.webContents.on('before-input-event', (e, i) => {
    if (i.type !== 'keyDown') return;
    if (i.key === 'F5' || (i.control && !i.shift && i.key.toLowerCase() === 'r')) { win.webContents.reload(); e.preventDefault(); }
    else if (i.key === 'F11') { win.setFullScreen(!win.isFullScreen()); e.preventDefault(); }
    else if (i.control && i.shift && i.key.toLowerCase() === 'i') { win.webContents.toggleDevTools(); e.preventDefault(); }
  });
  win.webContents.on('render-process-gone', () => { setTimeout(() => { if (win && !win.isDestroyed()) win.webContents.reload(); }, 800); });
  const keep = () => { if (!win || win.isDestroyed()) return; settings.maximized = win.isMaximized(); if (!settings.maximized && !win.isFullScreen()) settings.bounds = win.getBounds(); };
  win.on('resize', keep); win.on('move', keep);
  win.on('close', () => { keep(); saveSettings(); });
  win.on('closed', () => { win = null; });
}

/* ---------- things the page can ask for (see preload.js) ---------- */
const sfName = f => { const n = path.basename(String(f)); if (!/\.sf2$/i.test(n)) throw new Error('Not a .sf2 file'); return n; };
async function sfList() {
  await fsp.mkdir(P.sf, { recursive: true });
  const out = [];
  for (const n of await fsp.readdir(P.sf)) {
    if (!/\.sf2$/i.test(n)) continue;
    try { const s = await fsp.stat(path.join(P.sf, n)); if (s.isFile()) out.push({ file: n, size: s.size, mtime: s.mtimeMs }); } catch (e) { }
  }
  return out.sort((a, b) => a.file.localeCompare(b.file));
}
async function isSf2(f) {
  const fh = await fsp.open(f, 'r');
  try { const b = Buffer.alloc(12); await fh.read(b, 0, 12, 0); return b.toString('latin1', 0, 4) === 'RIFF' && b.toString('latin1', 8, 12) === 'sfbk'; }
  finally { await fh.close(); }
}
function registerIpc() {
  ipcMain.handle('app:info', async () => ({
    appVersion: app.getVersion(), pageVersion: served.version, pageSource: served.source, electron: process.versions.electron,
    arch: process.arch, os: process.getSystemVersion ? process.getSystemVersion() : '', dataDir: P.data, backupDir: P.backups, sfDir: P.sf,
    onTop: !!settings.onTop, startup: !!app.getLoginItemSettings().openAtLogin, site: SITE
  }));
  ipcMain.handle('sf:list', () => sfList());
  ipcMain.handle('sf:add', async () => {
    const r = await dialog.showOpenDialog(win, { title: 'Add SoundFont instruments', filters: [{ name: 'SoundFont (.sf2)', extensions: ['sf2'] }], properties: ['openFile', 'multiSelections'] });
    const added = [], skipped = [];
    if (!r.canceled) {
      await fsp.mkdir(P.sf, { recursive: true });
      for (const f of r.filePaths) {
        try { if (!(await isSf2(f))) { skipped.push(path.basename(f)); continue; } const n = sfName(f); await fsp.copyFile(f, path.join(P.sf, n)); added.push(n); }
        catch (e) { skipped.push(path.basename(f)); }
      }
    }
    return { added, skipped, list: await sfList() };
  });
  ipcMain.handle('sf:read', async (_e, f) => fsp.readFile(path.join(P.sf, sfName(f))));
  ipcMain.handle('sf:remove', async (_e, f) => {
    const p = path.join(P.sf, sfName(f));
    try { await shell.trashItem(p); } catch (e) { await fsp.unlink(p); }   // goes to the Recycle Bin, so it can be brought back
    return sfList();
  });
  ipcMain.handle('sf:open', async () => { await fsp.mkdir(P.sf, { recursive: true }); return shell.openPath(P.sf); });
  ipcMain.handle('bk:save', async (_e, name, text) => {
    if (!/^piano-log-backup-[\w-]+\.json$/.test(name)) throw new Error('Bad backup name');
    await fsp.mkdir(P.backups, { recursive: true });
    const f = path.join(P.backups, name); await writeAtomic(f, text);
    const all = (await fsp.readdir(P.backups)).filter(n => /^piano-log-backup-[\w-]+\.json$/.test(n)).sort();
    for (const n of all.slice(0, Math.max(0, all.length - 30))) await fsp.unlink(path.join(P.backups, n)).catch(() => { });   // keep the newest 30
    return f;
  });
  ipcMain.handle('bk:list', async () => {
    try {
      const all = (await fsp.readdir(P.backups)).filter(n => /^piano-log-backup-[\w-]+\.json$/.test(n)).sort();
      return { count: all.length, newest: all.length ? all[all.length - 1] : null, dir: P.backups };
    } catch (e) { return { count: 0, newest: null, dir: P.backups }; }
  });
  ipcMain.handle('bk:open', async () => { await fsp.mkdir(P.backups, { recursive: true }); return shell.openPath(P.backups); });
  ipcMain.handle('win:ontop', (_e, on) => { settings.onTop = !!on; if (win) win.setAlwaysOnTop(settings.onTop, 'floating'); saveSettings(); return settings.onTop; });
  ipcMain.handle('app:startup', (_e, on) => { app.setLoginItemSettings({ openAtLogin: !!on }); return !!app.getLoginItemSettings().openAtLogin; });
  ipcMain.handle('upd:check', () => checkUpdate());
  ipcMain.handle('app:restart', () => { app.relaunch(); app.exit(0); });
  ipcMain.handle('app:datadir', () => shell.openPath(P.data));
}

/* ---------- start ---------- */
app.on('second-instance', () => { if (win) { if (win.isMinimized()) win.restore(); win.show(); win.focus(); } });
app.on('window-all-closed', () => app.quit());
app.whenReady().then(async () => {
  if (!firstInstance) return;
  P.data = app.getPath('userData');
  P.sf = path.join(P.data, 'SoundFonts');
  P.cachedPage = path.join(P.data, 'page', 'piano-logger.html');
  P.settings = path.join(P.data, 'settings.json');
  P.backups = path.join(app.getPath('documents'), 'Piano Logger Backups');
  settings = await readJson(P.settings, settings);
  await pickPage();

  protocol.handle(SCHEME, async req => {
    const u = new URL(req.url);
    if (u.host === 'app' && (u.pathname === '/' || u.pathname === '/index.html')) {
      try { return new Response(await fsp.readFile(served.file), { headers: { 'content-type': 'text/html; charset=utf-8', 'cache-control': 'no-store' } }); }
      catch (e) { return new Response('Could not read the page: ' + e.message, { status: 500 }); }
    }
    return new Response('Not found', { status: 404 });
  });
  // MIDI keyboards, notifications, clipboard, full screen and folders are allowed without asking every time
  const OK = new Set(['midi', 'midiSysex', 'notifications', 'clipboard-read', 'clipboard-sanitized-write', 'fullscreen', 'fileSystem', 'media', 'speaker-selection']);
  session.defaultSession.setPermissionRequestHandler((_wc, perm, cb) => cb(OK.has(perm)));
  session.defaultSession.setPermissionCheckHandler((_wc, perm) => OK.has(perm));

  Menu.setApplicationMenu(null);
  registerIpc();
  createWindow();
  setTimeout(checkUpdate, 4000);                                 // look for a newer page on the website (needs internet; quietly skipped if offline)
});
