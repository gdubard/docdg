use super::conteneurs::{parse_type, Boites, TypeVal, Valeur};
use std::collections::BTreeMap;

const PROFONDEUR_MAX: usize = 200;

#[derive(Clone, Debug, PartialEq)]
pub struct Fonction {
    pub params: Vec<(String, TypeVal)>,
    pub retour: TypeVal,
    pub corps: String,
}

pub type Fonctions = BTreeMap<String, Fonction>;

fn groupe_apparie(s: &str, ouvre: char, ferme: char) -> Option<usize> {
    let mut profondeur = 0i32;
    for (i, c) in s.char_indices() {
        if c == ouvre {
            profondeur += 1;
        } else if c == ferme {
            profondeur -= 1;
            if profondeur == 0 {
                return Some(i);
            }
        }
    }
    None
}

fn coupe_args(s: &str) -> Vec<&str> {
    let mut morceaux = Vec::new();
    let mut profondeur = 0i32;
    let mut debut = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' | '{' | '[' => profondeur += 1,
            ')' | '}' | ']' => profondeur -= 1,
            ';' if profondeur == 0 => {
                morceaux.push(&s[debut..i]);
                debut = i + 1;
            }
            _ => {}
        }
    }
    if debut <= s.len() {
        morceaux.push(&s[debut..]);
    }
    morceaux
}

fn nom_valide(s: &str) -> bool {
    !s.is_empty()
        && s.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false)
        && s.chars().all(|c| c.is_alphanumeric() || c == '_')
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
        },
        consomme,
    ))
}

fn verifie_nombre(t: &TypeVal, n: f64, quoi: &str) -> Result<(), String> {
    super::conteneurs::verifie(t, &Valeur::Nombre(n)).map_err(|e| format!("{} : {}", quoi, e))
}

pub fn resoudre_appels(
    texte: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> String {
    if fonctions.is_empty() || !texte.contains('(') {
        return texte.to_string();
    }
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
            Ok(v) => out.push_str(&format!("({})", v)),
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
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<f64, String> {
    let t = corps.trim();
    if let Some(interieur) = t.strip_prefix('{').and_then(|r| r.strip_suffix('}')) {
        let lignes: Vec<&str> = interieur
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();
        if lignes.len() == 1
            && !lignes[0].starts_with("retourne ")
            && !lignes[0].starts_with("soit ")
        {
            return evalue_corps(lignes[0], locales, boites, fonctions, profondeur);
        }
        let mut vu_retour = false;
        let mut valeur = 0.0;
        for ligne in lignes {
            let l = ligne.trim();
            if l.is_empty() {
                continue;
            }
            if let Some(expr) = l.strip_prefix("retourne ") {
                valeur = evalue_corps(expr, locales, boites, fonctions, profondeur)?;
                vu_retour = true;
                break;
            }
            if let Some(aff) = l.strip_prefix("soit ") {
                if let Some((nom, expr)) = aff.split_once('=') {
                    let v = evalue_corps(expr, locales, boites, fonctions, profondeur)?;
                    locales.insert(nom.trim().to_string(), v);
                    continue;
                }
            }
            return Err(format!("{} — instruction non comprise dans une fonction", l));
        }
        if !vu_retour {
            return Err("la fonction ne retourne rien : il manque « retourne »".into());
        }
        return Ok(valeur);
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
                let vrai = crate::layout::rendu::evalue_condition_publique(&cond_resolue, locales);
                if vrai {
                    return evalue_corps(alors, locales, boites, fonctions, profondeur);
                }
                let Some(sinon) = sinon else {
                    return Err("un « si » sans « sinon » ne retourne rien quand la condition est fausse".into());
                };
                return evalue_corps(sinon, locales, boites, fonctions, profondeur);
            }
        }
    }
    evalue_expression(t, locales, boites, fonctions, profondeur)
}

pub fn appelle(
    nom: &str,
    args_bruts: &str,
    vars: &BTreeMap<String, f64>,
    boites: &Boites,
    fonctions: &Fonctions,
    profondeur: usize,
) -> Result<f64, String> {
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
    for ((p, t), a) in f.params.iter().zip(args.iter()) {
        let v = evalue_expression(a, vars, boites, fonctions, profondeur + 1)?;
        verifie_nombre(t, v, &format!("{}, argument {}", nom, p))?;
        locales.insert(p.clone(), v);
    }
    let valeur = evalue_corps(&f.corps, &mut locales, boites, fonctions, profondeur + 1)?;
    verifie_nombre(&f.retour, valeur, &format!("{}, valeur retournée", nom))?;
    Ok(valeur)
}
