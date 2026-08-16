use crate::utils::texte::{apres_cle, nom_apres};
use crate::Env;

const PI: f64 = std::f64::consts::PI;

fn nombre(v: f64) -> String {
    let arrondi = (v * 1e6).round() / 1e6;
    if (arrondi - arrondi.round()).abs() < 1e-9 {
        return format!("{}", arrondi.round() as i64);
    }
    let texte = format!("{:.2}", v);
    let court = texte.trim_end_matches('0').trim_end_matches('.');
    court.replace('.', "{,}")
}

fn ordonne(nom: &[char], a: char, b: char) -> String {
    let rang = |c: char| nom.iter().position(|x| *x == c).unwrap_or(0);
    if rang(a) <= rang(b) {
        format!("{}{}", a, b)
    } else {
        format!("{}{}", b, a)
    }
}

fn signe_egal(v: f64) -> &'static str {
    let arrondi = (v * 1e6).round() / 1e6;
    if (arrondi - arrondi.round()).abs() < 1e-9 {
        "="
    } else {
        "\\approx"
    }
}

fn reel(s: &str) -> Option<f64> {
    s.trim()
        .trim_end_matches(['.', ',', ';'])
        .replace(',', ".")
        .replace('−', "-")
        .trim()
        .parse::<f64>()
        .ok()
}

fn segment(nom: &str) -> String {
    let mut lettres: Vec<char> = nom.chars().filter(|c| c.is_alphabetic()).collect();
    lettres.sort_unstable();
    lettres.into_iter().collect()
}

fn mesures(desc: &str) -> Vec<(String, f64)> {
    let mut out = Vec::new();
    for morceau in desc.split(&[',', ';'][..]) {
        for bout in morceau.split(" et ") {
            if let Some((nom, valeur)) = bout.split_once('=') {
                let nom: String = nom
                    .split_whitespace()
                    .last()
                    .unwrap_or("")
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect();
                if let Some(v) = reel(valeur) {
                    if nom.len() >= 2 {
                        out.push((nom, v));
                    }
                }
            }
        }
    }
    out
}

fn valeur(liste: &[(String, f64)], nom: &str) -> Option<f64> {
    let cle = segment(nom);
    liste
        .iter()
        .find(|(n, _)| segment(n) == cle)
        .map(|(_, v)| *v)
}

fn nombre_apres(desc: &str, cle: &str) -> Option<f64> {
    let i = desc.to_lowercase().find(cle)? + cle.len();
    let reste = desc[i..].trim_start();
    let fin = reste
        .find(|c: char| !(c.is_ascii_digit() || c == ',' || c == '.' || c == '-'))
        .unwrap_or(reste.len());
    let valeur = reel(&reste[..fin])?;

    if reste[fin..].trim_start().starts_with("mm") {
        return Some(valeur / 10.0);
    }
    Some(valeur)
}

fn lettres_triangle(desc: &str, bas: &str) -> Option<Vec<char>> {
    let i = bas.find("triangle ")? + "triangle ".len();
    Some(
        desc[i..]
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect(),
    )
}

fn triangle_rectangle(desc: &str) -> Option<(Vec<char>, char)> {
    let bas = desc.to_lowercase();
    let nom = lettres_triangle(desc, &bas)?;
    let j = bas.find("rectangle en ")? + "rectangle en ".len();
    let sommet = desc[j..].trim_start().chars().next()?;
    if nom.len() != 3 || !nom.contains(&sommet) {
        return None;
    }
    Some((nom, sommet))
}

fn cible(desc: &str) -> Option<String> {
    let mot: String = desc
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphabetic())
        .collect();
    if mot.len() == 2 {
        Some(mot)
    } else {
        None
    }
}

fn prose(lignes: &[String]) -> Option<String> {
    Some(crate::maths::algebre::bloc_prose(lignes))
}

fn pythagore(desc: &str) -> Option<String> {
    let (nom, sommet) = triangle_rectangle(desc)?;
    let vise = cible(desc)?;
    let donnees = mesures(desc);
    let autres: Vec<char> = nom.iter().copied().filter(|c| *c != sommet).collect();
    let hypotenuse: String = autres.iter().collect();
    let cotes: Vec<String> = autres.iter().map(|c| ordonne(&nom, sommet, *c)).collect();
    let triangle: String = nom.iter().collect();

    if segment(&vise) == segment(&hypotenuse) {
        let a = valeur(&donnees, &cotes[0])?;
        let b = valeur(&donnees, &cotes[1])?;
        let carre = a * a + b * b;
        let r = carre.sqrt();
        return prose(&[
            format!(
                "Le triangle \\({}\\) est rectangle en \\({}\\), donc \\({}^2 = {}^2 + {}^2\\).",
                triangle, sommet, hypotenuse, cotes[0], cotes[1]
            ),
            format!(
                "\\[{}^2 = {}^2 + {}^2 = {}\\]",
                hypotenuse,
                nombre(a),
                nombre(b),
                nombre(carre)
            ),
            format!("\\[{} {} {}\\]", hypotenuse, signe_egal(r), nombre(r)),
        ]);
    }

    let autre = cotes.iter().find(|c| segment(c) != segment(&vise))?;
    let h = valeur(&donnees, &hypotenuse)?;
    let a = valeur(&donnees, autre)?;
    let carre = h * h - a * a;
    if carre < 0.0 {
        return None;
    }
    let r = carre.sqrt();
    prose(&[
        format!(
            "Le triangle \\({}\\) est rectangle en \\({}\\), donc \\({}^2 = {}^2 - {}^2\\).",
            triangle, sommet, vise, hypotenuse, autre
        ),
        format!(
            "\\[{}^2 = {}^2 - {}^2 = {}\\]",
            vise,
            nombre(h),
            nombre(a),
            nombre(carre)
        ),
        format!("\\[{} {} {}\\]", vise, signe_egal(r), nombre(r)),
    ])
}

fn reciproque_pythagore(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let nom = lettres_triangle(desc, &bas)?;
    if nom.len() != 3 {
        return None;
    }
    let donnees = mesures(desc);
    let mut cotes: Vec<(String, f64)> = Vec::new();
    for (a, b) in [(0usize, 1usize), (1, 2), (0, 2)] {
        let s = format!("{}{}", nom[a], nom[b]);
        cotes.push((s.clone(), valeur(&donnees, &s)?));
    }
    cotes.sort_by(|x, y| x.1.total_cmp(&y.1));
    let somme = cotes[0].1 * cotes[0].1 + cotes[1].1 * cotes[1].1;
    let carre = cotes[2].1 * cotes[2].1;
    let triangle: String = nom.iter().collect();
    let sommet = nom
        .iter()
        .find(|c| !cotes[2].0.contains(**c))
        .copied()
        .unwrap_or(nom[0]);
    let mut lignes = vec![
        format!(
            "Le plus grand côté est \\({}\\), de longueur \\({}\\).",
            cotes[2].0,
            nombre(cotes[2].1)
        ),
        format!(
            "\\[{}^2 = {} \\qquad {}^2 + {}^2 = {}\\]",
            cotes[2].0,
            nombre(carre),
            cotes[0].0,
            cotes[1].0,
            nombre(somme)
        ),
    ];
    if (carre - somme).abs() < 1e-9 {
        lignes.push(format!(
            "L'égalité de Pythagore est vérifiée : le triangle \\({}\\) est rectangle en \\({}\\).",
            triangle, sommet
        ));
    } else {
        lignes.push(format!(
            "L'égalité de Pythagore n'est pas vérifiée : le triangle \\({}\\) n'est pas rectangle.",
            triangle
        ));
    }
    prose(&lignes)
}

fn thales(desc: &str) -> Option<String> {
    let vise = cible(desc)?;
    let donnees = mesures(desc);
    if donnees.len() < 3 {
        return None;
    }
    let sommet = donnees[0].0.chars().next()?;
    let radiales: Vec<&(String, f64)> = donnees
        .iter()
        .filter(|(n, _)| n.starts_with(sommet))
        .collect();
    if radiales.len() < 2 {
        return None;
    }
    let (petit, grand) = (radiales[0], radiales[1]);
    let rapport = petit.1 / grand.1;

    if vise.starts_with(sommet) {
        let troisieme = radiales.get(2)?;
        let r = troisieme.1 / rapport;
        return prose(&[
            format!(
                "Le théorème de Thalès donne \\(\\dfrac{{{}}}{{{}}} = \\dfrac{{{}}}{{{}}}\\).",
                petit.0, grand.0, troisieme.0, vise
            ),
            format!(
                "\\[{} = {} \\times \\dfrac{{{}}}{{{}}} = {} \\times \\dfrac{{{}}}{{{}}}\\]",
                vise,
                troisieme.0,
                grand.0,
                petit.0,
                nombre(troisieme.1),
                nombre(grand.1),
                nombre(petit.1)
            ),
            format!("\\[{} {} {}\\]", vise, signe_egal(r), nombre(r)),
        ]);
    }

    let base = donnees.iter().find(|(n, _)| !n.starts_with(sommet))?;
    let r = base.1 * rapport;
    prose(&[
        format!(
            "Le théorème de Thalès donne \\(\\dfrac{{{}}}{{{}}} = \\dfrac{{{}}}{{{}}}\\).",
            petit.0, grand.0, vise, base.0
        ),
        format!(
            "\\[{} = {} \\times \\dfrac{{{}}}{{{}}} = {} \\times \\dfrac{{{}}}{{{}}}\\]",
            vise,
            base.0,
            petit.0,
            grand.0,
            nombre(base.1),
            nombre(petit.1),
            nombre(grand.1)
        ),
        format!("\\[{} {} {}\\]", vise, signe_egal(r), nombre(r)),
    ])
}

fn reciproque_thales(desc: &str) -> Option<String> {
    let donnees = mesures(desc);
    if donnees.len() < 4 {
        return None;
    }
    let i = desc.find('(')?;
    let premiere: String = desc[i + 1..].chars().take_while(|c| *c != ')').collect();
    let j = desc[i + 1..].find('(')? + i + 2;
    let seconde: String = desc[j..].chars().take_while(|c| *c != ')').collect();
    let (a, b, c, d) = (&donnees[0], &donnees[1], &donnees[2], &donnees[3]);
    let egaux = (a.1 / b.1 - c.1 / d.1).abs() < 1e-9;
    let mut lignes = vec![format!(
        "\\[\\dfrac{{{}}}{{{}}} = \\dfrac{{{}}}{{{}}} \\qquad \\dfrac{{{}}}{{{}}} = \\dfrac{{{}}}{{{}}}\\]",
        a.0,
        b.0,
        nombre(a.1),
        nombre(b.1),
        c.0,
        d.0,
        nombre(c.1),
        nombre(d.1)
    )];
    if egaux {
        lignes.push(format!(
            "Les rapports sont égaux : d'après la réciproque du théorème de Thalès, \
             les droites \\(({})\\) et \\(({})\\) sont parallèles.",
            premiere, seconde
        ));
    } else {
        lignes.push(format!(
            "Les rapports sont différents : les droites \\(({})\\) et \\(({})\\) \
             ne sont pas parallèles.",
            premiere, seconde
        ));
    }
    prose(&lignes)
}

fn ligne_rapport(
    triangle: &str,
    sommet: char,
    formule: &str,
    angle: char,
    haut: &str,
    bas: &str,
) -> String {
    format!(
        "Dans le triangle \\({}\\) rectangle en \\({}\\), \
         \\({}\\left(\\widehat{{{}}}\\right) = \\dfrac{{{}}}{{{}}}\\).",
        triangle, sommet, formule, angle, haut, bas
    )
}

fn angle_cherche(desc: &str) -> Option<String> {
    let (nom, sommet) = triangle_rectangle(desc)?;
    let donnees = mesures(desc);
    let angle = apres_cle(desc, "l'angle ")?.trim_start().chars().next()?;
    let autres: Vec<char> = nom.iter().copied().filter(|c| *c != sommet).collect();
    let hypotenuse: String = autres.iter().collect();
    let adjacent = format!("{}{}", angle, sommet);
    let oppose: String = autres.iter().filter(|c| **c != angle).collect();
    let oppose = format!("{}{}", sommet, oppose);
    let triangle: String = nom.iter().collect();
    let (h, adj, opp) = (
        valeur(&donnees, &hypotenuse),
        valeur(&donnees, &adjacent),
        valeur(&donnees, &oppose),
    );
    let (formule, haut, bas_nom, hv, bv) = match (h, adj, opp) {
        (Some(h), Some(a), _) => ("\\cos", adjacent.clone(), hypotenuse.clone(), a, h),
        (Some(h), _, Some(o)) => ("\\sin", oppose.clone(), hypotenuse.clone(), o, h),
        (_, Some(a), Some(o)) => ("\\tan", oppose.clone(), adjacent.clone(), o, a),
        _ => return None,
    };
    let rapport = hv / bv;
    let mesure = match formule {
        "\\cos" => rapport.acos(),
        "\\sin" => rapport.asin(),
        _ => rapport.atan(),
    }
    .to_degrees();
    prose(&[
        ligne_rapport(&triangle, sommet, formule, angle, &haut, &bas_nom),
        format!(
            "\\[{}\\left(\\widehat{{{}}}\\right) = \\dfrac{{{}}}{{{}}}\\]",
            formule,
            angle,
            nombre(hv),
            nombre(bv)
        ),
        format!(
            "\\[\\widehat{{{}}} {} {}^\\circ\\]",
            angle,
            signe_egal(mesure),
            nombre(mesure)
        ),
    ])
}

fn cote_par_angle(desc: &str) -> Option<String> {
    let (nom, sommet) = triangle_rectangle(desc)?;
    let donnees = mesures(desc);
    let vise = cible(desc)?;
    let angle = apres_cle(desc, "l'angle ")?.trim_start().chars().next()?;
    let mesure = nombre_apres(desc, "= ")?;
    let autres: Vec<char> = nom.iter().copied().filter(|c| *c != sommet).collect();
    let hypotenuse: String = autres.iter().collect();
    let h = valeur(&donnees, &hypotenuse)?;
    let adjacent = format!("{}{}", angle, sommet);
    let triangle: String = nom.iter().collect();
    let (formule, rapport) = if segment(&vise) == segment(&adjacent) {
        ("\\cos", mesure.to_radians().cos())
    } else {
        ("\\sin", mesure.to_radians().sin())
    };
    let r = h * rapport;
    prose(&[
        ligne_rapport(&triangle, sommet, formule, angle, &vise, &hypotenuse),
        format!(
            "\\[{} = {} \\times {}\\left({}^\\circ\\right)\\]",
            vise,
            nombre(h),
            formule,
            nombre(mesure)
        ),
        format!("\\[{} {} {}\\]", vise, signe_egal(r), nombre(r)),
    ])
}

fn mesure_usuelle(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let l = |cle: &str| nombre_apres(desc, cle);
    let pose = |quoi: &str, formule: &str, calcul: String, v: f64| {
        prose(&[
            format!("{} : \\({}\\).", quoi, formule),
            format!("\\[{} {} {}\\]", calcul, signe_egal(v), nombre(v)),
        ])
    };

    if bas.contains("périmètre du rectangle") {
        let (a, b) = (l("longueur ")?, l("largeur ")?);
        return pose(
            "Le périmètre d'un rectangle",
            "\\mathcal{P} = 2 \\times (L + \\ell)",
            format!("\\mathcal{{P}} = 2 \\times ({} + {})", nombre(a), nombre(b)),
            2.0 * (a + b),
        );
    }
    if bas.contains("périmètre du cercle") {
        let r = l("rayon ")?;
        return pose(
            "Le périmètre d'un cercle",
            "\\mathcal{P} = 2\\pi r",
            format!("\\mathcal{{P}} = 2\\pi \\times {}", nombre(r)),
            2.0 * PI * r,
        );
    }
    if bas.contains("périmètre du triangle") {
        let cotes: Vec<f64> = desc
            .split(&[',', ' '][..])
            .filter_map(reel)
            .take(3)
            .collect();
        if cotes.len() < 3 {
            return None;
        }
        return pose(
            "Le périmètre d'un triangle",
            "\\mathcal{P} = a + b + c",
            format!(
                "\\mathcal{{P}} = {} + {} + {}",
                nombre(cotes[0]),
                nombre(cotes[1]),
                nombre(cotes[2])
            ),
            cotes.iter().sum(),
        );
    }
    if bas.contains("aire du carré") {
        let c = l("côté ")?;
        return pose(
            "L'aire d'un carré",
            "\\mathcal{A} = c^2",
            format!("\\mathcal{{A}} = {}^2", nombre(c)),
            c * c,
        );
    }
    if bas.contains("aire du triangle") {
        let (b, h) = (l("base ")?, l("hauteur ")?);
        return pose(
            "L'aire d'un triangle",
            "\\mathcal{A} = \\dfrac{b \\times h}{2}",
            format!(
                "\\mathcal{{A}} = \\dfrac{{{} \\times {}}}{{2}}",
                nombre(b),
                nombre(h)
            ),
            b * h / 2.0,
        );
    }
    if bas.contains("aire du disque") {
        let r = l("rayon ")?;
        return pose(
            "L'aire d'un disque",
            "\\mathcal{A} = \\pi r^2",
            format!("\\mathcal{{A}} = \\pi \\times {}^2", nombre(r)),
            PI * r * r,
        );
    }
    if bas.contains("volume du pavé") {
        let (a, b, c) = (l("longueur ")?, l("largeur ")?, l("hauteur ")?);
        return pose(
            "Le volume d'un pavé droit",
            "\\mathcal{V} = L \\times \\ell \\times h",
            format!(
                "\\mathcal{{V}} = {} \\times {} \\times {}",
                nombre(a),
                nombre(b),
                nombre(c)
            ),
            a * b * c,
        );
    }
    if bas.contains("volume du cylindre") {
        let (r, h) = (l("rayon ")?, l("hauteur ")?);
        return pose(
            "Le volume d'un cylindre",
            "\\mathcal{V} = \\pi r^2 h",
            format!(
                "\\mathcal{{V}} = \\pi \\times {}^2 \\times {}",
                nombre(r),
                nombre(h)
            ),
            PI * r * r * h,
        );
    }
    if bas.contains("volume du cône") {
        let (r, h) = (l("rayon ")?, l("hauteur ")?);
        return pose(
            "Le volume d'un cône",
            "\\mathcal{V} = \\dfrac{\\pi r^2 h}{3}",
            format!(
                "\\mathcal{{V}} = \\dfrac{{\\pi \\times {}^2 \\times {}}}{{3}}",
                nombre(r),
                nombre(h)
            ),
            PI * r * r * h / 3.0,
        );
    }
    if bas.contains("volume de la boule") {
        let r = l("rayon ")?;
        return pose(
            "Le volume d'une boule",
            "\\mathcal{V} = \\dfrac{4}{3}\\pi r^3",
            format!("\\mathcal{{V}} = \\dfrac{{4}}{{3}}\\pi \\times {}^3", nombre(r)),
            4.0 / 3.0 * PI * r * r * r,
        );
    }
    None
}

const LONGUEURS: &[(&str, f64)] = &[
    ("km", 1e3),
    ("hm", 1e2),
    ("dam", 1e1),
    ("dm", 1e-1),
    ("cm", 1e-2),
    ("mm", 1e-3),
    ("m", 1.0),
];

const MASSES: &[(&str, f64)] = &[
    ("t", 1e6),
    ("kg", 1e3),
    ("hg", 1e2),
    ("dag", 1e1),
    ("dg", 1e-1),
    ("cg", 1e-2),
    ("mg", 1e-3),
    ("g", 1.0),
];

const CAPACITES: &[(&str, f64)] = &[
    ("hL", 1e-1),
    ("daL", 1e-2),
    ("dL", 1e-4),
    ("cL", 1e-5),
    ("mL", 1e-6),
    ("L", 1e-3),
];

fn puissance(unite: &str) -> (String, u32) {
    let u = unite.trim();
    for (marque, n) in [("^3", 3u32), ("³", 3), ("^2", 2), ("²", 2)] {
        if let Some(base) = u.strip_suffix(marque) {
            return (base.to_string(), n);
        }
    }
    (u.to_string(), 1)
}

fn genre(unite: &str) -> Option<(char, f64)> {
    let (base, n) = puissance(unite);
    if let Some((_, f)) = CAPACITES.iter().find(|(nom, _)| *nom == base) {
        return Some(('v', *f));
    }
    if let Some((_, f)) = MASSES.iter().find(|(nom, _)| *nom == base) {
        return Some(('m', *f));
    }
    let (_, f) = LONGUEURS.iter().find(|(nom, _)| *nom == base)?;
    match n {
        1 => Some(('l', *f)),
        2 => Some(('a', f * f)),
        _ => Some(('v', f * f * f)),
    }
}

fn conversion(desc: &str) -> Option<String> {
    let (avant, apres) = desc.split_once(" en ")?;
    let avant = avant.trim();
    let arrivee = apres.trim().trim_end_matches('.').split(' ').next()?.trim();
    let coupe = avant.find(|c: char| c.is_alphabetic()).filter(|i| *i > 0)?;
    let quantite = reel(&avant[..coupe])?;
    let depart = avant[coupe..].trim();
    let (ga, fa) = genre(depart)?;
    let (gb, fb) = genre(arrivee)?;
    if ga != gb {
        return None;
    }
    let r = quantite * fa / fb;
    prose(&[format!(
        "\\[{}\\ \\mathrm{{{}}} = {}\\ \\mathrm{{{}}}\\]",
        nombre(quantite),
        depart,
        nombre(r),
        arrivee
    )])
}

fn deux_noms(desc: &str, cle: &str, env: &Env) -> Option<(String, String)> {
    let i = desc.to_lowercase().find(cle)? + cle.len();
    let mots: Vec<String> = desc[i..]
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|m| !m.is_empty())
        .map(|m| m.to_string())
        .filter(|m| env.objects.contains_key(m))
        .collect();
    if mots.len() < 2 {
        return None;
    }
    Some((mots[0].clone(), mots[1].clone()))
}

fn un_nom(desc: &str, cle: &str, env: &Env) -> Option<String> {
    let i = desc.to_lowercase().find(cle)? + cle.len();
    desc[i..]
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|m| !m.is_empty())
        .find(|m| env.objects.contains_key(*m))
        .map(|m| m.to_string())
}

fn demande(requete: serde_json::Value, env: &Env) -> Option<String> {
    let mut complete = requete;
    complete.as_object_mut()?.insert(
        "defs".into(),
        crate::langage::commandes::objects_json(&env.objects),
    );
    crate::python::pont::ask(&complete.to_string()).ok()
}

fn en_prose(reponse: String) -> Option<String> {
    let lignes: Vec<String> = reponse.lines().map(|l| l.to_string()).collect();
    prose(&lignes)
}

fn en_formule(reponse: String) -> Option<String> {
    Some(crate::layout::rendu::bloc_calcul(&reponse))
}

fn vecteurs(verbe: &str, desc: &str, bas: &str, env: &Env) -> Option<String> {
    if verbe == "Calcule" && bas.contains("produit scalaire") {
        let (u, v) = deux_noms(desc, "scalaire", env)?;
        return en_prose(demande(
            serde_json::json!({"op":"dot","args":{"u":u,"v":v}}),
            env,
        )?);
    }
    if verbe == "Calcule" && bas.contains("la norme") {
        let u = un_nom(desc, "norme", env)?;
        return en_formule(demande(
            serde_json::json!({"op":"norm","args":{"u":u}}),
            env,
        )?);
    }
    if verbe == "Calcule" && bas.contains("l'angle entre") {
        let (u, v) = deux_noms(desc, "entre", env)?;
        return en_prose(demande(
            serde_json::json!({"op":"angle_vect","args":{"u":u,"v":v}}),
            env,
        )?);
    }
    if verbe == "Étudie" && bas.contains("colinéarité") {
        let (u, v) = deux_noms(desc, "colinéarité", env)?;
        return en_prose(demande(
            serde_json::json!({"op":"collinear","args":{"u":u,"v":v}}),
            env,
        )?);
    }
    if verbe == "Calcule" && bas.contains("projeté orthogonal") {
        let (u, v) = deux_noms(desc, "orthogonal", env)?;
        return en_prose(demande(
            serde_json::json!({"op":"projection","args":{"u":u,"v":v}}),
            env,
        )?);
    }
    if verbe == "Orthonormalise" {
        let i = bas.find("famille")? + "famille".len();
        let noms: Vec<String> = desc[i..]
            .split(|c: char| !(c.is_alphanumeric() || c == '_'))
            .filter(|m| !m.is_empty() && env.objects.contains_key(*m))
            .map(|m| m.to_string())
            .collect();
        if noms.len() < 2 {
            return None;
        }
        return en_prose(demande(
            serde_json::json!({"op":"gram_schmidt","args":{"noms":noms}}),
            env,
        )?);
    }
    if verbe == "Donne" && bas.contains("vecteur normal") {
        let nom = un_nom(desc, "normal", env)?;
        return en_prose(demande(
            serde_json::json!({"op":"plan_normal","args":{"name":nom}}),
            env,
        )?);
    }
    if verbe == "Calcule" && bas.contains("distance du point") && bas.contains("plan") {
        let nom = un_nom(desc, "plan", env)?;
        let i = desc.find('(')?;
        let point: Vec<String> = desc[i + 1..]
            .split(')')
            .next()?
            .split(&[';', ','][..])
            .map(|c| c.trim().to_string())
            .collect();
        if point.len() != 3 {
            return None;
        }
        return en_prose(demande(
            serde_json::json!({"op":"plan_distance","args":{"name":nom,"point":point}}),
            env,
        )?);
    }
    if verbe == "Calcule" && bas.contains("racines") && bas.contains("de l'unité") {
        let n = rang(bas)?;
        return en_formule(demande(
            serde_json::json!({"op":"roots_unity","args":{"n":n}}),
            env,
        )?);
    }

    if verbe == "Calcule" && bas.contains("racines ") && bas.contains(" de ") {
        let n = rang(bas)?;
        let z = desc
            .split('%')
            .next()
            .unwrap_or("")
            .rsplit_once(" de ")
            .map(|(_, r)| r.trim().trim_end_matches('.').to_string())
            .filter(|z| !z.is_empty())?;
        return en_formule(demande(
            serde_json::json!({"op":"solve","args":{"expr":format!("x^{} = {}", n, z),"domain":"C"}}),
            env,
        )?);
    }
    None
}

fn rang(bas: &str) -> Option<u32> {
    for (mot, n) in [
        ("carrées", 2u32),
        ("cubiques", 3),
        ("quatrièmes", 4),
        ("cinquièmes", 5),
        ("sixièmes", 6),
        ("septièmes", 7),
        ("huitièmes", 8),
        ("neuvièmes", 9),
        ("dixièmes", 10),
    ] {
        if bas.contains(mot) {
            return Some(n);
        }
    }
    None
}

pub fn commande(verbe: &str, desc: &str, corps: Option<&str>, env: &mut Env) -> Option<String> {
    let bas = desc.to_lowercase();
    if verbe == "Construis" {
        return construis(desc, env);
    }

    if verbe == "Place" && bas.contains("point") {
        if let Some(poses) = points_par_distances(desc) {
            for (nom, p) in &poses {
                env.objects.insert(
                    nom.clone(),
                    crate::langage::commandes::Obj::Point {
                        coords: vec![texte_fr(p.0), texte_fr(p.1)],
                    },
                );
            }
            let coins: Vec<Plan2> = poses.iter().map(|(_, p)| *p).collect();
            let (_, _, y0, y1) = bornes(coins.iter());
            let hauteur = ((y1 - y0) * CM + 44.0).max(60.0);
            let v = vue_centimetres(&coins, hauteur);
            let centre = (
                coins.iter().map(|p| p.0).sum::<f64>() / coins.len() as f64,
                coins.iter().map(|p| p.1).sum::<f64>() / coins.len() as f64,
            );
            let mut dessin = String::new();
            for (nom, p) in &poses {
                let dehors = (p.0 - centre.0, p.1 - centre.1);
                dessin.push_str(&marque_point_vers(&v, *p, nom, "#1a4fa0", dehors));
            }
            return Some(crate::maths::trace::enveloppe_haute(&dessin, "#1a4fa0", hauteur));
        }
        let nom = nom_apres(desc, "point");
        if nom.is_empty() {
            return None;
        }
        if let Some((x, y)) = coords_inline(desc) {
            if point_declare(&nom, env).is_none() {
                env.objects.insert(
                    nom.clone(),
                    crate::langage::commandes::Obj::Point {
                        coords: vec![texte_fr(x), texte_fr(y)],
                    },
                );
            }
        }
        match coords_inline(desc).or_else(|| point_declare(&nom, env)) {
            Some(p) => {
                let hauteur = 40.0;
                let v =
                    vue_centimetres(&[(p.0 - 0.6, p.1 - 0.4), (p.0 + 0.6, p.1 + 0.4)], hauteur);
                let dessin = marque_point(&v, p, &nom, "#1a4fa0");
                return Some(crate::maths::trace::enveloppe_haute(&dessin, "#1a4fa0", hauteur));
            }
            None => {
                return Some(crate::utils::erreur::bloc(
                    &format!("<Place>{}", desc),
                    &format!(
                        "le point {} demande ses coordonnées — <Place>un point {}(x ; y)",
                        nom, nom
                    ),
                ))
            }
        }
    }
    if verbe == "Trace" && !bas.contains("dans un repère") {
        if let Some(html) = cercle_trigonometrique(desc) {
            return Some(html);
        }

        if bas.starts_with("le point") && corps.is_none() {
            let nom = nom_apres(desc, "point");

            if let Some((x, y)) = coords_inline(desc) {
                if !nom.is_empty() && point_declare(&nom, env).is_none() {
                    env.objects.insert(
                        nom.clone(),
                        crate::langage::commandes::Obj::Point {
                            coords: vec![texte_fr(x), texte_fr(y)],
                        },
                    );
                }
            }
            if let Some(p) = coords_inline(desc).or_else(|| point_declare(&nom, env)) {
                let hauteur = 40.0;
                let v = vue_centimetres(&[(p.0 - 0.6, p.1 - 0.4), (p.0 + 0.6, p.1 + 0.4)], hauteur);
                let dessin = marque_point(&v, p, &nom, "#1a4fa0");
                return Some(crate::maths::trace::enveloppe_haute(&dessin, "#1a4fa0", hauteur));
            }
            return Some(crate::utils::erreur::bloc(
                &format!("<Trace>{}", desc),
                &format!("le point {} n'a pas été déclaré — posez-le par <Soit>un point {}(x;y)", nom, nom),
            ));
        }

        if (bas.starts_with("le cercle") || bas.starts_with("le disque"))
            && bas.contains("de rayon ")
            && nombre_apres(desc, "de rayon ").is_none()
        {
            let reference: String = apres_cle(desc, "de rayon ")
                .unwrap_or("")
                .trim()
                .chars()
                .take_while(|c| c.is_alphabetic())
                .collect();
            let lettres: Vec<String> = reference.chars().map(|c| c.to_string()).collect();
            if lettres.len() == 2 {
                if let (Some(a), Some(b)) = (
                    point_declare(&lettres[0], env),
                    point_declare(&lettres[1], env),
                ) {
                    let rayon = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
                    let recrit = desc.replacen(
                        &format!("de rayon {}", reference),
                        &format!("de rayon {}", texte_fr(rayon)),
                        1,
                    );
                    if let Some(html) = figure_seule(&recrit) {
                        return Some(html);
                    }
                }
                return Some(crate::utils::erreur::bloc(
                    &format!("<Trace>{}", desc),
                    &format!(
                        "le rayon {} demande les points {} et {} déclarés par <Soit>",
                        reference, lettres[0], lettres[1]
                    ),
                ));
            }
        }

        if corps.is_none()
            && !bas.contains("tel que")
            && !bas.contains("telle que")
            && (bas.starts_with("la droite (")
                || bas.starts_with("la demi-droite")
                || bas.starts_with("le segment"))
        {
            let lettres: Vec<String> = desc
                .chars()
                .skip_while(|c| !"[(".contains(*c))
                .take_while(|c| !"])".contains(*c))
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_string())
                .collect();
            if lettres.len() == 2 {
                match (point_declare(&lettres[0], env), point_declare(&lettres[1], env)) {
                    (Some(pa), Some(pb)) => {
                        let marge = 0.8;
                        let cadre = [
                            (pa.0.min(pb.0) - marge, pa.1.min(pb.1) - marge),
                            (pa.0.max(pb.0) + marge, pa.1.max(pb.1) + marge),
                        ];
                        let hauteur = ((cadre[1].1 - cadre[0].1) * CM + 30.0).max(56.0);
                        let v = vue_centimetres(&cadre, hauteur);
                        let mut dessin = elements_places(desc, &v, env, Vec::new());
                        for (nom, p) in [(&lettres[0], pa), (&lettres[1], pb)] {
                            dessin.push_str(&marque_point(&v, p, nom, "#1a4fa0"));
                        }
                        if dessin.is_empty() {
                            return None;
                        }
                        return Some(crate::maths::trace::enveloppe_haute(
                            &dessin, "#1a4fa0", hauteur,
                        ));
                    }
                    _ => {
                        return Some(crate::utils::erreur::bloc(
                            &format!("<Trace>{}", desc),
                            &format!(
                                "les points {} et {} doivent être déclarés par <Soit>",
                                lettres[0], lettres[1]
                            ),
                        ))
                    }
                }
            }
        }
        if let Some(html) = figure_seule(desc) {
            return Some(html);
        }
        if let Some(bloc) = corps {
            collecte_figure(bloc, env);
            if let Some(html) = figure_libre(bloc, env) {
                return Some(html);
            }
        }
    }
    if let Some(html) = vecteurs(verbe, desc, &bas, env) {
        return Some(html);
    }
    match verbe {
        "Convertis" => conversion(desc),
        "Vérifie" if bas.contains("est rectangle") => reciproque_pythagore(desc),
        "Vérifie" if bas.contains("parallèles") => reciproque_thales(desc),
        "Calcule" if bas.contains("thalès") || bas.contains("thales") => thales(desc),
        "Calcule" if bas.starts_with("l'angle") && bas.contains("rectangle en") => {
            angle_cherche(desc)
        }
        "Calcule" if bas.contains("rectangle en") && bas.contains("l'angle") => {
            cote_par_angle(desc)
        }
        "Calcule" if bas.contains("rectangle en") => pythagore(desc),
        "Calcule"
            if bas.contains("périmètre") || bas.contains("l'aire du") || bas.contains("volume") =>
        {
            mesure_usuelle(desc)
        }
        _ => None,
    }
}

type Plan2 = (f64, f64);

pub(crate) fn collecte_figure(corps: &str, env: &mut Env) {
    for ligne in corps.lines() {
        let t = ligne.trim();
        let b = t.to_lowercase();
        if !(b.starts_with("le point") || b.starts_with("un point")) {
            continue;
        }
        let nom = nom_apres(t, "point");
        if nom.is_empty() || env.objects.contains_key(&nom) {
            continue;
        }
        if let Some((x, y)) = coords_inline(t) {
            env.objects.insert(
                nom,
                crate::langage::commandes::Obj::Point {
                    coords: vec![texte_fr(x), texte_fr(y)],
                },
            );
        }
    }
}

pub(crate) fn collecte_place(after: &str, env: &mut Env) {
    let desc = after.lines().next().unwrap_or("").trim();
    if !desc.to_lowercase().contains("point") {
        return;
    }
    if let Some(poses) = points_par_distances(desc) {
        for (nom, p) in poses {
            env.objects.insert(
                nom,
                crate::langage::commandes::Obj::Point {
                    coords: vec![texte_fr(p.0), texte_fr(p.1)],
                },
            );
        }
        return;
    }
    let nom = nom_apres(desc, "point");
    if nom.is_empty() {
        return;
    }
    if let Some((x, y)) = coords_inline(desc) {
        env.objects.insert(
            nom,
            crate::langage::commandes::Obj::Point {
                coords: vec![texte_fr(x), texte_fr(y)],
            },
        );
    }
}

fn point_declare(nom: &str, env: &Env) -> Option<Plan2> {
    match env.objects.get(nom) {
        Some(crate::langage::commandes::Obj::Point { coords })
        | Some(crate::langage::commandes::Obj::Vecteur { coords }) => {
            if coords.len() < 2 {
                return None;
            }
            Some((reel(&coords[0])?, reel(&coords[1])?))
        }
        _ => None,
    }
}

fn centre_nomme(desc: &str, cle: &str, env: &Env) -> Option<Plan2> {
    let i = desc.to_lowercase().find(cle)? + cle.len();
    let nom: String = desc[i..]
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if nom.is_empty() {
        return None;
    }
    Some(point_declare(&nom, env).unwrap_or((0.0, 0.0)))
}

fn sommets_vises(desc: &str, env: &Env) -> Option<(Vec<String>, Vec<Plan2>, bool)> {
    let bas = desc.to_lowercase();
    let (i, ferme) = if let Some(i) = bas.find("triangle ") {
        (i + "triangle ".len(), true)
    } else if let Some(i) = bas.find("polygone ") {
        (i + "polygone ".len(), true)
    } else if let Some(i) = bas.find("segment [") {
        (i + "segment [".len(), false)
    } else {
        return None;
    };
    let noms: Vec<String> = desc[i..]
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphabetic())
        .map(|c| c.to_string())
        .collect();
    if noms.len() < 2 {
        return None;
    }
    let mut sommets = Vec::new();
    for nom in &noms {
        sommets.push(point_declare(nom, env)?);
    }
    Some((noms, sommets, ferme))
}

fn reflechit(p: Plan2, a: Plan2, b: Plan2) -> Plan2 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let norme = dx * dx + dy * dy;
    if norme == 0.0 {
        return p;
    }
    let t = ((p.0 - a.0) * dx + (p.1 - a.1) * dy) / norme;
    let h = (a.0 + t * dx, a.1 + t * dy);
    (2.0 * h.0 - p.0, 2.0 * h.1 - p.1)
}

struct Transformation {
    nom: String,
    image: Box<dyn Fn(Plan2) -> Plan2>,
    repere: Vec<(Plan2, Plan2)>,
    centre: Option<Plan2>,
}

fn transformation(desc: &str, env: &Env) -> Option<Transformation> {
    let bas = desc.to_lowercase();
    if bas.contains("symétrie axiale") {
        if bas.contains("axe des abscisses") {
            return Some(Transformation {
                nom: "la symétrie d'axe \\((Ox)\\)".into(),
                image: Box::new(|p: Plan2| (p.0, -p.1)),
                repere: vec![((-1e3, 0.0), (1e3, 0.0))],
                centre: None,
            });
        }
        if bas.contains("axe des ordonnées") {
            return Some(Transformation {
                nom: "la symétrie d'axe \\((Oy)\\)".into(),
                image: Box::new(|p: Plan2| (-p.0, p.1)),
                repere: vec![((0.0, -1e3), (0.0, 1e3))],
                centre: None,
            });
        }
        let i = bas.find("d'axe ")? + "d'axe ".len();
        let j = desc[i..].find('(')? + i + 1;
        let lettres: Vec<String> = desc[j..]
            .chars()
            .take_while(|c| c.is_alphabetic())
            .map(|c| c.to_string())
            .collect();
        if lettres.len() != 2 {
            return None;
        }
        let a = point_declare(&lettres[0], env)?;
        let b = point_declare(&lettres[1], env)?;
        let (ax, bx) = (a, b);
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let loin = 1e3;
        return Some(Transformation {
            nom: format!("la symétrie d'axe \\(({}{})\\)", lettres[0], lettres[1]),
            image: Box::new(move |p: Plan2| reflechit(p, ax, bx)),
            repere: vec![(
                (a.0 - loin * dx, a.1 - loin * dy),
                (a.0 + loin * dx, a.1 + loin * dy),
            )],
            centre: None,
        });
    }
    if bas.contains("symétrie centrale") {
        let c = centre_nomme(desc, "de centre ", env)?;
        return Some(Transformation {
            nom: "la symétrie de centre \\(O\\)".into(),
            image: Box::new(move |p: Plan2| (2.0 * c.0 - p.0, 2.0 * c.1 - p.1)),
            repere: Vec::new(),
            centre: Some(c),
        });
    }
    if bas.contains("translation") {
        let i = bas.find("vecteur ")? + "vecteur ".len();
        let nom: String = desc[i..]
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let u = point_declare(&nom, env)?;
        return Some(Transformation {
            nom: format!("la translation de vecteur \\(\\vec{{{}}}\\)", nom),
            image: Box::new(move |p: Plan2| (p.0 + u.0, p.1 + u.1)),
            repere: Vec::new(),
            centre: None,
        });
    }
    if bas.contains("rotation") {
        let c = centre_nomme(desc, "de centre ", env)?;
        let angle = nombre_apres(desc, "d'angle ")?;
        let (s, k) = (angle.to_radians().sin(), angle.to_radians().cos());
        return Some(Transformation {
            nom: format!(
                "la rotation de centre \\(O\\) et d'angle \\({}^\\circ\\)",
                nombre(angle)
            ),
            image: Box::new(move |p: Plan2| {
                let (x, y) = (p.0 - c.0, p.1 - c.1);
                (c.0 + k * x - s * y, c.1 + s * x + k * y)
            }),
            repere: Vec::new(),
            centre: Some(c),
        });
    }
    if bas.contains("homothétie") {
        let c = centre_nomme(desc, "de centre ", env)?;
        let rapport = nombre_apres(desc, "de rapport ")?;
        return Some(Transformation {
            nom: format!(
                "l'homothétie de centre \\(O\\) et de rapport \\({}\\)",
                nombre(rapport)
            ),
            image: Box::new(move |p: Plan2| {
                (
                    c.0 + rapport * (p.0 - c.0),
                    c.1 + rapport * (p.1 - c.1),
                )
            }),
            repere: Vec::new(),
            centre: Some(c),
        });
    }
    None
}

fn bornes<'a>(points: impl Iterator<Item = &'a Plan2>) -> (f64, f64, f64, f64) {
    let mut x0 = f64::INFINITY;
    let mut x1 = f64::NEG_INFINITY;
    let mut y0 = f64::INFINITY;
    let mut y1 = f64::NEG_INFINITY;
    for (x, y) in points {
        x0 = x0.min(*x);
        x1 = x1.max(*x);
        y0 = y0.min(*y);
        y1 = y1.max(*y);
    }
    (x0, x1, y0, y1)
}

fn cadre(points: &[Plan2]) -> crate::maths::trace::Repere {
    let (x0, x1, y0, y1) = bornes(points.iter().chain([&(0.0, 0.0)]));
    let marge = ((x1 - x0).max(y1 - y0) * 0.2).max(1.0);
    crate::maths::trace::Repere::isotrope(
        (x0 - marge).floor(),
        (x1 + marge).ceil(),
        (y0 - marge).floor(),
        (y1 + marge).ceil(),
    )
}

fn trace_chemin(points: impl Iterator<Item = Plan2>, ferme: bool, couleur: &str) -> String {
    let mut d = String::new();
    let mut n = 0usize;
    for (i, (x, y)) in points.enumerate() {
        d.push_str(&format!("{}{:.2},{:.2} ", if i == 0 { "M" } else { "L" }, x, y));
        n = i + 1;
    }
    if ferme && n > 2 {
        d.push('Z');
    }
    format!(
        "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"0.5\"/>",
        d.trim(),
        couleur
    )
}

fn polygone(r: &crate::maths::trace::Repere, pts: &[Plan2], ferme: bool, couleur: &str) -> String {
    trace_chemin(pts.iter().map(|p| (r.px(p.0), r.py(p.1))), ferme, couleur)
}

fn etiquettes(
    v: &Projection,
    noms: &[String],
    pts: &[Plan2],
    prime: bool,
    couleur: &str,
) -> String {
    let centre = centre_de(pts);
    let mut s = String::new();
    for (nom, p) in noms.iter().zip(pts) {
        let etiquette = format!("{}{}", nom, if prime { "'" } else { "" });
        s.push_str(&marque_point_vers(
            v,
            *p,
            &etiquette,
            couleur,
            vers_dehors(*p, centre),
        ));
    }
    s
}

fn construis(desc: &str, env: &Env) -> Option<String> {
    let (noms, sommets, ferme) = sommets_vises(desc, env)?;
    let t = transformation(desc, env)?;
    let images: Vec<Plan2> = sommets.iter().map(|p| (t.image)(*p)).collect();

    let mut tous: Vec<Plan2> = sommets.clone();
    tous.extend(images.iter().copied());
    if let Some(c) = t.centre {
        tous.push(c);
    }
    let r = cadre(&tous);

    let mut corps = crate::maths::trace::axes(&r, "y");
    for (a, b) in &t.repere {
        corps.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" \
             stroke=\"#777\" stroke-width=\"0.35\" stroke-dasharray=\"2.2 1.4\"/>",
            r.px(a.0.clamp(r.x0, r.x1)),
            r.py(a.1.clamp(r.y0, r.y1)),
            r.px(b.0.clamp(r.x0, r.x1)),
            r.py(b.1.clamp(r.y0, r.y1))
        ));
    }
    if let Some(c) = t.centre {
        corps.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"0.7\" fill=\"#777\"/>",
            r.px(c.0),
            r.py(c.1)
        ));
    }
    corps.push_str(&polygone(&r, &sommets, ferme, "#1a4fa0"));
    corps.push_str(&polygone(&r, &images, ferme, "#c00"));
    let vue = crate::maths::trace::projection(&r);
    corps.push_str(&etiquettes(&vue, &noms, &sommets, false, "#1a4fa0"));
    corps.push_str(&etiquettes(&vue, &noms, &images, true, "#c00"));

    let figure = crate::maths::trace::enveloppe_haute(&corps, "#1a4fa0", r.hauteur);
    let liste = noms
        .iter()
        .zip(&images)
        .map(|(nom, p)| {
            format!(
                "\\({}'\\left({}\\ ;\\ {}\\right)\\)",
                nom,
                nombre(p.0),
                nombre(p.1)
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    let objet: String = noms.concat();
    let mut html = crate::maths::algebre::bloc_prose(&[format!(
        "L'image de \\({}\\) par {} a pour sommets {}.",
        objet, t.nom, liste
    )]);
    html.push_str(&figure);
    Some(html)
}

pub(crate) struct Projection {
    pub sx: f64,
    pub sy: f64,
    pub ox: f64,
    pub oy: f64,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
}

impl Projection {
    pub(crate) fn px(&self, x: f64) -> f64 {
        self.ox + self.sx * x
    }
    pub(crate) fn py(&self, y: f64) -> f64 {
        self.oy - self.sy * y
    }
    fn dedans(&self, p: Plan2) -> bool {
        p.0 >= self.x0 - 1e-9 && p.0 <= self.x1 + 1e-9 && p.1 >= self.y0 - 1e-9
            && p.1 <= self.y1 + 1e-9
    }
    fn coupe(&self, p: Plan2, d: Plan2, rayon: bool) -> Option<(Plan2, Plan2)> {
        let mut tmin: f64 = if rayon { 0.0 } else { -1e9 };
        let mut tmax: f64 = 1e9;
        let pentes = [-d.0, d.0, -d.1, d.1];
        let restes = [
            p.0 - self.x0,
            self.x1 - p.0,
            p.1 - self.y0,
            self.y1 - p.1,
        ];
        for (pente, reste) in pentes.iter().zip(restes.iter()) {
            if pente.abs() < 1e-12 {
                if *reste < 0.0 {
                    return None;
                }
                continue;
            }
            let t = reste / pente;
            if *pente < 0.0 {
                tmin = tmin.max(t);
            } else {
                tmax = tmax.min(t);
            }
        }
        if tmin >= tmax {
            return None;
        }
        Some((
            (p.0 + tmin * d.0, p.1 + tmin * d.1),
            (p.0 + tmax * d.0, p.1 + tmax * d.1),
        ))
    }
}

fn ligne(v: &Projection, a: Plan2, b: Plan2, couleur: &str, tirets: bool) -> String {
    format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" \
         stroke-width=\"0.45\"{}/>",
        v.px(a.0),
        v.py(a.1),
        v.px(b.0),
        v.py(b.1),
        couleur,
        if tirets {
            " stroke-dasharray=\"2 1.3\""
        } else {
            ""
        }
    )
}

fn nombre_apres_egal(source: &str) -> Option<f64> {
    let apres = source.trim_start().strip_prefix('=')?;
    let brut: String = apres
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ',' || *c == '.')
        .collect();
    if brut.is_empty() {
        return None;
    }
    brut.replace(',', ".").parse::<f64>().ok()
}

fn noms_des_points(desc: &str) -> Vec<String> {
    let bas = desc.to_lowercase();
    let debut = match bas.find("points") {
        Some(i) => i + "points".len(),
        None => return Vec::new(),
    };
    let fin = bas[debut..]
        .find("tels que")
        .or_else(|| bas[debut..].find("telles que"))
        .map(|k| debut + k)
        .unwrap_or(desc.len());
    desc[debut..fin]
        .replace(" et ", ",")
        .split(',')
        .map(|m| m.trim().to_string())
        .filter(|m| !m.is_empty() && m.chars().all(|c| c.is_alphanumeric()))
        .collect()
}

fn distances_declarees(desc: &str, noms: &[String]) -> Vec<(usize, usize, f64)> {
    let bas = desc.to_lowercase();
    let debut = bas
        .find("tels que")
        .map(|i| i + "tels que".len())
        .or_else(|| bas.find("telles que").map(|i| i + "telles que".len()));
    let Some(debut) = debut else {
        return Vec::new();
    };
    let zone = &desc[debut..];
    let mut trouvees = Vec::new();
    for (i, a) in noms.iter().enumerate() {
        for (j, b) in noms.iter().enumerate() {
            if i >= j {
                continue;
            }
            for paire in [format!("{}{}", a, b), format!("{}{}", b, a)] {
                let mut reste = zone;
                while let Some(k) = reste.find(&paire) {
                    let apres = &reste[k + paire.len()..];
                    if let Some(d) = nombre_apres_egal(apres) {
                        if d > 0.0 {
                            trouvees.push((i, j, d));
                        }
                        break;
                    }
                    reste = &reste[k + paire.len()..];
                }
            }
        }
    }
    trouvees
}

pub(crate) fn points_par_distances(desc: &str) -> Option<Vec<(String, Plan2)>> {
    let bas = desc.to_lowercase();
    if !bas.contains("points") || !(bas.contains("tels que") || bas.contains("telles que")) {
        return None;
    }
    let noms = noms_des_points(desc);
    if noms.len() < 2 {
        return None;
    }
    let liens = distances_declarees(desc, &noms);
    if liens.is_empty() {
        return None;
    }
    let entre = |i: usize, j: usize| -> Option<f64> {
        liens
            .iter()
            .find(|(a, b, _)| (*a == i && *b == j) || (*a == j && *b == i))
            .map(|(_, _, d)| *d)
    };
    let mut places: Vec<Option<Plan2>> = vec![None; noms.len()];
    places[0] = Some((0.0, 0.0));
    places[1] = Some((entre(0, 1)?, 0.0));
    for i in 2..noms.len() {
        let mut pose = None;
        'appuis: for j in 0..noms.len() {
            for k in 0..noms.len() {
                if j == k || j == i || k == i {
                    continue;
                }
                let (Some(pj), Some(pk)) = (places[j], places[k]) else {
                    continue;
                };
                let (Some(rj), Some(rk)) = (entre(i, j), entre(i, k)) else {
                    continue;
                };
                let d = ((pk.0 - pj.0).powi(2) + (pk.1 - pj.1).powi(2)).sqrt();
                if d < 1e-9 || d > rj + rk || d < (rj - rk).abs() {
                    continue;
                }
                let a = (rj * rj - rk * rk + d * d) / (2.0 * d);
                let h2 = rj * rj - a * a;
                if h2 < 0.0 {
                    continue;
                }
                let h = h2.sqrt();
                let ux = (pk.0 - pj.0) / d;
                let uy = (pk.1 - pj.1) / d;
                let base = (pj.0 + a * ux, pj.1 + a * uy);
                pose = Some((base.0 - h * uy, base.1 + h * ux));
                break 'appuis;
            }
        }
        places[i] = Some(pose?);
    }
    Some(
        noms.into_iter()
            .zip(places)
            .filter_map(|(n, p)| p.map(|p| (n, p)))
            .collect(),
    )
}

fn marque_point(v: &Projection, p: Plan2, nom: &str, couleur: &str) -> String {
    marque_point_vers(v, p, nom, couleur, (1.0, 1.0))
}

fn marque_point_vers(
    v: &Projection,
    p: Plan2,
    nom: &str,
    couleur: &str,
    dehors: Plan2,
) -> String {
    let n = (dehors.0 * dehors.0 + dehors.1 * dehors.1).sqrt();
    let (ux, uy) = if n < 1e-9 {
        (std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2)
    } else {
        (dehors.0 / n, dehors.1 / n)
    };
    let ancre = if ux > 0.35 {
        "start"
    } else if ux < -0.35 {
        "end"
    } else {
        "middle"
    };
    let dy = if uy > 0.35 {
        -2.2
    } else if uy < -0.35 {
        3.6
    } else {
        1.2
    };
    let mut sortie = format!(
        "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"0.7\" fill=\"{}\"/>",
        v.px(p.0),
        v.py(p.1),
        couleur
    );
    if !nom.is_empty() {
        sortie.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\" fill=\"{}\" \
             text-anchor=\"{}\">{}</text>",
            v.px(p.0) + 2.4 * ux,
            v.py(p.1) + dy,
            couleur,
            ancre,
            nom
        ));
    }
    sortie
}

fn vers_dehors(p: Plan2, centre: Plan2) -> Plan2 {
    (p.0 - centre.0, p.1 - centre.1)
}

fn centre_de(points: &[Plan2]) -> Plan2 {
    let n = points.len().max(1) as f64;
    (
        points.iter().map(|p| p.0).sum::<f64>() / n,
        points.iter().map(|p| p.1).sum::<f64>() / n,
    )
}

fn fleche(v: &Projection, a: Plan2, b: Plan2, nom: &str, couleur: &str) -> String {
    let (ax, ay) = (v.px(a.0), v.py(a.1));
    let (bx, by) = (v.px(b.0), v.py(b.1));
    let mut s = format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" \
         stroke-width=\"0.5\" marker-end=\"url(#pointe)\"/>",
        ax, ay, bx, by, couleur
    );
    let (dx, dy) = (bx - ax, by - ay);
    let n = (dx * dx + dy * dy).sqrt();
    if !nom.is_empty() && n > 1e-9 {
        let mut angle = dy.atan2(dx).to_degrees();
        if angle >= 90.0 {
            angle -= 180.0;
        } else if angle < -90.0 {
            angle += 180.0;
        }
        let x = ax + 0.62 * dx + 2.8 * dy / n;
        let y = ay + 0.62 * dy - 2.8 * dx / n;
        s.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\" fill=\"{}\" text-anchor=\"middle\" \
             dominant-baseline=\"central\" transform=\"rotate({:.2} {:.2} {:.2})\">{}</text>",
            x, y, couleur, angle, x, y, nom
        ));
    }
    s
}

fn arc_angle(v: &Projection, sommet: Plan2, a: Plan2, b: Plan2, couleur: &str) -> String {
    let unit = |p: Plan2| {
        let (dx, dy) = (p.0 - sommet.0, p.1 - sommet.1);
        let n = (dx * dx + dy * dy).sqrt();
        if n == 0.0 {
            (0.0, 0.0)
        } else {
            (dx / n, dy / n)
        }
    };
    let (ua, ub) = (unit(a), unit(b));
    let rayon = 5.0;
    let p1 = (v.px(sommet.0) + rayon * ua.0, v.py(sommet.1) - rayon * ua.1);
    let p2 = (v.px(sommet.0) + rayon * ub.0, v.py(sommet.1) - rayon * ub.1);
    let sens = if ua.0 * ub.1 - ua.1 * ub.0 > 0.0 { 0 } else { 1 };
    format!(
        "<path d=\"M{:.2},{:.2} A{:.2},{:.2} 0 0 {} {:.2},{:.2}\" fill=\"none\" \
         stroke=\"{}\" stroke-width=\"0.4\"/>",
        p1.0, p1.1, rayon, rayon, sens, p2.0, p2.1, couleur
    )
}

fn cercle(v: &Projection, centre: Plan2, rayon: f64, couleur: &str, plein: bool) -> String {
    format!(
        "<ellipse cx=\"{:.2}\" cy=\"{:.2}\" rx=\"{:.2}\" ry=\"{:.2}\" \
         fill=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
        v.px(centre.0),
        v.py(centre.1),
        rayon * v.sx,
        rayon * v.sy,
        if plein { couleur } else { "none" },
        couleur
    )
}

fn point_nomme(nom: &str, locaux: &[(String, Plan2)], env: &Env) -> Option<Plan2> {
    locaux
        .iter()
        .find(|(n, _)| n == nom)
        .map(|(_, p)| *p)
        .or_else(|| point_declare(nom, env))
}

fn trois_points(
    lettres: &[String],
    locaux: &[(String, Plan2)],
    env: &Env,
) -> Option<(Plan2, Plan2, Plan2)> {
    if lettres.len() != 3 {
        return None;
    }
    Some((
        point_nomme(&lettres[0], locaux, env)?,
        point_nomme(&lettres[1], locaux, env)?,
        point_nomme(&lettres[2], locaux, env)?,
    ))
}

fn lettres_apres(texte: &str) -> Vec<String> {
    texte
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphabetic())
        .map(|c| c.to_string())
        .collect()
}

fn deux_points(ligne: &str, ouvrants: &[char], env: &Env, locaux: &[(String, Plan2)]) -> Option<(String, Plan2, String, Plan2)> {
    let i = ligne.find(|c: char| ouvrants.contains(&c))?;
    let lettres: Vec<String> = ligne[i + 1..]
        .chars()
        .take_while(|c| c.is_alphabetic())
        .map(|c| c.to_string())
        .collect();
    if lettres.len() != 2 {
        return None;
    }
    Some((
        lettres[0].clone(),
        point_nomme(&lettres[0], locaux, env)?,
        lettres[1].clone(),
        point_nomme(&lettres[1], locaux, env)?,
    ))
}

fn coords_inline(ligne: &str) -> Option<Plan2> {
    let i = ligne.find('(')?;
    let dedans = ligne[i + 1..].split(')').next()?;
    let mut it = dedans.split(&[';', ','][..]);
    Some((reel(it.next()?)?, reel(it.next()?)?))
}

pub(crate) fn elements(corps: &str, v: &Projection, env: &Env) -> String {
    elements_places(corps, v, env, Vec::new())
}

fn texte_fr(v: f64) -> String {
    nombre(v).replace("{,}", ",")
}

fn elements_places(
    corps: &str,
    v: &Projection,
    env: &Env,
    depart: Vec<(String, Plan2)>,
) -> String {
    const TRAIT: &str = "#1a4fa0";
    const AIDE: &str = "#c00";
    let mut s = String::new();
    let mut locaux: Vec<(String, Plan2)> = depart;
    let mut vecteurs: Vec<(String, Plan2)> = Vec::new();

    for brut in corps.lines() {
        let l = brut.trim();
        if l.is_empty() {
            continue;
        }
        let bas = l.to_lowercase();

        if bas.starts_with("le point") || bas.starts_with("un point") {
            let nom = nom_apres(l, "point");
            if let Some(p) = coords_inline(l).or_else(|| point_declare(&nom, env)) {
                locaux.push((nom.clone(), p));
                s.push_str(&marque_point(v, p, &nom, TRAIT));
            }
            continue;
        }

        if bas.starts_with("le cercle") || bas.starts_with("le disque") {
            let plein = bas.starts_with("le disque");
            let cle = if plein { "disque" } else { "cercle" };
            let nom = nom_apres(l, cle);
            let centre = coords_inline(l).unwrap_or((0.0, 0.0));
            if let Some(rayon) = nombre_apres(l, "rayon ") {
                s.push_str(&cercle(v, centre, rayon, TRAIT, plein));
                if !nom.is_empty() {
                    s.push_str(&format!(
                        "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">{}</text>",
                        v.px(centre.0 + 0.72 * rayon),
                        v.py(centre.1 + 0.72 * rayon),
                        nom
                    ));
                }
            }
            continue;
        }

        if bas.starts_with("le vecteur") {
            let reste = apres_cle(l, "vecteur").unwrap_or("").trim();
            let corps = reste.split("depuis").next().unwrap_or(reste).trim();
            let jetons: Vec<&str> = corps.split_whitespace().collect();
            let expression = jetons.iter().any(|j| *j == "+" || *j == "-");
            let nom: String = corps.chars().take_while(|c| c.is_alphanumeric()).collect();
            let depuis = bas
                .split_once("depuis la pointe de ")
                .and_then(|(_, r)| r.split_whitespace().next())
                .and_then(|n| vecteurs.iter().find(|(m, _)| m == n).map(|(_, p)| *p))
                .unwrap_or((0.0, 0.0));
            let composantes = if expression {
                let mut total = (0.0, 0.0);
                let mut signe = 1.0;
                for jeton in &jetons {
                    match *jeton {
                        "+" => signe = 1.0,
                        "-" => signe = -1.0,
                        autre => {
                            if let Some((_, p)) = vecteurs.iter().find(|(m, _)| m == autre) {
                                total = (total.0 + signe * p.0, total.1 + signe * p.1);
                            }
                        }
                    }
                }
                Some(total)
            } else {
                coords_inline(l).or_else(|| {
                    vecteurs
                        .iter()
                        .find(|(m, _)| *m == nom)
                        .map(|(_, p)| *p)
                        .or_else(|| point_declare(&nom, env))
                })
            };
            let etiquette = if expression {
                jetons
                    .iter()
                    .map(|jeton| if *jeton == "-" { "\u{2212}" } else { *jeton })
                    .collect::<Vec<&str>>()
                    .join(" ")
            } else {
                nom.clone()
            };
            if let Some(c) = composantes {
                let arrivee = (depuis.0 + c.0, depuis.1 + c.1);
                let couleur = if depuis == (0.0, 0.0) { TRAIT } else { AIDE };
                s.push_str(&fleche(v, depuis, arrivee, &etiquette, couleur));
                if !expression && !vecteurs.iter().any(|(m, _)| *m == nom) {
                    vecteurs.push((nom.clone(), c));
                }
            }
            continue;
        }

        if bas.starts_with("la médiatrice") {
            if let Some((_, a, _, b)) = deux_points(l, &['['], env, &locaux) {
                let milieu = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
                let d = (-(b.1 - a.1), b.0 - a.0);
                if let Some((p, q)) = v.coupe(milieu, d, false) {
                    s.push_str(&ligne(v, p, q, AIDE, false));
                }
            }
            continue;
        }

        if bas.starts_with("la bissectrice") {
            let i = bas.find("angle ").map(|i| i + "angle ".len());
            if let Some(i) = i {
                let lettres = lettres_apres(&l[i..]);
                if let Some((b, sommet, c)) = trois_points(&lettres, &locaux, env) {
                    let (ub, uc) = (direction_unitaire(sommet, b), direction_unitaire(sommet, c));
                    let d = (ub.0 + uc.0, ub.1 + uc.1);
                    if let Some((p, q)) = v.coupe(sommet, d, true) {
                        s.push_str(&ligne(v, p, q, AIDE, false));
                    }
                }
            }
            continue;
        }

        if bas.starts_with("l'angle") {
            let lettres = lettres_apres(apres_cle(l, "angle").unwrap_or(""));
            if let Some((a, sommet, c)) = trois_points(&lettres, &locaux, env) {
                s.push_str(&arc_angle(v, sommet, a, c, AIDE));
            }
            continue;
        }

        if bas.starts_with("la droite") && l.contains("y =") {
            if let Some((m, b_)) = pente_ordonnee(l) {
                if let Some((p, q)) = v.coupe((0.0, b_), (1.0, m), false) {
                    s.push_str(&ligne(v, p, q, TRAIT, false));
                }
            }
            continue;
        }

        if bas.starts_with("la région") {
            if let Some((m, b_)) = pente_ordonnee(l) {
                let dessus = l.contains('>');
                let large = l.contains(">=") || l.contains("<=") || l.contains('⩾') || l.contains('⩽');
                let (ya, yb) = (m * v.x0 + b_, m * v.x1 + b_);
                let bord = if dessus { v.y1 } else { v.y0 };
                s.push_str(&format!(
                    "<polygon points=\"{:.2},{:.2} {:.2},{:.2} {:.2},{:.2} {:.2},{:.2}\" fill=\"#1a4fa0\" fill-opacity=\"0.12\" stroke=\"none\"/>",
                    v.px(v.x0), v.py(ya.clamp(v.y0, v.y1)),
                    v.px(v.x1), v.py(yb.clamp(v.y0, v.y1)),
                    v.px(v.x1), v.py(bord),
                    v.px(v.x0), v.py(bord)
                ));
                if let Some((p, q)) = v.coupe((0.0, b_), (1.0, m), false) {
                    s.push_str(&format!(
                        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#1a4fa0\" stroke-width=\"1.2\"{}/>",
                        v.px(p.0), v.py(p.1), v.px(q.0), v.py(q.1),
                        if large { "" } else { " stroke-dasharray=\"5 4\"" }
                    ));
                }
            }
            continue;
        }
        let segment = bas.starts_with("le segment");
        let demi = bas.starts_with("la demi-droite");
        let droite = bas.starts_with("la droite");
        if segment || demi || droite {
            if let Some((na, a, nb, b)) = deux_points(l, &['[', '('], env, &locaux) {
                let d = (b.0 - a.0, b.1 - a.1);
                let trace = if segment {
                    Some((a, b))
                } else if droite {
                    v.coupe(a, d, false)
                } else if l.contains('[') && l.find('[') < l.find(')') {
                    v.coupe(a, d, true)
                } else {
                    v.coupe(b, (-d.0, -d.1), true)
                };
                if let Some((p, q)) = trace {
                    s.push_str(&ligne(v, p, q, TRAIT, false));
                }
                for (nom, p) in [(na, a), (nb, b)] {
                    if v.dedans(p) && !locaux.iter().any(|(n, _)| *n == nom) {
                        s.push_str(&marque_point(v, p, &nom, TRAIT));
                        locaux.push((nom, p));
                    }
                }
            }
            continue;
        }
    }
    s
}

const CM: f64 = 12.5;

fn vue_centimetres(points: &[Plan2], hauteur: f64) -> Projection {
    let (x0, x1, y0, y1) = bornes(points.iter());
    Projection {
        sx: CM,
        sy: CM,
        ox: 75.0 - CM * (x0 + x1) / 2.0,
        oy: hauteur / 2.0 + CM * (y0 + y1) / 2.0,
        x0: x0 - 2.0,
        x1: x1 + 2.0,
        y0: y0 - 2.0,
        y1: y1 + 2.0,
    }
}

fn marque_cote(v: &Projection, a: Plan2, b: Plan2, traits: usize) -> String {
    let milieu = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let n = (dx * dx + dy * dy).sqrt();
    if n == 0.0 {
        return String::new();
    }
    let (ux, uy) = (dx / n, dy / n);
    let mut s = String::new();
    for k in 0..traits {
        let decalage = (k as f64 - (traits as f64 - 1.0) / 2.0) * 0.12;
        let base = (milieu.0 + decalage * ux, milieu.1 + decalage * uy);
        s.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#1a4fa0\" \
             stroke-width=\"0.4\"/>",
            v.px(base.0) + 1.6 * uy,
            v.py(base.1) + 1.6 * ux,
            v.px(base.0) - 1.6 * uy,
            v.py(base.1) - 1.6 * ux
        ));
    }
    s
}

fn direction_unitaire(depuis: Plan2, vers: Plan2) -> Plan2 {
    let (dx, dy) = (vers.0 - depuis.0, vers.1 - depuis.1);
    let n = (dx * dx + dy * dy).sqrt();
    (dx / n, dy / n)
}

fn marque_droit(v: &Projection, sommet: Plan2, a: Plan2, b: Plan2) -> String {
    let (ua, ub) = (direction_unitaire(sommet, a), direction_unitaire(sommet, b));
    let c = 3.0;
    let (sx, sy) = (v.px(sommet.0), v.py(sommet.1));
    format!(
        "<path d=\"M{:.2},{:.2} L{:.2},{:.2} L{:.2},{:.2}\" fill=\"none\" stroke=\"#1a4fa0\" \
         stroke-width=\"0.4\"/>",
        sx + c * ua.0,
        sy - c * ua.1,
        sx + c * (ua.0 + ub.0),
        sy - c * (ua.1 + ub.1),
        sx + c * ub.0,
        sy - c * ub.1
    )
}

fn cote_mesuree(v: &Projection, a: Plan2, b: Plan2, longueur: f64) -> String {
    cote_ecran(v.px(a.0), v.py(a.1), v.px(b.0), v.py(b.1), longueur, 0.5)
}

pub(crate) fn cote_ecran(ax: f64, ay: f64, bx: f64, by: f64, longueur: f64, t: f64) -> String {
    let (dx, dy) = (bx - ax, by - ay);
    let n = (dx * dx + dy * dy).sqrt();
    if n < 1e-9 {
        return String::new();
    }
    let mut angle = dy.atan2(dx).to_degrees();
    if angle >= 90.0 {
        angle -= 180.0;
    } else if angle < -90.0 {
        angle += 180.0;
    }
    let x = ax + t * dx - 3.2 * dy / n;
    let y = ay + t * dy + 3.2 * dx / n;
    format!(
        "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\" dominant-baseline=\"central\" \
         transform=\"rotate({:.2} {:.2} {:.2})\">{} cm</text>",
        x,
        y,
        angle,
        x,
        y,
        texte_fr(longueur)
    )
}

fn pente_ordonnee(l: &str) -> Option<(f64, f64)> {

    let i = l.find('y')?;
    let apres_y = l[i + 1..].trim_start();
    let apres = apres_y
        .strip_prefix(">=")
        .or_else(|| apres_y.strip_prefix("<="))
        .or_else(|| apres_y.strip_prefix('='))
        .or_else(|| apres_y.strip_prefix('>'))
        .or_else(|| apres_y.strip_prefix('<'))?
        .trim();
    let fin = apres
        .find(|c: char| c == ',' && !apres.starts_with(','))
        .map(|_| apres.len())
        .unwrap_or(apres.len());
    let expr: String = apres[..fin]
        .chars()
        .take_while(|c| *c != '%')
        .collect::<String>()
        .split(|c: char| c == '>' || c == '<')
        .next()
        .unwrap_or("")
        .replace(' ', "");

    if let Some(i) = expr.find('x') {
        let coeff = &expr[..i];
        let m = match coeff {
            "" | "+" => 1.0,
            "-" => -1.0,
            autre => reel(autre)?,
        };
        let reste = &expr[i + 1..];
        let b = if reste.is_empty() { 0.0 } else { reel(reste)? };
        Some((m, b))
    } else {
        Some((0.0, reel(&expr)?))
    }
}

fn sommets_nommes(desc: &str, cle: &str) -> Vec<String> {
    let bas = desc.to_lowercase();
    match bas.find(cle) {
        Some(i) => desc[i + cle.len()..]
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphabetic())
            .map(|c| c.to_string())
            .collect(),
        None => Vec::new(),
    }
}

fn polygone_regle(desc: &str) -> Option<(Vec<String>, Vec<Plan2>, f64, bool)> {
    let bas = desc.to_lowercase();

    if bas.contains("triangle") && bas.contains("rectangle en ") {
        let noms = sommets_nommes(desc, "triangle ");
        if noms.len() == 3 {
            let sommet_droit = bas
                .split_once("rectangle en ")
                .and_then(|(_, r)| r.trim_start().chars().next())
                .map(|c| c.to_uppercase().to_string())?;
            let mut cotes: Vec<(String, f64)> = Vec::new();
            let mut reste = desc;
            while let Some(i) = reste.to_lowercase().find("de côté ") {
                let apres = &reste[i + "de côté ".len()..];
                let nom_cote: String = apres
                    .trim_start()
                    .chars()
                    .take_while(|c| c.is_alphabetic())
                    .collect();
                if let Some(longueur) =
                    nombre_apres(apres, &format!("{} ", nom_cote.to_lowercase()))
                {
                    cotes.push((nom_cote, longueur));
                }
                reste = apres;
            }
            let autre = |c: &str| -> Option<f64> {
                cotes
                    .iter()
                    .find(|(n, _)| n.contains(c) && n.contains(&sommet_droit))
                    .map(|(_, l)| *l)
            };
            let voisins: Vec<&String> =
                noms.iter().filter(|n| **n != sommet_droit).collect();
            if voisins.len() == 2 {
                let l1 = autre(voisins[0])?;
                let l2 = autre(voisins[1])?;

                let place = |nom: &String| -> Plan2 {
                    if *nom == sommet_droit {
                        (0.0, 0.0)
                    } else if nom == voisins[0] {
                        (l1, 0.0)
                    } else {
                        (0.0, l2)
                    }
                };
                let sommets: Vec<Plan2> = noms.iter().map(place).collect();
                return Some((noms, sommets, 0.0, false));
            }
        }
        return None;
    }

    if bas.contains("triangle") && bas.contains("isocèle en ") {
        let noms = sommets_nommes(desc, "triangle ");
        let cote = nombre_apres(desc, "de côté ")?;
        if noms.len() == 3 {
            let sommet = bas
                .split_once("isocèle en ")
                .and_then(|(_, r)| r.trim_start().chars().next())
                .map(|c| c.to_uppercase().to_string())?;
            let base = nombre_apres(desc, "de base ").unwrap_or(cote * 0.75);
            let h = (cote * cote - base * base / 4.0).max(0.01).sqrt();
            let autres: Vec<&String> = noms.iter().filter(|n| **n != sommet).collect();
            let place = |nom: &String| -> Plan2 {
                if *nom == sommet {
                    (base / 2.0, h)
                } else if autres.first().map(|a| *a == nom).unwrap_or(false) {
                    (0.0, 0.0)
                } else {
                    (base, 0.0)
                }
            };
            let sommets: Vec<Plan2> = noms.iter().map(place).collect();
            return Some((noms, sommets, 0.0, false));
        }
        return None;
    }
    let cote = nombre_apres(desc, "de côté ")?;
    if bas.contains("équilatéral") || bas.contains("equilateral") {
        let noms = sommets_nommes(desc, "triangle ");
        let h = cote * 3.0_f64.sqrt() / 2.0;
        return Some((
            noms,
            vec![(0.0, 0.0), (cote, 0.0), (cote / 2.0, h)],
            cote,
            false,
        ));
    }
    if bas.contains("carré") || bas.contains("carre") {
        let noms = sommets_nommes(desc, "carré ");
        return Some((
            noms,
            vec![
                (0.0, 0.0),
                (cote, 0.0),
                (cote, cote),
                (0.0, cote),
            ],
            cote,
            true,
        ));
    }
    if bas.contains("losange") {
        let noms = sommets_nommes(desc, "losange ");
        let angle = nombre_apres(desc, "d'angle ").unwrap_or(60.0).to_radians();
        let (c, s) = (angle.cos(), angle.sin());
        return Some((
            noms,
            vec![
                (0.0, 0.0),
                (cote, 0.0),
                (cote + cote * c, cote * s),
                (cote * c, cote * s),
            ],
            cote,
            false,
        ));
    }
    None
}

fn figure_seule(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();

    if bas.starts_with("le segment") || bas.starts_with("la demi-droite") {
        let demi = bas.starts_with("la demi-droite");
        let longueur = nombre_apres(desc, "= ")?;
        let noms: Vec<String> = desc
            .chars()
            .skip_while(|c| !"[(".contains(*c))
            .skip(1)
            .take_while(|c| !"])".contains(*c))
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_string())
            .collect();
        if noms.len() != 2 {
            return None;
        }
        let a = (0.0, 0.0);
        let b = (longueur, 0.0);
        let bout = if demi { longueur * 1.35 } else { longueur };
        let hauteur = 46.0;
        let v = vue_centimetres(&[a, (bout, 0.0)], hauteur);
        let corps = format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#1a4fa0\" stroke-width=\"1.2\"/>",
            v.px(a.0),
            v.py(a.1),
            v.px(bout),
            v.py(0.0),
        );
        let mut corps = corps;
        for (nom, p) in [(&noms[0], a), (&noms[1], b)] {
            corps.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#1a4fa0\" stroke-width=\"1.2\"/>",
                v.px(p.0), v.py(p.1) - 4.0, v.px(p.0), v.py(p.1) + 4.0
            ));
            corps.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\" text-anchor=\"middle\">{}</text>",
                v.px(p.0), v.py(p.1) - 9.0, nom
            ));
        }
        corps.push_str(&cote_mesuree(&v, a, b, longueur));
        return Some(crate::maths::trace::enveloppe_haute(&corps, "#1a4fa0", hauteur));
    }

    if bas.starts_with("le cercle") || bas.starts_with("le disque") {

        let rayon = nombre_apres(desc, "de rayon ")
            .or_else(|| nombre_apres(desc, "de diamètre ").map(|d| d / 2.0))?;
        let centre = coords_inline(desc).unwrap_or((0.0, 0.0));
        let hauteur = 2.2 * rayon * CM + 12.0;
        let v = vue_centimetres(
            &[
                (centre.0 - rayon, centre.1 - rayon),
                (centre.0 + rayon, centre.1 + rayon),
            ],
            hauteur,
        );
        let plein = bas.starts_with("le disque");
        let col = if plein { "#a8d8f0" } else { "#1a4fa0" };
        let nom = nom_apres(desc, if plein { "disque" } else { "cercle" });
        let mut corps = cercle(&v, centre, rayon, col, plein);
        corps.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"0.6\" fill=\"#1a4fa0\"/>",
            v.px(centre.0),
            v.py(centre.1)
        ));
        if !nom.is_empty() {
            corps.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\">{}</text>",
                v.px(centre.0) + 0.75 * rayon * CM,
                v.py(centre.1 + rayon) + 0.75 * rayon * CM * 0.35,
                nom
            ));
        }
        corps.push_str(&cote_mesuree(
            &v,
            centre,
            (centre.0 + rayon, centre.1),
            rayon,
        ));
        return Some(crate::maths::trace::enveloppe_haute(&corps, "#1a4fa0", hauteur));
    }

    let (noms, sommets, cote, angles_droits) = polygone_regle(desc)?;
    let marques = bas.contains("avec les marques");
    let hauteur = {
        let haut = sommets.iter().fold(f64::NEG_INFINITY, |m, p| m.max(p.1));
        let bas_y = sommets.iter().fold(f64::INFINITY, |m, p| m.min(p.1));
        (haut - bas_y) * CM + 22.0
    };
    let v = vue_centimetres(&sommets, hauteur);
    let mut corps = trace_chemin(
        sommets.iter().map(|p| (v.px(p.0), v.py(p.1))),
        true,
        "#1a4fa0",
    );
    let centre = centre_de(&sommets);
    for (i, p) in sommets.iter().enumerate() {
        let nom = noms.get(i).cloned().unwrap_or_default();
        corps.push_str(&marque_point_vers(
            &v,
            *p,
            &nom,
            "#1a4fa0",
            vers_dehors(*p, centre),
        ));
        if angles_droits {
            let avant = sommets[(i + sommets.len() - 1) % sommets.len()];
            let apres = sommets[(i + 1) % sommets.len()];
            corps.push_str(&marque_droit(&v, *p, avant, apres));
        }
    }
    for i in 0..sommets.len() {
        let a = sommets[i];
        let b = sommets[(i + 1) % sommets.len()];
        if marques {
            corps.push_str(&marque_cote(&v, a, b, 1));
        }
        let longueur = if cote > 0.0 {
            cote
        } else {
            ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
        };
        corps.push_str(&cote_mesuree(&v, a, b, longueur));
    }
    Some(crate::maths::trace::enveloppe_haute(&corps, "#1a4fa0", hauteur))
}

fn figure_libre(corps: &str, env: &Env) -> Option<String> {
    let mut noms = Vec::new();
    let mut sommets = Vec::new();
    let mut cote = 0.0;
    let mut droits = false;
    for l in corps.lines() {
        if let Some((n, s, c, d)) = polygone_regle(l) {
            noms = n;
            sommets = s;
            cote = c;
            droits = d;
            break;
        }
    }
    if sommets.len() < 3 || noms.len() < sommets.len() {

        let points: Vec<Plan2> = corps
            .lines()
            .filter_map(|l| coords_inline(l.trim()).or_else(|| {
                let t = l.trim().to_lowercase();
                if t.starts_with("le point") || t.starts_with("un point") {
                    point_declare(&nom_apres(l.trim(), "point"), env)
                } else {
                    None
                }
            }))
            .collect();
        if points.len() < 2 {
            return None;
        }
        let haut = points.iter().fold(f64::NEG_INFINITY, |m, p| m.max(p.1));
        let bas_ = points.iter().fold(f64::INFINITY, |m, p| m.min(p.1));
        let hauteur = ((haut - bas_) * CM + 44.0).max(60.0);
        let v = vue_centimetres(&points, hauteur);
        let dessin = elements_places(corps, &v, env, Vec::new());
        if dessin.is_empty() {
            return None;
        }
        return Some(crate::maths::trace::enveloppe_haute(&dessin, "#1a4fa0", hauteur));
    }
    let haut = sommets.iter().fold(f64::NEG_INFINITY, |m, p| m.max(p.1));
    let bas = sommets.iter().fold(f64::INFINITY, |m, p| m.min(p.1));
    let hauteur = (haut - bas) * CM + 34.0;
    let v = vue_centimetres(&sommets, hauteur);
    let mut dessin = trace_chemin(
        sommets.iter().map(|p| (v.px(p.0), v.py(p.1))),
        true,
        "#1a4fa0",
    );
    let places: Vec<(String, Plan2)> = noms.iter().cloned().zip(sommets.iter().copied()).collect();
    let centre = centre_de(&sommets);
    for (nom, p) in &places {
        dessin.push_str(&marque_point_vers(
            &v,
            *p,
            nom,
            "#1a4fa0",
            vers_dehors(*p, centre),
        ));
    }
    if droits {
        for (i, p) in sommets.iter().enumerate() {
            let avant = sommets[(i + sommets.len() - 1) % sommets.len()];
            let apres = sommets[(i + 1) % sommets.len()];
            dessin.push_str(&marque_droit(&v, *p, avant, apres));
        }
    }
    for i in 0..sommets.len() {
        let a = sommets[i];
        let b = sommets[(i + 1) % sommets.len()];

        let longueur = if cote > 0.0 {
            cote
        } else {
            ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
        };
        dessin.push_str(&cote_mesuree(&v, a, b, longueur));
    }
    dessin.push_str(&elements_places(corps, &v, env, places));
    Some(crate::maths::trace::enveloppe_haute(&dessin, "#1a4fa0", hauteur))
}

fn cercle_trigonometrique(desc: &str) -> Option<String> {
    if !desc.to_lowercase().contains("cercle trigonométrique") {
        return None;
    }
    let hauteur = 104.0;
    let v = Projection {
        sx: 36.0,
        sy: 36.0,
        ox: 75.0,
        oy: hauteur / 2.0,
        x0: -1.4,
        x1: 1.4,
        y0: -1.4,
        y1: 1.4,
    };
    let mut s = String::new();
    s.push_str(&ligne(&v, (-1.32, 0.0), (1.32, 0.0), "#333", false));
    s.push_str(&ligne(&v, (0.0, -1.32), (0.0, 1.32), "#333", false));
    s.push_str(&cercle(&v, (0.0, 0.0), 1.0, "#1a4fa0", false));
    let avec = desc.to_lowercase().contains("avec les valeurs");

    let angles: &[(i32, i32)] = &[
        (0, 1),
        (1, 6),
        (1, 4),
        (1, 3),
        (1, 2),
        (2, 3),
        (3, 4),
        (5, 6),
        (1, 1),
        (-1, 6),
        (-1, 4),
        (-1, 3),
        (-1, 2),
        (-2, 3),
        (-3, 4),
        (-5, 6),
    ];
    for (num, den) in angles {
        let angle = std::f64::consts::PI * *num as f64 / *den as f64;
        let (x, y) = (angle.cos(), angle.sin());
        s.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"0.8\" fill=\"#1a4fa0\"/>",
            v.px(x),
            v.py(y)
        ));
        if !avec {
            continue;
        }
        let texte = match (*num, *den) {
            (0, _) => "0".to_string(),
            (1, 1) => "\u{3c0}".to_string(),
            (n, d) => {
                let signe = if n < 0 { "-" } else { "" };
                let haut = if n.abs() == 1 {
                    "\u{3c0}".to_string()
                } else {
                    format!("{}\u{3c0}", n.abs())
                };
                format!("{}{}/{}", signe, haut, d)
            }
        };
        s.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>",
            v.px(1.2 * x),
            v.py(1.2 * y) + 1.1,
            texte
        ));
    }

    if avec {
        let remarquables: &[(i32, &str, &str, f64)] = &[
            (6, "\u{221a}3/2", "1/2", 0.0),
            (4, "\u{221a}2/2", "\u{221a}2/2", 4.0),
            (3, "1/2", "\u{221a}3/2", 0.0),
        ];
        for (den, cosinus, sinus, decalage) in remarquables {
            let angle = std::f64::consts::PI / *den as f64;
            let (x, y) = (angle.cos(), angle.sin());
            s.push_str(&ligne(&v, (x, y), (x, 0.0), "#777", true));
            s.push_str(&ligne(&v, (x, y), (0.0, y), "#777", true));
            s.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>",
                v.px(x),
                v.py(0.0) + 4.2 + decalage,
                cosinus
            ));
            s.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab droite\">{}</text>",
                v.px(0.0) - 1.6,
                v.py(y) + 1.1,
                sinus
            ));
        }
    }
    Some(crate::maths::trace::enveloppe_haute(&s, "#1a4fa0", hauteur))
}
