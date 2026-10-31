// Puts the newest website page inside the app before every build (the same job the Electron
// workflow does with "cp index.html desktop/page/piano-logger.html").
// Looks for ../index.html first (the GitHub repository layout), then ../piano-logger.html (this folder).
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, '..', '..');
const src = ['index.html', 'piano-logger.html'].map(n => join(root, n)).find(existsSync);
if (!src) { console.error('copy-page: no index.html or piano-logger.html next to Piano-Logger-tauri'); process.exit(1); }
const html = readFileSync(src, 'utf8');
const m = /const APP_VERSION\s*=\s*['"]([\d.]+)['"]/.exec(html);
if (!m || !html.includes('PianoLifetimeLogger')) { console.error('copy-page: ' + src + ' does not look like the Piano Logger page'); process.exit(1); }
const app = JSON.parse(readFileSync(join(here, '..', 'src-tauri', 'tauri.conf.json'), 'utf8')).version;
if (!app.startsWith(m[1] + '.') && app !== m[1]) console.warn('copy-page: note: page version ' + m[1] + ' and app version ' + app + ' differ');
const out = join(here, '..', 'web', 'index.html');
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, html);
console.log('copy-page: ' + src + ' (page ' + m[1] + ') -> web/index.html');
