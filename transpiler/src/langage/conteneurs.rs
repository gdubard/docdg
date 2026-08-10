use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeVal {
    Entier,
    Decimal,
    Reel,
    Complexe,
    Texte,
    Booleen,
    Collection(Box<TypeVal>),
    Dictionnaire(Box<TypeVal>),
    Matrice(Option<(usize, usize)>, Box<TypeVal>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Valeur {
    Nombre(f64),
    Complexe(f64, f64),
    Texte(String),
    Collection(Vec<Valeur>),
    Dictionnaire(Vec<(String, Valeur)>),
    Matrice(usize, usize, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Boite {
    pub type_val: TypeVal,
    pub val: Valeur,
}

pub type Boites = BTreeMap<String, Boite>;

fn nom_type(t: &TypeVal) -> String {
    match t {
        TypeVal::Entier => "entiers".into(),
        TypeVal::Decimal => "décimaux".into(),
        TypeVal::Reel => "réels".into(),
        TypeVal::Complexe => "complexes".into(),
        TypeVal::Texte => "textes".into(),
        TypeVal::Booleen => "booléens".into(),
        TypeVal::Collection(e) => format!("collections de {}", nom_type(e)),
        TypeVal::Dictionnaire(e) => format!("dictionnaires de textes et de {}", nom_type(e)),
        TypeVal::Matrice(Some((l, c)), e) => format!("matrices {}×{} de {}", l, c, nom_type(e)),
        TypeVal::Matrice(None, e) => format!("matrices de {}", nom_type(e)),
    }
}

fn sans_article(s: &str) -> &str {
    let s = s.trim();
    for a in ["une ", "un ", "des ", "de ", "d'"] {
        if let Some(r) = s.strip_prefix(a) {
            return r.trim_start();
        }
    }
    s
}

fn lit_dimensions(s: &str) -> Option<((usize, usize), &str)> {
    let s = s.trim_start();
    let fin = s.find(|c: char| c.is_whitespace())?;
    let mot = &s[..fin];
    let (a, b) = mot.split_once('×').or_else(|| mot.split_once('x'))?;
    let l = a.trim().parse::<usize>().ok()?;
    let c = b.trim().parse::<usize>().ok()?;
    Some(((l, c), &s[fin..]))
}

pub fn parse_type(s: &str) -> Option<TypeVal> {
    let s = sans_article(s);
    if let Some(r) = s.strip_prefix("collections").or_else(|| s.strip_prefix("collection")) {
        return Some(TypeVal::Collection(Box::new(parse_type(sans_article(r))?)));
    }
    if let Some(r) = s.strip_prefix("dictionnaire") {
        let r = sans_article(r);
        let r = r.strip_prefix("textes")?.trim_start();
        let r = r.strip_prefix("et")?.trim_start();
        return Some(TypeVal::Dictionnaire(Box::new(parse_type(sans_article(r))?)));
    }
    let apres_matrice = s
        .strip_prefix("matrices")
        .or_else(|| s.strip_prefix("matrice"));
    if let Some(r) = apres_matrice {
        if let Some((dims, reste)) = lit_dimensions(r) {
            return Some(TypeVal::Matrice(
                Some(dims),
                Box::new(parse_type(sans_article(reste))?),
            ));
        }
        return Some(TypeVal::Matrice(None, Box::new(parse_type(sans_article(r))?)));
    }
    match s.trim_end_matches(|c: char| c == '.' || c == ';') {
        "entiers" | "entier" => Some(TypeVal::Entier),
        "décimaux" | "decimaux" | "décimal" | "decimal" => Some(TypeVal::Decimal),
        "réels" | "reels" | "réel" | "reel" => Some(TypeVal::Reel),
        "complexes" | "complexe" => Some(TypeVal::Complexe),
        "textes" | "texte" => Some(TypeVal::Texte),
        "booléens" | "booleens" | "booléen" | "booleen" => Some(TypeVal::Booleen),
        _ => None,
    }
}

fn coupe_niveau_zero(s: &str, sep: char) -> Vec<&str> {
    let mut morceaux = Vec::new();
    let mut profondeur = 0i32;
    let mut debut = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '{' | '(' => profondeur += 1,
            '}' | ')' => profondeur -= 1,
            c2 if c2 == sep && profondeur == 0 => {
                morceaux.push(&s[debut..i]);
                debut = i + c2.len_utf8();
            }
            _ => {}
        }
    }
    morceaux.push(&s[debut..]);
    morceaux
}

fn nombre_de(v: &Valeur) -> Option<f64> {
    match v {
        Valeur::Nombre(n) => Some(*n),
        _ => None,
    }
}

fn est_entier(n: f64) -> bool {
    (n - n.round()).abs() < 1e-9
}

fn est_decimal(n: f64) -> bool {
    if !n.is_finite() {
        return false;
    }
    if est_entier(n) {
        return true;
    }
    let mut echelle = 1.0f64;
    for _ in 0..9 {
        echelle *= 10.0;
        let mis = n * echelle;
        if mis.abs() > 1e15 {
            return false;
        }
        if (mis - mis.round()).abs() < 1e-6 {
            return true;
        }
    }
    false
}

pub fn verifie(t: &TypeVal, v: &Valeur) -> Result<(), String> {
    match (t, v) {
        (TypeVal::Entier, Valeur::Nombre(n)) if est_entier(*n) => Ok(()),
        (TypeVal::Entier, Valeur::Nombre(n)) => {
            Err(format!("{} n'est pas un entier", crate::maths::calcul::format_number(*n)))
        }
        (TypeVal::Decimal, Valeur::Nombre(n)) if est_decimal(*n) => Ok(()),
        (TypeVal::Decimal, Valeur::Nombre(n)) => Err(format!(
            "{} n'est pas un décimal (son développement décimal n'est pas fini)",
            crate::maths::calcul::format_number(*n)
        )),
        (TypeVal::Reel, Valeur::Nombre(_)) => Ok(()),
        (TypeVal::Complexe, Valeur::Complexe(_, _)) => Ok(()),
        (TypeVal::Booleen, Valeur::Nombre(n)) if *n == 0.0 || *n == 1.0 => Ok(()),
        (TypeVal::Texte, Valeur::Texte(_)) => Ok(()),
        (TypeVal::Collection(e), Valeur::Collection(elems)) => {
            for el in elems {
                verifie(e, el)?;
            }
            Ok(())
        }
        (TypeVal::Dictionnaire(e), Valeur::Dictionnaire(paires)) => {
            for (_, el) in paires {
                verifie(e, el)?;
            }
            Ok(())
        }
        (TypeVal::Matrice(dims, e), Valeur::Matrice(l, c, cases)) => {
            if let Some((dl, dc)) = dims {
                if dl != l || dc != c {
                    return Err(format!(
                        "la matrice déclarée {}×{} reçoit {} rangée(s) de {} colonne(s)",
                        dl, dc, l, c
                    ));
                }
            }
            if matches!(**e, TypeVal::Complexe) {
                return Err("une matrice de complexes n'est pas encore prise en charge".into());
            }
            for n in cases {
                verifie(e, &Valeur::Nombre(*n))?;
            }
            Ok(())
        }
        _ => Err(format!(
            "la valeur ne correspond pas au type ({})",
            nom_type(t)
        )),
    }
}

fn parse_element(
    brut: &str,
    attendu: &TypeVal,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fns: &super::fonctions::Fonctions,
) -> Result<Valeur, String> {
    let brut = brut.trim();
    if brut.starts_with('{') {
        return parse_litteral(brut, attendu, vars, boites, fns);
    }
    if let Some(b) = boites.get(brut) {
        verifie(attendu, &b.val).map_err(|e| format!("{} : {}", brut, e))?;
        return Ok(b.val.clone());
    }
    if matches!(attendu, TypeVal::Complexe) {
        let interieur = brut
            .strip_prefix('(')
            .and_then(|r| r.strip_suffix(')'))
            .ok_or_else(|| format!("{} : un complexe s'écrit (partie réelle ; partie imaginaire)", brut))?;
        let parts = coupe_niveau_zero(interieur, ';');
        if parts.len() != 2 {
            return Err(format!("{} : un complexe est un couple de deux réels", brut));
        }
        let mut valeurs = [0.0f64; 2];
        for (k, part) in parts.iter().enumerate() {
            let expr = resoudre_lectures(part.trim(), vars, boites, true);
            valeurs[k] = crate::maths::calcul::eval(&expr, vars)
                .ok_or_else(|| format!("{} n'est pas calculable", part.trim()))?;
        }
        return Ok(Valeur::Complexe(valeurs[0], valeurs[1]));
    }
    match attendu {
        TypeVal::Texte => Ok(Valeur::Texte(brut.to_string())),
        TypeVal::Booleen => match brut {
            "vrai" => Ok(Valeur::Nombre(1.0)),
            "faux" => Ok(Valeur::Nombre(0.0)),
            _ => Err(format!("{} n'est ni vrai ni faux", brut)),
        },
        _ => {
            let expr = super::fonctions::resoudre_appels(brut, vars, boites, fns, 0);
            if let Some(d) = expr.find('\u{27E6}') {
                let f = expr[d..].find('\u{27E7}').map(|k| d + k).unwrap_or(expr.len());
                return Err(expr[d + '\u{27E6}'.len_utf8()..f].to_string());
            }
            let expr = resoudre_lectures(&expr, vars, boites, true);
            let n = crate::maths::calcul::eval(&expr, vars)
                .ok_or_else(|| format!("{} n'est pas calculable", brut))?;
            let v = Valeur::Nombre(n);
            verifie(attendu, &v)?;
            Ok(v)
        }
    }
}

pub fn parse_litteral(
    brut: &str,
    attendu: &TypeVal,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fns: &super::fonctions::Fonctions,
) -> Result<Valeur, String> {
    let brut = brut.trim();
    let interieur = brut
        .strip_prefix('{')
        .and_then(|r| r.strip_suffix('}'))
        .ok_or_else(|| "un littéral s'écrit entre accolades".to_string())?;
    let interieur = interieur.trim();
    match attendu {
        TypeVal::Collection(elem) => {
            if interieur.is_empty() {
                return Ok(Valeur::Collection(Vec::new()));
            }
            let mut elems = Vec::new();
            for part in coupe_niveau_zero(interieur, ';') {
                elems.push(parse_element(part, elem, vars, boites, fns)?);
            }
            Ok(Valeur::Collection(elems))
        }
        TypeVal::Dictionnaire(elem) => {
            if interieur.is_empty() {
                return Ok(Valeur::Dictionnaire(Vec::new()));
            }
            let mut paires = Vec::new();
            for part in coupe_niveau_zero(interieur, ';') {
                let (cle, val) = part
                    .split_once(':')
                    .ok_or_else(|| format!("{} : une paire s'écrit clé: valeur", part.trim()))?;
                paires.push((
                    cle.trim().to_string(),
                    parse_element(val, elem, vars, boites, fns)?,
                ));
            }
            Ok(Valeur::Dictionnaire(paires))
        }
        TypeVal::Matrice(dims, elem) => {
            if matches!(**elem, TypeVal::Complexe) {
                return Err("une matrice de complexes n'est pas encore prise en charge".into());
            }
            if interieur.is_empty() {
                let (l, c) = dims.ok_or_else(|| {
                    "une matrice vide exige ses dimensions dans la déclaration".to_string()
                })?;
                return Ok(Valeur::Matrice(l, c, vec![0.0; l * c]));
            }
            let rangees_brutes: Vec<String> = if interieur.contains('\n') {
                interieur
                    .lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .map(|l| {
                        format!(
                            "{{{}}}",
                            l.split('\t')
                                .map(|c| c.trim())
                                .filter(|c| !c.is_empty())
                                .collect::<Vec<_>>()
                                .join(" ; ")
                        )
                    })
                    .collect()
            } else {
                coupe_niveau_zero(interieur, ';')
                    .into_iter()
                    .map(|s| s.trim().to_string())
                    .collect()
            };
            let mut cases: Vec<f64> = Vec::new();
            let mut colonnes: Option<usize> = None;
            for rb in &rangees_brutes {
                let rangee = parse_litteral(
                    rb,
                    &TypeVal::Collection(elem.clone()),
                    vars,
                    boites,
                    fns,
                )?;
                let Valeur::Collection(elems) = rangee else {
                    return Err("rangée de matrice invalide".into());
                };
                let largeur = elems.len();
                match colonnes {
                    None => colonnes = Some(largeur),
                    Some(c) if c != largeur => {
                        return Err(format!(
                            "les rangées n'ont pas toutes la même longueur ({} puis {})",
                            c, largeur
                        ));
                    }
                    _ => {}
                }
                for el in elems {
                    cases.push(nombre_de(&el).ok_or("une matrice ne contient que des nombres")?);
                }
            }
            let l = rangees_brutes.len();
            let c = colonnes.unwrap_or(0);
            let v = Valeur::Matrice(l, c, cases);
            verifie(attendu, &v)?;
            Ok(v)
        }
        _ => Err(format!(
            "un littéral entre accolades ne convient pas au type ({})",
            nom_type(attendu)
        )),
    }
}

pub fn formate_valeur(v: &Valeur, t: &TypeVal) -> String {
    formate(v, t)
}

pub fn decoupe_elements(s: &str) -> Vec<String> {
    coupe_niveau_zero(s, ';').into_iter().map(|m| m.trim().to_string()).collect()
}

pub fn crochet_fermant(s: &str) -> Option<usize> {
    trouve_crochets(s)
}

fn formate(v: &Valeur, t: &TypeVal) -> String {
    match (v, t) {
        (Valeur::Nombre(n), TypeVal::Booleen) => {
            if *n == 0.0 { "faux".into() } else { "vrai".into() }
        }
        (Valeur::Nombre(n), _) => crate::maths::calcul::format_number(*n),
        (Valeur::Complexe(a, b), _) => format!(
            "({} ; {})",
            crate::maths::calcul::format_number(*a),
            crate::maths::calcul::format_number(*b)
        ),
        (Valeur::Texte(s), _) => s.clone(),
        (Valeur::Collection(elems), TypeVal::Collection(e)) => format!(
            "{{{}}}",
            elems.iter().map(|el| formate(el, e)).collect::<Vec<_>>().join(" ; ")
        ),
        (Valeur::Dictionnaire(paires), TypeVal::Dictionnaire(e)) => format!(
            "{{{}}}",
            paires
                .iter()
                .map(|(k, el)| format!("{}: {}", k, formate(el, e)))
                .collect::<Vec<_>>()
                .join(" ; ")
        ),
        (Valeur::Matrice(l, c, cases), TypeVal::Matrice(_, e)) => {
            let mut rangees = Vec::new();
            for i in 0..*l {
                let mut cellules = Vec::new();
                for j in 0..*c {
                    cellules.push(formate(&Valeur::Nombre(cases[i * c + j]), e));
                }
                rangees.push(format!("{{{}}}", cellules.join(" ; ")));
            }
            format!("{{{}}}", rangees.join(" ; "))
        }
        _ => String::new(),
    }
}

pub fn affiche(b: &Boite) -> String {
    formate(&b.val, &b.type_val)
}

fn type_element(b: &Boite) -> TypeVal {
    match &b.type_val {
        TypeVal::Collection(e) | TypeVal::Dictionnaire(e) | TypeVal::Matrice(_, e) => (**e).clone(),
        autre => autre.clone(),
    }
}

pub fn elements_pour_boucle(b: &Boite) -> Vec<String> {
    let te = type_element(b);
    match &b.val {
        Valeur::Collection(elems) => elems.iter().map(|el| formate(el, &te)).collect(),
        Valeur::Dictionnaire(paires) => paires.iter().map(|(k, _)| k.clone()).collect(),
        Valeur::Matrice(l, c, cases) => (0..*l)
            .map(|i| {
                format!(
                    "{{{}}}",
                    (0..*c)
                        .map(|j| formate(&Valeur::Nombre(cases[i * c + j]), &te))
                        .collect::<Vec<_>>()
                        .join(" ; ")
                )
            })
            .collect(),
        _ => Vec::new(),
    }
}

pub fn lit_index(
    nom: &str,
    b: &Boite,
    indices_bruts: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
) -> Result<(Valeur, TypeVal), String> {
    let parts: Vec<&str> = coupe_niveau_zero(indices_bruts, ';')
        .into_iter()
        .map(|s| s.trim())
        .collect();
    let te = type_element(b);
    match &b.val {
        Valeur::Collection(elems) => {
            let expr = resoudre_lectures(parts[0], vars, boites, true);
            let i = crate::maths::calcul::eval(&expr, vars)
                .filter(|n| est_entier(*n) && *n >= 0.0)
                .ok_or_else(|| format!("{}[{}] — indice invalide", nom, indices_bruts.trim()))?
                as usize;
            let el = elems.get(i).ok_or_else(|| {
                format!(
                    "{}[{}] — indice hors bornes (la collection compte {} élément(s))",
                    nom, i, elems.len()
                )
            })?;
            Ok((el.clone(), te))
        }
        Valeur::Dictionnaire(paires) => {
            let cle = parts[0];
            let el = paires
                .iter()
                .find(|(k, _)| k == cle)
                .map(|(_, v)| v.clone())
                .ok_or_else(|| format!("{}[{}] — clé absente", nom, cle))?;
            Ok((el, te))
        }
        Valeur::Matrice(l, c, cases) => {
            if parts.len() != 2 {
                return Err(format!(
                    "{}[{}] — une matrice s'indexe par [ligne ; colonne]",
                    nom,
                    indices_bruts.trim()
                ));
            }
            let ei = resoudre_lectures(parts[0], vars, boites, true);
            let ej = resoudre_lectures(parts[1], vars, boites, true);
            let i = crate::maths::calcul::eval(&ei, vars)
                .filter(|n| est_entier(*n) && *n >= 0.0)
                .ok_or_else(|| format!("{} — indice de ligne invalide", nom))? as usize;
            let j = crate::maths::calcul::eval(&ej, vars)
                .filter(|n| est_entier(*n) && *n >= 0.0)
                .ok_or_else(|| format!("{} — indice de colonne invalide", nom))? as usize;
            if i >= *l || j >= *c {
                return Err(format!(
                    "{}[{} ; {}] — indice hors bornes (matrice {}×{})",
                    nom, i, j, l, c
                ));
            }
            Ok((Valeur::Nombre(cases[i * c + j]), te))
        }
        _ => Err(format!("{} ne s'indexe pas", nom)),
    }
}

fn trouve_crochets(s: &str) -> Option<usize> {
    let mut profondeur = 0i32;
    for (i, c) in s.char_indices() {
        match c {
            '[' => profondeur += 1,
            ']' => {
                profondeur -= 1;
                if profondeur == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

pub fn resoudre_lectures(
    texte: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    pour_calcul: bool,
) -> String {
    let mut out = String::with_capacity(texte.len());
    let mut reste = texte;
    while !reste.is_empty() {
        let mut trouve = None;
        for (nom, b) in boites {
            let mut depuis = 0usize;
            while let Some(p) = reste[depuis..].find(nom.as_str()) {
                let debut = depuis + p;
                let avant_ok = debut == 0
                    || !reste[..debut]
                        .chars()
                        .last()
                        .map(|c| c.is_alphanumeric() || c == '_' || c == '#')
                        .unwrap_or(false);
                let apres = &reste[debut + nom.len()..];
                if avant_ok && apres.starts_with('[') {
                    match trouve {
                        Some((d, _, _, _)) if d <= debut => {}
                        _ => trouve = Some((debut, nom.clone(), b.clone(), apres.to_string())),
                    }
                }
                depuis = debut + nom.len();
            }
        }
        let Some((debut, nom, b, apres)) = trouve else {
            out.push_str(reste);
            break;
        };
        let Some(fin_crochet) = trouve_crochets(&apres) else {
            out.push_str(&reste[..debut + nom.len() + 1]);
            reste = &reste[debut + nom.len() + 1..];
            continue;
        };
        let indices = &apres[1..fin_crochet];
        let suite = &apres[fin_crochet + 1..];
        let cible_ecriture = suite.trim_start().starts_with('=') && !suite.trim_start().starts_with("==");
        out.push_str(&reste[..debut]);
        if cible_ecriture {
            out.push_str(&reste[debut..debut + nom.len() + fin_crochet + 2]);
        } else {
            match lit_index(&nom, &b, indices, vars, boites) {
                Ok((v, te)) => {
                    let texte_v = formate(&v, &te);
                    if pour_calcul {
                        if let Valeur::Nombre(n) = v {
                            out.push_str(&format!("{}", n));
                        } else {
                            out.push_str(&texte_v);
                        }
                    } else {
                        out.push_str(&texte_v);
                    }
                }
                Err(e) => {
                    out.push_str(&format!("⟦{}⟧", e));
                }
            }
        }
        let consomme = debut + nom.len() + fin_crochet + 1;
        reste = &texte[texte.len() - reste.len() + consomme..];
    }
    out
}

pub fn ecrit_index(
    nom: &str,
    indices_bruts: &str,
    rhs: &str,
    vars: &BTreeMap<String, f64>,
    boites: &mut Boites,
    fns: &super::fonctions::Fonctions,
) -> Result<(), String> {
    let b = boites
        .get(nom)
        .ok_or_else(|| format!("{} n'a pas été déclaré", nom))?
        .clone();
    let te = type_element(&b);
    let nouvel = parse_element(rhs, &te, vars, boites, fns)?;
    let parts: Vec<String> = coupe_niveau_zero(indices_bruts, ';')
        .iter()
        .map(|s| s.trim().to_string())
        .collect();
    let mut boite = b;
    match &mut boite.val {
        Valeur::Collection(elems) => {
            let expr = resoudre_lectures(&parts[0], vars, boites, true);
            let i = crate::maths::calcul::eval(&expr, vars)
                .filter(|n| est_entier(*n) && *n >= 0.0)
                .ok_or_else(|| format!("{}[{}] — indice invalide", nom, indices_bruts.trim()))?
                as usize;
            if i >= elems.len() {
                return Err(format!(
                    "{}[{}] — indice hors bornes (la collection compte {} élément(s)) ; une collection grandit par +, jamais par indice",
                    nom,
                    i,
                    elems.len()
                ));
            }
            elems[i] = nouvel;
        }
        Valeur::Dictionnaire(paires) => {
            let cle = parts[0].clone();
            match paires.iter_mut().find(|(k, _)| *k == cle) {
                Some((_, v)) => *v = nouvel,
                None => paires.push((cle, nouvel)),
            }
        }
        Valeur::Matrice(l, c, cases) => {
            if parts.len() != 2 {
                return Err(format!(
                    "{} — une matrice s'écrit par [ligne ; colonne]",
                    nom
                ));
            }
            let ei = resoudre_lectures(&parts[0], vars, boites, true);
            let ej = resoudre_lectures(&parts[1], vars, boites, true);
            let i = crate::maths::calcul::eval(&ei, vars)
                .filter(|n| est_entier(*n) && *n >= 0.0)
                .ok_or_else(|| format!("{} — indice de ligne invalide", nom))? as usize;
            let j = crate::maths::calcul::eval(&ej, vars)
                .filter(|n| est_entier(*n) && *n >= 0.0)
                .ok_or_else(|| format!("{} — indice de colonne invalide", nom))? as usize;
            if i >= *l || j >= *c {
                return Err(format!(
                    "{}[{} ; {}] — indice hors bornes (matrice {}×{})",
                    nom, i, j, l, c
                ));
            }
            let n = nombre_de(&nouvel).ok_or("une matrice ne contient que des nombres")?;
            cases[i * *c + j] = n;
        }
        _ => return Err(format!("{} ne s'indexe pas", nom)),
    }
    verifie(&boite.type_val, &boite.val)?;
    boites.insert(nom.to_string(), boite);
    Ok(())
}

pub fn concatene(
    nom: &str,
    rhs: &str,
    vars: &BTreeMap<String, f64>,
    boites: &mut Boites,
    fns: &super::fonctions::Fonctions,
) -> Result<(), String> {
    let b = boites
        .get(nom)
        .ok_or_else(|| format!("{} n'a pas été déclaré", nom))?
        .clone();
    let mut val = b.val.clone();
    let termes = coupe_niveau_zero(rhs, '+');
    if termes.first().map(|t| t.trim()) != Some(nom) {
        return Err(format!(
            "une réaffectation de conteneur s'écrit {} = {} + …",
            nom, nom
        ));
    }
    for terme in termes.iter().skip(1) {
        let terme = terme.trim();
        let apport = if terme.starts_with('{') {
            parse_litteral(terme, &b.type_val, vars, boites, fns)?
        } else if let Some(autre) = boites.get(terme) {
            verifie(&b.type_val, &autre.val).map_err(|e| format!("{} : {}", terme, e))?;
            autre.val.clone()
        } else {
            return Err(format!("{} n'est ni un littéral ni un conteneur", terme));
        };
        val = match (val, apport) {
            (Valeur::Collection(mut a), Valeur::Collection(b2)) => {
                a.extend(b2);
                Valeur::Collection(a)
            }
            (Valeur::Dictionnaire(mut a), Valeur::Dictionnaire(b2)) => {
                for (k, v) in b2 {
                    match a.iter_mut().find(|(k2, _)| *k2 == k) {
                        Some((_, v2)) => *v2 = v,
                        None => a.push((k, v)),
                    }
                }
                Valeur::Dictionnaire(a)
            }
            _ => {
                return Err(format!(
                    "+ ne concatène que deux conteneurs de même sorte ({})",
                    nom
                ))
            }
        };
    }
    verifie(&b.type_val, &val)?;
    boites.insert(
        nom.to_string(),
        Boite {
            type_val: b.type_val,
            val,
        },
    );
    Ok(())
}

pub struct Instruction {
    pub consomme: usize,
    pub remplacement: String,
}

const BRACE_OUVRANTE: char = '\u{E002}';
const BRACE_FERMANTE: char = '\u{E003}';

fn desentinelle(s: &str) -> String {
    s.replace(BRACE_OUVRANTE, "{{").replace(BRACE_FERMANTE, "}}")
}

fn prend_groupe(s: &str) -> Option<(usize, usize)> {
    let debut = s.find(['{', BRACE_OUVRANTE])?;
    let mut profondeur = 0i32;
    for (i, c) in s[debut..].char_indices() {
        match c {
            '{' => profondeur += 1,
            '}' => profondeur -= 1,
            c2 if c2 == BRACE_OUVRANTE => profondeur += 2,
            c2 if c2 == BRACE_FERMANTE => profondeur -= 2,
            _ => {}
        }
        if profondeur == 0 && (c == '}' || c == BRACE_FERMANTE) {
            return Some((debut, debut + i + c.len_utf8()));
        }
    }
    None
}

fn sans_syntaxe(s: &str) -> String {
    s.replace(['{', '}'], "'").replace('<', "«").replace('>', "»")
}

fn erreur_div(ligne: &str, message: &str) -> String {
    format!(
        "<rouge gras>{{⚠ {} — {}}}\n",
        sans_syntaxe(ligne.trim()),
        sans_syntaxe(message)
    )
}

pub fn instruction_conteneur(
    texte: &str,
    vars: &BTreeMap<String, f64>,
    boites: &mut Boites,
    fns: &super::fonctions::Fonctions,
) -> Option<Instruction> {
    let sans_alinea = texte.trim_start_matches(['\t', ' ']);
    let decale = texte.len() - sans_alinea.len();
    let fin_ligne = sans_alinea.find('\n').unwrap_or(sans_alinea.len());
    let ligne = &sans_alinea[..fin_ligne];
    if let Some(reste) = ligne.strip_prefix("soit ") {
        if let Some((nom, apres_dp)) = reste.split_once(':') {
            let nom = nom.trim();
            if !nom.is_empty()
                && nom.chars().all(|c| c.is_alphanumeric() || c == '_')
                && !apres_dp.trim_start().starts_with('=')
            {
                let (avant_egal, _) = apres_dp.split_once('=')?;
                let type_val = parse_type(avant_egal)?;
                let (g_deb, g_fin) = prend_groupe(sans_alinea)?;
                let litteral = desentinelle(&sans_alinea[g_deb..g_fin]);
                let litteral = litteral.as_str();
                let consomme = decale + g_fin;
                let remplacement =
                    match parse_litteral(litteral, &type_val, vars, boites, fns) {
                        Ok(val) => {
                            boites.insert(nom.to_string(), Boite { type_val, val });
                            String::new()
                        }
                        Err(e) => erreur_div(&format!("soit {}: …", nom), &e),
                    };
                return Some(Instruction {
                    consomme,
                    remplacement,
                });
            }
        }
        if let Some((nom, rhs)) = reste.split_once('=') {
            let nom = nom.trim();
            if boites.contains_key(nom) && !nom.contains('[') {
                let rhs = desentinelle(rhs[..rhs.len().min(fin_ligne)].trim());
                let remplacement = match concatene(nom, &rhs, vars, boites, fns) {
                    Ok(()) => String::new(),
                    Err(e) => erreur_div(ligne, &e),
                };
                return Some(Instruction {
                    consomme: decale + fin_ligne,
                    remplacement,
                });
            }
        }
        return None;
    }
    let crochet = ligne.find('[')?;
    let nom = ligne[..crochet].trim();
    if nom.is_empty() || !boites.contains_key(nom) {
        return None;
    }
    let fin_crochet = trouve_crochets(&ligne[crochet..])? + crochet;
    let apres = ligne[fin_crochet + 1..].trim_start();
    let rhs = apres.strip_prefix('=')?;
    if rhs.starts_with('=') {
        return None;
    }
    let indices = &ligne[crochet + 1..fin_crochet];
    let remplacement = match ecrit_index(nom, indices, rhs.trim(), vars, boites, fns) {
        Ok(()) => String::new(),
        Err(e) => erreur_div(ligne, &e),
    };
    Some(Instruction {
        consomme: decale + fin_ligne,
        remplacement,
    })
}
