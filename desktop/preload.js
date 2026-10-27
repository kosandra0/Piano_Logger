// Piano Logger desktop app — the bridge between the page and Windows.
// The page only gets these few functions; it cannot touch any other file on your computer.
const { contextBridge, ipcRenderer } = require('electron');
const inv = (ch, ...a) => ipcRenderer.invoke(ch, ...a);

contextBridge.exposeInMainWorld('pianoApp', {
  isApp: true,
  info: () => inv('app:info'),
  sf: {
    list: () => inv('sf:list'),
    add: () => inv('sf:add'),
    read: file => inv('sf:read', String(file)),
    remove: file => inv('sf:remove', String(file)),
    openFolder: () => inv('sf:open')
  },
  backup: {
    save: (name, text) => inv('bk:save', String(name), String(text)),
    begin: name => inv('bk:begin', String(name)),
    write: (id, bytes) => inv('bk:write', String(id), bytes),
    end: id => inv('bk:end', String(id)),
    abort: id => inv('bk:abort', String(id)),
    list: () => inv('bk:list'),
    openFolder: () => inv('bk:open')
  },
  setOnTop: on => inv('win:ontop', !!on),
  setStartup: on => inv('app:startup', !!on),
  checkUpdate: () => inv('upd:check'),
  restart: () => inv('app:restart'),
  openDataFolder: () => inv('app:datadir'),
  repairStorage: () => inv('app:repairStorage'),
  onUpdate: cb => { ipcRenderer.on('upd:ready', (_e, v) => { try { cb(v); } catch (e) { } }); }
});
