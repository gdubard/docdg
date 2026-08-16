/**
 * @typedef {Object} PageOpts
 * @property {string} orientation
 * @property {number[]} marges
 * @property {number[]} espacements
 * @property {string} script
 * @property {string} seyes
 * @property {string} math
 * @property {string} titre
 * @property {string} auteur
 * @property {string} institution
 * @property {string} date
 * @property {number} taille
 * @property {number} interligne
 * @property {number} tabulation
 * @property {number} hauteur
 * @property {number} decalage
 * @property {number} precision
 * @property {boolean} cesure
 * @property {number} veuves
 * @property {number} orphelines
 */

/**
 * @typedef {Object} PrefsInterface
 * @property {string} disposition
 * @property {number} part
 * @property {boolean} code
 * @property {boolean} apercu
 * @property {number} zoom
 */

/**
 * Un contrôle de formulaire du panneau de réglages, typé pour l'accès
 * à `.value`. Nom distinct de la variable locale `champ` employée pour
 * les champs de saisie du document, qui la masquerait.
 * @param {string} id
 * @returns {HTMLInputElement|HTMLSelectElement}
 */
/**
 * Remonte du point d'un événement jusqu'à l'ancêtre correspondant au
 * sélecteur, ou rend null si la cible n'est pas un élément.
 * @param {EventTarget} cible
 * @param {string} selecteur
 * @returns {Element|null}
 */
function element(cible, selecteur) {
    return cible instanceof Element ? cible.closest(selecteur) : null;
}

function controle(id) {
    return /** @type {HTMLInputElement|HTMLSelectElement} */ (document.getElementById(id));
}

function coche(id) {
    return /** @type {HTMLInputElement} */ (document.getElementById(id));
}

const editor = /** @type {HTMLTextAreaElement} */ (document.getElementById('editor'));
const pagesEl = document.getElementById('pages');
const measure = document.getElementById('measure');
const statusEl = document.getElementById('status');
const diagEl = document.getElementById('diag');
const wrapper = document.getElementById('preview-wrapper');

/** @type {PageOpts} */
const DEFAUTS_PAGE = {
    orientation: 'portrait',
    marges: [20, 20, 20, 20],
    espacements: [2, 2, 2, 2],
    script: '', math: '', seyes: '',
    titre: '', auteur: '', institution: '', date: '',
    taille: 11, interligne: 1.3,
    tabulation: 8, hauteur: 8, decalage: 100, precision: -1,
    cesure: true, veuves: 2, orphelines: 2, numerotation: 'composee'
};

/** @type {PageOpts} */
let pageOpts = Object.assign({}, DEFAUTS_PAGE);
let documentCompose = false;
let renderSeq = 0;
/** @type {Partial<PrefsInterface>} */
const prefsInitiales = window['__PREFS__'] || {};

/**
 * Le compte à rebours de la version.
 *
 * Discret tant que le terme est loin — un chiffre gris dans la barre — il
 * passe en avertissement dans les derniers jours. Le but n'est pas d'alarmer
 * mais d'éviter la surprise : personne ne doit découvrir l'échéance le matin
 * où il en a besoin.
 */
function afficheEcheance() {
    const e = window['__ECHEANCE__'];
    const zone = document.getElementById('echeance');
    if (!zone || !e || typeof e.jours !== 'number') return;
    const j = e.jours;
    const preavis = typeof e.preavis === 'number' ? e.preavis : 15;
    zone.textContent = j === 0
        ? 'dernier jour de cette version'
        : (j === 1 ? 'un jour restant' : j + ' jours restants');
    zone.title = 'Chaque version de docdg est valable un temps limité. '
        + 'Passé ce délai, installez la version suivante — vos documents ne sont pas affectés.';
    zone.classList.remove('hidden');
    zone.classList.toggle('proche', j <= preavis);
}
let zoom = prefsInitiales.zoom || 0.9;
let debounceTimer = null;
let modifie = false;

/*
 * Le document a-t-il changé depuis le dernier enregistrement ? Le panneau de
 * fermeture le demandait au moment de partir ; la pastille rouge le dit en
 * continu, dans la barre, à l'endroit où l'œil cherche le nom du fichier.
 */
function marqueModifie(valeur) {
    modifie = valeur;
    const p = document.getElementById('modifie');
    if (p) p.className = valeur ? '' : 'hidden';
}
/** @type {Object<string, string>} */
let saisies = {};
/** @type {Object<string, string>} */
let brouillons = {};
let focaliseSaisie = false;

function diag(message) {
    diagEl.textContent = message;
    diagEl.className = 'on';
}

window.addEventListener('error', (e) => {
    diag(`Erreur JavaScript ligne ${e.lineno}:${e.colno} — ${e.message}`);
});

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
    send({ cmd: 'render', content: editor.value, mode: mode || 'inc', saisies });
}

function recompose() {
    marqueModifie(true);
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => requestTranspile('inc'), 250);
}

/** @param {string} text */
function chargeContenu(text) {
    editor.value = text;
    // Le sélecteur de fichier est une fenêtre du système : en se refermant, il
    // ne rend pas toujours la main proprement à la vue web, qui cesse alors de
    // recevoir la molette. On la lui redonne explicitement.
    window.focus();
    editor.focus();
    histReinitialise();
    marqueModifie(false);
    saisies = {};
    brouillons = {};
    // À l'ouverture, personne n'est en train d'écrire : si le document
    // s'arrête sur une saisie, le curseur doit s'y trouver, pas dans
    // l'éditeur. Le drapeau ne vaut que pour ce rendu-là.
    focaliseSaisie = true;
    requestTranspile('full');
}

/**
 * Règles de validation des saisies, par type.
 * @type {Map<string, {valide: function(string): boolean, message: string}>}
 */
const REGLES_SAISIE = new Map([
    ['texte', {
        valide: (v) => v.trim().length > 0,
        message: 'Une réponse est attendue.'
    }],
    ['entier', {
        valide: (v) => /^[+-]?\d+$/.test(v.trim()),
        message: 'Un entier est attendu — par exemple 12.'
    }],
    ['décimal', {
        valide: (v) => /^[+-]?\d+(,\d+)?$/.test(v.trim()),
        message: 'Un nombre décimal est attendu, écrit avec une virgule — par exemple 1,65.'
    }],
    ['booléen', {
        valide: (v) => /^(vrai|faux)$/.test(v.trim().toLowerCase()),
        message: 'Réponds par vrai ou faux.'
    }],
    ['caractère', {
        valide: (v) => [...v.trim()].length === 1,
        message: 'Un seul caractère est attendu.'
    }]
]);

function montreErreurSaisie(bloc, message) {
    const erreur = bloc.querySelector('.saisie-erreur');
    if (!erreur) return;
    erreur.textContent = message;
    erreur.classList.add('on');
    clearTimeout(Number(erreur.dataset.minuterie));
    erreur.dataset.minuterie = String(setTimeout(() => erreur.classList.remove('on'), 2800));
}

/** @param {HTMLInputElement} champ */
function soumetSaisie(champ) {
    const bloc = champ.closest('.saisie');
    if (!bloc) return;
    const nom = bloc.getAttribute('data-nom');
    const type = bloc.getAttribute('data-type');
    const regle = REGLES_SAISIE.get(type) || REGLES_SAISIE.get('texte');
    if (!regle.valide(champ.value)) {
        montreErreurSaisie(bloc, regle.message);
        champ.focus();
        return;
    }
    delete brouillons[nom];
    saisies[nom] = champ.value.trim();
    focaliseSaisie = true;
    requestTranspile('inc');
}

function restaureSaisie() {
    const champ = /** @type {HTMLInputElement} */ (pagesEl.querySelector('.saisie-champ'));
    if (!champ) { focaliseSaisie = false; return; }
    const bloc = champ.closest('.saisie');
    const nom = bloc ? bloc.getAttribute('data-nom') : null;
    if (nom && brouillons[nom] !== undefined) champ.value = brouillons[nom];
    if (focaliseSaisie || document.activeElement !== editor) {
        focaliseSaisie = false;
        champ.focus();
        champ.setSelectionRange(champ.value.length, champ.value.length);
    }
}

pagesEl.addEventListener('keydown', (e) => {
    if (e.key !== 'Enter') return;
    const champ = /** @type {HTMLInputElement} */ (element(e.target, '.saisie-champ'));
    if (!champ) return;
    e.preventDefault();
    soumetSaisie(champ);
});

pagesEl.addEventListener('input', (e) => {
    const champ = /** @type {HTMLInputElement} */ (element(e.target, '.saisie-champ'));
    if (!champ) return;
    const bloc = champ.closest('.saisie');
    const nom = bloc ? bloc.getAttribute('data-nom') : null;
    if (nom) brouillons[nom] = champ.value;
});

pagesEl.addEventListener('click', (e) => {
    const lien = element(e.target, 'a[href^="#"]');
    if (lien) {
        e.preventDefault();
        const href = lien.getAttribute('href') || '';
        const cible = document.getElementById(href.slice(1));
        if (cible) cible.scrollIntoView({ behavior: 'smooth', block: 'center' });
        return;
    }
    const valeur = element(e.target, '.saisie-valeur');
    if (!valeur) return;
    const nom = valeur.getAttribute('data-nom');
    brouillons[nom] = saisies[nom];
    delete saisies[nom];
    focaliseSaisie = true;
    requestTranspile('inc');
});

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
    s.style.fontSize = `${pageOpts.taille || 11}pt`;
    s.style.lineHeight = String(pageOpts.interligne || 1.3);
    if (pageOpts.script) s.style.fontFamily = `'${pageOpts.script}', Georgia, 'Times New Roman', serif`;
    s.style.setProperty('--decalage', String((pageOpts.decalage || 100) / 100));
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

function analyseLignes(p) {
    const echelle = p.offsetWidth ? p.getBoundingClientRect().width / p.offsetWidth : 1;
    const marche = document.createTreeWalker(p, NodeFilter.SHOW_TEXT);
    const portee = document.createRange();
    const hauts = [];
    const departs = [];
    let noeud;
    while ((noeud = marche.nextNode())) {
        const t = noeud.textContent;
        let i = 0;
        while (i < t.length) {
            while (i < t.length && /\s/.test(t[i])) i++;
            if (i >= t.length) break;
            let j = i;
            while (j < t.length && !/\s/.test(t[j])) j++;
            portee.setStart(noeud, i);
            portee.setEnd(noeud, j);
            const rects = Array.from(portee.getClientRects()).filter((r) => r.width > 0 && r.height > 0);
            for (let k = 0; k < rects.length; k++) {
                const haut = rects[k].top / echelle;
                const h = rects[k].height / echelle;
                if (!hauts.length || haut - hauts[hauts.length - 1] > h * 0.5) {
                    hauts.push(haut);
                    if (k === 0) departs.push({ noeud, decalage: i, ligne: hauts.length - 1 });
                }
            }
            i = j;
        }
    }
    return { hauts, departs };
}

function scinde(p, depart) {
    const portee = document.createRange();
    const parent = depart.noeud.parentElement;
    const englobe = parent && parent.closest('.katex, .katex-display, svg');
    if (englobe && p.contains(englobe)) portee.setStartBefore(englobe);
    else portee.setStart(depart.noeud, depart.decalage);
    portee.setEndAfter(p.lastChild);
    const suite = p.cloneNode(false);
    suite.appendChild(portee.extractContents());
    suite.classList.add('alinea-suite');
    p.classList.add('alinea-coupe');
    return suite;
}

function coupeTableau(t, limite) {
    const entete = t.querySelector('tr.tab-entete');
    const rangees = Array.from(t.rows).filter((r) => r !== entete);
    if (rangees.length < 2) {
        diag(`Tableau non scindé : ${rangees.length} rangée(s) seulement.`);
        return null;
    }
    const hote = rangees[0].parentNode;
    const reportees = [];
    while (rangees.length > 1 && t.offsetHeight > limite) {
        const r = rangees.pop();
        hote.removeChild(r);
        reportees.unshift(r);
    }
    if (!reportees.length || t.offsetHeight > limite) {
        for (const r of reportees) hote.appendChild(r);
        if (reportees.length) {
            diag(`Tableau non scindé : aucune rangée ne tient dans ${Math.round(limite)} px.`);
        }
        return null;
    }
    const suite = t.cloneNode(false);
    suite.classList.add('tab-suite');
    const corps = document.createElement('tbody');
    if (entete) corps.appendChild(entete.cloneNode(true));
    for (const r of reportees) corps.appendChild(r);
    suite.appendChild(corps);
    return suite;
}

/*
 * La prose de calcul — résolutions pas à pas, études, positions relatives —
 * est une suite de paragraphes et de formules : elle se scinde par nature.
 * On garde les enfants entiers qui tiennent, on scinde le paragraphe
 * frontière s'il s'y prête, et le reste part en page suivante dans une
 * coquille identique. Une formule hors texte, elle, ne se coupe jamais.
 */
function coupeProse(b, limite, orphelines, veuves) {
    const enfants = Array.from(b.children);
    if (enfants.length < 2) return null;
    const haut = b.getBoundingClientRect().top;
    const echelle = b.offsetWidth ? b.getBoundingClientRect().width / b.offsetWidth : 1;
    let garde = 0;
    for (const c of enfants) {
        const r = c.getBoundingClientRect();
        if ((r.bottom - haut) / echelle <= limite) garde += 1;
        else break;
    }
    let reste = null;
    if (garde < enfants.length && enfants[garde].tagName === 'P') {
        const frontiere = enfants[garde];
        const dedans = (frontiere.getBoundingClientRect().top - haut) / echelle;
        reste = coupeParagraphe(frontiere, limite - dedans, orphelines, veuves);
        if (reste) garde += 1;
    }
    if (garde === 0 || garde >= enfants.length) return null;
    const suite = b.cloneNode(false);
    if (reste) suite.appendChild(reste);
    for (const c of enfants.slice(garde)) suite.appendChild(c);
    return suite;
}

/*
 * Le cahier se scinde comme la prose : on garde les lignes entières qui
 * tiennent, on scinde la ligne frontière à une frontière de ligne écrite, et
 * le reste part en page suivante dans une coquille identique.
 *
 * La réglure, elle, ne se coupe pas : elle se redessine. Chaque moitié reçoit
 * la sienne, posée depuis son propre bord haut, si bien que la page suivante
 * recommence une feuille entière — coiffe comprise. Elle est aussi la seule
 * enfant du bloc qui ne soit pas une ligne : positionnée, elle couvre toute
 * la hauteur, et la compter parmi ce qui doit tenir ferait échouer la coupe
 * dès la première mesure.
 */
function coupeLignes(b, limite, orphelines, veuves) {
    const reglure = b.querySelector(':scope > docdg-reglure');
    const enfants = Array.from(b.children).filter((c) => c !== reglure);
    if (!enfants.length) return null;
    const haut = b.getBoundingClientRect().top;
    const echelle = b.offsetWidth ? b.getBoundingClientRect().width / b.offsetWidth : 1;
    let garde = 0;
    for (const c of enfants) {
        const r = c.getBoundingClientRect();
        if ((r.bottom - haut) / echelle <= limite) garde += 1;
        else break;
    }
    let reste = null;
    if (garde < enfants.length) {
        const frontiere = enfants[garde];
        const dedans = (frontiere.getBoundingClientRect().top - haut) / echelle;
        reste = coupeParagraphe(frontiere, limite - dedans, orphelines, veuves);
        if (reste) garde += 1;
    }
    if (garde === 0 || garde >= enfants.length) return null;
    const suite = b.cloneNode(false);
    if (reglure) suite.appendChild(reglure.cloneNode(true));
    if (reste) suite.appendChild(reste);
    for (const c of enfants.slice(garde)) suite.appendChild(c);
    return suite;
}

function coupeBloc(b, limite, orphelines, veuves) {
    if (b.tagName === 'DOCDG-LIGNES') return coupeLignes(b, limite, orphelines, veuves);
    if (b.tagName === 'P') return coupeParagraphe(b, limite, orphelines, veuves);
    if (b.classList.contains('calcul-prose')) return coupeProse(b, limite, orphelines, veuves);
    if (!b.classList.contains('secable')) {
        if (b.tagName === 'TABLE' || b.classList.contains('cadre')) {
            diag(`${b.tagName === 'TABLE' ? 'Tableau' : 'Cadre'} non scindé : bloc insécable (classes « ${b.className} »).`);
        }
        return null;
    }
    if (b.tagName === 'TABLE') return coupeTableau(b, limite);
    const corps = Array.from(b.querySelectorAll('.cadre-corps > p'));
    const haut = b.getBoundingClientRect().top;
    const echelle = b.offsetWidth ? b.getBoundingClientRect().width / b.offsetWidth : 1;
    for (let i = corps.length - 1; i >= 0; i--) {
        const p = corps[i];
        const dedans = (p.getBoundingClientRect().top - haut) / echelle;
        if (dedans >= limite) continue;
        const reste = coupeParagraphe(p, limite - dedans, orphelines, veuves);
        if (!reste) continue;
        const suite = b.cloneNode(true);
        const jumeaux = Array.from(suite.querySelectorAll('.cadre-corps > p'));
        const titre = suite.querySelector('.cadre-titre');
        if (titre) titre.remove();
        for (let k = 0; k <= i; k++) jumeaux[k].remove();
        const zone = suite.querySelector('.cadre-corps');
        if (zone) zone.insertBefore(reste, zone.firstChild);
        while (corps.length > i + 1) corps.pop().remove();
        suite.classList.add('cadre-suite');
        b.classList.add('cadre-coupe');
        return suite;
    }
    return null;
}

function coupeParagraphe(p, limite, orphelines, veuves) {
    const { hauts, departs } = analyseLignes(p);
    const total = hauts.length;
    if (total < orphelines + veuves) return null;
    const interligne = total > 1 ? (hauts[total - 1] - hauts[0]) / (total - 1) : p.offsetHeight;
    let tiennent = 0;
    for (let i = 0; i < total; i++) {
        if (hauts[i] - hauts[0] + interligne <= limite) tiennent = i + 1;
    }
    let garde = Math.min(tiennent, total - veuves);
    while (garde >= orphelines) {
        const depart = departs.find((x) => x.ligne === garde);
        if (depart) return scinde(p, depart);
        garde--;
    }
    return null;
}

/*
 * La flottaison limitée. Quand un bloc insécable — figure, tableau de
 * variations — est reporté en page suivante, la page qu'il quitte garderait
 * un grand blanc : les blocs qui le suivent remontent le combler, tant
 * qu'ils tiennent en entier. Trois interdits préservent le sens de lecture :
 * jamais au-delà d'un titre, jamais un saut de page, et l'on s'arrête au
 * premier bloc qui ne tient pas — seul l'objet reporté « flotte », le texte
 * garde son ordre.
 */
function remonteFlottants(page, file, usable, hautPx) {
    let examines = 0;
    while (file.length && examines < 8) {
        const c = file[0];
        if (!c.classList
            || c.classList.contains('sec')
            || c.classList.contains('pagebreak')) {
            break;
        }
        page.appendChild(c);
        const bas = c.offsetTop + c.offsetHeight - page.offsetTop - hautPx;
        if (bas > usable) {
            page.removeChild(c);
            break;
        }
        file.shift();
        examines += 1;
    }
}

function flow(d) {
    pagesEl.innerHTML = '';
    const usable = mmToPx(d.ch) - 2;
    const hautPx = mmToPx(d.mt);
    const orphelines = Math.max(1, Math.round(pageOpts.orphelines || 2));
    const veuves = Math.max(1, Math.round(pageOpts.veuves || 2));
    let sheet = newPage(d);
    const file = Array.from(measure.children);
    while (file.length) {
        const b = file.shift();
        if (b.classList && b.classList.contains('pagebreak')) {
            sheet = newPage(d);
            continue;
        }
        sheet.appendChild(b);
        const used = b.offsetTop + b.offsetHeight - sheet.offsetTop - hautPx;
        if (used <= usable) continue;
        const titre = b.previousElementSibling;
        const suit = titre && titre.classList
            && titre.classList.contains('sec')
            && sheet.children.length > 2;
        const limite = usable - (b.offsetTop - sheet.offsetTop - hautPx);
        const reste = coupeBloc(b, limite, orphelines, veuves);
        if (reste) {
            sheet = newPage(d);
            file.unshift(reste);
            continue;
        }
        if (sheet.children.length <= 1) continue;
        const pageQuittee = sheet;
        sheet = newPage(d);
        if (suit) sheet.appendChild(titre);
        sheet.appendChild(b);
        // Le report laisse un blanc derrière lui : les blocs suivants le
        // comblent — sauf si le bloc a entraîné son titre, car remonter du
        // contenu le placerait avant le titre de sa propre section.
        if (!suit) {
            remonteFlottants(pageQuittee, file, usable, hautPx);
        }
        const deborde = b.offsetTop + b.offsetHeight - sheet.offsetTop - hautPx;
        if (deborde > usable) {
            const suite = coupeBloc(b, usable - (b.offsetTop - sheet.offsetTop - hautPx), orphelines, veuves);
            if (suite) {
                sheet = newPage(d);
                file.unshift(suite);
            }
        }
    }
    const pages = pagesEl.querySelectorAll('.page');
    const mode = pageOpts.numerotation || 'composee';
    pages.forEach((page, i) => {
        const no = page.querySelector('.pageno');
        if (mode === 'sans') {
            no.textContent = '';
        } else if (mode === 'simple') {
            no.textContent = String(i + 1);
        } else {
            no.textContent = `${i + 1} / ${pages.length}`;
        }
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

function onTranspiled(res) {
    const seq = ++renderSeq;
    try {
        pageOpts = Object.assign({}, DEFAUTS_PAGE, res.page);
        documentCompose = true;
        const d = pageDims(pageOpts);
        document.getElementById('printsize').textContent =
            `@page { size: ${d.w}mm ${d.h}mm; margin: 0; }`;
        measure.className = 'doc';
        measure.style.width = `${d.cw}mm`;
        measure.style.fontSize = `${pageOpts.taille || 11}pt`;
        measure.style.lineHeight = String(pageOpts.interligne || 1.3);
        if (pageOpts.script) measure.style.fontFamily = `'${pageOpts.script}', Georgia, 'Times New Roman', serif`;
        else measure.style.fontFamily = '';
        measure.style.setProperty('--decalage', String((pageOpts.decalage || 100) / 100));
        measure.innerHTML = res.html;
        const keepScroll = wrapper.scrollTop;
        typeset(measure).catch(() => {}).then(() => {
            if (seq !== renderSeq) return;
            flow(d);
            attachNotes();
            fillToc();
            restaureSaisie();
            ajusteBoite();
            wrapper.scrollTop = keepScroll;
            statusEl.textContent = res.stats || '';
            syncSettingsFromPage();
            diagEl.className = '';
        });
    } catch (e) {
        diag(`Rendu impossible — ${e}`);
    }
}

window['onTranspiled'] = onTranspiled;
window['onMessage'] = (message, ok) => {
    statusEl.textContent = ok ? message : '';
    if (!ok) diag(message); else diagEl.className = '';
};
window['setEditorContent'] = chargeContenu;

document.addEventListener('click', (e) => {
    const lien = element(e.target, 'a[href]');
    if (!lien) return;
    e.preventDefault();
    const href = lien.getAttribute('href') || '';
    if (href.startsWith('#')) {
        const cible = document.getElementById(href.slice(1));
        if (cible) cible.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
}, true);

window.addEventListener('beforeunload', (e) => {
    e.preventDefault();
});

function panneauFermeture() { return document.getElementById('fermeture'); }

/*
 * La croix de la fenêtre pose toujours la question : fermer est irréversible,
 * et l'on ne sait jamais si le clic était voulu. La question dit l'état du
 * document — enregistré ou non —, la réponse tient en oui ou non, et c'est
 * « Non » qui a la main, pour que la touche Entrée ne quitte pas.
 */
window['demandeFermeture'] = () => {
    document.getElementById('fermeture-question').textContent =
        modifie ? 'Quitter sans enregistrer\u00A0?' : 'Quitter docdg\u00A0?';
    panneauFermeture().className = '';
    document.getElementById('btn-rester').focus();
};

/*
 * Annuler / rétablir. Le textarea perd son historique natif dès que le code
 * réécrit `editor.value` (tabulation, réglages de page, chargement) : il faut
 * donc tenir l'historique nous-mêmes. Les frappes rapprochées sont groupées
 * par paquets de 400 ms — annuler remonte par gestes, pas caractère par
 * caractère.
 */
const histAvant = [];
const histApres = [];
let histDernierPush = 0;
let histEtatConnu = { texte: '', debut: 0, fin: 0 };
const btnAnnuler = document.getElementById('btn-annuler');
const btnRetablir = document.getElementById('btn-retablir');

function histEtat() {
    return { texte: editor.value, debut: editor.selectionStart, fin: editor.selectionEnd };
}

function majBoutonsHistorique() {
    btnAnnuler.disabled = histAvant.length === 0;
    btnRetablir.disabled = histApres.length === 0;
}

/** Enregistre l'état d'avant la mutation en cours. `force` isole les gestes
 *  ponctuels (tabulation, réglages) du flot des frappes. */
function histPousse(force) {
    const maintenant = Date.now();
    if (force || maintenant - histDernierPush > 400) {
        histAvant.push(histEtatConnu);
        if (histAvant.length > 500) {
            histAvant.shift();
        }
        histDernierPush = maintenant;
    }
    histApres.length = 0;
    majBoutonsHistorique();
}

function histRestaure(etat) {
    editor.value = etat.texte;
    editor.setSelectionRange(etat.debut, etat.fin);
    editor.focus();
    histEtatConnu = histEtat();
    majBoutonsHistorique();
    recompose();
}

function annuler() {
    if (histAvant.length === 0) {
        return;
    }
    histApres.push(histEtat());
    histRestaure(histAvant.pop());
}

function retablir() {
    if (histApres.length === 0) {
        return;
    }
    histAvant.push(histEtat());
    histRestaure(histApres.pop());
}

function histReinitialise() {
    histAvant.length = 0;
    histApres.length = 0;
    histEtatConnu = histEtat();
    histDernierPush = 0;
    majBoutonsHistorique();
}

btnAnnuler.addEventListener('click', annuler);
btnRetablir.addEventListener('click', retablir);

editor.addEventListener('input', () => {
    histPousse(false);
    histEtatConnu = histEtat();
    recompose();
});

function remplaceSelection(texte) {
    histPousse(true);
    const debut = editor.selectionStart;
    const fin = editor.selectionEnd;
    editor.value = editor.value.slice(0, debut) + texte + editor.value.slice(fin);
    editor.selectionStart = editor.selectionEnd = debut + texte.length;
    histEtatConnu = histEtat();
    editor.focus();
    recompose();
}

editor.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
        e.preventDefault();
        remplaceSelection('\t');
        return;
    }
    const raccourci = e.ctrlKey || e.metaKey;
    if (raccourci && !e.shiftKey && (e.key === 'z' || e.key === 'Z')) {
        e.preventDefault();
        annuler();
    } else if ((raccourci && (e.key === 'y' || e.key === 'Y'))
        || (raccourci && e.shiftKey && (e.key === 'z' || e.key === 'Z'))) {
        e.preventDefault();
        retablir();
    }
});

function setZoom(z) {
    zoom = Math.min(2, Math.max(0.3, z));
    document.documentElement.style.setProperty('--zoom', String(zoom));
    ajusteBoite();
}

/*
 * Une transformation d'échelle ne change pas la boîte de mise en page : elle
 * peint plus grand au même encombrement. Au-delà de 100 %, le bas des pages
 * sortait donc de la zone défilable sans qu'aucune barre n'apparaisse — le
 * document était là, hors d'atteinte. En deçà, l'inverse : un blanc en pied,
 * proportionnel à la réduction. On rend au conteneur la hauteur que la
 * transformation lui prend ou lui donne.
 *
 * `offsetHeight` est la hauteur de mise en page, que la transformation ignore :
 * la mesure ne dépend pas de son propre résultat, et rien ne s'emballe.
 */
function ajusteBoite() {
    if (!pagesEl) return;
    const h = pagesEl.offsetHeight;
    pagesEl.style.marginBottom = h ? `${Math.round(h * (zoom - 1))}px` : '';
}

function documentPourImpression() {
    const copie = document.documentElement.cloneNode(true);
    for (const script of copie.querySelectorAll('script')) {
        script.remove();
    }
    return `<!doctype html>\n${copie.outerHTML}`;
}

function exportPdf() {
    if (!documentCompose) { diag("Rien à exporter : le document n'a pas encore été composé."); return; }
    statusEl.textContent = 'export en cours…';
    send({ cmd: 'export', content: documentPourImpression() });
}

function settingsPanel() { return document.getElementById('settings'); }

function num(id, fallback) {
    const v = parseFloat(controle(id).value);
    return isFinite(v) ? v : fallback;
}

function syncSettingsFromPage() {
    if (!documentCompose) return;
    const panel = settingsPanel();
    if (panel.contains(document.activeElement)) return;
    controle('set-orientation').value =
        (pageOpts.orientation === 'paysage' || pageOpts.orientation === 'landscape') ? 'paysage' : 'portrait';
    const ids = ['mar-top', 'mar-right', 'mar-bottom', 'mar-left'];
    ids.forEach((id, i) => { controle(id).value = String(pageOpts.marges[i]); });
    const eids = ['esp-top', 'esp-right', 'esp-bottom', 'esp-left'];
    eids.forEach((id, i) => { controle(id).value = String(pageOpts.espacements[i]); });
    controle('set-police').value = String(pageOpts.taille);
    controle('set-interligne').value = String(pageOpts.interligne);
    controle('doc-titre').value = pageOpts.titre || '';
    controle('doc-auteur').value = pageOpts.auteur || '';
    controle('doc-institution').value = pageOpts.institution || '';
    controle('doc-date').value = pageOpts.date || '';
    controle('doc-police').value = pageOpts.script || '';
    controle('doc-seyes').value = pageOpts.seyes || '';
    controle('doc-math').value = pageOpts.math || '';
    controle('doc-tabulation').value = String(pageOpts.tabulation);
    controle('doc-hauteur').value = String(pageOpts.hauteur);
    controle('doc-decalage').value = String(pageOpts.decalage);
    controle('doc-precision').value = String(pageOpts.precision);
    coche('doc-cesure').checked = pageOpts.cesure !== false;
    controle('doc-orphelines').value = String(pageOpts.orphelines);
    controle('doc-veuves').value = String(pageOpts.veuves);
}

function quad(a, b, c, d) {
    if (a === b && b === c && c === d) return String(a);
    return `{${a};${b};${c};${d}}`;
}

function texte(id) {
    return controle(id).value.trim();
}

function serializePageBlock() {
    let bloc = 'document {\n';
    for (const [cle, id] of [['titre', 'doc-titre'], ['auteur', 'doc-auteur'], ['institution', 'doc-institution'], ['date', 'doc-date']]) {
        const v = texte(id);
        if (v) bloc += '\t' + cle + ': ' + v + ';\n';
    }
    bloc += '\torientation: ' + controle('set-orientation').value + ';\n'
        + '\tmarges: ' + quad(num('mar-top', 20), num('mar-right', 20), num('mar-bottom', 20), num('mar-left', 20)) + ';\n'
        + '\tespacements: ' + quad(num('esp-top', 2), num('esp-right', 2), num('esp-bottom', 2), num('esp-left', 2)) + ';\n';
    const script = texte('doc-police');
    if (script) bloc += '\tscript: ' + script + ';\n';
    const seyes = texte('doc-seyes');
    if (seyes) bloc += '\tseyès: ' + seyes + ';\n';
    const math = texte('doc-math');
    if (math) bloc += '\tmath: ' + math + ';\n';
    bloc += '\ttaille: ' + num('set-police', 11) + ';\n'
        + '\tinterligne: ' + num('set-interligne', 1.3) + ';\n';
    const tab = num('doc-tabulation', 8);
    if (tab !== 8) bloc += '\ttabulation: ' + tab + ';\n';
    const haut = num('doc-hauteur', 8);
    if (haut !== 8) bloc += '\thauteur: ' + haut + ';\n';
    const dec = num('doc-decalage', 100);
    if (dec !== 100) bloc += '\tdécalage: ' + dec + ';\n';
    const prec = num('doc-precision', -1);
    if (prec !== -1) bloc += '\tprécision: ' + prec + ';\n';
    if (!coche('doc-cesure').checked) bloc += '\tcésure: non;\n';
    const orph = num('doc-orphelines', 2);
    if (orph !== 2) bloc += '\torphelines: ' + orph + ';\n';
    const veuv = num('doc-veuves', 2);
    if (veuv !== 2) bloc += '\tveuves: ' + veuv + ';\n';
    // L'option n'a pas de widget : elle se recopie depuis le document pour
    // que l'ouverture des réglages ne l'efface pas.
    if (pageOpts.numerotation === 'simple') bloc += '\tnumérotation: simple;\n';
    if (pageOpts.numerotation === 'sans') bloc += '\tnumérotation: sans;\n';
    return bloc + '}';
}

function findPageBlock(text) {
    const motif = /(^|\n)\s*(?:page|document)\s*\{/;
    let debut = null;
    let fin = 0;
    for (;;) {
        const zone = text.slice(fin);
        const m = motif.exec(zone);
        if (!m) break;
        const prefixe = zone.slice(0, m.index + (m[1] ? m[1].length : 0));
        if (debut !== null && prefixe.trim() !== '') break;
        const open = fin + m.index + m[0].length - 1;
        let depth = 0;
        let ferme = -1;
        for (let i = open; i < text.length; i++) {
            if (text[i] === '{') depth++;
            else if (text[i] === '}') {
                depth--;
                if (depth === 0) { ferme = i + 1; break; }
            }
        }
        if (ferme < 0) break;
        if (debut === null) debut = fin + m.index + (m[1] ? m[1].length : 0);
        fin = ferme;
    }
    return debut === null ? null : { start: debut, end: fin };
}

function applySettings() {
    histPousse(true);
    marqueModifie(true);
    const block = serializePageBlock();
    const src = editor.value;
    const found = findPageBlock(src);
    editor.value = found
        ? src.slice(0, found.start) + block + src.slice(found.end)
        : `${block}\n\n${src}`;
    histEtatConnu = histEtat();
    // Appliquer, c'est en avoir fini : le panneau se referme sur le document
    // qu'il vient de changer, sans qu'on ait à rouvrir les paramètres pour le
    // voir.
    settingsPanel().className = 'hidden';
    requestTranspile('full');
}

document.getElementById('btn-load').addEventListener('click', () => send({ cmd: 'load' }));
// La pastille ne s'éteint pas au clic : le sélecteur peut être annulé,
// l'écriture peut échouer. C'est le moteur qui l'éteint, sur succès.
document.getElementById('btn-save').addEventListener('click', () => {
    send({ cmd: 'save', content: editor.value });
});
document.getElementById('btn-quitter').addEventListener('click', () => send({ cmd: 'quitter' }));
document.getElementById('btn-rester').addEventListener('click', () => {
    panneauFermeture().className = 'hidden';
    send({ cmd: 'rester' });
});
// Échap est le « non » du clavier : il emprunte le bouton, et non un chemin
// parallèle, pour qu'il n'y ait qu'une façon de rester.
panneauFermeture().addEventListener('keydown', (e) => {
    if (e.key === 'Escape') { document.getElementById('btn-rester').click(); }
});
document.getElementById('btn-export').addEventListener('click', exportPdf);
document.getElementById('btn-settings').addEventListener('click', () => {
    const panel = settingsPanel();
    panel.className = panel.className === 'hidden' ? '' : 'hidden';
    syncSettingsFromPage();
});
document.getElementById('btn-apply').addEventListener('click', applySettings);
document.getElementById('btn-zoom-in').addEventListener('click', () => { setZoom(zoom + 0.1); prefs.zoom = zoom; savePrefs(); });
document.getElementById('btn-zoom-out').addEventListener('click', () => { setZoom(zoom - 0.1); prefs.zoom = zoom; savePrefs(); });

const panneaux = document.getElementById('panneaux');
const separateur = document.getElementById('separateur');
const btnDisposition = document.getElementById('btn-disposition');
const btnVoletCode = document.getElementById('btn-volet-code');
const btnVoletApercu = document.getElementById('btn-volet-apercu');

/** @type {PrefsInterface} */
const prefs = Object.assign(
    { disposition: 'horizontale', part: 40, code: true, apercu: true, zoom: 0.9 },
    prefsInitiales
);

let prefsTimer = null;
function savePrefs() {
    clearTimeout(prefsTimer);
    prefsTimer = setTimeout(() => send({ cmd: 'prefs', content: JSON.stringify(prefs) }), 400);
}

function appliqueDisposition() {
    panneaux.dataset.disposition = prefs.disposition;
    panneaux.style.setProperty('--part', prefs.part + '%');
    panneaux.classList.toggle('sans-code', !prefs.code);
    panneaux.classList.toggle('sans-apercu', !prefs.apercu);
    btnVoletCode.classList.toggle('actif', prefs.code);
    btnVoletApercu.classList.toggle('actif', prefs.apercu);
    btnDisposition.textContent = prefs.disposition === 'horizontale' ? '⇄' : '⇅';
}

btnDisposition.addEventListener('click', () => {
    prefs.disposition = prefs.disposition === 'horizontale' ? 'verticale' : 'horizontale';
    appliqueDisposition();
    savePrefs();
});

btnVoletCode.addEventListener('click', () => {
    prefs.code = !prefs.code;
    if (!prefs.code && !prefs.apercu) prefs.apercu = true;
    appliqueDisposition();
    savePrefs();
});

btnVoletApercu.addEventListener('click', () => {
    prefs.apercu = !prefs.apercu;
    if (!prefs.code && !prefs.apercu) prefs.code = true;
    appliqueDisposition();
    savePrefs();
});

let glisse = false;
separateur.addEventListener('pointerdown', (e) => {
    glisse = true;
    separateur.classList.add('actif');
    separateur.setPointerCapture(e.pointerId);
});
separateur.addEventListener('pointermove', (e) => {
    if (!glisse) return;
    const zone = panneaux.getBoundingClientRect();
    let fraction;
    if (prefs.disposition === 'horizontale') {
        fraction = (e.clientX - zone.left) / zone.width;
    } else {
        // Le volet du code occupe le haut : sa part se mesure depuis le haut.
        fraction = (e.clientY - zone.top) / zone.height;
    }
    prefs.part = Math.round(Math.min(85, Math.max(15, fraction * 100)));
    panneaux.style.setProperty('--part', prefs.part + '%');
});
separateur.addEventListener('pointerup', (e) => {
    glisse = false;
    separateur.classList.remove('actif');
    separateur.releasePointerCapture(e.pointerId);
    savePrefs();
});
separateur.addEventListener('dblclick', () => {
    prefs.part = 40;
    appliqueDisposition();
    savePrefs();
});

appliqueDisposition();

afficheEcheance();

setZoom(zoom);

if (!ipcAvailable()) {
    diag("Le pont IPC n'est pas encore disponible au chargement. Si ce message persiste après un clic sur un bouton, la vue web n'a pas reçu le script d'initialisation de wry.");
}

editor.value = '';
histReinitialise();
requestTranspile('full');