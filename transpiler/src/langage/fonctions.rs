use super::conteneurs::{
    formate_valeur, parse_litteral, parse_type, verifie, Boite, Boites, TypeVal, Valeur,
};
use std::collections::BTreeMap;

const PROFONDEUR_MAX: usize = 200;

#[derive(Clone, Debug, PartialEq)]
pub struct Fonction {
    pub params: Vec<(String, TypeVal)>,
    pub retour: TypeVal,
    pub corps: String,

    pub prive: bool,

    pub prives: Vec<String>,

    pub ancetres: Vec<String>,
}

pub type Fonctions = BTreeMap<String, Fonction>;

use crate::utils::decoupe::groupe_apparie;

fn coupe_args(s: &str) -> Vec<&str> {
    crate::utils::decoupe::coupe_arguments(s)
}

fn nom_valide(s: &str) -> bool {
    !s.is_empty()
        && s.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false)
        && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub const CONSTRUCTEUR: &str = "\u{E020}classe";

pub const TEXTE_DEBUT: char = '\u{E030}';
pub const TEXTE_FIN: char = '\u{E031}';

pub fn desencadre_texte(s: &str) -> Option<String> {
    if s.contains(TEXTE_DEBUT) {
        Some(s.replace(TEXTE_DEBUT, "").replace(TEXTE_FIN, ""))
    } else {
        None
    }
}

pub const CONSTRUCTEUR_ABSTRAIT: &str = "\u{E020}classe abstraite";

pub const ABSTRAITE: &str = "\u{E020}méthode abstraite";

pub fn methodes_sans_corps(entrees: &[(String, Fonction)]) -> Vec<String> {
    entrees
        .iter()
        .filter(|(_, f)| f.corps == ABSTRAITE)
        .map(|(cle, _)| cle.rsplit('.').next().unwrap_or("").to_string())
        .collect()
}

pub fn est_abstraite(entrees: &[(String, Fonction)]) -> bool {
    entrees
        .first()
        .map(|(_, f)| f.corps == CONSTRUCTEUR_ABSTRAIT)
        .unwrap_or(false)
}

pub fn fin_de_classe(texte: &str) -> Option<usize> {
    let sans_alinea = texte.trim_start_matches(['\t', ' ']);
    let decale = texte.len() - sans_alinea.len();
    let reste = sans_alinea.strip_prefix("soit une classe ")?;
    let ouvre = reste.find('{')?;
    let fin = groupe_apparie(&reste[ouvre..], '{', '}')? + ouvre;
    Some(decale + (sans_alinea.len() - reste.len()) + fin + 1)
}

pub fn parse_classe(
    texte: &str,
    connues: &Fonctions,
) -> Option<(Vec<(String, Fonction)>, usize)> {
    let sans_alinea = texte.trim_start_matches(['\t', ' ']);
    let decale = texte.len() - sans_alinea.len();
    let reste = sans_alinea.strip_prefix("soit une classe ")?;
    let ouvre = reste.find('{')?;
    let entete = reste[..ouvre].trim();
    let (abstraite, entete) = match entete.strip_prefix("abstraite ") {
        Some(suite) => (true, suite.trim()),
        None => (false, entete),
    };

    let mut nom = entete.to_string();
    let mut parent = None;

    for tournure in [" qui hérite de la classe ", " hérite de "] {
        if let Some((n, pere)) = entete.split_once(tournure) {
            nom = n.trim().to_string();
            parent = Some(pere.trim().to_string());
            break;
        }
    }
    if !nom_valide(&nom) || !nom.chars().next()?.is_uppercase() {
        return None;
    }

    let mut ancetres: Vec<String> = Vec::new();
    let mut herites: Vec<(String, TypeVal)> = Vec::new();
    let mut prives_herites: Vec<String> = Vec::new();
    let mut methodes_heritees: Vec<(String, Fonction)> = Vec::new();
    if let Some(pere) = &parent {
        let constructeur = connues.get(pere)?;
        ancetres.push(pere.clone());
        ancetres.extend(constructeur.ancetres.iter().cloned());
        herites = constructeur.params.clone();
        prives_herites = constructeur.prives.clone();
        let prefixe = format!("{}.", pere);
        for (cle, f) in connues.iter() {
            if let Some(m) = cle.strip_prefix(&prefixe) {
                methodes_heritees.push((m.to_string(), f.clone()));
            }
        }
    }
    let fin = groupe_apparie(&reste[ouvre..], '{', '}')? + ouvre;
    let corps = &reste[ouvre + 1..fin];
    let consomme = decale + (sans_alinea.len() - reste.len()) + fin + 1;

    let mut attributs: Vec<(String, TypeVal)> = herites.clone();
    let mut prives: Vec<String> = prives_herites;
    let mut sorties: Vec<(String, Fonction)> = Vec::new();
    let mut position = 0usize;
    while position < corps.len() {

        if corps[position..].starts_with('\n') {
            position += 1;
            continue;
        }
        let reste_corps = &corps[position..];
        if reste_corps.trim().is_empty() {
            break;
        }

        let nu = reste_corps.trim_start_matches(['\t', ' ']);
        let (cache, reste_corps) = match nu
            .strip_prefix("privé ")
            .or_else(|| nu.strip_prefix("privée "))
        {
            Some(suite) => (true, suite),
            None => (false, reste_corps),
        };
        if reste_corps.trim_start_matches(['\t', ' ']).starts_with("soit ") {

            if let Some((nom_methode, signature, consomme_m)) = parse_signature(reste_corps) {
                let mut f = signature;
                f.prive = cache;
                let mut params = attributs.clone();
                params.append(&mut f.params);
                f.params = params;
                sorties.push((format!("{}.{}", nom, nom_methode), f));
                position += consomme_m + (corps[position..].len() - reste_corps.len());
                continue;
            }
            let (nom_methode, mut f, consomme_m) = parse_declaration(reste_corps)?;
            f.prive = cache;

            let mut params = attributs.clone();
            params.append(&mut f.params);
            f.params = params;
            sorties.push((format!("{}.{}", nom, nom_methode), f));
            position += consomme_m + (corps[position..].len() - reste_corps.len());
            continue;
        }
        let fin_ligne = reste_corps.find('\n').unwrap_or(reste_corps.len());
        let ligne = reste_corps[..fin_ligne].trim();
        if !ligne.is_empty() {
            let (n, t) = ligne.split_once(':')?;
            let n = n.trim();
            if !nom_valide(n) {
                return None;
            }
            if cache {
                prives.push(n.to_string());
            }
            attributs.push((n.to_string(), parse_type(t)?));
        }
        position += fin_ligne + (corps[position..].len() - reste_corps.len()) + 1;
    }

    for (m, f) in methodes_heritees {
        let cle = format!("{}.{}", nom, m);
        if sorties.iter().any(|(c, _)| *c == cle) {
            continue;
        }
        let mut propres: Vec<(String, TypeVal)> = f.params[herites.len().min(f.params.len())..].to_vec();
        let mut params = attributs.clone();
        params.append(&mut propres);
        sorties.push((
            cle,
            Fonction {
                params,
                retour: f.retour.clone(),
                corps: f.corps.clone(),
                prive: f.prive,
                prives: Vec::new(),
                ancetres: Vec::new(),
            },
        ));
    }

    let noms_methodes: Vec<String> = sorties
        .iter()
        .map(|(cle, _)| cle.rsplit('.').next().unwrap_or("").to_string())
        .collect();
    let noms_attributs: Vec<String> = attributs.iter().map(|(n, _)| n.clone()).collect();
    for (_, f) in sorties.iter_mut() {
        f.corps = complete_appels_internes(&f.corps, &nom, &noms_methodes, &noms_attributs);
    }

    sorties.insert(
        0,
        (
            nom.clone(),
            Fonction {
                params: attributs,
                retour: TypeVal::Objet(nom),
                corps: if abstraite {
                    CONSTRUCTEUR_ABSTRAIT.to_string()
                } else {
                    CONSTRUCTEUR.to_string()
                },
                prive: false,
                prives,
                ancetres,
            },
        ),
    );
    Some((sorties, consomme))
}

fn complete_appels_internes(
    corps: &str,
    classe: &str,
    methodes: &[String],
    attributs: &[String],
) -> String {
    let mut sortie = String::with_capacity(corps.len());
    let mut reste = corps;
    'balayage: loop {
        let mut trouve: Option<(usize, &String)> = None;
        for m in methodes {
            let mut depuis = 0usize;
            while let Some(p) = reste[depuis..].find(m.as_str()) {
                let debut = depuis + p;
                let avant_ok = debut == 0
                    || !reste[..debut]
                        .chars()
                        .last()
                        .map(|c| c.is_alphanumeric() || c == '_' || c == '.')
                        .unwrap_or(false);
                if avant_ok && reste[debut + m.len()..].starts_with('(') {
                    match trouve {
                        Some((d, _)) if d <= debut => {}
                        _ => trouve = Some((debut, m)),
                    }
                }
                depuis = debut + m.len();
            }
        }
        let Some((debut, m)) = trouve else {
            sortie.push_str(reste);
            break 'balayage;
        };
        let apres = &reste[debut + m.len()..];
        let Some(fin) = groupe_apparie(apres, '(', ')') else {
            sortie.push_str(&reste[..debut + m.len()]);
            reste = apres;
            continue;
        };
        let propres = apres[1..fin].trim();
        let mut arguments = attributs.to_vec();
        if !propres.is_empty() {
            arguments.push(propres.to_string());
        }
        sortie.push_str(&reste[..debut]);
        sortie.push_str(&format!("{}.{}({})", classe, m, arguments.join(" ; ")));
        reste = &apres[fin + 1..];
    }
    sortie
}

fn parse_signature(texte: &str) -> Option<(String, Fonction, usize)> {
    let sans_alinea = texte.trim_start_matches(['\t', ' ']);
    let decale = texte.len() - sans_alinea.len();
    let fin_ligne = sans_alinea.find('\n').unwrap_or(sans_alinea.len());
    let ligne = &sans_alinea[..fin_ligne];
    if ligne.contains('=') {
        return None;
    }
    let reste = ligne.strip_prefix("soit ")?;
    let debut_par = reste.find('(')?;
    let nom = reste[..debut_par].trim().to_string();
    if !nom_valide(&nom) {
        return None;
    }
    let fin_par = groupe_apparie(&reste[debut_par..], '(', ')')? + debut_par;
    let retour = parse_type(reste[fin_par + 1..].trim().strip_prefix(':')?)?;
    let mut params = Vec::new();
    let bruts = &reste[debut_par + 1..fin_par];
    if !bruts.trim().is_empty() {
        for morceau in coupe_args(bruts) {
            let (n, ty) = morceau.split_once(':')?;
            if !nom_valide(n.trim()) {
                return None;
            }
            params.push((n.trim().to_string(), parse_type(ty)?));
        }
    }
    Some((
        nom,
        Fonction {
            params,
            retour,
            corps: ABSTRAITE.to_string(),
            prive: false,
            prives: Vec::new(),
            ancetres: Vec::new(),
        },
        decale + fin_ligne,
    ))
}

pub fn parse_declaration(texte: &str) -> Option<(String, Fonction, usize)> {
    let sans_alinea = texte.trim_start_matches(['\t', ' ']);
    let decale = texte.len() - sans_alinea.len();
    let reste = sans_alinea.strip_prefix("soit ")?;
    let debut_par = reste.find('(')?;
    let nom = reste[..debut_par].trim().to_string();
    if !nom_valide(&nom) {
        return None;
    }
    let fin_par = groupe_apparie(&reste[debut_par..], '(', ')')? + debut_par;
    let params_bruts = &reste[debut_par + 1..fin_par];
    let apres = reste[fin_par + 1..].trim_start();
    let apres = apres.strip_prefix(':')?;
    let (type_retour_brut, corps_brut) = apres.split_once('=')?;
    let retour = parse_type(type_retour_brut)?;

    let mut params = Vec::new();
    if !params_bruts.trim().is_empty() {
        for p in coupe_args(params_bruts) {
            let (n, t) = p.split_once(':')?;
            let n = n.trim();
            if !nom_valide(n) {
                return None;
            }
            params.push((n.to_string(), parse_type(t)?));
        }
    }

    let debut_corps = sans_alinea.len() - corps_brut.len();
    let corps_trim = corps_brut.trim_start();
    let saut = corps_brut.len() - corps_trim.len();
    let (corps, consomme) = if corps_trim.starts_with('{') {
        let fin = groupe_apparie(&sans_alinea[debut_corps + saut..], '{', '}')?;
        let brut = &sans_alinea[debut_corps + saut..debut_corps + saut + fin + 1];
        (brut.to_string(), decale + debut_corps + saut + fin + 1)
    } else {
        let fin_ligne = corps_trim.find('\n').unwrap_or(corps_trim.len());
        (
            corps_trim[..fin_ligne].to_string(),
            decale + debut_corps + saut + fin_ligne,
        )
    };

    Some((
        nom,
        Fonction {
            params,
            retour,
            corps,
            prive: false,
            prives: Vec::new(),
            ancetres: Vec::new(),
        },
        consomme,
    ))
}

fn verifie_valeur(t: &TypeVal, v: &Valeur, quoi: &str) -> Result<(), String> {
    verifie(t, v).map_err(|e| format!("{} : {}", quoi, e))
}

fn avec_article(t: &TypeVal) -> String {
    match t {
        TypeVal::Collection(_) => format!("une {}", super::conteneurs::nom_type_singulier(t)),
        TypeVal::Matrice(_, _) => format!("une {}", super::conteneurs::nom_type_singulier(t)),
        autre => format!("un {}", super::conteneurs::nom_type_singulier(autre)),
    }
}

fn est_compose(t: &TypeVal) -> bool {
    matches!(
        t,
        TypeVal::Texte
            | TypeVal::Collection(_)
            | TypeVal::Dictionnaire(_)
            | TypeVal::Matrice(_, _)
            | TypeVal::Uplet(_)
            | TypeVal::Objet(_)
            | TypeVal::Pile(_)
            | TypeVal::File(_)
    )
}

pub fn evalue_valeur(
    brut: &str,
    attendu: &TypeVal,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
) -> Result<Valeur, String> {
    evalue_vers(brut, attendu, vars, boites, fonctions, 0)
}

pub fn devine_valeur(
    brut: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
) -> Result<(Valeur, TypeVal), String> {
    valeur_de_fragment(brut, vars, boites, fonctions, 0)
}

fn evalue_vers(
    brut: &str,
    attendu: &TypeVal,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<Valeur, String> {
    let brut = brut.trim();
    if !est_compose(attendu) {
        let n = evalue_expression(brut, vars, boites, fonctions, profondeur)?;
        return Ok(Valeur::Nombre(n));
    }
    if brut.starts_with('{') {
        return parse_litteral(brut, attendu, vars, boites, fonctions);
    }
    if let Some(b) = boites.get(brut) {
        verifie(attendu, &b.val)?;
        return Ok(b.val.clone());
    }

    if let Some((nom, args, reste)) = tete_d_appel(brut, fonctions) {
        if reste.trim().is_empty() {
            let v = appelle(&nom, &args, vars, boites, fonctions, profondeur)?;
            verifie(attendu, &v)?;
            return Ok(v);
        }
    }

    let resolu = resoudre_appels(brut, vars, boites, fonctions, profondeur);
    if let Some(pos) = resolu.find('[') {
        let tete = resolu[..pos].trim();
        if let Some(b) = boites.get(tete) {
            if let Some(fin) = super::conteneurs::crochet_fermant(&resolu[pos..]) {
                if resolu[pos + fin + 1..].trim().is_empty() {
                    let (v, _) = super::conteneurs::lit_index(
                        tete,
                        b,
                        &resolu[pos + 1..pos + fin],
                        vars,
                        boites,
                    )?;
                    verifie(attendu, &v)?;
                    return Ok(v);
                }
            }
        }
    }

    if let Some(v) = evalue_concatenation(brut, attendu, vars, boites, fonctions, profondeur)? {
        return Ok(v);
    }

    if let Ok((v, _)) = valeur_de_fragment(brut, vars, boites, fonctions, profondeur) {
        match verifie(attendu, &v) {
            Ok(()) => return Ok(v),

            Err(e) if matches!(attendu, TypeVal::Objet(_)) => return Err(e),
            Err(_) => {}
        }
    }
    if matches!(attendu, TypeVal::Texte) {

        let contenu = brut
            .strip_prefix('"')
            .and_then(|r| r.strip_suffix('"'))
            .or_else(|| brut.strip_prefix('«').and_then(|r| r.strip_suffix('»')))
            .unwrap_or(brut);
        return Ok(Valeur::Texte(contenu.to_string()));
    }
    Err(format!(
        "{} ne se lit pas comme {}",
        brut,
        avec_article(attendu)
    ))
}

fn evalue_concatenation(
    brut: &str,
    attendu: &TypeVal,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<Option<Valeur>, String> {
    let morceaux = coupe_niveau_zero_plus(brut);
    if morceaux.len() < 2 {
        return Ok(None);
    }
    let mut total: Option<Valeur> = None;
    for m in morceaux {
        let v = evalue_vers(&m, attendu, vars, boites, fonctions, profondeur)?;
        total = Some(match total {
            None => v,
            Some(Valeur::Collection(mut a)) => match v {
                Valeur::Collection(b) => {
                    a.extend(b);
                    Valeur::Collection(a)
                }
                autre => {
                    a.push(autre);
                    Valeur::Collection(a)
                }
            },
            Some(Valeur::Texte(mut a)) => match v {
                Valeur::Texte(b) => {
                    a.push_str(&b);
                    Valeur::Texte(a)
                }
                _ => return Err("on ne joint qu'un texte à un texte".into()),
            },
            Some(autre) => return Err(format!("{:?} ne se concatène pas", autre)),
        });
    }
    Ok(total)
}

fn coupe_niveau_zero_plus(s: &str) -> Vec<String> {
    let mut morceaux = Vec::new();
    let mut profondeur = 0i32;
    let mut debut = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' | '{' | '[' => profondeur += 1,
            ')' | '}' | ']' => profondeur -= 1,
            '+' if profondeur == 0 => {
                morceaux.push(s[debut..i].trim().to_string());
                debut = i + 1;
            }
            _ => {}
        }
    }
    morceaux.push(s[debut..].trim().to_string());
    morceaux.into_iter().filter(|m| !m.is_empty()).collect()
}

fn tete_d_appel(s: &str, fonctions: &Fonctions) -> Option<(String, String, String)> {
    let s = s.trim();
    let coupure = s.find('(')?;
    let nom = s[..coupure].trim().to_string();
    if !fonctions.contains_key(&nom) {
        return None;
    }
    let fin = groupe_apparie(&s[coupure..], '(', ')')? + coupure;
    Some((nom, s[coupure + 1..fin].to_string(), s[fin + 1..].to_string()))
}

const PRIMITIVES: &[(&str, usize, usize)] = &[
    ("longueur", 1, 0),
    ("tri", 1, 0),
    ("inverse", 1, 0),
    ("somme", 1, 0),
    ("min", 1, 0),
    ("max", 1, 0),
    ("contient", 2, 1),
    ("indice de", 2, 1),
    ("insère", 3, 0),
    ("supprime", 2, 0),
    ("ajoute", 2, 0),
    ("quotient", 2, usize::MAX),
    ("reste", 2, usize::MAX),
    ("majuscule", 1, 0),
    ("minuscule", 1, 0),
    ("sans accents", 1, 0),

    ("élague", 1, 0),
    ("compacte", 1, 0),
    ("code", 1, 0),
    ("caractère", 1, usize::MAX),
    ("texte", 1, usize::MAX),
    ("nombre", 1, usize::MAX),
    ("aléatoire", 2, usize::MAX),

    ("empile", 2, 0),
    ("dépile", 1, 0),
    ("sommet", 1, 0),

    ("enfile", 2, 0),
    ("défile", 1, 0),
    ("tête", 1, 0),

    ("est vide", 1, 0),

    ("jonction", 2, 0),
    ("découpe", 2, 0),
];

fn tirage() -> u64 {
    use std::cell::Cell;
    thread_local! {
        static ETAT: Cell<u64> = const { Cell::new(0) };
    }
    ETAT.with(|etat| {
        let mut x = etat.get();
        if x == 0 {
            x = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x2545_F491_4F6C_DD1D)
                | 1;
        }
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        etat.set(x);
        x
    })
}

fn valeur_de_fragment(
    brut: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<(Valeur, TypeVal), String> {
    let brut = brut.trim();
    if let Some(b) = boites.get(brut) {
        return Ok((b.val.clone(), b.type_val.clone()));
    }

    let sans_point;
    let brut = if brut.contains('.') {
        sans_point = resoudre_points(brut, vars, boites, fonctions, profondeur);
        sans_point.as_str()
    } else {
        brut
    };

    if let Some(contenu) = brut
        .strip_prefix('"')
        .and_then(|r| r.strip_suffix('"'))
        .or_else(|| brut.strip_prefix('«').and_then(|r| r.strip_suffix('»')))
    {
        return Ok((Valeur::Texte(contenu.to_string()), TypeVal::Texte));
    }

    if brut.starts_with('(') && brut.ends_with(')') {
        let interieur = &brut[1..brut.len() - 1];
        let parts = coupe_args(interieur);
        if parts.len() >= 2 {
            let mut valeurs = Vec::with_capacity(parts.len());
            let mut types = Vec::with_capacity(parts.len());
            let mut lisible = true;
            for m in &parts {
                match valeur_de_fragment(m, vars, boites, fonctions, profondeur) {
                    Ok((v, te)) => {
                        valeurs.push(v);
                        types.push(te);
                    }
                    Err(_) => {
                        lisible = false;
                        break;
                    }
                }
            }
            if lisible {
                return Ok((Valeur::Uplet(valeurs), TypeVal::Uplet(types)));
            }
        }
    }
    if brut.starts_with('{') {
        for essai in [
            TypeVal::Collection(Box::new(TypeVal::Entier)),
            TypeVal::Collection(Box::new(TypeVal::Decimal)),
            TypeVal::Collection(Box::new(TypeVal::Reel)),
            TypeVal::Collection(Box::new(TypeVal::Texte)),
        ] {
            if let Ok(v) = parse_litteral(brut, &essai, vars, boites, fonctions) {
                return Ok((v, essai));
            }
        }
        return Err(format!("{} ne se lit pas comme un conteneur", brut));
    }

    if let Some(coupure) = brut.find('(') {
        let tete = brut[..coupure].trim();
        if let Some((nom, arite, _)) = PRIMITIVES
            .iter()
            .filter(|(n, _, _)| !fonctions.contains_key(*n))
            .find(|(n, _, _)| *n == tete)
        {
            if let Some(fin) = groupe_apparie(&brut[coupure..], '(', ')') {
                if brut[coupure + fin + 1..].trim().is_empty() {
                    let bruts = coupe_args(&brut[coupure + 1..coupure + fin]);
                    if bruts.len() == *arite {
                        let mut args = Vec::with_capacity(*arite);
                        for b in &bruts {
                            args.push(valeur_de_fragment(b, vars, boites, fonctions, profondeur)?);
                        }
                        return applique_primitive(nom, &args);
                    }
                }
            }
        }
    }

    if let Some(pos) = brut.find('[') {
        let tete = brut[..pos].trim();
        if let Some(b) = boites.get(tete) {
            if let Some(fin) = super::conteneurs::crochet_fermant(&brut[pos..]) {
                if brut[pos + fin + 1..].trim().is_empty() {
                    let indices =
                        resoudre_appels(&brut[pos + 1..pos + fin], vars, boites, fonctions, profondeur);
                    return super::conteneurs::lit_index(tete, b, &indices, vars, boites);
                }
            }
        }
    }
    if let Some((nom, args, reste)) = tete_d_appel(brut, fonctions) {
        if reste.trim().is_empty() {
            let retour = fonctions.get(&nom).map(|f| f.retour.clone()).unwrap_or(TypeVal::Reel);
            let v = appelle(&nom, &args, vars, boites, fonctions, profondeur)?;
            return Ok((v, retour));
        }
    }
    let n = evalue_expression(brut, vars, boites, fonctions, profondeur)?;
    Ok((Valeur::Nombre(n), TypeVal::Reel))
}

fn ordre(a: &Valeur, b: &Valeur) -> std::cmp::Ordering {
    match (a, b) {
        (Valeur::Nombre(x), Valeur::Nombre(y)) => {
            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Valeur::Texte(x), Valeur::Texte(y)) => {
            let cle = |s: &str| -> String {
                s.chars().map(super::conteneurs::depouille).flat_map(|c| c.to_lowercase()).collect()
            };
            cle(x).cmp(&cle(y)).then_with(|| x.cmp(y))
        }
        _ => std::cmp::Ordering::Equal,
    }
}

fn position(v: &Valeur) -> Result<usize, String> {
    match v {
        Valeur::Nombre(n) if *n >= 0.0 => Ok(n.round() as usize),
        _ => Err("une position doit être un entier positif".into()),
    }
}

fn applique_primitive(
    nom: &str,
    args: &[(Valeur, TypeVal)],
) -> Result<(Valeur, TypeVal), String> {
    let elements = |i: usize| -> Result<Vec<Valeur>, String> {
        match &args[i].0 {
            Valeur::Collection(e) => Ok(e.clone()),
            autre => Err(format!(
                "{} attend une liste ; il a reçu {}",
                nom,
                super::conteneurs::nom_type_singulier(&super::conteneurs::type_de_valeur(autre))
            )),
        }
    };
    let meme_type = |v: Valeur| -> (Valeur, TypeVal) { (v, args[0].1.clone()) };
    let entier = |n: f64| -> (Valeur, TypeVal) { (Valeur::Nombre(n), TypeVal::Entier) };

    Ok(match nom {
        "longueur" => entier(super::conteneurs::cardinal(&args[0].0) as f64),
        "tri" => {
            let mut e = elements(0)?;
            e.sort_by(ordre);
            meme_type(Valeur::Collection(e))
        }
        "inverse" => match &args[0].0 {
            Valeur::Collection(e) => {
                let mut e = e.clone();
                e.reverse();
                meme_type(Valeur::Collection(e))
            }
            Valeur::Texte(s) => (Valeur::Texte(s.chars().rev().collect()), TypeVal::Texte),
            _ => return Err("inverse attend une liste ou une chaîne de caractères".into()),
        },
        "somme" => {
            let mut total = 0.0;
            for x in elements(0)? {
                match x {
                    Valeur::Nombre(n) => total += n,
                    _ => return Err("somme ne s'applique qu'à des nombres".into()),
                }
            }
            (Valeur::Nombre(total), TypeVal::Reel)
        }
        "min" | "max" => {
            let e = elements(0)?;
            if e.is_empty() {
                return Err(format!("{} n'a pas de sens sur une liste vide", nom));
            }
            let mut retenu = e[0].clone();
            for x in e.into_iter().skip(1) {
                let plus_petit = ordre(&x, &retenu) == std::cmp::Ordering::Less;
                if (nom == "min") == plus_petit && ordre(&x, &retenu) != std::cmp::Ordering::Equal {
                    retenu = x;
                }
            }
            let te = match &args[0].1 {
                TypeVal::Collection(e) => (**e).clone(),
                autre => autre.clone(),
            };
            (retenu, te)
        }
        "contient" => {
            let present = match &args[1].0 {
                Valeur::Collection(e) => e.iter().any(|x| *x == args[0].0),
                Valeur::Dictionnaire(p) => match &args[0].0 {
                    Valeur::Texte(k) => p.iter().any(|(c, _)| c == k),
                    _ => false,
                },
                Valeur::Texte(s) => match &args[0].0 {
                    Valeur::Texte(m) => s.contains(m.as_str()),
                    _ => false,
                },
                _ => return Err("contient attend un conteneur en second argument".into()),
            };
            (
                Valeur::Nombre(if present { 1.0 } else { 0.0 }),
                TypeVal::Booleen,
            )
        }
        "indice de" => {
            let e = elements(1)?;
            match e.iter().position(|x| *x == args[0].0) {
                Some(i) => entier(i as f64),

                None => {
                    return Err(format!(
                        "{} ne figure pas dans la liste",
                        super::conteneurs::formate_valeur(&args[0].0, &args[0].1)
                    ))
                }
            }
        }

        "quotient" => {
            let (a, b) = match (&args[0].0, &args[1].0) {
                (Valeur::Nombre(x), Valeur::Nombre(y)) => (*x, *y),
                _ => return Err("quotient attend deux nombres".into()),
            };
            if b == 0.0 {
                return Err("la division par zéro n'a pas de quotient".into());
            }
            entier((a / b).floor())
        }
        "reste" => {
            let (a, b) = match (&args[0].0, &args[1].0) {
                (Valeur::Nombre(x), Valeur::Nombre(y)) => (*x, *y),
                _ => return Err("reste attend deux nombres".into()),
            };
            if b == 0.0 {
                return Err("la division par zéro n'a pas de reste".into());
            }
            entier(a - b * (a / b).floor())
        }

        "ajoute" => {
            let mut e = elements(0)?;
            e.push(args[1].0.clone());
            meme_type(Valeur::Collection(e))
        }
        "insère" => {
            let mut e = elements(0)?;
            let i = position(&args[1].0)?;
            if i > e.len() {
                return Err(format!(
                    "on ne peut pas insérer à la position {} : la liste compte {} élément(s)",
                    i,
                    e.len()
                ));
            }
            e.insert(i, args[2].0.clone());
            meme_type(Valeur::Collection(e))
        }
        "supprime" => {
            let mut e = elements(0)?;
            let i = position(&args[1].0)?;
            if i >= e.len() {
                return Err(format!(
                    "la position {} sort des bornes : la liste compte {} élément(s)",
                    i,
                    e.len()
                ));
            }
            e.remove(i);
            meme_type(Valeur::Collection(e))
        }
        "majuscule" | "minuscule" | "sans accents" | "élague" | "compacte" => {
            let s = match &args[0].0 {
                Valeur::Texte(s) => s.clone(),
                autre => super::conteneurs::formate_valeur(autre, &args[0].1),
            };
            let sortie = match nom {

                "majuscule" => s.to_uppercase(),
                "minuscule" => s.to_lowercase(),

                "élague" => s.trim().to_string(),
                "compacte" => s.chars().filter(|c| !c.is_whitespace()).collect(),
                _ => s.chars().map(super::conteneurs::depouille).collect(),
            };
            (Valeur::Texte(sortie), TypeVal::Texte)
        }
        "code" => {
            let s = match &args[0].0 {
                Valeur::Texte(s) => s.clone(),
                _ => return Err("code attend une lettre".into()),
            };
            match s.chars().next() {
                Some(c) if s.chars().count() == 1 => entier(c as u32 as f64),
                _ => return Err(format!("code attend une seule lettre, pas « {} »", s)),
            }
        }
        "caractère" => {
            let n = match &args[0].0 {
                Valeur::Nombre(n) => *n,
                _ => return Err("caractère attend un entier".into()),
            };
            match u32::try_from(n.round() as i64).ok().and_then(char::from_u32) {
                Some(c) => (Valeur::Texte(c.to_string()), TypeVal::Texte),
                None => return Err(format!("{} ne désigne aucune lettre", n)),
            }
        }
        "texte" => (
            Valeur::Texte(super::conteneurs::formate_valeur(&args[0].0, &args[0].1)),
            TypeVal::Texte,
        ),
        "nombre" => {
            let s = match &args[0].0 {
                Valeur::Texte(s) => s.clone(),
                autre => return Ok((autre.clone(), args[0].1.clone())),
            };
            match crate::maths::calcul::eval(&s.replace(',', "."), &BTreeMap::new()) {
                Some(n) => (Valeur::Nombre(n), TypeVal::Reel),
                None => return Err(format!("« {} » ne se lit pas comme un nombre", s)),
            }
        }

        "empile" | "enfile" => {
            let mut e = match (&args[0].0, nom) {
                (Valeur::Pile(e), "empile") => e.clone(),
                (Valeur::File(e), "enfile") => e.clone(),
                (autre, _) => {
                    return Err(format!(
                        "{} attend {} ; il a reçu {}",
                        nom,
                        if nom == "empile" { "une pile" } else { "une file" },
                        super::conteneurs::nom_type_singulier(
                            &super::conteneurs::type_de_valeur(autre)
                        )
                    ))
                }
            };
            e.push(args[1].0.clone());
            let v = if nom == "empile" { Valeur::Pile(e) } else { Valeur::File(e) };
            (v, args[0].1.clone())
        }
        "dépile" | "défile" => {
            let (mut e, est_pile) = match (&args[0].0, nom) {
                (Valeur::Pile(e), "dépile") => (e.clone(), true),
                (Valeur::File(e), "défile") => (e.clone(), false),
                _ => {
                    return Err(format!(
                        "{} attend {}",
                        nom,
                        if nom == "dépile" { "une pile" } else { "une file" }
                    ))
                }
            };
            if e.is_empty() {
                return Err(format!(
                    "on ne {} pas {} vide",
                    nom,
                    if est_pile { "une pile" } else { "une file" }
                ));
            }
            if est_pile {
                e.pop();
                (Valeur::Pile(e), args[0].1.clone())
            } else {
                e.remove(0);
                (Valeur::File(e), args[0].1.clone())
            }
        }
        "sommet" | "tête" => {
            let (e, element) = match (&args[0].0, &args[0].1) {
                (Valeur::Pile(e), TypeVal::Pile(t)) if nom == "sommet" => (e, (**t).clone()),
                (Valeur::File(e), TypeVal::File(t)) if nom == "tête" => (e, (**t).clone()),
                (Valeur::Pile(e), _) if nom == "sommet" => (e, TypeVal::Reel),
                (Valeur::File(e), _) if nom == "tête" => (e, TypeVal::Reel),
                _ => {
                    return Err(format!(
                        "{} attend {}",
                        nom,
                        if nom == "sommet" { "une pile" } else { "une file" }
                    ))
                }
            };
            match if nom == "sommet" { e.last() } else { e.first() } {
                Some(v) => (v.clone(), element),
                None => {
                    return Err(format!(
                        "{} vide n'a pas de {}",
                        if nom == "sommet" { "une pile" } else { "une file" },
                        nom
                    ))
                }
            }
        }

        "jonction" => {
            let separateur = match &args[1].0 {
                Valeur::Texte(s) => s.clone(),
                autre => super::conteneurs::formate_valeur(
                    autre,
                    &super::conteneurs::type_de_valeur(autre),
                ),
            };
            let element = |v: &Valeur| -> String {
                match v {
                    Valeur::Texte(s) => s.clone(),
                    autre => super::conteneurs::formate_valeur(
                        autre,
                        &super::conteneurs::type_de_valeur(autre),
                    ),
                }
            };
            let morceaux: Vec<String> = match &args[0].0 {
                Valeur::Collection(e) | Valeur::Pile(e) | Valeur::File(e) => {
                    e.iter().map(element).collect()
                }
                Valeur::Texte(s) => s.chars().map(|c| c.to_string()).collect(),
                autre => {
                    return Err(format!(
                        "jonction attend une liste ou une chaîne de caractères ; elle a reçu {}",
                        super::conteneurs::nom_type_singulier(
                            &super::conteneurs::type_de_valeur(autre)
                        )
                    ))
                }
            };
            (Valeur::Texte(morceaux.join(&separateur)), TypeVal::Texte)
        }

        "découpe" => {
            let (chaine, separateur) = match (&args[0].0, &args[1].0) {
                (Valeur::Texte(c), Valeur::Texte(s)) => (c.clone(), s.clone()),
                _ => return Err("découpe attend deux chaînes de caractères".into()),
            };
            let morceaux: Vec<Valeur> = if separateur.is_empty() {
                chaine.chars().map(|c| Valeur::Texte(c.to_string())).collect()
            } else {
                chaine
                    .split(separateur.as_str())
                    .map(|m| Valeur::Texte(m.to_string()))
                    .collect()
            };
            (
                Valeur::Collection(morceaux),
                TypeVal::Collection(Box::new(TypeVal::Texte)),
            )
        }
        "est vide" => (
            Valeur::Nombre(if super::conteneurs::cardinal(&args[0].0) == 0 { 1.0 } else { 0.0 }),
            TypeVal::Booleen,
        ),
        "aléatoire" => {
            let (a, b) = match (&args[0].0, &args[1].0) {
                (Valeur::Nombre(x), Valeur::Nombre(y)) => (x.round() as i64, y.round() as i64),
                _ => return Err("aléatoire attend deux entiers".into()),
            };
            if b < a {
                return Err(format!(
                    "aléatoire attend une borne inférieure au plus égale à l'autre : {} dépasse {}",
                    a, b
                ));
            }
            let etendue = (b - a + 1) as u64;
            entier((a + (tirage() % etendue) as i64) as f64)
        }
        autre => return Err(format!("{} n'est pas une primitive connue", autre)),
    })
}

fn division_en_toutes_lettres(texte: &str) -> String {
    let mut sortie = String::with_capacity(texte.len());
    let mut reste = texte;
    'balayage: loop {
        let mut trouve: Option<(usize, &str)> = None;
        for mot in ["quotient de ", "reste de "] {
            if let Some(p) = reste.find(mot) {
                match trouve {
                    Some((d, _)) if d <= p => {}
                    _ => trouve = Some((p, mot)),
                }
            }
        }
        let Some((debut, mot)) = trouve else {
            sortie.push_str(reste);
            break 'balayage;
        };
        let apres = &reste[debut + mot.len()..];

        let mut profondeur = 0i32;
        let mut coupure = None;
        let octets: Vec<(usize, char)> = apres.char_indices().collect();
        for (i, c) in &octets {
            match c {
                '(' | '{' | '[' => profondeur += 1,
                ')' | '}' | ']' => profondeur -= 1,
                _ => {}
            }
            if profondeur == 0 && apres[*i..].starts_with(" par ") {
                coupure = Some(*i);
                break;
            }
        }
        let Some(coupure) = coupure else {
            sortie.push_str(&reste[..debut + mot.len()]);
            reste = apres;
            continue;
        };
        let dividende = apres[..coupure].trim();
        let suite = &apres[coupure + " par ".len()..];

        const ARRETS: &[&str] = &[
            " vaut ", " moins de ", " plus de ", " au moins ", " au plus ",
            " différent ", " égal ", " et ", " ou ", " par ",
        ];
        let mut profondeur = 0i32;
        let mut fin = suite.len();
        for (i, c) in suite.char_indices() {
            match c {
                '(' | '{' | '[' => profondeur += 1,
                ')' | '}' | ']' | ';' | ',' if profondeur == 0 => {
                    fin = i;
                    break;
                }
                ')' | '}' | ']' => profondeur -= 1,
                _ => {}
            }
            if profondeur == 0 && ARRETS.iter().any(|a| suite[i..].starts_with(a)) {
                fin = i;
                break;
            }
        }
        let diviseur = suite[..fin].trim();
        let nom = mot.trim_end().trim_end_matches(" de");
        sortie.push_str(&reste[..debut]);
        sortie.push_str(&format!("{}({} ; {})", nom, dividende, diviseur));
        reste = &suite[fin..];
    }
    sortie
}

fn resoudre_points(
    texte: &str,
    _vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    _profondeur: usize,
) -> String {
    if !texte.contains('.') {
        return texte.to_string();
    }
    let mut out = String::with_capacity(texte.len());
    let mut reste = texte;
    loop {
        let Some(point) = reste.find('.') else {
            out.push_str(reste);
            return out;
        };
        let avant = &reste[..point];
        let depart = avant
            .rfind(|c: char| !(c.is_alphanumeric() || c == '_'))
            .map(|i| i + 1)
            .unwrap_or(0);
        let porteur = &avant[depart..];
        let apres = &reste[point + 1..];
        let fin_membre = apres
            .find(|c: char| !(c.is_alphanumeric() || c == '_'))
            .unwrap_or(apres.len());
        let membre = &apres[..fin_membre];

        let objet = boites.get(porteur).map(|b| b.val.clone());
        let Some(Valeur::Objet(classe, _, attributs)) = objet else {
            out.push_str(&reste[..point + 1]);
            reste = apres;
            continue;
        };
        out.push_str(&reste[..depart]);

        if apres[fin_membre..].starts_with('(') {
            let cle = format!("{}.{}", classe, membre);
            if fonctions.get(&cle).map(|f| f.prive).unwrap_or(false) {
                out.push_str(&format!(
                    "⟦{} est une méthode privée de {} : elle ne s'appelle que depuis la classe⟧",
                    membre, classe
                ));
                reste = &apres[fin_membre..];
                continue;
            }
            let suite = &apres[fin_membre..];
            match groupe_apparie(suite, '(', ')') {
                Some(fin) if fonctions.contains_key(&cle) => {

                    let mut arguments: Vec<String> = attributs
                        .iter()
                        .map(|(_, v)| super::conteneurs::forme_relisible(v))
                        .collect();
                    let propres = suite[1..fin].trim();
                    if !propres.is_empty() {
                        arguments.extend(coupe_args(propres).iter().map(|s| s.trim().to_string()));
                    }
                    out.push_str(&format!("{}({})", cle, arguments.join(" ; ")));
                    reste = &suite[fin + 1..];
                    continue;
                }
                _ => {
                    out.push_str(&format!("⟦{} n'est pas une méthode de {}⟧", membre, classe));
                    reste = &apres[fin_membre..];
                    continue;
                }
            }
        }

        if fonctions
            .get(&classe)
            .map(|f| f.prives.iter().any(|n| n == membre))
            .unwrap_or(false)
        {
            out.push_str(&format!(
                "⟦{} est un attribut privé de {} : il ne se lit que depuis la classe⟧",
                membre, classe
            ));
            reste = &apres[fin_membre..];
            continue;
        }
        match attributs.iter().find(|(n, _)| n == membre) {

            Some((_, Valeur::Texte(s))) => {
                out.push(TEXTE_DEBUT);
                out.push_str(s);
                out.push(TEXTE_FIN);
            }
            Some((_, v)) => out.push_str(&super::conteneurs::forme_relisible(v)),
            None => out.push_str(&format!("⟦{} n'a pas d'attribut {}⟧", classe, membre)),
        }
        reste = &apres[fin_membre..];
    }
}

fn resoudre_primitives(
    texte: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> String {
    let avec_points = resoudre_points(texte, vars, boites, fonctions, profondeur);
    let en_lettres = division_en_toutes_lettres(&avec_points);
    let texte: &str = &en_lettres;
    let mut out = String::with_capacity(texte.len());
    let mut reste = texte;
    'balayage: loop {

        let mut trouve: Option<(usize, &str, usize)> = None;
        for (nom, arite, _) in PRIMITIVES {

            if fonctions.contains_key(*nom) {
                continue;
            }
            let mut depuis = 0usize;
            while let Some(p) = reste[depuis..].find(nom) {
                let debut = depuis + p;
                let avant_ok = debut == 0
                    || !reste[..debut]
                        .chars()
                        .last()
                        .map(|c| c.is_alphanumeric() || c == '_')
                        .unwrap_or(false);
                if avant_ok && reste[debut + nom.len()..].starts_with('(') {
                    match trouve {
                        Some((d, _, _)) if d <= debut => {}
                        _ => trouve = Some((debut, nom, *arite)),
                    }
                }
                depuis = debut + nom.len();
            }
        }
        let Some((debut, nom, arite)) = trouve else {
            out.push_str(reste);
            break 'balayage;
        };
        let apres = &reste[debut + nom.len()..];
        let Some(fin) = groupe_apparie(apres, '(', ')') else {
            out.push_str(&reste[..debut + nom.len()]);
            reste = apres;
            continue;
        };
        let bruts = coupe_args(&apres[1..fin]);

        let bruts: Vec<&str> = if bruts.len() == 1 && bruts[0].trim().is_empty() {
            Vec::new()
        } else {
            bruts
        };
        let place = PRIMITIVES
            .iter()
            .find(|(n, _, _)| *n == nom)
            .map(|(_, _, p)| *p)
            .unwrap_or(usize::MAX);
        let avant = &reste[..debut];
        let rogne = avant.trim_end();
        let mut recepteur: Option<(String, usize)> = None;
        if place != usize::MAX && rogne.len() < avant.len() && bruts.len() + 1 == arite {
            let depart = rogne
                .rfind(|c: char| !(c.is_alphanumeric() || c == '_'))
                .map(|i| i + 1)
                .unwrap_or(0);
            let identifiant = &rogne[depart..];
            if !identifiant.is_empty() && boites.contains_key(identifiant) {

                let avant_nom = rogne[..depart].trim_end();
                let coupe = if avant_nom.ends_with("dans") {
                    avant_nom.len() - "dans".len()
                } else {
                    depart
                };
                recepteur = Some((identifiant.to_string(), coupe));
            }
        }

        if recepteur.is_none() && bruts.len() != arite {
            out.push_str(&reste[..debut + nom.len()]);
            reste = apres;
            continue;
        }
        match &recepteur {
            Some((_, depart)) => out.push_str(&avant[..*depart]),
            None => out.push_str(avant),
        }
        let mut args = Vec::with_capacity(arite);
        let mut faute = None;
        let mut fragments: Vec<String> = bruts.iter().map(|s| s.to_string()).collect();
        if let Some((identifiant, _)) = &recepteur {
            fragments.insert(place.min(fragments.len()), identifiant.clone());
        }
        for b in &fragments {
            match valeur_de_fragment(b, vars, boites, fonctions, profondeur) {
                Ok(v) => args.push(v),
                Err(e) => {
                    faute = Some(e);
                    break;
                }
            }
        }
        match faute {
            Some(e) => out.push_str(&format!("⟦{}⟧", e)),
            None => match applique_primitive(nom, &args) {

                Ok((Valeur::Nombre(n), TypeVal::Booleen)) => {
                    out.push_str(if n == 0.0 { "faux" } else { "vrai" })
                }
                Ok((Valeur::Nombre(n), _)) => out.push_str(&format!("({})", n)),
                Ok((Valeur::Texte(s), _)) => {
                    out.push(TEXTE_DEBUT);
                    out.push_str(&s);
                    out.push(TEXTE_FIN);
                }
                Ok((v, t)) => out.push_str(&super::conteneurs::formate_valeur(&v, &t)),
                Err(e) => out.push_str(&format!("⟦{}⟧", e)),
            },
        }
        reste = &apres[fin + 1..];
    }
    out
}

pub fn resoudre_appels(
    texte: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> String {

    if !texte.contains('(') && !texte.contains(" par ") && !texte.contains('.') {
        return texte.to_string();
    }
    let avec_primitives = resoudre_primitives(texte, vars, boites, fonctions, profondeur);
    if fonctions.is_empty() {
        return avec_primitives;
    }
    let texte: &str = &avec_primitives;
    let mut out = String::with_capacity(texte.len());
    let mut reste = texte;
    'balayage: while !reste.is_empty() {
        let mut meilleur: Option<(usize, String)> = None;
        for nom in fonctions.keys() {
            let mut depuis = 0usize;
            while let Some(p) = reste[depuis..].find(nom.as_str()) {
                let debut = depuis + p;
                let avant_ok = debut == 0
                    || !reste[..debut]
                        .chars()
                        .last()
                        .map(|c| c.is_alphanumeric() || c == '_')
                        .unwrap_or(false);
                if avant_ok && reste[debut + nom.len()..].starts_with('(') {
                    match &meilleur {
                        Some((d, _)) if *d <= debut => {}
                        _ => meilleur = Some((debut, nom.clone())),
                    }
                }
                depuis = debut + nom.len();
            }
        }
        let Some((debut, nom)) = meilleur else {
            out.push_str(reste);
            break 'balayage;
        };
        let apres = &reste[debut + nom.len()..];
        let Some(fin) = groupe_apparie(apres, '(', ')') else {
            out.push_str(&reste[..debut + nom.len()]);
            reste = &reste[debut + nom.len()..];
            continue;
        };
        out.push_str(&reste[..debut]);
        let args = &apres[1..fin];
        match appelle(&nom, args, vars, boites, fonctions, profondeur) {
            Ok(Valeur::Nombre(n)) => out.push_str(&format!("({})", n)),

            Ok(autre) => {
                let t = fonctions.get(&nom).map(|f| f.retour.clone()).unwrap_or(TypeVal::Reel);
                out.push_str(&formate_valeur(&autre, &t));
            }
            Err(e) => out.push_str(&format!("⟦{}⟧", e)),
        }
        reste = &apres[fin + 1..];
    }
    out
}

fn evalue_expression(
    expr: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<f64, String> {
    let avec_appels = resoudre_appels(expr, vars, boites, fonctions, profondeur);
    if let Some(d) = avec_appels.find('⟦') {
        let fin = avec_appels[d..].find('⟧').map(|f| d + f).unwrap_or(avec_appels.len());
        return Err(avec_appels[d + '⟦'.len_utf8()..fin].to_string());
    }
    let resolu = super::conteneurs::resoudre_lectures(&avec_appels, vars, boites, true);
    crate::maths::calcul::eval(&resolu, vars)
        .ok_or_else(|| format!("{} n'est pas calculable", expr.trim()))
}

fn evalue_corps(
    corps: &str,
    locales: &mut BTreeMap<String, f64>,
    boites: &mut Boites,
    attendu: &TypeVal,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<Valeur, String> {
    let t = corps.trim();
    if let Some(interieur) = t.strip_prefix('{').and_then(|r| r.strip_suffix('}')) {

        let mot_du_langage = ["soit ", "retourne ", "renvoie ", "pour ", "si ", "tant que", "faire", "sortir"]
            .iter()
            .any(|m| {
                interieur
                    .lines()
                    .any(|l| l.trim().starts_with(m) || l.trim() == m.trim())
            });
        if !mot_du_langage
            && matches!(
                attendu,
                TypeVal::Collection(_) | TypeVal::Dictionnaire(_) | TypeVal::Matrice(_, _)
            )
        {
            return parse_litteral(t, attendu, locales, boites, fonctions);
        }
        let instructions = decoupe_instructions(interieur);
        if instructions.len() == 1
            && !instructions[0].starts_with("retourne ")
            && !instructions[0].starts_with("soit ")
            && !instructions[0].starts_with("pour ")
            && !instructions[0].starts_with("tant que")
        {
            return evalue_corps(&instructions[0], locales, boites, attendu, fonctions, profondeur);
        }
        return match execute_bloc(&instructions, locales, boites, attendu, fonctions, profondeur)? {
            Flux::Retour(v) => Ok(v),
            _ => Err("la fonction ne retourne rien : il manque « retourne »".into()),
        };
    }
    if let Some(apres_si) = t.strip_prefix("si ") {
        if let Some(ouvre) = apres_si.find('{') {
            let cond = apres_si[..ouvre].trim();
            if let Some(fin) = groupe_apparie(&apres_si[ouvre..], '{', '}') {
                let alors = &apres_si[ouvre..ouvre + fin + 1];
                let suite = apres_si[ouvre + fin + 1..].trim_start();
                let sinon = suite.strip_prefix("sinon").map(|r| r.trim_start());
                let cond_resolue = resoudre_appels(cond, locales, boites, fonctions, profondeur);
                let cond_resolue =
                    super::conteneurs::resoudre_lectures(&cond_resolue, locales, boites, true);
                let vrai = crate::layout::controle::evalue_condition_publique(&cond_resolue, locales);
                if vrai {
                    return evalue_corps(alors, locales, boites, attendu, fonctions, profondeur);
                }
                let Some(sinon) = sinon else {
                    return Err("un « si » sans « sinon » ne retourne rien quand la condition est fausse".into());
                };
                return evalue_corps(sinon, locales, boites, attendu, fonctions, profondeur);
            }
        }
    }
    evalue_vers(t, attendu, locales, boites, fonctions, profondeur)
}

fn pose_locale(
    nom: &str,
    expr: &str,
    locales: &mut BTreeMap<String, f64>,
    boites: &mut Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<(), String> {
    let e = expr.trim();

    if let Some((n, t)) = nom.split_once(':') {
        let n = n.trim();
        let declare = parse_type(t)
            .ok_or_else(|| format!("{} n'est pas un type connu", t.trim()))?;
        let v = evalue_vers(e, &declare, locales, boites, fonctions, profondeur)?;
        verifie(&declare, &v)?;
        boites.insert(n.to_string(), Boite { type_val: declare, val: v });
        return Ok(());
    }

    if let Some(source) = boites.get(e).cloned() {
        boites.insert(nom.to_string(), source);
        return Ok(());
    }

    if let Some(b) = boites.get(nom).cloned() {
        let v = evalue_vers(e, &b.type_val, locales, boites, fonctions, profondeur)?;
        boites.insert(nom.to_string(), Boite { type_val: b.type_val, val: v });
        return Ok(());
    }

    match valeur_de_fragment(e, locales, boites, fonctions, profondeur) {
        Ok((Valeur::Nombre(n), _)) => {
            locales.insert(nom.to_string(), n);
            Ok(())
        }
        Ok((v, type_val)) => {
            boites.insert(nom.to_string(), Boite { type_val, val: v });
            Ok(())
        }
        Err(_) => {
            let n = evalue_expression(e, locales, boites, fonctions, profondeur)?;
            locales.insert(nom.to_string(), n);
            Ok(())
        }
    }
}

fn ecriture_indexee(ligne: &str) -> Option<(&str, &str)> {
    let pos = ligne.find('=')?;
    if ligne[pos..].starts_with("==") {
        return None;
    }
    let (gauche, droite) = ligne.split_at(pos);
    let gauche = gauche.trim();
    if !gauche.ends_with(']') || !gauche.contains('[') {
        return None;
    }
    Some((gauche, droite[1..].trim()))
}

pub fn appelle(
    nom: &str,
    args_bruts: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<Valeur, String> {
    if profondeur >= PROFONDEUR_MAX {
        return Err(format!(
            "{} — la récursion dépasse {} appels imbriqués",
            nom, PROFONDEUR_MAX
        ));
    }
    let f = fonctions
        .get(nom)
        .ok_or_else(|| format!("{} n'est pas une fonction déclarée", nom))?;
    let args: Vec<&str> = if args_bruts.trim().is_empty() {
        Vec::new()
    } else {
        coupe_args(args_bruts)
    };
    if args.len() != f.params.len() {
        return Err(format!(
            "{} attend {} argument(s), en reçoit {}",
            nom,
            f.params.len(),
            args.len()
        ));
    }
    let mut locales = BTreeMap::new();

    let mut locales_boites: Boites = boites.clone();
    for ((p, t), a) in f.params.iter().zip(args.iter()) {
        let v = evalue_vers(a, t, vars, boites, fonctions, profondeur + 1)?;
        verifie_valeur(t, &v, &format!("{}, argument {}", nom, p))?;
        match v {
            Valeur::Nombre(n) if !est_compose(t) => {
                locales.insert(p.clone(), n);
            }
            autre => {
                locales_boites.insert(
                    p.clone(),
                    Boite { type_val: t.clone(), val: autre },
                );
            }
        }
    }

    if f.corps == CONSTRUCTEUR_ABSTRAIT {
        return Err(format!(
            "{} est une classe abstraite : elle ne s'instancie pas, seules ses classes filles le font",
            nom
        ));
    }
    if f.corps == ABSTRAITE {
        return Err(format!(
            "{} n'a pas de corps : c'est à une classe fille de la définir",
            nom
        ));
    }
    if f.corps == CONSTRUCTEUR {
        let mut attributs = Vec::with_capacity(f.params.len());
        for (nom_attribut, _) in f.params.iter() {
            let v = locales_boites
                .get(nom_attribut)
                .map(|b| b.val.clone())
                .or_else(|| locales.get(nom_attribut).map(|n| Valeur::Nombre(*n)))
                .unwrap_or(Valeur::Nombre(0.0));
            attributs.push((nom_attribut.clone(), v));
        }
        return Ok(Valeur::Objet(
            nom.to_string(),
            f.ancetres.clone(),
            attributs,
        ));
    }

    let valeur = evalue_corps(
        &f.corps,
        &mut locales,
        &mut locales_boites,
        &f.retour,
        fonctions,
        profondeur + 1,
    )?;
    verifie_valeur(&f.retour, &valeur, &format!("{}, valeur retournée", nom))?;
    Ok(valeur)
}

enum Flux {
    Continue,
    Retour(Valeur),

    Sortir,

    Continuer,
}

fn decoupe_instructions(corps: &str) -> Vec<String> {
    let mut sortie = Vec::new();
    let mut courante = String::new();
    let mut profondeur = 0i32;
    for ligne in corps.lines() {
        let nue = ligne.trim();
        if nue.is_empty() && profondeur == 0 {
            continue;
        }
        if !courante.is_empty() {
            courante.push('\n');
        }
        courante.push_str(nue);
        profondeur += nue.matches('{').count() as i32 - nue.matches('}').count() as i32;
        if profondeur <= 0 {
            profondeur = 0;

            if !nue.ends_with("sinon") && !nue.ends_with("sinon {") {
                sortie.push(courante.trim().to_string());
                courante.clear();
            }
        }
    }
    if !courante.trim().is_empty() {
        sortie.push(courante.trim().to_string());
    }
    sortie
}

fn tete_et_bloc(s: &str) -> Option<(String, String, String)> {
    let ouvre = s.find('{')?;
    let fin = groupe_apparie(&s[ouvre..], '{', '}')? + ouvre;
    Some((
        s[..ouvre].trim().to_string(),
        s[ouvre + 1..fin].to_string(),
        s[fin + 1..].trim().to_string(),
    ))
}

fn execute_bloc(
    instructions: &[String],
    locales: &mut BTreeMap<String, f64>,
    boites: &mut Boites,
    attendu: &TypeVal,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<Flux, String> {
    for i in instructions {
        match execute(i, locales, boites, attendu, fonctions, profondeur)? {
            Flux::Continue => {}
            retour => return Ok(retour),
        }
    }
    Ok(Flux::Continue)
}

fn execute_texte(
    corps: &str,
    locales: &mut BTreeMap<String, f64>,
    boites: &mut Boites,
    attendu: &TypeVal,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<Flux, String> {
    let instructions = decoupe_instructions(corps);

    if instructions.len() == 1 {
        let seule = instructions[0].trim();
        if !seule.starts_with("retourne ")
            && !seule.starts_with("soit ")
            && !seule.starts_with("pour ")
            && !seule.starts_with("tant que")
            && !seule.starts_with("si ")

            && seule != "sortir"
            && seule != "continuer"
            && ecriture_indexee(seule).is_none()
        {
            let v = evalue_vers(seule, attendu, locales, boites, fonctions, profondeur)?;
            return Ok(Flux::Retour(v));
        }
    }
    execute_bloc(&instructions, locales, boites, attendu, fonctions, profondeur)
}

fn execute(
    instruction: &str,
    locales: &mut BTreeMap<String, f64>,
    boites: &mut Boites,
    attendu: &TypeVal,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<Flux, String> {
    let l = instruction.trim();
    if l.is_empty() {
        return Ok(Flux::Continue);
    }

    if let Some(expr) = l.strip_prefix("retourne ") {
        let v = evalue_vers(expr, attendu, locales, boites, fonctions, profondeur)?;
        return Ok(Flux::Retour(v));
    }

    if l == "sortir" {
        return Ok(Flux::Sortir);
    }

    if l == "continuer" {
        return Ok(Flux::Continuer);
    }

    if l.starts_with("si ") {
        let (tete, alors, suite) = tete_et_bloc(l)
            .ok_or_else(|| format!("{} — le « si » n'est pas refermé", l))?;
        let cond = tete.strip_prefix("si ").unwrap_or(&tete).trim();
        if condition_vraie(cond, locales, boites, fonctions, profondeur) {
            return execute_texte(&alors, locales, boites, attendu, fonctions, profondeur);
        }
        let suite = suite.trim();
        if let Some(reste) = suite.strip_prefix("sinon") {
            let reste = reste.trim();
            if reste.starts_with("si ") {
                return execute(reste, locales, boites, attendu, fonctions, profondeur);
            }
            let (_, sinon, _) = tete_et_bloc(reste)
                .ok_or_else(|| format!("{} — le « sinon » n'est pas refermé", l))?;
            return execute_texte(&sinon, locales, boites, attendu, fonctions, profondeur);
        }
        return Ok(Flux::Continue);
    }

    if let Some(reste) = l.strip_prefix("pour ") {
        let (tete, corps, _) = tete_et_bloc(reste)
            .ok_or_else(|| format!("{} — la boucle n'est pas refermée", l))?;

        if let Some((nom, source)) = tete.split_once(" dans ") {
            let nom = nom.trim();
            let source = source.trim();

            let b = match boites.get(source).cloned() {
                Some(b) => b,
                None => {
                    let (val, type_val) =
                        valeur_de_fragment(source, locales, boites, fonctions, profondeur)
                            .map_err(|_| format!("{} n'est pas un conteneur", source))?;
                    Boite { type_val, val }
                }
            };

            if let Valeur::Pile(_) | Valeur::File(_) = b.val {
                return Err(format!(
                    "on ne parcourt pas {} : on la vide, en la dépilant ou en la défilant",
                    if matches!(b.val, Valeur::Pile(_)) { "une pile" } else { "une file" }
                ));
            }

            if let Some(valeurs) = super::conteneurs::valeurs_pour_boucle(&b) {
                for v in valeurs {
                    let type_val = super::conteneurs::type_de_valeur(&v);
                    boites.insert(nom.to_string(), Boite { type_val, val: v });
                    match execute_texte(&corps, locales, boites, attendu, fonctions, profondeur)? {
                        Flux::Continue | Flux::Continuer => {}
                        Flux::Sortir => break,
                        retour => return Ok(retour),
                    }
                }
                return Ok(Flux::Continue);
            }
            let te = super::conteneurs::type_element(&b);
            for element in super::conteneurs::elements_pour_boucle(&b) {

                if matches!(te, TypeVal::Texte) || matches!(b.val, Valeur::Dictionnaire(_)) {
                    boites.insert(
                        nom.to_string(),
                        Boite { type_val: TypeVal::Texte, val: Valeur::Texte(element) },
                    );
                    match execute_texte(&corps, locales, boites, attendu, fonctions, profondeur)? {
                        Flux::Continue | Flux::Continuer => {}
                        Flux::Sortir => break,
                        retour => return Ok(retour),
                    }
                    continue;
                }
                pose_locale(nom, &element, locales, boites, fonctions, profondeur)?;
                match execute_texte(&corps, locales, boites, attendu, fonctions, profondeur)? {
                    Flux::Continue | Flux::Continuer => {}
                    Flux::Sortir => break,
                    retour => return Ok(retour),
                }
            }
            return Ok(Flux::Continue);
        }

        let (nom, bornes) = tete
            .split_once(" de ")
            .ok_or_else(|| format!("{} — une boucle s'écrit « pour n de a à b »", l))?;
        let (debut, fin) = bornes
            .split_once(" à ")
            .ok_or_else(|| format!("{} — il manque « à » dans la boucle", l))?;
        let nom = nom.trim();
        let depart = evalue_expression(debut, locales, boites, fonctions, profondeur)?;
        let (fin_txt, pas) = match fin.split_once("avec un pas de") {
            Some((f, p)) => (
                f.to_string(),
                evalue_expression(p, locales, boites, fonctions, profondeur)?,
            ),
            None => (fin.to_string(), 1.0),
        };
        let arrivee = evalue_expression(&fin_txt, locales, boites, fonctions, profondeur)?;
        if pas == 0.0 {
            return Err("un pas de 0 ne fait jamais avancer la boucle".into());
        }
        let mut n = depart;
        let mut tours = 0usize;
        while (pas > 0.0 && n <= arrivee + 1e-9) || (pas < 0.0 && n >= arrivee - 1e-9) {
            tours += 1;
            if tours > 1_000_000 {
                return Err("cette boucle dépasse un million de tours".into());
            }
            locales.insert(nom.to_string(), n);
            match execute_texte(&corps, locales, boites, attendu, fonctions, profondeur)? {
                Flux::Continue | Flux::Continuer => {}
                Flux::Sortir => break,
                retour => return Ok(retour),
            }
            n += pas;
        }
        return Ok(Flux::Continue);
    }

    if let Some(reste) = l.strip_prefix("tant que") {
        let (tete, corps, _) = tete_et_bloc(reste)
            .ok_or_else(|| format!("{} — la boucle n'est pas refermée", l))?;
        let cond = tete.trim().trim_end_matches("faire").trim().to_string();
        let mut tours = 0usize;
        while condition_vraie(&cond, locales, boites, fonctions, profondeur) {
            tours += 1;
            if tours > 1_000_000 {
                return Err("cette boucle dépasse un million de tours : sa condition ne devient jamais fausse".into());
            }
            match execute_texte(&corps, locales, boites, attendu, fonctions, profondeur)? {
                Flux::Continue | Flux::Continuer => {}
                Flux::Sortir => break,
                retour => return Ok(retour),
            }
        }
        return Ok(Flux::Continue);
    }

    let prefixe;
    let l: &str = if !l.starts_with("soit ") {
        match l.split_once('=') {
            Some((gauche, droite))
                if !droite.starts_with('=')
                    && {
                        let nom = gauche.trim();
                        !nom.is_empty()
                            && nom.chars().all(|c| c.is_alphanumeric() || c == '_')
                            && (locales.contains_key(nom) || boites.contains_key(nom))
                    } =>
            {
                prefixe = format!("soit {}", l);
                prefixe.as_str()
            }
            _ => l,
        }
    } else {
        l
    };

    if let Some(aff) = l.strip_prefix("soit ") {

        if aff.trim_start().starts_with('(') {
            if let Some((noms_bruts, expr)) = aff.split_once('=') {
                if let Some(interieur) = noms_bruts
                    .trim()
                    .strip_prefix('(')
                    .and_then(|r| r.strip_suffix(')'))
                {
                    let noms: Vec<String> =
                        coupe_args(interieur).iter().map(|s| s.trim().to_string()).collect();
                    if noms.len() >= 2 {
                        let (v, te) =
                            valeur_de_fragment(expr, locales, boites, fonctions, profondeur)?;
                        let (valeurs, types) = match (v, te) {
                            (Valeur::Uplet(vs), TypeVal::Uplet(ts)) => (vs, ts),
                            _ => return Err("on ne délie que les p-uplets".into()),
                        };
                        if valeurs.len() != noms.len() {
                            return Err(format!(
                                "la déliaison attend {} nom(s) ; le p-uplet en compte {}",
                                noms.len(),
                                valeurs.len()
                            ));
                        }
                        for ((n, valeur), type_val) in
                            noms.iter().zip(valeurs).zip(types.into_iter())
                        {
                            match valeur {
                                Valeur::Nombre(x) if !est_compose(&type_val) => {
                                    locales.insert(n.clone(), x);
                                }
                                autre => {
                                    boites.insert(n.clone(), Boite { type_val, val: autre });
                                }
                            }
                        }
                        return Ok(Flux::Continue);
                    }
                }
            }
        }
        if let Some((nom, expr)) = aff.split_once('=') {
            pose_locale(nom.trim(), expr, locales, boites, fonctions, profondeur)?;
            return Ok(Flux::Continue);
        }
    }

    if let Some(egal) = l.find('=') {
        if !l[egal..].starts_with("==") {
            let gauche = l[..egal].trim();
            if let Some((porteur, membre)) = gauche.split_once('.') {
                let porteur = porteur.trim();
                let membre = membre.trim();
                if boites.contains_key(porteur) && !membre.is_empty()
                    && membre.chars().all(|c| c.is_alphanumeric() || c == '_')
                {
                    super::conteneurs::ecrit_attribut(
                        porteur,
                        membre,
                        l[egal + 1..].trim(),
                        locales,
                        boites,
                        fonctions,
                    )?;
                    return Ok(Flux::Continue);
                }
            }
        }
    }

    if let Some((cible, rhs)) = ecriture_indexee(l) {
        if let Some((nom, indices)) = cible.split_once('[') {
            let indices = indices.trim_end_matches(']');
            super::conteneurs::ecrit_index(nom.trim(), indices, rhs, locales, boites, fonctions)?;
            return Ok(Flux::Continue);
        }
    }

    Err(format!("{} — instruction non comprise dans une fonction", l))
}

fn condition_vraie(
    cond: &str,
    locales: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> bool {

    let resolue = resoudre_appels(cond, locales, boites, fonctions, profondeur);
    let resolue = desencadre_texte(&resolue).unwrap_or(resolue);
    let resolue = super::conteneurs::resoudre_lectures(&resolue, locales, boites, true);
    let resolue = super::conteneurs::resoudre_noms_scalaires(&resolue, boites);
    crate::layout::controle::evalue_condition_publique(&resolue, locales)
}
