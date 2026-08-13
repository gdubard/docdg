//! Le contrôle de flux du document : boucles, conditions, affectations.
//!
//! Ce module tenait dans `rendu.rs`, qui mêlait sur plus de quatre mille
//! lignes l'analyse de la source, l'exécution des boucles, les saisies
//! interactives et la composition du HTML. C'est là que `sortir` avait fini
//! par fonctionner dans les boucles `pour` sans fonctionner dans les
//! `tant que` : deux traitements du même mot, à six cents lignes l'un de
//! l'autre, que rien ne rapprochait.
//!
//! Ce qui relève de l'exécution est ici ; ce qui relève du rendu reste chez
//! le voisin.

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

/// Repère une ligne réduite au seul mot `sortir` et rend sa position.
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
/// `somme = somme + k` — l'affectation d'un nom **déjà déclaré**.
///
/// `soit` déclare ; le répéter à chaque tour n'apprend rien et ne se lit pas.
/// La condition tient en un mot : le nom doit déjà exister. Une ligne de prose
/// qui contient un signe égal n'est donc jamais prise pour une affectation —
/// c'est ce qui rend la levée sûre.
///
/// La réécriture se fait ici, sur le texte, avant que quoi que ce soit ne
/// mesure des positions : rien en aval n'a à changer.
pub(crate) fn normalise_affectations(texte: &str) -> String {
    let mut connus: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut sortie = String::with_capacity(texte.len());
    for ligne in texte.lines() {
        let nu = ligne.trim_start();
        let retrait = &ligne[..ligne.len() - nu.len()];

        // ce que la ligne déclare
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

        // ce que la ligne affecte
        let mut reecrite = None;
        if !nu.starts_with("soit ") && !nu.starts_with('<') && !nu.starts_with('[') {
            if let Some((gauche, droite)) = nu.split_once('=') {
                let nom = gauche.trim();
                // Un dièse à droite signe la prose : on écrit `k=#k` pour
                // afficher, jamais pour affecter — dans du code, le nom se
                // lit directement.
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
                // Une déclaration de fonction emporte son corps : les boucles
                // qu'il contient appartiennent à la fonction, non au document.
                // Sans ce saut, la coupure tombait au milieu de la déclaration,
                // dont l'accolade fermante restait de l'autre côté — la
                // fonction n'était alors jamais enregistrée.
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
                // Une collection d'objets ne se parcourt pas par substitution
                // de texte : l'objet ne survivrait pas à son impression. Ses
                // valeurs voyagent telles quelles, en boîte, un tour après
                // l'autre.
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
                        // La source n'est pas forcément un nom : une chaîne
                        // écrite sur place se parcourt lettre à lettre, comme
                        // celle qu'on aurait posée dans une variable.
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
                        // Une source qui n'est ni un nom posé, ni un littéral,
                        // ni une chaîne entre guillemets se disait en silence
                        // comme un unique tour — `pour c dans un code` faisait
                        // un tour sur « un code » au lieu d'en lire les
                        // lettres. La faute vaut mieux que le faux-semblant.
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
                                // Chaque tour reçoit sa propre boîte, sous un
                                // nom qui lui est propre : le contenu du tour
                                // est déroulé maintenant mais lu plus tard,
                                // et un nom réemployé ne porterait que la
                                // dernière valeur.
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
                        // Les conditions du tour sont résolues ici : sans quoi
                        // un `sortir` niché dans un `si` resterait invisible,
                        // les conditions n'étant développées qu'après les
                        // boucles.
                        let substituee = if substituee.contains("si ") {
                            expand_conditions_avec(&substituee, vars, Some(boites), Some(fonctions))
                        } else {
                            substituee
                        };
                        if let Some(retenu) = condition_de_tour(&substituee, vars, boites, fonctions) {
                            let contenu = applique_affectations(&retenu, vars, boites, fonctions);
                            // `sortir` arrête ce tour à cet endroit, et la
                            // boucle avec lui : c'est la recherche qui cesse
                            // dès qu'elle a trouvé.
                            // `continuer` coupe le tour, la boucle poursuit.
                            if let Some(coupe) = position_du_mot(&contenu, "continuer") {
                                // Un tour sauté ne produit rien : lui donner
                                // son séparateur ferait apparaître une ligne
                                // vide là où il n'y a précisément rien.
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

/// Substitue les variables **jusque dans les accolades**.
///
/// `subst_var` les protège, et c'est ce qu'il faut : une accolade délimite des
/// données, qu'on ne veut pas voir réécrites. Mais dans le corps d'un `tant
/// que`, `soit v = v + {k}` a besoin de la valeur du tour — sans quoi
/// l'accumulateur ne reçoit que la valeur d'avant la boucle. La levée ne vaut
/// donc que pour les lignes qui font croître un conteneur.
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

/// Ce qui met fin à un tour de boucle. `sortir` arrête la boucle avec lui,
/// `continuer` passe au tour suivant — les deux mots du programme de NSI.
#[derive(PartialEq)]
pub(crate) enum IssueDeTour {
    Normale,
    Continuer,
    Sortir,
}

/// La condition d'une boucle, ramenée à quelque chose d'évaluable : les appels
/// de fonction résolus, les lectures de conteneur faites, les scalaires
/// remplacés par leur valeur. Les trois passes allaient toujours ensemble et
/// s'écrivaient à la main à chaque endroit qui en avait besoin.
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

/// Le contenu d'un bloc, débarrassé du saut de ligne qui suit l'accolade
/// ouvrante et de l'indentation qui précède la fermante. Sans ce nettoyage,
/// chaque `si` d'un corps de boucle ajoutait une ligne vide au document.
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

        // Les deux sorties, avant tout le reste : ce qui suit dans le tour ne
        // doit ni s'exécuter ni s'afficher.
        if t == "sortir" {
            return (out, IssueDeTour::Sortir);
        }
        if t == "continuer" {
            return (out, IssueDeTour::Continuer);
        }

        // Un `si` du corps se résout **ici**, avec les variables telles
        // qu'elles sont à ce point du tour — et non telles qu'elles étaient
        // en entrant. Sans quoi un `sortir` niché dans un `si` restait
        // invisible : les conditions n'étaient développées qu'après la
        // boucle, quand il était trop tard pour l'arrêter.
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
    // Une opération sur conteneur s'applique **pendant** le tour : c'est
    // le seul moment où la variable de boucle a sa valeur du tour.
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

/// Comme les conditions et les boucles `pour`, un `tant que` de document doit
/// voir les conteneurs : sa condition peut les interroger, et son corps les
/// faire croître. Sans cela `soit v = v + {k}` n'y accumulait que la valeur
/// que `k` avait avant la boucle — les accolades étant protégées de la
/// substitution, le tour ne pouvait rien y injecter.
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
                // Même règle que pour les boucles `pour` : le corps d'une
                // fonction n'appartient pas au document. Sans ce saut, la
                // boucle était extraite du corps et la fonction rendait la
                // valeur qu'elle avait avant d'entrer dedans.
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
                    // Un tour interrompu qui ne produit rien n'a pas de
                    // séparateur : lui en donner un ferait apparaître une
                    // ligne vide là où il n'y a précisément rien.
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
/// La condition d'un `si` de document doit voir les conteneurs et les
/// fonctions, sans quoi `si v contient(1)` reste du texte mort — alors que la
/// même écriture fonctionne dans un corps de fonction.
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
    // La négation, troisième connecteur logique enseigné avec « et » et « ou ».
    // Elle se lit après eux : « non a et b » se comprend « (non a) et b ».
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
            // Deux textes se comparent aussi : l'égalité d'un palindrome à son
            // inverse n'est pas un calcul de nombres. La collation française
            // sert d'ordre — « école » avant « Zoé ».
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
