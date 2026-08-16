use crate::maths::trace::{axes, couleur, enveloppe_haute, Repere};

type P2 = (f64, f64);
type P3 = [f64; 3];

const FUITE: P2 = (0.45, 0.35);
const LARGEUR: f64 = 150.0;

fn evalue2(expr: &str, x: f64, y: f64) -> Option<f64> {
    let mut vars = std::collections::BTreeMap::new();
    vars.insert("x".to_string(), x);
    vars.insert("y".to_string(), y);
    let expr = expr.replace('π', "pi").replace('−', "-");
    crate::maths::calcul::eval(&expr, &vars).filter(|v| v.is_finite())
}

fn intervalle(desc: &str, cle: &str) -> Option<(f64, f64)> {
    let i = desc.to_lowercase().find(cle)? + cle.len();
    let dedans = desc[i..].trim_start().strip_prefix('[')?.split_once(']')?.0;
    let (a, b) = dedans.split_once(';')?;
    let lit = |s: &str| -> Option<f64> {
        crate::maths::calcul::eval(
            &s.trim().replace(',', ".").replace('π', "pi").replace('−', "-"),
            &std::collections::BTreeMap::new(),
        )
    };
    Some((lit(a)?, lit(b)?))
}

fn expression_z(desc: &str, cle: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find(cle)? + cle.len();
    let mut zone = &desc[i..];
    if let Some(j) = zone.to_lowercase().find(" pour ") {
        zone = &zone[..j];
    }
    let e = zone.trim().trim_end_matches(',').trim();
    if e.is_empty() {
        None
    } else {
        Some(e.to_string())
    }
}

fn texte_fr(v: f64) -> String {
    let a = (v * 100.0).round() / 100.0;
    let t = if (a - a.round()).abs() < 1e-9 {
        format!("{}", a.round() as i64)
    } else {
        let mut t = format!("{:.2}", a);
        while t.ends_with('0') {
            t.pop();
        }
        t
    };
    t.replace('.', ",")
}

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
        let mut s = ((LARGEUR - 26.0) / dx).min(112.0 / dy);
        if s <= 0.0 {
            s = 1.0;
        }
        let hauteur = s * dy + 24.0;
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

fn rgb(hex: &str) -> (f64, f64, f64) {
    let h = hex.trim_start_matches('#');
    let lit = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap_or(64) as f64;
    (lit(0), lit(2), lit(4))
}

fn teinte(base: (f64, f64, f64), lumiere: f64) -> String {
    let melange = |c: f64| -> u8 {
        (255.0 * (1.0 - lumiere) + c * lumiere).round().clamp(0.0, 255.0) as u8
    };
    format!("#{:02x}{:02x}{:02x}", melange(base.0), melange(base.1), melange(base.2))
}

fn assombri(base: (f64, f64, f64)) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (base.0 * 0.55) as u8,
        (base.1 * 0.55) as u8,
        (base.2 * 0.55) as u8
    )
}

fn surface(desc: &str) -> Option<String> {
    let expr = expression_z(desc, "z = ")?;
    let (x0, x1) = intervalle(desc, "x dans ")?;
    let (y0, y1) = intervalle(desc, "y dans ")?;
    let bas = desc.to_lowercase();
    let n = bas
        .split_once("avec ")
        .and_then(|(_, s)| s.split_whitespace().next())
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|_| bas.contains("mailles"))
        .unwrap_or(24)
        .clamp(6, 60);
    let mut z = vec![vec![None; n + 1]; n + 1];
    let mut zmin = f64::INFINITY;
    let mut zmax = f64::NEG_INFINITY;
    for i in 0..=n {
        for j in 0..=n {
            let x = x0 + (x1 - x0) * i as f64 / n as f64;
            let y = y0 + (y1 - y0) * j as f64 / n as f64;
            if let Some(v) = evalue2(&expr, x, y) {
                z[i][j] = Some(v);
                zmin = zmin.min(v);
                zmax = zmax.max(v);
            }
        }
    }
    if !zmin.is_finite() {
        return None;
    }
    let etendue_z = (zmax - zmin).max(1e-9);
    let (lx, ly, lz) = (4.0, 4.0, 2.8);
    let monde = |i: usize, j: usize| -> Option<P3> {
        z[i][j].map(|v| {
            [
                lx * i as f64 / n as f64,
                ly * j as f64 / n as f64,
                lz * (v - zmin) / etendue_z,
            ]
        })
    };
    let mut projetes = Vec::new();
    for i in 0..=n {
        for j in 0..=n {
            if let Some(p) = monde(i, j) {
                projetes.push(proj(p));
            }
        }
    }
    for p in [
        [lx + 1.0, 0.0, 0.0],
        [0.0, ly + 1.0, 0.0],
        [0.0, 0.0, lz + 0.9],
        [-0.5, -0.5, -0.3],
    ] {
        projetes.push(proj(p));
    }
    let v = Vue::cadre(&projetes);
    let base = rgb(couleur(desc));
    let contour = assombri(base);
    let lum = {
        let l = (-0.35f64, -0.55f64, 0.76f64);
        let norme = (l.0 * l.0 + l.1 * l.1 + l.2 * l.2).sqrt();
        (l.0 / norme, l.1 / norme, l.2 / norme)
    };
    let mut quads: Vec<(f64, [P3; 4])> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            match (monde(i, j), monde(i + 1, j), monde(i + 1, j + 1), monde(i, j + 1)) {
                (Some(a), Some(b), Some(c), Some(d)) => {
                    let prof = (a[1] + c[1]) / 2.0 + FUITE.0 * (a[0] + c[0]) / 2.0;
                    quads.push((prof, [a, b, c, d]));
                }
                _ => {}
            }
        }
    }
    quads.sort_by(|p, q| q.0.total_cmp(&p.0));
    let mut s = String::new();
    let axe = |a: P3, b: P3, nom: &str, dx: f64, dy: f64| -> String {
        let (pa, pb) = (proj(a), proj(b));
        format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"axe\" marker-end=\"url(#fleche)\"/><text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">{}</text>",
            v.px(pa.0),
            v.py(pa.1),
            v.px(pb.0),
            v.py(pb.1),
            v.px(pb.0) + dx,
            v.py(pb.1) + dy,
            nom
        )
    };
    s.push_str(&axe([-0.4, 0.0, 0.0], [lx + 1.0, 0.0, 0.0], "x", 2.0, 3.0));
    s.push_str(&axe([0.0, -0.4, 0.0], [0.0, ly + 1.0, 0.0], "y", 2.0, -1.5));
    s.push_str(&axe([0.0, 0.0, -0.3], [0.0, 0.0, lz + 0.9], "z", -4.0, 0.0));
    for (_, q) in &quads {
        let u = [q[1][0] - q[0][0], q[1][1] - q[0][1], q[1][2] - q[0][2]];
        let w = [q[3][0] - q[0][0], q[3][1] - q[0][1], q[3][2] - q[0][2]];
        let nrm = [
            u[1] * w[2] - u[2] * w[1],
            u[2] * w[0] - u[0] * w[2],
            u[0] * w[1] - u[1] * w[0],
        ];
        let long = (nrm[0] * nrm[0] + nrm[1] * nrm[1] + nrm[2] * nrm[2]).sqrt().max(1e-12);
        let t = ((nrm[0] * lum.0 + nrm[1] * lum.1 + nrm[2] * lum.2) / long).abs();
        let remplissage = teinte(base, 0.30 + 0.55 * t);
        let mut d = String::new();
        for (k, coin) in q.iter().enumerate() {
            let p = proj(*coin);
            d.push_str(&format!(
                "{}{:.2},{:.2} ",
                if k == 0 { "M" } else { "L" },
                v.px(p.0),
                v.py(p.1)
            ));
        }
        s.push_str(&format!(
            "<path d=\"{}Z\" fill=\"{}\" stroke=\"{}\" stroke-width=\"0.22\"/>",
            d, remplissage, contour
        ));
    }
    Some(enveloppe_haute(&s, couleur(desc), v.hauteur))
}

fn niveaux_liste(desc: &str, bloc: Option<&str>, zmin: f64, zmax: f64) -> Vec<f64> {
    if desc.to_lowercase().contains("aux niveaux") {
        let dedans = desc
            .split_once("aux niveaux")
            .and_then(|(_, r)| r.split_once('{'))
            .and_then(|(_, r)| r.split_once('}'))
            .map(|(d, _)| d.to_string())
            .or_else(|| bloc.map(|c| c.to_string()));
        if let Some(dedans) = dedans {
            let vals: Vec<f64> = dedans
                .split(&[';', ','][..])
                .filter_map(|m| {
                    crate::maths::calcul::eval(
                        &m.trim().replace(',', "."),
                        &std::collections::BTreeMap::new(),
                    )
                })
                .collect();
            if !vals.is_empty() {
                return vals;
            }
        }
    }
    (1..=9)
        .map(|k| zmin + (zmax - zmin) * k as f64 / 10.0)
        .collect()
}

fn niveaux(desc: &str, bloc: Option<&str>) -> Option<String> {
    let expr = expression_z(desc, "z = ")?;
    let (x0, x1) = intervalle(desc, "x dans ")?;
    let (y0, y1) = intervalle(desc, "y dans ")?;
    let n = 90usize;
    let mut grille = vec![vec![f64::NAN; n + 1]; n + 1];
    let mut zmin = f64::INFINITY;
    let mut zmax = f64::NEG_INFINITY;
    for i in 0..=n {
        for j in 0..=n {
            let x = x0 + (x1 - x0) * i as f64 / n as f64;
            let y = y0 + (y1 - y0) * j as f64 / n as f64;
            if let Some(v) = evalue2(&expr, x, y) {
                grille[i][j] = v;
                zmin = zmin.min(v);
                zmax = zmax.max(v);
            }
        }
    }
    if !zmin.is_finite() || (zmax - zmin).abs() < 1e-12 {
        return None;
    }
    let r = Repere::isotrope(x0, x1, y0, y1);
    let mut corps = axes(&r, "y");
    let pas_x = (x1 - x0) / n as f64;
    let pas_y = (y1 - y0) / n as f64;
    for c in niveaux_liste(desc, bloc, zmin, zmax) {
        let mut d = String::new();
        let mut etiquette: Option<P2> = None;
        for i in 0..n {
            for j in 0..n {
                let (v00, v10, v11, v01) =
                    (grille[i][j], grille[i + 1][j], grille[i + 1][j + 1], grille[i][j + 1]);
                if !(v00.is_finite() && v10.is_finite() && v11.is_finite() && v01.is_finite()) {
                    continue;
                }
                let gx = x0 + i as f64 * pas_x;
                let gy = y0 + j as f64 * pas_y;
                let mut coupes: Vec<P2> = Vec::new();
                let mut bord = |a: f64, b: f64, pa: P2, pb: P2| {
                    if (a - c) * (b - c) < 0.0 {
                        let t = (c - a) / (b - a);
                        coupes.push((pa.0 + t * (pb.0 - pa.0), pa.1 + t * (pb.1 - pa.1)));
                    }
                };
                bord(v00, v10, (gx, gy), (gx + pas_x, gy));
                bord(v10, v11, (gx + pas_x, gy), (gx + pas_x, gy + pas_y));
                bord(v11, v01, (gx + pas_x, gy + pas_y), (gx, gy + pas_y));
                bord(v01, v00, (gx, gy + pas_y), (gx, gy));
                for paire in coupes.chunks(2) {
                    if let [p, q] = paire {
                        d.push_str(&format!(
                            "M{:.2},{:.2} L{:.2},{:.2} ",
                            r.px(p.0),
                            r.py(p.1),
                            r.px(q.0),
                            r.py(q.1)
                        ));
                        if etiquette.is_none() {
                            etiquette = Some(*p);
                        }
                    }
                }
            }
        }
        if d.is_empty() {
            continue;
        }
        corps.push_str(&format!("<path d=\"{}\" class=\"courbe\"/>", d));
        if let Some(p) = etiquette {
            corps.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>",
                r.px(p.0) + 1.2,
                r.py(p.1) - 1.2,
                texte_fr(c)
            ));
        }
    }
    Some(enveloppe_haute(&corps, couleur(desc), r.hauteur))
}

pub fn commande(verbe: &str, desc: &str, _corps: Option<&str>, _env: &mut crate::Env) -> Option<String> {
    if verbe != "Trace" && verbe != "Représente" {
        return None;
    }
    let bas = desc.to_lowercase();
    if bas.contains("lignes de niveau") {
        return niveaux(desc, _corps);
    }
    if bas.contains("la surface") {
        return surface(desc);
    }
    None
}
