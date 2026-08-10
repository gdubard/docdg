mod langage;
mod layout;
mod maths;
mod python;
mod utils;

pub use utils::cesure::cesure_html;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub use layout::rendu::{set_base_dir, PageOpts};
pub use python::pont::{bassin_ouvert, prechauffe};
pub use utils::notation;

pub const MARQUE_TOC: char = '\u{E010}';
pub const MARQUE_NOTE_APPEL: char = '\u{E014}';
pub const MARQUE_NOTE_CORPS: char = '\u{E015}';

const SEUIL_PARALLELE: usize = 2;
const CACHE_MAX: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Def {
    Style(Vec<String>),
    Component { param: String, template: String },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Env {
    pub defs: BTreeMap<String, Def>,
    pub counters: [u32; 3],
    pub chapitre: u32,
    pub objects: langage::commandes::Objects,
    pub vars: BTreeMap<String, f64>,
    pub donnees: BTreeMap<String, String>,
    pub etudiees: std::collections::BTreeSet<String>,
    pub textes: BTreeMap<String, String>,
    pub saisies: BTreeMap<String, String>,
    pub conteneurs: BTreeMap<String, langage::conteneurs::Boite>,
    pub fonctions: langage::fonctions::Fonctions,
    pub bloque: bool,
}

impl Hash for Env {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.defs.hash(h);
        self.counters.hash(h);
        self.chapitre.hash(h);
        self.objects.hash(h);
        for (nom, valeur) in &self.vars {
            nom.hash(h);
            valeur.to_bits().hash(h);
        }
        self.donnees.hash(h);
        self.etudiees.hash(h);
        self.textes.hash(h);
        self.saisies.hash(h);
        for (nom, boite) in &self.conteneurs {
            nom.hash(h);
            langage::conteneurs::affiche(boite).hash(h);
        }
        for (nom, f) in &self.fonctions {
            nom.hash(h);
            f.corps.hash(h);
        }
        self.bloque.hash(h);
    }
}

#[derive(Clone, Debug)]
pub struct TocEntry {
    pub level: u8,
    pub num: String,
    pub title: String,
    pub id: String,
}

pub struct RenderResult {
    pub page: PageOpts,
    pub html: String,
}

struct CacheEntry {
    html: String,
    toc: Vec<TocEntry>,
    generation: u64,
}

#[derive(Default)]
pub struct Engine {
    cache: HashMap<u64, CacheEntry>,
    generation: u64,
    pub saisies: BTreeMap<String, String>,
}

fn empreinte_env(env: &Env) -> u64 {
    let mut h = seahash::SeaHasher::new();
    env.hash(&mut h);
    h.finish()
}

fn empreinte(base: u64, seg: &str) -> u64 {
    let mut h = seahash::SeaHasher::new();
    base.hash(&mut h);
    seg.hash(&mut h);
    h.finish()
}

pub fn preprocess(src: &str) -> String {
    let octets = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut lit = 0u32;
    let mut debut = 0;
    let mut i = 0;
    while i + 1 < octets.len() {
        let c = octets[i];
        if octets[i + 1] == c {
            let marque = match c {
                b'<' => Some('\u{E000}'),
                b'>' => Some('\u{E001}'),
                b'#' => Some('\u{E004}'),
                b'$' => Some('\u{E005}'),
                b'{' => {
                    lit += 1;
                    Some('\u{E002}')
                }
                b'}' if lit > 0 => {
                    lit -= 1;
                    Some('\u{E003}')
                }
                _ => None,
            };
            if let Some(marque) = marque {
                out.push_str(&src[debut..i]);
                out.push(marque);
                i += 2;
                debut = i;
                continue;
            }
        }
        i += 1;
    }
    out.push_str(&src[debut..]);
    out
}

fn ecris_entier(dest: &mut String, mut v: u32) {
    if v == 0 {
        dest.push('0');
        return;
    }
    let mut chiffres = [0u8; 10];
    let mut n = 0;
    while v > 0 {
        chiffres[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    while n > 0 {
        n -= 1;
        dest.push(chiffres[n] as char);
    }
}

enum Marque<'a> {
    Texte(&'a str),
    Appel,
    Corps,
}

fn collecte_renvois(html: &str) -> (std::collections::BTreeMap<String, (String, String)>, String) {
    let mut table = std::collections::BTreeMap::new();
    let octets = html.as_bytes();
    let mut out = String::with_capacity(html.len());
    let mut debut = 0;
    let mut i = 0;
    while i + 2 < octets.len() {
        if octets[i] == 0xEE && octets[i + 1] == 0x80 && octets[i + 2] == 0x96 {
            let corps_debut = i + 3;
            let mut j = corps_debut;
            while j + 2 < octets.len()
                && !(octets[j] == 0xEE && octets[j + 1] == 0x80 && octets[j + 2] == 0x97)
            {
                j += 1;
            }
            if j + 2 < octets.len() {
                let corps = &html[corps_debut..j];
                let mut morceaux = corps.splitn(3, '|');
                if let (Some(nom), Some(num), Some(id)) =
                    (morceaux.next(), morceaux.next(), morceaux.next())
                {
                    table
                        .entry(nom.to_string())
                        .or_insert((num.to_string(), id.to_string()));
                }
                out.push_str(&html[debut..i]);
                i = j + 3;
                debut = i;
                continue;
            }
        }
        i += 1;
    }
    out.push_str(&html[debut..]);
    (table, out)
}

fn resous_renvois(html: &str, table: &std::collections::BTreeMap<String, (String, String)>) -> String {
    let octets = html.as_bytes();
    let mut out = String::with_capacity(html.len());
    let mut debut = 0;
    let mut i = 0;
    while i + 2 < octets.len() {
        if octets[i] == 0xEE && octets[i + 1] == 0x80 && octets[i + 2] == 0x98 {
            let corps_debut = i + 3;
            let mut j = corps_debut;
            while j + 2 < octets.len()
                && !(octets[j] == 0xEE && octets[j + 1] == 0x80 && octets[j + 2] == 0x99)
            {
                j += 1;
            }
            if j + 2 < octets.len() {
                let nom = &html[corps_debut..j];
                out.push_str(&html[debut..i]);
                match table.get(nom) {
                    Some((num, id)) => {
                        out.push_str(&format!(
                            "<a class=\"renvoi\" href=\"#{}\">{}</a>",
                            id, num
                        ));
                    }
                    None => {
                        out.push_str("<span class=\"renvoi-absent\">??</span>");
                    }
                }
                i = j + 3;
                debut = i;
                continue;
            }
        }
        i += 1;
    }
    out.push_str(&html[debut..]);
    out
}

fn finalise(html: &str, toc: &str) -> String {
    let (table, html) = collecte_renvois(html);
    let html: String = resous_renvois(&html, &table);
    let html: &str = &html;
    let octets = html.as_bytes();
    let mut out = String::with_capacity(html.len() + toc.len());
    let mut appels = 0u32;
    let mut corps = 0u32;
    let mut debut = 0;
    let mut i = 0;
    while i + 2 < octets.len() {
        if octets[i] == 0xEE && octets[i + 1] == 0x80 {
            let marque = match octets[i + 2] {
                0x80 => Some(Marque::Texte("&lt;")),
                0x81 => Some(Marque::Texte("&gt;")),
                0x82 => Some(Marque::Texte("{")),
                0x83 => Some(Marque::Texte("}")),
                0x84 => Some(Marque::Texte("#")),
                0x85 => Some(Marque::Texte("$")),
                0x90 => Some(Marque::Texte(toc)),
                0x94 => Some(Marque::Appel),
                0x95 => Some(Marque::Corps),
                _ => None,
            };
            if let Some(marque) = marque {
                out.push_str(&html[debut..i]);
                match marque {
                    Marque::Texte(t) => out.push_str(t),
                    Marque::Appel => {
                        appels += 1;
                        ecris_entier(&mut out, appels);
                    }
                    Marque::Corps => {
                        corps += 1;
                        ecris_entier(&mut out, corps);
                    }
                }
                i += 3;
                debut = i;
                continue;
            }
        }
        i += 1;
    }
    out.push_str(&html[debut..]);
    out
}

pub fn unescape(s: &str) -> String {
    finalise(s, "")
}

pub fn segments(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth: i32 = 0;
    let mut vides = 0usize;
    for line in body.lines() {
        let d0 = depth;
        utils::texte::maj_profondeur(line, &mut depth);
        if line.trim().is_empty() && d0 <= 0 && depth <= 0 {
            if !cur.trim().is_empty() {
                out.push(std::mem::take(&mut cur));
                vides = 1;
            } else {
                cur.clear();
                vides += 1;
            }
        } else {
            if vides > 0 {
                out.push(format!("\u{E011}{}", vides));
            }
            vides = 0;
            cur.push_str(line);
            cur.push('\n');
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur);
    }
    out
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }

    pub fn render(&mut self, src: &str, parallel: bool) -> RenderResult {
        let pre = preprocess(src);
        let (page, body) = layout::rendu::parse_page(&pre);
        layout::rendu::set_reglages_page(layout::rendu::ReglagesPage {
            tabulation_cm: page.tabulation / 10.0,
            hauteur_cm: page.hauteur / 10.0,
            precision: page.precision,
        });
        let segs = segments(&body);
        self.generation = self.generation.wrapping_add(1);
        let generation = self.generation;

        let mut env = Env::default();
        env.saisies = self.saisies.clone();
        for (k, v) in [
            ("titre", &page.titre),
            ("auteur", &page.auteur),
            ("institution", &page.institution),
            ("date", &page.date),
        ] {
            if !v.is_empty() {
                env.textes.insert(format!("document:{}", k), v.clone());
            }
        }
        layout::rendu::collecte_donnees(&body, &mut env);

        let mut cles: Vec<u64> = Vec::with_capacity(segs.len());
        let mut manquants: Vec<(usize, Env)> = Vec::new();
        let mut prevus: HashSet<u64> = HashSet::new();
        let mut base = empreinte_env(&env);
        for (i, s) in segs.iter().enumerate() {
            let cle = empreinte(base, s);
            if !self.cache.contains_key(&cle) && prevus.insert(cle) {
                manquants.push((i, env.clone()));
            }
            cles.push(cle);
            if !layout::rendu::inerte(s) {
                layout::rendu::scan_env(s, &mut env);
                base = empreinte_env(&env);
            }
        }

        let reglages = layout::rendu::reglages_page();
        let rendu = |(i, e): &(usize, Env)| -> (usize, String, Vec<TocEntry>) {
            layout::rendu::set_reglages_page(reglages);
            let mut local = e.clone();
            let (html, toc) = layout::rendu::render_segment(&segs[*i], &mut local);
            (*i, html, toc)
        };
        let calcules: Vec<(usize, String, Vec<TocEntry>)> =
            if parallel && manquants.len() >= SEUIL_PARALLELE {
                use rayon::prelude::*;
                manquants.par_iter().map(rendu).collect()
            } else {
                manquants.iter().map(rendu).collect()
            };

        for (i, html, toc) in calcules {
            self.cache.insert(
                cles[i],
                CacheEntry {
                    html,
                    toc,
                    generation,
                },
            );
        }
        for cle in &cles {
            if let Some(entree) = self.cache.get_mut(cle) {
                entree.generation = generation;
            }
        }
        if self.cache.len() > CACHE_MAX {
            self.cache.retain(|_, e| e.generation == generation);
        }

        let mut toc_all: Vec<&TocEntry> = Vec::new();
        for cle in &cles {
            if let Some(entree) = self.cache.get(cle) {
                toc_all.extend(entree.toc.iter());
            }
        }
        let toc_html = layout::rendu::toc_html(&toc_all);

        let mut html = String::new();
        let mut vides = 0usize;
        for cle in &cles {
            let h = match self.cache.get(cle) {
                Some(e) => e.html.as_str(),
                None => continue,
            };
            if let Some(n) = h.strip_prefix('\u{E012}') {
                vides = vides.max(n.parse::<usize>().unwrap_or(0));
                continue;
            }
            if h.is_empty() {
                continue;
            }
            if !html.is_empty() {
                let hauteur = layout::rendu::reglages_page().hauteur_cm;
                for _ in 0..vides {
                    html.push_str(&format!(
                        "<div class=\"ligne-vide\" style=\"height:{}cm\"></div>",
                        hauteur
                    ));
                }
            }
            vides = 0;
            html.push_str(h);
        }

        RenderResult {
            page,
            html: finalise(&html, &toc_html),
        }
    }
}

#[cfg(test)]
mod invariant {
    use super::*;

    const CORPUS: &[(&str, &str)] = &[
        ("basique1", include_str!("../tests/basique1.txt")),
        ("basique2", include_str!("../tests/basique2.txt")),
        ("basique3", include_str!("../tests/basique3.txt")),
        ("algebre2", include_str!("../tests/algebre2.txt")),
        ("algebre3", include_str!("../tests/algebre3.txt")),
        ("algebre4", include_str!("../tests/algebre4.txt")),
        ("analyse2", include_str!("../tests/analyse2.txt")),
        ("analyse3", include_str!("../tests/analyse3.txt")),
        ("analyse4", include_str!("../tests/analyse4.txt")),
        ("geometrie2", include_str!("../tests/geometrie2.txt")),
        ("geometrie3", include_str!("../tests/geometrie3.txt")),
        ("geometrie4", include_str!("../tests/geometrie4.txt")),
        ("factorisation", include_str!("../tests/factorisation.txt")),
        ("stat2", include_str!("../tests/statistiques-probabilites2.txt")),
        ("stat3", include_str!("../tests/statistiques-probabilites3.txt")),
        ("stat4", include_str!("../tests/statistiques-probabilites4.txt")),
    ];

fn sur_corpus(mut visite: impl FnMut(&str, usize, &str, &mut Env)) {
        set_base_dir(Some(std::path::PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests"
        ))));
        for (nom, src) in CORPUS {
            let pre = preprocess(src);
            let (_, body) = layout::rendu::parse_page(&pre);
            let mut env = Env::default();
            layout::rendu::collecte_donnees(&body, &mut env);
            for (i, seg) in segments(&body).iter().enumerate() {
                visite(nom, i, seg, &mut env);
            }
        }
    }

    #[test]
    fn un_segment_inerte_ne_modifie_jamais_l_environnement() {
        let mut fautes = Vec::new();
        let mut inertes = 0usize;
        sur_corpus(|nom, i, seg, env| {
            let avant = env.clone();
            layout::rendu::scan_env(seg, env);
            if layout::rendu::inerte(seg) {
                inertes += 1;
                if avant != *env {
                    fautes.push(format!("{} segment {}", nom, i));
                }
            }
        });
        assert!(inertes > 0, "aucun segment inerte : le test ne prouve rien");
        assert!(
            fautes.is_empty(),
            "des segments juges inertes modifient l'environnement : {:?}",
            fautes
        );
    }

    #[test]
    fn scan_env_predit_exactement_le_rendu() {
        let mut fautes = Vec::new();
        sur_corpus(|nom, i, seg, env| {
            let mut predit = env.clone();
            layout::rendu::scan_env(seg, &mut predit);
            let mut rendu = env.clone();
            let _ = layout::rendu::render_segment(seg, &mut rendu);
            if predit != rendu {
                fautes.push(format!("{} segment {}", nom, i));
            }
            *env = predit;
        });
        assert!(
            fautes.is_empty(),
            "scan_env diverge de render_segment : {:?}",
            fautes
        );
    }
}
