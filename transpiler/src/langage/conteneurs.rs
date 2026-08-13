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
    /// Le p-uplet du programme de NSI : arité fixe, types hétérogènes,
    /// noté `(entier ; entier)`. Il se distingue de la collection, homogène et
    /// de longueur variable — le programme les distingue explicitement.
    Uplet(Vec<TypeVal>),
    /// Une pile : dernier entré, premier sorti. Le sommet est le dernier
    /// élément écrit.
    Pile(Box<TypeVal>),
    /// Une file : premier entré, premier sorti. La tête est le premier.
    File(Box<TypeVal>),
    /// Une classe, désignée par son nom. Un nom de classe commence par une
    /// majuscule : c'est ce qui le distingue d'un type du langage.
    Objet(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Valeur {
    Nombre(f64),
    Complexe(f64, f64),
    Texte(String),
    Collection(Vec<Valeur>),
    Dictionnaire(Vec<(String, Valeur)>),
    Matrice(usize, usize, Vec<f64>),
    Uplet(Vec<Valeur>),
    Pile(Vec<Valeur>),
    File(Vec<Valeur>),
    /// Un objet : le nom de sa classe, la liste de ses ancêtres du plus
    /// proche au plus lointain, puis ses attributs dans l'ordre où la classe
    /// les déclare. La lignée voyage avec la valeur : c'est ce qui permet à
    /// un chien de tenir la place d'un animal sans que le vérificateur ait
    /// besoin de consulter les classes.
    Objet(String, Vec<String>, Vec<(String, Valeur)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Boite {
    pub type_val: TypeVal,
    pub val: Valeur,
}

pub type Boites = BTreeMap<String, Boite>;

/// Le nom du type au singulier — pour les messages qui parlent d'une valeur,
/// non d'une famille de valeurs.
pub fn nom_type_singulier(t: &TypeVal) -> String {
    match t {
        TypeVal::Entier => "entier".into(),
        TypeVal::Decimal => "décimal".into(),
        TypeVal::Reel => "réel".into(),
        TypeVal::Complexe => "complexe".into(),
        TypeVal::Texte => "chaîne de caractères".into(),
        TypeVal::Booleen => "booléen".into(),
        TypeVal::Collection(e) => format!("liste {}", elide(&nom_type(e))),
        TypeVal::Dictionnaire(e) => {
            format!("dictionnaire de textes et {}", elide(&nom_type(e)))
        }
        TypeVal::Matrice(Some((l, c)), e) => {
            format!("matrice {}×{} {}", l, c, elide(&nom_type(e)))
        }
        TypeVal::Matrice(None, e) => format!("matrice {}", elide(&nom_type(e))),
        TypeVal::Pile(e) => format!("pile {}", elide(&nom_type(e))),
        TypeVal::File(e) => format!("file {}", elide(&nom_type(e))),
        TypeVal::Objet(n) => n.clone(),
        TypeVal::Uplet(ts) => format!(
            "p-uplet ({})",
            ts.iter().map(nom_type_singulier).collect::<Vec<_>>().join(" ; ")
        ),
    }
}

/// « de » devant consonne, « d' » devant voyelle — l'élision française.
fn elide(mot: &str) -> String {
    let premiere = mot.chars().next().unwrap_or('x').to_lowercase().next().unwrap_or('x');
    if "aeiouyàâéèêëîïôöùûü".contains(premiere) {
        format!("d'{}", mot)
    } else {
        format!("de {}", mot)
    }
}

fn nom_type(t: &TypeVal) -> String {
    match t {
        TypeVal::Entier => "entiers".into(),
        TypeVal::Decimal => "décimaux".into(),
        TypeVal::Reel => "réels".into(),
        TypeVal::Complexe => "complexes".into(),
        TypeVal::Texte => "chaînes de caractères".into(),
        TypeVal::Booleen => "booléens".into(),
        TypeVal::Collection(e) => format!("listes {}", elide(&nom_type(e))),
        TypeVal::Dictionnaire(e) => format!("dictionnaires de textes et de {}", nom_type(e)),
        TypeVal::Matrice(Some((l, c)), e) => format!("matrices {}×{} de {}", l, c, nom_type(e)),
        TypeVal::Matrice(None, e) => format!("matrices de {}", nom_type(e)),
        TypeVal::Pile(e) => format!("piles {}", elide(&nom_type(e))),
        TypeVal::File(e) => format!("files {}", elide(&nom_type(e))),
        TypeVal::Objet(n) => format!("objets {}", n),
        TypeVal::Uplet(ts) => format!(
            "p-uplets ({})",
            ts.iter().map(nom_type_singulier).collect::<Vec<_>>().join(" ; ")
        ),
    }
}

/// Retire les guillemets qui délimitent une chaîne de caractères, droits ou
/// français. Ils ne font pas partie de la valeur : ils disent seulement où
/// elle commence et où elle finit.
pub fn sans_guillemets(s: &str) -> &str {
    let s = s.trim();
    s.strip_prefix('"')
        .and_then(|r| r.strip_suffix('"'))
        .or_else(|| s.strip_prefix('«').and_then(|r| r.strip_suffix('»')))
        .unwrap_or(s)
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
    // `(entier ; entier)`, éventuellement précédé de « p-uplet »
    let s = s.strip_prefix("p-uplets").or_else(|| s.strip_prefix("p-uplet")).unwrap_or(s).trim();
    if let Some(interieur) = s.strip_prefix('(').and_then(|r| r.strip_suffix(')')) {
        let parts = coupe_niveau_zero(interieur, ';');
        if parts.len() >= 2 {
            let mut membres = Vec::with_capacity(parts.len());
            for m in parts {
                membres.push(parse_type(m)?);
            }
            return Some(TypeVal::Uplet(membres));
        }
    }
    // « liste » et « tableau » sont les mots du programme de NSI ;
    // « collection » reste accepté. Un même type, trois façons de le nommer.
    for (tete, pile) in [("piles", true), ("pile", true), ("files", false), ("file", false)] {
        if let Some(r) = s.strip_prefix(tete) {
            if r.is_empty() || r.starts_with(|c: char| c.is_whitespace() || c == '\'') {
                let element = Box::new(parse_type(sans_article(r))?);
                return Some(if pile {
                    TypeVal::Pile(element)
                } else {
                    TypeVal::File(element)
                });
            }
        }
    }
    // `liste` est le mot du programme de NSI, et le seul du langage :
    // une notion, un mot.
    for tete in ["listes", "liste"] {
        if let Some(r) = s.strip_prefix(tete) {
            if r.is_empty() || r.starts_with(|c: char| c.is_whitespace() || c == '\'') {
                return Some(TypeVal::Collection(Box::new(parse_type(sans_article(r))?)));
            }
        }
    }
    if let Some(r) = s.strip_prefix("dictionnaire") {
        let r = sans_article(r);
        // La clé est toujours une chaîne de caractères ; le mot qui la nomme
        // suit le vocabulaire du langage, non une forme figée.
        let r = ["chaînes de caractères", "chaines de caracteres"]
            .iter()
            .find_map(|mot| r.strip_prefix(mot))?
            .trim_start();
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
    let nu = s.trim_end_matches(|c: char| c == '.' || c == ';').trim();
    if nu.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
        && nu.chars().all(|c| c.is_alphanumeric() || c == '_')
    {
        return Some(TypeVal::Objet(nu.to_string()));
    }
    match nu {
        "entiers" | "entier" => Some(TypeVal::Entier),
        "décimaux" | "decimaux" | "décimal" | "decimal" => Some(TypeVal::Decimal),
        "réels" | "reels" | "réel" | "reel" => Some(TypeVal::Reel),
        "complexes" | "complexe" => Some(TypeVal::Complexe),
        // « chaîne de caractères » est le terme qu'emploient les professeurs
        // d'informatique : c'est le seul du langage. Les variantes sans
        // accent sont admises parce qu'un clavier peut manquer, non parce
        // qu'elles nomment autre chose.
        "chaînes de caractères" | "chaîne de caractères"
        | "chaines de caracteres" | "chaine de caracteres" => Some(TypeVal::Texte),
        "booléens" | "booleens" | "booléen" | "booleen" => Some(TypeVal::Booleen),
        _ => None,
    }
}

fn coupe_niveau_zero(s: &str, sep: char) -> Vec<&str> {
    crate::utils::decoupe::coupe_elements(s, sep)
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
        (TypeVal::Pile(e), Valeur::Pile(elems)) | (TypeVal::File(e), Valeur::File(elems)) => {
            for el in elems {
                verifie(e, el)?;
            }
            Ok(())
        }
        (TypeVal::Objet(attendu), Valeur::Objet(recu, ancetres, _)) => {
            if attendu == recu || ancetres.iter().any(|a| a == attendu) {
                Ok(())
            } else {
                Err(format!("un {} n'est pas un {}", recu, attendu))
            }
        }
        (TypeVal::Uplet(ts), Valeur::Uplet(vs)) => {
            if ts.len() != vs.len() {
                return Err(format!(
                    "ce p-uplet compte {} valeur(s) au lieu de {}",
                    vs.len(),
                    ts.len()
                ));
            }
            for (te, v) in ts.iter().zip(vs.iter()) {
                verifie(te, v)?;
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
    // Une chaîne peut s'écrire entre guillemets — droits ou français. Les
    // guillemets ne font pas partie de la valeur ; ils la délimitent, ce qui
    // permet d'y garder les espaces et les signes du langage.
    if matches!(attendu, TypeVal::Texte) {
        let sans = brut
            .strip_prefix('"')
            .and_then(|r| r.strip_suffix('"'))
            .or_else(|| brut.strip_prefix('«').and_then(|r| r.strip_suffix('»')));
        if let Some(contenu) = sans {
            // Les guillemets servent précisément à garder les espaces :
            // un séparateur comme « - » en dépend.
            return Ok(Valeur::Texte(contenu.to_string()));
        }
    }
    // `{Chien("Rex") ; Chat("Mia")}` — un élément peut être un objet qu'on
    // construit sur place. Seul ce cas passe par l'évaluateur d'expressions :
    // les autres types se lisent déjà ici, et l'y renvoyer tournerait en rond.
    if matches!(attendu, TypeVal::Objet(_)) {
        return super::fonctions::evalue_valeur(brut, attendu, vars, boites, fns);
    }
    if let TypeVal::Uplet(membres) = attendu {
        let interieur = brut
            .strip_prefix('(')
            .and_then(|r| r.strip_suffix(')'))
            .ok_or_else(|| format!("{} : un p-uplet s'écrit (valeur ; valeur)", brut))?;
        let parts = coupe_niveau_zero(interieur, ';');
        if parts.len() != membres.len() {
            return Err(format!(
                "{} : ce p-uplet compte {} valeur(s) au lieu de {}",
                brut,
                parts.len(),
                membres.len()
            ));
        }
        let mut valeurs = Vec::with_capacity(parts.len());
        for (m, te) in parts.iter().zip(membres.iter()) {
            valeurs.push(parse_element(m, te, vars, boites, fns)?);
        }
        return Ok(Valeur::Uplet(valeurs));
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
    if let TypeVal::Pile(elem) | TypeVal::File(elem) = attendu {
        let interieur = interieur.trim();
        let mut valeurs = Vec::new();
        if !interieur.is_empty() {
            for m in coupe_niveau_zero(interieur, ';') {
                valeurs.push(parse_element(m, elem, vars, boites, fns)?);
            }
        }
        return Ok(match attendu {
            TypeVal::Pile(_) => Valeur::Pile(valeurs),
            _ => Valeur::File(valeurs),
        });
    }
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
                // La clé est une chaîne de caractères : les guillemets la
                // délimitent, comme partout ailleurs, sans lui appartenir.
                paires.push((
                    sans_guillemets(cle.trim()).to_string(),
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
        (Valeur::Pile(elems), TypeVal::Pile(e)) | (Valeur::File(elems), TypeVal::File(e)) => {
            format!(
                "{{{}}}",
                elems.iter().map(|el| formate(el, e)).collect::<Vec<_>>().join(" ; ")
            )
        }
        (Valeur::Objet(nom, _, attributs), _) => format!(
            "{}({})",
            nom,
            attributs
                .iter()
                .map(|(n, v)| format!("{}: {}", n, formate(v, &type_de_valeur(v))))
                .collect::<Vec<_>>()
                .join(" ; ")
        ),
        (Valeur::Uplet(vs), TypeVal::Uplet(ts)) => format!(
            "({})",
            vs.iter()
                .zip(ts.iter())
                .map(|(v, te)| formate(v, te))
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

/// Dépouille une lettre de son accent — trente lignes, aucune dépendance.
/// C'est ce qui fait classer « école » avant « Zoé » alors que le codepoint
/// de « é » vient après celui de « z ».
pub fn depouille(c: char) -> char {
    match c {
        'à' | 'â' | 'ä' | 'á' | 'ã' => 'a',
        'ç' => 'c',
        'é' | 'è' | 'ê' | 'ë' => 'e',
        'î' | 'ï' | 'ì' | 'í' => 'i',
        'ô' | 'ö' | 'ò' | 'ó' | 'õ' => 'o',
        'ù' | 'û' | 'ü' | 'ú' => 'u',
        'ÿ' | 'ý' => 'y',
        'ñ' => 'n',
        'À' | 'Â' | 'Ä' | 'Á' | 'Ã' => 'A',
        'Ç' => 'C',
        'É' | 'È' | 'Ê' | 'Ë' => 'E',
        'Î' | 'Ï' | 'Ì' | 'Í' => 'I',
        'Ô' | 'Ö' | 'Ò' | 'Ó' | 'Õ' => 'O',
        'Ù' | 'Û' | 'Ü' | 'Ú' => 'U',
        'Ñ' => 'N',
        autre => autre,
    }
}

/// Le type que porte une valeur, pour les messages de faute.
pub fn type_de_valeur(v: &Valeur) -> TypeVal {
    match v {
        Valeur::Nombre(_) => TypeVal::Reel,
        Valeur::Complexe(_, _) => TypeVal::Complexe,
        Valeur::Texte(_) => TypeVal::Texte,
        Valeur::Collection(_) => TypeVal::Collection(Box::new(TypeVal::Reel)),
        Valeur::Dictionnaire(_) => TypeVal::Dictionnaire(Box::new(TypeVal::Reel)),
        Valeur::Matrice(l, c, _) => TypeVal::Matrice(Some((*l, *c)), Box::new(TypeVal::Reel)),
        Valeur::Uplet(v) => TypeVal::Uplet(v.iter().map(type_de_valeur).collect()),
        Valeur::Pile(e) => TypeVal::Pile(Box::new(
            e.first().map(type_de_valeur).unwrap_or(TypeVal::Reel),
        )),
        Valeur::File(e) => TypeVal::File(Box::new(
            e.first().map(type_de_valeur).unwrap_or(TypeVal::Reel),
        )),
        Valeur::Objet(n, _, _) => TypeVal::Objet(n.clone()),
    }
}

/// Le nombre d'éléments d'un conteneur — ce que rend `longueur(v)`.
pub fn cardinal(v: &Valeur) -> usize {
    match v {
        Valeur::Collection(e) => e.len(),
        Valeur::Dictionnaire(p) => p.len(),
        Valeur::Matrice(l, _, _) => *l,
        Valeur::Texte(s) => s.chars().count(),
        Valeur::Uplet(v) => v.len(),
        Valeur::Pile(e) | Valeur::File(e) => e.len(),
        Valeur::Objet(_, _, a) => a.len(),
        _ => 1,
    }
}

/// La forme **relisible** d'une valeur : celle qu'on peut réécrire dans le
/// texte et qui sera relue à l'identique.
///
/// Elle diffère de la forme affichée : un objet s'imprime `Point(abscisse: 3)`
/// pour le lecteur, mais se réécrit `Point(3)` — l'appel qui le construit.
/// C'est ce qui permet à un attribut de contenir lui-même un objet, donc à un
/// arbre ou à une liste chaînée d'exister.
pub fn forme_relisible(v: &Valeur) -> String {
    match v {
        Valeur::Texte(s) => format!("\"{}\"", s),
        Valeur::Objet(nom, _, attributs) => format!(
            "{}({})",
            nom,
            attributs
                .iter()
                .map(|(_, x)| forme_relisible(x))
                .collect::<Vec<_>>()
                .join(" ; ")
        ),
        Valeur::Collection(e) | Valeur::Pile(e) | Valeur::File(e) => format!(
            "{{{}}}",
            e.iter().map(forme_relisible).collect::<Vec<_>>().join(" ; ")
        ),
        Valeur::Uplet(e) => format!(
            "({})",
            e.iter().map(forme_relisible).collect::<Vec<_>>().join(" ; ")
        ),
        Valeur::Dictionnaire(paires) => format!(
            "{{{}}}",
            paires
                .iter()
                .map(|(k, x)| format!("{}: {}", k, forme_relisible(x)))
                .collect::<Vec<_>>()
                .join(" ; ")
        ),
        autre => formate(autre, &type_de_valeur(autre)),
    }
}

pub fn affiche(b: &Boite) -> String {
    formate(&b.val, &b.type_val)
}

/// L'empreinte structurelle d'une boîte, **sans rien allouer**.
///
/// Le cache de segments hachait jusqu'ici la forme imprimée de chaque
/// conteneur — `affiche(boite)` — et il le refaisait après *chaque* segment :
/// le coût du rendu était le produit du nombre de segments par celui des
/// conteneurs, une chaîne complète étant construite puis jetée à chaque
/// croisement. Ici on descend dans la valeur elle-même.
///
/// L'empreinte distingue au moins tout ce que la forme imprimée distinguait :
/// deux boîtes de même empreinte sont bien interchangeables pour le cache.
pub fn empreinte_boite<H: std::hash::Hasher>(b: &Boite, h: &mut H) {
    empreinte_type(&b.type_val, h);
    empreinte_valeur(&b.val, h);
}

fn empreinte_type<H: std::hash::Hasher>(t: &TypeVal, h: &mut H) {
    use std::hash::Hash;
    match t {
        TypeVal::Entier => h.write_u8(0),
        TypeVal::Decimal => h.write_u8(1),
        TypeVal::Reel => h.write_u8(2),
        TypeVal::Complexe => h.write_u8(3),
        TypeVal::Texte => h.write_u8(4),
        TypeVal::Booleen => h.write_u8(5),
        TypeVal::Collection(e) => {
            h.write_u8(6);
            empreinte_type(e, h);
        }
        TypeVal::Dictionnaire(e) => {
            h.write_u8(7);
            empreinte_type(e, h);
        }
        TypeVal::Matrice(dim, e) => {
            h.write_u8(8);
            match dim {
                Some((l, c)) => {
                    h.write_u8(1);
                    h.write_usize(*l);
                    h.write_usize(*c);
                }
                None => h.write_u8(0),
            }
            empreinte_type(e, h);
        }
        TypeVal::Uplet(ts) => {
            h.write_u8(9);
            h.write_usize(ts.len());
            for x in ts {
                empreinte_type(x, h);
            }
        }
        TypeVal::Pile(e) => {
            h.write_u8(10);
            empreinte_type(e, h);
        }
        TypeVal::File(e) => {
            h.write_u8(11);
            empreinte_type(e, h);
        }
        TypeVal::Objet(nom) => {
            h.write_u8(12);
            nom.hash(h);
        }
    }
}

fn empreinte_valeur<H: std::hash::Hasher>(v: &Valeur, h: &mut H) {
    use std::hash::Hash;
    match v {
        // `to_bits` plutôt que le nombre : c'est la seule façon de hacher un
        // flottant, et elle sépare `0,0` de `-0,0` — ce que la forme imprimée
        // ne faisait pas non plus.
        Valeur::Nombre(n) => {
            h.write_u8(0);
            h.write_u64(n.to_bits());
        }
        Valeur::Complexe(re, im) => {
            h.write_u8(1);
            h.write_u64(re.to_bits());
            h.write_u64(im.to_bits());
        }
        Valeur::Texte(s) => {
            h.write_u8(2);
            s.hash(h);
        }
        Valeur::Collection(e) => {
            h.write_u8(3);
            empreinte_suite(e, h);
        }
        Valeur::Dictionnaire(paires) => {
            h.write_u8(4);
            h.write_usize(paires.len());
            for (cle, val) in paires {
                cle.hash(h);
                empreinte_valeur(val, h);
            }
        }
        Valeur::Matrice(l, c, cases) => {
            h.write_u8(5);
            h.write_usize(*l);
            h.write_usize(*c);
            for x in cases {
                h.write_u64(x.to_bits());
            }
        }
        Valeur::Uplet(e) => {
            h.write_u8(6);
            empreinte_suite(e, h);
        }
        Valeur::Pile(e) => {
            h.write_u8(7);
            empreinte_suite(e, h);
        }
        Valeur::File(e) => {
            h.write_u8(8);
            empreinte_suite(e, h);
        }
        Valeur::Objet(classe, ancetres, attributs) => {
            h.write_u8(9);
            classe.hash(h);
            h.write_usize(ancetres.len());
            for a in ancetres {
                a.hash(h);
            }
            h.write_usize(attributs.len());
            for (nom, val) in attributs {
                nom.hash(h);
                empreinte_valeur(val, h);
            }
        }
    }
}

fn empreinte_suite<H: std::hash::Hasher>(e: &[Valeur], h: &mut H) {
    h.write_usize(e.len());
    for x in e {
        empreinte_valeur(x, h);
    }
}

pub fn type_element(b: &Boite) -> TypeVal {
    match &b.type_val {
        TypeVal::Collection(e) | TypeVal::Dictionnaire(e) | TypeVal::Matrice(_, e) => (**e).clone(),
        autre => autre.clone(),
    }
}

/// Les valeurs d'un conteneur, telles quelles. Un objet ne survit pas à
/// l'écriture puis à la relecture de son texte : la boucle qui le parcourt
/// doit recevoir la valeur, non sa forme imprimée.
pub fn valeurs_pour_boucle(b: &Boite) -> Option<Vec<Valeur>> {
    match &b.val {
        Valeur::Collection(e) if e.iter().any(|v| matches!(v, Valeur::Objet(_, _, _))) => {
            Some(e.clone())
        }
        _ => None,
    }
}

pub fn elements_pour_boucle(b: &Boite) -> Vec<String> {
    let te = type_element(b);
    match &b.val {
        Valeur::Collection(elems) => elems.iter().map(|el| formate(el, &te)).collect(),
        Valeur::Dictionnaire(paires) => paires.iter().map(|(k, _)| k.clone()).collect(),
        // `pour c dans mot` livre les lettres, une à une.
        Valeur::Texte(s) => s.chars().map(|c| c.to_string()).collect(),
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
    // Une clé peut être une variable : `pour k dans d { … d[k] … }`. Le nom
    // nu est remplacé par son contenu avant la recherche.
    let resolus;
    let indices_bruts = if matches!(b.val, Valeur::Dictionnaire(_)) {
        let indices_bruts = sans_guillemets(indices_bruts);
        resolus = resoudre_noms_scalaires(indices_bruts, boites);
        resolus.as_str()
    } else {
        indices_bruts
    };
    let parts: Vec<&str> = coupe_niveau_zero(indices_bruts, ';')
        .into_iter()
        .map(|s| s.trim())
        .collect();
    let te = type_element(b);
    // `v[i à j]` — une tranche, bornes incluses, comme « de 1 à 5 » fait cinq
    // tours. Le « à » évite la collision avec l'indexation matricielle
    // `A[i ; j]`. Une tranche dont la borne gauche dépasse la droite est vide :
    // c'est ce qui permet de conclure une fusion de listes en une ligne.
    if parts.len() == 1 {
        if let Some((g, d)) = parts[0].split_once(" à ") {
            let borne = |e: &str| -> Result<i64, String> {
                let r = resoudre_lectures(e, vars, boites, true);
                crate::maths::calcul::eval(&r, vars)
                    .map(|n| n.round() as i64)
                    .ok_or_else(|| format!("{} n'est pas une position", e.trim()))
            };
            let (i, j) = (borne(g)?, borne(d)?);
            let taille = cardinal(&b.val) as i64;
            if i > j {
                return Ok(match &b.val {
                    Valeur::Texte(_) => (Valeur::Texte(String::new()), TypeVal::Texte),
                    _ => (Valeur::Collection(Vec::new()), b.type_val.clone()),
                });
            }
            if i < 0 || j >= taille {
                return Err(format!(
                    "la tranche de {} à {} sort des bornes : {} compte {} élément(s)",
                    i, j, nom, taille
                ));
            }
            return Ok(match &b.val {
                Valeur::Collection(elems) => (
                    Valeur::Collection(elems[i as usize..=j as usize].to_vec()),
                    b.type_val.clone(),
                ),
                Valeur::Texte(s) => {
                    let lettres: Vec<char> = s.chars().collect();
                    (
                        Valeur::Texte(lettres[i as usize..=j as usize].iter().collect()),
                        TypeVal::Texte,
                    )
                }
                _ => return Err(format!("{} ne se découpe pas en tranches", nom)),
            });
        }
    }
    if let Valeur::Uplet(vs) = &b.val {
        let expr = resoudre_lectures(parts[0], vars, boites, true);
        let i = crate::maths::calcul::eval(&expr, vars)
            .ok_or_else(|| format!("{} n'est pas une position", parts[0]))?
            .round() as i64;
        if i < 0 || i as usize >= vs.len() {
            return Err(format!(
                "l'indice {} sort des bornes : {} compte {} valeur(s), d'indices 0 à {}",
                i, nom, vs.len(), vs.len().saturating_sub(1)
            ));
        }
        let te = match &b.type_val {
            TypeVal::Uplet(ts) => ts[i as usize].clone(),
            autre => autre.clone(),
        };
        return Ok((vs[i as usize].clone(), te));
    }
    if let Valeur::Texte(s) = &b.val {
        // Une lettre est une chaîne d'un seul caractère : docdg n'a pas de
        // type « caractère » distinct, et le programme de NSI n'en demande
        // pas — en Python non plus une lettre n'est qu'une chaîne de un.
        let expr = resoudre_lectures(parts[0], vars, boites, true);
        let i = crate::maths::calcul::eval(&expr, vars)
            .ok_or_else(|| format!("{} n'est pas une position", parts[0]))?
            .round() as i64;
        let lettres: Vec<char> = s.chars().collect();
        if i < 0 || i as usize >= lettres.len() {
            return Err(format!(
                "l'indice {} sort des bornes : {} compte {} lettre(s), d'indices 0 à {}",
                i,
                nom,
                lettres.len(),
                lettres.len().saturating_sub(1)
            ));
        }
        return Ok((Valeur::Texte(lettres[i as usize].to_string()), TypeVal::Texte));
    }
    match &b.val {
        Valeur::Collection(elems) => {
            let expr = resoudre_lectures(parts[0], vars, boites, true);
            let i = crate::maths::calcul::eval(&expr, vars)
                .filter(|n| est_entier(*n) && *n >= 0.0)
                .ok_or_else(|| format!("{}[{}] — indice invalide", nom, indices_bruts.trim()))?
                as usize;
            let el = elems.get(i).ok_or_else(|| {
                format!(
                    "{}[{}] — indice hors bornes (la liste compte {} élément(s))",
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

/// Remplace les noms nus qui désignent une **chaîne** par leur contenu.
/// Les conteneurs ne sont pas touchés : ils n'ont de sens qu'indexés ou passés
/// à une primitive, qui les lit d'elle-même.
pub fn resoudre_noms_scalaires(texte: &str, boites: &Boites) -> String {
    let mut sortie = String::with_capacity(texte.len());
    let lettres: Vec<char> = texte.chars().collect();
    let mut i = 0usize;
    while i < lettres.len() {
        if !(lettres[i].is_alphabetic() || lettres[i] == '_') {
            sortie.push(lettres[i]);
            i += 1;
            continue;
        }
        let debut = i;
        while i < lettres.len() && (lettres[i].is_alphanumeric() || lettres[i] == '_') {
            i += 1;
        }
        let mot: String = lettres[debut..i].iter().collect();
        // un nom suivi d'une parenthèse ou d'un crochet est un appel ou une
        // lecture : on le laisse à qui sait le traiter
        let colle = lettres.get(i).map(|c| *c == '(' || c == &'[').unwrap_or(false);
        match boites.get(&mot) {
            Some(b) if !colle && matches!(b.val, Valeur::Texte(_)) => {
                if let Valeur::Texte(s) = &b.val {
                    sortie.push_str(s);
                }
            }
            _ => sortie.push_str(&mot),
        }
    }
    sortie
}

/// `p.abscisse = 5` — la modification d'un attribut.
///
/// docdg n'a **pas de références** : une boîte n'est jamais partagée. Muter la
/// boîte et lui réaffecter une copie modifiée sont donc indiscernables, et la
/// question mutation ou copie ne se pose pas — il n'y a rien qu'un `&`
/// pourrait distinguer. Le langage reste entièrement par valeur.
pub fn ecrit_attribut(
    nom: &str,
    membre: &str,
    rhs: &str,
    vars: &BTreeMap<String, f64>,
    boites: &mut Boites,
    fns: &super::fonctions::Fonctions,
) -> Result<(), String> {
    let boite = boites
        .get(nom)
        .ok_or_else(|| format!("{} n'a pas été posé", nom))?
        .clone();
    let Valeur::Objet(classe, ancetres, attributs) = boite.val else {
        return Err(format!("{} n'est pas un objet", nom));
    };
    if fns
        .get(&classe)
        .map(|f| f.prives.iter().any(|n| n == membre))
        .unwrap_or(false)
    {
        return Err(format!(
            "{} est un attribut privé de {} : il ne s'écrit que depuis la classe",
            membre, classe
        ));
    }
    let position = attributs
        .iter()
        .position(|(n, _)| n == membre)
        .ok_or_else(|| format!("{} n'a pas d'attribut {}", classe, membre))?;
    let attendu = fns
        .get(&classe)
        .and_then(|f| f.params.get(position).map(|(_, t)| t.clone()))
        .unwrap_or_else(|| type_de_valeur(&attributs[position].1));
    let valeur = super::fonctions::evalue_valeur(rhs, &attendu, vars, boites, fns).map_err(
        |_| {
            format!(
                "{} ne se lit pas comme {}",
                rhs.trim(),
                nom_type_singulier(&attendu)
            )
        },
    )?;
    verifie(&attendu, &valeur).map_err(|e| format!("{}.{} : {}", nom, membre, e))?;
    let mut nouveaux = attributs;
    nouveaux[position].1 = valeur;
    boites.insert(
        nom.to_string(),
        Boite {
            type_val: boite.type_val,
            val: Valeur::Objet(classe, ancetres, nouveaux),
        },
    );
    Ok(())
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
                    "{}[{}] — indice hors bornes (la liste compte {} élément(s)) ; une liste grandit par +, jamais par indice",
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
    crate::utils::erreur::source(&sans_syntaxe(ligne), &sans_syntaxe(message))
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
        // `soit (q ; r) = divise(17 ; 5)` — la déliaison d'un p-uplet. Les
        // deux noms sont posés d'un coup, chacun avec le type de son membre.
        if reste.trim_start().starts_with('(') {
            if let Some((noms_bruts, rhs)) = reste.split_once('=') {
                let noms_bruts = noms_bruts.trim();
                if let Some(interieur) = noms_bruts
                    .strip_prefix('(')
                    .and_then(|r| r.strip_suffix(')'))
                {
                    let noms: Vec<String> = coupe_niveau_zero(interieur, ';')
                        .iter()
                        .map(|s| s.trim().to_string())
                        .collect();
                    if noms.len() >= 2 && noms.iter().all(|n| {
                        !n.is_empty() && n.chars().all(|c| c.is_alphanumeric() || c == '_')
                    }) {
                        let brut = desentinelle(rhs[..rhs.len().min(fin_ligne)].trim());
                        let remplacement =
                            match super::fonctions::devine_valeur(&brut, vars, boites, fns) {
                                Ok((Valeur::Uplet(valeurs), TypeVal::Uplet(types)))
                                    if valeurs.len() == noms.len() =>
                                {
                                    for ((n, v), te) in
                                        noms.iter().zip(valeurs).zip(types.into_iter())
                                    {
                                        boites.insert(
                                            n.clone(),
                                            Boite { type_val: te, val: v },
                                        );
                                    }
                                    String::new()
                                }
                                Ok((Valeur::Uplet(valeurs), _)) => erreur_div(
                                    ligne,
                                    &format!(
                                        "la déliaison attend {} nom(s) ; le p-uplet en compte {}",
                                        noms.len(),
                                        valeurs.len()
                                    ),
                                ),
                                Ok(_) => erreur_div(
                                    ligne,
                                    "on ne délie que les p-uplets",
                                ),
                                Err(e) => erreur_div(ligne, &e),
                            };
                        return Some(Instruction {
                            consomme: decale + fin_ligne,
                            remplacement,
                        });
                    }
                }
            }
        }
        if let Some((nom, apres_dp)) = reste.split_once(':') {
            let nom = nom.trim();
            if !nom.is_empty()
                && nom.chars().all(|c| c.is_alphanumeric() || c == '_')
                && !apres_dp.trim_start().starts_with('=')
            {
                let (avant_egal, apres_egal) = apres_dp.split_once('=')?;
                let type_val = parse_type(avant_egal)?;
                // Un type scalaire n'a pas de littéral entre accolades : sa
                // valeur tient sur la fin de la ligne. C'est ce qui permet
                // `soit m: chaîne de caractères = "bonjour"`.
                if !matches!(
                    type_val,
                    TypeVal::Collection(_) | TypeVal::Dictionnaire(_) | TypeVal::Matrice(_, _)
                ) {
                    let brut = apres_egal[..apres_egal.len().min(fin_ligne)].trim();
                    let brut = desentinelle(brut);
                    // Un membre droit peut être un littéral — `"bonjour"`,
                    // `(3 ; 2)` — ou une expression : un appel, une primitive.
                    // On tente la seconde lecture d'abord, la plus large.
                    // Si les deux lectures échouent, c'est la première qu'on
                    // rapporte : elle nomme le type attendu, là où la seconde
                    // dit seulement qu'un calcul est impossible.
                    let lu = super::fonctions::evalue_valeur(&brut, &type_val, vars, boites, fns)
                        .or_else(|premiere| {
                            parse_element(&brut, &type_val, vars, boites, fns)
                                .map_err(|_| premiere)
                        });
                    let remplacement = match lu {
                        Ok(val) => {
                            boites.insert(nom.to_string(), Boite { type_val, val });
                            String::new()
                        }
                        Err(e) => erreur_div(&format!("soit {}: …", nom), &e),
                    };
                    return Some(Instruction {
                        consomme: decale + fin_ligne,
                        remplacement,
                    });
                }
                // Un membre droit qui ne commence pas par une accolade est
                // une expression : un appel, une primitive, une concaténation.
                if !apres_egal.trim_start().starts_with('{') {
                    let brut = desentinelle(apres_egal[..apres_egal.len().min(fin_ligne)].trim());
                    let remplacement =
                        match super::fonctions::evalue_valeur(&brut, &type_val, vars, boites, fns) {
                            Ok(val) => {
                                boites.insert(nom.to_string(), Boite { type_val, val });
                                String::new()
                            }
                            Err(e) => erreur_div(&format!("soit {}: …", nom), &e),
                        };
                    return Some(Instruction {
                        consomme: decale + fin_ligne,
                        remplacement,
                    });
                }
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
            let nom_simple = !nom.is_empty()
                && !nom.contains('[')
                && nom.chars().next().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false)
                && nom.chars().all(|c| c.is_alphanumeric() || c == '_');
            if !nom_simple {
                return None;
            }
            let rhs = desentinelle(rhs[..rhs.len().min(fin_ligne)].trim());

            // Une boîte déjà posée : la concaténation d'abord — c'est
            // l'accumulateur `soit S = S + {k}`. Si elle n'a pas de sens, on
            // réévalue l'expression dans le type de la boîte : c'est ce qui
            // permet `soit v = tri(v)`.
            if let Some(b) = boites.get(nom).cloned() {
                if concatene(nom, &rhs, vars, boites, fns).is_ok() {
                    return Some(Instruction { consomme: decale + fin_ligne, remplacement: String::new() });
                }
                let remplacement =
                    match super::fonctions::evalue_valeur(&rhs, &b.type_val, vars, boites, fns) {
                        Ok(val) => {
                            boites.insert(nom.to_string(), Boite { type_val: b.type_val, val });
                            String::new()
                        }
                        Err(e) => erreur_div(ligne, &e),
                    };
                return Some(Instruction { consomme: decale + fin_ligne, remplacement });
            }

            // Un nom neuf : si l'expression produit une valeur composée, elle
            // mérite une boîte, sans qu'il faille en écrire le type. Un nombre
            // reste un nombre et suit son chemin habituel.
            if let Ok((val, type_val)) = super::fonctions::devine_valeur(&rhs, vars, boites, fns) {
                if !matches!(val, Valeur::Nombre(_) | Valeur::Complexe(_, _)) {
                    boites.insert(nom.to_string(), Boite { type_val, val });
                    return Some(Instruction { consomme: decale + fin_ligne, remplacement: String::new() });
                }
            }
        }
        return None;
    }
    // `p.abscisse = 5`, sans `soit` puisque rien n'est déclaré
    if let Some(egal) = ligne.find('=') {
        if !ligne[egal..].starts_with("==") {
            let gauche = ligne[..egal].trim();
            if let Some((porteur, membre)) = gauche.split_once('.') {
                let porteur = porteur.trim();
                let membre = membre.trim();
                if boites.contains_key(porteur)
                    && membre.chars().all(|c| c.is_alphanumeric() || c == '_')
                    && !membre.is_empty()
                {
                    let rhs = desentinelle(ligne[egal + 1..].trim());
                    let remplacement =
                        match ecrit_attribut(porteur, membre, &rhs, vars, boites, fns) {
                            Ok(()) => String::new(),
                            Err(e) => erreur_div(ligne, &e),
                        };
                    return Some(Instruction {
                        consomme: decale + fin_ligne,
                        remplacement,
                    });
                }
            }
        }
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
