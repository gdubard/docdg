use crate::maths::trace::{axes, couleur, enveloppe_haute, Repere};

fn lit(s: &str) -> Option<f64> {
    crate::maths::calcul::eval(
        &s.trim().replace(',', ".").replace('−', "-"),
        &std::collections::BTreeMap::new(),
    )
}

fn nombre_apres(desc: &str, cle: &str) -> Option<f64> {
    let i = desc.to_lowercase().find(&cle.to_lowercase())? + cle.len();
    let brut: String = desc[i..]
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ',' || *c == '.' || *c == '-' || *c == '/')
        .collect();
    lit(&brut)
}

fn densite_normale(x: f64, m: f64, s: f64) -> f64 {
    let u = (x - m) / s;
    (-u * u / 2.0).exp() / (s * (2.0 * std::f64::consts::PI).sqrt())
}

fn courbe_normale(r: &Repere, m: f64, s: f64) -> String {
    let n = 300;
    let mut d = String::new();
    for k in 0..=n {
        let x = r.x0 + (r.x1 - r.x0) * k as f64 / n as f64;
        let y = densite_normale(x, m, s);
        d.push_str(&format!(
            "{}{:.2},{:.2} ",
            if k == 0 { "M" } else { "L" },
            r.px(x),
            r.py(y)
        ));
    }
    format!("<path d=\"{}\" class=\"courbe\"/>", d)
}

fn normale(desc: &str) -> Option<String> {
    let m = nombre_apres(desc, "espérance ").or_else(|| nombre_apres(desc, "esperance "))?;
    let s = nombre_apres(desc, "écart type ")
        .or_else(|| nombre_apres(desc, "ecart type "))
        .filter(|v| *v > 0.0)?;
    let (x0, x1) = (m - 4.0 * s, m + 4.0 * s);
    let sommet = densite_normale(m, m, s);
    let r = Repere::etire(x0, x1, -0.06 * sommet, 1.12 * sommet);
    let mut corps = axes(&r, "f(x)");
    corps.push_str(&courbe_normale(&r, m, s));
    corps.push_str(&format!(
        "<line x1=\"{0:.2}\" y1=\"{1:.2}\" x2=\"{0:.2}\" y2=\"{2:.2}\" class=\"repere\"/>",
        r.px(m),
        r.py(0.0),
        r.py(sommet)
    ));
    Some(enveloppe_haute(&corps, couleur(desc), r.hauteur))
}

fn somme_de_des(n: usize) -> Vec<f64> {
    let mut loi = vec![1.0f64 / 6.0; 6];
    for _ in 1..n {
        let mut suivante = vec![0.0; loi.len() + 5];
        for (i, p) in loi.iter().enumerate() {
            for face in 0..6 {
                suivante[i + face] += p / 6.0;
            }
        }
        loi = suivante;
    }
    loi
}

fn tcl(desc: &str) -> Option<String> {
    let n = nombre_apres(desc, "somme de ").map(|v| v as usize).filter(|v| (2..=40).contains(v))?;
    let loi = somme_de_des(n);
    let m = 3.5 * n as f64;
    let s = (35.0 * n as f64 / 12.0).sqrt();
    let sommet = loi.iter().cloned().fold(0.0f64, f64::max).max(densite_normale(m, m, s));
    let (x0, x1) = ((n as f64 - 0.8).max(m - 4.2 * s), (6.0 * n as f64 + 0.8).min(m + 4.2 * s));
    let r = Repere::etire(x0, x1, -0.06 * sommet, 1.12 * sommet);
    let mut corps = axes(&r, "P");
    let demi = 0.42;
    for (i, p) in loi.iter().enumerate() {
        let k = (n + i) as f64;
        if k < x0 || k > x1 {
            continue;
        }
        corps.push_str(&format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#9db9de\" stroke=\"#1a3a6b\" stroke-width=\"0.2\"/>",
            r.px(k - demi),
            r.py(*p),
            r.px(k + demi) - r.px(k - demi),
            r.py(0.0) - r.py(*p)
        ));
    }
    corps.push_str(&courbe_normale(&r, m, s));
    Some(enveloppe_haute(&corps, "#c0392b", r.hauteur))
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
    if bas.contains("théorème central limite") || bas.contains("theoreme central limite") {
        return tcl(desc);
    }
    if bas.contains("densité de la loi normale") || bas.contains("densite de la loi normale") {
        return normale(desc);
    }
    None
}
