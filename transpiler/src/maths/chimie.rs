use crate::maths::calcul::format_number;
use std::collections::BTreeMap;

const ELEMENTS: &[(&str, &str, f64)] = &[
    ("H", "hydrogène", 1.008),
    ("He", "hélium", 4.003),
    ("Li", "lithium", 6.94),
    ("Be", "béryllium", 9.012),
    ("B", "bore", 10.81),
    ("C", "carbone", 12.011),
    ("N", "azote", 14.007),
    ("O", "oxygène", 15.999),
    ("F", "fluor", 18.998),
    ("Ne", "néon", 20.180),
    ("Na", "sodium", 22.990),
    ("Mg", "magnésium", 24.305),
    ("Al", "aluminium", 26.982),
    ("Si", "silicium", 28.085),
    ("P", "phosphore", 30.974),
    ("S", "soufre", 32.06),
    ("Cl", "chlore", 35.45),
    ("Ar", "argon", 39.948),
    ("K", "potassium", 39.098),
    ("Ca", "calcium", 40.078),
    ("Sc", "scandium", 44.956),
    ("Ti", "titane", 47.867),
    ("V", "vanadium", 50.942),
    ("Cr", "chrome", 51.996),
    ("Mn", "manganèse", 54.938),
    ("Fe", "fer", 55.845),
    ("Co", "cobalt", 58.933),
    ("Ni", "nickel", 58.693),
    ("Cu", "cuivre", 63.546),
    ("Zn", "zinc", 65.38),
    ("Ga", "gallium", 69.723),
    ("Ge", "germanium", 72.630),
    ("As", "arsenic", 74.922),
    ("Se", "sélénium", 78.971),
    ("Br", "brome", 79.904),
    ("Kr", "krypton", 83.798),
    ("Rb", "rubidium", 85.468),
    ("Sr", "strontium", 87.62),
    ("Y", "yttrium", 88.906),
    ("Zr", "zirconium", 91.224),
    ("Nb", "niobium", 92.906),
    ("Mo", "molybdène", 95.95),
    ("Tc", "technétium", 98.0),
    ("Ru", "ruthénium", 101.07),
    ("Rh", "rhodium", 102.906),
    ("Pd", "palladium", 106.42),
    ("Ag", "argent", 107.868),
    ("Cd", "cadmium", 112.414),
    ("In", "indium", 114.818),
    ("Sn", "étain", 118.710),
    ("Sb", "antimoine", 121.760),
    ("Te", "tellure", 127.60),
    ("I", "iode", 126.904),
    ("Xe", "xénon", 131.293),
    ("Cs", "césium", 132.905),
    ("Ba", "baryum", 137.327),
    ("La", "lanthane", 138.905),
    ("Ce", "cérium", 140.116),
    ("Pr", "praséodyme", 140.908),
    ("Nd", "néodyme", 144.242),
    ("Pm", "prométhium", 145.0),
    ("Sm", "samarium", 150.36),
    ("Eu", "europium", 151.964),
    ("Gd", "gadolinium", 157.25),
    ("Tb", "terbium", 158.925),
    ("Dy", "dysprosium", 162.500),
    ("Ho", "holmium", 164.930),
    ("Er", "erbium", 167.259),
    ("Tm", "thulium", 168.934),
    ("Yb", "ytterbium", 173.045),
    ("Lu", "lutécium", 174.967),
    ("Hf", "hafnium", 178.486),
    ("Ta", "tantale", 180.948),
    ("W", "tungstène", 183.84),
    ("Re", "rhénium", 186.207),
    ("Os", "osmium", 190.23),
    ("Ir", "iridium", 192.217),
    ("Pt", "platine", 195.084),
    ("Au", "or", 196.967),
    ("Hg", "mercure", 200.592),
    ("Tl", "thallium", 204.38),
    ("Pb", "plomb", 207.2),
    ("Bi", "bismuth", 208.980),
    ("Po", "polonium", 209.0),
    ("At", "astate", 210.0),
    ("Rn", "radon", 222.0),
    ("Fr", "francium", 223.0),
    ("Ra", "radium", 226.0),
    ("Ac", "actinium", 227.0),
    ("Th", "thorium", 232.038),
    ("Pa", "protactinium", 231.036),
    ("U", "uranium", 238.029),
    ("Np", "neptunium", 237.0),
    ("Pu", "plutonium", 244.0),
];

fn element(symbole: &str) -> Option<&'static (&'static str, &'static str, f64)> {
    ELEMENTS.iter().find(|(s, _, _)| *s == symbole)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Espece {
    pub atomes: BTreeMap<String, i64>,
    pub charge: i64,
    pub brut: String,
}

fn lit_entier(s: &[u8], i: &mut usize) -> i64 {
    let debut = *i;
    while *i < s.len() && s[*i].is_ascii_digit() {
        *i += 1;
    }
    if *i == debut {
        1
    } else {
        std::str::from_utf8(&s[debut..*i]).unwrap().parse().unwrap_or(1)
    }
}

fn parse_groupe(s: &[u8], i: &mut usize, atomes: &mut BTreeMap<String, i64>, mult: i64) -> bool {
    while *i < s.len() {
        match s[*i] {
            b'(' => {
                *i += 1;
                let mut interne = BTreeMap::new();
                if !parse_groupe(s, i, &mut interne, 1) {
                    return false;
                }
                if *i >= s.len() || s[*i] != b')' {
                    return false;
                }
                *i += 1;
                let n = lit_entier(s, i);
                for (k, v) in interne {
                    *atomes.entry(k).or_insert(0) += v * n * mult;
                }
            }
            b')' => return true,
            b'A'..=b'Z' => {
                let debut = *i;
                *i += 1;
                if *i < s.len() && s[*i].is_ascii_lowercase() {
                    *i += 1;
                }
                let sym = std::str::from_utf8(&s[debut..*i]).unwrap();
                if element(sym).is_none() {
                    return false;
                }
                let n = lit_entier(s, i);
                *atomes.entry(sym.to_string()).or_insert(0) += n * mult;
            }
            _ => return false,
        }
    }
    true
}

pub fn parse_espece(brut: &str) -> Option<Espece> {
    let brut = brut.trim();
    let (formule, charge) = match brut.split_once('^') {
        Some((f, c)) => {
            let c = c.trim();
            let (chiffres, signe): (String, i64) = if let Some(r) = c.strip_suffix('+') {
                (r.to_string(), 1)
            } else if let Some(r) = c.strip_suffix('-') {
                (r.to_string(), -1)
            } else {
                return None;
            };
            let n: i64 = if chiffres.is_empty() {
                1
            } else {
                chiffres.parse().ok()?
            };
            (f.trim(), n * signe)
        }
        None => (brut, 0),
    };
    if formule == "e" {
        return Some(Espece {
            atomes: BTreeMap::new(),
            charge,
            brut: brut.to_string(),
        });
    }
    let mut prefixe = 0usize;
    let octets = formule.as_bytes();
    while prefixe < octets.len() && octets[prefixe].is_ascii_digit() {
        prefixe += 1;
    }
    let mult: i64 = if prefixe == 0 {
        1
    } else {
        formule[..prefixe].parse().ok()?
    };
    let corps = &formule[prefixe..];
    if corps.is_empty() {
        return None;
    }
    let mut atomes = BTreeMap::new();
    let mut i = 0usize;
    if !parse_groupe(corps.as_bytes(), &mut i, &mut atomes, mult) || i != corps.len() {
        return None;
    }
    Some(Espece {
        atomes,
        charge: charge * mult,
        brut: corps.to_string()
            + &if charge == 0 {
                String::new()
            } else {
                format!("^{}", suffixe_charge(charge))
            },
    })
}

fn suffixe_charge(c: i64) -> String {
    match c {
        1 => "+".into(),
        -1 => "-".into(),
        n if n > 0 => format!("{}+", n),
        n => format!("{}-", -n),
    }
}

fn latex_espece(e: &Espece) -> String {
    let corps = e.brut.split('^').next().unwrap_or(&e.brut);
    if corps == "e" || (e.atomes.is_empty() && e.charge == -1) {
        return "\\mathrm{e}^{-}".into();
    }
    let mut out = String::from("\\mathrm{");
    for c in corps.chars() {
        match c {
            '0'..='9' => {
                out.push_str("}_{");
                out.push(c);
                out.push_str("}\\mathrm{");
            }
            _ => out.push(c),
        }
    }
    out.push('}');
    let mut out = out.replace("\\mathrm{}", "");
    if e.charge != 0 {
        out.push_str(&format!("^{{{}}}", suffixe_charge(e.charge)));
    }
    out
}

fn pgcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs().max(1)
    } else {
        pgcd(b, a % b)
    }
}

fn equilibre(gauche: &[Espece], droite: &[Espece]) -> Option<Vec<i64>> {
    let mut lignes: Vec<String> = Vec::new();
    for e in gauche.iter().chain(droite) {
        for k in e.atomes.keys() {
            if !lignes.contains(k) {
                lignes.push(k.clone());
            }
        }
    }
    let charges = gauche.iter().chain(droite).any(|e| e.charge != 0);
    let n = gauche.len() + droite.len();
    let m = lignes.len() + usize::from(charges);
    let mut a = vec![vec![0i64; n]; m];
    for (j, e) in gauche.iter().chain(droite.iter()).enumerate() {
        let signe = if j < gauche.len() { 1 } else { -1 };
        for (idx, elt) in lignes.iter().enumerate() {
            a[idx][j] = signe * e.atomes.get(elt).copied().unwrap_or(0);
        }
        if charges {
            a[lignes.len()][j] = signe * e.charge;
        }
    }
    let mut pivots: Vec<usize> = Vec::new();
    let mut r = 0usize;
    for c in 0..n {
        let Some(p) = (r..m).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(r, p);
        for i in 0..m {
            if i != r && a[i][c] != 0 {
                let (x, y) = (a[r][c], a[i][c]);
                for k in 0..n {
                    a[i][k] = a[i][k] * x - a[r][k] * y;
                }
                let g = a[i].iter().fold(0, |acc, &v| pgcd(acc, v));
                if g > 1 {
                    for v in a[i].iter_mut() {
                        *v /= g;
                    }
                }
            }
        }
        pivots.push(c);
        r += 1;
        if r == m {
            break;
        }
    }
    if pivots.len() != n - 1 {
        return None;
    }
    let libre = (0..n).find(|c| !pivots.contains(c))?;
    let mut num = vec![0i64; n];
    let mut den = vec![1i64; n];
    num[libre] = 1;
    for (ligne, &c) in pivots.iter().enumerate() {
        num[c] = -a[ligne][libre];
        den[c] = a[ligne][c];
    }
    let mut ppcm = 1i64;
    for &d in &den {
        ppcm = ppcm / pgcd(ppcm, d) * d.abs();
    }
    let mut coeffs: Vec<i64> = num
        .iter()
        .zip(&den)
        .map(|(&nu, &de)| nu * (ppcm / de))
        .collect();
    if coeffs.iter().any(|&c| c < 0) && coeffs.iter().all(|&c| c <= 0) {
        for c in coeffs.iter_mut() {
            *c = -*c;
        }
    }
    if coeffs.iter().any(|&c| c <= 0) {
        return None;
    }
    let g = coeffs.iter().fold(0, |acc, &v| pgcd(acc, v));
    for c in coeffs.iter_mut() {
        *c /= g;
    }
    Some(coeffs)
}

pub fn parse_equation(desc: &str) -> Option<(Vec<Espece>, Vec<Espece>)> {
    let normalise = desc.replace('→', "->").replace('⟶', "->");
    let (g, d) = normalise
        .split_once("->")
        .or_else(|| normalise.split_once('='))?;
    let membre = |s: &str| -> Option<Vec<Espece>> {
        s.split(" + ")
            .map(|m| parse_espece(m.trim().trim_end_matches('.')))
            .collect()
    };
    let gauche = membre(g)?;
    let droite = membre(d)?;
    if gauche.is_empty() || droite.is_empty() {
        return None;
    }
    Some((gauche, droite))
}

fn latex_membre(especes: &[Espece], coeffs: &[i64]) -> String {
    especes
        .iter()
        .zip(coeffs)
        .map(|(e, &c)| {
            if c == 1 {
                latex_espece(e)
            } else {
                format!("{}\\,{}", c, latex_espece(e))
            }
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn commande_equilibre(desc: &str) -> Option<String> {
    let apres = desc
        .split_once("équation")
        .or_else(|| desc.split_once("equation"))
        .map(|(_, r)| r)
        .unwrap_or(desc);
    let (gauche, droite) = parse_equation(apres)?;
    let coeffs = equilibre(&gauche, &droite)?;
    let (cg, cd) = coeffs.split_at(gauche.len());
    Some(crate::layout::rendu::bloc_calcul(&format!(
        "{} \\longrightarrow {}",
        latex_membre(&gauche, cg),
        latex_membre(&droite, cd)
    )))
}

fn masse_molaire(e: &Espece) -> f64 {
    e.atomes
        .iter()
        .map(|(sym, n)| element(sym).map(|(_, _, m)| m * *n as f64).unwrap_or(0.0))
        .sum()
}

fn commande_masse_molaire(desc: &str) -> Option<String> {
    let apres = desc
        .split_once("masse molaire de")
        .or_else(|| desc.split_once("masse molaire du"))
        .map(|(_, r)| r)?
        .trim()
        .trim_end_matches('.');
    let brut = apres.split_whitespace().next()?;
    let e = parse_espece(brut)?;
    let mut termes: Vec<String> = Vec::new();
    for (sym, n) in &e.atomes {
        element(sym)?;
        termes.push(if *n == 1 {
            format!("M(\\mathrm{{{}}})", sym)
        } else {
            format!("{}\\,M(\\mathrm{{{}}})", n, sym)
        });
    }
    let mut valeurs: Vec<String> = Vec::new();
    for (sym, n) in &e.atomes {
        let (_, _, m) = element(sym)?;
        valeurs.push(if *n == 1 {
            format_number(*m).replace(',', "{,}")
        } else {
            format!("{} \\times {}", n, format_number(*m).replace(',', "{,}"))
        });
    }
    let total = masse_molaire(&e);
    Some(crate::layout::rendu::bloc_calcul(&format!(
        "M({}) = {} = {} = {}\\ \\mathrm{{g\\,mol^{{-1}}}}",
        latex_espece(&e),
        termes.join(" + "),
        valeurs.join(" + "),
        format_number((total * 10.0).round() / 10.0).replace(',', "{,}")
    )))
}

fn nombre_apres(desc: &str, espece: &str) -> Option<f64> {
    let cle = format!("n({})", espece);
    let i = desc.find(&cle)? + cle.len();
    let reste = desc[i..].trim_start().strip_prefix('=')?.trim_start();
    let fin = reste
        .find(|c: char| !(c.is_ascii_digit() || c == ',' || c == '.'))
        .unwrap_or(reste.len());
    reste[..fin].trim_end_matches(['.', ',']).replace(',', ".").parse().ok()
}

fn commande_avancement(desc: &str) -> Option<String> {
    let apres = desc
        .split_once("pour")
        .map(|(_, r)| r)
        .unwrap_or(desc);
    let (equation, quantites) = match apres.split_once(" avec ") {
        Some((e, q)) => (e, q),
        None => (apres, ""),
    };
    let (gauche, droite) = parse_equation(equation)?;
    let coeffs = equilibre(&gauche, &droite)?;
    let (cg, cd) = coeffs.split_at(gauche.len());
    let initiales: Vec<Option<f64>> = gauche
        .iter()
        .map(|e| nombre_apres(quantites, e.brut.split('^').next().unwrap_or(&e.brut)))
        .collect();
    let entete = format!(
        "\\[{} \\longrightarrow {}\\]",
        latex_membre(&gauche, cg),
        latex_membre(&droite, cd)
    );
    let cellule = |_e: &Espece, c: i64, n0: Option<f64>, signe: f64, x: Option<f64>| -> String {
        let nom_generique = signe < 0.0;
        let coeff = if c == 1 {
            "x".to_string()
        } else {
            format!("{}x", c)
        };
        match (n0, x) {
            (Some(n0), None) => format!(
                "\\({} {} {}\\)",
                format_number(n0).replace(',', "{,}"),
                if signe > 0.0 { "+" } else { "-" },
                coeff
            ),
            (Some(n0), Some(x)) => format!(
                "\\({}\\)",
                format_number(((n0 + signe * c as f64 * x) * 1e6).round() / 1e6)
                    .replace(',', "{,}")
            ),
            (None, None) => format!(
                "\\({}{}{}\\)",
                if nom_generique { "n_0 " } else { "" },
                if signe > 0.0 {
                    if nom_generique { "+ " } else { "" }
                } else {
                    "- "
                },
                coeff
            ),
            (None, Some(x)) => format!(
                "\\({}\\)",
                format_number((signe * c as f64 * x * 1e6).round() / 1e6).replace(',', "{,}")
            ),
        }
    };
    let toutes = initiales.iter().all(|n| n.is_some());
    let xmax = if toutes {
        let (jmax, x) = gauche
            .iter()
            .zip(cg)
            .zip(&initiales)
            .enumerate()
            .map(|(j, ((_, &c), n0))| (j, n0.unwrap() / c as f64))
            .fold((0usize, f64::INFINITY), |acc, (j, x)| {
                if x < acc.1 {
                    (j, x)
                } else {
                    acc
                }
            });
        Some((jmax, x))
    } else {
        None
    };
    let mut lignes = String::new();
    let etat = |nom: &str, x: Option<f64>| -> String {
        let mut cellules = format!("<td>{}</td>", nom);
        for ((e, &c), n0) in gauche.iter().zip(cg).zip(&initiales) {
            let contenu = match (nom, n0, x) {
                ("État initial", Some(v), _) => {
                    format!("\\({}\\)", format_number(*v).replace(',', "{,}"))
                }
                ("État initial", None, _) => "\\(n_0\\)".into(),
                _ => cellule(e, c, *n0, -1.0, x),
            };
            cellules.push_str(&format!("<td>{}</td>", contenu));
        }
        for (e, &c) in droite.iter().zip(cd) {
            let contenu = match (nom, x) {
                ("État initial", _) => "\\(0\\)".into(),
                ("En cours", _) => cellule(e, c, None, 1.0, None),
                _ => cellule(e, c, Some(0.0), 1.0, x),
            };
            cellules.push_str(&format!("<td>{}</td>", contenu));
        }
        format!("<tr>{}</tr>", cellules)
    };
    lignes.push_str(&etat("État initial", None));
    lignes.push_str(&etat("En cours", None));
    lignes.push_str(&etat(
        "État final",
        xmax.map(|(_, x)| x),
    ));
    let mut tetes = String::from("<th>Avancement (mol)</th>");
    for e in gauche.iter().chain(droite.iter()) {
        tetes.push_str(&format!("<th>\\({}\\)</th>", latex_espece(e)));
    }
    let mut conclusion = String::new();
    if let Some((jmax, x)) = xmax {
        conclusion = format!(
            "<p>Le réactif limitant est \\({}\\) : \\(x_{{\\max}} = \\dfrac{{{}}}{{{}}} = {}\\ \\mathrm{{mol}}\\).</p>",
            latex_espece(&gauche[jmax]),
            format_number(initiales[jmax].unwrap()).replace(',', "{,}"),
            cg[jmax],
            format_number((x * 1e6).round() / 1e6).replace(',', "{,}")
        );
    }
    Some(format!(
        "<div class=\"calcul\">{}<table class=\"signes avancement\"><tr>{}</tr>{}</table>{}</div>",
        entete, tetes, lignes, conclusion
    ))
}

pub fn commande(
    verbe: &str,
    desc: &str,
    _corps: Option<&str>,
    _env: &mut crate::Env,
) -> Option<String> {
    let bas = desc.to_lowercase();
    match verbe {
        "Équilibre" => commande_equilibre(desc),
        "Calcule" | "Détermine" if bas.contains("masse molaire") => commande_masse_molaire(desc),
        "Dresse" if bas.contains("tableau d'avancement") || bas.contains("tableau d’avancement") => {
            commande_avancement(desc)
        }
        _ => None,
    }
}
