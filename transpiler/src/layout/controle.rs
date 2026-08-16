use super::rendu::*;

pub(crate) fn accolade_du_corps(entete: &str) -> Option<usize> {
    let mut i = 0usize;
    while let Some(p) = entete[i..].find('{') {
        let abs = i + p;
        let avant = entete[..abs].trim_end();
        if avant.ends_with(" dans") || avant.ends_with(" dans ") {
            let mut profondeur = 0i32;
            let mut fin = None;
            for (k, c) in entete[abs..].char_indices() {
                match c {
                    '{' => profondeur += 1,
                    '}' => {
                        profondeur -= 1;
                        if profondeur == 0 {
                            fin = Some(abs + k + 1);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            match fin {
                Some(f) => {
                    i = f;
                    continue;
                }
                None => return Some(abs),
            }
        }
        return Some(abs);
    }
    None
}

pub(crate) fn position_de_sortir(contenu: &str) -> Option<usize> {
    position_du_mot(contenu, "sortir")
}

pub(crate) fn position_du_mot(contenu: &str, mot: &str) -> Option<usize> {
    let mut position = 0usize;
    for ligne in contenu.lines() {
        if ligne.trim() == mot {
            return Some(position);
        }
        position += ligne.len() + 1;
    }
    None
}

pub(crate) fn condition_de_tour(
    body: &str,
    vars: &std::collections::BTreeMap<String, f64>,
    boites: &crate::langage::conteneurs::Boites,
    fonctions: &crate::langage::fonctions::Fonctions,
) -> Option<String> {
    let t = body.trim_start();
    if !t.starts_with("si ") {
        return Some(body.to_string());
    }
    let Some(bi) = t.find('{') else {
        return Some(body.to_string());
    };
    let cond_txt = t[3..bi].trim();
    let Some((alors, apres)) = take_group(t, bi) else {
        return Some(body.to_string());
    };
    let apres_t = apres.trim_start();
    let (sinon, reste) = if let Some(r) = apres_t.strip_prefix("sinon") {
        let r = r.trim_start();
        match take_group(r, 0) {
            Some((b, s)) => (Some(b), s),
            None => (None, apres.clone()),
        }
    } else {
        (None, apres.clone())
    };
    if !reste.trim().is_empty() {
        return Some(body.to_string());
    }
    let cond_resolue = crate::langage::fonctions::resoudre_appels(cond_txt, vars, boites, fonctions, 0);
    let cond_resolue = crate::langage::conteneurs::resoudre_lectures(&cond_resolue, vars, boites, true);
    if eval_condition(&cond_resolue, vars) {
        Some(dedent(&alors))
    } else {
        sinon.map(|b| dedent(&b))
    }
}

pub const SEP_ITERATION: char = '\u{E013}';

pub(crate) fn sans_separateur(body: &str) -> String {
    body.lines()
        .filter(|l| l.trim() != "\u{E013}")
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn retrait(l: &str) -> usize {
    l.chars().take_while(|c| c.is_whitespace()).count()
}

pub(crate) fn dedent(body: &str) -> String {
    let min = body
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(retrait)
        .min()
        .unwrap_or(0);
    body.lines()
        .map(|l| match l.char_indices().nth(min) {
            Some((i, _)) if retrait(l) >= min => &l[i..],
            _ => l.trim_start(),
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim_matches('\n')
        .to_string()
}

pub(crate) fn normalise_affectations(texte: &str) -> String {
    let mut connus: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut sortie = String::with_capacity(texte.len());
    for ligne in texte.lines() {
        let nu = ligne.trim_start();
        let retrait = &ligne[..ligne.len() - nu.len()];

        if let Some(reste) = nu.strip_prefix("soit ") {
            let tete = reste
                .split_once('=')
                .map(|(g, _)| g)
                .unwrap_or(reste)
                .trim();
            let nom = tete.split([':', '(']).next().unwrap_or("").trim();
            if !nom.is_empty() && nom.chars().all(|c| c.is_alphanumeric() || c == '_') {
                connus.insert(nom.to_string());
            }
        }
        if let Some(reste) = nu.strip_prefix("pour ") {
            if let Some(nom) = reste.split_whitespace().next() {
                connus.insert(nom.to_string());
            }
        }

        let mut reecrite = None;
        if !nu.starts_with("soit ") && !nu.starts_with('<') && !nu.starts_with('[') {
            if let Some((gauche, droite)) = nu.split_once('=') {
                let nom = gauche.trim();

                let simple = !nom.is_empty()
                    && !droite.contains('#')
                    && !droite.starts_with('=')
                    && nom.chars().next().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false)
                    && nom.chars().all(|c| c.is_alphanumeric() || c == '_');
                if simple && connus.contains(nom) {
                    reecrite = Some(format!("{}soit {}", retrait, nu));
                }
            }
        }
        sortie.push_str(reecrite.as_deref().unwrap_or(ligne));
        sortie.push('\n');
    }
    if !texte.ends_with('\n') {
        sortie.pop();
    }
    sortie
}

pub(crate) fn applique_affectations(
    body: &str,
    etat: &mut std::collections::BTreeMap<String, f64>,
    boites: &mut crate::langage::conteneurs::Boites,
    fonctions: &mut crate::langage::fonctions::Fonctions,
) -> String {
    let mut sortie = String::new();
    let mut pos = 0usize;
    while pos < body.len() {
        let rest = &body[pos..];
        let debut_ligne = rest.trim_start_matches(['\t', ' ']);
        let controle = ["pour ", "tant que ", "faire", "si ", "sinon"]
            .iter()
            .any(|m| debut_ligne.starts_with(m));
        if controle && debut_ligne.contains('{') {
            let decale = rest.len() - debut_ligne.len();
            if let Some(ouvre) = debut_ligne.find('{') {
                if let Some((corps, apres)) = take_group(&debut_ligne[ouvre..], 0) {
                    let fin = decale + ouvre + corps.len() + 2 + (debut_ligne[ouvre..].len() - apres.len() - corps.len() - 2);
                    let _ = fin;
                    let consomme = decale + ouvre + (debut_ligne[ouvre..].len() - apres.len());
                    sortie.push_str(&rest[..consomme]);
                    pos += consomme;
                    continue;
                }
            }
        }
        if let Some((entrees, consomme)) = crate::langage::fonctions::parse_classe(rest, fonctions) {
            let manquantes = crate::langage::fonctions::methodes_sans_corps(&entrees);
            if !manquantes.is_empty() && !crate::langage::fonctions::est_abstraite(&entrees) {
                sortie.push_str(&format!(
                    "<rouge gras>{{⚠ {} laisse {} sans corps : ou bien la classe les définit, ou bien elle se déclare abstraite}}\n",
                    entrees[0].0,
                    manquantes.join(", ")
                ));
            } else {
                for (nom, f) in entrees {
                    fonctions.insert(nom, f);
                }
            }
            pos += consomme;
            if body[pos..].starts_with('\n') {
                pos += 1;
            }
            continue;
        }
        if let Some((nom, f, consomme)) =
            crate::langage::fonctions::parse_declaration(rest)
        {
            fonctions.insert(nom, f);
            pos += consomme;
            if body[pos..].starts_with('\n') {
                pos += 1;
            }
            continue;
        }
        if let Some(instr) = crate::langage::conteneurs::instruction_conteneur(rest, etat, boites, fonctions) {
            sortie.push_str(&instr.remplacement);
            pos += instr.consomme;
            if body[pos..].starts_with('\n') {
                pos += 1;
            }
            continue;
        }
        let fin = body[pos..].find('\n').map(|j| pos + j).unwrap_or(body.len());
        let ligne = &body[pos..fin];
        let mut courante = ligne.to_string();
        for (nom, valeur) in etat.iter() {
            courante = subst_var(&courante, nom, &crate::maths::calcul::format_number(*valeur));
        }
        let t = courante.trim_start();
        let mut gardee = true;
        if t.starts_with("soit ") && !est_ternaire(t) {
            let reste = t.trim_start_matches("soit").trim_start();
            if let Some((lhs, rhs)) = reste.split_once('=') {
                let (lhs, rhs) = (lhs.trim(), rhs.trim());
                if !lhs.contains('{') && !rhs.starts_with('<') && !rhs.starts_with('{') {
                    let rhs2 = crate::langage::fonctions::resoudre_appels(rhs, etat, boites, fonctions, 0);
                    let rhs2 = crate::langage::conteneurs::resoudre_lectures(&rhs2, etat, boites, true);
                    if let Some(v) = crate::maths::calcul::eval(&rhs2, etat) {
                        etat.insert(lhs.to_string(), v);
                        gardee = false;
                    }
                }
            }
        }
        if gardee {
            sortie.push_str(&courante);
        }
        if fin < body.len() {
            sortie.push('\n');
        }
        pos = fin + 1;
    }
    sortie
}

pub(crate) fn execute_conteneurs(
    texte: &str,
    vars: &std::collections::BTreeMap<String, f64>,
    boites: &mut crate::langage::conteneurs::Boites,
    fonctions: &mut crate::langage::fonctions::Fonctions,
    noms_math: &std::collections::BTreeSet<String>,
) -> String {
    let mut sortie = String::new();
    let mut pos = 0usize;
    while pos < texte.len() {
        let rest = &texte[pos..];
        if let Some((entrees, consomme)) = crate::langage::fonctions::parse_classe(rest, fonctions) {
            let manquantes = crate::langage::fonctions::methodes_sans_corps(&entrees);
            if !manquantes.is_empty() && !crate::langage::fonctions::est_abstraite(&entrees) {
                sortie.push_str(&format!(
                    "<rouge gras>{{⚠ {} laisse {} sans corps : ou bien la classe les définit, ou bien elle se déclare abstraite}}\n",
                    entrees[0].0,
                    manquantes.join(", ")
                ));
            } else {
                for (nom, f) in entrees {
                    fonctions.insert(nom, f);
                }
            }
            pos += consomme;
            if texte[pos..].starts_with('\n') {
                pos += 1;
            }
            continue;
        }
        if let Some((nom, f, consomme)) =
            crate::langage::fonctions::parse_declaration(rest)
        {
            if noms_math.contains(&nom) {
                sortie.push_str(&format!(
                    "<rouge gras>{{⚠ {} est déjà une fonction mathématique : un nom ne peut désigner qu'un seul objet}}\n",
                    nom
                ));
            } else {
                fonctions.insert(nom, f);
            }
            pos += consomme;
            if texte[pos..].starts_with('\n') {
                pos += 1;
            }
            continue;
        }
        if let Some(instr) = crate::langage::conteneurs::instruction_conteneur(rest, vars, boites, fonctions) {
            sortie.push_str(&instr.remplacement);
            pos += instr.consomme;
            if texte[pos..].starts_with('\n') {
                pos += 1;
            }
            continue;
        }
        let fin = texte[pos..].find('\n').map(|j| pos + j + 1).unwrap_or(texte.len());
        sortie.push_str(&texte[pos..fin]);
        pos = fin;
    }
    sortie
}

pub(crate) fn expand_loops_avec(
    src: &str,
    vars: &mut std::collections::BTreeMap<String, f64>,
    boites: &mut crate::langage::conteneurs::Boites,
    fonctions: &mut crate::langage::fonctions::Fonctions,
    noms_math: &std::collections::BTreeSet<String>,
) -> String {
    let mut out = String::new();
    let mut rest = src.to_string();
    loop {
        let mut found = None;
        {
            let mut i = 0usize;
            while i < rest.len() {
                let fin_ligne = rest[i..].find('\n').map(|j| i + j).unwrap_or(rest.len());
                let line = &rest[i..fin_ligne];
                let t = line.trim_start();

                if t.starts_with("soit ") {
                    if let Some(consomme) = crate::langage::fonctions::fin_de_classe(&rest[i..]) {
                        i += consomme;
                        continue;
                    }
                    if let Some((_, _, consomme)) =
                        crate::langage::fonctions::parse_declaration(&rest[i..])
                    {
                        i += consomme;
                        continue;
                    }
                }
                if t.starts_with("pour ")
                    && t.contains('{')
                    && (t.contains(" dans ") || (t.contains(" de ") && t.contains(" à ")))
                {
                    found = Some(i + (line.len() - t.len()));
                    break;
                }
                i = fin_ligne + 1;
            }
        }
        match found {
            Some(start) => {
                let prefixe = execute_conteneurs(&rest[..start], vars, boites, fonctions, noms_math);
                let head_end = accolade_du_corps(&rest[start..]).unwrap_or(0) + start;
                let head = &rest[start..head_end];
                let spec = head.trim_start_matches("pour").trim();
                let vide: std::collections::BTreeMap<String, f64> = std::collections::BTreeMap::new();
                let mut values: Vec<String> = Vec::new();

                let mut objets: Option<Vec<crate::langage::conteneurs::Valeur>> = None;
                let mut faute: Option<String> = None;
                let var;
                if let Some((v, liste)) = spec.split_once(" dans ") {
                    var = v.trim().to_string();
                    let liste = liste.trim();
                    if liste.starts_with('{') {
                        let inner = liste.trim_start_matches('{').trim_end_matches('}');
                        for item in crate::langage::conteneurs::decoupe_elements(inner) {
                            if !item.is_empty() {
                                values.push(item);
                            }
                        }
                    } else if let Some(b) = boites.get(liste) {
                        objets = crate::langage::conteneurs::valeurs_pour_boucle(b);
                        values = crate::langage::conteneurs::elements_pour_boucle(b);
                    } else if let Ok((val, type_val)) =
                        crate::langage::fonctions::devine_valeur(liste, vars, boites, fonctions)
                    {

                        let boite = crate::langage::conteneurs::Boite { type_val, val };
                        objets = crate::langage::conteneurs::valeurs_pour_boucle(&boite);
                        values = crate::langage::conteneurs::elements_pour_boucle(&boite);
                    } else if liste.starts_with('[') {
                        let inner = liste.trim_start_matches('[').trim_end_matches(']');
                        for item in inner.split(',') {
                            let item = item.trim();
                            if !item.is_empty() {
                                values.push(item.to_string());
                            }
                        }
                    } else {

                        faute = Some(format!(
                            "{} n'est pas un conteneur : une chaîne écrite sur place se met entre guillemets",
                            liste
                        ));
                    }
                } else {
                    let p1: Vec<&str> = spec.splitn(2, " de ").collect();
                    var = p1[0].trim().to_string();
                    let reste = p1.get(1).copied().unwrap_or("1 à 1");
                    let p2: Vec<&str> = reste.splitn(2, " à ").collect();
                    let a = crate::maths::calcul::eval(p2[0].trim(), &vide).unwrap_or(1.0);
                    let fin = p2.get(1).copied().unwrap_or("1");
                    let (fin, pas) = match fin.split_once("avec un pas de") {
                        Some((f, p)) => (
                            f.trim().trim_end_matches(','),
                            crate::maths::calcul::eval(p.trim(), &vide).unwrap_or(1.0),
                        ),
                        None => (fin.trim(), 1.0),
                    };
                    let b = crate::maths::calcul::eval(fin, &vide).unwrap_or(1.0);
                    let mut n = a;
                    let eps = pas.abs() * 1e-9;
                    while (pas > 0.0 && n <= b + eps) || (pas < 0.0 && n >= b - eps) {
                        values.push(crate::maths::calcul::format_number(n));
                        n += pas;
                        if values.len() >= TOURS_MAX {
                            break;
                        }
                    }
                }
                let tail = &rest[head_end..];
                if let Some((body, after)) = take_group(tail, 0) {
                    let mut expanded = String::new();
                    if let Some(message) = &faute {
                        expanded.push_str(&format!("<rouge gras>{{⚠ {}}}\n", message));
                    }
                    let body = dedent(&body);
                    let debordement = values.len() >= TOURS_MAX;
                    values.truncate(TOURS_MAX);
                    let avant = vars.clone();
                    for (rang, v) in values.iter().enumerate() {
                        if let Some(n) = crate::maths::calcul::eval(v, &vide) {
                            vars.insert(var.clone(), n);
                        }
                        let substituee = match objets.as_ref().and_then(|o| o.get(rang)) {
                            Some(objet) => {

                                let nom_du_tour = format!("{}__{}", var, rang);
                                boites.insert(
                                    nom_du_tour.clone(),
                                    crate::langage::conteneurs::Boite {
                                        type_val: crate::langage::conteneurs::type_de_valeur(objet),
                                        val: objet.clone(),
                                    },
                                );
                                subst_var(&body, &var, &nom_du_tour)
                            }
                            None => subst_var(&body, &var, v),
                        };
                        let substituee = if substituee.contains("pour ") {
                            expand_loops_avec(&substituee, vars, boites, fonctions, noms_math)
                        } else {
                            substituee
                        };

                        let substituee = if substituee.contains("si ") {
                            expand_conditions_avec(&substituee, vars, Some(boites), Some(fonctions))
                        } else {
                            substituee
                        };
                        if let Some(retenu) = condition_de_tour(&substituee, vars, boites, fonctions) {
                            let contenu = applique_affectations(&retenu, vars, boites, fonctions);

                            if let Some(coupe) = position_du_mot(&contenu, "continuer") {

                                let garde = &contenu[..coupe];
                                if !garde.trim().is_empty() {
                                    expanded.push_str(garde);
                                    expanded.push_str("\n\u{E013}\n");
                                }
                                continue;
                            }
                            if let Some(coupe) = position_de_sortir(&contenu) {
                                expanded.push_str(&contenu[..coupe]);
                                expanded.push_str("\n\u{E013}\n");
                                break;
                            }
                            expanded.push_str(&contenu);
                            expanded.push_str("\n\u{E013}\n");
                        }
                    }
                    vars.remove(&var);
                    for (nom, valeur) in vars.iter() {
                        if avant.get(nom) != Some(valeur) {
                            expanded.push_str(&format!(
                                "soit {} = {}\n\u{E013}\n",
                                nom,
                                crate::maths::calcul::format_number(*valeur)
                            ));
                        }
                    }
                    if debordement {
                        expanded.push_str(&format!(
                            "La boucle a été arrêtée après {} tours.\n\u{E013}\n",
                            TOURS_MAX
                        ));
                    }
                    out.push_str(&prefixe);
                    rest = format!("{}\n{}", expanded, after);
                } else {
                    out.push_str(&prefixe);
                    out.push_str(&rest[start..head_end + 1]);
                    rest = rest[head_end + 1..].to_string();
                }
            }
            None => {
                out.push_str(&execute_conteneurs(&rest, vars, boites, fonctions, noms_math));
                break;
            }
        }
    }
    out
}

pub(crate) fn subst_vars_multi(s: &str, vars: &std::collections::BTreeMap<String, f64>) -> String {
    let mut out = s.to_string();
    for (nom, val) in vars {
        out = subst_var(&out, nom, &crate::maths::calcul::format_number(*val));
    }
    out
}

pub(crate) fn subst_vars_partout(
    s: &str,
    vars: &std::collections::BTreeMap<String, f64>,
) -> String {
    let lettres: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < lettres.len() {
        if lettres[i].is_alphanumeric() || lettres[i] == '_' {
            let debut = i;
            while i < lettres.len() && (lettres[i].is_alphanumeric() || lettres[i] == '_') {
                i += 1;
            }
            let mot: String = lettres[debut..i].iter().collect();
            match vars.get(&mot) {
                Some(v) => out.push_str(&crate::maths::calcul::format_number(*v)),
                None => out.push_str(&mot),
            }
        } else {
            out.push(lettres[i]);
            i += 1;
        }
    }
    out
}

#[derive(PartialEq)]
pub(crate) enum IssueDeTour {
    Normale,
    Continuer,
    Sortir,
}

pub(crate) fn condition_resolue(
    cond_txt: &str,
    vars: &std::collections::BTreeMap<String, f64>,
    boites: &crate::langage::conteneurs::Boites,
    fonctions: &crate::langage::fonctions::Fonctions,
) -> String {
    crate::langage::conteneurs::resoudre_noms_scalaires(
        &crate::langage::conteneurs::resoudre_lectures(
            &crate::langage::fonctions::resoudre_appels(cond_txt, vars, boites, fonctions, 0),
            vars,
            boites,
            true,
        ),
        boites,
    )
}

pub(crate) fn corps_de_bloc(brut: &str) -> String {
    let plat = dedent(brut);
    let sans_tete = plat.strip_prefix('\n').unwrap_or(&plat);
    sans_tete.trim_end_matches([' ', '\t']).to_string()
}

pub(crate) fn traite_tour_tant_que(
    body: &str,
    vars: &mut std::collections::BTreeMap<String, f64>,
    boites: &mut crate::langage::conteneurs::Boites,
    fonctions: &crate::langage::fonctions::Fonctions,
) -> (String, IssueDeTour) {
    let mut out = String::new();
    let mut i = 0usize;
    while i < body.len() {
        let fin_ligne = body[i..].find('\n').map(|j| i + j).unwrap_or(body.len());
        let line = &body[i..fin_ligne];
        let t = line.trim_start();

        if t == "sortir" {
            return (out, IssueDeTour::Sortir);
        }
        if t == "continuer" {
            return (out, IssueDeTour::Continuer);
        }

        if t.starts_with("si ") && t.contains('{') {
            let debut = i + (line.len() - t.len());
            if let Some(rel) = body[debut..].find('{') {
                let head_end = debut + rel;
                let cond_txt = body[debut + 3..head_end].trim().to_string();
                if let Some((alors, apres)) = take_group(&body[head_end..], 0) {
                    let apres_t = apres.trim_start();
                    let (sinon, suite) = if let Some(r) = apres_t.strip_prefix("sinon") {
                        let r = r.trim_start();
                        match take_group(r, 0) {
                            Some((b, s)) => (Some(b), s),
                            None => (None, apres.clone()),
                        }
                    } else {
                        (None, apres.clone())
                    };
                    let condition = condition_resolue(&cond_txt, vars, boites, fonctions);
                    let branche = if eval_condition(&condition, vars) {
                        Some(corps_de_bloc(&alors))
                    } else {
                        sinon.map(|b| corps_de_bloc(&b))
                    };
                    if let Some(b) = branche {
                        let (texte, issue) = traite_tour_tant_que(&b, vars, boites, fonctions);
                        out.push_str(&texte);
                        if issue != IssueDeTour::Normale {
                            return (out, issue);
                        }
                    }
                    let suite = suite.strip_prefix('\n').unwrap_or(&suite).to_string();
                    let (texte, issue) = traite_tour_tant_que(&suite, vars, boites, fonctions);
                    out.push_str(&texte);
                    return (out, issue);
                }
            }
        }

        out.push_str(&traite_ligne_de_tour(line, t, vars, boites, fonctions));
        i = fin_ligne + 1;
    }
    (out, IssueDeTour::Normale)
}

pub(crate) fn traite_ligne_de_tour(
    line: &str,
    t: &str,
    vars: &mut std::collections::BTreeMap<String, f64>,
    boites: &mut crate::langage::conteneurs::Boites,
    fonctions: &crate::langage::fonctions::Fonctions,
) -> String {
    if let Some(reste) = t.strip_prefix("soit ") {
        if let Some((lhs, rhs)) = reste.split_once('=') {
            let lhs = lhs.trim();
            let rhs = rhs.trim();
            if !lhs.contains('{') && !rhs.starts_with('{') && !rhs.starts_with('<') {
                if let Some(v) = crate::maths::calcul::eval(rhs, vars) {
                    vars.insert(lhs.to_string(), v);
                    return String::new();
                }
            }
        }
    }

    let ligne_seule = format!("{}\n", line);
    if let Some(instr) =
        crate::langage::conteneurs::instruction_conteneur(&ligne_seule, vars, boites, fonctions)
    {
        return instr.remplacement;
    }
    let croissance = t.starts_with("soit ")
        && t.split_once('=').map(|(_, d)| d.contains('{')).unwrap_or(false);
    let mut out = if croissance {
        subst_vars_partout(line, vars)
    } else {
        subst_vars_multi(line, vars)
    };
    out.push('\n');
    out
}

pub(crate) fn reaffectations_finales(
    avant: &std::collections::BTreeMap<String, f64>,
    apres: &std::collections::BTreeMap<String, f64>,
) -> String {
    let mut out = String::new();
    for (nom, val_avant) in avant {
        if let Some(val_apres) = apres.get(nom) {
            if (val_apres - val_avant).abs() > 1e-12 {
                out.push_str(&format!(
                    "soit {} = {}\n",
                    nom,
                    crate::maths::calcul::format_number(*val_apres)
                ));
            }
        }
    }
    out
}

pub(crate) fn expand_tant_que_avec(
    src: &str,
    vars_init: &std::collections::BTreeMap<String, f64>,
    boites: &mut crate::langage::conteneurs::Boites,
    fonctions: &crate::langage::fonctions::Fonctions,
) -> String {
    let mut out = String::new();
    let mut rest = src.to_string();
    loop {
        let mut avant = None;
        let mut apres = None;
        {
            let mut i = 0usize;
            while i < rest.len() {
                let fin_ligne = rest[i..].find('\n').map(|j| i + j).unwrap_or(rest.len());
                let line = &rest[i..fin_ligne];
                let t = line.trim_start();

                if t.starts_with("soit ") {
                    if let Some(consomme) = crate::langage::fonctions::fin_de_classe(&rest[i..]) {
                        i += consomme;
                        continue;
                    }
                    if let Some((_, _, consomme)) =
                        crate::langage::fonctions::parse_declaration(&rest[i..])
                    {
                        i += consomme;
                        continue;
                    }
                }
                if t.starts_with("tant que ") && t.contains('{') {
                    avant = Some(i + (line.len() - t.len()));
                    break;
                }
                if (t == "faire" || t.starts_with("faire ") || t.starts_with("faire{"))
                    && t.contains('{')
                {
                    apres = Some(i + (line.len() - t.len()));
                    break;
                }
                i = fin_ligne + 1;
            }
        }
        if let Some(start) = avant {
            let head_end = rest[start..].find('{').unwrap() + start;
            let head = rest[start..head_end].trim();
            let head = head.trim_start_matches("tant que").trim();
            let cond_txt = head.trim_end_matches("faire").trim().to_string();
            let tail = &rest[head_end..];
            if let Some((body, after)) = take_group(tail, 0) {
                let body = dedent(&body);
                let mut vars = vars_init.clone();
                let mut expanded = String::new();
                let mut tours = 0usize;
                while eval_condition(
                    &condition_resolue(&cond_txt, &vars, boites, fonctions),
                    &vars,
                ) && tours < TOURS_MAX
                {
                    let (texte, issue) =
                        traite_tour_tant_que(&body, &mut vars, boites, fonctions);
                    tours += 1;

                    if !(texte.trim().is_empty() && issue != IssueDeTour::Normale) {
                        expanded.push_str(&texte);
                        expanded.push_str("\n\u{E013}\n");
                    }
                    if issue == IssueDeTour::Sortir {
                        break;
                    }
                }
                expanded.push_str(&reaffectations_finales(vars_init, &vars));
                out.push_str(&rest[..start]);
                rest = format!("{}\n{}", expanded, after);
            } else {
                out.push_str(&rest[..head_end + 1]);
                rest = rest[head_end + 1..].to_string();
            }
        } else if let Some(start) = apres {
            let head_end = rest[start..].find('{').unwrap() + start;
            let tail = &rest[head_end..];
            if let Some((body, after)) = take_group(tail, 0) {
                let apres_t = after.trim_start();
                if let Some(r) = apres_t.strip_prefix("tant que") {
                    let fin_ligne = r.find('\n').unwrap_or(r.len());
                    let cond_txt = r[..fin_ligne].trim().to_string();
                    let reste_apres = &r[fin_ligne..];
                    let body = dedent(&body);
                    let mut vars = vars_init.clone();
                    let mut expanded = String::new();
                    let mut tours = 0usize;
                    loop {
                        let (texte, issue) =
                            traite_tour_tant_que(&body, &mut vars, boites, fonctions);
                        tours += 1;
                        if !(texte.trim().is_empty() && issue != IssueDeTour::Normale) {
                            expanded.push_str(&texte);
                            expanded.push_str("\n\u{E013}\n");
                        }
                        if issue == IssueDeTour::Sortir {
                            break;
                        }
                        let condition = condition_resolue(&cond_txt, &vars, boites, fonctions);
                        if !eval_condition(&condition, &vars) || tours >= TOURS_MAX {
                            break;
                        }
                    }
                    expanded.push_str(&reaffectations_finales(vars_init, &vars));
                    out.push_str(&rest[..start]);
                    rest = format!("{}\n{}", expanded, reste_apres);
                } else {
                    out.push_str(&rest[..head_end + 1]);
                    rest = rest[head_end + 1..].to_string();
                }
            } else {
                out.push_str(&rest[..head_end + 1]);
                rest = rest[head_end + 1..].to_string();
            }
        } else {
            out.push_str(&rest);
            break;
        }
    }
    out
}

pub(crate) fn expand_conditions_avec(
    src: &str,
    vars: &std::collections::BTreeMap<String, f64>,
    boites: Option<&crate::langage::conteneurs::Boites>,
    fonctions: Option<&crate::langage::fonctions::Fonctions>,
) -> String {
    let mut rest = src.to_string();
    let mut out = String::new();
    loop {
        let found = rest.lines().scan(0usize, |off, line| {
            let start = *off;
            *off += line.len() + 1;
            Some((start, line))
        }).find(|(_, line)| {
            let t = line.trim_start();
            t.starts_with("si ") && t.contains('{')
        }).map(|(start, line)| start + (line.len() - line.trim_start().len()));
        let start = match found {
            Some(i) => i,
            None => {
                out.push_str(&rest);
                break;
            }
        };
        let head_end = match rest[start..].find('{') {
            Some(i) => start + i,
            None => {
                out.push_str(&rest);
                break;
            }
        };
        let cond_txt = rest[start + 3..head_end].trim().to_string();
        let cond_txt = match (boites, fonctions) {
            (Some(b), Some(f)) => {
                let resolue = crate::langage::fonctions::resoudre_appels(&cond_txt, vars, b, f, 0);
                crate::langage::conteneurs::resoudre_lectures(&resolue, vars, b, true)
            }
            _ => cond_txt,
        };
        let tail = &rest[head_end..];
        let (alors, apres) = match take_group(tail, 0) {
            Some(x) => x,
            None => {
                out.push_str(&rest);
                break;
            }
        };
        let apres_t = apres.trim_start();
        let (sinon, suite) = if let Some(r) = apres_t.strip_prefix("sinon") {
            let r = r.trim_start();
            match take_group(r, 0) {
                Some((b, s)) => (Some(b), s),
                None => (None, apres.clone()),
            }
        } else {
            (None, apres.clone())
        };
        let vrai = eval_condition(&cond_txt, vars);
        out.push_str(&rest[..start]);
        if vrai {
            out.push_str(&dedent(&alors));
        } else if let Some(b) = sinon {
            out.push_str(&dedent(&b));
        }
        rest = suite;
    }
    out
}

pub(crate) fn eval_operande(s: &str, vars: &std::collections::BTreeMap<String, f64>) -> Option<f64> {
    match s.trim() {
        "vrai" => Some(1.0),
        "faux" => Some(0.0),
        autre => crate::maths::calcul::eval(autre, vars),
    }
}

pub(crate) fn depouille_parentheses_globales(s: &str) -> String {
    let t = s.trim();
    if t.starts_with('(') && t.ends_with(')') {
        let interieur = &t[1..t.len() - 1];
        let mut profondeur = 0i32;
        let mut referme_avant_la_fin = false;
        for (idx, c) in interieur.char_indices() {
            match c {
                '(' => profondeur += 1,
                ')' => {
                    profondeur -= 1;
                    if profondeur < 0 && idx + 1 < interieur.len() {
                        referme_avant_la_fin = true;
                    }
                }
                _ => {}
            }
        }
        if profondeur == 0 && !referme_avant_la_fin {
            return interieur.trim().to_string();
        }
    }
    t.to_string()
}

pub(crate) fn parse_intervalle(s: &str) -> Option<(String, String)> {
    let s = s.trim();
    let i = s.find('[')?;
    let j = s.rfind(']')?;
    if j <= i {
        return None;
    }
    let (a, b) = s[i + 1..j].split_once(';')?;
    Some((a.trim().to_string(), b.trim().to_string()))
}

pub(crate) fn split_top_niveau<'a>(s: &'a str, sep: &str) -> Option<(&'a str, &'a str)> {
    let mut profondeur = 0i32;
    for (i, c) in s.char_indices() {
        match c {
            '(' => profondeur += 1,
            ')' => profondeur -= 1,
            _ => {}
        }
        if profondeur == 0 && s[i..].starts_with(sep) {
            return Some((&s[..i], &s[i + sep.len()..]));
        }
    }
    None
}
pub fn evalue_condition_publique(
    cond: &str,
    vars: &std::collections::BTreeMap<String, f64>,
) -> bool {
    eval_condition(cond, vars)
}

pub(crate) fn eval_condition(cond: &str, vars: &std::collections::BTreeMap<String, f64>) -> bool {
    let cond = depouille_parentheses_globales(cond);
    let cond: &str = &cond;
    if let Some((g, d)) = split_top_niveau(cond, " ou ") {
        return eval_condition(g, vars) || eval_condition(d, vars);
    }

    if let Some(reste) = cond.trim().strip_prefix("non ") {
        return !eval_condition(reste, vars);
    }
    match cond.trim() {
        "vrai" => return true,
        "faux" => return false,
        _ => {}
    }
    if let Some((g, d)) = split_top_niveau(cond, " et ") {
        return eval_condition(g, vars) && eval_condition(d, vars);
    }
    eval_condition_simple(cond, vars)
}

pub(crate) fn eval_condition_simple(cond: &str, vars: &std::collections::BTreeMap<String, f64>) -> bool {
    let cond = depouille_parentheses_globales(cond);
    let cond: &str = &cond;
    if let Some((g, d)) = cond.split_once("n'appartient pas à") {
        return match (eval_operande(g, vars), parse_intervalle(d)) {
            (Some(v), Some((a, b))) => match (eval_operande(&a, vars), eval_operande(&b, vars)) {
                (Some(a), Some(b)) => !(v >= a && v <= b),
                _ => false,
            },
            _ => false,
        };
    }
    if let Some((g, d)) = cond.split_once("appartient à") {
        return match (eval_operande(g, vars), parse_intervalle(d)) {
            (Some(v), Some((a, b))) => match (eval_operande(&a, vars), eval_operande(&b, vars)) {
                (Some(a), Some(b)) => v >= a && v <= b,
                _ => false,
            },
            _ => false,
        };
    }
    let comparateurs: &[(&str, fn(f64, f64) -> bool)] = &[
        ("au moins", |a, b| a >= b),
        ("au plus", |a, b| a <= b),
        ("moins de", |a, b| a < b),
        ("plus de", |a, b| a > b),
        ("différent de", |a, b| (a - b).abs() > 1e-9),
        ("vaut", |a, b| (a - b).abs() < 1e-9),
        (">=", |a, b| a >= b),
        ("<=", |a, b| a <= b),
        ("!=", |a, b| (a - b).abs() > 1e-9),
        ("<", |a, b| a < b),
        (">", |a, b| a > b),
        ("=", |a, b| (a - b).abs() < 1e-9),
    ];
    for (mot, f) in comparateurs {
        if let Some((g, d)) = cond.split_once(mot) {
            let (gt, dt) = (g.trim().to_string(), d.trim().to_string());
            let g = eval_operande(g, vars);
            let d = eval_operande(d, vars);
            if let (Some(g), Some(d)) = (g, d) {
                return f(g, d);
            }

            if !gt.is_empty() && !dt.is_empty() {
                let cle = |s: &str| -> String {
                    s.trim_matches('"')
                        .chars()
                        .map(crate::langage::conteneurs::depouille)
                        .flat_map(|c| c.to_lowercase())
                        .collect()
                };
                let ordre = cle(&gt).cmp(&cle(&dt));
                let (a, b) = match ordre {
                    std::cmp::Ordering::Less => (0.0, 1.0),
                    std::cmp::Ordering::Equal => (0.0, 0.0),
                    std::cmp::Ordering::Greater => (1.0, 0.0),
                };
                return f(a, b);
            }
            return false;
        }
    }
    eval_operande(cond, vars).map(|v| v.abs() > 1e-9).unwrap_or(false)
}
