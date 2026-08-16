use crate::maths::trace::{axes, couleur, enveloppe_haute, Repere};
use crate::Env;

type P2 = (f64, f64);

fn evalue(expr: &str, lettre: &str, valeur: f64) -> Option<f64> {
    let mut vars = std::collections::BTreeMap::new();
    vars.insert(lettre.to_string(), valeur);
    let expr = expr.replace('π', "pi").replace('−', "-");
    crate::maths::calcul::eval(&expr, &vars).filter(|v| v.is_finite())
}

fn borne(s: &str) -> Option<f64> {
    let s = s.trim().replace('π', "pi").replace('−', "-").replace(',', ".");
    let s = {
        let mut out = String::new();
        let mut precedent_chiffre = false;
        for c in s.chars() {
            if c == 'p' && precedent_chiffre {
                out.push('*');
            }
            precedent_chiffre = c.is_ascii_digit() || c == '.';
            out.push(c);
        }
        out
    };
    crate::maths::calcul::eval(&s, &std::collections::BTreeMap::new())
}

fn intervalle_t(desc: &str, cle: &str) -> Option<(f64, f64)> {
    let i = desc.to_lowercase().find(cle)? + cle.len();
    let dedans = desc[i..].trim_start().strip_prefix('[')?.split_once(']')?.0;
    let (a, b) = dedans.split_once(';')?;
    Some((borne(a)?, borne(b)?))
}

fn expression_entre(desc: &str, debut: &str, fins: &[&str]) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find(debut)? + debut.len();
    let mut zone = &desc[i..];
    for f in fins {
        if let Some(j) = zone.to_lowercase().find(f) {
            zone = &zone[..j];
        }
    }
    let e = zone.trim().trim_end_matches(',').trim();
    if e.is_empty() {
        None
    } else {
        Some(e.to_string())
    }
}

fn segments_depuis(points: Vec<Option<P2>>, saut: f64) -> Vec<Vec<P2>> {
    let mut segments = Vec::new();
    let mut courant: Vec<P2> = Vec::new();
    for p in points {
        match p {
            Some(q) => {
                if let Some(&d) = courant.last().as_ref() {
                    let ecart = ((q.0 - d.0).powi(2) + (q.1 - d.1).powi(2)).sqrt();
                    if ecart > saut {
                        if courant.len() > 1 {
                            segments.push(std::mem::take(&mut courant));
                        } else {
                            courant.clear();
                        }
                    }
                }
                courant.push(q);
            }
            None => {
                if courant.len() > 1 {
                    segments.push(std::mem::take(&mut courant));
                } else {
                    courant.clear();
                }
            }
        }
    }
    if courant.len() > 1 {
        segments.push(courant);
    }
    segments
}

fn cadre_isotrope(points: &[P2]) -> Option<Repere> {
    let xs: Vec<f64> = points.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = points.iter().map(|p| p.1).collect();
    let (x0, x1) = (
        xs.iter().cloned().fold(f64::INFINITY, f64::min),
        xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );
    let (y0, y1) = (
        ys.iter().cloned().fold(f64::INFINITY, f64::min),
        ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );
    if !x0.is_finite() || !y0.is_finite() {
        return None;
    }
    let mx = ((x1 - x0) * 0.12).max(0.6);
    let my = ((y1 - y0) * 0.12).max(0.6);
    Some(Repere::isotrope(x0 - mx, x1 + mx, y0 - my, y1 + my))
}

fn dessine_segments(r: &Repere, segments: &[Vec<P2>]) -> String {
    let mut s = String::new();
    for seg in segments {
        let mut d = String::new();
        for (i, p) in seg.iter().enumerate() {
            d.push_str(&format!(
                "{}{:.2},{:.2} ",
                if i == 0 { "M" } else { "L" },
                r.px(p.0),
                r.py(p.1)
            ));
        }
        s.push_str(&format!("<path d=\"{}\" class=\"courbe\"/>", d));
    }
    s
}

fn figure(segments: &[Vec<P2>], marques: &[(P2, String)], col: &str) -> Option<String> {
    let mut tous: Vec<P2> = segments.iter().flatten().cloned().collect();
    tous.extend(marques.iter().map(|(p, _)| *p));
    let r = cadre_isotrope(&tous)?;
    let mut corps = axes(&r, "y");
    corps.push_str(&dessine_segments(&r, segments));
    for (p, nom) in marques {
        corps.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"0.9\" class=\"point\"/>",
            r.px(p.0),
            r.py(p.1)
        ));
        if !nom.is_empty() {
            corps.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">{}</text>",
                r.px(p.0) + 1.8,
                r.py(p.1) - 1.8,
                nom
            ));
        }
    }
    Some(enveloppe_haute(&corps, col, r.hauteur))
}

fn parametree(desc: &str) -> Option<String> {
    let ex = expression_entre(desc, "x = ", &[" et y", " pour "])?;
    let ey = expression_entre(desc, "y = ", &[" pour ", " en abscisse", " en bleu", " en rouge", " en vert", " en noir", " en orange", " en violet"])?;
    let (t0, t1) = intervalle_t(desc, "t dans ").unwrap_or((0.0, 2.0 * std::f64::consts::PI));
    let n = 600;
    let points: Vec<Option<P2>> = (0..=n)
        .map(|i| {
            let t = t0 + (t1 - t0) * i as f64 / n as f64;
            match (evalue(&ex, "t", t), evalue(&ey, "t", t)) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            }
        })
        .collect();
    let valides: Vec<P2> = points.iter().flatten().cloned().collect();
    if valides.len() < 2 {
        return None;
    }
    let diag = {
        let r = cadre_isotrope(&valides)?;
        ((r.x1 - r.x0).powi(2) + (r.y1 - r.y0).powi(2)).sqrt()
    };
    let segments = segments_depuis(points, diag / 4.0);
    figure(&segments, &[], couleur(desc))
}

fn polaire(desc: &str) -> Option<String> {
    let er = expression_entre(desc, "r = ", &[" pour ", " en bleu", " en rouge", " en vert", " en noir", " en orange", " en violet"])?;
    let (t0, t1) = intervalle_t(desc, "t dans ").unwrap_or((0.0, 2.0 * std::f64::consts::PI));
    let n = 720;
    let points: Vec<Option<P2>> = (0..=n)
        .map(|i| {
            let t = t0 + (t1 - t0) * i as f64 / n as f64;
            evalue(&er, "t", t).map(|r| (r * t.cos(), r * t.sin()))
        })
        .collect();
    let valides: Vec<P2> = points.iter().flatten().cloned().collect();
    if valides.len() < 2 {
        return None;
    }
    let diag = {
        let r = cadre_isotrope(&valides)?;
        ((r.x1 - r.x0).powi(2) + (r.y1 - r.y0).powi(2)).sqrt()
    };
    let segments = segments_depuis(points, diag / 4.0);
    figure(&segments, &[], couleur(desc))
}

struct Conique {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

fn terme_conique(t: &str) -> Option<(f64, u8, u8)> {
    let t: String = t.chars().filter(|c| !c.is_whitespace() && *c != '*').collect();
    let octets: Vec<char> = t.chars().collect();
    let mut i = 0;
    let mut prefixe = String::new();
    while i < octets.len() && (octets[i].is_ascii_digit() || octets[i] == '.') {
        prefixe.push(octets[i]);
        i += 1;
    }
    let mut coef: f64 = if prefixe.is_empty() {
        1.0
    } else {
        prefixe.parse().ok()?
    };
    let mut dx = 0u8;
    let mut dy = 0u8;
    while i < octets.len() && (octets[i] == 'x' || octets[i] == 'y') {
        let lettre = octets[i];
        i += 1;
        let mut degre = 1u8;
        if i + 1 < octets.len() && octets[i] == '^' && octets[i + 1] == '2' {
            degre = 2;
            i += 2;
        }
        if lettre == 'x' {
            dx += degre;
        } else {
            dy += degre;
        }
    }
    if i < octets.len() && octets[i] == '/' {
        let reste: String = octets[i + 1..].iter().collect();
        let div: f64 = reste.parse().ok()?;
        if div == 0.0 {
            return None;
        }
        coef /= div;
        i = octets.len();
    }
    if i < octets.len() || dx + dy > 2 {
        return None;
    }
    Some((coef, dx, dy))
}

fn coefficients(equation: &str) -> Option<Conique> {
    let equation = equation
        .replace('²', "^2")
        .replace('−', "-")
        .replace(',', ".");
    let (gauche, droite) = equation.split_once('=')?;
    let mut q = Conique {
        a: 0.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
    };
    for (membre, signe_membre) in [(gauche, 1.0), (droite, -1.0)] {
        let mut signe = 1.0;
        let mut jeton = String::new();
        let mut jetons: Vec<(f64, String)> = Vec::new();
        for ch in membre.chars().chain(['+']) {
            if ch == '+' || ch == '-' {
                if !jeton.trim().is_empty() {
                    jetons.push((signe, jeton.trim().to_string()));
                }
                signe = if ch == '-' { -1.0 } else { 1.0 };
                jeton.clear();
            } else {
                jeton.push(ch);
            }
        }
        for (s, t) in jetons {
            let (coef, dx, dy) = terme_conique(&t)?;
            let v = signe_membre * s * coef;
            match (dx, dy) {
                (2, 0) => q.a += v,
                (1, 1) => q.b += v,
                (0, 2) => q.c += v,
                (1, 0) => q.d += v,
                (0, 1) => q.e += v,
                (0, 0) => q.f += v,
                _ => return None,
            }
        }
    }
    if q.a.abs() < 1e-12 && q.b.abs() < 1e-12 && q.c.abs() < 1e-12 {
        return None;
    }
    Some(q)
}

fn ltx(v: f64) -> String {
    let arrondi = (v * 1e4).round() / 1e4;
    let t = if (arrondi - arrondi.round()).abs() < 1e-9 {
        format!("{}", arrondi.round() as i64)
    } else {
        let mut t = format!("{:.4}", arrondi);
        while t.ends_with('0') {
            t.pop();
        }
        t
    };
    if t.contains('.') {
        t.replace('.', "{,}")
    } else {
        t
    }
}

fn couple_ltx(p: P2) -> String {
    format!("({} ; {})", ltx(p.0), ltx(p.1))
}

fn prose(lignes: &[String]) -> String {
    crate::maths::algebre::bloc_prose(lignes)
}

fn tourne(u: P2, x: f64, y: f64, centre: P2) -> P2 {
    let v = (-u.1, u.0);
    (
        centre.0 + x * u.0 + y * v.0,
        centre.1 + x * u.1 + y * v.1,
    )
}

fn conique(desc: &str, _env: &Env) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("équation")? + "équation".len();
    let equation = desc[i..].trim().trim_end_matches('.');
    let q = coefficients(equation)?;
    let (a, b, c, d, e, f) = (q.a, q.b, q.c, q.d, q.e, q.f);
    let delta = a * c - b * b / 4.0;
    let mut lignes = vec![format!(
        "L'équation \\({}\\) est du second degré, de partie quadratique \\(A = {}\\), \\(B = {}\\), \\(C = {}\\).",
        equation.trim(),
        ltx(a),
        ltx(b),
        ltx(c)
    )];
    lignes.push(format!(
        "Le discriminant de la partie quadratique vaut \\(AC - \\dfrac{{B^2}}{{4}} = {}\\).",
        ltx(delta)
    ));
    let tr = a + c;
    let ecart = ((a - c) * (a - c) + b * b).sqrt();
    let l1 = (tr + ecart) / 2.0;
    let l2 = (tr - ecart) / 2.0;
    let vecteur_propre = |l: f64| -> P2 {
        let cand = if b.abs() > 1e-12 {
            (b / 2.0, l - a)
        } else if (l - a).abs() < (l - c).abs() {
            (1.0, 0.0)
        } else {
            (0.0, 1.0)
        };
        let n = (cand.0 * cand.0 + cand.1 * cand.1).sqrt();
        (cand.0 / n, cand.1 / n)
    };
    if b.abs() > 1e-9 {
        let theta = 0.5 * (b).atan2(a - c);
        lignes.push(format!(
            "Le terme croisé \\(Bxy\\) s'élimine par une rotation d'angle \\(\\theta = \\dfrac{{1}}{{2}}\\arctan\\dfrac{{B}}{{A-C}} \\approx {}\\) rad ; dans le repère tourné, les coefficients quadratiques deviennent les valeurs propres \\(\\lambda_1 = {}\\) et \\(\\lambda_2 = {}\\).",
            ltx(theta),
            ltx(l1),
            ltx(l2)
        ));
    }
    if delta.abs() > 1e-9 {
        let den = 4.0 * a * c - b * b;
        let x0 = (b * e - 2.0 * c * d) / den;
        let y0 = (b * d - 2.0 * a * e) / den;
        let centre = (x0, y0);
        let fc = a * x0 * x0 + b * x0 * y0 + c * y0 * y0 + d * x0 + e * y0 + f;
        lignes.push(format!(
            "C'est une conique à centre. Le centre \\(\\Omega\\) annule les dérivées partielles : \\(\\Omega\\,{}\\), et l'équation réduite s'écrit \\(\\lambda_1 X^2 + \\lambda_2 Y^2 = {}\\).",
            couple_ltx(centre),
            ltx(-fc)
        ));
        let r1 = -fc / l1;
        let r2 = -fc / l2;
        if delta > 0.0 {
            if r1 <= 1e-12 || r2 <= 1e-12 {
                if fc.abs() < 1e-9 {
                    lignes.push("Le second membre est nul : la conique est réduite au point \\(\\Omega\\).".to_string());
                } else {
                    lignes.push("Le second membre est du mauvais signe : l'ensemble est vide.".to_string());
                }
                return Some(prose(&lignes));
            }
            let (ga, pa, ug) = if r1 >= r2 {
                (r1.sqrt(), r2.sqrt(), vecteur_propre(l1))
            } else {
                (r2.sqrt(), r1.sqrt(), vecteur_propre(l2))
            };
            let cf = (ga * ga - pa * pa).sqrt();
            let exc = cf / ga;
            let f1 = (centre.0 + cf * ug.0, centre.1 + cf * ug.1);
            let f2 = (centre.0 - cf * ug.0, centre.1 - cf * ug.1);
            lignes.push(format!(
                "\\(\\lambda_1\\) et \\(\\lambda_2\\) sont de même signe : c'est une ellipse, de demi-grand axe \\(a = {}\\) et de demi-petit axe \\(b = {}\\).",
                ltx(ga),
                ltx(pa)
            ));
            if cf < 1e-9 {
                lignes.push(format!(
                    "\\(a = b\\) : c'est un cercle de centre \\(\\Omega\\) et de rayon \\({}\\).",
                    ltx(ga)
                ));
            } else {
                lignes.push(format!(
                    "\\(c = \\sqrt{{a^2 - b^2}} = {}\\), l'excentricité vaut \\(e = \\dfrac{{c}}{{a}} = {}\\) et les foyers sont \\(F\\,{}\\) et \\(F'\\,{}\\).",
                    ltx(cf),
                    ltx(exc),
                    couple_ltx(f1),
                    couple_ltx(f2)
                ));
            }
            let n = 240;
            let pts: Vec<Option<P2>> = (0..=n)
                .map(|i| {
                    let t = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                    Some(tourne(ug, ga * t.cos(), pa * t.sin(), centre))
                })
                .collect();
            let segments = segments_depuis(pts, f64::INFINITY);
            let mut marques = vec![(centre, "Ω".to_string())];
            if cf >= 1e-9 {
                marques.push((f1, "F".to_string()));
                marques.push((f2, "F'".to_string()));
            }
            let dessin = figure(&segments, &marques, couleur(desc))?;
            return Some(format!("{}{}", prose(&lignes), dessin));
        }
        if r1 <= 1e-12 && r2 <= 1e-12 {
            lignes.push("Les deux rapports sont négatifs : l'équation est celle de deux droites sécantes.".to_string());
            return Some(prose(&lignes));
        }
        let (ta, tb, ug) = if r1 > 0.0 {
            (r1.sqrt(), (-r2).sqrt(), vecteur_propre(l1))
        } else {
            (r2.sqrt(), (-r1).sqrt(), vecteur_propre(l2))
        };
        if fc.abs() < 1e-9 {
            lignes.push("Le second membre est nul : l'équation est celle de deux droites sécantes.".to_string());
            return Some(prose(&lignes));
        }
        let cf = (ta * ta + tb * tb).sqrt();
        let exc = cf / ta;
        let f1 = (centre.0 + cf * ug.0, centre.1 + cf * ug.1);
        let f2 = (centre.0 - cf * ug.0, centre.1 - cf * ug.1);
        lignes.push(format!(
            "\\(\\lambda_1\\) et \\(\\lambda_2\\) sont de signes contraires : c'est une hyperbole, de demi-axe transverse \\(a = {}\\) et de demi-axe non transverse \\(b = {}\\).",
            ltx(ta),
            ltx(tb)
        ));
        lignes.push(format!(
            "\\(c = \\sqrt{{a^2 + b^2}} = {}\\), l'excentricité vaut \\(e = \\dfrac{{c}}{{a}} = {}\\), les foyers sont \\(F\\,{}\\) et \\(F'\\,{}\\), et les asymptotes ont pour pentes \\(\\pm\\dfrac{{b}}{{a}}\\) dans le repère réduit.",
            ltx(cf),
            ltx(exc),
            couple_ltx(f1),
            couple_ltx(f2)
        ));
        let n = 160;
        let borne_s = 2.4f64;
        let mut pts: Vec<Option<P2>> = Vec::new();
        for signe in [1.0, -1.0] {
            for i in 0..=n {
                let s = -borne_s + 2.0 * borne_s * i as f64 / n as f64;
                pts.push(Some(tourne(
                    ug,
                    signe * ta * s.cosh(),
                    tb * s.sinh(),
                    centre,
                )));
            }
            pts.push(None);
        }
        let segments = segments_depuis(pts, f64::INFINITY);
        let marques = vec![
            (centre, "Ω".to_string()),
            (f1, "F".to_string()),
            (f2, "F'".to_string()),
        ];
        let dessin = figure(&segments, &marques, couleur(desc))?;
        return Some(format!("{}{}", prose(&lignes), dessin));
    }
    let l = if l1.abs() > l2.abs() { l1 } else { l2 };
    let u = vecteur_propre(l);
    let w0 = (-u.1, u.0);
    let ep = d * u.0 + e * u.1;
    let mut dp = d * w0.0 + e * w0.1;
    let mut w = w0;
    lignes.push(
        "Le discriminant quadratique est nul : la conique est du genre parabole, d'axe porté par la direction propre associée à la valeur propre nulle."
            .to_string(),
    );
    if dp.abs() < 1e-9 {
        lignes.push("Le terme linéaire le long de l'axe est nul : l'équation est celle de deux droites parallèles (ou confondues, ou d'un ensemble vide).".to_string());
        return Some(prose(&lignes));
    }
    let mut deux_p = -dp / l;
    if deux_p < 0.0 {
        w = (-w0.0, -w0.1);
        dp = -dp;
        deux_p = -deux_p;
    }
    let p = deux_p / 2.0;
    let qs = -ep / (2.0 * l);
    let ss = -(f - ep * ep / (4.0 * l)) / dp;
    let sommet = (qs * u.0 + ss * w.0, qs * u.1 + ss * w.1);
    let foyer = (sommet.0 + p / 2.0 * w.0, sommet.1 + p / 2.0 * w.1);
    let pied = (sommet.0 - p / 2.0 * w.0, sommet.1 - p / 2.0 * w.1);
    let k = w.0 * pied.0 + w.1 * pied.1;
    lignes.push(format!(
        "La mise sous forme canonique donne \\(Y^2 = 2pX\\) avec \\(p = {}\\) : le sommet est \\(S\\,{}\\), le foyer \\(F\\,{}\\) et la directrice a pour équation \\({}x {} {}y = {}\\).",
        ltx(p),
        couple_ltx(sommet),
        couple_ltx(foyer),
        ltx(w.0),
        if w.1 >= 0.0 { "+" } else { "-" },
        ltx(w.1.abs()),
        ltx(k)
    ));
    let n = 240;
    let etendue = (4.0 * p).max(2.0);
    let pts: Vec<Option<P2>> = (0..=n)
        .map(|i| {
            let y = -etendue + 2.0 * etendue * i as f64 / n as f64;
            let x = y * y / (2.0 * p);
            Some((
                sommet.0 + x * w.0 + y * u.0,
                sommet.1 + x * w.1 + y * u.1,
            ))
        })
        .collect();
    let segments = segments_depuis(pts, f64::INFINITY);
    let marques = vec![(sommet, "S".to_string()), (foyer, "F".to_string())];
    let dessin = figure(&segments, &marques, couleur(desc))?;
    Some(format!("{}{}", prose(&lignes), dessin))
}

pub fn commande(verbe: &str, desc: &str, _corps: Option<&str>, env: &mut Env) -> Option<String> {
    let bas = desc.to_lowercase();
    if (verbe == "Trace" || verbe == "Représente") && bas.contains("courbe paramétrée") {
        return parametree(desc);
    }
    if (verbe == "Trace" || verbe == "Représente") && bas.contains("courbe polaire") {
        return polaire(desc);
    }

    if (verbe == "Trace" || verbe == "Représente") && bas.contains("rosace") {
        let n: u32 = desc
            .split("rosace à")
            .nth(1)?
            .trim()
            .split_whitespace()
            .next()?
            .parse()
            .ok()?;
        if n == 0 {
            return None;
        }
        let k = if n % 2 == 1 { n } else { n / 2 };
        let style = desc
            .split_once(" pétales")
            .map(|(_, r)| r.trim_start_matches(|c: char| c == ','))
            .unwrap_or("");
        let recrit = format!("la courbe polaire r = cos({}*t){}", k, style);
        return polaire(&recrit);
    }
    if verbe == "Étudie" && bas.contains("la conique") && bas.contains("équation") {
        return conique(desc, env);
    }
    None
}
