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
      list: function () { return inv('bk_list'); },
      openFolder: function () { return inv('bk_open'); }
    }),
    setOnTop: function (on) { return inv('win_ontop', { on: !!on }); },
    setStartup: function (on) { return inv('app_startup', { on: !!on }); },
    checkUpdate: checkUpdate,
    restart: function () { return inv('app_restart'); },
    openDataFolder: function () { return inv('open_data_dir'); },
    repairStorage: function () { return inv('repair_storage'); },
    onUpdate: function (cb) { if (typeof cb === 'function') listeners.push(cb); }
  });

  // once per app start (not on every reload), a few seconds after the window opens; quietly skipped when offline
  try {
    if (!sessionStorage.getItem('pll.updChecked')) { sessionStorage.setItem('pll.updChecked', '1'); setTimeout(function () { checkUpdate().catch(function () { }); }, 4000); }
  } catch (e) { }
})();
