use crate::Env;

const LARGEUR: f64 = 150.0;
pub(crate) const HAUTEUR: f64 = 105.0;
const MARGE: f64 = 8.0;

pub(crate) struct Repere {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub hauteur: f64,
}

impl Repere {
    pub(crate) fn etire(x0: f64, x1: f64, y0: f64, y1: f64) -> Repere {
        Repere { x0, x1, y0, y1, hauteur: HAUTEUR }
    }
    pub(crate) fn isotrope(x0: f64, x1: f64, y0: f64, y1: f64) -> Repere {
        Repere::gradue(x0, x1, y0, y1, 1.0)
    }
    pub(crate) fn gradue(x0: f64, x1: f64, y0: f64, y1: f64, rapport: f64) -> Repere {
        let unite = (LARGEUR - 2.0 * MARGE) / (x1 - x0);
        let hauteur = (2.0 * MARGE + unite * rapport * (y1 - y0)).clamp(40.0, 165.0);
        Repere { x0, x1, y0, y1, hauteur }
    }
    pub(crate) fn px(&self, x: f64) -> f64 {
        MARGE + (x - self.x0) / (self.x1 - self.x0) * (LARGEUR - 2.0 * MARGE)
    }
    pub(crate) fn py(&self, y: f64) -> f64 {
        self.hauteur - MARGE
            - (y - self.y0) / (self.y1 - self.y0) * (self.hauteur - 2.0 * MARGE)
    }
    pub(crate) fn dans(&self, x: f64, y: f64) -> bool {
        x >= self.x0 - 1e-9 && x <= self.x1 + 1e-9 && y >= self.y0 - 1e-9 && y <= self.y1 + 1e-9
    }
}

fn nombre_fr(v: f64) -> String {
    let arrondi = (v * 1000.0).round() / 1000.0;
    if (arrondi - arrondi.round()).abs() < 1e-9 {
        format!("{}", arrondi.round() as i64)
    } else {
        format!("{}", arrondi).replace('.', ",")
    }
}

pub(crate) fn axes(r: &Repere, nom_y: &str) -> String {
    let mut s = String::new();
    let zx = r.px(0.0).clamp(MARGE, LARGEUR - MARGE);
    let zy = r.py(0.0).clamp(MARGE, r.hauteur - MARGE);
    let pas_x = ((r.x1 - r.x0) / 10.0).max(0.5);
    let pas_x = if pas_x <= 0.5 { 0.5 } else { pas_x.ceil() };
    let pas_y = ((r.y1 - r.y0) / 8.0).max(0.5);
    let pas_y = if pas_y <= 0.5 { 0.5 } else { pas_y.ceil() };
    let mut k = (r.x0 / pas_x).ceil();
    while k * pas_x <= r.x1 + 1e-9 {
        let v = k * pas_x;
        k += 1.0;
        if v.abs() < 1e-9 {
            continue;
        }
        s.push_str(&graduation_x(r.px(v), zy, 1.2, 5.0, v));
    }
    let mut k = (r.y0 / pas_y).ceil();
    while k * pas_y <= r.y1 + 1e-9 {
        let v = k * pas_y;
        k += 1.0;
        if v.abs() < 1e-9 {
            continue;
        }
        let y = r.py(v);
        s.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"grad\"/>\
             <text x=\"{:.2}\" y=\"{:.2}\" class=\"lab droite\">{}</text>",
            zx - 1.2,
            y,
            zx + 1.2,
            y,
            zx - 2.0,
            y + 1.4,
            nombre_fr(v)
        ));
    }
    s.push_str(&format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"axe\" marker-end=\"url(#fleche)\"/>",
        MARGE - 4.0,
        zy,
        LARGEUR - MARGE + 4.0,
        zy
    ));
    s.push_str(&format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"axe\" marker-end=\"url(#fleche)\"/>",
        zx,
        r.hauteur - MARGE + 4.0,
        zx,
        MARGE - 4.0
    ));
    s.push_str(&format!(
        "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">x</text>",
        LARGEUR - MARGE + 5.0,
        zy - 2.0
    ));
    s.push_str(&format!(
        "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">{}</text>",
        zx + 2.5,
        MARGE - 5.0,
        nom_y
    ));
    if r.dans(0.0, 0.0) {
        s.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab droite\">O</text>",
            zx - 1.8,
            zy + 4.5
        ));
    }
    s
}

fn enveloppe(corps: &str, couleur: &str) -> String {
    enveloppe_haute(corps, couleur, HAUTEUR)
}

pub(crate) fn enveloppe_haute(corps: &str, couleur: &str, hauteur: f64) -> String {
    format!(
        "<div class=\"trace\"><svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\
         <defs><marker id=\"fleche\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"5\" \
         markerHeight=\"5\" orient=\"auto-start-reverse\"><path d=\"M0,1 L9,5 L0,9 z\" fill=\"#333\"/></marker>\
         <marker id=\"pointe\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"4\" \
         markerHeight=\"4\" orient=\"auto-start-reverse\"><path d=\"M0,1 L9,5 L0,9 z\" fill=\"{}\"/></marker>\
         </defs><style>\
         .axe{{stroke:#333;stroke-width:0.35;fill:none}}.grad{{stroke:#333;stroke-width:0.3}}\
         .lab{{font-size:3.2px;fill:#333;text-anchor:middle;font-family:serif}}\
         .droite{{text-anchor:end}}.nom{{font-size:3.6px;fill:#333;font-style:italic;font-family:serif}}\
         .courbe{{stroke:{};stroke-width:0.55;fill:none}}\
         .tangente{{stroke:#c00;stroke-width:0.4;fill:none;stroke-dasharray:1.6 1.1}}\
         .asymptote{{stroke:#777;stroke-width:0.35;fill:none;stroke-dasharray:2.2 1.4}}\
         .aire{{fill:{};fill-opacity:0.22;stroke:none}}\
         .escalier{{stroke:#c00;stroke-width:0.35;fill:none}}\
         .repere{{stroke:#c00;stroke-width:0.35;fill:none;stroke-dasharray:1.2 1}}\
         .point{{fill:{};stroke:none}}\
         .creux{{fill:#fff;stroke:{};stroke-width:0.4}}\
         </style>{}</svg></div>",
        LARGEUR, hauteur, couleur, couleur, couleur, couleur, couleur, corps
    )
}

fn couleur(desc: &str) -> &'static str {
    let bas = desc.to_lowercase();
    for (mot, code) in [
        ("en bleu", "#1a4fa0"),
        ("en rouge", "#c0392b"),
        ("en vert", "#1e7d32"),
        ("en noir", "#222"),
        ("en orange", "#d35400"),
        ("en violet", "#6c3483"),
    ] {
        if bas.contains(mot) {
            return code;
        }
    }
    "#1a4fa0"
}

fn bornes(dedans: &str) -> Option<(f64, f64)> {
    let (a, b) = dedans.split_once(';')?;
    Some((reel(a)?, reel(b)?))
}

fn intervalle_avant(desc: &str, cle: &str) -> Option<(f64, f64)> {
    let i = desc.to_lowercase().find(cle)?;
    let avant = &desc[..i];
    let ouvre = avant.rfind('[')?;
    let ferme = avant[ouvre..].find(']')? + ouvre;
    bornes(&avant[ouvre + 1..ferme])
}

fn intervalle(desc: &str, cle: &str) -> Option<(f64, f64)> {
    let i = desc.to_lowercase().find(cle)? + cle.len();
    let reste = desc[i..].trim_start();
    let dedans = reste.strip_prefix('[')?.split_once(']')?.0;
    bornes(dedans)
}

fn reel(s: &str) -> Option<f64> {
    s.trim()
        .replace(',', ".")
        .replace('−', "-")
        .trim()
        .parse::<f64>()
        .ok()
}

struct Courbe {
    segments: Vec<Vec<(f64, f64)>>,
    extremums: Vec<(f64, f64)>,
    tangentes_v: Vec<(f64, f64)>,
    asymptotes_v: Vec<f64>,
    asymptotes_d: Vec<(f64, f64)>,
    points_pleins: Vec<(f64, f64)>,
    points_creux: Vec<(f64, f64)>,
}

fn echantillonne(nom: &str, r: &Repere, n: usize, brut: bool, env: &Env) -> Option<Courbe> {
    let req = serde_json::json!({
        "op": "curve",
        "args": {"name": nom, "x0": r.x0.to_string(), "x1": r.x1.to_string(),
                 "y0": r.y0.to_string(), "y1": r.y1.to_string(),
                 "samples": n, "plain": brut},
        "defs": crate::langage::commandes::objects_json(&env.objects),
    });
    let reponse = crate::python::pont::ask(&req.to_string()).ok()?;
    let mut c = Courbe {
        segments: Vec::new(),
        extremums: Vec::new(),
        tangentes_v: Vec::new(),
        asymptotes_v: Vec::new(),
        asymptotes_d: Vec::new(),
        points_pleins: Vec::new(),
        points_creux: Vec::new(),
    };
    for ligne in reponse.lines() {
        let (cle, valeur) = match ligne.split_once('|') {
            Some(t) => t,
            None => continue,
        };
        match cle {
            "SEG" => {
                let pts: Vec<(f64, f64)> = valeur
                    .split_whitespace()
                    .filter_map(|p| p.split_once(','))
                    .filter_map(|(a, b)| Some((reel(a)?, reel(b)?)))
                    .collect();
                if pts.len() > 1 {
                    c.segments.push(pts);
                }
            }
            "EXTREMUM" | "TANGENTE_V" => {
                if let Some((a, b)) = valeur.split_once(',') {
                    if let (Some(a), Some(b)) = (reel(a), reel(b)) {
                        if cle == "EXTREMUM" {
                            c.extremums.push((a, b));
                        } else {
                            c.tangentes_v.push((a, b));
                        }
                    }
                }
            }
            "POINT_PLEIN" | "POINT_CREUX" => {
                if let Some((a, b)) = valeur.split_once(',') {
                    if let (Some(a), Some(b)) = (reel(a), reel(b)) {
                        if cle == "POINT_PLEIN" {
                            c.points_pleins.push((a, b));
                        } else {
                            c.points_creux.push((a, b));
                        }
                    }
                }
            }
            "ASYMPTOTE_V" => {
                if let Some(v) = reel(valeur) {
                    c.asymptotes_v.push(v);
                }
            }
            "ASYMPTOTE_D" => {
                if let Some((a, b)) = valeur.split_once(',') {
                    if let (Some(a), Some(b)) = (reel(a), reel(b)) {
                        c.asymptotes_d.push((a, b));
                    }
                }
            }
            _ => {}
        }
    }
    Some(c)
}

fn chemin(r: &Repere, pts: &[(f64, f64)]) -> String {
    let mut d = String::new();
    let mut ouvert = false;
    for (x, y) in pts {
        if *y < r.y0 || *y > r.y1 {
            ouvert = false;
            continue;
        }
        d.push_str(&format!(
            "{}{:.2},{:.2} ",
            if ouvert { "L" } else { "M" },
            r.px(*x),
            r.py(*y)
        ));
        ouvert = true;
    }
    d
}

fn demi_longueur(base: f64, positions: &[f64]) -> f64 {
    let mut ecart = f64::INFINITY;
    for (i, a) in positions.iter().enumerate() {
        for b in positions.iter().skip(i + 1) {
            ecart = ecart.min((a - b).abs());
        }
    }
    if ecart.is_finite() && ecart > 0.0 {
        base.min(ecart * 0.35)
    } else {
        base
    }
}

fn trace_courbe(r: &Repere, c: &Courbe, tangentes: bool) -> String {
    let mut s = String::new();
    for x in &c.asymptotes_v {
        if *x >= r.x0 && *x <= r.x1 {
            s.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"asymptote\"/>",
                r.px(*x),
                MARGE,
                r.px(*x),
                r.hauteur - MARGE
            ));
        }
    }
    for (a, b) in &c.asymptotes_d {
        let (ya, yb) = (a * r.x0 + b, a * r.x1 + b);
        if ya.min(yb) > r.y1 || ya.max(yb) < r.y0 {
            continue;
        }
        s.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"asymptote\"/>",
            r.px(r.x0),
            r.py(ya.clamp(r.y0, r.y1)),
            r.px(r.x1),
            r.py(yb.clamp(r.y0, r.y1))
        ));
    }
    if tangentes {
        let abscisses: Vec<f64> = c.extremums.iter().map(|(x, _)| *x).collect();
        let demi = demi_longueur((r.x1 - r.x0) * 0.09, &abscisses);
        for (x, y) in &c.extremums {
            if !r.dans(*x, *y) {
                continue;
            }
            s.push_str(&trait_tangente(r.px(x - demi), r.py(*y), r.px(x + demi), r.py(*y)));
        }
        let ordonnees: Vec<f64> = c.tangentes_v.iter().map(|(_, y)| *y).collect();
        let demi_y = demi_longueur((r.y1 - r.y0) * 0.09, &ordonnees);
        for (x, y) in &c.tangentes_v {
            if !r.dans(*x, *y) {
                continue;
            }
            s.push_str(&trait_tangente(r.px(*x), r.py(y - demi_y), r.px(*x), r.py(y + demi_y)));
        }
    }
    for seg in &c.segments {
        let d = chemin(r, seg);
        if !d.trim().is_empty() {
            s.push_str(&format!("<path d=\"{}\" class=\"courbe\"/>", d.trim()));
        }
    }

    for (classe, points) in [("point", &c.points_pleins), ("point creux", &c.points_creux)] {
        for (x, y) in points {
            if !r.dans(*x, *y) {
                continue;
            }
            s.push_str(&format!(
                "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"0.85\" class=\"{}\"/>",
                r.px(*x),
                r.py(*y),
                classe
            ));
        }
    }
    s
}

fn graduation_x(x: f64, zy: f64, demi: f64, decal: f64, valeur: f64) -> String {
    format!(
        "<line x1=\"{x:.2}\" y1=\"{:.2}\" x2=\"{x:.2}\" y2=\"{:.2}\" class=\"grad\"/>\
         <text x=\"{x:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>",
        zy - demi,
        zy + demi,
        zy + decal,
        nombre_fr(valeur)
    )
}

fn trait_tangente(x1: f64, y1: f64, x2: f64, y2: f64) -> String {
    format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"tangente\" \
         marker-start=\"url(#pointe)\" marker-end=\"url(#pointe)\"/>",
        x1, y1, x2, y2
    )
}

fn ajoute_points(d: &mut String, r: &Repere, points: &[(f64, f64)]) {
    for (x, y) in points {
        d.push_str(&format!("{:.2},{:.2} L", r.px(*x), r.py(y.clamp(r.y0, r.y1))));
    }
}

fn groupe_apres(desc: &str, cle: &str) -> Option<String> {
    desc.split_once(cle).and_then(|(_, r)| {
        r.trim_start()
            .strip_prefix('{')
            .and_then(|s| s.split_once('}'))
            .map(|(a, _)| a.to_string())
    })
}

fn aire(r: &Repere, haut: &Courbe, bas: Option<&Courbe>, a: f64, b: f64) -> String {
    let dans = |x: f64| x >= a - 1e-9 && x <= b + 1e-9;
    let mut dessus: Vec<(f64, f64)> = Vec::new();
    for seg in &haut.segments {
        dessus.extend(seg.iter().copied().filter(|(x, _)| dans(*x)));
    }
    if dessus.len() < 2 {
        return String::new();
    }
    let mut d = String::from("M");
    ajoute_points(&mut d, r, &dessus);
    let retour: Vec<(f64, f64)> = match bas {
        Some(c) => {
            let mut v: Vec<(f64, f64)> = Vec::new();
            for seg in &c.segments {
                v.extend(seg.iter().copied().filter(|(x, _)| dans(*x)));
            }
            v.reverse();
            v
        }
        None => dessus.iter().rev().map(|(x, _)| (*x, 0.0)).collect(),
    };
    ajoute_points(&mut d, r, &retour);
    d.truncate(d.len() - 1);
    d.push('Z');
    format!("<path d=\"{}\" class=\"aire\"/>", d)
}

fn escalier(r: &Repere, nom: &str, depart: f64, termes: usize, env: &Env) -> String {
    let req = serde_json::json!({
        "op": "iterate",
        "args": {"name": nom, "start": depart.to_string(), "count": termes},
        "defs": crate::langage::commandes::objects_json(&env.objects),
    });
    let reponse = match crate::python::pont::ask(&req.to_string()) {
        Ok(r) => r,
        Err(_) => return String::new(),
    };
    let suite: Vec<f64> = reponse
        .trim_start_matches("SUITE|")
        .split_whitespace()
        .filter_map(reel)
        .collect();
    if suite.len() < 2 {
        return String::new();
    }
    let mut s = format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"repere\"/>",
        r.px(r.x0.max(r.y0)),
        r.py(r.x0.max(r.y0)),
        r.px(r.x1.min(r.y1)),
        r.py(r.x1.min(r.y1))
    );
    let mut d = format!("M{:.2},{:.2} ", r.px(suite[0]), r.py(0.0));
    for i in 0..suite.len() - 1 {
        d.push_str(&format!("L{:.2},{:.2} ", r.px(suite[i]), r.py(suite[i + 1])));
        d.push_str(&format!(
            "L{:.2},{:.2} ",
            r.px(suite[i + 1]),
            r.py(suite[i + 1])
        ));
    }
    s.push_str(&format!("<path d=\"{}\" class=\"escalier\"/>", d.trim()));
    for (i, v) in suite.iter().take(3).enumerate() {
        s.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">u{}</text>",
            r.px(*v),
            r.py(0.0) + 5.0,
            "₀₁₂".chars().nth(i).unwrap_or('₀')
        ));
    }
    s
}

fn droite_graduee(desc: &str) -> Option<String> {
    const BANDE: f64 = 20.0;
    let (x0, x1) = intervalle(desc, "sur ")?;
    let r = Repere::etire(x0, x1, -1.0, 1.0);
    let col = couleur(desc);
    let mut s = String::new();
    let zy = BANDE / 2.0;
    let mut k = x0.ceil();
    while k <= x1 + 1e-9 {
        s.push_str(&graduation_x(r.px(k), zy, 1.5, 6.0, k));
        k += 1.0;
    }
    if let Some(spec) = groupe_apres(desc, "d'intervalle") {
        let ferme_g = spec.trim_start().starts_with('[');
        let ferme_d = spec.trim_end().ends_with(']');
        let dedans: String = spec
            .trim()
            .trim_start_matches(['[', ']', '('])
            .trim_end_matches([']', '[', ')'])
            .to_string();
        if let Some((a, b)) = dedans.split_once(',').or_else(|| dedans.split_once(';')) {
            if let (Some(a), Some(b)) = (reel(a), reel(b)) {
                s.push_str(&format!(
                    "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" \
                     stroke=\"{}\" stroke-width=\"1.2\"/>",
                    r.px(a),
                    zy,
                    r.px(b),
                    zy,
                    col
                ));
                for (v, ferme) in [(a, ferme_g), (b, ferme_d)] {
                    s.push_str(&format!(
                        "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"1.1\" stroke=\"{}\" \
                         stroke-width=\"0.4\" fill=\"{}\"/>",
                        r.px(v),
                        zy,
                        col,
                        if ferme { col } else { "#fff" }
                    ));
                }
            }
        }
    }
    if let Some(spec) = groupe_apres(desc, "de points") {
        for p in spec.split(&[',', ';'][..]).filter_map(reel) {
            s.push_str(&format!(
                "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"1.1\" fill=\"{}\"/>",
                r.px(p),
                zy,
                col
            ));
        }
    }
    s.push_str(&format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"axe\" \
         marker-end=\"url(#fleche)\"/>",
        MARGE - 4.0,
        zy,
        LARGEUR - MARGE + 4.0,
        zy
    ));
    Some(enveloppe_haute(&s, col, BANDE))
}

fn nom_apres(desc: &str, cle: &str) -> Option<String> {
    let i = desc.to_lowercase().find(cle)? + cle.len();
    let mot = desc[i..].trim_start().split(|c: char| !c.is_alphanumeric() && c != '_');
    mot.into_iter().find(|m| !m.is_empty()).map(|m| m.to_string())
}

pub fn represente(desc: &str, bloc: Option<&str>, env: &Env) -> Option<String> {
    let bas = desc.to_lowercase();
    if bas.contains("droite graduée") {
        return droite_graduee(desc);
    }
    if let Some(i) = bas.find("les fonctions ") {
        let debut = i + "les fonctions ".len();
        let fin = bas[debut..]
            .find(" pour ")
            .map(|k| debut + k)
            .unwrap_or(desc.len());
        let noms: Vec<String> = desc[debut..fin]
            .replace(" et ", ",")
            .split(',')
            .map(|m| m.trim().to_string())
            .filter(|m| !m.is_empty())
            .collect();
        if noms.is_empty() {
            return None;
        }
        let mut html = String::new();
        for nom in &noms {
            let un = format!("{}la fonction {}{}", &desc[..i], nom, &desc[fin..]);
            html.push_str(&represente(&un, bloc, env)?);
        }
        return Some(html);
    }
    let nom = nom_apres(desc, "la fonction ")?;
    let (x0, x1) = intervalle_avant(desc, "en abscisse")
        .or_else(|| intervalle(desc, " sur "))
        .or_else(|| intervalle(desc, "x dans"))?;
    let (y0, y1) = intervalle_avant(desc, "en ordonnée")
        .or_else(|| intervalle_avant(desc, "en ordonnee"))
        .or_else(|| intervalle(desc, "y dans"))
        .unwrap_or((x0, x1));
    let r = Repere::etire(x0, x1, y0, y1);
    let n = bas
        .split_once("avec ")
        .and_then(|(_, s)| s.split_whitespace().next())
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(240);
    let col = couleur(desc);
    let etudiee = env.etudiees.contains(&nom);
    let courbe = echantillonne(&nom, &r, n, !etudiee, env)?;
    let mut corps = axes(&r, &format!("{}(x)", nom));

    if bas.contains("l'aire") {
        let bornes = if let Some((_, reste)) = bas.split_once("entre ") {
            let mut it = reste.split(" et ");
            match (it.next().and_then(reel), it.next().and_then(reel)) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            }
        } else {

            let entre_accolades = desc
                .split_once("area:")
                .and_then(|(_, r)| r.trim_start().strip_prefix('{')?.split_once('}'))
                .map(|(d, _)| d.to_string())
                .or_else(|| bas.contains("area:").then(|| bloc.unwrap_or("").to_string()));
            entre_accolades
                .as_deref()
                .and_then(|d| d.split_once(&[',', ';'][..]))
                .and_then(|(a, b)| Some((reel(a)?, reel(b)?)))
        };
        let plancher = nom_apres(desc, "jusqu'à ")
            .and_then(|autre| echantillonne(&autre, &r, n, true, env));
        if let Some((a, b)) = bornes {
            corps.push_str(&aire(&r, &courbe, plancher.as_ref(), a, b));
        }
        if let Some(p) = &plancher {
            corps.push_str(&trace_courbe(&r, p, false));
        }
    }
    if bas.contains("escalier") {
        let depart = bas
            .split_once("depuis ")
            .and_then(|(_, s)| s.split_whitespace().next())
            .and_then(reel)
            .unwrap_or(0.0);
        let termes = bas
            .split_once("sur ")
            .and_then(|(_, s)| s.rsplit_once(" termes"))
            .and_then(|(v, _)| v.split_whitespace().last())
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(10);
        corps.push_str(&escalier(&r, &nom, depart, termes, env));
    }
    corps.push_str(&trace_courbe(&r, &courbe, etudiee));
    Some(enveloppe(&corps, col))
}

fn unites_graphiques(desc: &str) -> Option<f64> {
    let bas = desc.to_lowercase();
    if !bas.contains("unités graphiques") {
        return None;
    }
    let unite = |cle: &str| -> Option<f64> {
        let i = bas.find(cle)?;
        let avant = &desc[..i];
        let j = avant.to_lowercase().rfind("de ")? + "de ".len();
        avant[j..]
            .split_whitespace()
            .next()
            .and_then(reel)
    };
    let ux = unite("pour l'axe des abscisses")?;
    let uy = unite("pour l'axe des ordonnées")?;
    if ux <= 0.0 || uy <= 0.0 {
        None
    } else {
        Some(uy / ux)
    }
}

pub(crate) fn projection(r: &Repere) -> crate::maths::geometrie::Projection {
    let sx = (LARGEUR - 2.0 * MARGE) / (r.x1 - r.x0);
    let sy = (r.hauteur - 2.0 * MARGE) / (r.y1 - r.y0);
    crate::maths::geometrie::Projection {
        sx,
        sy,
        ox: MARGE - r.x0 * sx,
        oy: r.hauteur - MARGE + r.y0 * sy,
        x0: r.x0,
        x1: r.x1,
        y0: r.y0,
        y1: r.y1,
    }
}

pub fn trace(desc: &str, corps_bloc: &str, env: &Env) -> Option<String> {
    let (x0, x1) = intervalle(desc, "l'abscisse appartient à")
        .or_else(|| intervalle(desc, "x appartient à"))?;
    let (y0, y1) = intervalle(desc, "l'ordonnée à").or_else(|| intervalle(desc, "y à"))?;
    let mut noms = Vec::new();
    for ligne in corps_bloc.lines() {
        if let Some(nom) = nom_apres(ligne, "la fonction ") {
            noms.push(nom);
        }
    }
    let unites = unites_graphiques(desc);
    let sonde = Repere::etire(x0, x1, y0, y1);
    let geometrique =
        !crate::maths::geometrie::elements(corps_bloc, &projection(&sonde), env).is_empty();
    let r = match (unites, geometrique) {
        (Some(rapport), _) => Repere::gradue(x0, x1, y0, y1, rapport),
        (None, true) => Repere::isotrope(x0, x1, y0, y1),
        (None, false) => sonde,
    };
    let figures = crate::maths::geometrie::elements(corps_bloc, &projection(&r), env);
    if noms.is_empty() && figures.is_empty() {
        return None;
    }
    let mut corps = axes(&r, noms.first().map(|s| s.as_str()).unwrap_or("y"));
    for nom in &noms {
        let etudiee = env.etudiees.contains(nom);
        if let Some(c) = echantillonne(nom, &r, 400, !etudiee, env) {
            corps.push_str(&trace_courbe(&r, &c, etudiee));
        }
    }
    corps.push_str(&figures);
    Some(enveloppe_haute(&corps, couleur(desc), r.hauteur))
}

pub fn commande(verbe: &str, desc: &str, corps: Option<&str>, env: &mut Env) -> Option<String> {
    match verbe {
        "Représente" => match corps {
            Some(bloc) if desc.to_lowercase().contains("repère") => trace(desc, bloc, env),
            _ => represente(desc, corps, env),
        },
        "Trace" => trace(desc, corps?, env),
        _ => None,
    }
}
