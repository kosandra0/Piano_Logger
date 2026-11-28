// Piano Logger desktop app (Tauri 2) — the bridge between the page and Windows.
// It gives the page the same window.pianoApp the Electron app gives it, so the page itself never
// needs Tauri: on the website window.pianoApp simply does not exist and the app-only extras stay hidden.
// The page only gets these few functions; it cannot touch any other file on your computer.
(function () {
  'use strict';
  if (window.top !== window || window.pianoApp) return;
  var SITE = 'https://kosandra0.github.io/Piano_Logger/';
  var inv = function (cmd, args, opts) { return window.__TAURI_INTERNALS__.invoke(cmd, args || {}, opts); };
  var listeners = [];
  var verOf = function (html) { var m = /const APP_VERSION\s*=\s*['"]([\d.]+)['"]/.exec(html || ''); return m ? m[1] : '0'; };
  var looksLikePage = function (html) { return typeof html === 'string' && html.length > 100000 && html.indexOf('const APP_VERSION') >= 0 && /<\/html>\s*$/i.test(html.trim()) && html.indexOf('PianoLifetimeLogger') >= 0; };

  // v9.2.4: MIDI goes through the app, not through the browser. WebView2's own Web MIDI opens every input as soon as it is
  // used and never gives one back, so it is replaced here (before the page runs). An input is open in Windows only while the
  // page has a message handler on it; every other port stays free for other programs.
  (function () {
    var ins = new Map(), outs = new Map(), access = null, timer = null, queued = false, busy = {}, anyBusy = false;
    function Port(type, name) { this.type = type; this.name = name; this.id = 'native-' + type + '-' + name; this.manufacturer = ''; this.state = 'connected'; this.connection = 'closed'; this._h = null; }
    Object.defineProperty(Port.prototype, 'onmidimessage', {
      get: function () { return this._h; },
      set: function (h) { this._h = typeof h === 'function' ? h : null; if (this.type === 'input') { this.connection = this._h ? 'open' : 'closed'; plan(); } }
    });
    Port.prototype.open = function () { return Promise.resolve(this); };   // an input opens when it gets a handler
    Port.prototype.close = function () {
      if (this.type === 'input') { this._h = null; plan(); } else inv('midi_close_output', { name: this.name }).catch(function () { });
      this.connection = 'closed'; return Promise.resolve(this);
    };
    Port.prototype.send = function (data, when) {
      var self = this, bytes = Array.from(data);
      var go = function () { inv('midi_send', { name: self.name, bytes: bytes }).then(function () { self.connection = 'open'; delete busy['out ' + self.name]; }).catch(function () { warn('out ' + self.name, self.name); }); };
      if (when > performance.now()) setTimeout(go, when - performance.now()); else go();
    };
    function warn(key, name) { if (busy[key]) return; busy[key] = true; window.dispatchEvent(new CustomEvent('pianoapp-midi-busy', { detail: name })); }
    function plan() { if (queued) return; queued = true; Promise.resolve().then(sync); }
    function sync() {
      queued = false;
      var wanted = []; ins.forEach(function (p) { if (p._h && p.state === 'connected') wanted.push(p.name); });
      return inv('midi_sync', { inputs: wanted }).then(function (failed) {
        anyBusy = failed.length > 0;
        wanted.forEach(function (n) { if (failed.indexOf(n) < 0) delete busy['in ' + n]; else warn('in ' + n, n); });
      }).catch(function () { });
    }
    function apply(map, type, names) {
      var changed = [];
      names.forEach(function (n) {
        var id = 'native-' + type + '-' + n, p = map.get(id);
        if (!p) { p = new Port(type, n); map.set(id, p); changed.push(p); }
        else if (p.state !== 'connected') { p.state = 'connected'; changed.push(p); }
      });
      map.forEach(function (p) { if (p.state === 'connected' && names.indexOf(p.name) < 0) { p.state = 'disconnected'; p.connection = 'closed'; changed.push(p); } });
      return changed;
    }
    function poll() {
      return inv('midi_list').then(function (l) {
        var changed = apply(ins, 'input', l.inputs).concat(apply(outs, 'output', l.outputs));
        if (access && access.onstatechange) changed.forEach(function (p) { try { access.onstatechange({ port: p }); } catch (e) { } });
        if (changed.length || anyBusy) plan();
      });
    }
    window.addEventListener('pianoapp-midi', function (e) {
      (e.detail || []).forEach(function (m) {
        var p = ins.get('native-input-' + m[0]);
        if (p && p._h) { try { p._h({ data: new Uint8Array(m[1]), timeStamp: performance.now() - m[2], target: p }); } catch (err) { console.error(err); } }
      });
    });
    var request = function () {
      return poll().then(function () {
        if (!access) access = { inputs: ins, outputs: outs, onstatechange: null, sysexEnabled: false, nativePorts: true };
        if (!timer) timer = setInterval(function () { poll().catch(function () { }); }, 2000);   // plugging and unplugging
        return access;
      });
    };
    try { Object.defineProperty(navigator, 'requestMIDIAccess', { value: request, configurable: true }); } catch (e) { }
  })();

  // look for a newer page on the website; the app keeps it and uses it from the next start
  async function checkUpdate() {
    var current = '0';
    try { current = (await inv('app_info')).pageVersion; } catch (e) { }
    try {
      var r = await fetch(SITE + '?t=' + Date.now(), { cache: 'no-store', signal: AbortSignal.timeout(20000) });
      if (!r.ok) return { ok: false, error: 'HTTP ' + r.status, current: current };
      var html = await r.text();
      if (!looksLikePage(html)) return { ok: false, error: 'The website did not return the Piano Logger page', current: current };
      var res = await inv('upd_store', { html: html });
      if (res.updated) listeners.forEach(function (cb) { try { cb(res.latest); } catch (e) { } });
      return res;
    } catch (e) { return { ok: false, error: String(e && e.message || e), current: current }; }
  }

  window.pianoApp = Object.freeze({
    isApp: true,
    runtime: 'tauri',
    // 9.2.0: what this installed shell can do. A shell without caps.cloudSync has no permission to reach the cloud sync
    // service (its security rules are part of the installed program), so the page asks the user to update instead.
    caps: Object.freeze({ cloudSync: 1, oauthLoopback: 1 }),
    // Google sign-in: the shell listens on 127.0.0.1 for the browser's return trip, then tells the page ('pianoapp-oauth' event).
    cloud: Object.freeze({
      oauthBegin: function () { return inv('oauth_begin'); },
      oauthCancel: function () { return inv('oauth_cancel'); }
    }),
    info: function () { return inv('app_info'); },
    sf: Object.freeze({
      list: function () { return inv('sf_list'); },
      add: function () { return inv('sf_add'); },
      read: function (file) { return inv('sf_read', { file: String(file) }).then(function (b) { return new Uint8Array(b); }); },
      remove: function (file) { return inv('sf_remove', { file: String(file) }); },
      openFolder: function () { return inv('sf_open'); }
    }),
    backup: Object.freeze({
      save: function (name, text) { return inv('bk_save', { name: String(name), text: String(text) }); },
      begin: function (name) { return inv('bk_begin', { name: String(name) }); },
      write: function (id, bytes) { return inv('bk_write', bytes instanceof Uint8Array ? bytes : new Uint8Array(bytes), { headers: { 'x-backup-id': String(id) } }); },
      end: function (id) { return inv('bk_end', { id: String(id) }); },
      abort: function (id) { return inv('bk_abort', { id: String(id) }); },
      // v8.0.1: Save dialog for "Create backup file"; resolves to an id for write/end, or '' when cancelled
      pick: function (name) { return inv('bk_pick', { name: String(name) }); },
      list: function () { return inv('bk_list'); },
      openFolder: function () { return inv('bk_open'); }
    }),
    // v7.5.0: Save dialog for the Accumulated Notes export; resolves to the saved path, or '' if cancelled
    saveExport: function (name, text) { return inv('save_export', { name: String(name), text: String(text) }); },
    // v9.0.0: Sheet Music — Save dialog for .mid / .pdf; resolves to the saved path, or '' when cancelled
    music: Object.freeze({
      save: async function (name, kind, bytes) {
        var id = await inv('music_pick', { name: String(name), kind: ({ pdf: 'pdf', mp4: 'mp4', webm: 'webm', md: 'md' })[kind] || 'mid' });
        if (!id) return '';
        return inv('music_write', bytes instanceof Uint8Array ? bytes : new Uint8Array(bytes), { headers: { 'x-music-id': String(id) } });
      }
    }),
    setOnTop: function (on) { return inv('win_ontop', { on: !!on }); },
    setStartup: function (on) { return inv('app_startup', { on: !!on }); },
    // 7.2: closing the window keeps Piano Logger running in the system tray (logging only, no key presses)
    setCloseToTray: function (on) { return inv('app_close_to_tray', { on: !!on }); },
    quit: function () { return inv('app_quit'); },
    checkUpdate: checkUpdate,
    restart: function () { return inv('app_restart'); },
    openDataFolder: function () { return inv('open_data_dir'); },
    repairStorage: function () { return inv('repair_storage'); },
    onUpdate: function (cb) { if (typeof cb === 'function') listeners.push(cb); },
    // 7.1: MIDI Keyboard Bridge — real key presses, only into the chosen target window (checked again in Rust)
    keys: Object.freeze({
      send: function (events, target) { return inv('bridge_send', { events: events, target: target || {} }); },
      releaseAll: function () { return inv('bridge_release_all'); },
      foreground: function (target) { return inv('bridge_foreground', { target: target || {} }); },
      focusTarget: function (target) { return inv('bridge_focus_target', { target: target || {} }); },
      status: function () { return inv('bridge_status'); }
    }),
    // 7.0.2: the window has no Windows frame; the page's title bar uses these
    window: Object.freeze({
      state: function () { return inv('win_state'); },
      minimize: function () { return inv('win_minimize'); },
      toggleMaximize: function () { return inv('win_toggle_max'); },
      close: function () { return inv('win_close'); },
      startDrag: function () { return inv('win_drag'); }
    })
  });

  // once per app start (not on every reload), a few seconds after the window opens; quietly skipped when offline
  try {
    if (!sessionStorage.getItem('pll.updChecked')) { sessionStorage.setItem('pll.updChecked', '1'); setTimeout(function () { checkUpdate().catch(function () { }); }, 4000); }
  } catch (e) { }
})();
