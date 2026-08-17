use super::controle::*;

use crate::utils::couleurs::parse_color_at;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

fn base_slot() -> &'static Mutex<Option<PathBuf>> {
    static SLOT: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

pub fn set_base_dir(dir: Option<PathBuf>) {
    if let Ok(mut g) = base_slot().lock() {
        *g = dir;
    }
}

pub(crate) fn base_dir() -> Option<PathBuf> {
    base_slot().lock().ok().and_then(|g| g.clone())
}

#[derive(Clone, Copy, Debug)]
pub struct ReglagesPage {
    pub tabulation_cm: f32,
    pub hauteur_cm: f32,
    pub precision: i32,
}

impl ReglagesPage {
    const CONST_DEFAUT: ReglagesPage = ReglagesPage {
        tabulation_cm: 1.0,
        hauteur_cm: 0.5,
        precision: -1,
    };
}

impl Default for ReglagesPage {
    fn default() -> Self {
        ReglagesPage {
            tabulation_cm: 1.0,
            hauteur_cm: 0.5,
            precision: -1,
        }
    }
}

thread_local! {
    static REGLAGES: std::cell::Cell<ReglagesPage> =
        const { std::cell::Cell::new(ReglagesPage::CONST_DEFAUT) };
}

pub fn set_reglages_page(r: ReglagesPage) {
    REGLAGES.with(|c| c.set(r));
}

pub fn reglages_page() -> ReglagesPage {
    REGLAGES.with(|c| c.get())
}

thread_local! {

    static POLICE_SEYES: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

pub fn set_police_seyes(nom: &str) {
    POLICE_SEYES.with(|c| *c.borrow_mut() = nom.to_string());
}

pub fn police_seyes() -> String {
    POLICE_SEYES.with(|c| c.borrow().clone())
}
use crate::utils::notation::to_latex;
use crate::{Def, Env, TocEntry};

#[derive(Clone, Debug)]
pub struct PageOpts {
    pub orientation: String,
    pub marges: [f32; 4],
    pub espacements: [f32; 4],

    pub script: String,

    pub math: String,

    pub seyes: String,
    pub titre: String,
    pub auteur: String,
    pub institution: String,
    pub date: String,
    pub taille: f32,
    pub interligne: f32,
    pub tabulation: f32,
    pub hauteur: f32,
    pub decalage: f32,
    pub precision: i32,
    pub cesure: bool,
    pub veuves: u32,
    pub orphelines: u32,

    pub numerotation: String,
}

impl Default for PageOpts {
    fn default() -> Self {
        PageOpts {
            orientation: "portrait".into(),
            marges: [2.0, 2.0, 2.0, 2.0],
            espacements: [0.0, 0.0, 0.0, 0.0],
            script: String::new(),
            math: String::new(),
            seyes: String::new(),
            titre: String::new(),
            auteur: String::new(),
            institution: String::new(),
            date: String::new(),
            taille: 11.0,
            interligne: 1.3,

            tabulation: 8.0,
            hauteur: 8.0,
            decalage: 100.0,
            precision: -1,
            cesure: true,
            veuves: 2,
            orphelines: 2,
            numerotation: "composée".to_string(),
        }
    }
}

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

impl PageOpts {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"orientation\":\"{}\",\"marges\":[{},{},{},{}],\"espacements\":[{},{},{},{}],\"script\":{},\"math\":{},\"seyes\":{},\"taille\":{},\"interligne\":{},\"tabulation\":{},\"hauteur\":{},\"decalage\":{},\"precision\":{},\"cesure\":{},\"veuves\":{},\"orphelines\":{},\"numerotation\":\"{}\",\"titre\":{},\"auteur\":{},\"institution\":{},\"date\":{}}}",
            self.orientation,
            self.marges[0], self.marges[1], self.marges[2], self.marges[3],
            self.espacements[0], self.espacements[1], self.espacements[2], self.espacements[3],
            json_string(&self.script), json_string(&self.math), json_string(&self.seyes),
            self.taille, self.interligne,
            self.tabulation, self.hauteur, self.decalage, self.precision,
            self.cesure, self.veuves, self.orphelines,
            match self.numerotation.as_str() { "simple" => "simple", "sans" => "sans", _ => "composee" },
            json_string(&self.titre), json_string(&self.auteur),
            json_string(&self.institution), json_string(&self.date)
        )
    }
}

fn parse_quad(v: &str) -> Option<[f32; 4]> {
    let inner = v.trim().trim_start_matches('{').trim_end_matches('}');
    let parts: Vec<f32> = inner
        .split(';')
        .filter_map(|p| p.trim().replace(',', ".").parse::<f32>().ok())
        .collect();
    if parts.len() == 4 {
        Some([parts[0], parts[1], parts[2], parts[3]])
    } else if parts.len() == 1 {
        Some([parts[0]; 4])
    } else {
        None
    }
}

fn applique_cle(opts: &mut PageOpts, k: &str, v: &str) {
    match k {
        "orientation" => opts.orientation = v.to_string(),
        "marges" | "margin" => {
            if let Some(q) = parse_quad(v) {
                opts.marges = q;
            }
        }
        "espacements" | "padding" => {
            if let Some(q) = parse_quad(v) {
                opts.espacements = q;
            }
        }
        "script" => opts.script = v.to_string(),
        "math" => opts.math = v.to_string(),
        "seyès" | "seyes" => opts.seyes = v.to_string(),
        "titre" => opts.titre = v.to_string(),
        "auteur" => opts.auteur = v.to_string(),
        "institution" => opts.institution = v.to_string(),
        "date" => opts.date = v.to_string(),
        "taille" => {
            if let Ok(p) = v.trim_end_matches("pt").trim().parse::<f32>() {
                opts.taille = p;
            }
        }
        "interligne" => {
            if let Ok(p) = v.replace(',', ".").parse::<f32>() {
                opts.interligne = p;
            }
        }
        "tabulation" => {
            if let Ok(p) = v.trim_end_matches("mm").trim().replace(',', ".").parse::<f32>() {
                opts.tabulation = p;
            }
        }
        "hauteur" => {
            if let Ok(p) = v.trim_end_matches("mm").trim().replace(',', ".").parse::<f32>() {
                opts.hauteur = p;
            }
        }
        "décalage" | "decalage" => {
            if let Ok(p) = v.trim_end_matches('%').trim().replace(',', ".").parse::<f32>() {
                opts.decalage = p;
            }
        }
        "précision" | "precision" => {
            if let Ok(p) = v.trim().parse::<i32>() {
                opts.precision = p;
            }
        }
        "césure" | "cesure" => opts.cesure = !matches!(v.trim(), "non" | "faux" | "0"),
        "veuves" => {
            if let Ok(p) = v.trim().parse::<u32>() {
                opts.veuves = p.max(1);
            }
        }
        "orphelines" => {
            if let Ok(p) = v.trim().parse::<u32>() {
                opts.orphelines = p.max(1);
            }
        }
        "numérotation" | "numerotation" => {
            let val = v.trim().to_lowercase();
            opts.numerotation = match val.as_str() {
                "simple" => "simple".to_string(),
                "sans" | "non" | "aucune" => "sans".to_string(),
                _ => "composée".to_string(),
            };
        }
        _ => {}
    }
}

fn cle_connue(k: &str) -> bool {
    matches!(
        k,
        "orientation"
            | "marges"
            | "margin"
            | "espacements"
            | "padding"
            | "script"
            | "math"
            | "seyès"
            | "seyes"
            | "titre"
            | "auteur"
            | "institution"
            | "date"
            | "taille"
            | "interligne"
            | "tabulation"
            | "hauteur"
            | "décalage"
            | "decalage"
            | "précision"
            | "precision"
            | "césure"
            | "cesure"
            | "veuves"
            | "orphelines"
            | "numérotation"
            | "numerotation"
    )
}

pub fn parse_page(src: &str) -> (PageOpts, String) {
    let mut opts = PageOpts::default();
    let mut reste = src;
    let mut inconnues: Vec<String> = Vec::new();
    loop {
        let trimmed = reste.trim_start();
        let apres_mot = trimmed
            .strip_prefix("document")
            .or_else(|| trimmed.strip_prefix("page"));
        let Some(suite) = apres_mot else { break };
        let suite = suite.trim_start();
        if !suite.starts_with('{') {
            break;
        }
        let Some((body, after)) = take_group(suite, 0) else {
            break;
        };
        for item in split_top(&body, ';') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            match item.split_once(':') {
                Some((k, v)) => {
                    let k = k.trim();
                    if cle_connue(k) {
                        applique_cle(&mut opts, k, v.trim());
                    } else {
                        inconnues.push(k.to_string());
                    }
                }

                None => inconnues.push(item.to_string()),
            }
        }
        let fin = src.len() - after.len();
        reste = &src[fin..];
    }
    let corps = if inconnues.is_empty() {
        reste.to_string()
    } else {
        format!(
            "{}{}",
            crate::utils::erreur::source(
                &inconnues.join(" ; "),
                if inconnues.len() > 1 {
                    "réglages de page non reconnus"
                } else {
                    "réglage de page non reconnu"
                }
            ),
            reste
        )
    };
    (opts, corps)
}

pub(crate) fn take_group(s: &str, open_idx: usize) -> Option<(String, String)> {
    crate::utils::decoupe::prend_accolades(s, open_idx)
        .map(|(corps, suite)| (corps.to_string(), suite.to_string()))
}

fn split_top(s: &str, sep: char) -> Vec<String> {
    crate::utils::decoupe::coupe_fragments(s, sep)
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn read_tag(s: &str) -> Option<(String, String)> {
    let mut it = s.char_indices();
    match it.next() {
        Some((_, '<')) => {}
        _ => return None,
    }
    let mut depth = 0i32;
    for (i, c) in it {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            '>' if depth <= 0 && !s[i + 1..].starts_with('=') => {
                return Some((s[1..i].to_string(), s[i + 1..].to_string()));
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn subst_var(s: &str, var: &str, val: &str) -> String {
    let s = subst_in_calc_groups(s, var, val);
    let s: &str = &s;
    let pat = format!("#{}", var);
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find(&pat) {
        out.push_str(&rest[..i]);
        let after = &rest[i + pat.len()..];
        let boundary = after
            .chars()
            .next()
            .map(|c| !c.is_alphanumeric())
            .unwrap_or(true);
        if boundary {
            out.push_str(val);
            rest = after;
        } else {
            out.push_str(&pat);
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

fn noms_de_fonctions_math(env: &Env, seg: &str) -> std::collections::BTreeSet<String> {
    let mut noms: std::collections::BTreeSet<String> = env
        .objects
        .iter()
        .filter(|(_, o)| matches!(o, crate::langage::commandes::Obj::Function { .. }))
        .map(|(n, _)| n.clone())
        .collect();
    let mut depuis = 0usize;
    while let Some(p) = seg[depuis..].find("une fonction ") {
        let debut = depuis + p + "une fonction ".len();
        let suite = &seg[debut..];
        let fin = suite
            .find(|c: char| !(c.is_alphanumeric() || c == '_'))
            .unwrap_or(suite.len());
        if fin > 0 && suite[fin..].starts_with('(') {
            noms.insert(suite[..fin].to_string());
        }
        depuis = debut;
    }
    noms
}

pub(crate) const TOURS_MAX: usize = 2000;

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn est_mot_majuscule(w: &str) -> bool {
    let mut a_alpha = false;
    for c in w.chars() {
        if c.is_alphabetic() {
            a_alpha = true;
            if !c.is_uppercase() {
                return false;
            }
        }
    }
    a_alpha
}

fn nom_police_at(refs: &[&str]) -> Option<(String, usize)> {
    if refs.is_empty() || !est_mot_majuscule(refs[0]) {
        return None;
    }
    let mut n = 1;
    while n < refs.len() && est_mot_majuscule(refs[n]) {
        n += 1;
    }
    Some((refs[..n].join(" "), n))
}

pub(crate) fn style_css(words: &[String]) -> (String, Option<u8>) {
    let mut css = String::new();
    let mut level = None;
    let mut i = 0;
    let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
    while i < refs.len() {
        if let Some((c, n)) = parse_color_at(&refs[i..]) {
            css.push_str(&format!("color:{};", c));
            i += n;
            continue;
        }
        if refs.get(i..i + 2) == Some(&["petites", "capitales"]) {
            css.push_str("font-variant:small-caps;");
            i += 2;
            continue;
        }
        if refs.get(i..i + 2) == Some(&["sans", "empattements"]) {
            css.push_str("font-family:'Helvetica Neue',Arial,sans-serif;");
            i += 2;
            continue;
        }
        if refs.get(i..i + 2) == Some(&["à", "gauche"]) {
            css.push_str("display:block;text-align:left;");
            i += 2;
            continue;
        }
        if refs.get(i..i + 2) == Some(&["au", "centre"]) {
            css.push_str("display:block;text-align:center;");
            i += 2;
            continue;
        }
        if refs.get(i..i + 2) == Some(&["à", "droite"]) {
            css.push_str("display:block;text-align:right;");
            i += 2;
            continue;
        }
        if let Some((nom, n)) = nom_police_at(&refs[i..]) {
            css.push_str(&format!("font-family:'{}';", nom));
            i += n;
            continue;
        }
        let w = refs[i];
        match w {
            "gras" => css.push_str("font-weight:700;"),
            "italique" => css.push_str("font-style:italic;"),
            "souligné" => css.push_str("text-decoration:underline;"),
            "barré" => css.push_str("text-decoration:line-through;"),
            "centre" => css.push_str("display:block;text-align:center;"),
            "gauche" => css.push_str("display:block;text-align:left;"),
            "droite" => css.push_str("display:block;text-align:right;"),
            "justifié" | "justifie" => css.push_str("display:block;text-align:justify;"),
            "petit" => css.push_str("font-size:0.85em;"),
            "grand" => css.push_str("font-size:1.25em;"),
            "chapitre" => level = Some(0),
            "section" => level = Some(1),
            "sous-section" => level = Some(2),
            "sous-sous-section" => level = Some(3),
            "num" => {}
            _ => {
                if let Some(pt) = w.strip_suffix("pt") {
                    if pt.chars().all(|c| c.is_ascii_digit() || c == '.') {
                        css.push_str(&format!("font-size:{}pt;", pt));
                    }
                }
            }
        }
        i += 1;
    }
    (css, level)
}

fn heading(env: &mut Env, toc: &mut Vec<TocEntry>, level: u8, css: &str, text: &str) -> String {
    if level == 0 {
        env.chapitre += 1;
        env.counters = [0; 3];
        env.environnements.clear();
        let num = env.chapitre.to_string();
        let id = format!("chap-{}", num);
        let title = render_inline(text.trim(), env, toc);
        toc.push(TocEntry {
            level,
            num: num.clone(),
            title: text.trim().to_string(),
            id: id.clone(),
        });
        return format!(
            "<h1 id=\"{id}\" class=\"sec lvl0\" style=\"{css}\"><span class=\"secnum\">{num}</span>&nbsp;&nbsp;{title}</h1>"
        );
    }
    let l = level as usize;
    env.counters[l - 1] += 1;
    for c in env.counters.iter_mut().skip(l) {
        *c = 0;
    }
    let mut num: String = env.counters[..l]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(".");
    if env.chapitre > 0 {
        num = format!("{}.{}", env.chapitre, num);
    }
    let id = format!("sec-{}", num.replace('.', "-"));
    let title = render_inline(text.trim(), env, toc);
    toc.push(TocEntry {
        level,
        num: num.clone(),
        title: text.trim().to_string(),
        id: id.clone(),
    });
    format!(
        "<h{l} id=\"{id}\" class=\"sec lvl{level}\" style=\"{css}\"><span class=\"secnum\">{num}</span>&nbsp;&nbsp;{title}</h{l}>",
        l = level + 1
    )
}

fn bump_heading(env: &mut Env, level: u8) {
    if level == 0 {
        env.chapitre += 1;
        env.counters = [0; 3];
        env.environnements.clear();
        return;
    }
    let l = level as usize;
    env.counters[l - 1] += 1;
    for c in env.counters.iter_mut().skip(l) {
        *c = 0;
    }
}

fn bump_heading_si_style(env: &mut Env, cle: &str) {
    if let Some(Def::Style(w)) = env.defs.get(cle).cloned() {
        if let (_, Some(lvl)) = style_css(&w) {
            bump_heading(env, lvl);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TypeSaisie {
    Texte,
    Entier,
    Decimal,
    Booleen,
    Caractere,
}

impl TypeSaisie {
    fn nom(self) -> &'static str {
        match self {
            TypeSaisie::Texte => "texte",
            TypeSaisie::Entier => "entier",
            TypeSaisie::Decimal => "décimal",
            TypeSaisie::Booleen => "booléen",
            TypeSaisie::Caractere => "caractère",
        }
    }
}

enum ValeurSaisie {
    Nombre(f64),
    Booleen(bool),
    Texte(String),
}

fn parse_saisie(line: &str) -> Option<(String, TypeSaisie, String)> {
    let reste = line.trim_start().strip_prefix("soit ")?;
    let (lhs, rhs) = reste.split_once('=')?;
    let nom = lhs.trim();
    if nom.is_empty() || nom.contains('{') {
        return None;
    }
    let (tag, apres) = read_tag(rhs.trim_start())?;
    if tag.trim() != "Saisis" {
        return None;
    }
    let apres = apres.trim_start();
    let bi = apres.find('{')?;
    let desc = apres[..bi].split_whitespace().collect::<Vec<_>>().join(" ");
    let ty = match desc.as_str() {

        "une chaîne de caractères" | "une chaine de caracteres" => TypeSaisie::Texte,
        "un entier" => TypeSaisie::Entier,
        "un décimal" => TypeSaisie::Decimal,
        "un booléen" => TypeSaisie::Booleen,
        "un caractère" => TypeSaisie::Caractere,
        _ => return None,
    };
    let (question, _) = take_group(&apres[bi..], 0)?;
    Some((nom.to_string(), ty, question.trim().to_string()))
}

fn decimal_saisi(brut: &str) -> Option<f64> {
    let t = brut.trim();
    let chiffres = t.strip_prefix(['+', '-']).unwrap_or(t);
    let (entiere, decimale) = match chiffres.split_once(',') {
        Some((a, b)) => (a, Some(b)),
        None => (chiffres, None),
    };
    if entiere.is_empty() || !entiere.bytes().all(|o| o.is_ascii_digit()) {
        return None;
    }
    if let Some(d) = decimale {
        if d.is_empty() || !d.bytes().all(|o| o.is_ascii_digit()) {
            return None;
        }
    }
    t.replace(',', ".").parse::<f64>().ok().filter(|v| v.is_finite())
}

fn valide_saisie(ty: TypeSaisie, brut: &str) -> Option<ValeurSaisie> {
    let t = brut.trim();
    match ty {
        TypeSaisie::Texte => (!t.is_empty()).then(|| ValeurSaisie::Texte(t.to_string())),
        TypeSaisie::Entier => t.parse::<i64>().ok().map(|v| ValeurSaisie::Nombre(v as f64)),
        TypeSaisie::Decimal => decimal_saisi(t).map(ValeurSaisie::Nombre),
        TypeSaisie::Booleen => match t.to_lowercase().as_str() {
            "vrai" => Some(ValeurSaisie::Booleen(true)),
            "faux" => Some(ValeurSaisie::Booleen(false)),
            _ => None,
        },
        TypeSaisie::Caractere => {
            let mut it = t.chars();
            match (it.next(), it.next()) {
                (Some(c), None) => Some(ValeurSaisie::Texte(c.to_string())),
                _ => None,
            }
        }
    }
}

fn applique_saisie(nom: &str, valeur: &ValeurSaisie, env: &mut Env) {
    match valeur {
        ValeurSaisie::Nombre(v) => {
            env.vars.insert(nom.to_string(), *v);
        }
        ValeurSaisie::Booleen(b) => {
            env.vars.insert(nom.to_string(), if *b { 1.0 } else { 0.0 });
            env.textes
                .insert(nom.to_string(), if *b { "vrai" } else { "faux" }.to_string());
        }
        ValeurSaisie::Texte(t) => {
            env.textes.insert(nom.to_string(), t.clone());
        }
    }
}

fn saisie_repondue(nom: &str, ty: TypeSaisie, env: &Env) -> Option<ValeurSaisie> {
    valide_saisie(ty, env.saisies.get(nom)?)
}

fn affichage_saisie(valeur: &ValeurSaisie) -> String {
    match valeur {
        ValeurSaisie::Nombre(v) => crate::maths::calcul::format_number(*v),
        ValeurSaisie::Booleen(b) => if *b { "vrai" } else { "faux" }.to_string(),
        ValeurSaisie::Texte(t) => t.clone(),
    }
}

fn echappe_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}

fn coupe_au_blocage(seg: &str, env: &Env) -> (String, bool) {
    let mut out = String::new();
    let mut profondeur = 0i32;
    for line in seg.lines() {
        out.push_str(line);
        out.push('\n');
        if profondeur <= 0 {
            if let Some((nom, ty, _)) = parse_saisie(line) {
                if saisie_repondue(&nom, ty, env).is_none() {
                    return (out, true);
                }
            }
        }
        crate::utils::texte::maj_profondeur(line, &mut profondeur);
    }
    (out, false)
}

fn applique_saisies_du_segment(seg: &str, env: &mut Env) {
    let mut profondeur = 0i32;
    for line in seg.lines() {
        if profondeur <= 0 {
            if let Some((nom, ty, _)) = parse_saisie(line) {
                if let Some(valeur) = saisie_repondue(&nom, ty, env) {
                    applique_saisie(&nom, &valeur, env);
                }
            }
        }
        crate::utils::texte::maj_profondeur(line, &mut profondeur);
    }
}

fn html_saisie(nom: &str, ty: TypeSaisie, question: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let question = render_inline(question, env, toc);
    match saisie_repondue(nom, ty, env) {
        Some(valeur) => {
            applique_saisie(nom, &valeur, env);
            format!(
                "<p class=\"saisie-faite\"><span class=\"saisie-question\">{}</span> <span class=\"saisie-valeur\" data-nom=\"{}\" title=\"Cliquer pour répondre à nouveau\">{}</span></p>",
                question,
                echappe_html(nom),
                echappe_html(&affichage_saisie(&valeur))
            )
        }
        None => {
            env.bloque = true;
            format!(
                "<div class=\"saisie\" data-nom=\"{}\" data-type=\"{}\"><span class=\"saisie-question\">{}</span> <input class=\"saisie-champ\" type=\"text\" autocomplete=\"off\" spellcheck=\"false\"><span class=\"saisie-erreur\"></span></div>",
                echappe_html(nom),
                ty.nom(),
                question
            )
        }
    }
}

fn vars_avec_affectations_locales(
    seg: &str,
    vars: &std::collections::BTreeMap<String, f64>,
) -> std::collections::BTreeMap<String, f64> {
    let mut locales = vars.clone();
    let mut profondeur = 0i32;
    for line in seg.lines() {
        let t = line.trim_start();
        let niveau = profondeur;
        profondeur += line.matches('{').count() as i32 - line.matches('}').count() as i32;
        if niveau > 0 || !t.starts_with("soit ") || est_ternaire(t) {
            continue;
        }
        let reste = t.trim_start_matches("soit").trim_start();
        let Some((lhs, rhs)) = reste.split_once('=') else {
            continue;
        };
        let lhs = lhs.trim();
        let rhs = rhs.trim();
        if lhs.contains('{') || rhs.starts_with('{') || rhs.starts_with('<') || rhs.starts_with("si ") {
            continue;
        }
        if let Some(v) = crate::maths::calcul::eval(rhs, &locales) {
            locales.insert(lhs.to_string(), v);
        }
    }
    locales
}

fn parse_def(line: &str, env: &mut Env) {
    if parse_saisie(line).is_some() {
        return;
    }
    let rest = line.trim_start().trim_start_matches("soit").trim_start();
    if let Some((lhs, rhs)) = rest.split_once('=') {
        let lhs = lhs.trim();
        let rhs = rhs.trim();
        if rhs.starts_with('{') {
            if let Some(fin) = rhs.rfind('}') {
                env.donnees
                    .insert(lhs.to_string(), rhs[1..fin].trim_start_matches('\n').to_string());
            }
            return;
        }
        if rhs.starts_with("si ") {
            if let Some((choisi, autre)) = parse_ternaire_branches(rhs, &env.vars) {
                let nombre = crate::maths::calcul::eval(&choisi, &env.vars).filter(|_| {
                    autre
                        .as_deref()
                        .map(|a| crate::maths::calcul::eval(a, &env.vars).is_some())
                        .unwrap_or(true)
                });
                if let Some(v) = nombre {
                    env.vars.insert(lhs.to_string(), v);
                    env.textes.remove(lhs);
                } else {
                    env.textes.insert(lhs.to_string(), choisi);
                    env.vars.remove(lhs);
                }
            }
            return;
        }
        if !rhs.starts_with('<') {
            if !lhs.contains('{') {
                if let Some(v) = crate::maths::calcul::eval(rhs, &env.vars) {
                    env.vars.insert(lhs.to_string(), v);
                }
            }
            return;
        }
        let inner = match read_tag(rhs) {
            Some((t, _)) => t,
            None => rhs.trim_start_matches('<').trim_end_matches('>').to_string(),
        };
        if let Some(bi) = lhs.find('{') {
            let name = lhs[..bi].trim().to_string();
            let param = lhs[bi + 1..].trim_end_matches('}').trim().to_string();
            env.defs.insert(name, Def::Component { param, template: inner });
        } else {
            let words: Vec<String> = inner.split_whitespace().map(|s| s.to_string()).collect();
            env.defs.insert(lhs.to_string(), Def::Style(words));
        }
    }
}

pub fn collecte_donnees(seg: &str, env: &mut Env) {
    crate::maths::statistiques::collecte_lois(seg, env);
    let mut nom_bloc: Option<String> = None;
    let mut contenu = String::new();
    for line in seg.lines() {
        let t = line.trim_start();
        if nom_bloc.is_some() {
            if t == "}" {
                if let Some(nom) = nom_bloc.take() {
                    env.donnees.insert(nom, std::mem::take(&mut contenu));
                }
            } else {
                contenu.push_str(line);
                contenu.push('\n');
            }
            continue;
        }
        if t.starts_with("soit ") {
            let reste = t.trim_start_matches("soit").trim_start();
            if let Some((lhs, rhs)) = reste.split_once('=') {
                if rhs.trim() == "{" {
                    nom_bloc = Some(lhs.trim().to_string());
                }
            }
        }
    }
}

pub fn inerte(seg: &str) -> bool {
    !seg.contains('<') && !seg.contains("soit") && !(seg.contains('[') && seg.contains('='))
}

pub fn scan_env(seg: &str, env: &mut Env) {
    if env.bloque {
        return;
    }
    let (seg, _) = coupe_au_blocage(seg, env);
    let seg: &str = &seg;
    collecte_donnees(seg, env);
    for line in seg.lines() {
        let t = line.trim_start();
        if t.starts_with("soit ") && !est_ternaire(t) {
            parse_def(t, env);
        }
    }
    applique_saisies_du_segment(seg, env);
    let mut vars_scan = env.vars.clone();
    let mut boites_scan = (*env.conteneurs).clone();
    let mut fonctions_scan = (*env.fonctions).clone();
    let noms_math = noms_de_fonctions_math(env, seg);
    let seg = normalise_affectations(seg);
    let seg = expand_tant_que_avec(&seg, &vars_scan, &mut boites_scan, &fonctions_scan);
    let seg = expand_loops_avec(&seg, &mut vars_scan, &mut boites_scan, &mut fonctions_scan, &noms_math);
    env.conteneurs = std::sync::Arc::new(boites_scan);
    env.fonctions = std::sync::Arc::new(fonctions_scan);
    let seg = expand_conditions_avec(
        &seg,
        &env.vars,
        Some(&env.conteneurs),
        Some(&env.fonctions),
    );
    for line in seg.lines() {
        let t = line.trim_start();
        if t.starts_with("soit ") && !est_ternaire(t) {
            parse_def(t, env);
        }
    }
    applique_saisies_du_segment(&seg, env);
    let (seg, bloque) = coupe_au_blocage(&seg, env);
    env.bloque = bloque;
    let seg: &str = &seg;
    for chunk in logical_chunks(&seg) {
        if let Some((tag, after)) = read_tag(chunk.trim_start()) {
            let tag_t = tag.trim();
            if tag_t == "Soit" || tag_t.starts_with("Soit ") || tag_t.starts_with("On pose") {
                let _ = declare(tag_t, &after, env);
            } else if crate::utils::texte::meme_mot(tag_t, "Énonce") {
                crate::layout::environnements::scan(&after, env);
            } else if tag_t == "Place" {

                crate::maths::geometrie::collecte_place(&after, env);
            } else if tag_t.starts_with("Trace") {
                let (_, rest_in_tag) = verbe_et_reste(tag_t);
                let (desc, corps) = desc_et_corps(rest_in_tag, &after);
                if let Some(corps) = corps {
                    if !desc.to_lowercase().contains("dans un repère") {
                        crate::maths::geometrie::collecte_figure(&corps, env);
                    }
                }
            } else if tag_t.starts_with("Construis") {
                let (_, rest_in_tag) = verbe_et_reste(tag_t);
                let (desc, corps) = desc_et_corps(rest_in_tag, &after);
                if let Some(corps) = corps {
                    crate::maths::algebre::declare_graphe(&desc, &corps, env);
                }
            } else if tag_t.starts_with("Dresse") {
                let (_, rest_in_tag) = verbe_et_reste(tag_t);
                let (desc, _) = desc_et_corps(rest_in_tag, &after);
                let bas = desc.to_lowercase();
                if bas.contains("tableau de variations") || bas.contains("tableaux de variation") {
                    for nom in crate::maths::analyse::noms_listes(&desc) {
                        env.etudiees.insert(nom);
                    }
                }
            }
        }
    }
    for line in seg.lines() {
        let t = line.trim_start();
        if t.starts_with("soit ") {
            parse_def(t, env);
        } else if t.starts_with('<') {
            if let Some((tag, _)) = read_tag(t) {
                let tag_t = tag.trim();
                let cle = saut_de_page(tag_t).unwrap_or(tag_t);
                bump_heading_si_style(env, cle);
            }
        }
    }
    for chunk in logical_chunks(seg) {
        let t = chunk.trim_start();
        if chunk.contains('\n') && est_ternaire(t) {
            parse_def(t, env);
        }
    }
}

pub(crate) fn est_ternaire(line: &str) -> bool {
    line.trim_start()
        .strip_prefix("soit ")
        .and_then(|r| r.split_once('='))
        .map(|(_, rhs)| rhs.trim_start().starts_with("si "))
        .unwrap_or(false)
}

fn ouvre_matrice(line: &str) -> Option<char> {
    let t = line.trim_start();
    if !t.starts_with('<') {
        return None;
    }
    let bas = t.to_lowercase();
    if !bas.contains("matrice") {
        return None;
    }
    match t.trim_end().chars().last() {
        Some('(') => Some(')'),
        Some('[') => Some(']'),
        _ => None,
    }
}

fn logical_chunks(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    let mut fin_matrice: Option<char> = None;

    let mut suite_de_consigne = false;
    let mut retrait_consigne = 0usize;
    for line in text.lines() {
        if suite_de_consigne {
            cur.push('\n');
            cur.push_str(line);
            let t = line.trim_end();

            let en_retrait = retrait(line) > retrait_consigne;
            if (t.ends_with(',') || t.ends_with(':') || en_retrait) && !line.contains('{') {
                continue;
            }
            suite_de_consigne = false;

            depth = 0;
            crate::utils::texte::maj_profondeur(&cur, &mut depth);
            if depth <= 0 {
                depth = 0;
                out.push(std::mem::take(&mut cur));
            }
            continue;
        }
        if let Some(f) = fin_matrice {
            cur.push('\n');
            cur.push_str(line);
            if line.trim_start().starts_with(f) {
                out.push(std::mem::take(&mut cur));
                fin_matrice = None;
            }
            continue;
        }
        if depth <= 0 {
            if let Some(f) = ouvre_matrice(line) {
                cur.push_str(line);
                fin_matrice = Some(f);
                continue;
            }
        }
        let d0 = depth;
        crate::utils::texte::maj_profondeur(line, &mut depth);
        if d0 > 0 {
            cur.push('\n');
            cur.push_str(line);
            if depth <= 0 {
                out.push(std::mem::take(&mut cur));
                depth = 0;
            }
            continue;
        }
        if line.trim().is_empty() {
            out.push(String::new());
            continue;
        }
        if depth > 0 {
            cur.push_str(line);
        } else if depth == 0 && consigne_a_suivre(line) {
            cur.push_str(line);
            retrait_consigne = retrait(line);
            suite_de_consigne = true;
        } else {
            out.push(line.to_string());
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn retrait(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

fn consigne_a_suivre(line: &str) -> bool {
    let t = line.trim();
    if !(t.ends_with(',') || t.ends_with(':')) || !t.starts_with('<') {
        return false;
    }
    match t.find('>') {
        Some(i) => read_tag(t).is_some() && !t[..i].contains('\n'),
        None => false,
    }
}

fn parse_ternaire_branches(
    rhs: &str,
    vars: &std::collections::BTreeMap<String, f64>,
) -> Option<(String, Option<String>)> {
    let apres_si = rhs.trim_start().strip_prefix("si ")?;
    let bi = apres_si.find('{')?;
    let cond = apres_si[..bi].trim();
    let (alors, suite) = take_group(&apres_si[bi..], 0)?;
    let suite_t = suite.trim_start();
    let sinon = suite_t
        .strip_prefix("sinon")
        .and_then(|r| take_group(r.trim_start(), 0))
        .map(|(b, _)| b);
    let propre = |b: &str| dedent(b).trim().to_string();
    let (choisi, autre) = if eval_condition(cond, vars) {
        (propre(&alors), sinon.as_deref().map(propre))
    } else {
        (
            propre(sinon.as_deref().unwrap_or_default()),
            Some(propre(&alors)),
        )
    };
    Some((choisi, autre))
}

pub fn render_segment(seg: &str, env: &mut Env) -> (String, Vec<TocEntry>) {
    if let Some(n) = seg.strip_prefix('\u{E011}') {
        let n: usize = n.trim().parse().unwrap_or(0);
        return (format!("\u{E012}{}", n), Vec::new());
    }
    if env.bloque {
        return (String::new(), Vec::new());
    }
    let (seg, _) = coupe_au_blocage(seg, env);
    let seg: &str = &seg;
    collecte_donnees(seg, env);
    applique_saisies_du_segment(seg, env);
    let mut vars_locales = vars_avec_affectations_locales(seg, &env.vars);
    let mut boites = (*env.conteneurs).clone();
    let mut fonctions = (*env.fonctions).clone();
    let noms_math = noms_de_fonctions_math(env, seg);
    let seg = normalise_affectations(seg);
    let seg = expand_tant_que_avec(&seg, &vars_locales, &mut boites, &fonctions);
    let seg: &str = &seg;
    let seg = expand_loops_avec(seg, &mut vars_locales, &mut boites, &mut fonctions, &noms_math);
    let seg: &str = &seg;
    let seg = expand_conditions_avec(seg, &vars_locales, Some(&boites), Some(&fonctions));
    for (nom, valeur) in &vars_locales {
        env.vars.insert(nom.clone(), *valeur);
    }
    env.conteneurs = std::sync::Arc::new(boites);
    env.fonctions = std::sync::Arc::new(fonctions);
    let mut toc = Vec::new();
    let html = render_body(&seg, env, &mut toc);
    (html, toc)
}

fn indentation_cm(ligne: &str) -> f32 {
    let r = reglages_page();
    let mut cm = 0.0f32;
    for c in ligne.chars() {
        match c {
            '\t' => cm += r.tabulation_cm,
            ' ' => cm += r.tabulation_cm / 4.0,
            _ => break,
        }
    }
    cm
}

fn flush_para(para: &mut Vec<String>, out: &mut String) {
    if !para.is_empty() {
        out.push_str("<p>");
        out.push_str(&para.join("<br>"));
        out.push_str("</p>");
        para.clear();
    }
}

fn lignes_vides(n: usize, out: &mut String) {
    let r = reglages_page();
    for _ in 0..n {
        out.push_str(&format!(
            "<div class=\"ligne-vide\" style=\"height:{}cm\"></div>",
            r.hauteur_cm
        ));
    }
}

pub fn render_body(text: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    render_body_indent(text, env, toc, true)
}

pub fn render_body_indent(
    text: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
    texte_libre: bool,
) -> String {
    let mut out = String::new();
    let mut para: Vec<String> = Vec::new();
    let mut vides = 0usize;
    for chunk in logical_chunks(text) {
        if env.bloque {
            break;
        }
        let t = chunk.trim();
        if t == "\u{E013}" {
            flush_para(&mut para, &mut out);
            continue;
        }
        if t.is_empty() {
            flush_para(&mut para, &mut out);
            vides += 1;
            continue;
        }
        if vides > 0 {
            if !out.is_empty() {
                lignes_vides(vides, &mut out);
            }
            vides = 0;
        }
        if texte_libre {
            if let Some((nom, ty, question)) = parse_saisie(t) {
                flush_para(&mut para, &mut out);
                out.push_str(&html_saisie(&nom, ty, &question, env, toc));
                continue;
            }
        }
        if t.starts_with("soit ") {
            flush_para(&mut para, &mut out);
            parse_def(t, env);
            continue;
        }
        if t.starts_with('<') {
            if let Some(html) = dispatch_chunk(t, env, toc) {
                flush_para(&mut para, &mut out);
                {
                    let cm = indentation_cm(&chunk);
                    if cm > 0.0 {
                        let (saut, corps) = match html.strip_prefix(SAUT_HTML) {
                            Some(reste) => (SAUT_HTML, reste),
                            None => ("", html.as_str()),
                        };
                        out.push_str(saut);
                        if !corps.is_empty() {
                            out.push_str(&format!(
                                "<div style=\"margin-left:{}cm\">{}</div>",
                                cm, corps
                            ));
                        }
                        continue;
                    }
                }
                out.push_str(&html);
                continue;
            }
        }

        if trouve_commande_en_ligne(t).filter(|p| *p > 0).is_some() {
            let mut morceaux = String::new();
            let mut cursor = 0usize;
            let mut bloc: Option<String> = None;
            while cursor < t.len() {
                match trouve_commande_en_ligne(&t[cursor..]) {
                    Some(p) => {
                        let abs = cursor + p;
                        morceaux.push_str(&render_inline(&t[cursor..abs], env, toc));
                        let fin = trouve_commande_en_ligne(&t[abs + 1..])
                            .map(|q| abs + 1 + q)
                            .unwrap_or(t.len());
                        let cmd = t[abs..fin].trim_end();
                        match read_tag(cmd).and_then(|(tag, after)| {
                            dispatch_command_inline(tag.trim(), &after, env)
                        }) {
                            Some(inline) => {
                                morceaux.push_str(&inline);
                                if fin < t.len() {
                                    morceaux.push(' ');
                                }
                                cursor = fin;
                            }
                            None => {
                                bloc = Some(t[abs..].to_string());
                                break;
                            }
                        }
                    }
                    None => {
                        morceaux.push_str(&render_inline(&t[cursor..], env, toc));
                        break;
                    }
                }
            }
            match bloc {
                Some(cmd) => {
                    let texte = morceaux.trim_end().to_string();
                    if !texte.is_empty() {
                        para.push(texte);
                    }
                    flush_para(&mut para, &mut out);
                    let html = dispatch_chunk(&cmd, env, toc)
                        .unwrap_or_else(|| format!("<p>{}</p>", render_inline(&cmd, env, toc)));
                    out.push_str(&html);
                }
                None => para.push(morceaux),
            }
            continue;
        }
        let mut rendu = render_inline(t, env, toc);
        {
            let cm = indentation_cm(&chunk);
            if cm > 0.0 {

                let saut = if para.is_empty() { "" } else { "<br>" };
                rendu = format!(
                    "{}<span style=\"display:inline-block;width:{}cm\"></span>{}",
                    saut, cm, rendu
                );
            }
        }
        para.push(rendu);
    }
    flush_para(&mut para, &mut out);
    out
}

pub(crate) fn find_body_brace(s: &str) -> Option<(String, String)> {
    let ferme = s.trim_end();
    if !ferme.ends_with('}') {
        return None;
    }
    let octets = ferme.as_bytes();
    let mut depth = 0i32;
    let mut open = None;
    for i in (0..octets.len()).rev() {
        match octets[i] {
            b'}' => depth += 1,
            b'{' => {
                depth -= 1;
                if depth == 0 {
                    open = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }
    let open = open?;
    Some((ferme[..open].to_string(), ferme[open + 1..ferme.len() - 1].to_string()))
}

const CALCUL: u8 = 1;
const EN_LIGNE: u8 = 2;

const VERBES: &[(&str, u8)] = &[
    ("Ajuste", CALCUL | EN_LIGNE),
    ("Applique", EN_LIGNE),
    ("Calcule", CALCUL | EN_LIGNE),
    ("Complète", 0),
    ("Construis", CALCUL),
    ("Convertis", CALCUL),
    ("Décompose", CALCUL | EN_LIGNE),
    ("Dénombre", EN_LIGNE),
    ("Détermine", CALCUL | EN_LIGNE),
    ("Développe", CALCUL | EN_LIGNE),
    ("Diagonalise", CALCUL | EN_LIGNE),
    ("Donne", CALCUL),
    ("Dresse", 0),
    ("Effectue", CALCUL | EN_LIGNE),
    ("Exprime", EN_LIGNE),
    ("Factorise", CALCUL | EN_LIGNE),
    ("Insère", 0),
    ("Orthonormalise", CALCUL),
    ("Représente", 0),
    ("Résous", CALCUL | EN_LIGNE),
    ("Simplifie", CALCUL | EN_LIGNE),
    ("Place", 0),
    ("Trace", 0),
    ("Trigonalise", CALCUL | EN_LIGNE),
    ("Vérifie", CALCUL | EN_LIGNE),
    ("Écris", EN_LIGNE),
    ("Équilibre", CALCUL),
    ("Propage", CALCUL | EN_LIGNE),
    ("Étudie", CALCUL | EN_LIGNE),
];

fn verbe(mot: &str) -> &str {
    VERBES
        .iter()
        .find(|(v, _)| crate::utils::texte::meme_mot(v, mot))
        .map(|(v, _)| *v)
        .unwrap_or(mot)
}

fn drapeaux(mot: &str) -> u8 {
    VERBES
        .iter()
        .find(|(v, _)| crate::utils::texte::meme_mot(v, mot))
        .map(|(_, f)| *f)
        .unwrap_or(0)
}

pub(crate) fn bloc_calcul(inner: &str) -> String {
    format!("<div class=\"calcul\">\\[{}\\]</div>", inner)
}

fn bloc_inconnu(commande: &str) -> String {
    let (verbe, suite) = match commande.split_once('>') {
        Some((v, r)) => (v, r),
        None => (commande, ""),
    };
    crate::utils::erreur::bloc(
        &format!("<{}>{}", verbe, suite),
        "commande non prise en charge",
    )
}

const OPS_AVEC_SOURCE: &[&str] = &["arith", "factor", "expand", "simplify", "apart", "canonical"];

fn trouve_commande_en_ligne(t: &str) -> Option<usize> {
    let mut idx = 0;
    while let Some(p) = t[idx..].find('<') {
        let pos = idx + p;
        let reste = &t[pos + 1..];
        if let Some(fin) = reste.find('>') {
            let mot = reste[..fin].split_whitespace().next().unwrap_or("");
            if drapeaux(mot) & EN_LIGNE != 0 {
                return Some(pos);
            }
        }
        idx = pos + '<'.len_utf8();
    }
    None
}

fn verbe_et_reste(tag_t: &str) -> (&str, &str) {
    let (mot, reste) = match tag_t.split_once(char::is_whitespace) {
        Some((v, r)) => (v, r.trim()),
        None => (tag_t, ""),
    };
    (verbe(mot), reste)
}

fn dispatch_command_inline(tag_t: &str, after: &str, env: &mut Env) -> Option<String> {
    let (verb, rest_in_tag) = verbe_et_reste(tag_t);
    let (rest, corps) = desc_et_corps(rest_in_tag, after);
    if corps.is_some() {
        return None;
    }
    if let Some(tex) = crate::maths::algebre::commande_en_ligne(verb, &rest) {
        return Some(format!("<span class=\"calcul-en-ligne\">\\({}\\)</span>", tex));
    }
    if drapeaux(verb) & CALCUL == 0 {
        return None;
    }
    let req = match crate::langage::commandes::parse_command(verb, &rest)? {
        serde_json::Value::Array(_) => return None,
        r => r,
    };
    let op = req.get("op").and_then(|v| v.as_str()).unwrap_or("");
    if OPS_EN_PROSE.contains(&op) {
        return None;
    }
    let mut full = req.clone();
    if let Some(o) = full.as_object_mut() {
        o.insert("defs".into(), crate::langage::commandes::objects_json(&env.objects));
    }
    let latex = crate::python::pont::ask(&full.to_string()).ok()?;
    if latex.contains('\n') || latex.contains("\\text") {
        return None;
    }
    let inner = match req
        .get("args")
        .and_then(|a| a.get("expr"))
        .and_then(|v| v.as_str())
        .filter(|_| OPS_AVEC_SOURCE.contains(&op))
        .map(to_latex)
    {
        Some(src) => format!("{} = {}", src, latex),
        None => latex,
    };
    Some(format!("<span class=\"calcul-en-ligne\">\\({}\\)</span>", inner))
}

const OPS_EN_PROSE: &[&str] = &[
    "system",
    "markov",
    "polydiv",
    "polygcd",
    "integral_nature",
    "series_nature",
    "critical",
    "lagrange",
    "multi_integral",
    "residus",
    "densite",
    "normale",
    "fourier",
    "wronskian",
    "laplace_inv",
    "convexity",
    "asymptotes",
    "trig_solve",
    "vecteur",
    "incertitude",
];

fn execute(req: &serde_json::Value, env: &Env, source: &str) -> String {
    let mut full = req.clone();
    if let Some(o) = full.as_object_mut() {
        o.insert(
            "defs".into(),
            crate::langage::commandes::objects_json(&env.objects),
        );
    }
    let op = req.get("op").and_then(|v| v.as_str()).unwrap_or("");
    let args = req.get("args");
    let affichage = args
        .and_then(|a| a.get("expr"))
        .and_then(|v| v.as_str())
        .filter(|_| OPS_AVEC_SOURCE.contains(&op))
        .map(to_latex);
    let annonce = op == "factor"
        && args
        .and_then(|a| a.get("annonce"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    match crate::python::pont::ask(&full.to_string()) {
        Ok(latex) => {
            if OPS_EN_PROSE.contains(&op) {
                let lignes: Vec<String> = latex.lines().map(|l| l.to_string()).collect();
                return crate::maths::algebre::bloc_prose(&lignes);
            }
            if annonce {
                let anneau = args
                    .and_then(|a| a.get("ring"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("R");
                let src = affichage.unwrap_or_default();
                return crate::maths::algebre::bloc_prose(&[format!(
                    "Dans \\(\\mathbb{{{}}}[X]\\) : \\({} = {}\\).",
                    anneau, src, latex
                )]);
            }
            match affichage {
                Some(src) => bloc_calcul(&format!("{} = {}", src, latex)),
                None => bloc_calcul(&latex),
            }
        }
        Err(e) => format!(
            "{}",
            crate::utils::erreur::bloc(&format!("<{}>", source), &e)
        ),
    }
}

fn relation_tex(eq: &str) -> String {
    let nettoye = eq.replace('≤', "<=").replace('≥', ">=");
    for (op, tex) in [("<=", "\\leqslant"), (">=", "\\geqslant"), ("=", "="), ("<", "<"), (">", ">")]
    {
        if let Some((g, d)) = nettoye.split_once(op) {
            return format!(
                "{} &{} {}",
                crate::maths::algebre::decimales_fr(&to_latex(g.trim())),
                tex,
                crate::maths::algebre::decimales_fr(&to_latex(d.trim()))
            );
        }
    }
    crate::maths::algebre::decimales_fr(&to_latex(nettoye.trim()))
}

fn systeme_tex(eqs: &[String]) -> String {
    let rangs: Vec<String> = eqs.iter().map(|e| relation_tex(e)).collect();
    format!(
        "\\left\\{{\\begin{{aligned}}{}\\end{{aligned}}\\right.",
        rangs.join(" \\\\ ")
    )
}

fn matrice_pmatrix(rows: &[Vec<String>]) -> String {
    let corps = rows
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| crate::maths::algebre::decimales_fr(&to_latex(c)))
                .collect::<Vec<_>>()
                .join(" & ")
        })
        .collect::<Vec<_>>()
        .join(" \\\\ ");
    format!("\\begin{{pmatrix}}{}\\end{{pmatrix}}", corps)
}

fn math_notation(texte: &str) -> String {
    let t = texte.trim();
    let (tete, definition) = match t.split_once("défini par") {
        Some((a, b)) => (a.trim(), Some(b.trim())),
        None => (t, None),
    };
    let tete = tete
        .replace("de RR^3 dans RR^3", "de \\(\\mathbb{R}^3\\) dans \\(\\mathbb{R}^3\\)")
        .replace("de RR^2 dans RR^2", "de \\(\\mathbb{R}^2\\) dans \\(\\mathbb{R}^2\\)")
        .replace("de RR dans RR", "de \\(\\mathbb{R}\\) dans \\(\\mathbb{R}\\)");
    match definition {
        Some(d) => format!(
            " {} défini par \\({}\\)",
            tete,
            crate::maths::algebre::decimales_fr(&to_latex(d))
        ),
        None => format!(" {}", tete),
    }
}

fn declaration_html(noms: &[String], env: &Env, pose: bool) -> String {
    let mut out = String::new();
    let mut fonctions: Vec<String> = Vec::new();

    let mut points: Vec<(String, String)> = Vec::new();
    let mut vecteurs: Vec<(String, String)> = Vec::new();
    for nom in noms {
        match env.objects.get(nom) {

            Some(crate::langage::commandes::Obj::Notation { phrase }) => {
                let (avant, apres) = phrase
                    .split_once(&format!("vecteurs {}", nom))
                    .map(|(a, b)| (a.to_string(), b.to_string()))
                    .unwrap_or((phrase.clone(), String::new()));
                let nature = avant.trim_end().trim_end_matches("le champ de");
                let _ = nature;
                out.push_str(&crate::maths::algebre::bloc_prose(&[format!(
                    "Soit \\({}\\) le champ de vecteurs{}.",
                    nom,
                    math_notation(apres.trim_end_matches('.'))
                )]));
            }
            Some(crate::langage::commandes::Obj::Matrix { rows }) => {
                out.push_str(&crate::maths::algebre::bloc_prose(&[format!(
                    "Soit \\({}\\) la matrice définie par : \\[{} = {}\\]",
                    nom,
                    nom,
                    matrice_pmatrix(rows)
                )]))
            }

            Some(crate::langage::commandes::Obj::System { eqs }) => {
                out.push_str(&crate::maths::algebre::bloc_prose(&[format!(
                    "Soit \\(({})\\) le système : \\[{}\\]",
                    nom,
                    systeme_tex(eqs)
                )]))
            }
            Some(crate::langage::commandes::Obj::Sequence { first, rec }) => {
                let relation = crate::maths::algebre::decimales_fr(&to_latex(rec))
                    .replace("PREV", &format!("{}_{{n}}", nom));
                out.push_str(&crate::maths::algebre::bloc_prose(&[format!(
                    "Soit \\(({}_n)\\) la suite définie par récurrence, avec \\({}_0 = {}\\) et \\({}_{{n+1}} = {}\\).",
                    nom,
                    nom,
                    crate::maths::algebre::decimales_fr(&to_latex(first)),
                    nom,
                    relation
                )]));
            }
            Some(crate::langage::commandes::Obj::Point { coords }) => points.push((
                format!("\\({}\\)", nom),
                format!("\\(\\left({}\\right)\\)", coords.join("\\ ;\\ ")),
            )),
            Some(crate::langage::commandes::Obj::Vecteur { coords }) => vecteurs.push((
                format!("\\(\\vec{{{}}}\\)", nom),
                format!("\\(\\left({}\\right)\\)", coords.join("\\ ;\\ ")),
            )),
            Some(crate::langage::commandes::Obj::Plan { equation }) => {
                out.push_str(&crate::maths::algebre::bloc_prose(&[format!(
                    "Soit le plan \\(\\mathcal{{P}}\\) d'équation \\({}\\).",
                    to_latex(equation)
                )]))
            }
            Some(crate::langage::commandes::Obj::Function { .. }) => fonctions.push(nom.clone()),
            _ => {}
        }
    }

    for (objets, singulier, pluriel) in [
        (&points, "le point de coordonnées", "les points de coordonnées"),
        (&vecteurs, "le vecteur de coordonnées", "les vecteurs de coordonnées"),
    ] {
        if objets.is_empty() {
            continue;
        }
        let joint = |v: Vec<String>| match v.len() {
            1 => v[0].clone(),
            n => format!("{} et {}", v[..n - 1].join(", "), v[n - 1]),
        };
        let noms = joint(objets.iter().map(|(n, _)| n.clone()).collect());
        let coords = joint(objets.iter().map(|(_, c)| c.clone()).collect());
        let (tete, nature) = if objets.len() == 1 {
            ("Soit", singulier)
        } else {
            ("Soient", pluriel)
        };
        out.push_str(&crate::maths::algebre::bloc_prose(&[format!(
            "{} {} {} {}.",
            tete, noms, nature, coords
        )]));
    }
    if !fonctions.is_empty() {

        let corps: Vec<String> = fonctions
            .iter()
            .filter_map(|nom| match env.objects.get(nom) {
                Some(crate::langage::commandes::Obj::Function { var, expr }) => Some(format!(
                    "{}({}) = {}",
                    to_latex(nom),
                    to_latex(var),
                    crate::maths::algebre::decimales_fr(&to_latex(expr))
                )),
                _ => None,
            })
            .collect();

        let noms: Vec<String> = fonctions.iter().map(|n| to_latex(n)).collect();
        let liste_noms = match noms.len() {
            1 => format!("\\({}\\)", noms[0]),
            n => format!(
                "\\({}\\) et \\({}\\)",
                noms[..n - 1].join("\\), \\("),
                noms[n - 1]
            ),
        };
        let formules: Vec<String> = corps.iter().map(|c| format!("\\({}\\)", c)).collect();
        let liste_formules = match formules.len() {
            1 => formules[0].clone(),
            n => format!("{} et {}", formules[..n - 1].join(", "), formules[n - 1]),
        };
        let phrase = match (pose, corps.len() > 1) {
            (true, false) => format!("On pose {}.", liste_formules),
            (true, true) => format!("On pose {}.", liste_formules),
            (false, false) => format!(
                "Soit {} la fonction définie par {}.",
                liste_noms, liste_formules
            ),
            (false, true) => format!(
                "Soient {} les fonctions définies par {}.",
                liste_noms, liste_formules
            ),
        };
        out.push_str(&crate::maths::algebre::bloc_prose(&[phrase]));
    }
    out
}

fn declare(tag_t: &str, after: &str, env: &mut Env) -> Option<String> {
    let pose = tag_t.starts_with("On pose");
    let head = if pose {
        tag_t["On pose".len()..].trim()
    } else if let Some(r) = tag_t.strip_prefix("Soit") {
        r.trim()
    } else {
        ""
    };
    let after_trim = after.trim_start();
    let block = after_trim
        .find('{')
        .and_then(|i| take_group(after_trim, i).map(|(b, _)| b));
    let decl = if head.is_empty() {

        let ligne = after_trim.lines().next().unwrap_or("");
        match ligne.find('{') {
            Some(i) if i > 0 => ligne[..i].trim().to_string(),
            _ => ligne.trim().to_string(),
        }
    } else {
        head.to_string()
    };

    if decl.starts_with('{') {
        let interieur = block.as_deref()?;
        let mut html = String::new();
        for ligne in interieur.lines() {
            let ligne = ligne.trim();
            if ligne.is_empty() {
                continue;
            }
            match declare(tag_t, ligne, env) {
                Some(un) => html.push_str(&un),
                None => {
                    html.push_str(&crate::utils::erreur::bloc(
                        &format!("<{}>{}", tag_t, ligne),
                        "déclaration non reconnue",
                    ));
                }
            }
        }
        return Some(html);
    }
    if decl.is_empty() {
        return None;
    }
    let noms = crate::langage::commandes::parse_declaration(&decl, block.as_deref(), &mut env.objects)?;
    Some(declaration_html(&noms, env, pose))
}

fn desc_et_corps(rest_in_tag: &str, after: &str) -> (String, Option<String>) {
    if !rest_in_tag.is_empty() {
        let after_trim = after.trim_start();
        let corps = if after_trim.starts_with('{') {
            take_group(after_trim, 0).map(|(b, _)| b)
        } else {
            None
        };
        return (rest_in_tag.to_string(), corps);
    }
    if let Some((desc, corps)) = find_body_brace(after) {
        return (desc.trim().to_string(), Some(corps));
    }
    (
        after.trim().lines().next().unwrap_or("").trim().to_string(),
        None,
    )
}

fn desc_et_corps_affiche_dresse(tag_t: &str, after: &str) -> Option<(String, String)> {
    let verb_len = if tag_t.starts_with("Affiche") { 7 } else { 6 };
    let desc_in_tag = tag_t[verb_len..].trim().to_string();
    let after_trim = after.trim_start();
    if after_trim.starts_with('{') && !desc_in_tag.is_empty() {
        take_group(after_trim, 0).map(|(b, _)| (desc_in_tag.clone(), b))
    } else {
        find_body_brace(after).map(|(d2, b)| (format!("{} {}", desc_in_tag, d2.trim()), b))
    }
}

fn dispatch_command(tag_t: &str, after: &str, env: &mut Env) -> Option<String> {
    let (verb, rest_in_tag) = verbe_et_reste(tag_t);

    if verb == "Soit" || tag_t.starts_with("On pose") {
        return declare(tag_t, after, env).or_else(|| Some(bloc_inconnu(tag_t)));
    }

    let (rest, corps) = desc_et_corps(rest_in_tag, after);
    let rest = interpole_diese(&rest, &env.vars, &env.textes);
    if let Some(html) = crate::maths::algebre::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::layout::frise::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::analyse::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::chimie::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::physique::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::espace::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::geometrie::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::statistiques::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::courbes::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::surfaces::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::complexe::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::groupes::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::lois::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if let Some(html) = crate::maths::trace::commande(verb, &rest, corps.as_deref(), env) {
        return Some(html);
    }
    if drapeaux(verb) & CALCUL == 0 {
        return None;
    }

    let etiquette = if rest_in_tag.is_empty() {
        format!("{}>{}", verb, rest)
    } else {
        format!("{} {}", verb, rest)
    };
    match crate::langage::commandes::parse_command(verb, &rest) {
        Some(serde_json::Value::Array(items)) => {
            let mut out = String::new();
            for it in &items {
                out.push_str(&execute(it, env, &etiquette));
            }
            Some(out)
        }
        Some(req) => Some(execute(&req, env, &etiquette)),
        None => Some(bloc_inconnu(tag_t)),
    }
}

const SAUT_HTML: &str = "<div class=\"pagebreak\"></div>";

pub(crate) fn saut_de_page(tag: &str) -> Option<&str> {
    let t = tag.trim();
    let (mot, reste) = match t.find(char::is_whitespace) {
        Some(i) => t.split_at(i),
        None => return None,
    };
    if !mot.eq_ignore_ascii_case("page") {
        return None;
    }
    let reste = reste.trim_start();
    let fin = reste.find(char::is_whitespace).unwrap_or(reste.len());
    if !reste[..fin].eq_ignore_ascii_case("suivante") {
        return None;
    }
    Some(reste[fin..].trim())
}

fn balise_supprimee(tag: &str) -> bool {
    let t = tag.trim();
    if saut_de_page(t).is_some() {
        return false;
    }
    if t == "ligne" || t == "nouvelle ligne" {
        return true;
    }
    for suffixe in ["tabulations", "tabulation", "lignes", "ligne"] {
        if let Some(n) = t.strip_suffix(suffixe) {
            let n = n.trim();
            if !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()) {
                return true;
            }
        }
    }
    false
}

fn render_matrice(after: &str) -> Option<String> {
    let t = after.trim();
    let bas = t.to_lowercase();
    let reste = bas
        .strip_prefix("la matrice")
        .or_else(|| bas.strip_prefix("une matrice"))?;
    let debut = t.len() - reste.len();
    let corps = t[debut..].trim_start();
    let ouvrant = corps.chars().next()?;
    let (fermant, gauche, droite) = match ouvrant {
        '(' => (')', "\\left(", "\\right)"),
        '[' => (']', "\\left[", "\\right]"),
        _ => return None,
    };
    let fin = corps.rfind(fermant)?;
    let interieur = &corps[ouvrant.len_utf8()..fin];

    let mut rangees: Vec<Vec<String>> = Vec::new();
    let mut barre: Option<usize> = None;
    for ligne in interieur.lines() {
        let l = ligne.trim();
        if l.is_empty() || l == SEP_ITERATION.to_string() {
            continue;
        }
        let mut cellules: Vec<String> = Vec::new();
        for morceau in l.split(';') {
            let morceau = morceau.trim();
            if let Some((g, d)) = morceau.split_once('|') {
                cellules.push(g.trim().to_string());
                barre = Some(cellules.len());
                cellules.push(d.trim().to_string());
            } else {
                cellules.push(morceau.to_string());
            }
        }
        rangees.push(cellules);
    }
    if rangees.is_empty() {
        return None;
    }

    let ncols = rangees.iter().map(|r| r.len()).max().unwrap_or(1);
    let mut spec = String::new();
    for i in 0..ncols {
        if barre == Some(i) {
            spec.push('|');
        }
        spec.push('c');
    }

    let corps_tex = rangees
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| to_latex(c))
                .collect::<Vec<_>>()
                .join(" & ")
        })
        .collect::<Vec<_>>()
        .join(" \\\\ ");

    Some(bloc_calcul(&format!(
        "{}\\begin{{array}}{{{}}}{}\\end{{array}}{}",
        gauche, spec, corps_tex, droite
    )))
}

fn dispatch_chunk(chunk: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> Option<String> {
    let (tag, after) = read_tag(chunk)?;
    let tag_t = tag.trim();
    if balise_supprimee(tag_t) {
        let reste = after.trim();
        if reste.is_empty() {
            return Some(String::new());
        }
        return Some(format!("<p>{}</p>", render_inline(reste, env, toc)));
    }
    if tag_t == "page de titre" {
        let lit = |env: &Env, k: &str| env.textes.get(&format!("document:{}", k)).cloned();
        let mut html = String::from("<div class=\"page-de-titre\">");
        if let Some(t) = lit(env, "titre") {
            html.push_str(&format!(
                "<div class=\"titre-doc\">{}</div>",
                render_inline(&t, env, toc)
            ));
        }
        if let Some(a) = lit(env, "auteur") {
            html.push_str(&format!(
                "<div class=\"auteur-doc\">{}</div>",
                render_inline(&a, env, toc)
            ));
        }
        if let Some(i) = lit(env, "institution") {
            html.push_str(&format!(
                "<div class=\"institution-doc\">{}</div>",
                render_inline(&i, env, toc)
            ));
        }
        if let Some(d) = lit(env, "date") {
            html.push_str(&format!(
                "<div class=\"date-doc\">{}</div>",
                render_inline(&d, env, toc)
            ));
        }
        html.push_str("</div>");
        html.push_str(SAUT_HTML);
        return Some(html);
    }
    if tag_t.starts_with("table des matières") {
        let title = take_group(after.trim_start(), 0)
            .map(|(t, _)| t)
            .unwrap_or_else(|| "Sommaire".into());
        return Some(format!(
            "<div class=\"toc\"><div class=\"toc-title\">{}</div>\u{E010}</div>",
            html_escape(&title)
        ));
    }
    if let Some(rest) = saut_de_page(tag_t) {
        let mut html = SAUT_HTML.to_string();
        let suite = after.trim();
        let style = match env.defs.get(rest).cloned() {
            Some(Def::Style(w)) => Some(style_css(&w)),
            _ => None,
        };
        match style {
            Some((css, Some(lvl))) => html.push_str(&heading(env, toc, lvl, &css, suite)),
            Some((css, None)) => html.push_str(&format!(
                "<p><span style=\"{}\">{}</span></p>",
                css,
                render_inline(suite, env, toc)
            )),
            None if !suite.is_empty() => match dispatch_chunk(suite, env, toc) {
                Some(bloc) => html.push_str(&bloc),
                None => html.push_str(&format!("<p>{}</p>", render_inline(suite, env, toc))),
            },
            None => {}
        }
        return Some(html);
    }

    if crate::utils::texte::meme_mot(tag_t, "Énonce") {
        return Some(crate::layout::environnements::rend(&after, env, toc));
    }
    if tag_t == "Montre" || tag_t.starts_with("Montre ") {
        let (verb, modificateur) = verbe_et_reste(tag_t);
        if verb == "Montre" {

            if !after.trim_start().lines().next().unwrap_or("").contains('{') {
                let complement = after.trim().lines().next().unwrap_or("").trim();
                let complement = interpole_diese(complement, &env.vars, &env.textes);

                let (depuis_complement, reste) =
                    super::demonstration::separe_raisonnement(&complement);
                let (modificateur, desc) = if !modificateur.is_empty() {
                    (modificateur.to_string(), complement.clone())
                } else {
                    (depuis_complement, reste)
                };
                let modificateur = modificateur.as_str();
                if !desc.is_empty() {
                    match modificateur {
                        "par récurrence" => {
                            if let Some(html) =
                                super::demonstration::recurrence_automatique(&desc, env, toc)
                            {
                                return Some(html);
                            }
                        }
                        "par élément quelconque" => {
                            if let Some(html) =
                                super::demonstration::pour_tout_automatique(&desc, env, toc)
                            {
                                return Some(html);
                            }
                        }
                        _ => {}
                    }

                    if let Some(html) = super::demonstration::depuis_bibliotheque(
                        &desc,
                        modificateur,
                        env,
                        toc,
                    ) {
                        return Some(html);
                    }

                    let proches = super::demonstration::suggestions(&desc);
                    if !proches.is_empty() {
                        return Some(crate::utils::erreur::bloc(
                            &format!("<{}>", tag_t),
                            &format!(
                                "cette démonstration n'est ni calculable ni en bibliothèque — écrivez son corps entre accolades, ou voyez : {}",
                                proches.join(" ; ")
                            ),
                        ));
                    }
                }
            }

            if let Some((desc, corps)) = find_body_brace(&after) {
                let desc = interpole_diese(desc.trim(), &env.vars, &env.textes);
                let (depuis_complement, reste) =
                    super::demonstration::separe_raisonnement(&desc);
                let (modificateur, desc) = if !modificateur.is_empty() {
                    (modificateur.to_string(), desc)
                } else {
                    (depuis_complement, reste)
                };
                let modificateur = modificateur.as_str();
                if modificateur.is_empty() && desc.trim().starts_with("l'existence et l'unicité")
                {
                    if let Some(html) =
                        super::demonstration::existence_unicite(&desc, &corps, env, toc)
                    {
                        return Some(html);
                    }
                }
                let tag_norme = if modificateur.is_empty() {
                    "Montre".to_string()
                } else {
                    format!("Montre {}", modificateur)
                };
                if let Some(html) =
                    super::demonstration::montre(&tag_norme, &desc, &corps, env, toc)
                {
                    return Some(html);
                }
            }
        }
    }

    if verbe_et_reste(tag_t).0 == "Écris" {
        if let Some((desc, corps)) = find_body_brace(&after) {
            if let Some(html) = super::ecriture::ecris(&desc, &corps, env, toc) {
                return Some(html);
            }
        }
    }
    if let Some(html) = dispatch_command(tag_t, &after, env) {
        return Some(html);
    }
    if tag_t.starts_with("Affiche") {
        if let Some(html) = render_matrice(&after) {
            return Some(html);
        }
    }
    if tag_t.starts_with("Affiche") || tag_t.starts_with("Dresse") {
        if let Some((desc, body)) = desc_et_corps_affiche_dresse(tag_t, &after) {
            return Some(dispatch_block(&desc, &body, env, toc));
        }
    }
    if tag_t.starts_with("Insère") {
        let img = render_image(tag_t, after.trim_start());
        return Some(format!("<p>{}</p>", img));
    }
    let name = tag_t.split_whitespace().next().unwrap_or("");

    if !after.trim_start().starts_with('{') {
        if let Some(Def::Style(w)) = env.defs.get(tag_t).cloned() {
            let (css, lvl) = style_css(&w);
            if let Some(lvl) = lvl {
                return Some(heading(env, toc, lvl, &css, after.trim()));
            }
            return Some(format!(
                "<p><span style=\"{}\">{}</span></p>",
                css,
                render_inline(after.trim(), env, toc)
            ));
        }
    }
    if let Some(Def::Component { param, template }) = env.defs.get(name).cloned() {
        let arg = tag_t[name.len()..].trim().trim_start_matches('#').to_string();
        let expanded = subst_var(&template, &param, &arg);
        let synthetic = format!("<{}>{}", expanded, after);
        return dispatch_chunk(&synthetic, env, toc);
    }

    if tag_t
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
    {
        let mots: Vec<String> = tag_t.split_whitespace().map(|x| x.to_string()).collect();
        if known_style_words(&mots) {
            return None;
        }
        let suite = after.trim().lines().next().unwrap_or("").trim();
        let commande = if suite.is_empty() {
            tag_t.to_string()
        } else {
            format!("{}>{}", tag_t, suite)
        };
        return Some(bloc_inconnu(&commande));
    }
    None
}

fn extract_titled(desc: &str) -> (String, Option<String>) {
    if let Some(i) = desc.find("titre") {
        let after = &desc[i + 5..];
        if let Some(bi) = after.find('{') {
            if after[..bi].trim().is_empty() {
                if let Some((title, rest)) = take_group(after, bi) {
                    let mut d = desc[..i].to_string();
                    d.push_str(&rest);
                    return (d, Some(title));
                }
            }
        }
    }
    (desc.to_string(), None)
}

fn qualifie_autrui(desc_words: &[&str], i: usize) -> bool {
    let mut j = i;
    while j > 0 {
        match desc_words[j - 1].trim_matches(|c: char| c == ',' || c == '.') {
            "sur" => return true,
            "un" | "une" | "le" | "la" | "les" | "des" | "de" | "du" => j -= 1,
            _ => return false,
        }
    }
    false
}

fn desc_color(desc_words: &[&str], key: &str) -> Option<&'static str> {
    let mut i = 0;
    while i < desc_words.len() {
        if desc_words[i].trim_matches(|c: char| c == ',') == key && !qualifie_autrui(desc_words, i) {
            if let Some((c, _)) = parse_color_at(&clean_words(&desc_words[i + 1..])) {
                return Some(c);
            }
        }
        i += 1;
    }
    None
}

fn est_secable(desc: &str) -> bool {
    let d = desc.to_lowercase();
    (d.contains("sécable") || d.contains("secable"))
        && !d.contains("insécable")
        && !d.contains("insecable")
}

fn clean_words<'a>(w: &[&'a str]) -> Vec<&'a str> {
    w.iter()
        .map(|s| s.trim_matches(|c: char| c == ',' || c == '.'))
        .filter(|s| !s.is_empty())
        .collect()
}

fn desc_mm(desc: &str, key: &str) -> Option<f32> {
    let i = desc.find(key)?;
    let after = &desc[i + key.len()..];
    let after = after.trim_start().trim_start_matches("de").trim_start();
    let num: String = after
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
        .collect();
    let value = num.replace(',', ".").parse::<f32>().ok()?;
    let rest = after[num.len()..].trim_start();
    let factor = if rest.starts_with("cm") {
        10.0
    } else if rest.starts_with("mm") {
        1.0
    } else if rest.starts_with("pt") {
        0.352_778
    } else {
        1.0
    };
    Some(value * factor)
}

fn numero_courant(env: &Env) -> Option<(String, String)> {
    let dernier = env.counters.iter().rposition(|&c| c != 0);
    match dernier {
        Some(l) => {
            let mut num: String = env.counters[..=l]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(".");
            if env.chapitre > 0 {
                num = format!("{}.{}", env.chapitre, num);
            }
            let id = format!("sec-{}", num.replace('.', "-"));
            Some((num, id))
        }
        None if env.chapitre > 0 => {
            Some((env.chapitre.to_string(), format!("chap-{}", env.chapitre)))
        }
        None => None,
    }
}

pub(crate) fn declare_renvoi(nom: &str, num: &str, id: &str) -> String {
    format!("\u{E016}{}|{}|{}\u{E017}", nom, num, id)
}

fn render_bibliography(body: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let mut html = String::from("<div class=\"bibliographie\">");
    let mut n = 0usize;
    for ligne in body.lines() {
        let t = ligne.trim();
        if t.is_empty() || !t.starts_with('[') {
            continue;
        }
        let Some(fin) = t.find(']') else { continue };
        let cle = t[1..fin].trim();
        let texte = t[fin + 1..].trim();
        if cle.is_empty() {
            continue;
        }
        n += 1;
        html.push_str(&declare_renvoi(
            &format!("cite:{}", cle),
            &format!("[{}]", n),
            &format!("bib-{}", cle),
        ));
        html.push_str(&format!(
            "<div class=\"bib-entree\" id=\"bib-{}\"><span class=\"bib-num\">[{}]</span> {}</div>",
            cle,
            n,
            render_inline(texte, env, toc)
        ));
    }
    html.push_str("</div>");
    html
}

fn dispatch_block(desc: &str, body: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    if desc.contains("une bibliographie") {
        render_bibliography(body, env, toc)
    } else if desc.contains("un cadre") {
        render_frame(desc, body, env, toc)
    } else if desc.contains("une liste") {
        render_list(desc, body, env, toc)
    } else if desc.contains("un tableau") {
        render_table(desc, body, env, toc)
    } else if desc.contains("une rangée") {
        render_row(desc, body, env, toc)
    } else if desc.contains("une grille") {
        render_grid(desc, body, env, toc)
    } else {
        format!("<div>{}</div>", render_body_indent(body, env, toc, false))
    }
}

fn render_frame(desc: &str, body: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let (desc2, title) = extract_titled(desc);
    let words: Vec<&str> = desc2.split_whitespace().collect();
    let bg = desc_color(&words, "fond").unwrap_or("transparent");
    let border = desc_color(&words, "bordure").unwrap_or("#444");
    let radius = desc_mm(&desc2, "coins arrondis").unwrap_or(0.0);
    let secable = est_secable(&desc2);
    let mut html = format!(
        "<div class=\"cadre{}\" style=\"background:{};border:0.4mm solid {};border-radius:{}mm\">",
        if secable { " secable" } else { "" },
        bg, border, radius
    );
    if let Some(t) = title {
        html.push_str(&format!(
            "<div class=\"cadre-titre\" style=\"background:{};border-radius:{}mm {}mm 0 0\">{}</div>",
            border,
            (radius - 0.4).max(0.0),
            (radius - 0.4).max(0.0),
            render_inline(t.trim(), env, toc)
        ));
    }
    let mut segments: Vec<String> = Vec::new();
    {
        let mut cur = String::new();
        for line in body.lines() {
            if line.trim() == "---" {
                segments.push(std::mem::take(&mut cur));
            } else {
                cur.push_str(line);
                cur.push('\n');
            }
        }
        segments.push(cur);
    }
    for (i, part) in segments.iter().enumerate() {
        if i > 0 {
            html.push_str(&format!(
                "<div class=\"cadre-sep\" style=\"border-top:0.3mm dashed {}\"></div>",
                border
            ));
        }
        html.push_str("<div class=\"cadre-corps\">");
        html.push_str(&render_body_indent(part, env, toc, false));
        html.push_str("</div>");
    }
    html.push_str("</div>");
    html
}

fn list_style(desc: &str) -> (&'static str, &'static str, bool) {
    if desc.contains("à cocher") {
        ("ul", "none", true)
    } else if desc.contains("puces vides") {
        ("ul", "circle", false)
    } else if desc.contains("puces carrées") {
        ("ul", "square", false)
    } else if desc.contains("chiffres romains") {
        ("ol", "upper-roman", false)
    } else if desc.contains("numérotée") {
        ("ol", "decimal", false)
    } else {
        ("ul", "disc", false)
    }
}

fn render_list(desc: &str, body: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let body = &sans_separateur(body);
    let (tag, style, check) = list_style(desc);
    let mut html = format!("<{} class=\"liste\" style=\"list-style-type:{}\">", tag, style);
    let mut open = false;
    for chunk in logical_chunks(body) {
        let t = chunk.trim();
        if t.is_empty() {
            continue;
        }
        if t.starts_with('<') {
            if let Some((itag, iafter)) = read_tag(t) {
                let itag_t = itag.trim();
                if itag_t.starts_with("Dresse") || itag_t.starts_with("Affiche") {
                    if itag_t.contains("liste") || iafter.trim_start().starts_with("une liste") {
                        if let Some((d, b)) = desc_et_corps_affiche_dresse(itag_t, &iafter) {
                            html.push_str(&render_list(&d, &b, env, toc));
                            continue;
                        }
                    }
                    if let Some(h) = dispatch_chunk(t, env, toc) {
                        if open {
                            html.push_str("</li>");
                            open = false;
                        }
                        html.push_str(&format!("<li style=\"list-style:none\">{}</li>", h));
                        continue;
                    }
                }
            }
        }
        if open {
            html.push_str("</li>");
        }
        let prefix = if check { "\u{2610}&nbsp;&nbsp;" } else { "" };
        html.push_str(&format!("<li>{}{}", prefix, render_inline(t, env, toc)));
        open = true;
    }
    if open {
        html.push_str("</li>");
    }
    html.push_str(&format!("</{}>", tag));
    html
}

fn render_row(desc: &str, body: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let gap = desc_mm(desc, "écart").unwrap_or(4.0);
    let mut html = format!(
        "<div class=\"rangee\" style=\"display:flex;gap:{}mm;align-items:stretch\">",
        gap
    );
    for chunk in logical_chunks(body) {
        let t = chunk.trim();
        if t.is_empty() {
            continue;
        }
        if let Some(h) = dispatch_chunk(t, env, toc) {
            html.push_str(&format!("<div class=\"col\" style=\"flex:1 1 0\">{}</div>", h));
        } else {
            html.push_str(&format!(
                "<div class=\"col\" style=\"flex:1 1 0\"><p>{}</p></div>",
                render_inline(t, env, toc)
            ));
        }
    }
    html.push_str("</div>");
    html
}

#[derive(Clone, Copy, PartialEq)]
enum Entete {
    Aucune,
    Verticale,
    Horizontale,
}

struct Cell {
    content: String,
    colspan: usize,
    rowspan: usize,
    header: Entete,
    col: usize,
}

fn alignement_d_une_colonne(s: &str) -> (char, char) {
    let t = s.trim().to_lowercase();
    let mut v = 'm';
    let mut h = 'g';
    let mut long = false;
    for (motif, vertical) in [("en haut", 'h'), ("en bas", 'b'), ("au milieu", 'm')] {
        if t.contains(motif) {
            v = vertical;
            long = true;
        }
    }
    for (motif, horizontal) in [
        ("à gauche", 'g'),
        ("a gauche", 'g'),
        ("à droite", 'd'),
        ("a droite", 'd'),
        ("au centre", 'c'),
        ("centré", 'c'),
        ("centre", 'c'),
    ] {
        if t.contains(motif) {
            h = horizontal;
            long = true;
        }
    }
    if long {
        return (v, h);
    }
    let mut ch = t.chars();
    (ch.next().unwrap_or('m'), ch.next().unwrap_or('g'))
}

fn colspec_parse(desc: &str) -> Vec<(char, char)> {
    if let Some(i) = desc.find('[') {
        if let Some(j) = desc[i..].find(']') {
            let interieur = &desc[i + 1..i + j];
            let morceaux: Vec<&str> = if interieur.contains(';') {
                interieur.split(';').collect()
            } else {
                interieur.split(',').collect()
            };
            return morceaux
                .into_iter()
                .map(alignement_d_une_colonne)
                .collect();
        }
    }
    vec![('m', 'g')]
}

fn align_css(v: char, h: char) -> String {
    let va = match v {
        'h' => "top",
        'b' => "bottom",
        _ => "middle",
    };
    let ha = match h {
        'c' => "center",
        'd' => "right",
        _ => "left",
    };
    format!("vertical-align:{};text-align:{};", va, ha)
}

fn strip_colspec(desc: &str) -> String {
    match (desc.find('['), desc.find(']')) {
        (Some(i), Some(j)) if j > i => format!("{}{}", &desc[..i], &desc[j + 1..]),
        _ => desc.to_string(),
    }
}

fn count_before(desc: &str, key: &str) -> Option<usize> {
    let i = desc.find(key)?;
    let before = desc[..i].trim_end();
    let num: String = before.chars().rev().take_while(|c| c.is_ascii_digit()).collect();
    if num.is_empty() {
        return None;
    }
    num.chars().rev().collect::<String>().parse().ok()
}

fn parse_merge_cell(cell: &str) -> Option<(usize, usize, String)> {
    let t = cell.trim();
    if !t.starts_with('<') {
        return None;
    }
    let close = t.find('>')?;
    let desc = &t[1..close];
    let colspan = count_before(desc, "colonnes")
        .or_else(|| count_before(desc, "colonne"))
        .unwrap_or(1);
    let rowspan = count_before(desc, "lignes")
        .or_else(|| count_before(desc, "ligne"))
        .unwrap_or(1);
    if colspan == 1 && rowspan == 1 {
        return None;
    }
    let rest = t[close + 1..].trim_start();
    let content = if rest.starts_with('{') {
        take_group(rest, 0)?.0
    } else {
        rest.to_string()
    };
    Some((colspan, rowspan, content))
}

fn render_table(desc: &str, body: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let body = &sans_separateur(body);
    let spec = colspec_parse(desc);
    let ncols = spec.len();
    let dsc = strip_colspec(desc);
    let words: Vec<&str> = dsc.split_whitespace().collect();

    let pad = desc_mm(&dsc, "écart").unwrap_or(1.5);
    let bg = desc_color(&words, "fond").unwrap_or("transparent");
    let bcol = desc_color(&words, "bordure").unwrap_or("#000");
    let (head_fg, head_bg) = {
        if let Some(i) = dsc
            .find("entêtes en")
            .or_else(|| dsc.find("entête en"))
            .or_else(|| dsc.find("entetes en"))
            .or_else(|| dsc.find("entete en"))
        {
            let after = &dsc[i..];
            let after = after.split_once(" en ").map(|(_, r)| r).unwrap_or(after);
            let ws: Vec<&str> = after.split_whitespace().collect();
            let cw = clean_words(&ws);
            let fg = parse_color_at(&cw).map(|(c, _)| c).unwrap_or("inherit");
            let bgc = match after
                .find("sur un fond")
                .map(|j| j + "sur un fond".len())
                .or_else(|| after.find("sur fond").map(|j| j + "sur fond".len()))
            {
                Some(j) => {
                    let ws2: Vec<&str> = after[j..].split_whitespace().collect();
                    parse_color_at(&clean_words(&ws2)).map(|(c, _)| c).unwrap_or("#eee")
                }
                None => "#eee",
            };
            (fg, bgc)
        } else {
            ("inherit", "#00000010")
        }
    };
    let mut rows: Vec<Vec<Cell>> = Vec::new();
    let mut owner: Vec<Option<(usize, usize)>> = vec![None; ncols];
    let mut occupied: Vec<usize> = vec![0; ncols];
    let mut depth: Option<usize> = None;
    let mut seen_header = false;
    for raw in body.lines() {
        let line = raw.trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let t = line.trim();
        let has_bracket = t.contains('[');
        if !has_bracket {
            let cells: Vec<&str> = t.split('\t').map(|c| c.trim()).filter(|c| !c.is_empty()).collect();
            let parsed: Vec<(usize, usize, String)> = cells
                .iter()
                .map(|c| parse_merge_cell(c).unwrap_or((1, 1, (*c).to_string())))
                .collect();
            let explicit = parsed.iter().any(|(cs, rs, _)| *cs > 1 || *rs > 1)
                || cells.iter().any(|c| *c == ".");
            if explicit {
                let ri = rows.len();
                let mut row: Vec<Cell> = Vec::new();
                let mut col = 0usize;
                for (i, (cs, rs, content)) in parsed.iter().enumerate() {
                    if cells[i] == "." {
                        continue;
                    }
                    while col < ncols && occupied[col] > 0 {
                        col += 1;
                    }
                    if col >= ncols {
                        break;
                    }
                    let cs = (*cs).max(1).min(ncols - col);
                    let rs = (*rs).max(1);
                    row.push(Cell {
                        content: render_inline(content.trim_end_matches(':').trim(), env, toc),
                        colspan: cs,
                        rowspan: rs,
                        header: if !seen_header && cs < ncols {
                            Entete::Verticale
                        } else {
                            Entete::Aucune
                        },
                        col,
                    });
                    for k in 0..cs {
                        if col + k < ncols {
                            occupied[col + k] = rs;
                            owner[col + k] = Some((ri, row.len() - 1));
                        }
                    }
                    col += cs;
                }
                for o in occupied.iter_mut() {
                    if *o > 0 {
                        *o -= 1;
                    }
                }
                if row.iter().any(|c| c.colspan < ncols) {
                    seen_header = true;
                }
                rows.push(row);
                continue;
            }
            if cells.len() == 1 && !seen_header {
                rows.push(vec![Cell {
                    content: render_inline(cells[0], env, toc),
                    colspan: ncols,
                    rowspan: 1,
                    header: Entete::Aucune,
                    col: 0,
                }]);
                continue;
            }
            let mut row = Vec::new();
            let n = cells.len();
            for (i, c) in cells.iter().enumerate() {
                let colspan = if i + 1 == n && n < ncols { ncols - n + 1 } else { 1 };
                row.push(Cell {
                    content: render_inline(c.trim_end_matches(':').trim(), env, toc),
                    colspan,
                    rowspan: 1,
                    header: if seen_header { Entete::Aucune } else { Entete::Verticale },
                    col: i,
                });
            }
            if !seen_header {
                seen_header = true;
            }
            let ri = rows.len();
            let mut col = 0;
            for (ci, c) in row.iter().enumerate() {
                for k in 0..c.colspan {
                    if col + k < ncols {
                        owner[col + k] = Some((ri, ci));
                    }
                }
                col += c.colspan;
            }
            for o in occupied.iter_mut() {
                if *o > 0 {
                    *o -= 1;
                }
            }
            rows.push(row);
            continue;
        }
        let bi = t.find('[').unwrap();
        let head_part = &t[..bi];
        let bracket_end = t.rfind(']').unwrap_or(t.len() - 1);
        let data_part = &t[bi + 1..bracket_end];
        let headers: Vec<String> = head_part
            .split('\t')
            .map(|c| c.trim())
            .filter(|c| !c.is_empty())
            .map(|c| c.trim_end_matches(':').trim().to_string())
            .collect();
        let data: Vec<String> = split_top(data_part, ';')
            .iter()
            .map(|c| c.trim().to_string())
            .collect();
        let h = headers.len();
        let d = data.len();
        if depth.is_none() {
            depth = Some(h);
        }
        let dep = depth.unwrap_or(h).max(h);
        let missing = dep.saturating_sub(h);
        let ri = rows.len();
        for c in 0..missing.min(ncols) {
            if let Some((r, ci)) = owner[c] {
                rows[r][ci].rowspan += 1;
            }
        }
        let mut row = Vec::new();
        let mut col = missing;
        for hcell in &headers {
            row.push(Cell {
                content: render_inline(hcell, env, toc),
                colspan: 1,
                rowspan: 1,
                header: Entete::Horizontale,
                col,
            });
            owner[col.min(ncols - 1)] = Some((ri, row.len() - 1));
            col += 1;
        }
        let start_data = col;
        for (i, dc) in data.iter().enumerate() {
            let remaining = ncols.saturating_sub(start_data);
            let colspan = if i + 1 == d && d < remaining { remaining - d + 1 } else { 1 };
            row.push(Cell {
                content: render_inline(dc, env, toc),
                colspan,
                rowspan: 1,
                header: Entete::Aucune,
                col,
            });
            for k in 0..colspan {
                if col + k < ncols {
                    owner[col + k] = Some((ri, row.len() - 1));
                }
            }
            col += colspan;
        }
        rows.push(row);
    }
    let secable = est_secable(&dsc);
    let mut html = format!(
        "<table class=\"tab{}\" style=\"border-collapse:collapse;width:100%;table-layout:auto;\
         background:{};border:0.3mm solid {};\">",
        if secable { " secable" } else { "" },
        bg,
        bcol
    );
    let mut entete_posee = false;
    for row in &rows {
        let entete = secable
            && !entete_posee
            && row.len() > 1
            && row.iter().all(|c| !matches!(c.header, Entete::Aucune));
        if entete {
            entete_posee = true;
        }
        html.push_str(if entete {
            "<tr class=\"tab-entete\">"
        } else {
            "<tr>"
        });
        for c in row {
            let (va, ha) = spec.get(c.col.min(ncols - 1)).cloned().unwrap_or(('m', 'g'));
            let mut style = align_css(va, ha);
            style.push_str(&format!("padding:{}mm {}mm;", pad * 0.6, pad));
            style.push_str(&format!("border:0.3mm solid {};", bcol));
            match c.header {
                Entete::Verticale => style.push_str(&format!(
                    "color:{};background:{};font-weight:700;",
                    head_fg, head_bg
                )),
                Entete::Horizontale => style.push_str(&format!(
                    "color:{};background:{};font-style:italic;",
                    head_fg, head_bg
                )),
                Entete::Aucune => {}
            }
            let span = format!(
                "{}{}",
                if c.colspan > 1 { format!(" colspan=\"{}\"", c.colspan) } else { String::new() },
                if c.rowspan > 1 { format!(" rowspan=\"{}\"", c.rowspan) } else { String::new() }
            );
            html.push_str(&format!("<td{} style=\"{}\">{}</td>", span, style, c.content));
        }
        html.push_str("</tr>");
    }
    html.push_str("</table>");
    html
}

fn render_grid(desc: &str, body: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let gap = desc_mm(desc, "écart").unwrap_or(3.0);
    let bordered = desc.contains("bordures");
    let mut areas = String::new();
    let mut ncols = 0usize;
    let mut nrows = 0usize;
    if let Some(i) = desc.find("zones:[") {
        if let Some(j) = desc[i..].find(']') {
            let inner = &desc[i + 7..i + j];
            for part in inner.split(',') {
                let z = part.trim().trim_matches('"').trim();
                if z.is_empty() {
                    continue;
                }
                ncols = ncols.max(z.split_whitespace().count());
                nrows += 1;
                areas.push_str(&format!("'{}' ", z));
            }
        }
    }
    if ncols == 0 {
        ncols = 1;
    }
    if nrows == 0 {
        nrows = 1;
    }
    let mut html = format!(
        "<div class=\"grille\" style=\"display:grid;grid-template-areas:{};grid-template-columns:repeat({}, 1fr);grid-template-rows:repeat({}, auto);gap:{}mm\">",
        areas.trim(),
        ncols,
        nrows,
        gap
    );
    let mut rest = body.to_string();
    loop {
        let start = match rest.find('[') {
            Some(i) => i,
            None => break,
        };
        let end = match rest[start..].find(']') {
            Some(j) => start + j,
            None => break,
        };
        let spec = rest[start + 1..end].to_string();
        let after = &rest[end + 1..];
        let bi = match after.find('{') {
            Some(i) => i,
            None => break,
        };
        let (content, tail) = match take_group(after, bi) {
            Some(x) => x,
            None => break,
        };
        let (name, place) = match spec.split_once(':') {
            Some((n, p)) => (n.trim().to_string(), p.trim().to_string()),
            None => {
                let mut it = spec.split_whitespace();
                let n = it.next().unwrap_or("z").to_string();
                (n, it.collect::<Vec<_>>().join(" "))
            }
        };
        let mut style = format!("grid-area:{};padding:2mm;display:flex;flex-direction:column;", name);
        let p = place.to_lowercase();
        if p.contains("en haut") || p.contains(" h") && p.len() <= 3 || p == "hg" || p == "hc" || p == "hd" {
            style.push_str("justify-content:flex-start;");
        }
        if p.contains("en bas") || p.starts_with('b') && p.len() == 2 {
            style.push_str("justify-content:flex-end;");
        }
        if p.contains("au milieu") || p.starts_with('m') && p.len() == 2 {
            style.push_str("justify-content:center;");
        }
        if p.contains("à gauche") || p.ends_with('g') && p.len() == 2 {
            style.push_str("align-items:flex-start;text-align:left;");
        }
        if p.contains("au centre") || p.ends_with('c') && p.len() == 2 || p.contains("en mc") {
            style.push_str("align-items:center;text-align:center;");
        }
        if p.contains("à droite") || p.ends_with('d') && p.len() == 2 {
            style.push_str("align-items:flex-end;text-align:right;");
        }
        let ws: Vec<&str> = place.split_whitespace().collect();
        if let Some(c) = desc_color(&ws, "fond") {
            style.push_str(&format!("background:{};", c));
        }
        if let Some(c) = desc_color(&ws, "bordure") {
            style.push_str(&format!("border:0.3mm solid {};", c));
        } else if bordered {
            style.push_str("border:0.3mm solid #999;");
        }
        html.push_str(&format!(
            "<div style=\"{}\">{}</div>",
            style,
            render_body_indent(&content, env, toc, false)
        ));
        rest = tail;
    }
    html.push_str("</div>");
    html
}

pub(crate) fn base64(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        if chunk.len() > 1 {
            out.push(T[(n >> 6) as usize & 63] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(T[n as usize & 63] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn image_mime(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("").to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        _ => "application/octet-stream",
    }
}

const IMAGE_MAX: u64 = 8 * 1024 * 1024;

pub(crate) fn chemin_sur(rel: &str) -> bool {
    let p = std::path::Path::new(rel);
    !p.is_absolute()
        && p.components()
        .all(|c| matches!(c, std::path::Component::Normal(_)))
}

fn embed_image(rel: &str) -> Option<String> {
    if !chemin_sur(rel) {
        return None;
    }
    let base = base_dir()?;
    let path = base.join(rel);
    if std::fs::metadata(&path).ok()?.len() > IMAGE_MAX {
        return None;
    }
    let data = std::fs::read(&path).ok()?;
    Some(format!("data:{};base64,{}", image_mime(rel), base64(&data)))
}

fn avec_legende(contenu: &str, legende: Option<&str>) -> String {
    match legende {
        Some(l) => format!(
            "<figure style=\"display:inline-block;margin:0;text-align:center\">{}<figcaption style=\"font-size:0.85em;font-style:italic\">{}</figcaption></figure>",
            contenu,
            html_escape(l)
        ),
        None => contenu.to_string(),
    }
}

fn groupe_apres_mot_cle(tag: &str, mot: &str) -> Option<String> {
    let i = tag.find(mot)?;
    let a = &tag[i..];
    let bi = a.find('{')?;
    take_group(a, bi).map(|(f, _)| f)
}

fn render_image(tag: &str, after: &str) -> String {
    let width = desc_mm(tag, "largeur").unwrap_or(30.0);
    let legende = groupe_apres_mot_cle(tag, "légende");
    let folder = groupe_apres_mot_cle(tag, "dossier").unwrap_or_default();
    let file = take_group(after, 0).map(|(f, _)| f).unwrap_or_default();
    let src = if folder.is_empty() {
        file
    } else {
        format!("{}/{}", folder, file)
    };
    match embed_image(&src) {
        Some(data) => {
            let img = format!(
                "<img src=\"{}\" style=\"width:{}mm\" alt=\"{}\">",
                data,
                width,
                html_escape(&src)
            );
            avec_legende(&img, legende.as_deref())
        }
        None => {
            let bloc = format!(
                "<span class=\"img-absente\" style=\"display:inline-block;width:{}mm;border:0.3mm dashed #b00;color:#b00;font-size:8pt;padding:1mm;text-align:center\">{}</span>",
                width,
                html_escape(&src)
            );
            avec_legende(&bloc, legende.as_deref())
        }
    }
}

fn subst_in_calc_groups(s: &str, var: &str, val: &str) -> String {
    let mut out = String::new();
    let mut rest = s.to_string();
    while let Some(i) = rest.find("#{") {
        out.push_str(&rest[..i + 1]);
        let tail = rest[i + 1..].to_string();
        match take_group(&tail, 0) {
            Some((body, after)) => {
                out.push('{');
                out.push_str(&subst_word(&body, var, val));
                out.push('}');
                rest = after;
            }
            None => {
                out.push('{');
                rest = tail[1..].to_string();
            }
        }
    }
    out.push_str(&rest);
    out
}

fn subst_word(s: &str, var: &str, val: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let target: Vec<char> = var.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let fin = i + target.len();
        let avant_ok = i == 0 || !(chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        let apres_ok = fin >= chars.len() || !(chars[fin].is_alphanumeric() || chars[fin] == '_');
        if avant_ok && fin <= chars.len() && chars[i..fin] == target[..] && apres_ok {
            out.push_str(val);
            i = fin;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

fn fin_de_char(s: &str, i: usize) -> usize {
    i + s[i..].chars().next().map(char::len_utf8).unwrap_or(1)
}

fn diese(
    s: &str,
    i: usize,
    vars: &std::collections::BTreeMap<String, f64>,
    textes: &std::collections::BTreeMap<String, String>,
) -> Option<(String, usize)> {
    diese_avec(s, i, vars, textes, None, None)
}

fn diese_avec(
    s: &str,
    i: usize,
    vars: &std::collections::BTreeMap<String, f64>,
    textes: &std::collections::BTreeMap<String, String>,
    boites: Option<&crate::langage::conteneurs::Boites>,
    fns: Option<&crate::langage::fonctions::Fonctions>,
) -> Option<(String, usize)> {
    let apres = &s[i + 1..];
    if apres.starts_with('{') {
        if let Some((expr, reste)) = take_group(apres, 0) {
            let fin = i + 1 + (apres.len() - reste.len());
            let original = expr.clone();
            let expr = match (boites, fns) {
                (Some(b), Some(fs)) => {
                    let e = crate::langage::fonctions::resoudre_appels(&expr, vars, b, fs, 0);
                    if let Some(d) = e.find('\u{27E6}') {
                        let f = e[d..].find('\u{27E7}').map(|k| d + k).unwrap_or(e.len());
                        let message = &e[d + '\u{27E6}'.len_utf8()..f];
                        return Some((
                            format!(
                                "<span class=\"calcul-absent\">{}</span>",
                                echappe_html(message)
                            ),
                            fin,
                        ));
                    }
                    crate::langage::conteneurs::resoudre_lectures(&e, vars, b, true)
                }
                (Some(b), None) => {
                    crate::langage::conteneurs::resoudre_lectures(&expr, vars, b, true)
                }
                _ => expr,
            };

            if let Some(texte) =
                crate::langage::fonctions::desencadre_texte(&expr)
            {
                return Some((echappe_html(texte.trim()), fin));
            }
            if expr != original && crate::maths::calcul::eval(&expr, vars).is_none() {
                return Some((echappe_html(expr.trim()), fin));
            }
            return Some((crate::maths::calcul::eval_display(&expr, vars), fin));
        }
    }
    let fin_nom = apres
        .char_indices()
        .find(|(_, c)| !(c.is_alphanumeric() || *c == '_'))
        .map(|(j, _)| j)
        .unwrap_or(apres.len());
    let bornes: Vec<usize> = apres[..fin_nom]
        .char_indices()
        .map(|(j, c)| j + c.len_utf8())
        .collect();
    for &fin in bornes.iter().rev() {
        if let Some(t) = textes.get(&apres[..fin]) {
            return Some((echappe_html(t), i + 1 + fin));
        }
        if let Some(v) = vars.get(&apres[..fin]) {
            return Some((crate::maths::calcul::format_number(*v), i + 1 + fin));
        }
        if let Some(bmap) = boites {
            if let Some(b) = bmap.get(&apres[..fin]) {
                let suite = &apres[fin..];
                if suite.starts_with('[') {
                    if let Some(fc) = crate::langage::conteneurs::crochet_fermant(suite) {
                        let indices = &suite[1..fc];
                        let texte = match crate::langage::conteneurs::lit_index(
                            &apres[..fin], b, indices, vars, bmap,
                        ) {
                            Ok((v, t)) => {
                                echappe_html(&crate::langage::conteneurs::formate_valeur(&v, &t))
                            }
                            Err(e) => format!(
                                "{}",
                                crate::utils::erreur::en_ligne(&e)
                            ),
                        };
                        return Some((texte, i + 1 + fin + fc + 1));
                    }
                }
                return Some((
                    echappe_html(&crate::langage::conteneurs::affiche(b)),
                    i + 1 + fin,
                ));
            }
        }
    }
    None
}

pub(crate) fn interpole_diese(
    s: &str,
    vars: &std::collections::BTreeMap<String, f64>,
    textes: &std::collections::BTreeMap<String, String>,
) -> String {
    let octets = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < octets.len() {
        if octets[i] == b'#' {
            if let Some((texte, fin)) = diese(s, i, vars, textes) {
                out.push_str(&texte);
                i = fin;
                continue;
            }
        }
        let fin = fin_de_char(s, i);
        out.push_str(&s[i..fin]);
        i = fin;
    }
    out
}

pub fn render_inline(s: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let mut out = String::new();
    let octets = s.as_bytes();
    let mut i = 0;
    while i < octets.len() {
        let c = octets[i];
        if c == b'#' {
            if let Some((texte, fin)) = diese_avec(s, i, &env.vars, &env.textes, Some(&env.conteneurs), Some(&env.fonctions)) {
                out.push_str(&texte);
                i = fin;
                continue;
            }
        }
        if c == b'$' {
            if let Some(j) = s[i + 1..].find('$') {
                let inner = interpole_diese(&s[i + 1..i + 1 + j], &env.vars, &env.textes);
                out.push_str("\\(");
                out.push_str(&to_latex(&inner));
                out.push_str("\\)");
                i = i + 1 + j + 1;
                continue;
            }
        }
        if c == b'<' {
            let rest = &s[i..];
            let advance = |i: &mut usize, remaining: &str| {
                *i += rest.len() - remaining.len();
            };
            if let Some((tag, after)) = read_tag(rest) {
                let tag_t = tag.trim().to_string();
                if balise_supprimee(&tag_t) {
                    advance(&mut i, &after);
                    continue;
                }
                if tag_t == "exposant" || tag_t.starts_with("exposant ")
                    || tag_t == "indice" {
                    let a2 = after.trim_start();
                    if let Some((arg, tail)) = take_group(a2, 0) {
                        let contenu = render_inline(&arg, env, toc);
                        if tag_t == "indice" {
                            out.push_str(&format!("<sub>{}</sub>", contenu));
                        } else if let Some(mm) = desc_mm(&tag_t, "exposant") {
                            out.push_str(&format!(
                                "<span style=\"vertical-align:{}mm;font-size:0.7em\">{}</span>",
                                mm, contenu
                            ));
                        } else {
                            out.push_str(&format!("<sup>{}</sup>", contenu));
                        }
                        advance(&mut i, &tail);
                        continue;
                    }
                }
                if tag_t == "étiquette" || tag_t == "etiquette" {
                    let a2 = after.trim_start();
                    if let Some((arg, tail)) = take_group(a2, 0) {
                        if let Some((num, id)) = numero_courant(env) {
                            out.push_str(&declare_renvoi(arg.trim(), &num, &id));
                        }
                        advance(&mut i, &tail);
                        continue;
                    }
                }
                if tag_t == "renvoi" {
                    let a2 = after.trim_start();
                    if let Some((arg, tail)) = take_group(a2, 0) {
                        out.push_str(&format!("\u{E018}{}\u{E019}", arg.trim()));
                        advance(&mut i, &tail);
                        continue;
                    }
                }
                if tag_t == "cite" {
                    let a2 = after.trim_start();
                    if let Some((arg, tail)) = take_group(a2, 0) {
                        let mut premiers = true;
                        for cle in arg.split([',', ';']) {
                            let cle = cle.trim();
                            if cle.is_empty() {
                                continue;
                            }
                            if !premiers {
                                out.push_str("<span class=\"cite-sep\">,</span>");
                            }
                            premiers = false;
                            out.push_str(&format!("\u{E018}cite:{}\u{E019}", cle));
                        }
                        advance(&mut i, &tail);
                        continue;
                    }
                }
                if tag_t == "note" {
                    let a2 = after.trim_start();
                    if let Some((arg, tail)) = take_group(a2, 0) {
                        out.push_str(&format!(
                            "<sup class=\"note-ref\">{}</sup><span class=\"note-corps\" data-num=\"{}\" style=\"display:none\">{}</span>",
                            crate::MARQUE_NOTE_APPEL,
                            crate::MARQUE_NOTE_CORPS,
                            render_inline(&arg, env, toc)
                        ));
                        advance(&mut i, &tail);
                        continue;
                    }
                }
                if tag_t == "au centre" {
                    if let Some((arg, tail)) = take_group(after.trim_start(), 0) {
                        out.push_str(&format!(
                            "<span style=\"display:block;text-align:center\">{}</span>",
                            render_inline(&arg, env, toc)
                        ));
                        advance(&mut i, &tail);
                        continue;
                    }
                }
                if tag_t.starts_with("Insère") {
                    let img_after = after.trim_start();
                    if img_after.starts_with('{') {
                        if let Some((_, tail)) = take_group(img_after, 0) {
                            out.push_str(&render_image(&tag_t, img_after));
                            advance(&mut i, &tail);
                            continue;
                        }
                    }
                }
                let style_words: Option<Vec<String>> = match env.defs.get(&tag_t) {
                    Some(Def::Style(w)) => Some(w.clone()),
                    _ => {
                        let ws: Vec<String> =
                            tag_t.split_whitespace().map(|x| x.to_string()).collect();
                        let (css, _) = style_css(&ws);
                        if !css.is_empty() && known_style_words(&ws) {
                            Some(ws)
                        } else {
                            None
                        }
                    }
                };
                if let Some(w) = style_words {
                    let (css, lvl) = style_css(&w);
                    if let Some(lvl) = lvl {
                        out.push_str(&heading(env, toc, lvl, &css, after.trim()));
                        return out;
                    }
                    let a2 = after.trim_start();
                    if a2.starts_with('{') {
                        if let Some((arg, tail)) = take_group(a2, 0) {
                            out.push_str(&format!(
                                "<span style=\"{}\">{}</span>",
                                css,
                                render_inline(&arg, env, toc)
                            ));
                            advance(&mut i, &tail);
                            continue;
                        }
                    }
                    out.push_str(&format!(
                        "<span style=\"{}\">{}</span>",
                        css,
                        render_inline(after.trim_start(), env, toc)
                    ));
                    return out;
                }
                out.push_str(&html_escape(&format!("<{}>", tag_t)));
                advance(&mut i, &after);
                continue;
            }
            out.push_str("&lt;");
            i += 1;
            continue;
        }
        match c {
            b'&' => {
                out.push_str("&amp;");
                i += 1;
            }
            b'>' => {
                out.push_str("&gt;");
                i += 1;
            }
            b'\t' | b' ' => {
                let r = reglages_page();
                let mut cm = 0.0f32;
                let mut j = i;
                while j < octets.len() && (octets[j] == b'\t' || octets[j] == b' ') {
                    cm += if octets[j] == b'\t' { r.tabulation_cm } else { r.tabulation_cm / 4.0 };
                    j += 1;
                }
                if cm > r.tabulation_cm / 4.0 {
                    out.push_str(&format!(
                        "<span style=\"display:inline-block;width:{}cm\"></span>",
                        cm
                    ));
                } else {
                    out.push(' ');
                }
                i = j;
            }
            _ => {
                let fin = fin_de_char(s, i);
                out.push_str(&s[i..fin]);
                i = fin;
            }
        }
    }
    out
}

fn known_style_words(ws: &[String]) -> bool {
    let refs: Vec<&str> = ws.iter().map(|s| s.as_str()).collect();
    let mut i = 0;
    while i < refs.len() {
        if let Some((_, n)) = parse_color_at(&refs[i..]) {
            i += n;
            continue;
        }
        if refs.get(i..i + 2) == Some(&["petites", "capitales"])
            || refs.get(i..i + 2) == Some(&["sans", "empattements"])
            || refs.get(i..i + 2) == Some(&["à", "gauche"])
            || refs.get(i..i + 2) == Some(&["au", "centre"])
            || refs.get(i..i + 2) == Some(&["à", "droite"])
        {
            i += 2;
            continue;
        }
        if let Some((_, n)) = nom_police_at(&refs[i..]) {
            i += n;
            continue;
        }
        let w = refs[i];
        let ok = matches!(
            w,
            "gras" | "italique" | "souligné" | "barré" | "centre" | "gauche" | "droite"
                | "justifié" | "justifie"
                | "petit" | "grand" | "num" | "chapitre" | "section" | "sous-section"
                | "sous-sous-section"
        ) || (w.ends_with("pt")
            && w[..w.len() - 2].chars().all(|c| c.is_ascii_digit() || c == '.'));
        if !ok {
            return false;
        }
        i += 1;
    }
    !ws.is_empty()
}

pub fn toc_html(entries: &[&TocEntry]) -> String {
    let mut out = String::new();
    for e in entries {
        out.push_str(&format!(
            "<div class=\"toc-e lvl{}\"><a href=\"#{}\"><span class=\"toc-num\">{}</span> <span class=\"toc-txt\">{}</span><span class=\"toc-dots\"></span><span class=\"toc-pg\" data-target=\"{}\"></span></a></div>",
            e.level,
            e.id,
            e.num,
            html_escape(&e.title),
            e.id
        ));
    }
    out
}
