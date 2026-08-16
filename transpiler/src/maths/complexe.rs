use crate::maths::trace::{axes, couleur, enveloppe_haute, Repere};

type P2 = (f64, f64);

#[derive(Clone, Copy, Debug)]
pub(crate) struct C {
    pub re: f64,
    pub im: f64,
}

impl C {
    fn r(v: f64) -> C {
        C { re: v, im: 0.0 }
    }
    fn plus(self, o: C) -> C {
        C {
            re: self.re + o.re,
            im: self.im + o.im,
        }
    }
    fn moins(self, o: C) -> C {
        C {
            re: self.re - o.re,
            im: self.im - o.im,
        }
    }
    fn fois(self, o: C) -> C {
        C {
            re: self.re * o.re - self.im * o.im,
            im: self.re * o.im + self.im * o.re,
        }
    }
    fn sur(self, o: C) -> Option<C> {
        let n = o.re * o.re + o.im * o.im;
        if n < 1e-300 {
            return None;
        }
        Some(C {
            re: (self.re * o.re + self.im * o.im) / n,
            im: (self.im * o.re - self.re * o.im) / n,
        })
    }
    fn module(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
    fn exp(self) -> C {
        let m = self.re.exp();
        C {
            re: m * self.im.cos(),
            im: m * self.im.sin(),
        }
    }
    fn ln(self) -> Option<C> {
        let m = self.module();
        if m < 1e-300 {
            return None;
        }
        Some(C {
            re: m.ln(),
            im: self.im.atan2(self.re),
        })
    }
    fn puissance(self, e: C) -> Option<C> {
        if e.im.abs() < 1e-12 && (e.re - e.re.round()).abs() < 1e-12 && e.re.abs() < 40.0 {
            let n = e.re.round() as i64;
            let mut acc = C::r(1.0);
            let base = if n >= 0 { self } else { C::r(1.0).sur(self)? };
            for _ in 0..n.unsigned_abs() {
                acc = acc.fois(base);
            }
            return Some(acc);
        }
        Some(e.fois(self.ln()?).exp())
    }
    fn sin(self) -> C {
        C {
            re: self.re.sin() * self.im.cosh(),
            im: self.re.cos() * self.im.sinh(),
        }
    }
    fn cos(self) -> C {
        C {
            re: self.re.cos() * self.im.cosh(),
            im: -self.re.sin() * self.im.sinh(),
        }
    }
    fn racine(self) -> C {
        let m = self.module().sqrt();
        let a = self.im.atan2(self.re) / 2.0;
        C {
            re: m * a.cos(),
            im: m * a.sin(),
        }
    }
    fn fini(self) -> bool {
        self.re.is_finite() && self.im.is_finite() && self.module() < 1e9
    }
}

#[derive(Clone, Debug, PartialEq)]
enum J {
    Nombre(f64),
    Mot(String),
    Op(char),
    Ouvre,
    Ferme,
}

fn jetons(src: &str) -> Vec<J> {
    let src = src.replace('π', "pi").replace('−', "-").replace(',', ".");
    let mut out = Vec::new();
    let lettres: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < lettres.len() {
        let c = lettres[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() || c == '.' {
            let mut t = String::new();
            while i < lettres.len() && (lettres[i].is_ascii_digit() || lettres[i] == '.') {
                t.push(lettres[i]);
                i += 1;
            }
            out.push(J::Nombre(t.parse().unwrap_or(f64::NAN)));
        } else if c.is_alphabetic() {
            let mut t = String::new();
            while i < lettres.len() && lettres[i].is_alphanumeric() {
                t.push(lettres[i]);
                i += 1;
            }
            out.push(J::Mot(t));
        } else if c == '(' {
            out.push(J::Ouvre);
            i += 1;
        } else if c == ')' {
            out.push(J::Ferme);
            i += 1;
        } else if "+-*/^".contains(c) {
            out.push(J::Op(c));
            i += 1;
        } else {
            i += 1;
        }
    }
    out
}

struct Lecteur {
    j: Vec<J>,
    i: usize,
    z: C,
}

impl Lecteur {
    fn regarde(&self) -> Option<&J> {
        self.j.get(self.i)
    }
    fn expr(&mut self) -> Option<C> {
        let mut v = self.terme()?;
        loop {
            match self.regarde() {
                Some(J::Op('+')) => {
                    self.i += 1;
                    v = v.plus(self.terme()?);
                }
                Some(J::Op('-')) => {
                    self.i += 1;
                    v = v.moins(self.terme()?);
                }
                _ => return Some(v),
            }
        }
    }
    fn terme(&mut self) -> Option<C> {
        let mut v = self.facteur()?;
        loop {
            match self.regarde() {
                Some(J::Op('*')) => {
                    self.i += 1;
                    v = v.fois(self.facteur()?);
                }
                Some(J::Op('/')) => {
                    self.i += 1;
                    let d = self.facteur()?;
                    v = v.sur(d)?;
                }
                _ => return Some(v),
            }
        }
    }
    fn facteur(&mut self) -> Option<C> {
        if let Some(J::Op('-')) = self.regarde() {
            self.i += 1;
            return Some(C::r(0.0).moins(self.facteur()?));
        }
        let base = self.atome()?;
        if let Some(J::Op('^')) = self.regarde() {
            self.i += 1;
            let e = self.facteur()?;
            return base.puissance(e);
        }
        Some(base)
    }
    fn atome(&mut self) -> Option<C> {
        match self.regarde().cloned()? {
            J::Nombre(v) => {
                self.i += 1;
                Some(C::r(v))
            }
            J::Ouvre => {
                self.i += 1;
                let v = self.expr()?;
                if self.regarde() == Some(&J::Ferme) {
                    self.i += 1;
                }
                Some(v)
            }
            J::Mot(m) => {
                self.i += 1;
                if self.regarde() == Some(&J::Ouvre) {
                    self.i += 1;
                    let a = self.expr()?;
                    if self.regarde() == Some(&J::Ferme) {
                        self.i += 1;
                    }
                    return match m.as_str() {
                        "exp" => Some(a.exp()),
                        "ln" | "log" => a.ln(),
                        "sin" => Some(a.sin()),
                        "cos" => Some(a.cos()),
                        "sqrt" | "racine" => Some(a.racine()),
                        _ => None,
                    };
                }
                match m.as_str() {
                    "z" => Some(self.z),
                    "i" => Some(C { re: 0.0, im: 1.0 }),
                    "pi" => Some(C::r(std::f64::consts::PI)),
                    "e" => Some(C::r(std::f64::consts::E)),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

pub(crate) fn evalue_complexe(expr: &str, z: C) -> Option<C> {
    let mut l = Lecteur {
        j: jetons(expr),
        i: 0,
        z,
    };
    let v = l.expr()?;
    if l.i < l.j.len() || !v.fini() {
        return None;
    }
    Some(v)
}

fn segments(points: Vec<Option<P2>>, saut: f64) -> Vec<Vec<P2>> {
    let mut out = Vec::new();
    let mut courant: Vec<P2> = Vec::new();
    for p in points {
        match p {
            Some(q) => {
                if let Some(&d) = courant.last().as_ref() {
                    if ((q.0 - d.0).powi(2) + (q.1 - d.1).powi(2)).sqrt() > saut {
                        if courant.len() > 1 {
                            out.push(std::mem::take(&mut courant));
                        } else {
                            courant.clear();
                        }
                    }
                }
                courant.push(q);
            }
            None => {
                if courant.len() > 1 {
                    out.push(std::mem::take(&mut courant));
                } else {
                    courant.clear();
                }
            }
        }
    }
    if courant.len() > 1 {
        out.push(courant);
    }
    out
}

fn expression_w(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("par w = ").or_else(|| bas.find("par "))?;
    let saute = if bas[i..].starts_with("par w = ") {
        "par w = ".len()
    } else {
        "par ".len()
    };
    let mut zone = desc[i + saute..].trim();
    for fin in [" en bleu", " en rouge", " en vert", " en violet", " en orange", " en noir"] {
        if let Some(j) = zone.to_lowercase().find(fin) {
            zone = zone[..j].trim();
        }
    }
    let e = zone.trim_end_matches(&[',', '.'][..]).trim();
    if e.is_empty() {
        None
    } else {
        Some(e.to_string())
    }
}

fn lit(s: &str) -> Option<f64> {
    crate::maths::calcul::eval(
        &s.trim().replace(',', ".").replace('π', "pi").replace('−', "-"),
        &std::collections::BTreeMap::new(),
    )
}

fn intervalle(zone: &str) -> Option<(f64, f64)> {
    let dedans = zone.trim().strip_prefix('[')?.split_once(']')?.0;
    let (a, b) = dedans.split_once(';')?;
    Some((lit(a)?, lit(b)?))
}

fn dessin(familles: &[(Vec<Vec<P2>>, &str)]) -> Option<String> {
    let tous: Vec<P2> = familles
        .iter()
        .flat_map(|(f, _)| f.iter().flatten().cloned())
        .collect();
    if tous.len() < 2 {
        return None;
    }
    let xs: Vec<f64> = tous.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = tous.iter().map(|p| p.1).collect();
    let (x0, x1) = (
        xs.iter().cloned().fold(f64::INFINITY, f64::min),
        xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );
    let (y0, y1) = (
        ys.iter().cloned().fold(f64::INFINITY, f64::min),
        ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );
    let mx = ((x1 - x0) * 0.1).max(0.4);
    let my = ((y1 - y0) * 0.1).max(0.4);
    let r = Repere::isotrope(x0 - mx, x1 + mx, y0 - my, y1 + my);
    let mut corps = axes(&r, "Im");
    for (famille, coul) in familles {
        for seg in famille {
            let mut d = String::new();
            for (k, p) in seg.iter().enumerate() {
                d.push_str(&format!(
                    "{}{:.2},{:.2} ",
                    if k == 0 { "M" } else { "L" },
                    r.px(p.0),
                    r.py(p.1)
                ));
            }
            corps.push_str(&format!(
                "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.4\"/>",
                d, coul
            ));
        }
    }
    Some(enveloppe_haute(&corps, "#1a4fa0", r.hauteur))
}

fn image_carre(desc: &str) -> Option<String> {
    let expr = expression_w(desc)?;
    let bas = desc.to_lowercase();
    let i = bas.find("carré")? + "carré".len();
    let zone = &desc[i..];
    let premier = zone.find('[')?;
    let (a, b) = intervalle(&zone[premier..])?;
    let ferme = zone[premier..].find(']')? + premier;
    let second = zone[ferme..].find('[')? + ferme;
    let (c, d) = intervalle(&zone[second..])?;
    let lignes = 11usize;
    let n = 140usize;
    let mut verticales = Vec::new();
    let mut horizontales = Vec::new();
    let etendue = |pts: &Vec<Option<P2>>| -> Vec<Option<P2>> { pts.clone() };
    for k in 0..lignes {
        let x = a + (b - a) * k as f64 / (lignes - 1) as f64;
        let pts: Vec<Option<P2>> = (0..=n)
            .map(|j| {
                let y = c + (d - c) * j as f64 / n as f64;
                evalue_complexe(&expr, C { re: x, im: y }).map(|w| (w.re, w.im))
            })
            .collect();
        verticales.push(etendue(&pts));
        let y = c + (d - c) * k as f64 / (lignes - 1) as f64;
        let pts: Vec<Option<P2>> = (0..=n)
            .map(|j| {
                let x = a + (b - a) * j as f64 / n as f64;
                evalue_complexe(&expr, C { re: x, im: y }).map(|w| (w.re, w.im))
            })
            .collect();
        horizontales.push(etendue(&pts));
    }
    let valides: Vec<P2> = verticales
        .iter()
        .chain(horizontales.iter())
        .flatten()
        .flatten()
        .cloned()
        .collect();
    if valides.len() < 4 {
        return None;
    }
    let diag = {
        let xs: Vec<f64> = valides.iter().map(|p| p.0).collect();
        let ys: Vec<f64> = valides.iter().map(|p| p.1).collect();
        let dx = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
            - xs.iter().cloned().fold(f64::INFINITY, f64::min);
        let dy = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
            - ys.iter().cloned().fold(f64::INFINITY, f64::min);
        (dx * dx + dy * dy).sqrt().max(1.0)
    };
    let fv: Vec<Vec<P2>> = verticales
        .into_iter()
        .flat_map(|p| segments(p, diag / 5.0))
        .collect();
    let fh: Vec<Vec<P2>> = horizontales
        .into_iter()
        .flat_map(|p| segments(p, diag / 5.0))
        .collect();
    dessin(&[(fv, couleur(desc)), (fh, "#c0392b")])
}

fn image_cercle(desc: &str) -> Option<String> {
    let expr = expression_w(desc)?;
    let bas = desc.to_lowercase();
    let rayon = if bas.contains("cercle unité") {
        1.0
    } else {
        let i = bas.find("cercle de rayon ")? + "cercle de rayon ".len();
        lit(
            &desc[i..]
                .trim_start()
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == ',' || *c == '.')
                .collect::<String>(),
        )?
    };
    let n = 720usize;
    let pts: Vec<Option<P2>> = (0..=n)
        .map(|k| {
            let t = 2.0 * std::f64::consts::PI * k as f64 / n as f64;
            evalue_complexe(
                &expr,
                C {
                    re: rayon * t.cos(),
                    im: rayon * t.sin(),
                },
            )
            .map(|w| (w.re, w.im))
        })
        .collect();
    let valides: Vec<P2> = pts.iter().flatten().cloned().collect();
    if valides.len() < 4 {
        return None;
    }
    let famille = segments(pts, 1e9);
    dessin(&[(famille, couleur(desc))])
}

pub fn commande(
    verbe: &str,
    desc: &str,
    _corps: Option<&str>,
    _env: &mut crate::Env,
) -> Option<String> {
    if verbe != "Trace" && verbe != "Représente" {
        return None;
    }
    let bas = desc.to_lowercase();
    if !bas.contains("l'image du") && !bas.contains("l'image de") {
        return None;
    }
    if bas.contains("carré") {
        return image_carre(desc);
    }
    if bas.contains("cercle") {
        return image_cercle(desc);
    }
    None
}
