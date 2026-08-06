/**
 * @typedef {object} PageOpts
 * @property {string} orientation
 * @property {number[]} marges
 * @property {number[]} espacements
 * @property {number} police
 * @property {number} interligne
 */

/**
 * @typedef {object} Transpilation
 * @property {PageOpts} page
 * @property {string} html
 * @property {string} stats
 */

const editor = document.getElementById('editor');
const pagesEl = document.getElementById('pages');
const measure = document.getElementById('measure');
const statusEl = document.getElementById('status');
const diagEl = document.getElementById('diag');
const wrapper = document.getElementById('preview-wrapper');

/** @type {PageOpts | null} */
let pageOpts = null;
let renderSeq = 0;
let zoom = 0.9;
let debounceTimer = null;
let modifie = false;

function diag(message) {
    diagEl.textContent = message;
    diagEl.className = 'on';
}

window.onerror = (message, source, ligne, colonne) => {
    diag(`Erreur JavaScript ligne ${ligne}:${colonne} — ${message}`);
    return false;
};

window.addEventListener('unhandledrejection', (e) => {
    diag(`Promesse rejetée — ${e.reason}`);
});

function ipcAvailable() {
    return !!(window['ipc'] && typeof window['ipc'].postMessage === 'function');
}

function send(obj) {
    if (!ipcAvailable()) {
        diag("Le pont IPC n'est pas disponible : window.ipc est absent. Aucune commande ne peut atteindre Rust.");
        return;
    }
    try {
        window['ipc'].postMessage(JSON.stringify(obj));
    } catch (e) {
        diag(`Envoi IPC impossible — ${e}`);
    }
}

function requestTranspile(mode) {
    statusEl.textContent = 'composition…';
    send({ cmd: 'render', content: editor.value, mode: mode || 'inc' });
}

function recompose() {
    modifie = true;
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => requestTranspile('inc'), 250);
}

function chargeContenu(text) {
    editor.value = text;
    modifie = false;
    requestTranspile('full');
}

/** @param {PageOpts} opts */
function pageDims(opts) {
    let w = 210, h = 297;
    if (opts.orientation === 'paysage' || opts.orientation === 'landscape') { w = 297; h = 210; }
    const m = opts.marges, e = opts.espacements;
    const clamp = (v, max) => Math.min(Math.max(v, 0), max);
    const mt = clamp(m[0] + e[0], h * 0.4);
    const mr = clamp(m[1] + e[1], w * 0.4);
    const mb = clamp(m[2] + e[2], h * 0.4);
    const ml = clamp(m[3] + e[3], w * 0.4);
    return { w, h, mt, mr, mb, ml, cw: w - ml - mr, ch: h - mt - mb };
}

function mmToPx(mm) {
    const probe = document.createElement('div');
    probe.style.width = `${mm}mm`;
    probe.style.position = 'absolute';
    probe.style.visibility = 'hidden';
    document.body.appendChild(probe);
    const px = probe.getBoundingClientRect().width;
    probe.remove();
    return px;
}

function newPage(d) {
    const p = document.createElement('div');
    p.className = 'page';
    p.style.width = `${d.w}mm`;
    p.style.height = `${d.h}mm`;
    const s = document.createElement('div');
    s.className = 'sheet doc';
    s.style.padding = `${d.mt}mm ${d.mr}mm ${d.mb}mm ${d.ml}mm`;
    s.style.fontSize = `${pageOpts.police || 11}pt`;
    s.style.lineHeight = String(pageOpts.interligne || 1.3);
    p.appendChild(s);
    const no = document.createElement('div');
    no.className = 'pageno';
    p.appendChild(no);
    pagesEl.appendChild(p);
    return s;
}

function attachNotes() {
    for (const page of pagesEl.querySelectorAll('.page')) {
        const corps = page.querySelectorAll('.note-corps');
        if (!corps.length) continue;
        const pied = document.createElement('div');
        pied.className = 'notes-pied';
        for (const note of corps) {
            const item = document.createElement('div');
            item.innerHTML = `<sup>${note.getAttribute('data-num')}</sup> ${note.innerHTML}`;
            pied.appendChild(item);
        }
        page.appendChild(pied);
    }
}

function flow(d) {
    pagesEl.innerHTML = '';
    const usable = mmToPx(d.ch) - 2;
    const hautPx = mmToPx(d.mt);
    let sheet = newPage(d);
    const blocks = Array.from(measure.children);
    for (const b of blocks) {
        if (b.className && b.className.includes('pagebreak')) {
            sheet = newPage(d);
            continue;
        }
        sheet.appendChild(b);
        const used = b.offsetTop + b.offsetHeight - sheet.offsetTop - hautPx;
        if (used > usable && sheet.children.length > 1) {
            const titre = b.previousElementSibling;
            const suit = titre && titre.className
                && titre.className.includes('sec')
                && sheet.children.length > 2;
            sheet = newPage(d);
            if (suit) sheet.appendChild(titre);
            sheet.appendChild(b);
        }
    }
    const pages = pagesEl.querySelectorAll('.page');
    pages.forEach((page, i) => {
        page.querySelector('.pageno').textContent = `${i + 1} / ${pages.length}`;
    });
}

function fillToc() {
    const pages = Array.from(pagesEl.querySelectorAll('.page'));
    for (const mark of pagesEl.querySelectorAll('.toc-pg')) {
        const target = document.getElementById(mark.getAttribute('data-target'));
        if (!target) continue;
        const numero = pages.findIndex((page) => page.contains(target));
        if (numero !== -1) mark.textContent = String(numero + 1);
    }
}

const DELIMITEURS = [
    { left: '\\[', right: '\\]', display: true },
    { left: '\\(', right: '\\)', display: false }
];

function typeset(node) {
    if (window['renderMathInElement']) {
        try {
            window['renderMathInElement'](node, {
                delimiters: DELIMITEURS,
                throwOnError: false,
                errorColor: '#b00',
                trust: false
            });
        } catch (e) {
            diag(`Composition mathématique impossible — ${e}`);
        }
    }
    return Promise.resolve();
}

/** @param {Transpilation} res */
function onTranspiled(res) {
    const seq = ++renderSeq;
    try {
        pageOpts = res.page;
        const d = pageDims(pageOpts);
        document.getElementById('printsize').textContent =
            `@page { size: ${d.w}mm ${d.h}mm; margin: 0; }`;
        measure.className = 'doc';
        measure.style.width = `${d.cw}mm`;
        measure.style.fontSize = `${pageOpts.police || 11}pt`;
        measure.style.lineHeight = String(pageOpts.interligne || 1.3);
        measure.innerHTML = res.html;
        const keepScroll = wrapper.scrollTop;
        typeset(measure).catch(() => {}).then(() => {
            if (seq !== renderSeq) return;
            flow(d);
            attachNotes();
            fillToc();
            wrapper.scrollTop = keepScroll;
            statusEl.textContent = res.stats || '';
            syncSettingsFromPage();
            diagEl.className = '';
        });
    } catch (e) {
        diag(`Rendu impossible — ${e}`);
    }
}

window.onTranspiled = onTranspiled;
window.onMessage = (message, ok) => {
    statusEl.textContent = ok ? message : '';
    if (!ok) diag(message); else diagEl.className = '';
};
window.setEditorContent = chargeContenu;

function panneauFermeture() { return document.getElementById('fermeture'); }

window.demandeFermeture = () => {
    if (!modifie) { send({ cmd: 'quitter' }); return; }
    panneauFermeture().className = '';
};

editor.addEventListener('input', recompose);

function remplaceSelection(texte) {
    const debut = editor.selectionStart;
    const fin = editor.selectionEnd;
    editor.value = editor.value.slice(0, debut) + texte + editor.value.slice(fin);
    editor.selectionStart = editor.selectionEnd = debut + texte.length;
    editor.focus();
    recompose();
}

editor.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
        e.preventDefault();
        remplaceSelection('\t');
    }
});

function setZoom(z) {
    zoom = Math.min(2, Math.max(0.3, z));
    document.documentElement.style.setProperty('--zoom', String(zoom));
}

function documentPourImpression() {
    const copie = document.documentElement.cloneNode(true);
    for (const script of copie.querySelectorAll('script')) {
        script.remove();
    }
    return `<!doctype html>\n${copie.outerHTML}`;
}

function exportPdf() {
    if (!pageOpts) { diag("Rien à exporter : le document n'a pas encore été composé."); return; }
    statusEl.textContent = 'export en cours…';
    send({ cmd: 'export', content: documentPourImpression() });
}

function settingsPanel() { return document.getElementById('settings'); }

function num(id, fallback) {
    const v = parseFloat(document.getElementById(id).value);
    return isFinite(v) ? v : fallback;
}

function syncSettingsFromPage() {
    if (!pageOpts) return;
    const panel = settingsPanel();
    if (panel.contains(document.activeElement)) return;
    document.getElementById('set-orientation').value =
        (pageOpts.orientation === 'paysage' || pageOpts.orientation === 'landscape') ? 'paysage' : 'portrait';
    const ids = ['mar-top', 'mar-right', 'mar-bottom', 'mar-left'];
    ids.forEach((id, i) => { document.getElementById(id).value = pageOpts.marges[i]; });
    const eids = ['esp-top', 'esp-right', 'esp-bottom', 'esp-left'];
    eids.forEach((id, i) => { document.getElementById(id).value = pageOpts.espacements[i]; });
    document.getElementById('set-police').value = pageOpts.police;
    document.getElementById('set-interligne').value = pageOpts.interligne;
}

function quad(a, b, c, d) {
    if (a === b && b === c && c === d) return String(a);
    return `{${a};${b};${c};${d}}`;
}

function serializePageBlock() {
    return 'page {\n\torientation: ' + document.getElementById('set-orientation').value + ';\n'
        + '\tmarges: ' + quad(num('mar-top', 20), num('mar-right', 20), num('mar-bottom', 20), num('mar-left', 20)) + ';\n'
        + '\tespacements: ' + quad(num('esp-top', 2), num('esp-right', 2), num('esp-bottom', 2), num('esp-left', 2)) + ';\n'
        + '\ttaille: ' + num('set-police', 11) + ';\n'
        + '\tinterligne: ' + num('set-interligne', 1.3) + ';\n}';
}

function findPageBlock(text) {
    const m = /(^|\n)\s*page\s*\{/.exec(text);
    if (!m) return null;
    const open = m.index + m[0].length - 1;
    let depth = 0;
    for (let i = open; i < text.length; i++) {
        if (text[i] === '{') depth++;
        else if (text[i] === '}') {
            depth--;
            if (depth === 0) return { start: m.index + (m[1] ? m[1].length : 0), end: i + 1 };
        }
    }
    return null;
}

function applySettings() {
    modifie = true;
    const block = serializePageBlock();
    const src = editor.value;
    const found = findPageBlock(src);
    editor.value = found
        ? src.slice(0, found.start) + block + src.slice(found.end)
        : `${block}\n\n${src}`;
    requestTranspile('full');
}

document.getElementById('btn-load').addEventListener('click', () => send({ cmd: 'load' }));
document.getElementById('btn-save').addEventListener('click', () => {
    modifie = false;
    send({ cmd: 'save', content: editor.value });
});
document.getElementById('btn-quitter').addEventListener('click', () => send({ cmd: 'quitter' }));
document.getElementById('btn-rester').addEventListener('click', () => {
    panneauFermeture().className = 'hidden';
});
document.getElementById('btn-export').addEventListener('click', exportPdf);
document.getElementById('btn-settings').addEventListener('click', () => {
    const panel = settingsPanel();
    panel.className = panel.className === 'hidden' ? '' : 'hidden';
    syncSettingsFromPage();
});
document.getElementById('btn-apply').addEventListener('click', applySettings);
document.getElementById('btn-zoom-in').addEventListener('click', () => setZoom(zoom + 0.1));
document.getElementById('btn-zoom-out').addEventListener('click', () => setZoom(zoom - 0.1));

setZoom(zoom);

if (!ipcAvailable()) {
    diag("Le pont IPC n'est pas encore disponible au chargement. Si ce message persiste après un clic sur un bouton, la vue web n'a pas reçu le script d'initialisation de wry.");
}

editor.value = '';
requestTranspile('full');
