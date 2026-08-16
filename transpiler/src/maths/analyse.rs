use crate::langage::commandes::Obj;
use crate::Env;

pub(crate) struct Tableau {
    pub bornes: Vec<String>,
    pub lignes: Vec<(String, Vec<String>)>,
}

impl Tableau {
    pub fn html(&self) -> String {
        let m = self.bornes.len();
        if m < 2 {
            return String::new();
        }
        let mut html = String::from("<table class=\"signes\">");
        html.push_str("<tr><th>\\(x\\)</th>");
        for (j, b) in self.bornes.iter().enumerate() {
            html.push_str(&format!("<th class=\"borne\">\\({}\\)</th>", b));
            if j + 1 < m {
                html.push_str("<th></th>");
            }
        }
        html.push_str("</tr>");
        for (nom, cellules) in &self.lignes {

            let mut cases: Vec<String> = cellules.clone();
            if cases.len() + 2 == 2 * m - 1 {
                cases.insert(0, String::new());
                cases.push(String::new());
            }
            let variation = cases
                .iter()
                .any(|c| c.starts_with('^') || c.starts_with('_') || c.starts_with('='));
            html.push_str(&format!("<tr><th>\\({}\\)</th>", nom));
            for (k, c) in cases.iter().enumerate() {
                let borne = k % 2 == 0;
                let brut = c.trim();
                let (classe, contenu) = match brut.chars().next() {
                    Some('^') => ("haut", format!("\\({}\\)", &brut[1..])),
                    Some('_') => ("bas", format!("\\({}\\)", &brut[1..])),
                    Some('=') => ("milieu", format!("\\({}\\)", &brut[1..])),

                    Some('#') => ("hachure", String::new()),
                    _ if brut.is_empty() => {
                        (if borne { "borne" } else { "signe" }, String::new())
                    }
                    _ => (
                        if borne { "borne" } else { "signe" },
                        format!("\\({}\\)", tex_signe(brut)),
                    ),
                };
                html.push_str(&format!(
                    "<td class=\"{}{}\">{}</td>",
                    classe,
                    if variation { " var" } else { "" },
                    contenu
                ));
            }
            html.push_str("</tr>");
        }
        html.push_str("</table>");
        html
    }
}

fn tex_signe(c: &str) -> String {
    match c {
        "+" => "+".into(),
        "-" | "−" => "-".into(),
        "||" | "‖" => "\\|".into(),
        autre => crate::utils::notation::to_latex(autre),
    }
}

pub(crate) fn tableau_explicite(desc: &str, corps: &str) -> Option<String> {
    let i = desc.find("x:")?;
    let reste = desc[i + 2..].trim_start();
    let bornes_brutes = reste.strip_prefix('{')?.split_once('}')?.0;
    let bornes: Vec<String> = bornes_brutes
        .split('|')
        .map(|b| crate::utils::notation::to_latex(b.trim()))
        .collect();
    if bornes.len() < 2 {
        return None;
    }
    let mut lignes = Vec::new();
    for ligne in corps.lines() {
        let l = ligne.trim();
        if l.is_empty() {
            continue;
        }
        let (nom, cellules) = l.split_once(':')?;
        lignes.push((
            crate::utils::notation::to_latex(nom.trim()),
            cellules.split('|').map(|c| c.trim().to_string()).collect(),
        ));
    }
    if lignes.is_empty() {
        return None;
    }
    Some(Tableau { bornes, lignes }.html())
}

pub(crate) fn tableau_calcule(op: &str, nom: &str, env: &Env) -> Option<String> {
    if !matches!(env.objects.get(nom), Some(Obj::Function { .. })) {
        return None;
    }
    let req = serde_json::json!({
        "op": op,
        "args": {"name": nom},
        "defs": crate::langage::commandes::objects_json(&env.objects),
    });
    let brut = crate::python::pont::ask(&req.to_string()).ok()?;
    tableau_depuis_cas(&brut)
}

fn tableau_depuis_cas(brut: &str) -> Option<String> {
    let mut lignes_src = brut.lines();
    let entete = lignes_src.next()?;
    let bornes: Vec<String> = entete
        .split('|')
        .skip(1)
        .map(|b| b.trim().to_string())
        .collect();
    if bornes.len() < 2 {
        return None;
    }
    let mut lignes = Vec::new();
    for l in lignes_src {
        if l.trim().is_empty() {
            continue;
        }
        let mut parts = l.split('|');
        let nom = parts.next()?.trim().to_string();
        lignes.push((nom, parts.map(|c| c.trim().to_string()).collect()));
    }
    Some(Tableau { bornes, lignes }.html())
}

fn image(desc: &str, env: &Env) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("l'image de")? + "l'image de".len();
    let (valeur, nom) = desc[i..].split_once(" par ")?;
    let valeur = valeur.trim();
    let nom = nom.trim().trim_end_matches('.');
    let (var, expr) = match env.objects.get(nom) {
        Some(Obj::Function { var, expr }) => (var.clone(), expr.clone()),
        _ => return None,
    };
    let negatif = valeur.starts_with('-');
    let remplacant = if negatif {
        format!("({})", valeur)
    } else {
        valeur.to_string()
    };
    let mut substituee = String::new();
    let source: Vec<char> = expr.chars().collect();
    let mut i = 0usize;
    while i < source.len() {
        let c = source[i];

        let debut = i == 0 || !(source[i - 1].is_alphabetic() || source[i - 1] == '_');
        if debut && expr[i..].starts_with(&var) {
            let apres = source.get(i + var.chars().count()).copied();
            if apres.map(|c| !c.is_alphanumeric() && c != '_').unwrap_or(true) {

                if substituee
                    .chars()
                    .last()
                    .map(|p| p.is_ascii_digit() || p == ')')
                    .unwrap_or(false)
                {
                    substituee.push('*');
                }
                substituee.push_str(&remplacant);
                i += var.chars().count();
                continue;
            }
        }
        substituee.push(c);
        i += 1;
    }
    let req = serde_json::json!({
        "op": "eval",
        "args": {"name": nom, "value": valeur},
        "defs": crate::langage::commandes::objects_json(&env.objects),
    });
    let resultat = crate::python::pont::ask(&req.to_string()).ok()?;
    let pose = crate::maths::algebre::decimales_fr(&crate::utils::notation::to_latex(&substituee));
    let arg = crate::maths::algebre::decimales_fr(&crate::utils::notation::to_latex(valeur));
    let tete = format!("{}({})", crate::utils::notation::to_latex(nom), arg);
    let final_tex = if pose == resultat {
        format!("{} = {}", tete, resultat)
    } else {
        format!("{} = {} = {}", tete, pose, resultat)
    };
    Some(crate::layout::rendu::bloc_calcul(&final_tex))
}

pub(crate) fn noms_listes(desc: &str) -> Vec<String> {
    let tail = match desc.trim().trim_end_matches('.').rsplit_once(" de ") {
        Some((_, r)) => r,
        None => return Vec::new(),
    };
    crate::utils::texte::noms_separes(tail)
}

fn tableaux(desc: &str, env: &mut Env, op: &str) -> Option<String> {
    let noms = noms_listes(desc);
    if noms.is_empty() {
        return None;
    }
    let mut html = String::new();
    let mut manquantes: Vec<String> = Vec::new();
    for nom in &noms {
        if op == "vartab" {
            env.etudiees.insert(nom.clone());
        }
        match tableau_calcule(op, nom, env) {
            Some(table) => html.push_str(&table),
            None => manquantes.push(nom.clone()),
        }
    }
    if !manquantes.is_empty() {
        let sujet = if op == "vartab" {
            "de variations"
        } else {
            "de signes"
        };
        html.push_str(&crate::maths::algebre::bloc_prose(&[format!(
            "Le tableau {} n'a pas pu être dressé pour \\({}\\) : la fonction \
             n'est pas déclarée ou le calcul formel n'a rien renvoyé.",
            sujet,
            manquantes.join("\\), \\(")
        )]));
    }
    Some(html)
}

pub fn commande(verbe: &str, desc: &str, corps: Option<&str>, env: &mut Env) -> Option<String> {
    let bas = desc.to_lowercase();
    match verbe {
        "Dresse" if bas.contains("tableaux de variation") => tableaux(desc, env, "vartab"),
        "Dresse" if bas.contains("tableaux de signe") => tableaux(desc, env, "signtab"),
        "Dresse" if bas.contains("tableau de signes") => match corps {
            Some(corps) => tableau_explicite(desc, corps),
            None => {
                let nom = desc.rsplit(' ').next()?.trim_end_matches('.');
                tableau_calcule("signtab", nom, env)
            }
        },
        "Calcule" if bas.contains("l'image de") => image(desc, env),
        "Dresse" if bas.contains("tableau de variations") => {
            let nom = desc.rsplit(' ').next()?.trim_end_matches('.');
            env.etudiees.insert(nom.to_string());
            tableau_calcule("vartab", nom, env)
        }
        _ => None,
    }
}
