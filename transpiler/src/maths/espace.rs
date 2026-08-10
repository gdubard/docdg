use crate::langage::commandes::Obj;
use crate::Env;

type P2 = (f64, f64);
type P3 = [f64; 3];

const FUITE: P2 = (0.45, 0.35);
const LARGEUR: f64 = 150.0;
const CM: f64 = 12.5;
const BLEU: &str = "#1a4fa0";

fn proj(p: P3) -> P2 {
    (p[0] + FUITE.0 * p[1], p[2] + FUITE.1 * p[1])
}

struct Vue {
    s: f64,
    ox: f64,
    oy: f64,
    hauteur: f64,
}

impl Vue {
    fn cadre(points: &[P2]) -> Vue {
        let x0 = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        let x1 = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
        let y0 = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        let y1 = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
        let dx = (x1 - x0).max(0.5);
        let dy = (y1 - y0).max(0.5);
        let mut s = CM.min((LARGEUR - 30.0) / dx).min(116.0 / dy);
        if s <= 0.0 {
            s = 1.0;
        }
        let hauteur = s * dy + 26.0;
        Vue {
            s,
            ox: LARGEUR / 2.0 - s * (x0 + x1) / 2.0,
            oy: hauteur / 2.0 + s * (y0 + y1) / 2.0,
            hauteur,
        }
    }
    fn px(&self, x: f64) -> f64 {
        self.ox + self.s * x
    }
    fn py(&self, y: f64) -> f64 {
        self.oy - self.s * y
    }
}

fn texte_fr(v: f64) -> String {
    let t = if (v - v.round()).abs() < 1e-9 {
        format!("{}", v.round() as i64)
    } else {
        let mut t = format!("{:.2}", v);
        while t.ends_with('0') {
            t.pop();
        }
        t
    };
    t.replace('.', ",")
}

fn nombre_apres(desc: &str, cle: &str) -> Option<f64> {
    let bas = desc.to_lowercase();
    let i = bas.find(&cle.to_lowercase())? + cle.len();
    let brut: String = desc[i..]
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ',' || *c == '.')
        .collect();
    if brut.is_empty() {
        return None;
    }
    brut.replace(',', ".").parse().ok()
}

fn trait2(v: &Vue, a: P2, b: P2, couleur: &str, tirets: bool) -> String {
    format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" \
         stroke-width=\"0.45\"{}/>",
        v.px(a.0),
        v.py(a.1),
        v.px(b.0),
        v.py(b.1),
        couleur,
        if tirets { " stroke-dasharray=\"2 1.3\"" } else { "" }
    )
}

fn arete(v: &Vue, a: P3, b: P3, tirets: bool) -> String {
    trait2(v, proj(a), proj(b), BLEU, tirets)
}

fn chemin(v: &Vue, pts: &[P2], couleur: &str, tirets: bool, ferme: bool) -> String {
    let mut d = String::new();
    for (i, p) in pts.iter().enumerate() {
        d.push_str(&format!(
            "{}{:.2},{:.2} ",
            if i == 0 { "M" } else { "L" },
            v.px(p.0),
            v.py(p.1)
        ));
    }
    if ferme {
        d.push('Z');
    }
    format!(
        "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.45\"{}/>",
        d,
        couleur,
        if tirets { " stroke-dasharray=\"2 1.3\"" } else { "" }
    )
}

fn arc_horizontal(v: &Vue, centre: P3, r: f64, t0: f64, t1: f64, tirets: bool) -> String {
    let n = 48;
    let pts: Vec<P2> = (0..=n)
        .map(|i| {
            let t = t0 + (t1 - t0) * i as f64 / n as f64;
            proj([centre[0] + r * t.cos(), centre[1] + r * t.sin(), centre[2]])
        })
        .collect();
    chemin(v, &pts, BLEU, tirets, false)
}


fn cote(v: &Vue, a: P2, b: P2, longueur: f64) -> String {
    cote_a(v, a, b, longueur, 0.5)
}

fn cote_a(v: &Vue, a: P2, b: P2, longueur: f64, t: f64) -> String {
    crate::maths::geometrie::cote_ecran(v.px(a.0), v.py(a.1), v.px(b.0), v.py(b.1), longueur, t)
}

fn point_plein(v: &Vue, p: P2) -> String {
    format!(
        "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"0.8\" fill=\"{}\"/>",
        v.px(p.0),
        v.py(p.1),
        BLEU
    )
}

fn enveloppe(corps: &str, hauteur: f64) -> String {
    crate::maths::trace::enveloppe_haute(corps, BLEU, hauteur)
}

fn coins_pave(l: f64, p: f64, h: f64) -> [P3; 8] {
    [
        [0.0, 0.0, 0.0],
        [l, 0.0, 0.0],
        [l, 0.0, h],
        [0.0, 0.0, h],
        [0.0, p, 0.0],
        [l, p, 0.0],
        [l, p, h],
        [0.0, p, h],
    ]
}

fn dessin_pave(l: f64, p: f64, h: f64, cotes: &[(usize, usize, f64)]) -> String {
    let coins = coins_pave(l, p, h);
    let projetes: Vec<P2> = coins.iter().map(|c| proj(*c)).collect();
    let v = Vue::cadre(&projetes);
    let mut s = String::new();
    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
        s.push_str(&arete(&v, coins[a], coins[b], false));
    }
    for (a, b, cache) in [(4usize, 5usize, true), (5, 6, false), (6, 7, false), (7, 4, true)] {
        s.push_str(&arete(&v, coins[a], coins[b], cache));
    }
    for (i, cache) in [(0usize, true), (1, false), (2, false), (3, false)] {
        s.push_str(&arete(&v, coins[i], coins[i + 4], cache));
    }
    for (a, b, valeur) in cotes {
        s.push_str(&cote(&v, projetes[*a], projetes[*b], *valeur));
    }
    enveloppe(&s, v.hauteur)
}

fn silhouettes_cylindre() -> (f64, f64) {
    let t = FUITE.0.atan();
    (t, t + std::f64::consts::PI)
}

fn arcs_base(v: &Vue, centre: P3, r: f64, t_gauche: f64, t_droit: f64) -> String {
    let deux_pi = 2.0 * std::f64::consts::PI;
    let mut s = arc_horizontal(v, centre, r, t_gauche, t_droit, true);
    s.push_str(&arc_horizontal(v, centre, r, t_droit, t_gauche + deux_pi, false));
    s
}

fn dessin_cylindre(r: f64, h: f64) -> String {
    let (tg, td) = silhouettes_cylindre();
    let deux_pi = 2.0 * std::f64::consts::PI;
    let extremes: Vec<P2> = (0..64)
        .flat_map(|i| {
            let t = deux_pi * i as f64 / 64.0;
            [
                proj([r * t.cos(), r * t.sin(), 0.0]),
                proj([r * t.cos(), r * t.sin(), h]),
            ]
        })
        .collect();
    let v = Vue::cadre(&extremes);
    let mut s = arcs_base(&v, [0.0, 0.0, 0.0], r, tg, td);
    s.push_str(&arc_horizontal(&v, [0.0, 0.0, h], r, 0.0, deux_pi, false));
    for t in [tg, td] {
        let (x, y) = (r * t.cos(), r * t.sin());
        s.push_str(&arete(&v, [x, y, 0.0], [x, y, h], false));
    }
    s.push_str(&arete(&v, [0.0, 0.0, h], [r, 0.0, h], false));
    s.push_str(&point_plein(&v, proj([0.0, 0.0, h])));
    s.push_str(&cote(&v, proj([r, 0.0, h]), proj([0.0, 0.0, h]), r));
    s.push_str(&cote(
        &v,
        proj([r * tg.cos(), r * tg.sin(), 0.0]),
        proj([r * tg.cos(), r * tg.sin(), h]),
        h,
    ));
    enveloppe(&s, v.hauteur)
}

fn dessin_cone(r: f64, h: f64) -> String {
    let deux_pi = 2.0 * std::f64::consts::PI;
    let sommet = proj([0.0, 0.0, h]);
    let mut extremes: Vec<P2> = (0..64)
        .map(|i| {
            let t = deux_pi * i as f64 / 64.0;
            proj([r * t.cos(), r * t.sin(), 0.0])
        })
        .collect();
    extremes.push(sommet);
    let v = Vue::cadre(&extremes);
    let cercle = |t: f64| -> P2 { proj([r * t.cos(), r * t.sin(), 0.0]) };
    let derive = |t: f64| -> P2 {
        (
            -r * t.sin() + FUITE.0 * r * t.cos(),
            FUITE.1 * r * t.cos(),
        )
    };
    let g = |t: f64| -> f64 {
        let p = cercle(t);
        (p.0 - sommet.0) * derive(t).1 - (p.1 - sommet.1) * derive(t).0
    };
    let mut tangentes = Vec::new();
    let n = 720;
    for i in 0..n {
        let t0 = deux_pi * i as f64 / n as f64;
        let t1 = deux_pi * (i + 1) as f64 / n as f64;
        if g(t0) * g(t1) <= 0.0 && g(t0).abs() > 1e-12 {
            tangentes.push((t0 + t1) / 2.0);
        }
    }
    let mut s = String::new();
    if tangentes.len() >= 2 {
        let (mut ta, mut tb) = (tangentes[0], tangentes[1]);
        if !(ta..tb).contains(&std::f64::consts::FRAC_PI_2) {
            std::mem::swap(&mut ta, &mut tb);
            tb += deux_pi;
        }
        s.push_str(&arc_horizontal(&v, [0.0, 0.0, 0.0], r, ta, tb, true));
        s.push_str(&arc_horizontal(&v, [0.0, 0.0, 0.0], r, tb, ta + deux_pi, false));
        for t in [ta, tb] {
            s.push_str(&trait2(&v, cercle(t), sommet, BLEU, false));
        }
    }
    s.push_str(&trait2(&v, proj([0.0, 0.0, 0.0]), sommet, BLEU, true));
    s.push_str(&trait2(&v, proj([0.0, 0.0, 0.0]), proj([r, 0.0, 0.0]), BLEU, false));
    s.push_str(&point_plein(&v, proj([0.0, 0.0, 0.0])));
    s.push_str(&cote(&v, proj([0.0, 0.0, 0.0]), proj([r, 0.0, 0.0]), r));
    s.push_str(&cote(&v, proj([0.0, 0.0, 0.0]), sommet, h));
    enveloppe(&s, v.hauteur)
}

fn dessin_sphere(r: f64) -> String {
    let pts = [
        proj([-r, 0.0, -r]),
        proj([r, 0.0, r]),
        proj([r * (1.0 + FUITE.0), r, 0.0]),
        proj([-r * (1.0 + FUITE.0), -r, 0.0]),
    ];
    let v = Vue::cadre(&pts);
    let c = proj([0.0, 0.0, 0.0]);
    let mut s = format!(
        "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.45\"/>",
        v.px(c.0),
        v.py(c.1),
        v.s * r,
        BLEU
    );
    let pi = std::f64::consts::PI;
    s.push_str(&arc_horizontal(&v, [0.0, 0.0, 0.0], r, 0.0, pi, true));
    s.push_str(&arc_horizontal(&v, [0.0, 0.0, 0.0], r, pi, 2.0 * pi, false));
    s.push_str(&trait2(&v, c, proj([r, 0.0, 0.0]), BLEU, false));
    s.push_str(&point_plein(&v, c));
    s.push_str(&cote(&v, proj([r, 0.0, 0.0]), c, r));
    enveloppe(&s, v.hauteur)
}

fn dessin_pyramide(base: f64, h: f64) -> String {
    let sol = [
        [0.0, 0.0, 0.0],
        [base, 0.0, 0.0],
        [base, base, 0.0],
        [0.0, base, 0.0],
    ];
    let sommet = [base / 2.0, base / 2.0, h];
    let mut projetes: Vec<P2> = sol.iter().map(|c| proj(*c)).collect();
    projetes.push(proj(sommet));
    let v = Vue::cadre(&projetes);
    let mut s = String::new();
    for (i, cache) in [(0usize, false), (1, false), (2, true), (3, true)] {
        s.push_str(&arete(&v, sol[i], sol[(i + 1) % 4], cache));
    }
    for (i, coin) in sol.iter().enumerate() {
        s.push_str(&arete(&v, *coin, sommet, i == 3));
    }
    s.push_str(&cote(&v, projetes[0], projetes[1], base));
    enveloppe(&s, v.hauteur)
}

fn dessin_prisme(base: f64, longueur: f64) -> String {
    let h = base * 3f64.sqrt() / 2.0;
    let avant = [[0.0, 0.0, 0.0], [base, 0.0, 0.0], [base / 2.0, 0.0, h]];
    let arriere = [
        [0.0, longueur, 0.0],
        [base, longueur, 0.0],
        [base / 2.0, longueur, h],
    ];
    let projetes: Vec<P2> = avant.iter().chain(arriere.iter()).map(|c| proj(*c)).collect();
    let v = Vue::cadre(&projetes);
    let mut s = String::new();
    for i in 0..3 {
        s.push_str(&arete(&v, avant[i], avant[(i + 1) % 3], false));
    }
    for (i, cache) in [(0usize, true), (1, false), (2, false)] {
        s.push_str(&arete(&v, arriere[i], arriere[(i + 1) % 3], cache));
    }
    for (i, cache) in [(0usize, true), (1, false), (2, false)] {
        s.push_str(&arete(&v, avant[i], arriere[i], cache));
    }
    s.push_str(&cote(&v, projetes[0], projetes[1], base));
    s.push_str(&cote(&v, projetes[1], projetes[4], longueur));
    enveloppe(&s, v.hauteur)
}

fn solide(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    if !bas.contains("le solide") {
        return None;
    }
    if bas.contains("cube") {
        let a = nombre_apres(desc, "d'arête ")?;
        return Some(dessin_pave(a, a, a, &[(0, 1, a)]));
    }
    if bas.contains("pavé") {
        let l = nombre_apres(desc, "de longueur ")?;
        let p = nombre_apres(desc, "de largeur ")?;
        let h = nombre_apres(desc, "de hauteur ")?;
        return Some(dessin_pave(l, p, h, &[(0, 1, l), (1, 5, p), (1, 2, h)]));
    }
    if bas.contains("pyramide") {
        let base = nombre_apres(desc, "de base ")?;
        let h = nombre_apres(desc, "de hauteur ")?;
        return Some(dessin_pyramide(base, h));
    }
    if bas.contains("prisme") {
        let base = nombre_apres(desc, "de base ")?;
        let longueur = nombre_apres(desc, "de longueur ")?;
        return Some(dessin_prisme(base, longueur));
    }
    if bas.contains("cylindre") {
        let r = nombre_apres(desc, "de rayon ")?;
        let h = nombre_apres(desc, "de hauteur ")?;
        return Some(dessin_cylindre(r, h));
    }
    if bas.contains("cône") {
        let r = nombre_apres(desc, "de rayon ")?;
        let h = nombre_apres(desc, "de hauteur ")?;
        return Some(dessin_cone(r, h));
    }
    if bas.contains("sphère") || bas.contains("boule") {
        let r = nombre_apres(desc, "de rayon ")?;
        return Some(dessin_sphere(r));
    }
    None
}

fn rect(v: &Vue, x: f64, y: f64, l: f64, h: f64) -> String {
    chemin(
        v,
        &[(x, y), (x + l, y), (x + l, y + h), (x, y + h)],
        BLEU,
        false,
        true,
    )
}

fn patron_pave(l: f64, p: f64, h: f64) -> String {
    let coins = [
        (0.0, 0.0),
        (2.0 * l + 2.0 * p, h),
        (p, h + l),
        (p, -l),
    ];
    let v = Vue::cadre(&coins);
    let mut s = String::new();
    let mut x = 0.0;
    for largeur in [p, l, p, l] {
        s.push_str(&rect(&v, x, 0.0, largeur, h));
        x += largeur;
    }
    s.push_str(&rect(&v, p, h, l, p));
    s.push_str(&rect(&v, p, -p, l, p));
    s.push_str(&cote(&v, (p, -p), (p + l, -p), l));
    s.push_str(&cote(&v, (0.0, 0.0), (p, 0.0), p));
    s.push_str(&cote(&v, (0.0, h), (0.0, 0.0), h));
    enveloppe(&s, v.hauteur)
}

fn patron_cylindre(r: f64, h: f64) -> String {
    let l = 2.0 * std::f64::consts::PI * r;
    let coins = [(0.0, -2.0 * r), (l, h + 2.0 * r)];
    let v = Vue::cadre(&coins);
    let mut s = rect(&v, 0.0, 0.0, l, h);
    for cy in [h + r, -r] {
        s.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.45\"/>",
            v.px(l / 2.0),
            v.py(cy),
            v.s * r,
            BLEU
        ));
    }
    s.push_str(&cote_a(&v, (0.0, 0.0), (l, 0.0), l, 0.18));
    s.push_str(&cote(&v, (0.0, h), (0.0, 0.0), h));
    s.push_str(&trait2(&v, (l / 2.0, h + r), (l / 2.0 + r, h + r), BLEU, false));
    s.push_str(&cote(&v, (l / 2.0 + r, h + r), (l / 2.0, h + r), r));
    enveloppe(&s, v.hauteur)
}

fn patron_cone(r: f64, h: f64) -> String {
    let g = (r * r + h * h).sqrt();
    let alpha = 2.0 * std::f64::consts::PI * r / g;
    let depart = -std::f64::consts::FRAC_PI_2 - alpha / 2.0;
    let sommet = (0.0, 0.0);
    let n = 48;
    let arc: Vec<P2> = (0..=n)
        .map(|i| {
            let t = depart + alpha * i as f64 / n as f64;
            (g * t.cos(), g * t.sin())
        })
        .collect();
    let bas_arc = (0.0, -g);
    let centre_disque = (0.0, -g - r);
    let mut coins = arc.clone();
    coins.push(sommet);
    coins.push((centre_disque.0, centre_disque.1 - r));
    coins.push((centre_disque.0 - r, centre_disque.1));
    coins.push((centre_disque.0 + r, centre_disque.1));
    let v = Vue::cadre(&coins);
    let mut s = chemin(&v, &arc, BLEU, false, false);
    s.push_str(&trait2(&v, sommet, arc[0], BLEU, false));
    s.push_str(&trait2(&v, sommet, arc[n], BLEU, false));
    s.push_str(&format!(
        "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.45\"/>",
        v.px(centre_disque.0),
        v.py(centre_disque.1),
        v.s * r,
        BLEU
    ));
    s.push_str(&trait2(&v, sommet, bas_arc, BLEU, true));
    s.push_str(&cote(&v, sommet, bas_arc, g));
    s.push_str(&trait2(&v, centre_disque, (centre_disque.0 + r, centre_disque.1), BLEU, false));
    s.push_str(&cote(&v, (centre_disque.0 + r, centre_disque.1), centre_disque, r));
    enveloppe(&s, v.hauteur)
}

fn patron_pyramide(base: f64, h: f64) -> String {
    let ap = (h * h + base * base / 4.0).sqrt();
    let m = base / 2.0;
    let coins = [
        (-m - ap, -m - ap),
        (m + ap, m + ap),
    ];
    let v = Vue::cadre(&coins);
    let mut s = rect(&v, -m, -m, base, base);
    let triangles: [(P2, P2, P2); 4] = [
        ((-m, m), (m, m), (0.0, m + ap)),
        ((-m, -m), (m, -m), (0.0, -m - ap)),
        ((-m, -m), (-m, m), (-m - ap, 0.0)),
        ((m, -m), (m, m), (m + ap, 0.0)),
    ];
    for (a, b, c) in triangles {
        s.push_str(&trait2(&v, a, c, BLEU, false));
        s.push_str(&trait2(&v, b, c, BLEU, false));
    }
    s.push_str(&cote(&v, (m, -m), (-m, -m), base));
    s.push_str(&trait2(&v, (0.0, m), (0.0, m + ap), BLEU, true));
    s.push_str(&cote(&v, (0.0, m), (0.0, m + ap), ap));
    enveloppe(&s, v.hauteur)
}

fn patron(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    if !bas.contains("le patron") {
        return None;
    }
    if bas.contains("cube") {
        let a = nombre_apres(desc, "d'arête ")?;
        return Some(patron_pave(a, a, a));
    }
    if bas.contains("pavé") {
        let l = nombre_apres(desc, "de longueur ")?;
        let p = nombre_apres(desc, "de largeur ")?;
        let h = nombre_apres(desc, "de hauteur ")?;
        return Some(patron_pave(l, p, h));
    }
    if bas.contains("cylindre") {
        let r = nombre_apres(desc, "de rayon ")?;
        let h = nombre_apres(desc, "de hauteur ")?;
        return Some(patron_cylindre(r, h));
    }
    if bas.contains("cône") {
        let r = nombre_apres(desc, "de rayon ")?;
        let h = nombre_apres(desc, "de hauteur ")?;
        return Some(patron_cone(r, h));
    }
    if bas.contains("pyramide") {
        let base = nombre_apres(desc, "de base ")?;
        let h = nombre_apres(desc, "de hauteur ")?;
        return Some(patron_pyramide(base, h));
    }
    None
}

fn coordonnees_3d(nom: &str, env: &Env) -> Option<P3> {
    let coords = match env.objects.get(nom) {
        Some(Obj::Point { coords }) | Some(Obj::Vecteur { coords }) => coords,
        _ => return None,
    };
    if coords.len() < 3 {
        return None;
    }
    let vide = std::collections::BTreeMap::new();
    let mut p = [0.0; 3];
    for i in 0..3 {
        p[i] = crate::maths::calcul::eval(&coords[i].replace(',', "."), &vide)?;
    }
    Some(p)
}

fn noms_apres(desc: &str, cle: &str) -> Vec<String> {
    let bas = desc.to_lowercase();
    let i = match bas.find(cle) {
        Some(i) => i + cle.len(),
        None => return Vec::new(),
    };
    let arrets = [
        "le segment",
        "les segments",
        "le vecteur",
        "les vecteurs",
        "la droite",
        "les droites",
    ];
    let mut zone = &desc[i..];
    for a in arrets {
        if let Some(j) = zone.to_lowercase().find(a) {
            zone = &zone[..j];
        }
    }
    zone.split(&[',', ';'][..])
        .flat_map(|m| m.split(" et "))
        .map(|m| m.trim().trim_end_matches('.').to_string())
        .filter(|m| !m.is_empty() && m.chars().all(|c| c.is_alphabetic() || c == '\''))
        .collect()
}

fn segments_cites(desc: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut reste = desc;
    while let Some(i) = reste.find('[') {
        let fin = match reste[i..].find(']') {
            Some(j) => i + j,
            None => break,
        };
        let noms: Vec<char> = reste[i + 1..fin].chars().filter(|c| c.is_alphabetic()).collect();
        if noms.len() == 2 {
            out.push((noms[0].to_string(), noms[1].to_string()));
        }
        reste = &reste[fin + 1..];
    }
    out
}

fn vecteurs_cites(desc: &str) -> Vec<(String, String)> {
    let bas = desc.to_lowercase();
    let mut out = Vec::new();
    let mut depuis = 0;
    while let Some(i) = bas[depuis..].find("vecteur") {
        let apres = depuis + i + "vecteur".len();
        let zone = desc[apres..].trim_start_matches('s').trim_start();
        for morceau in zone.split(&[',', ';'][..]).flat_map(|m| m.split(" et ")) {
            let nom: String = morceau
                .trim()
                .chars()
                .take_while(|c| c.is_alphabetic())
                .collect();
            if nom.len() == 2 && nom.chars().all(|c| c.is_uppercase()) {
                let mut lettres = nom.chars();
                out.push((
                    lettres.next().unwrap().to_string(),
                    lettres.next().unwrap().to_string(),
                ));
            } else {
                break;
            }
        }
        depuis = apres;
    }
    out
}

fn droites_citees(desc: &str, env: &Env) -> Vec<(String, P3, P3)> {
    let mut noms = Vec::new();
    if let Some((g, d)) = deux_noms_apres(desc, "droites ") {
        noms.push(g);
        noms.push(d);
    } else if let Some(n) = nom_apres_mot(desc, "droite ") {
        noms.push(n);
    }
    noms.into_iter()
        .filter_map(|n| droite_de(&n, env).map(|(a, u)| (n, a, u)))
        .collect()
}

fn repere_espace(desc: &str, env: &Env) -> Option<String> {
    let noms = {
        let mut n = noms_apres(desc, "les points ");
        if n.is_empty() {
            n = noms_apres(desc, "le point ");
        }
        n
    };
    let points: Vec<(String, P3)> = noms
        .iter()
        .filter_map(|n| coordonnees_3d(n, env).map(|p| (n.clone(), p)))
        .collect();
    let droites = droites_citees(desc, env);
    if points.is_empty() && droites.is_empty() {
        return None;
    }
    let mut xm: f64 = 3.0;
    let mut ym: f64 = 3.0;
    let mut zm: f64 = 3.0;
    for (_, p) in &points {
        xm = xm.max(p[0] + 1.0);
        ym = ym.max(p[1] + 1.0);
        zm = zm.max(p[2] + 1.0);
    }
    for (_, a, _) in &droites {
        xm = xm.max(a[0] + 1.0);
        ym = ym.max(a[1] + 1.0);
        zm = zm.max(a[2] + 1.0);
    }
    let bornes = [
        proj([xm + 0.6, 0.0, 0.0]),
        proj([0.0, ym + 0.6, 0.0]),
        proj([0.0, 0.0, zm + 0.6]),
        proj([-0.8, -1.2, -0.8]),
    ];
    let mut tous: Vec<P2> = bornes.to_vec();
    for (_, p) in &points {
        tous.push(proj(*p));
    }
    let v = Vue::cadre(&tous);
    let axe = |a: P3, b: P3, nom: &str, dx: f64, dy: f64| -> String {
        let mut s = format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"axe\" marker-end=\"url(#fleche)\"/>",
            v.px(proj(a).0),
            v.py(proj(a).1),
            v.px(proj(b).0),
            v.py(proj(b).1)
        );
        s.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">{}</text>",
            v.px(proj(b).0) + dx,
            v.py(proj(b).1) + dy,
            nom
        ));
        s
    };
    let mut s = axe([-0.6, 0.0, 0.0], [xm, 0.0, 0.0], "x", 2.0, 3.0);
    s.push_str(&axe([0.0, -1.0, 0.0], [0.0, ym, 0.0], "y", 2.0, -1.5));
    s.push_str(&axe([0.0, 0.0, -0.6], [0.0, 0.0, zm], "z", -4.0, 0.0));
    s.push_str(&format!(
        "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab droite\">O</text>",
        v.px(proj([0.0, 0.0, 0.0]).0) - 1.5,
        v.py(proj([0.0, 0.0, 0.0]).1) + 4.0
    ));
    for u in 1..=(xm as i64) {
        let p = proj([u as f64, 0.0, 0.0]);
        s.push_str(&format!(
            "<line x1=\"{0:.2}\" y1=\"{1:.2}\" x2=\"{0:.2}\" y2=\"{2:.2}\" class=\"grad\"/>",
            v.px(p.0),
            v.py(p.1) - 1.1,
            v.py(p.1) + 1.1
        ));
    }
    for u in 1..=(zm as i64) {
        let p = proj([0.0, 0.0, u as f64]);
        s.push_str(&format!(
            "<line x1=\"{1:.2}\" y1=\"{0:.2}\" x2=\"{2:.2}\" y2=\"{0:.2}\" class=\"grad\"/>",
            v.py(p.1),
            v.px(p.0) - 1.1,
            v.px(p.0) + 1.1
        ));
    }
    let mut table = std::collections::BTreeMap::new();
    for (nom, p) in &points {
        table.insert(nom.clone(), *p);
        let sol = [p[0], p[1], 0.0];
        let bord = [p[0], 0.0, 0.0];
        let ch: Vec<P2> = [[0.0, 0.0, 0.0], bord, sol, *p].iter().map(|q| proj(*q)).collect();
        s.push_str(&format!(
            "<path d=\"M{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2}\" class=\"repere\"/>",
            v.px(ch[0].0),
            v.py(ch[0].1),
            v.px(ch[1].0),
            v.py(ch[1].1),
            v.px(ch[2].0),
            v.py(ch[2].1),
            v.px(ch[3].0),
            v.py(ch[3].1)
        ));
        s.push_str(&point_plein(&v, proj(*p)));
        s.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">{}</text>",
            v.px(proj(*p).0) + 1.6,
            v.py(proj(*p).1) - 1.6,
            nom
        ));
    }
    for (a, b) in segments_cites(desc) {
        if let (Some(pa), Some(pb)) = (table.get(&a), table.get(&b)) {
            s.push_str(&trait2(&v, proj(*pa), proj(*pb), BLEU, false));
        }
    }
    for (nom, a, u) in &droites {
        let mut t0 = f64::NEG_INFINITY;
        let mut t1 = f64::INFINITY;
        let bornes_axes = [(-0.6, xm + 0.4), (-1.0, ym + 0.4), (-0.6, zm + 0.4)];
        for i in 0..3 {
            let (lo, hi) = bornes_axes[i];
            if u[i].abs() < 1e-12 {
                if a[i] < lo || a[i] > hi {
                    t0 = f64::INFINITY;
                }
            } else {
                let (ta, tb) = ((lo - a[i]) / u[i], (hi - a[i]) / u[i]);
                t0 = t0.max(ta.min(tb));
                t1 = t1.min(ta.max(tb));
            }
        }
        if t0 < t1 {
            let da = combinaison(*a, t0, *u);
            let db = combinaison(*a, t1, *u);
            s.push_str(&trait2(&v, proj(da), proj(db), "#1e7d32", false));
            let bout = combinaison(*a, t0 + 0.92 * (t1 - t0), *u);
            s.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">{}</text>",
                v.px(proj(bout).0) + 1.8,
                v.py(proj(bout).1) - 1.8,
                nom
            ));
        }
    }
    for (a, b) in vecteurs_cites(desc) {
        if let (Some(pa), Some(pb)) = (table.get(&a), table.get(&b)) {
            s.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#c0392b\" \
                 stroke-width=\"0.5\" marker-end=\"url(#pointe)\"/>",
                v.px(proj(*pa).0),
                v.py(proj(*pa).1),
                v.px(proj(*pb).0),
                v.py(proj(*pb).1)
            ));
        }
    }
    Some(enveloppe(&s, v.hauteur))
}

fn valeur_num(s: &str) -> Option<f64> {
    let vide = std::collections::BTreeMap::new();
    crate::maths::calcul::eval(&s.trim().replace(',', "."), &vide)
}

fn equation_plan(eq: &str) -> Option<([f64; 3], f64)> {
    let (gauche, droite) = eq.split_once('=')?;
    let mut n = [0.0; 3];
    let mut k = 0.0;
    for (membre, signe_membre) in [(gauche, 1.0), (droite, -1.0)] {
        let mut signe = 1.0;
        let mut jeton = String::new();
        let mut jetons: Vec<(f64, String)> = Vec::new();
        for c in membre.chars().chain(['+']) {
            if c == '+' || c == '-' {
                if !jeton.trim().is_empty() {
                    jetons.push((signe, jeton.trim().to_string()));
                }
                signe = if c == '-' { -1.0 } else { 1.0 };
                jeton.clear();
            } else {
                jeton.push(c);
            }
        }
        for (s, t) in jetons {
            let (brut, variable) = match t.chars().last() {
                Some(v @ ('x' | 'y' | 'z')) => (t[..t.len() - 1].trim().to_string(), Some(v)),
                _ => (t.clone(), None),
            };
            let coef = if brut.is_empty() {
                1.0
            } else {
                valeur_num(&brut)?
            };
            match variable {
                Some('x') => n[0] += signe_membre * s * coef,
                Some('y') => n[1] += signe_membre * s * coef,
                Some('z') => n[2] += signe_membre * s * coef,
                _ => k -= signe_membre * s * coef,
            }
        }
    }
    if n.iter().all(|c| c.abs() < 1e-12) {
        return None;
    }
    Some((n, k))
}

fn plan_de(nom: &str, env: &Env) -> Option<([f64; 3], f64)> {
    match env.objects.get(nom) {
        Some(Obj::Plan { equation }) => equation_plan(equation),
        _ => None,
    }
}

fn droite_de(nom: &str, env: &Env) -> Option<(P3, P3)> {
    match env.objects.get(nom) {
        Some(Obj::Droite { point, vecteur }) => {
            let mut a = [0.0; 3];
            let mut u = [0.0; 3];
            for i in 0..3 {
                a[i] = valeur_num(&point[i])?;
                u[i] = valeur_num(&vecteur[i])?;
            }
            Some((a, u))
        }
        _ => None,
    }
}

fn ltx(v: f64) -> String {
    let t = texte_fr(v);
    if t.contains(',') {
        t.replace(',', "{,}")
    } else {
        t
    }
}

fn triplet_ltx(p: P3) -> String {
    format!("({} ; {} ; {})", ltx(p[0]), ltx(p[1]), ltx(p[2]))
}

fn composante(lettre: char, a: f64, u: f64) -> String {
    if u.abs() < 1e-12 {
        return format!("{} = {}", lettre, ltx(a));
    }
    let facteur = if (u - 1.0).abs() < 1e-12 {
        "t".to_string()
    } else if (u + 1.0).abs() < 1e-12 {
        "-t".to_string()
    } else {
        format!("{}t", ltx(u))
    };
    if a.abs() < 1e-12 {
        format!("{} = {}", lettre, facteur)
    } else if facteur.starts_with('-') {
        format!("{} = {} - {}", lettre, ltx(a), facteur.trim_start_matches('-'))
    } else {
        format!("{} = {} + {}", lettre, ltx(a), facteur)
    }
}

fn systeme_parametrique(nom: &str, a: P3, u: P3) -> String {
    format!(
        "\\[{} :\\; \\begin{{cases}} {} \\\\ {} \\\\ {} \\end{{cases}} \\qquad t \\in \\mathbb{{R}}\\]",
        nom,
        composante('x', a[0], u[0]),
        composante('y', a[1], u[1]),
        composante('z', a[2], u[2])
    )
}

fn nom_apres_mot(desc: &str, mot: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find(mot)? + mot.len();
    let nom: String = desc[i..]
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphabetic() || *c == '\'')
        .collect();
    if nom.is_empty() {
        None
    } else {
        Some(nom)
    }
}

fn deux_noms_apres(desc: &str, mot: &str) -> Option<(String, String)> {
    let bas = desc.to_lowercase();
    let i = bas.find(mot)? + mot.len();
    let zone = desc[i..].trim_start();
    let (g, d) = zone.split_once(" et ")?;
    let prend = |m: &str| -> String {
        m.trim()
            .chars()
            .take_while(|c| c.is_alphabetic() || *c == '\'')
            .collect()
    };
    let (g, d) = (prend(g), prend(d));
    if g.is_empty() || d.is_empty() {
        None
    } else {
        Some((g, d))
    }
}

fn croix(u: P3, v: P3) -> P3 {
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}

fn scalaire(u: P3, v: P3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

fn difference(a: P3, b: P3) -> P3 {
    [b[0] - a[0], b[1] - a[1], b[2] - a[2]]
}

fn presque_nul(u: P3) -> bool {
    u.iter().all(|c| c.abs() < 1e-9)
}

fn combinaison(a: P3, t: f64, u: P3) -> P3 {
    [a[0] + t * u[0], a[1] + t * u[1], a[2] + t * u[2]]
}

fn prose(lignes: &[String]) -> Option<String> {
    Some(crate::maths::algebre::bloc_prose(lignes))
}

fn parametrique(desc: &str, env: &Env) -> Option<String> {
    let nom = nom_apres_mot(desc, "droite ")?;
    let (a, u) = droite_de(&nom, env)?;
    prose(&[
        format!(
            "La droite \\({}\\) passe par le point \\(A\\,{}\\) et a pour vecteur directeur \\(\\vec{{u}}\\,{}\\).",
            nom,
            triplet_ltx(a),
            triplet_ltx(u)
        ),
        format!(
            "Un point \\(M(x ; y ; z)\\) appartient à \\({}\\) si et seulement si \\(\\overrightarrow{{AM}} = t\\,\\vec{{u}}\\) pour un réel \\(t\\) :",
            nom
        ),
        systeme_parametrique(&nom, a, u),
    ])
}

fn droites_relatives(g: &str, d: &str, env: &Env) -> Option<String> {
    let (a, u) = droite_de(g, env)?;
    let (b, v) = droite_de(d, env)?;
    let mut lignes = vec![format!(
        "\\({}\\) passe par \\(A\\,{}\\) avec \\(\\vec{{u}}\\,{}\\) ; \\({}\\) passe par \\(B\\,{}\\) avec \\(\\vec{{v}}\\,{}\\).",
        g,
        triplet_ltx(a),
        triplet_ltx(u),
        d,
        triplet_ltx(b),
        triplet_ltx(v)
    )];
    let w = difference(a, b);
    if presque_nul(croix(u, v)) {
        lignes.push(format!(
            "Les vecteurs \\(\\vec{{u}}\\) et \\(\\vec{{v}}\\) sont colinéaires : les droites \\({}\\) et \\({}\\) sont parallèles.",
            g, d
        ));
        if presque_nul(croix(u, w)) {
            lignes.push(format!(
                "De plus \\(\\overrightarrow{{AB}}\\,{}\\) est colinéaire à \\(\\vec{{u}}\\) : les droites sont confondues.",
                triplet_ltx(w)
            ));
        } else {
            lignes.push(format!(
                "Mais \\(\\overrightarrow{{AB}}\\,{}\\) n'est pas colinéaire à \\(\\vec{{u}}\\) : les droites sont strictement parallèles.",
                triplet_ltx(w)
            ));
        }
        return prose(&lignes);
    }
    lignes.push(
        "Les vecteurs \\(\\vec{u}\\) et \\(\\vec{v}\\) ne sont pas colinéaires : les droites sont sécantes ou non coplanaires.".to_string(),
    );
    let paires = [(0usize, 1usize), (0, 2), (1, 2)];
    let mut meilleur = (0.0f64, 0usize, 0usize);
    for (i, j) in paires {
        let det = u[i] * (-v[j]) - u[j] * (-v[i]);
        if det.abs() > meilleur.0.abs() {
            meilleur = (det, i, j);
        }
    }
    let (det, i, j) = meilleur;
    if det.abs() < 1e-12 {
        return None;
    }
    let t = ((b[i] - a[i]) * (-v[j]) - (b[j] - a[j]) * (-v[i])) / det;
    let s = (u[i] * (b[j] - a[j]) - u[j] * (b[i] - a[i])) / det;
    let p1 = combinaison(a, t, u);
    let p2 = combinaison(b, s, v);
    if difference(p1, p2).iter().all(|c| c.abs() < 1e-6) {
        lignes.push(format!(
            "La résolution de \\(A + t\\,\\vec{{u}} = B + s\\,\\vec{{v}}\\) donne \\(t = {}\\) et \\(s = {}\\), et la troisième équation est vérifiée.",
            ltx(t),
            ltx(s)
        ));
        lignes.push(format!(
            "Les droites \\({}\\) et \\({}\\) sont sécantes au point \\(I\\,{}\\).",
            g,
            d,
            triplet_ltx(p1)
        ));
    } else {
        lignes.push(format!(
            "La résolution de deux équations donne \\(t = {}\\) et \\(s = {}\\), mais la troisième équation n'est pas vérifiée : le système n'a pas de solution.",
            ltx(t),
            ltx(s)
        ));
        lignes.push(format!(
            "Les droites \\({}\\) et \\({}\\) ne sont pas coplanaires.",
            g, d
        ));
    }
    prose(&lignes)
}

fn droite_et_plan(nd: &str, np: &str, env: &Env) -> Option<String> {
    let (a, u) = droite_de(nd, env)?;
    let (n, k) = plan_de(np, env)?;
    let mut lignes = vec![format!(
        "\\({}\\) passe par \\(A\\,{}\\) avec \\(\\vec{{u}}\\,{}\\) ; le plan \\({}\\) a pour vecteur normal \\(\\vec{{n}}\\,{}\\).",
        nd,
        triplet_ltx(a),
        triplet_ltx(u),
        np,
        triplet_ltx(n)
    )];
    let produit = scalaire(n, u);
    lignes.push(format!(
        "\\[\\vec{{n}} \\cdot \\vec{{u}} = {}\\]",
        ltx(produit)
    ));
    if produit.abs() < 1e-9 {
        lignes.push(format!(
            "\\(\\vec{{n}} \\cdot \\vec{{u}} = 0\\) : la droite \\({}\\) est parallèle au plan \\({}\\).",
            nd, np
        ));
        if (scalaire(n, a) - k).abs() < 1e-9 {
            lignes.push(format!(
                "De plus \\(A\\) vérifie l'équation de \\({}\\) : la droite est incluse dans le plan.",
                np
            ));
        } else {
            lignes.push(format!(
                "Mais \\(A\\) ne vérifie pas l'équation de \\({}\\) : la droite est strictement parallèle au plan.",
                np
            ));
        }
        return prose(&lignes);
    }
    let t = (k - scalaire(n, a)) / produit;
    let point = combinaison(a, t, u);
    lignes.push(format!(
        "\\(\\vec{{n}} \\cdot \\vec{{u}} \\neq 0\\) : la droite et le plan sont sécants. En reportant la représentation paramétrique dans l'équation du plan, \\(t = {}\\).",
        ltx(t)
    ));
    lignes.push(format!(
        "La droite \\({}\\) coupe le plan \\({}\\) au point \\(I\\,{}\\).",
        nd,
        np,
        triplet_ltx(point)
    ));
    prose(&lignes)
}

fn plans_relatifs(g: &str, d: &str, env: &Env) -> Option<String> {
    let (n1, k1) = plan_de(g, env)?;
    let (n2, k2) = plan_de(d, env)?;
    let mut lignes = vec![format!(
        "Le plan \\({}\\) a pour vecteur normal \\(\\vec{{n_1}}\\,{}\\), le plan \\({}\\) pour vecteur normal \\(\\vec{{n_2}}\\,{}\\).",
        g,
        triplet_ltx(n1),
        d,
        triplet_ltx(n2)
    )];
    let w = croix(n1, n2);
    if presque_nul(w) {
        lignes.push(
            "Les vecteurs normaux sont colinéaires : les plans sont parallèles.".to_string(),
        );
        let rapport = (0..3)
            .find(|i| n2[*i].abs() > 1e-12)
            .map(|i| n1[i] / n2[i])
            .unwrap_or(1.0);
        if (k1 - rapport * k2).abs() < 1e-9 {
            lignes.push("Les équations sont proportionnelles : les plans sont confondus.".to_string());
        } else {
            lignes.push(
                "Les équations ne sont pas proportionnelles : les plans sont strictement parallèles."
                    .to_string(),
            );
        }
        return prose(&lignes);
    }
    let axe = (0..3).max_by(|i, j| w[*i].abs().total_cmp(&w[*j].abs())).unwrap();
    let (i, j) = match axe {
        0 => (1, 2),
        1 => (0, 2),
        _ => (0, 1),
    };
    let det = n1[i] * n2[j] - n1[j] * n2[i];
    let mut point = [0.0; 3];
    point[i] = (k1 * n2[j] - k2 * n1[j]) / det;
    point[j] = (n1[i] * k2 - n2[i] * k1) / det;
    lignes.push(format!(
        "Les vecteurs normaux ne sont pas colinéaires : les plans \\({}\\) et \\({}\\) sont sécants selon une droite \\(\\Delta\\), dirigée par \\(\\vec{{n_1}} \\wedge \\vec{{n_2}}\\,{}\\).",
        g,
        d,
        triplet_ltx(w)
    ));
    lignes.push(format!(
        "Un point commun s'obtient en résolvant les deux équations : \\(I\\,{}\\)."
        ,
        triplet_ltx(point)
    ));
    lignes.push(systeme_parametrique("\\Delta", point, w));
    prose(&lignes)
}

fn position_relative(desc: &str, env: &Env) -> Option<String> {
    let bas = desc.to_lowercase();
    if bas.contains("des droites ") {
        let (g, d) = deux_noms_apres(desc, "droites ")?;
        return droites_relatives(&g, &d, env);
    }
    if bas.contains("des plans ") {
        let (g, d) = deux_noms_apres(desc, "plans ")?;
        return plans_relatifs(&g, &d, env);
    }
    if bas.contains("droite ") && bas.contains("plan ") {
        let nd = nom_apres_mot(desc, "droite ")?;
        let np = nom_apres_mot(desc, "plan ")?;
        return droite_et_plan(&nd, &np, env);
    }
    None
}

pub fn commande(verbe: &str, desc: &str, _corps: Option<&str>, env: &mut Env) -> Option<String> {
    let bas = desc.to_lowercase();
    if verbe == "Donne" && bas.contains("représentation paramétrique") {
        return parametrique(desc, env);
    }
    if verbe == "Étudie" && bas.contains("position relative") {
        return position_relative(desc, env);
    }
    if verbe != "Trace" {
        return None;
    }
    if bas.contains("repère de l'espace") {
        return repere_espace(desc, env);
    }
    if let Some(html) = solide(desc) {
        return Some(html);
    }
    patron(desc)
}
