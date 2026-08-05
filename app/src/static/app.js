var editor = document.getElementById('editor');
var pagesEl = document.getElementById('pages');
var measure = document.getElementById('measure');
var statusEl = document.getElementById('status');
var diagEl = document.getElementById('diag');
var pageOpts = null;
var renderSeq = 0;
var zoom = 0.9;
var debounceTimer = null;
var wrapper = document.getElementById('preview-wrapper');
var modifie = false;

function diag(message) {
    diagEl.textContent = message;
    diagEl.className = 'on';
}

window.onerror = function (message, source, line, col) {
    diag('Erreur JavaScript ligne ' + line + ':' + col + ' — ' + message);
    return false;
};

window.addEventListener('unhandledrejection', function (e) {
    diag('Promesse rejetée — ' + e.reason);
});

function ipcAvailable() {
    return !!(window.ipc && typeof window.ipc.postMessage === 'function');
}

function send(obj) {
    if (!ipcAvailable()) {
        diag("Le pont IPC n'est pas disponible : window.ipc est absent. Aucune commande ne peut atteindre Rust.");
        return;
    }
    try {
        window.ipc.postMessage(JSON.stringify(obj));
    } catch (e) {
        diag('Envoi IPC impossible — ' + e);
    }
}

function requestTranspile(mode) {
    statusEl.textContent = 'composition…';
    send({ cmd: 'render', content: editor.value, mode: mode || 'inc' });
}

function chargeContenu(text) {
    editor.value = text;
    modifie = false;
    requestTranspile('full');
}

function pageDims(opts) {
    var w = 210, h = 297;
    if (opts.orientation === 'paysage' || opts.orientation === 'landscape') { w = 297; h = 210; }
    var m = opts.marges, e = opts.espacements;
    function clamp(v, max) { return Math.min(Math.max(v, 0), max); }
    var mt = clamp(m[0] + e[0], h * 0.4);
    var mr = clamp(m[1] + e[1], w * 0.4);
    var mb = clamp(m[2] + e[2], h * 0.4);
    var ml = clamp(m[3] + e[3], w * 0.4);
    return { w: w, h: h, mt: mt, mr: mr, mb: mb, ml: ml, cw: w - ml - mr, ch: h - mt - mb };
}

function mmToPx(mm) {
    var probe = document.createElement('div');
    probe.style.width = mm + 'mm';
    probe.style.position = 'absolute';
    probe.style.visibility = 'hidden';
    document.body.appendChild(probe);
    var px = probe.getBoundingClientRect().width;
    probe.parentNode.removeChild(probe);
    return px;
}

function newPage(d) {
    var p = document.createElement('div');
    p.className = 'page';
    p.style.width = d.w + 'mm';
    p.style.height = d.h + 'mm';
    var s = document.createElement('div');
    s.className = 'sheet doc';
    s.style.padding = d.mt + 'mm ' + d.mr + 'mm ' + d.mb + 'mm ' + d.ml + 'mm';
    s.style.fontSize = (pageOpts.police || 11) + 'pt';
    s.style.lineHeight = String(pageOpts.interligne || 1.3);
    p.appendChild(s);
    var no = document.createElement('div');
    no.className = 'pageno';
    p.appendChild(no);
    pagesEl.appendChild(p);
    return s;
}

function attachNotes(d) {
    var pages = pagesEl.querySelectorAll('.page');
    for (var i = 0; i < pages.length; i++) {
        var corps = pages[i].querySelectorAll('.note-corps');
        if (!corps.length) continue;
        var pied = document.createElement('div');
        pied.className = 'notes-pied';
        for (var j = 0; j < corps.length; j++) {
            var item = document.createElement('div');
            item.innerHTML = '<sup>' + corps[j].getAttribute('data-num') + '</sup> '
                + corps[j].innerHTML;
            pied.appendChild(item);
        }
        pages[i].appendChild(pied);
    }
}

function flow(d) {
    pagesEl.innerHTML = '';
    var usable = mmToPx(d.ch) - 2;
    var hautPx = mmToPx(d.mt);
    var sheet = newPage(d);
    var blocks = [];
    var i;
    for (i = 0; i < measure.children.length; i++) blocks.push(measure.children[i]);
    for (i = 0; i < blocks.length; i++) {
        var b = blocks[i];
        if (b.className && b.className.indexOf('pagebreak') !== -1) {
            sheet = newPage(d);
            continue;
        }
        sheet.appendChild(b);
        var used = b.offsetTop + b.offsetHeight - sheet.offsetTop - hautPx;
        if (used > usable && sheet.children.length > 1) {
            var titre = b.previousElementSibling;
            var suit = titre && titre.className
                && titre.className.indexOf('sec') !== -1
                && sheet.children.length > 2;
            sheet = newPage(d);
            if (suit) sheet.appendChild(titre);
            sheet.appendChild(b);
        }
    }
    var pages = pagesEl.querySelectorAll('.page');
    for (i = 0; i < pages.length; i++) {
        pages[i].querySelector('.pageno').textContent = (i + 1) + ' / ' + pages.length;
    }
}

function fillToc() {
    var marks = pagesEl.querySelectorAll('.toc-pg');
    var pages = pagesEl.querySelectorAll('.page');
    for (var i = 0; i < marks.length; i++) {
        var target = document.getElementById(marks[i].getAttribute('data-target'));
        if (!target) continue;
        for (var j = 0; j < pages.length; j++) {
            if (pages[j].contains(target)) {
                marks[i].textContent = String(j + 1);
                break;
            }
        }
    }
}

var DELIMITEURS = [
    { left: '\\[', right: '\\]', display: true },
    { left: '\\(', right: '\\)', display: false }
];

function typeset(node) {
    if (window.renderMathInElement) {
        try {
            window.renderMathInElement(node, {
                delimiters: DELIMITEURS,
                throwOnError: false,
                errorColor: '#b00',
                trust: false
            });
        } catch (e) {
            diag('Composition mathématique impossible — ' + e);
        }
    }
    return Promise.resolve();
}

function onTranspiled(res) {
    var seq = ++renderSeq;
    try {
        pageOpts = res.page;
        var d = pageDims(pageOpts);
        document.getElementById('printsize').textContent =
            '@page { size: ' + d.w + 'mm ' + d.h + 'mm; margin: 0; }';
        measure.className = 'doc';
        measure.style.width = d.cw + 'mm';
        measure.style.fontSize = (pageOpts.police || 11) + 'pt';
        measure.style.lineHeight = String(pageOpts.interligne || 1.3);
        measure.innerHTML = res.html;
        var keepScroll = wrapper.scrollTop;
        typeset(measure).catch(function () {}).then(function () {
            if (seq !== renderSeq) return;
            flow(d);
            attachNotes(d);
            fillToc();
            wrapper.scrollTop = keepScroll;
            statusEl.textContent = res.stats || '';
            syncSettingsFromPage();
            diagEl.className = '';
        });
    } catch (e) {
        diag('Rendu impossible — ' + e);
    }
}

window.onTranspiled = onTranspiled;
window.onMessage = function (message, ok) {
    statusEl.textContent = ok ? message : '';
    if (!ok) diag(message); else diagEl.className = '';
};
window.setEditorContent = chargeContenu;

function panneauFermeture() { return document.getElementById('fermeture'); }

window.demandeFermeture = function () {
    if (!modifie) { send({ cmd: 'quitter' }); return; }
    panneauFermeture().className = '';
};

editor.addEventListener('input', function () {
    modifie = true;
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(function () { requestTranspile('inc'); }, 250);
});

function selectionEditeur() {
    return editor.value.slice(editor.selectionStart, editor.selectionEnd);
}

function remplaceSelection(texte) {
    var debut = editor.selectionStart;
    var fin = editor.selectionEnd;
    editor.value = editor.value.slice(0, debut) + texte + editor.value.slice(fin);
    editor.selectionStart = editor.selectionEnd = debut + texte.length;
    modifie = true;
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(function () { requestTranspile('inc'); }, 250);
}

function ecrisPressePapiers(texte) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
        return navigator.clipboard.writeText(texte);
    }
    try {
        document.execCommand('copy');
        return Promise.resolve();
    } catch (e) {
        return Promise.reject(e);
    }
}

document.addEventListener('keydown', function (e) {
    if (!(e.metaKey || e.ctrlKey) || e.altKey) return;
    var touche = (e.key || '').toLowerCase();
    var dansEditeur = document.activeElement === editor;

    if (touche === 'a' && dansEditeur) {
        e.preventDefault();
        editor.select();
        return;
    }
    if (touche === 'c') {
        var choix = dansEditeur ? selectionEditeur() : String(window.getSelection() || '');
        if (!choix) return;
        e.preventDefault();
        ecrisPressePapiers(choix).catch(function () {
            diag('Copie refusée par le système.');
        });
        return;
    }
    if (touche === 'x' && dansEditeur) {
        var coupe = selectionEditeur();
        if (!coupe) return;
        e.preventDefault();
        ecrisPressePapiers(coupe)
            .then(function () { remplaceSelection(''); })
            .catch(function () { diag('Couper refusé par le système.'); });
        return;
    }
    if (touche === 'v' && dansEditeur) {
        if (!navigator.clipboard || !navigator.clipboard.readText) return;
        e.preventDefault();
        navigator.clipboard.readText().then(remplaceSelection).catch(function () {
            diag('Collage refusé par le système.');
        });
    }
}, true);

editor.addEventListener('keydown', function (e) {
    if (e.key === 'Tab') {
        e.preventDefault();
        modifie = true;
        var s = editor.selectionStart;
        editor.value = editor.value.slice(0, s) + '\t' + editor.value.slice(editor.selectionEnd);
        editor.selectionStart = editor.selectionEnd = s + 1;
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(function () { requestTranspile('inc'); }, 250);
    }
});

function setZoom(z) {
    zoom = Math.min(2, Math.max(0.3, z));
    document.documentElement.style.setProperty('--zoom', String(zoom));
}



function documentPourImpression() {
    var copie = document.documentElement.cloneNode(true);
    var scripts = copie.querySelectorAll('script');
    for (var i = 0; i < scripts.length; i++) {
        scripts[i].parentNode.removeChild(scripts[i]);
    }
    return '<!doctype html>\n' + copie.outerHTML;
}

function exportPdf() {
    if (!pageOpts) { diag('Rien à exporter : le document n\'a pas encore été composé.'); return; }
    statusEl.textContent = 'export en cours…';
    send({ cmd: 'export', content: documentPourImpression() });
}

function settingsPanel() { return document.getElementById('settings'); }

function num(id, fallback) {
    var v = parseFloat(document.getElementById(id).value);
    return isFinite(v) ? v : fallback;
}

function syncSettingsFromPage() {
    if (!pageOpts) return;
    var panel = settingsPanel();
    if (panel.contains(document.activeElement)) return;
    document.getElementById('set-orientation').value =
        (pageOpts.orientation === 'paysage' || pageOpts.orientation === 'landscape') ? 'paysage' : 'portrait';
    var ids = ['mar-top', 'mar-right', 'mar-bottom', 'mar-left'];
    var i;
    for (i = 0; i < 4; i++) document.getElementById(ids[i]).value = pageOpts.marges[i];
    var eids = ['esp-top', 'esp-right', 'esp-bottom', 'esp-left'];
    for (i = 0; i < 4; i++) document.getElementById(eids[i]).value = pageOpts.espacements[i];
    document.getElementById('set-police').value = pageOpts.police;
    document.getElementById('set-interligne').value = pageOpts.interligne;
}

function quad(a, b, c, e) {
    if (a === b && b === c && c === e) return String(a);
    return '{' + a + ';' + b + ';' + c + ';' + e + '}';
}

function serializePageBlock() {
    return 'page {\n\torientation: ' + document.getElementById('set-orientation').value + ';\n'
        + '\tmarges: ' + quad(num('mar-top', 20), num('mar-right', 20), num('mar-bottom', 20), num('mar-left', 20)) + ';\n'
        + '\tespacements: ' + quad(num('esp-top', 2), num('esp-right', 2), num('esp-bottom', 2), num('esp-left', 2)) + ';\n'
        + '\ttaille: ' + num('set-police', 11) + ';\n'
        + '\tinterligne: ' + num('set-interligne', 1.3) + ';\n}';
}

function findPageBlock(text) {
    var m = /(^|\n)\s*page\s*\{/.exec(text);
    if (!m) return null;
    var open = m.index + m[0].length - 1;
    var depth = 0;
    for (var i = open; i < text.length; i++) {
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
    var block = serializePageBlock();
    var src = editor.value;
    var found = findPageBlock(src);
    editor.value = found
        ? src.slice(0, found.start) + block + src.slice(found.end)
        : block + '\n\n' + src;
    requestTranspile('full');
}

document.getElementById('btn-load').addEventListener('click', function () { send({ cmd: 'load' }); });
document.getElementById('btn-save').addEventListener('click', function () {
    modifie = false;
    send({ cmd: 'save', content: editor.value });
});
document.getElementById('btn-quitter').addEventListener('click', function () { send({ cmd: 'quitter' }); });
document.getElementById('btn-rester').addEventListener('click', function () {
    panneauFermeture().className = 'hidden';
});
document.getElementById('btn-export').addEventListener('click', exportPdf);
document.getElementById('btn-settings').addEventListener('click', function () {
    var panel = settingsPanel();
    panel.className = panel.className === 'hidden' ? '' : 'hidden';
    syncSettingsFromPage();
});
document.getElementById('btn-apply').addEventListener('click', applySettings);
document.getElementById('btn-zoom-in').addEventListener('click', function () { setZoom(zoom + 0.1); });
document.getElementById('btn-zoom-out').addEventListener('click', function () { setZoom(zoom - 0.1); });

setZoom(zoom);

if (!ipcAvailable()) {
    diag("Le pont IPC n'est pas encore disponible au chargement. Si ce message persiste après un clic sur un bouton, la vue web n'a pas reçu le script d'initialisation de wry.");
}

editor.value = '';
requestTranspile('full');
