use crate::Env;

fn prose(lignes: &[String]) -> Option<String> {
    Some(crate::maths::algebre::bloc_prose(lignes))
}

fn modulo(desc: &str) -> Option<usize> {
    let sans = desc.replace('ℤ', "Z");
    let i = sans.find("Z/")? + 2;
    let n: String = sans[i..].chars().take_while(|c| c.is_ascii_digit()).collect();
    let n: usize = n.parse().ok()?;
    if (2..=24).contains(&n) {
        Some(n)
    } else {
        None
    }
}

fn cellule(texte: &str, entete: bool) -> String {
    format!(
        "<td style=\"border:0.2mm solid #1a3a6b;padding:0.4mm 1.6mm;text-align:center;{}\">{}</td>",
        if entete { "background:#e8eef7;font-weight:bold;" } else { "" },
        texte
    )
}

fn table_znz(n: usize, multiplication: bool) -> String {
    let op = |a: usize, b: usize| if multiplication { (a * b) % n } else { (a + b) % n };
    let signe = if multiplication { "×" } else { "+" };
    let mut lignes = Vec::new();
    let mut entete = cellule(&format!("\\({}\\)", signe), true);
    for b in 0..n {
        entete.push_str(&cellule(&b.to_string(), true));
    }
    lignes.push(entete);
    for a in 0..n {
        let mut ligne = cellule(&a.to_string(), true);
        for b in 0..n {
            ligne.push_str(&cellule(&op(a, b).to_string(), false));
        }
        lignes.push(ligne);
    }
    let mut html = String::from(
        "<table class=\"tab\" style=\"border-collapse:collapse;width:auto;border:0.3mm solid #1a3a6b;margin:0 0 1em;\">",
    );
    for l in lignes {
        html.push_str(&format!("<tr>{}</tr>", l));
    }
    html.push_str("</table>");
    html
}

fn pgcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        pgcd(b, a % b)
    }
}

fn generateurs(desc: &str) -> Option<String> {
    let n = modulo(desc)?;
    let gens: Vec<String> = (1..n).filter(|k| pgcd(*k, n) == 1).map(|k| k.to_string()).collect();
    let phi = gens.len();
    prose(&[
        format!(
            "Un élément \\(k\\) engendre \\((\\mathbb{{Z}}/{}\\mathbb{{Z}}, +)\\) si et seulement si \\(k\\) est premier avec \\({}\\).",
            n, n
        ),
        format!(
            "Les générateurs sont \\(\\{{{}\\}}\\), au nombre de \\(\\varphi({}) = {}\\).",
            gens.join("\\,;\\,"),
            n,
            phi
        ),
    ])
}

fn ppcm(a: usize, b: usize) -> usize {
    a / pgcd(a, b) * b
}

fn permutation(desc: &str) -> Option<String> {
    let ouvre = desc.find('(')?;
    let ferme = desc[ouvre..].find(')')? + ouvre;
    let images: Vec<usize> = desc[ouvre + 1..ferme]
        .split(|c: char| c == ' ' || c == ',' || c == ';')
        .filter(|m| !m.is_empty())
        .map(|m| m.parse::<usize>())
        .collect::<Result<_, _>>()
        .ok()?;
    let n = images.len();
    if n == 0 || images.iter().any(|&v| v == 0 || v > n) {
        return None;
    }
    let mut trie = images.clone();
    trie.sort_unstable();
    if trie != (1..=n).collect::<Vec<_>>() {
        return None;
    }
    let mut vus = vec![false; n + 1];
    let mut cycles: Vec<Vec<usize>> = Vec::new();
    for depart in 1..=n {
        if vus[depart] {
            continue;
        }
        let mut cycle = vec![depart];
        vus[depart] = true;
        let mut courant = images[depart - 1];
        while courant != depart {
            vus[courant] = true;
            cycle.push(courant);
            courant = images[courant - 1];
        }
        cycles.push(cycle);
    }
    let vrais: Vec<&Vec<usize>> = cycles.iter().filter(|c| c.len() >= 2).collect();
    let fixes: Vec<String> = cycles
        .iter()
        .filter(|c| c.len() == 1)
        .map(|c| c[0].to_string())
        .collect();
    let notation = if vrais.is_empty() {
        "\\mathrm{id}".to_string()
    } else {
        vrais
            .iter()
            .map(|c| {
                format!(
                    "({})",
                    c.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("\\;")
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };
    let signature: i32 = if (n - cycles.len()) % 2 == 0 { 1 } else { -1 };
    let ordre = cycles.iter().fold(1usize, |acc, c| ppcm(acc, c.len()));
    let mut lignes = vec![format!(
        "La permutation \\(\\sigma = \\begin{{pmatrix}} {} \\\\ {} \\end{{pmatrix}}\\) se décompose en cycles à supports disjoints : \\(\\sigma = {}\\).",
        (1..=n).map(|v| v.to_string()).collect::<Vec<_>>().join(" & "),
        images.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" & "),
        notation
    )];
    if !fixes.is_empty() {
        lignes.push(format!(
            "Les points fixes sont \\(\\{{{}\\}}\\).",
            fixes.join("\\,;\\,")
        ));
    }
    lignes.push(format!(
        "La signature vaut \\(\\varepsilon(\\sigma) = (-1)^{{n - c}} = {}\\) — la permutation est {} — et son ordre est \\(\\mathrm{{ppcm}}\\) des longueurs de cycles : \\({}\\).",
        if signature == 1 { "+1" } else { "-1" },
        if signature == 1 { "paire" } else { "impaire" },
        ordre
    ));
    prose(&lignes)
}

pub fn commande(verbe: &str, desc: &str, _corps: Option<&str>, _env: &mut Env) -> Option<String> {
    let bas = desc.to_lowercase();
    if verbe == "Dresse" && bas.contains("la table de") && desc.replace('ℤ', "Z").contains("Z/") {
        let n = modulo(desc)?;
        let multiplication = bas.contains("multiplication");
        return Some(table_znz(n, multiplication));
    }
    if (verbe == "Détermine" || verbe == "Décompose")
        && (bas.contains("cycles") || bas.contains("en cycles"))
        && bas.contains("permutation")
    {
        return permutation(desc);
    }
    if verbe == "Détermine" && bas.contains("générateurs") && desc.replace('ℤ', "Z").contains("Z/") {
        return generateurs(desc);
    }
    None
}
