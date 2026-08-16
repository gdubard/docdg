use crate::maths::calcul::format_number;

const UNITES: &[(&str, char, f64)] = &[
    ("s", 't', 1.0),
    ("min", 't', 60.0),
    ("h", 't', 3600.0),
    ("j", 't', 86400.0),
    ("an", 't', 31_557_600.0),
    ("ans", 't', 31_557_600.0),
    ("m/s", 'v', 1.0),
    ("km/h", 'v', 1.0 / 3.6),
    ("km/s", 'v', 1000.0),
    ("Pa", 'p', 1.0),
    ("hPa", 'p', 100.0),
    ("kPa", 'p', 1000.0),
    ("bar", 'p', 1.0e5),
    ("atm", 'p', 101_325.0),
    ("mmHg", 'p', 133.322),
    ("J", 'e', 1.0),
    ("kJ", 'e', 1.0e3),
    ("MJ", 'e', 1.0e6),
    ("Wh", 'e', 3600.0),
    ("kWh", 'e', 3.6e6),
    ("eV", 'e', 1.602_176_634e-19),
    ("keV", 'e', 1.602_176_634e-16),
    ("MeV", 'e', 1.602_176_634e-13),
    ("cal", 'e', 4.184),
    ("kcal", 'e', 4184.0),
    ("W", 'w', 1.0),
    ("kW", 'w', 1.0e3),
    ("MW", 'w', 1.0e6),
    ("ch", 'w', 735.5),
    ("Hz", 'f', 1.0),
    ("kHz", 'f', 1.0e3),
    ("MHz", 'f', 1.0e6),
    ("GHz", 'f', 1.0e9),
];

const CONSTANTES: &[(&[&str], &str, f64, &str)] = &[
    (
        &["vitesse de la lumière", "vitesse de la lumiere", "célérité de la lumière"],
        "c",
        2.997_924_58e8,
        "m\\,s^{-1}",
    ),
    (
        &["constante de planck"],
        "h",
        6.626_070_15e-34,
        "J\\,s",
    ),
    (
        &["constante de planck réduite", "constante de planck reduite"],
        "\\hbar",
        1.054_571_817e-34,
        "J\\,s",
    ),
    (
        &["constante de gravitation", "constante gravitationnelle"],
        "G",
        6.674_30e-11,
        "m^{3}\\,kg^{-1}\\,s^{-2}",
    ),
    (
        &["nombre d'avogadro", "constante d'avogadro"],
        "N_A",
        6.022_140_76e23,
        "mol^{-1}",
    ),
    (
        &["constante des gaz parfaits", "constante universelle des gaz"],
        "R",
        8.314_462_618,
        "J\\,mol^{-1}\\,K^{-1}",
    ),
    (
        &["constante de boltzmann"],
        "k_B",
        1.380_649e-23,
        "J\\,K^{-1}",
    ),
    (
        &["charge élémentaire", "charge elementaire"],
        "e",
        1.602_176_634e-19,
        "C",
    ),
    (
        &["masse de l'électron", "masse de l'electron"],
        "m_e",
        9.109_383_7015e-31,
        "kg",
    ),
    (
        &["masse du proton"],
        "m_p",
        1.672_621_923_69e-27,
        "kg",
    ),
    (
        &["masse du neutron"],
        "m_n",
        1.674_927_498_04e-27,
        "kg",
    ),
    (
        &["permittivité du vide", "permittivite du vide"],
        "\\varepsilon_0",
        8.854_187_812_8e-12,
        "F\\,m^{-1}",
    ),
    (
        &["perméabilité du vide", "permeabilite du vide"],
        "\\mu_0",
        1.256_637_062_12e-6,
        "H\\,m^{-1}",
    ),
    (
        &["constante de faraday"],
        "F",
        96_485.332_12,
        "C\\,mol^{-1}",
    ),
    (
        &["constante de stefan", "constante de stefan-boltzmann"],
        "\\sigma",
        5.670_374_419e-8,
        "W\\,m^{-2}\\,K^{-4}",
    ),
    (
        &["unité de masse atomique", "unite de masse atomique"],
        "u",
        1.660_539_066_60e-27,
        "kg",
    ),
    (
        &["intensité de la pesanteur", "intensite de la pesanteur", "accélération de la pesanteur"],
        "g",
        9.806_65,
        "m\\,s^{-2}",
    ),
];

fn scientifique(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    let exposant = v.abs().log10().floor() as i32;
    if (-3..6).contains(&exposant) {
        return format_number((v * 1e9).round() / 1e9).replace(',', "{,}");
    }
    let mantisse = v / 10f64.powi(exposant);
    let mantisse = (mantisse * 1e9).round() / 1e9;
    format!(
        "{} \\times 10^{{{}}}",
        format_number(mantisse).replace(',', "{,}"),
        exposant
    )
}

fn latex_unite(u: &str) -> String {
    match u {
        "m/s" => "m\\,s^{-1}".into(),
        "km/h" => "km\\,h^{-1}".into(),
        "km/s" => "km\\,s^{-1}".into(),
        _ => u.into(),
    }
}

fn reel(s: &str) -> Option<f64> {
    s.trim()
        .trim_end_matches(['.', ',', ';'])
        .replace(',', ".")
        .replace('−', "-")
        .replace(' ', "")
        .parse()
        .ok()
}

fn temperature(quantite: f64, depart: &str, arrivee: &str) -> Option<String> {
    let celsius = ["°C", "°c", "degrés Celsius", "degres Celsius"];
    let kelvin = ["K", "kelvin", "kelvins"];
    let (valeur, de, vers) = if celsius.contains(&depart) && kelvin.contains(&arrivee) {
        (quantite + 273.15, "^{\\circ}\\mathrm{C}", "\\mathrm{K}")
    } else if kelvin.contains(&depart) && celsius.contains(&arrivee) {
        (quantite - 273.15, "\\mathrm{K}", "^{\\circ}\\mathrm{C}")
    } else {
        return None;
    };
    Some(crate::layout::rendu::bloc_calcul(&format!(
        "{}\\ {} = {}\\ {}",
        format_number(quantite).replace(',', "{,}"),
        de,
        format_number((valeur * 1e6).round() / 1e6).replace(',', "{,}"),
        vers
    )))
}

fn conversion(desc: &str) -> Option<String> {
    let (avant, apres) = desc.split_once(" en ")?;
    let avant = avant.trim();
    let arrivee = apres.trim().trim_end_matches('.').trim();
    let coupe = avant.find(|c: char| c.is_alphabetic() || c == '°')?;
    if coupe == 0 {
        return None;
    }
    let quantite = reel(&avant[..coupe])?;
    let depart = avant[coupe..].trim();
    if let Some(html) = temperature(quantite, depart, arrivee) {
        return Some(html);
    }
    let (_, ga, fa) = UNITES.iter().find(|(nom, _, _)| *nom == depart)?;
    let (_, gb, fb) = UNITES.iter().find(|(nom, _, _)| *nom == arrivee)?;
    if ga != gb {
        return None;
    }
    let r = quantite * fa / fb;
    Some(crate::layout::rendu::bloc_calcul(&format!(
        "{}\\ \\mathrm{{{}}} = {}\\ \\mathrm{{{}}}",
        format_number(quantite).replace(',', "{,}"),
        latex_unite(depart),
        scientifique(r),
        latex_unite(arrivee)
    )))
}

fn constante(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let (_, symbole, valeur, unite) = CONSTANTES
        .iter()
        .filter(|(cles, _, _, _)| cles.iter().any(|c| bas.contains(c)))
        .max_by_key(|(cles, _, _, _)| cles.iter().map(|c| c.len()).max().unwrap_or(0))?;
    Some(crate::layout::rendu::bloc_calcul(&format!(
        "{} = {}\\ \\mathrm{{{}}}",
        symbole,
        scientifique(*valeur),
        unite
    )))
}

pub fn commande(
    verbe: &str,
    desc: &str,
    _corps: Option<&str>,
    _env: &mut crate::Env,
) -> Option<String> {
    match verbe {
        "Convertis" => conversion(desc),
        "Donne" | "Affiche" | "Calcule"
            if desc.to_lowercase().contains("constante")
                || desc.to_lowercase().contains("vitesse de la lumi")
                || desc.to_lowercase().contains("charge élémentaire")
                || desc.to_lowercase().contains("charge elementaire")
                || desc.to_lowercase().contains("nombre d'avogadro")
                || desc.to_lowercase().contains("masse de l'électron")
                || desc.to_lowercase().contains("masse de l'electron")
                || desc.to_lowercase().contains("masse du proton")
                || desc.to_lowercase().contains("masse du neutron")
                || desc.to_lowercase().contains("pesanteur")
                || desc.to_lowercase().contains("masse atomique") =>
        {
            constante(desc)
        }
        _ => None,
    }
}
