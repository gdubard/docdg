use crate::Env;

fn arrondi(v: f64, decimales: usize) -> String {
    let net = (v * 1e6).round() / 1e6;
    if (net - net.round()).abs() < 1e-9 {
        return format!("{}", net.round() as i64);
    }
    let texte = format!("{:.*}", decimales, net);
    let court = texte.trim_end_matches('0').trim_end_matches('.');
    if court.is_empty() || court == "-" || court == "-0" {
        return "0".to_string();
    }
    court.to_string()
}

fn nombre(v: f64) -> String {
    arrondi(v, 4).replace('.', "{,}")
}

fn reel(jeton: &str) -> Option<f64> {
    let net: String = jeton
        .trim()
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ',' || *c == '.' || *c == '-')
        .collect();
    net.replace(',', ".").parse::<f64>().ok()
}

fn nombres(source: &str) -> Vec<f64> {
    let mut valeurs = Vec::new();
    let mut courant = String::new();
    for c in source.chars() {

        if c.is_ascii_digit() || c == ',' || c == '.' || (c == '-' && courant.is_empty()) {
            courant.push(c);
        } else {
            if let Some(v) = reel(&courant) {
                valeurs.push(v);
            }
            courant.clear();
        }
    }
    if let Some(v) = reel(&courant) {
        valeurs.push(v);
    }
    valeurs
}

fn serie(desc: &str, apres: &str) -> Vec<f64> {
    match desc.to_lowercase().find(apres) {
        Some(i) => nombres(&desc[i + apres.len()..]),
        None => Vec::new(),
    }
}

fn liste_texte(valeurs: &[f64]) -> String {
    valeurs
        .iter()
        .map(|v| nombre(*v).replace("{,}", ","))
        .collect::<Vec<String>>()
        .join(" ; ")
}

fn rang(n: usize) -> String {
    format!("{}\u{1d49}", n)
}

fn triee(valeurs: &[f64]) -> Vec<f64> {
    let mut t = valeurs.to_vec();
    t.sort_by(|a, b| a.total_cmp(b));
    t
}

fn prose(lignes: &[String]) -> Option<String> {
    Some(crate::maths::algebre::bloc_prose(lignes))
}

fn valeurs_et_effectifs(desc: &str, bas: &str) -> Option<(Vec<f64>, Vec<f64>)> {
    let (valeurs, effectifs) = if bas.contains("effectifs") {
        let coupe = bas.find("et d'effectifs")?;
        (serie(&desc[..coupe], "de valeurs"), nombres(&desc[coupe..]))
    } else {
        let v = serie(desc, "la série");
        let n = v.len();
        (v, vec![1.0; n])
    };
    if valeurs.is_empty() || valeurs.len() != effectifs.len() {
        return None;
    }
    Some((valeurs, effectifs))
}

fn somme_ponderee(valeurs: &[f64], effectifs: &[f64]) -> (f64, f64) {
    let total: f64 = effectifs.iter().sum();
    let somme: f64 = valeurs.iter().zip(effectifs).map(|(v, n)| v * n).sum();
    (total, somme)
}

fn prologue_barres(maxi: f64, corps: &mut String) -> (f64, f64, f64) {
    corps.push_str(&axes(GAUCHE));
    let plafond = graduations_verticales(maxi, corps);
    (plafond, LARGE - GAUCHE - DROITE, HAUT - CIME - PIED)
}

fn moyenne(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    if bas.contains("effectifs") {
        let (valeurs, effectifs) = valeurs_et_effectifs(desc, &bas)?;
        let (total, somme) = somme_ponderee(&valeurs, &effectifs);
        let produits = valeurs
            .iter()
            .zip(&effectifs)
            .map(|(v, n)| format!("{} \\times {}", nombre(*v), nombre(*n)))
            .collect::<Vec<String>>()
            .join(" + ");
        return prose(&[format!(
            "La moyenne pondérée vaut \\(\\bar{{x}} = \\dfrac{{{}}}{{{}}} = \
             \\dfrac{{{}}}{{{}}} = {}\\).",
            produits,
            nombre(total),
            nombre(somme),
            nombre(total),
            nombre(somme / total)
        )]);
    }
    let valeurs = serie(desc, "la série");
    if valeurs.is_empty() {
        return None;
    }
    let somme: f64 = valeurs.iter().sum();
    let n = valeurs.len() as f64;
    prose(&[format!(
        "\\(\\bar{{x}} = \\dfrac{{{}}}{{{}}} = \\dfrac{{{}}}{{{}}} = {}\\).",
        valeurs
            .iter()
            .map(|v| nombre(*v))
            .collect::<Vec<String>>()
            .join(" + "),
        nombre(n),
        nombre(somme),
        nombre(n),
        nombre(somme / n)
    )])
}

fn mediane(desc: &str) -> Option<String> {
    let valeurs = serie(desc, "la série");
    if valeurs.is_empty() {
        return None;
    }
    let t = triee(&valeurs);
    let n = t.len();
    let mut lignes = vec![format!(
        "On range la série dans l'ordre croissant : {} ({} valeurs).",
        liste_texte(&t),
        n
    )];
    if n % 2 == 0 {
        let (a, b) = (t[n / 2 - 1], t[n / 2]);
        lignes.push(format!(
            "L'effectif est pair : la médiane est la demi-somme des {} et {} valeurs, \
             \\(m = \\dfrac{{{} + {}}}{{2}} = {}\\).",
            rang(n / 2),
            rang(n / 2 + 1),
            nombre(a),
            nombre(b),
            nombre((a + b) / 2.0)
        ));
    } else {
        lignes.push(format!(
            "L'effectif est impair : la médiane est la {} valeur, \\(m = {}\\).",
            rang(n / 2 + 1),
            nombre(t[n / 2])
        ));
    }
    prose(&lignes)
}

fn etendue(desc: &str) -> Option<String> {
    let valeurs = serie(desc, "la série");
    if valeurs.is_empty() {
        return None;
    }
    let t = triee(&valeurs);
    let (mini, maxi) = (t[0], t[t.len() - 1]);
    prose(&[format!(
        "L'étendue est l'écart entre la plus grande et la plus petite valeur : \
         \\(e = {} - {} = {}\\).",
        nombre(maxi),
        nombre(mini),
        nombre(maxi - mini)
    )])
}

fn quartiles(desc: &str) -> Option<String> {
    let valeurs = serie(desc, "la série");
    if valeurs.is_empty() {
        return None;
    }
    let t = triee(&valeurs);
    let n = t.len();
    let premier = ((n as f64) / 4.0).ceil().max(1.0) as usize;
    let troisieme = ((3.0 * n as f64) / 4.0).ceil().max(1.0) as usize;
    prose(&[
        format!("Série rangée : {} ({} valeurs).", liste_texte(&t), n),
        format!(
            "Le premier quartile est la {} valeur (\\(\\lceil n/4 \\rceil\\)) : \\(Q_1 = {}\\) ; \
             le troisième est la {} (\\(\\lceil 3n/4 \\rceil\\)) : \\(Q_3 = {}\\).",
            rang(premier),
            nombre(t[premier - 1]),
            rang(troisieme),
            nombre(t[troisieme - 1])
        ),
    ])
}

fn quatrieme_proportionnelle(desc: &str) -> Option<String> {
    let v = nombres(desc);
    if v.len() < 3 {
        return None;
    }
    let (a, b, c) = (v[0], v[1], v[2]);
    if a == 0.0 {
        return None;
    }
    prose(&[format!(
        "On cherche \\(x\\) tel que \\(\\dfrac{{{}}}{{{}}} = \\dfrac{{{}}}{{x}}\\). \
         Le produit en croix donne \\({} \\times x = {} \\times {}\\), d'où \
         \\(x = \\dfrac{{{} \\times {}}}{{{}}} = {}\\).",
        nombre(a),
        nombre(b),
        nombre(c),
        nombre(a),
        nombre(b),
        nombre(c),
        nombre(b),
        nombre(c),
        nombre(a),
        nombre(b * c / a)
    )])
}

fn deux_lignes(corps: &str) -> Option<(Vec<String>, Vec<String>)> {
    let lignes: Vec<&str> = corps
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && *l != "{" && *l != "}")
        .collect();
    if lignes.len() < 2 {
        return None;
    }
    let cases = |l: &str| -> Vec<String> {
        l.split(';').map(|c| c.trim().to_string()).collect()
    };
    let haut = cases(lignes[0]);
    let bas = cases(lignes[1]);
    if haut.len() != bas.len() || haut.is_empty() {
        return None;
    }
    Some((haut, bas))
}

fn verifie_proportionnalite(corps: Option<&str>) -> Option<String> {
    let (haut, bas) = deux_lignes(corps?)?;
    let mut rapports = Vec::new();
    for (h, b) in haut.iter().zip(&bas) {
        let (x, y) = (reel(h)?, reel(b)?);
        if x == 0.0 {
            return None;
        }
        rapports.push((x, y, y / x));
    }
    let reference = rapports[0].2;
    let egaux = rapports.iter().all(|(_, _, r)| (r - reference).abs() < 1e-9);
    let detail = rapports
        .iter()
        .map(|(x, y, r)| {
            format!(
                "\\(\\dfrac{{{}}}{{{}}} = {}\\)",
                nombre(*y),
                nombre(*x),
                nombre(*r)
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    let conclusion = if egaux {
        format!(
            "Tous les rapports sont égaux : le tableau est un tableau de proportionnalité, \
             de coefficient \\({}\\).",
            nombre(reference)
        )
    } else {
        "Les rapports ne sont pas tous égaux : ce n'est pas un tableau de proportionnalité."
            .to_string()
    };
    prose(&[
        format!("On compare les rapports colonne par colonne : {}.", detail),
        conclusion,
    ])
}

fn complete_proportionnalite(corps: Option<&str>) -> Option<String> {
    let (haut, bas) = deux_lignes(corps?)?;
    let mut coefficient = None;
    for (h, b) in haut.iter().zip(&bas) {
        if let (Some(x), Some(y)) = (reel(h), reel(b)) {
            if x != 0.0 {
                coefficient = Some(y / x);
                break;
            }
        }
    }
    let k = coefficient?;
    let mut ligne_haut = Vec::new();
    let mut ligne_bas = Vec::new();
    for (h, b) in haut.iter().zip(&bas) {
        match (reel(h), reel(b)) {
            (Some(x), Some(y)) => {
                ligne_haut.push((nombre(x), false));
                ligne_bas.push((nombre(y), false));
            }
            (Some(x), None) => {
                ligne_haut.push((nombre(x), false));
                ligne_bas.push((nombre(x * k), true));
            }
            (None, Some(y)) => {
                ligne_haut.push((nombre(y / k), true));
                ligne_bas.push((nombre(y), false));
            }
            (None, None) => return None,
        }
    }
    Some(table_proportion(&ligne_haut, &ligne_bas, k))
}

fn table_proportion(haut: &[(String, bool)], bas: &[(String, bool)], k: f64) -> String {
    let n = haut.len();
    if n == 0 {
        return String::new();
    }
    let (x0, x1) = (32.0, 118.0);
    let (y0, y1, y2) = (10.0, 24.0, 38.0);
    let pas = (x1 - x0) / n as f64;
    let mut s = format!(
        "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" \
         stroke=\"{}\" stroke-width=\"0.5\"/>\
         <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" \
         stroke-width=\"0.5\"/>",
        x0, y0, x1 - x0, y2 - y0, TRAIT, x0, y1, x1, y1, TRAIT
    );
    for i in 1..n {
        let x = x0 + pas * i as f64;
        s.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" \
             stroke-width=\"0.5\"/>",
            x, y0, x, y2, TRAIT
        ));
    }
    for (i, (dessus, dessous)) in haut.iter().zip(bas).enumerate() {
        let x = x0 + pas * (i as f64 + 0.5);
        for (valeur, complete, y) in [
            (&dessus.0, dessus.1, (y0 + y1) / 2.0 + 1.2),
            (&dessous.0, dessous.1, (y1 + y2) / 2.0 + 1.2),
        ] {
            s.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\"{}>{}</text>",
                x,
                y,
                if complete { " font-weight=\"bold\"" } else { "" },
                valeur.replace("{,}", ",")
            ));
        }
    }
    let milieu = (y0 + y2) / 2.0;
    for (bord, controle, depart, arrivee, centre, operateur) in [
        (x0 - 3.0, x0 - 24.0, y0 + 3.0, y2 - 3.0, 18.5, "×"),
        (x1 + 3.0, x1 + 24.0, y2 - 3.0, y0 + 3.0, 131.5, "÷"),
    ] {
        s.push_str(&format!(
            "<path d=\"M{:.2},{:.2} Q{:.2},{:.2} {:.2},{:.2}\" fill=\"none\" stroke=\"{}\" \
             stroke-width=\"0.45\" marker-end=\"url(#pointe)\"/>\
             <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"6\" fill=\"#fff\" stroke=\"{}\" \
             stroke-width=\"0.45\"/>\
             <text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\" fill=\"{}\">{}{}</text>",
            bord, depart, controle, milieu, bord, arrivee, AIDE,
            centre, milieu, AIDE,
            centre, milieu + 1.2, AIDE, operateur, nombre(k).replace("{,}", ",")
        ));
    }
    crate::maths::trace::enveloppe_haute(&s, AIDE, 46.0)
}

fn pourcentage_de(desc: &str) -> Option<String> {
    let v = nombres(desc);
    if v.len() < 2 {
        return None;
    }
    let (taux, base) = (v[0], v[1]);
    prose(&[format!(
        "\\({}\\,\\%\\) de \\({}\\) : \\(\\dfrac{{{}}}{{100}} \\times {} = {}\\).",
        nombre(taux),
        nombre(base),
        nombre(taux),
        nombre(base),
        nombre(taux * base / 100.0)
    )])
}

fn evolution(desc: &str, hausse: bool) -> Option<String> {
    let v = nombres(desc);
    if v.len() < 2 {
        return None;
    }
    let (taux, base) = (v[0], v[1]);
    let facteur = if hausse {
        1.0 + taux / 100.0
    } else {
        1.0 - taux / 100.0
    };
    let mot = if hausse { "augmentation" } else { "diminution" };
    let signe = if hausse { "+" } else { "-" };
    prose(&[format!(
        "Une {} de \\({}\\,\\%\\) revient à multiplier par \
         \\(1 {} \\dfrac{{{}}}{{100}} = {}\\). Donc \\({} \\times {} = {}\\).",
        mot,
        nombre(taux),
        signe,
        nombre(taux),
        nombre(facteur),
        nombre(base),
        nombre(facteur),
        nombre(base * facteur)
    )])
}

fn taux_evolution(desc: &str) -> Option<String> {
    let v = nombres(desc);
    if v.len() < 2 {
        return None;
    }
    let (debut, fin) = (v[0], v[1]);
    if debut == 0.0 {
        return None;
    }
    let taux = (fin - debut) / debut * 100.0;
    let mot = if taux >= 0.0 {
        "une augmentation"
    } else {
        "une diminution"
    };
    prose(&[format!(
        "\\(t = \\dfrac{{\\text{{valeur finale}} - \\text{{valeur initiale}}}}\
         {{\\text{{valeur initiale}}}} \\times 100 = \\dfrac{{{} - {}}}{{{}}} \\times 100 = {}\\) : \
         {} de \\({}\\,\\%\\).",
        nombre(fin),
        nombre(debut),
        nombre(debut),
        nombre(taux),
        mot,
        nombre(taux.abs())
    )])
}

fn en_centimetres(valeur: f64, unite: &str) -> Option<f64> {
    match unite {
        "mm" => Some(valeur / 10.0),
        "cm" => Some(valeur),
        "dm" => Some(valeur * 10.0),
        "m" => Some(valeur * 100.0),
        "km" => Some(valeur * 100_000.0),
        _ => None,
    }
}

fn unite_apres(desc: &str, depart: usize) -> Option<String> {
    desc[depart..]
        .split_whitespace()
        .find(|m| en_centimetres(1.0, &m.to_lowercase()).is_some())
        .map(|m| m.to_lowercase())
}

fn echelle(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let coupe = bas.find("représentent")?;
    let plan = nombres(&desc[..coupe]);
    let reel_valeurs = nombres(&desc[coupe..]);
    let sur_plan = *plan.last()?;
    let sur_terrain = *reel_valeurs.first()?;
    let unite_plan = unite_apres(desc, 0)?;
    let unite_terrain = unite_apres(desc, coupe)?;
    let a = en_centimetres(sur_plan, &unite_plan)?;
    let b = en_centimetres(sur_terrain, &unite_terrain)?;
    if a == 0.0 {
        return None;
    }
    let denominateur = b / a;
    prose(&[format!(
        "L'échelle est le rapport \\(\\dfrac{{\\text{{distance sur le plan}}}}\
         {{\\text{{distance réelle}}}}\\), les deux dans la même unité : \
         \\(\\dfrac{{{}\\text{{ cm}}}}{{{}\\text{{ cm}}}} = \\dfrac{{1}}{{{}}}\\) — \
         un centimètre sur le plan représente \\({}\\) cm dans la réalité.",
        nombre(a),
        nombre(b),
        nombre(denominateur),
        nombre(denominateur)
    )])
}

fn duree_heures(desc: &str) -> Option<(f64, String)> {
    let bas = desc.to_lowercase();
    let position = bas.find(" h")?;
    let avant = nombres(&desc[..position]);
    let heures = *avant.last()?;
    let apres = nombres(&desc[position + 2..]);
    let minutes = if bas[position..].contains("min") {
        apres.first().copied().unwrap_or(0.0)
    } else {
        0.0
    };
    let total = heures + minutes / 60.0;
    let ecrit = if minutes > 0.0 {
        format!(
            "{}\\text{{ h }}{}\\text{{ min}}",
            nombre(heures),
            nombre(minutes)
        )
    } else {
        format!("{}\\text{{ h}}", nombre(heures))
    };
    Some((total, ecrit))
}

fn heures_minutes(valeur: f64) -> String {
    let heures = valeur.floor();
    let minutes = ((valeur - heures) * 60.0).round();
    if minutes == 0.0 {
        format!("{}\\text{{ h}}", nombre(heures))
    } else {
        format!(
            "{}\\text{{ h }}{}\\text{{ min}}",
            nombre(heures),
            nombre(minutes)
        )
    }
}

fn vitesse(desc: &str) -> Option<String> {
    let (duree, ecrit) = duree_heures(desc)?;
    let bas = desc.to_lowercase();
    let coupe = bas.find(" km")?;
    let distance = *nombres(&desc[..coupe]).last()?;
    if duree == 0.0 {
        return None;
    }
    prose(&[
        format!(
            "On convertit la durée : \\({} = {}\\text{{ h}}\\). \
             La vitesse moyenne vaut \\(v = \\dfrac{{d}}{{t}}\\).",
            ecrit,
            nombre(duree)
        ),
        format!(
            "Ici, \\(v = \\dfrac{{{}\\text{{ km}}}}{{{}\\text{{ h}}}} = {}\\) km/h.",
            nombre(distance),
            nombre(duree),
            nombre(distance / duree)
        ),
    ])
}

fn distance_parcourue(desc: &str) -> Option<String> {
    let (duree, ecrit) = duree_heures(desc)?;
    let bas = desc.to_lowercase();
    let coupe = bas.find("km/h")?;
    let allure = *nombres(&desc[..coupe]).last()?;
    prose(&[
        format!(
            "On convertit la durée : \\({} = {}\\text{{ h}}\\). \
             La distance vaut \\(d = v \\times t\\).",
            ecrit,
            nombre(duree)
        ),
        format!(
            "Ici, \\(d = {} \\times {} = {}\\) km.",
            nombre(allure),
            nombre(duree),
            nombre(allure * duree)
        ),
    ])
}

fn duree_trajet(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let coupe = bas.find(" km")?;
    let distance = *nombres(&desc[..coupe]).last()?;
    let allure = *nombres(&desc[coupe..]).first()?;
    if allure == 0.0 {
        return None;
    }
    let heures = distance / allure;
    prose(&[
        "La durée vaut \\(t = \\dfrac{d}{v}\\).".to_string(),
        format!(
            "Ici, \\(t = \\dfrac{{{}}}{{{}}} = {}\\text{{ h}} = {}\\).",
            nombre(distance),
            nombre(allure),
            nombre(heures),
            heures_minutes(heures)
        ),
    ])
}

pub fn commande(verbe: &str, desc: &str, corps: Option<&str>, env: &mut Env) -> Option<String> {
    let bas = desc.to_lowercase();
    match verbe {
        "Calcule" if bas.contains("la probabilité que") => probabilite_loi(desc),
        "Détermine" if bas.contains("quantile") => quantile_loi(desc),
        "Calcule" if bas.contains("l'espérance de la loi") => moment_loi(desc, false),
        "Calcule" if bas.contains("l'écart type de la loi") => moment_loi(desc, true),
        "Calcule" if bas.contains("intervalle de fluctuation") => fluctuation(desc),
        "Applique" if bas.contains("tchebychev") => tchebychev(desc),
        "Calcule" if bas.contains("la variance de la série") => variance_serie(desc, false),
        "Calcule" if bas.contains("l'écart type de la série") => variance_serie(desc, true),
        "Calcule" if bas.contains("la covariance des séries") => covariance_series(desc),
        "Construis" if bas.contains("arbre") => arbre(corps?),
        "Dresse" if bas.contains("loi de probabilité") => loi_dressee(desc, corps),
        "Calcule" if bas.contains("l'espérance de") => moments_loi_x(desc, env, 0),
        "Calcule" if bas.contains("la variance de") => moments_loi_x(desc, env, 1),
        "Calcule" if bas.contains("l'écart type de") => moments_loi_x(desc, env, 2),
        "Calcule" if bas.contains("la moyenne") => moyenne(desc),
        "Calcule" if bas.contains("la médiane") => mediane(desc),
        "Calcule" if bas.contains("l'étendue") => etendue(desc),
        "Calcule" if bas.contains("les quartiles") => quartiles(desc),
        "Calcule" if bas.contains("quatrième proportionnelle") => {
            quatrieme_proportionnelle(desc)
        }
        "Calcule" if bas.contains("taux d'évolution") => taux_evolution(desc),
        "Calcule" if bas.contains("l'échelle") => echelle(desc),
        "Calcule" if bas.contains("la vitesse") => vitesse(desc),
        "Calcule" if bas.contains("la distance") => distance_parcourue(desc),
        "Calcule" if bas.contains("la durée") => duree_trajet(desc),
        "Calcule" if bas.contains('%') => pourcentage_de(desc),
        "Applique" if bas.contains("augmentation") => evolution(desc, true),
        "Applique" if bas.contains("diminution") => evolution(desc, false),
        "Vérifie" if bas.contains("proportionnalité") => verifie_proportionnalite(corps),
        "Complète" if bas.contains("proportionnalité") => complete_proportionnalite(corps),
        "Représente" if bas.contains("statistique") => {
            diagramme(desc, corps, env)
        }
        _ => None,
    }
}

const LARGE: f64 = 150.0;
const HAUT: f64 = 90.0;
const GAUCHE: f64 = 20.0;
const DROITE: f64 = 8.0;
const CIME: f64 = 8.0;
const PIED: f64 = 16.0;
const TRAIT: &str = "#1a4fa0";
const AIDE: &str = "#c00";
const PALETTE: [&str; 6] = ["#1a4fa0", "#c0392b", "#1e7d32", "#b8860b", "#6a1b9a", "#00838f"];

fn decimal(v: f64) -> String {
    arrondi(v, 4).replace('.', ",")
}

fn case(contenu: &str, italique: bool) -> String {
    format!(
        "<td style=\"border:0.3mm solid #1a3a6b;padding:1.2mm 2.4mm;text-align:center;{}\">{}</td>",
        if italique { "font-style:italic;" } else { "" },
        contenu
    )
}

fn tableau_borde(lignes: &[String]) -> String {
    let mut html = String::from(
        "<table class=\"tab\" style=\"border-collapse:collapse;width:auto;\
         border:0.3mm solid #1a3a6b;margin:0 0 1em;\">",
    );
    for ligne in lignes {
        html.push_str(&format!("<tr>{}</tr>", ligne));
    }
    html.push_str("</table>");
    html
}

fn accolade(desc: &str, apres: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let depart = bas.find(apres)? + apres.len();
    let ouvre = desc[depart..].find('{')? + depart;
    let ferme = desc[ouvre..].find('}')? + ouvre;
    Some(desc[ouvre + 1..ferme].to_string())
}

fn paires(bloc: &str) -> Vec<(String, f64)> {
    bloc.split(|c| c == '|' || c == '\n')
        .filter_map(|m| {
            let (nom, valeur) = m.split_once(':')?;
            Some((nom.trim().to_string(), reel(valeur)?))
        })
        .collect()
}

pub(crate) fn couples(bloc: &str) -> Vec<(f64, f64)> {
    let mut res = Vec::new();
    let mut reste = bloc;
    while let Some(o) = reste.find('(') {
        let f = match reste[o..].find(')') {
            Some(f) => o + f,
            None => break,
        };
        let v = nombres(&reste[o + 1..f]);
        if v.len() >= 2 {
            res.push((v[0], v[1]));
        }
        reste = &reste[f + 1..];
    }
    res
}

fn pas_joli(maxi: f64) -> f64 {
    if maxi <= 0.0 {
        return 1.0;
    }
    let brut = maxi / 4.0;
    let ordre = 10f64.powf(brut.log10().floor());
    for facteur in [1.0, 2.0, 2.5, 5.0] {
        if facteur * ordre >= brut {
            return facteur * ordre;
        }
    }
    10.0 * ordre
}

fn svg(corps: &str) -> String {
    crate::maths::trace::enveloppe_haute(corps, TRAIT, HAUT)
}

fn figure(html: String) -> Option<String> {
    if html.is_empty() {
        None
    } else {
        Some(html)
    }
}

fn graduation(tick: (f64, f64, f64, f64), texte: (f64, f64), valeur: &str, classe: &str) -> String {
    format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"grad\"/>\
         <text x=\"{:.2}\" y=\"{:.2}\" class=\"{}\">{}</text>",
        tick.0, tick.1, tick.2, tick.3, texte.0, texte.1, classe, valeur
    )
}

fn barre(x: f64, y: f64, largeur: f64, hauteur: f64) -> String {
    format!(
        "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\" \
         fill-opacity=\"0.75\" stroke=\"{}\" stroke-width=\"0.4\"/>",
        x, y, largeur, hauteur, TRAIT, TRAIT
    )
}

fn axes(gauche: f64) -> String {
    format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"axe\"/>\
         <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"axe\"/>",
        gauche,
        HAUT - PIED,
        LARGE - DROITE,
        HAUT - PIED,
        gauche,
        HAUT - PIED,
        gauche,
        CIME
    )
}

fn graduations_verticales(maxi: f64, corps: &mut String) -> f64 {
    let pas = pas_joli(maxi);
    let plafond = (maxi / pas).ceil().max(1.0) * pas;
    let hauteur = HAUT - CIME - PIED;
    let mut k = pas;
    while k <= plafond + 1e-9 {
        let y = HAUT - PIED - k / plafond * hauteur;
        corps.push_str(&graduation(
            (GAUCHE - 1.4, y, GAUCHE, y),
            (GAUCHE - 2.6, y + 1.1),
            &decimal(k),
            "lab droite",
        ));
        k += pas;
    }
    plafond
}

fn barres_verticales(donnees: &[(String, f64)]) -> String {
    let maxi = donnees.iter().map(|(_, v)| *v).fold(0.0, f64::max);
    let mut corps = String::new();
    let (plafond, large, hauteur) = prologue_barres(maxi, &mut corps);
    let pas = large / donnees.len() as f64;
    for (i, (nom, valeur)) in donnees.iter().enumerate() {
        let h = valeur / plafond * hauteur;
        let x = GAUCHE + pas * i as f64 + pas * 0.25;
        corps.push_str(&barre(x, HAUT - PIED - h, pas * 0.5, h));
        corps.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>",
            x + pas * 0.25,
            HAUT - PIED + 4.6,
            nom
        ));
    }
    svg(&corps)
}

fn barres_horizontales(donnees: &[(String, f64)]) -> String {
    let maxi = donnees.iter().map(|(_, v)| *v).fold(0.0, f64::max);
    let pas_grad = pas_joli(maxi);
    let plafond = (maxi / pas_grad).ceil().max(1.0) * pas_grad;
    let gauche = 28.0;
    let large = LARGE - gauche - DROITE;
    let hauteur = HAUT - CIME - PIED;
    let mut corps = axes(gauche);
    let mut k = 0.0;
    while k <= plafond + 1e-9 {
        let x = gauche + k / plafond * large;
        corps.push_str(&graduation(
            (x, HAUT - PIED, x, HAUT - PIED + 1.4),
            (x, HAUT - PIED + 4.6),
            &decimal(k),
            "lab",
        ));
        k += pas_grad;
    }
    let epaisseur = hauteur / donnees.len() as f64;
    for (i, (nom, valeur)) in donnees.iter().enumerate() {
        let l = valeur / plafond * large;
        let y = CIME + epaisseur * i as f64 + epaisseur * 0.25;
        corps.push_str(&barre(gauche, y, l, epaisseur * 0.5));
        corps.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab droite\">{}</text>",
            gauche - 2.0,
            y + epaisseur * 0.25 + 1.2,
            nom
        ));
    }
    svg(&corps)
}

fn camembert(donnees: &[(String, f64)]) -> String {
    let total: f64 = donnees.iter().map(|(_, v)| *v).sum();
    if total <= 0.0 {
        return String::new();
    }
    let (cx, cy, r) = (40.0, 45.0, 30.0);
    let mut corps = String::new();
    let mut angle = -std::f64::consts::FRAC_PI_2;
    for (i, (nom, valeur)) in donnees.iter().enumerate() {
        let part = valeur / total;
        let suivant = angle + part * std::f64::consts::TAU;
        let couleur = PALETTE[i % PALETTE.len()];
        let (x1, y1) = (cx + r * angle.cos(), cy + r * angle.sin());
        let (x2, y2) = (cx + r * suivant.cos(), cy + r * suivant.sin());
        let grand = if part > 0.5 { 1 } else { 0 };
        corps.push_str(&format!(
            "<path d=\"M{:.2},{:.2} L{:.2},{:.2} A{:.2},{:.2} 0 {} 1 {:.2},{:.2} Z\" \
             fill=\"{}\" fill-opacity=\"0.8\" stroke=\"#fff\" stroke-width=\"0.5\"/>",
            cx, cy, x1, y1, r, r, grand, x2, y2, couleur
        ));
        let ligne = CIME + 10.0 + 9.0 * i as f64;
        corps.push_str(&format!(
            "<rect x=\"82\" y=\"{:.2}\" width=\"4\" height=\"4\" fill=\"{}\" \
             fill-opacity=\"0.8\" stroke=\"#fff\" stroke-width=\"0.4\"/>\
             <text x=\"88\" y=\"{:.2}\" class=\"lab\" style=\"text-anchor:start\">{} ({} %)</text>",
            ligne - 3.2,
            couleur,
            ligne,
            nom,
            decimal(part * 100.0)
        ));
        angle = suivant;
    }
    svg(&corps)
}

fn histogramme(bornes: &[f64], effectifs: &[f64]) -> String {
    if bornes.len() != effectifs.len() + 1 || effectifs.is_empty() {
        return String::new();
    }
    let maxi = effectifs.iter().fold(0.0f64, |a, b| a.max(*b));
    let mut corps = String::new();
    let (plafond, large, hauteur) = prologue_barres(maxi, &mut corps);
    let (debut, fin) = (bornes[0], bornes[bornes.len() - 1]);
    if (fin - debut).abs() < 1e-12 {
        return String::new();
    }
    let px = |x: f64| GAUCHE + (x - debut) / (fin - debut) * large;
    for (i, effectif) in effectifs.iter().enumerate() {
        let h = effectif / plafond * hauteur;
        corps.push_str(&barre(
            px(bornes[i]),
            HAUT - PIED - h,
            px(bornes[i + 1]) - px(bornes[i]),
            h,
        ));
    }
    for b in bornes {
        corps.push_str(&format!(
            "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>",
            px(*b),
            HAUT - PIED + 4.6,
            decimal(*b)
        ));
    }
    svg(&corps)
}

fn boite(valeurs: &[f64]) -> String {
    let t = triee(valeurs);
    let n = t.len();
    if n < 3 {
        return String::new();
    }
    let indice = |f: f64| (((n as f64) * f).ceil().max(1.0) as usize).min(n) - 1;
    let (mini, maxi) = (t[0], t[n - 1]);
    let q1 = t[indice(0.25)];
    let q3 = t[indice(0.75)];
    let me = if n % 2 == 0 {
        (t[n / 2 - 1] + t[n / 2]) / 2.0
    } else {
        t[n / 2]
    };
    if (maxi - mini).abs() < 1e-12 {
        return String::new();
    }
    let large = LARGE - GAUCHE - DROITE;
    let marge = (maxi - mini) * 0.12;
    let (bas, haut) = (mini - marge, maxi + marge);
    let px = |x: f64| GAUCHE + (x - bas) / (haut - bas) * large;
    let axe = 30.0;
    let (sommet, base) = (8.0, 24.0);
    let milieu = (sommet + base) / 2.0;
    let mut corps = format!(
        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"axe\"/>",
        GAUCHE, axe, LARGE - DROITE, axe
    );
    for (borne, bord) in [(mini, q1), (maxi, q3)] {
        corps.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" \
             stroke-width=\"0.4\"/>\
             <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" \
             stroke-width=\"0.5\"/>",
            px(borne),
            milieu,
            px(bord),
            milieu,
            TRAIT,
            px(borne),
            sommet + 5.0,
            px(borne),
            base - 5.0,
            TRAIT
        ));
    }
    corps.push_str(&format!(
        "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\" \
         fill-opacity=\"0.3\" stroke=\"{}\" stroke-width=\"0.5\"/>\
         <line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"0.7\"/>",
        px(q1),
        sommet,
        px(q3) - px(q1),
        base - sommet,
        TRAIT,
        TRAIT,
        px(me),
        sommet,
        px(me),
        base,
        TRAIT
    ));
    for (valeur, etiquette) in [
        (mini, "min"),
        (q1, "Q1"),
        (me, "Me"),
        (q3, "Q3"),
        (maxi, "max"),
    ] {
        corps.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" class=\"grad\"/>\
             <text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>\
             <text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>",
            px(valeur),
            axe,
            px(valeur),
            axe + 1.4,
            px(valeur),
            axe + 4.6,
            decimal(valeur),
            px(valeur),
            axe + 9.4,
            etiquette
        ));
    }
    crate::maths::trace::enveloppe_haute(&corps, TRAIT, 44.0)
}

fn nuage(points: &[(f64, f64)], ajustement: bool) -> String {
    if points.len() < 2 {
        return String::new();
    }
    let xs: Vec<f64> = points.iter().map(|(x, _)| *x).collect();
    let ys: Vec<f64> = points.iter().map(|(_, y)| *y).collect();
    let xmin = xs.iter().cloned().fold(f64::MAX, f64::min);
    let xmax = xs.iter().cloned().fold(f64::MIN, f64::max);
    let ymax = ys.iter().cloned().fold(f64::MIN, f64::max);
    if (xmax - xmin).abs() < 1e-12 {
        return String::new();
    }
    let large = LARGE - GAUCHE - DROITE;
    let hauteur = HAUT - CIME - PIED;
    let marge = (xmax - xmin) * 0.12;
    let (bas, haut) = (xmin - marge, xmax + marge);
    let mut corps = String::new();
    corps.push_str(&axes(GAUCHE));
    let plafond = graduations_verticales(ymax, &mut corps);
    let px = |x: f64| GAUCHE + (x - bas) / (haut - bas) * large;
    let py = |y: f64| HAUT - PIED - y / plafond * hauteur;
    if ajustement {
        let n = points.len() as f64;
        let mx = xs.iter().sum::<f64>() / n;
        let my = ys.iter().sum::<f64>() / n;
        let numerateur: f64 = points.iter().map(|(x, y)| (x - mx) * (y - my)).sum();
        let denominateur: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
        if denominateur.abs() > 1e-12 {
            let a = numerateur / denominateur;
            let b = my - a * mx;
            corps.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#c00\" \
                 stroke-width=\"0.5\"/>",
                px(bas),
                py(a * bas + b),
                px(haut),
                py(a * haut + b)
            ));
        }
    }
    for (x, y) in points {
        corps.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"1\" class=\"point\"/>\
             <text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>",
            px(*x),
            py(*y),
            px(*x),
            HAUT - PIED + 4.6,
            decimal(*x)
        ));
    }
    svg(&corps)
}

fn diagramme(desc: &str, corps: Option<&str>, env: &Env) -> Option<String> {
    let bas = desc.to_lowercase();
    if bas.contains("histogramme") {
        let bornes = nombres(&accolade(desc, "les bornes")?);
        let effectifs = nombres(corps?);
        return figure(histogramme(&bornes, &effectifs));
    }
    let nomme = bas
        .find("les données")
        .map(|i| i + "les données".len())
        .and_then(|apres| desc[apres..].split_whitespace().next())
        .and_then(|nom| env.donnees.get(nom))
        .cloned();
    let bloc = match nomme {
        Some(b) => b,
        None => corps?.to_string(),
    };
    if bas.contains("moustaches") {
        return figure(boite(&nombres(&bloc)));
    }
    if bas.contains("nuage") {
        return figure(nuage(&couples(&bloc), bas.contains("ajustement")));
    }
    let donnees = paires(&bloc);
    if donnees.is_empty() {
        return None;
    }
    if bas.contains("camembert") {
        return figure(camembert(&donnees));
    }
    if bas.contains("horizontale") {
        return Some(barres_horizontales(&donnees));
    }
    Some(barres_verticales(&donnees))
}

fn approx(v: f64) -> String {
    arrondi(v, 3).replace('.', "{,}")
}

fn erf(x: f64) -> f64 {
    let signe = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let y = 1.0
        - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t
            * (-x * x).exp();
    signe * y
}

fn ln_gamma(x: f64) -> f64 {
    let coefficients = [
        76.180091729471457,
        -86.505320329416767,
        24.014098240830911,
        -1.231739572450155,
        0.001208650973866179,
        -0.000005395239384953,
    ];
    let mut y = x;
    let tmp = x + 5.5 - (x + 0.5) * (x + 5.5).ln();
    let mut serie = 1.000000000190015;
    for c in coefficients {
        y += 1.0;
        serie += c / y;
    }
    -tmp + (2.5066282746310005 * serie / x).ln()
}

fn gamma_inferieure(a: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x < a + 1.0 {
        let mut terme = 1.0 / a;
        let mut somme = terme;
        let mut n = a;
        for _ in 0..500 {
            n += 1.0;
            terme *= x / n;
            somme += terme;
            if terme.abs() < somme.abs() * 1e-14 {
                break;
            }
        }
        somme * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        let mut b = x + 1.0 - a;
        let mut c = 1e30;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..500 {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            if pas_lentz(an, b, &mut c, &mut d, &mut h) {
                break;
            }
        }
        1.0 - (-x + a * x.ln() - ln_gamma(a)).exp() * h
    }
}

fn pas_lentz(aa: f64, b: f64, c: &mut f64, d: &mut f64, h: &mut f64) -> bool {
    *d = b + aa * *d;
    if d.abs() < 1e-30 {
        *d = 1e-30;
    }
    *c = b + aa / *c;
    if c.abs() < 1e-30 {
        *c = 1e-30;
    }
    *d = 1.0 / *d;
    let delta = *d * *c;
    *h *= delta;
    (delta - 1.0).abs() < 1e-14
}

fn beta_continue(a: f64, b: f64, x: f64) -> f64 {
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < 1e-30 {
        d = 1e-30;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..300 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        pas_lentz(aa, 1.0, &mut c, &mut d, &mut h);
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        if pas_lentz(aa, 1.0, &mut c, &mut d, &mut h) {
            break;
        }
    }
    h
}

fn beta_incomplete(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let facteur = (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b)
        + a * x.ln()
        + b * (1.0 - x).ln())
    .exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        facteur * beta_continue(a, b, x) / a
    } else {
        1.0 - facteur * beta_continue(b, a, 1.0 - x) / b
    }
}

fn combinaison(n: f64, k: f64) -> f64 {
    (ln_gamma(n + 1.0) - ln_gamma(k + 1.0) - ln_gamma(n - k + 1.0)).exp()
}

fn masse(nom: &str, p: &[f64], k: f64) -> Option<f64> {
    match nom {
        "binomiale" => {
            let (n, q) = (*p.first()?, *p.get(1)?);
            if k < 0.0 || k > n {
                return Some(0.0);
            }
            Some(combinaison(n, k.round()) * q.powf(k.round()) * (1.0 - q).powf(n - k.round()))
        }
        "poisson" => {
            let l = *p.first()?;
            if k < 0.0 {
                return Some(0.0);
            }
            Some((-l + k.round() * l.ln() - ln_gamma(k.round() + 1.0)).exp())
        }
        _ => None,
    }
}

fn repartition(nom: &str, p: &[f64], x: f64) -> Option<f64> {
    match nom {
        "normale" | "normal" => {
            let (mu, sigma) = (*p.first()?, *p.get(1)?);
            Some(0.5 * (1.0 + erf((x - mu) / (sigma * std::f64::consts::SQRT_2))))
        }
        "uniforme" => {
            let (a, b) = (*p.first()?, *p.get(1)?);
            Some(((x - a) / (b - a)).clamp(0.0, 1.0))
        }
        "exponentielle" => {
            let l = *p.first()?;
            Some(if x <= 0.0 { 0.0 } else { 1.0 - (-l * x).exp() })
        }
        "student" => {
            let v = *p.first()?;
            let part = 0.5 * beta_incomplete(v / 2.0, 0.5, v / (v + x * x));
            Some(if x >= 0.0 { 1.0 - part } else { part })
        }
        "khi-deux" | "khideux" => {
            let k = *p.first()?;
            Some(gamma_inferieure(k / 2.0, x / 2.0))
        }
        "binomiale" | "poisson" => {
            let mut total = 0.0;
            let mut k = 0.0;
            while k <= x + 1e-9 {
                total += masse(nom, p, k)?;
                k += 1.0;
                if k > 100_000.0 {
                    break;
                }
            }
            Some(total.min(1.0))
        }
        _ => None,
    }
}

fn quantile(nom: &str, p: &[f64], ordre: f64) -> Option<f64> {
    if nom == "binomiale" || nom == "poisson" {
        let plafond = if nom == "binomiale" {
            *p.first()?
        } else {
            let l = *p.first()?;
            l + 40.0 * l.sqrt() + 100.0
        };
        let mut cumul = 0.0;
        let mut k = 0.0;
        while k <= plafond {
            cumul += masse(nom, p, k)?;
            if cumul >= ordre - 1e-12 {
                return Some(k);
            }
            k += 1.0;
        }
        return Some(plafond);
    }
    let (mut bas, mut haut) = (-1e4, 1e4);
    for _ in 0..80 {
        let milieu = (bas + haut) / 2.0;
        if repartition(nom, p, milieu)? < ordre {
            bas = milieu;
        } else {
            haut = milieu;
        }
    }
    Some((bas + haut) / 2.0)
}

fn moments(nom: &str, p: &[f64]) -> Option<(f64, f64)> {
    match nom {
        "normale" | "normal" => Some((*p.first()?, *p.get(1)?)),
        "binomiale" => {
            let (n, q) = (*p.first()?, *p.get(1)?);
            Some((n * q, (n * q * (1.0 - q)).sqrt()))
        }
        "poisson" => {
            let l = *p.first()?;
            Some((l, l.sqrt()))
        }
        "uniforme" => {
            let (a, b) = (*p.first()?, *p.get(1)?);
            Some(((a + b) / 2.0, (b - a) / 12f64.sqrt()))
        }
        "exponentielle" => {
            let l = *p.first()?;
            Some((1.0 / l, 1.0 / l))
        }
        "student" => {
            let v = *p.first()?;
            Some((0.0, (v / (v - 2.0)).sqrt()))
        }
        "khi-deux" | "khideux" => {
            let k = *p.first()?;
            Some((k, (2.0 * k).sqrt()))
        }
        _ => None,
    }
}

fn loi_lue(desc: &str) -> Option<(String, Vec<f64>)> {
    let bas = desc.to_lowercase();
    let depart = bas.find("loi ")? + 4;
    let reste = &desc[depart..];
    let nom: String = reste
        .chars()
        .take_while(|c| c.is_alphabetic() || *c == '-')
        .collect::<String>()
        .to_lowercase();
    let parametres = match reste.find('(') {
        Some(o) => {
            let f = reste[o..].find(')')? + o;
            nombres(&reste[o + 1..f])
        }
        None => Vec::new(),
    };
    Some((nom, parametres))
}

fn probabilite_loi(desc: &str) -> Option<String> {
    let (nom, parametres) = loi_lue(desc)?;
    let bas = desc.to_lowercase();
    let borne = bas.find("que x")? + 5;
    let condition = &desc[borne..bas.find("pour la loi")?];
    let seuil = *nombres(condition).first()?;
    let (sens, valeur) = if condition.contains(">=") || condition.contains('\u{2265}') {
        (
            "\\geqslant",
            1.0 - repartition(&nom, &parametres, seuil)?
                + masse(&nom, &parametres, seuil).unwrap_or(0.0),
        )
    } else if condition.contains("<=") || condition.contains('\u{2264}') {
        ("\\leqslant", repartition(&nom, &parametres, seuil)?)
    } else {
        ("=", masse(&nom, &parametres, seuil)?)
    };
    prose(&[format!(
        "\\(P(X {} {}) \\approx {}\\)",
        sens,
        nombre(seuil),
        approx(valeur)
    )])
}

fn quantile_loi(desc: &str) -> Option<String> {
    let (nom, parametres) = loi_lue(desc)?;
    let bas = desc.to_lowercase();
    let debut = bas.find("d'ordre")? + 7;
    let ordre = *nombres(&desc[debut..]).first()?;
    let valeur = quantile(&nom, &parametres, ordre)?;
    prose(&[format!(
        "\\(x_{{{}}} \\approx {}\\)",
        nombre(ordre),
        approx(valeur)
    )])
}

fn moment_loi(desc: &str, ecart: bool) -> Option<String> {
    let (nom, parametres) = loi_lue(desc)?;
    let (esperance, sigma) = moments(&nom, &parametres)?;
    let (symbole, valeur) = if ecart {
        ("\\sigma(X)", sigma)
    } else {
        ("E(X)", esperance)
    };
    prose(&[format!("\\({} \\approx {}\\)", symbole, approx(valeur))])
}

fn fluctuation(desc: &str) -> Option<String> {
    let v = nombres(desc);
    if v.len() < 2 {
        return None;
    }
    let (n, p) = (v[0], v[1]);
    if n <= 0.0 {
        return None;
    }
    let rayon = 1.0 / n.sqrt();
    prose(&[format!(
        "Au seuil de 95 %, l'intervalle de fluctuation d'une fréquence sur un échantillon de \
         taille \\(n = {}\\), pour une proportion \\(p = {}\\), est \
         \\(\\left[\\, p - \\dfrac{{1}}{{\\sqrt{{n}}}} \\,;\\, p + \\dfrac{{1}}{{\\sqrt{{n}}}} \\,\\right] \
         = \\left[\\, {} \\,;\\, {} \\,\\right]\\).",
        nombre(n),
        nombre(p),
        approx(p - rayon),
        approx(p + rayon)
    )])
}

fn tchebychev(desc: &str) -> Option<String> {
    let v = nombres(desc);
    if v.len() < 3 {
        return None;
    }
    let (mu, variance, ecart) = (v[0], v[1], v[2]);
    if ecart == 0.0 {
        return None;
    }
    prose(&[format!(
        "Pour une variable aléatoire \\(X\\) d'espérance \\(\\mu = {}\\) et de variance \
         \\(V = {}\\), l'inégalité de Bienaymé-Tchebychev donne, pour l'écart \\(a = {}\\) : \
         \\(P\\left(\\,\\left|X - \\mu\\right| \\geqslant a\\,\\right) \\leqslant \
         \\dfrac{{V}}{{a^2}} = {}\\).",
        nombre(mu),
        nombre(variance),
        nombre(ecart),
        approx(variance / (ecart * ecart))
    )])
}

fn variance_serie(desc: &str, ecart: bool) -> Option<String> {
    let bas = desc.to_lowercase();
    let (valeurs, effectifs) = valeurs_et_effectifs(desc, &bas)?;
    let (total, somme) = somme_ponderee(&valeurs, &effectifs);
    let moyenne: f64 = somme / total;
    let variance: f64 = valeurs
        .iter()
        .zip(&effectifs)
        .map(|(v, n)| n * (v - moyenne) * (v - moyenne))
        .sum::<f64>()
        / total;
    let pondere = bas.contains("effectifs");
    if ecart {
        return prose(&[
            "L'écart type vaut \\(\\sigma = \\sqrt{V}\\).".to_string(),
            format!(
                "Ici, \\(\\bar{{x}} = {}\\), \\(V = {}\\) puis \\(\\sigma = \\sqrt{{{}}} \\approx {}\\).",
                nombre(moyenne),
                approx(variance),
                approx(variance),
                approx(variance.sqrt())
            ),
        ]);
    }
    let formule = if pondere {
        "\\(V = \\dfrac{1}{N} \\sum n_i (x_i - \\bar{x})^2\\)"
    } else {
        "\\(V = \\dfrac{1}{n} \\sum (x_i - \\bar{x})^2\\)"
    };
    prose(&[
        format!("La variance vaut {}.", formule),
        format!(
            "Ici, \\(\\bar{{x}} = {}\\) puis \\(V = {}\\).",
            nombre(moyenne),
            approx(variance)
        ),
    ])
}

fn covariance_series(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let depart = bas.find("des séries")? + "des séries".len();
    let coupe = bas[depart..].find(" et ")? + depart;
    let x = nombres(&desc[depart..coupe]);
    let y = nombres(&desc[coupe..]);
    if x.is_empty() || x.len() != y.len() {
        return None;
    }
    let n = x.len() as f64;
    let mx = x.iter().sum::<f64>() / n;
    let my = y.iter().sum::<f64>() / n;
    let covariance = x
        .iter()
        .zip(&y)
        .map(|(a, b)| (a - mx) * (b - my))
        .sum::<f64>()
        / n;
    prose(&[
        "La covariance vaut \\(\\operatorname{cov}(x, y) = \\dfrac{1}{n} \
         \\sum (x_i - \\bar{x})(y_i - \\bar{y})\\)."
            .to_string(),
        format!(
            "Ici, \\(\\bar{{x}} = {}\\), \\(\\bar{{y}} \\approx {}\\) puis \
             \\(\\operatorname{{cov}}(x, y) \\approx {}\\).",
            nombre(mx),
            approx(my),
            approx(covariance)
        ),
    ])
}

struct Noeud {
    nom: String,
    proba: f64,
    enfants: Vec<Noeud>,
}

fn lire_noeuds(lignes: &[&str], i: &mut usize) -> Vec<Noeud> {
    let mut res = Vec::new();
    while *i < lignes.len() {
        let t = lignes[*i].trim();
        if t == "}" {
            *i += 1;
            break;
        }
        if t.is_empty() {
            *i += 1;
            continue;
        }
        let ouvre = t.ends_with('{');
        let tete = t.trim_end_matches('{').trim();
        let mut mots = tete.split_whitespace();
        let nom = mots.next().unwrap_or("").to_string();
        let proba = mots.next().and_then(reel).unwrap_or(0.0);
        *i += 1;
        let enfants = if ouvre {
            lire_noeuds(lignes, i)
        } else {
            Vec::new()
        };
        res.push(Noeud {
            nom,
            proba,
            enfants,
        });
    }
    res
}

fn complete_arbre(noeuds: &mut Vec<Noeud>) {
    if noeuds.len() == 1 {
        let nom = match noeuds[0].nom.strip_prefix('!') {
            Some(r) => r.to_string(),
            None => format!("!{}", noeuds[0].nom),
        };
        let proba = 1.0 - noeuds[0].proba;
        noeuds.push(Noeud {
            nom,
            proba,
            enfants: Vec::new(),
        });
    }
    for n in noeuds.iter_mut() {
        complete_arbre(&mut n.enfants);
    }
}

fn compte_feuilles(noeuds: &[Noeud]) -> usize {
    noeuds
        .iter()
        .map(|n| {
            if n.enfants.is_empty() {
                1
            } else {
                compte_feuilles(&n.enfants)
            }
        })
        .sum()
}

struct Place {
    y: f64,
    enfants: Vec<Place>,
}

fn place_arbre(noeuds: &[Noeud], curseur: &mut f64, pas: f64) -> Vec<Place> {
    noeuds
        .iter()
        .map(|n| {
            if n.enfants.is_empty() {
                let y = *curseur;
                *curseur += pas;
                Place {
                    y,
                    enfants: Vec::new(),
                }
            } else {
                let enfants = place_arbre(&n.enfants, curseur, pas);
                let y = (enfants[0].y + enfants[enfants.len() - 1].y) / 2.0;
                Place { y, enfants }
            }
        })
        .collect()
}

fn surligne(nom: &str) -> String {
    match nom.strip_prefix('!') {
        Some(r) => format!("<tspan style=\"text-decoration:overline\">{}</tspan>", r),
        None => nom.to_string(),
    }
}

fn trace_arbre(
    noeuds: &[Noeud],
    places: &[Place],
    niveau: usize,
    pere: (f64, f64),
    chemin: &str,
    produit: f64,
    corps: &mut String,
) {
    let x = 8.0 + 42.0 * (niveau + 1) as f64;
    for (noeud, place) in noeuds.iter().zip(places) {
        let suite = if chemin.is_empty() {
            surligne(&noeud.nom)
        } else {
            format!("{} ∩ {}", chemin, surligne(&noeud.nom))
        };
        let cumul = produit * noeud.proba;
        corps.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" \
             stroke-width=\"0.4\"/>\
             <text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\">{}</text>\
             <text x=\"{:.2}\" y=\"{:.2}\" class=\"nom\" style=\"text-anchor:middle\">{}</text>",
            pere.0 + 4.5,
            pere.1,
            x - 4.5,
            place.y,
            TRAIT,
            (pere.0 + x) / 2.0,
            (pere.1 + place.y) / 2.0 - 1.4,
            decimal(noeud.proba),
            x,
            place.y + 1.2,
            surligne(&noeud.nom)
        ));
        if noeud.enfants.is_empty() {
            corps.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" class=\"lab\" style=\"text-anchor:start\">\
                 P({}) = {}</text>",
                x + 8.0,
                place.y + 1.1,
                suite,
                decimal(cumul)
            ));
        } else {
            trace_arbre(
                &noeud.enfants,
                &place.enfants,
                niveau + 1,
                (x, place.y),
                &suite,
                cumul,
                corps,
            );
        }
    }
}

fn arbre(corps: &str) -> Option<String> {
    let lignes: Vec<&str> = corps.lines().collect();
    let mut i = 0usize;
    let mut racines = lire_noeuds(&lignes, &mut i);
    if racines.is_empty() {
        return None;
    }
    complete_arbre(&mut racines);
    let feuilles = compte_feuilles(&racines);
    let pas = 14.0;
    let mut curseur = 8.0;
    let places = place_arbre(&racines, &mut curseur, pas);
    let racine_y = (places[0].y + places[places.len() - 1].y) / 2.0;
    let mut dessin = String::new();
    trace_arbre(&racines, &places, 0, (6.0, racine_y), "", 1.0, &mut dessin);
    let hauteur = pas * feuilles as f64 + 6.0;
    Some(crate::maths::trace::enveloppe_haute(&dessin, TRAIT, hauteur))
}

#[derive(Clone, Copy)]
struct Frac {
    n: i64,
    d: i64,
}

fn pgcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs().max(1)
    } else {
        pgcd(b, a % b)
    }
}

impl Frac {
    fn new(n: i64, d: i64) -> Frac {
        let signe = if d < 0 { -1 } else { 1 };
        let g = pgcd(n, d);
        Frac {
            n: signe * n / g,
            d: signe * d / g,
        }
    }
    fn plus(self, autre: Frac) -> Frac {
        Frac::new(self.n * autre.d + autre.n * self.d, self.d * autre.d)
    }
    fn fois(self, autre: Frac) -> Frac {
        Frac::new(self.n * autre.n, self.d * autre.d)
    }
    fn moins(self, autre: Frac) -> Frac {
        self.plus(Frac::new(-autre.n, autre.d))
    }
    fn reel(self) -> f64 {
        self.n as f64 / self.d as f64
    }
    fn tex(self) -> String {
        if self.d == 1 {
            format!("{}", self.n)
        } else {
            format!("\\dfrac{{{}}}{{{}}}", self.n, self.d)
        }
    }
}

fn fraction(jeton: &str) -> Option<Frac> {
    let t = jeton.trim();
    if t.is_empty() {
        return None;
    }
    if let Some((a, b)) = t.split_once('/') {
        let (x, y) = (reel(a)?, reel(b)?);
        if y == 0.0 {
            return None;
        }
        return Some(Frac::new(x.round() as i64, y.round() as i64));
    }
    let v = reel(t)?;
    let mut d = 1i64;
    let mut n = v;
    while (n - n.round()).abs() > 1e-9 && d < 1_000_000 {
        n *= 10.0;
        d *= 10;
    }
    Some(Frac::new(n.round() as i64, d))
}

fn racine_tex(f: Frac) -> Option<String> {
    let racine_entiere = |m: i64| -> Option<i64> {
        let r = (m as f64).sqrt().round() as i64;
        if r * r == m {
            Some(r)
        } else {
            None
        }
    };
    let bas = racine_entiere(f.d)?;
    match racine_entiere(f.n) {
        Some(haut) => Some(Frac::new(haut, bas).tex()),
        None => {
            if bas == 1 {
                Some(format!("\\sqrt{{{}}}", f.n))
            } else {
                Some(format!("\\dfrac{{\\sqrt{{{}}}}}{{{}}}", f.n, bas))
            }
        }
    }
}

fn crochet(corps: &str, etiquette: &str) -> Option<Vec<Frac>> {
    let bas = corps.to_lowercase();
    let depart = bas.find(etiquette)? + etiquette.len();
    let ouvre = corps[depart..].find('[')? + depart;
    let ferme = corps[ouvre..].find(']')? + ouvre;
    corps[ouvre + 1..ferme]
        .split(';')
        .map(fraction)
        .collect::<Option<Vec<Frac>>>()
}

fn loi_dressee(desc: &str, corps: Option<&str>) -> Option<String> {
    let corps = corps?;
    let nom = desc.split_whitespace().last()?.trim_end_matches('.').to_string();
    let valeurs = crochet(corps, "valeurs")?;
    let probabilites = crochet(corps, "probabilités")?;
    if valeurs.len() != probabilites.len() || valeurs.is_empty() {
        return None;
    }
    let total = probabilites
        .iter()
        .fold(Frac::new(0, 1), |acc, p| acc.plus(*p));
    if total.n != total.d {
        return prose(&[format!(
            "La somme des probabilités vaut \\({}\\) au lieu de \\(1\\) : la loi est refusée.",
            total.tex()
        )]);
    }
    let maths = |contenu: String, entete: bool| case(&format!("\\({}\\)", contenu), entete);
    let mut haut = maths("x_i".to_string(), true);
    let mut bas = maths(format!("P({} = x_i)", nom), true);
    for (v, p) in valeurs.iter().zip(&probabilites) {
        haut.push_str(&maths(v.tex(), false));
        bas.push_str(&maths(p.tex(), false));
    }
    Some(tableau_borde(&[haut, bas]))
}

fn moments_loi_x(desc: &str, env: &Env, quoi: u8) -> Option<String> {
    let nom = desc.split_whitespace().last()?.trim_end_matches('.').to_string();
    let corps = env.donnees.get(&format!("loi {}", nom))?;
    let valeurs = crochet(corps, "valeurs")?;
    let probabilites = crochet(corps, "probabilités")?;
    if valeurs.len() != probabilites.len() {
        return None;
    }
    let esperance = valeurs
        .iter()
        .zip(&probabilites)
        .fold(Frac::new(0, 1), |acc, (v, p)| acc.plus(v.fois(*p)));
    if quoi == 0 {
        return prose(&[
            format!("\\(E({}) = \\sum x_i P({} = x_i)\\).", nom, nom),
            format!(
                "Ici, \\(E({}) = {} \\approx {}\\).",
                nom,
                esperance.tex(),
                nombre(esperance.reel())
            ),
        ]);
    }
    let carre = valeurs
        .iter()
        .zip(&probabilites)
        .fold(Frac::new(0, 1), |acc, (v, p)| acc.plus(v.fois(*v).fois(*p)));
    let variance = carre.moins(esperance.fois(esperance));
    if quoi == 1 {
        return prose(&[
            format!("\\(V({}) = E({}^2) - E({})^2\\).", nom, nom, nom),
            format!(
                "Ici, \\(V({}) = {} \\approx {}\\).",
                nom,
                variance.tex(),
                nombre(variance.reel())
            ),
        ]);
    }
    let exact = racine_tex(variance)
        .map(|t| format!("{} \\approx ", t))
        .unwrap_or_default();
    prose(&[
        format!("\\(\\sigma({}) = \\sqrt{{V({})}}\\).", nom, nom),
        format!(
            "Ici, \\(\\sigma({}) = {}{}\\).",
            nom,
            exact,
            nombre(variance.reel().sqrt())
        ),
    ])
}

pub(crate) fn collecte_lois(seg: &str, env: &mut Env) {
    let mut nom: Option<String> = None;
    let mut contenu = String::new();
    for ligne in seg.lines() {
        let t = ligne.trim();
        if nom.is_some() {
            if t == "}" {
                if let Some(n) = nom.take() {
                    env.donnees
                        .insert(format!("loi {}", n), std::mem::take(&mut contenu));
                }
            } else {
                contenu.push_str(ligne);
                contenu.push('\n');
            }
            continue;
        }
        let bas = t.to_lowercase();
        if bas.starts_with("<dresse>la loi de probabilité de") && t.ends_with('{') {
            nom = t
                .trim_end_matches('{')
                .trim()
                .split_whitespace()
                .last()
                .map(|m| m.to_string());
        }
    }
}
