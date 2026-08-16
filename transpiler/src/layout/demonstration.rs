use super::rendu::{render_body_indent, render_inline};
use crate::{Env, TocEntry};

enum Morceau {
    Etape { nom: String, titre: String, corps: String },
    Libre(String),
}

fn etape_ouverte(ligne: &str) -> Option<(String, String)> {
    let t = ligne.trim();
    let sans = t.strip_suffix('{')?.trim_end();
    let bas: String = sans
        .chars()
        .map(crate::utils::texte::pli)
        .collect::<String>()
        .to_lowercase();
    for nom in [
        "initialisation",
        "hérédité",
        "contraposée",
        "absurde",
        "contradiction",
        "analyse",
        "synthèse",
        "directe",
        "réciproque",
        "soit",
        "objets",
        "tiroirs",
        "existence",
        "unicité",
    ] {
        let nom_plie: String = nom
            .chars()
            .map(crate::utils::texte::pli)
            .collect::<String>()
            .to_lowercase();
        if bas == nom_plie {
            return Some((nom.to_string(), String::new()));
        }
    }
    if let Some(titre) = bas.strip_prefix("cas ").or(bas.strip_prefix("cas")) {
        if titre.trim().is_empty() && bas != "cas" {
            return None;
        }
        let titre_source = sans[3.min(sans.len())..].trim().to_string();
        return Some(("cas".to_string(), titre_source));
    }
    None
}

fn morceaux(corps: &str) -> Vec<Morceau> {
    let mut out = Vec::new();
    let mut libre = String::new();
    let mut lignes = corps.lines().peekable();
    while let Some(ligne) = lignes.next() {
        if let Some((nom, titre)) = etape_ouverte(ligne) {
            if !libre.trim().is_empty() {
                out.push(Morceau::Libre(std::mem::take(&mut libre)));
            } else {
                libre.clear();
            }
            let mut interieur = String::new();
            let mut profondeur = 1i32;
            for suite in lignes.by_ref() {
                crate::utils::texte::maj_profondeur(suite, &mut profondeur);
                if profondeur <= 0 {
                    break;
                }
                interieur.push_str(suite);
                interieur.push('\n');
            }
            out.push(Morceau::Etape {
                nom,
                titre,
                corps: interieur,
            });
        } else {
            libre.push_str(ligne);
            libre.push('\n');
        }
    }
    if !libre.trim().is_empty() {
        out.push(Morceau::Libre(libre));
    }
    out
}

fn sans_que(desc: &str) -> String {
    let t = desc.trim();
    t.strip_prefix("que ")
        .or_else(|| t.strip_prefix("qu'"))
        .unwrap_or(t)
        .trim()
        .trim_end_matches('.')
        .to_string()
}

fn ordinal(i: usize) -> String {
    match i {
        0 => "Premier cas".to_string(),
        1 => "Deuxième cas".to_string(),
        2 => "Troisième cas".to_string(),
        3 => "Quatrième cas".to_string(),
        4 => "Cinquième cas".to_string(),
        n => format!("Cas {}", n + 1),
    }
}

fn phrase(texte: &str) -> String {
    format!("<p>{}</p>", texte)
}

fn etiquette(nom: &str, complement: &str) -> String {
    if complement.is_empty() {
        format!("<p><strong>{}.</strong></p>", nom)
    } else {
        format!("<p><strong>{}</strong> — {}.</p>", nom, complement)
    }
}

fn manque(tag: &str, nom: &str) -> String {
    crate::utils::erreur::bloc(
        &format!("<{}>", tag),
        &format!("il manque l'étape « {}{{…}} »", nom),
    )
}

pub(crate) fn montre(
    tag_t: &str,
    desc: &str,
    corps: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
) -> Option<String> {
    let modif = tag_t
        .strip_prefix("Montre")?
        .trim()
        .replace('\u{2019}', "'");
    let desc = desc.trim();
    let enonce_rendu = render_inline(desc, env, toc);
    let enonce_nu = render_inline(&sans_que(desc), env, toc);
    let parts = morceaux(corps);

    let rend = |p: &Morceau, env: &mut Env, toc: &mut Vec<TocEntry>| -> String {
        match p {
            Morceau::Libre(t) => render_body_indent(t, env, toc, false),
            Morceau::Etape { corps, .. } => render_body_indent(corps, env, toc, false),
        }
    };
    let trouve = |nom: &str| {
        parts.iter().find_map(|p| match p {
            Morceau::Etape { nom: n, corps, .. } if n == nom => Some(corps.clone()),
            _ => None,
        })
    };
    let en_ligne = |texte: &str, env: &mut Env, toc: &mut Vec<TocEntry>| -> String {
        render_inline(texte.trim().trim_end_matches('.'), env, toc)
    };

    let mut html = String::new();
    match modif.as_str() {

        "" => {
            html.push_str(&phrase(&format!("Montrons {}.", enonce_rendu)));
            for p in &parts {
                html.push_str(&rend(p, env, toc));
            }
        }

        "par contraposée" => {
            let c = match trouve("contraposée") {
                Some(c) => c,
                None => return Some(manque(tag_t, "contraposée")),
            };
            let _ = &c;
            html.push_str(&phrase(&format!("Montrons {}.", enonce_rendu)));
            for p in &parts {
                match p {
                    Morceau::Etape { nom, corps, .. } if nom == "contraposée" => {
                        html.push_str(&phrase(&format!(
                            "Raisonnons par contraposée : montrons que {}.",
                            en_ligne(corps, env, toc)
                        )));
                    }
                    _ => html.push_str(&rend(p, env, toc)),
                }
            }
            html.push_str(&phrase(&format!("Par contraposition, {}.", enonce_nu)));
        }

        "par l'absurde" => {
            let h = match trouve("absurde") {
                Some(h) => h,
                None => return Some(manque(tag_t, "absurde")),
            };
            let _ = &h;
            html.push_str(&phrase(&format!("Montrons {}.", enonce_rendu)));

            for p in &parts {
                match p {
                    Morceau::Etape { nom, corps, .. } if nom == "absurde" => {
                        html.push_str(&phrase(&format!(
                            "Raisonnons par l'absurde : supposons que {}.",
                            en_ligne(corps, env, toc)
                        )));
                    }
                    Morceau::Etape { nom, corps, .. } if nom == "contradiction" => {
                        html.push_str(&phrase(&format!(
                            "Contradiction : {}.",
                            en_ligne(corps, env, toc)
                        )));
                    }
                    _ => html.push_str(&rend(p, env, toc)),
                }
            }
            html.push_str(&phrase(&format!("Donc {}.", enonce_nu)));
        }

        "par récurrence" => {
            let init = match trouve("initialisation") {
                Some(i) => i,
                None => return Some(manque(tag_t, "initialisation")),
            };
            let her = match trouve("hérédité") {
                Some(h) => h,
                None => return Some(manque(tag_t, "hérédité")),
            };
            html.push_str(&phrase(&format!(
                "Montrons par récurrence {}.",
                enonce_rendu
            )));
            html.push_str(&etiquette("Initialisation", ""));
            html.push_str(&render_body_indent(&init, env, toc, false));
            html.push_str(&etiquette("Hérédité", ""));
            html.push_str(&render_body_indent(&her, env, toc, false));
            html.push_str(&phrase(&format!(
                "La propriété est vraie au premier rang et héréditaire : d'après le principe de récurrence, {}.",
                enonce_nu
            )));
        }

        "par disjonction de cas" => {
            html.push_str(&phrase(&format!("Montrons {}.", enonce_rendu)));
            html.push_str(&phrase("Raisonnons par disjonction de cas."));
            let mut numero = 0usize;
            for p in &parts {
                match p {
                    Morceau::Etape { nom, titre, corps } if nom == "cas" => {
                        let complement = en_ligne(titre, env, toc);
                        html.push_str(&etiquette(&ordinal(numero), &complement));
                        html.push_str(&render_body_indent(corps, env, toc, false));
                        numero += 1;
                    }
                    _ => html.push_str(&rend(p, env, toc)),
                }
            }
            if numero == 0 {
                return Some(manque(tag_t, "cas …"));
            }
            html.push_str(&phrase(&format!("Dans tous les cas, {}.", enonce_nu)));
        }

        "par analyse-synthèse" => {
            let ana = match trouve("analyse") {
                Some(a) => a,
                None => return Some(manque(tag_t, "analyse")),
            };
            let syn = match trouve("synthèse") {
                Some(s) => s,
                None => return Some(manque(tag_t, "synthèse")),
            };
            html.push_str(&phrase(&format!("Montrons {}.", enonce_rendu)));
            html.push_str(&phrase("Raisonnons par analyse-synthèse."));
            html.push_str(&etiquette("Analyse", ""));
            html.push_str(&render_body_indent(&ana, env, toc, false));
            html.push_str(&etiquette("Synthèse", ""));
            html.push_str(&render_body_indent(&syn, env, toc, false));
        }

        "par double inclusion" => {
            let d = match trouve("directe") {
                Some(d) => d,
                None => return Some(manque(tag_t, "directe")),
            };
            let r = match trouve("réciproque") {
                Some(r) => r,
                None => return Some(manque(tag_t, "réciproque")),
            };
            html.push_str(&phrase(&format!(
                "Montrons par double inclusion {}.",
                enonce_rendu
            )));
            html.push_str(&etiquette("Première inclusion", ""));
            html.push_str(&render_body_indent(&d, env, toc, false));
            html.push_str(&etiquette("Seconde inclusion", ""));
            html.push_str(&render_body_indent(&r, env, toc, false));
            html.push_str(&phrase(&format!("Par double inclusion, {}.", enonce_nu)));
        }

        "par élément quelconque" => {
            let s = match trouve("soit") {
                Some(s) => s,
                None => return Some(manque(tag_t, "soit")),
            };
            let _ = &s;
            html.push_str(&phrase(&format!("Montrons que pour tout {}.", enonce_rendu)));
            for p in &parts {
                match p {
                    Morceau::Etape { nom, corps, .. } if nom == "soit" => {
                        html.push_str(&phrase(&format!("Soit {}.", en_ligne(corps, env, toc))));
                    }
                    _ => html.push_str(&rend(p, env, toc)),
                }
            }
            html.push_str(&phrase(&format!(
                "L'élément était quelconque : pour tout {}.",
                enonce_rendu
            )));
        }

        "par le principe des tiroirs" => {
            let o = match trouve("objets") {
                Some(o) => o,
                None => return Some(manque(tag_t, "objets")),
            };
            let t = match trouve("tiroirs") {
                Some(t) => t,
                None => return Some(manque(tag_t, "tiroirs")),
            };
            html.push_str(&phrase(&format!("Montrons {}.", enonce_rendu)));
            html.push_str(&phrase(&format!(
                "Appliquons le principe des tiroirs. Les objets sont {} ; les tiroirs sont {}.",
                en_ligne(&o, env, toc),
                en_ligne(&t, env, toc)
            )));
            for p in &parts {
                if !matches!(p, Morceau::Etape { nom, .. } if nom == "objets" || nom == "tiroirs") {
                    html.push_str(&rend(p, env, toc));
                }
            }
            let _ = ();
            html.push_str(&phrase(&format!(
                "Il y a plus d'objets que de tiroirs : par le principe des tiroirs, {}.",
                enonce_nu
            )));
        }

        _ if desc.starts_with("l'existence et l'unicité") && modif.is_empty() => unreachable!(),
        autre => {

            let _ = autre;
            return Some(crate::utils::erreur::bloc(
                &format!("<{}>", tag_t),
                "raisonnement inconnu — les formes admises sont : par contraposée, par l'absurde, par récurrence, par disjonction de cas, par analyse-synthèse, par double inclusion, par élément quelconque, par le principe des tiroirs",
            ));
        }
    }
    Some(html)
}

pub(crate) fn existence_unicite(
    desc: &str,
    corps: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
) -> Option<String> {
    let parts = morceaux(corps);
    let trouve = |nom: &str| {
        parts.iter().find_map(|p| match p {
            Morceau::Etape { nom: n, corps, .. } if n == nom => Some(corps.clone()),
            _ => None,
        })
    };
    let e = match trouve("existence") {
        Some(e) => e,
        None => return Some(manque("Montre", "existence")),
    };
    let u = match trouve("unicité") {
        Some(u) => u,
        None => return Some(manque("Montre", "unicité")),
    };
    let mut html = String::new();
    html.push_str(&phrase(&format!(
        "Montrons {}.",
        render_inline(desc.trim(), env, toc)
    )));
    html.push_str(&etiquette("Existence", ""));
    html.push_str(&render_body_indent(&e, env, toc, false));
    html.push_str(&etiquette("Unicité", ""));
    html.push_str(&render_body_indent(&u, env, toc, false));
    Some(html)
}

fn motif_somme(desc: &str) -> Option<(String, String, String, String)> {
    let i = desc.find("somme(")?;
    let interieur_debut = i + "somme(".len();
    let fin = interieur_debut + desc[interieur_debut..].find(')')?;
    let bornes = &desc[interieur_debut..fin];
    let (indice, depart) = bornes.split_once('=')?;
    let (depart, haut) = depart.split_once(';')?;
    if haut.trim() != "n" {
        return None;
    }
    let reste = desc[fin + 1..].trim();
    let fin_math = reste.find('$').unwrap_or(reste.len());
    let (terme, formule) = reste[..fin_math].split_once('=')?;
    Some((
        indice.trim().to_string(),
        depart.trim().to_string(),
        terme.trim().to_string(),
        formule.trim().to_string(),
    ))
}

pub(crate) fn recurrence_automatique(
    desc: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
) -> Option<String> {
    let (indice, depart, terme, formule) = motif_somme(desc)?;
    let req = serde_json::json!({
        "op": "recurrence_somme",
        "args": {"index": indice, "from": depart, "term": terme, "closed": formule},
    });
    let brut = match crate::python::pont::ask(&req.to_string()) {
        Ok(b) => b,
        Err(e) => {
            return Some(crate::utils::erreur::bloc(
                &format!("<Montre par récurrence>{}", desc),
                &e,
            ))
        }
    };
    let m: Vec<&str> = brut.trim().split('|').collect();
    if m.len() != 5 {
        return None;
    }
    let (init_g, init_d, suivant, formule, cible) = (m[0], m[1], m[2], m[3], m[4]);
    let enonce = render_inline(desc.trim(), env, toc);
    let mut html = String::new();
    html.push_str(&phrase(&format!("Montrons par récurrence {}.", enonce)));
    html.push_str(&etiquette("Initialisation", ""));
    html.push_str(&phrase(&format!(
        "Au rang \\({}\\), le membre de gauche vaut \\({}\\) et le membre de droite vaut \\({}\\) : la propriété est vraie au premier rang.",
        depart, init_g, init_d
    )));
    html.push_str(&etiquette("Hérédité", ""));
    html.push_str(&phrase(&format!(
        "Soit \\(n \\geqslant {}\\) un entier pour lequel la propriété est vraie. En ajoutant le terme suivant, puis en utilisant l'hypothèse de récurrence : \\[\\sum_{{{i}={d}}}^{{n+1}} {t} = \\left(\\sum_{{{i}={d}}}^{{n}} {t}\\right) + {s} = {f} + {s} = {c}\\] La propriété est vraie au rang \\(n+1\\).",
        depart,
        i = indice,
        d = depart,
        t = render_math_terme(&terme),
        s = suivant,
        f = formule,
        c = cible
    )));
    html.push_str(&phrase(
        "La propriété est vraie au premier rang et héréditaire : d'après le principe de récurrence, elle est vraie pour tout rang.",
    ));
    Some(html)
}

fn render_math_terme(terme: &str) -> String {
    crate::utils::notation::to_latex(terme)
}

pub(crate) fn pour_tout_automatique(
    desc: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
) -> Option<String> {
    let math = desc.split('$').nth(3).or_else(|| desc.split('$').nth(1))?;
    let (gauche, droite) = math.split_once(">=")?;
    let variable = desc
        .split('$')
        .nth(1)
        .map(|v| v.trim().to_string())
        .filter(|v| v.len() <= 2)
        .unwrap_or_else(|| "x".to_string());
    let req = serde_json::json!({
        "op": "toujours_positif",
        "args": {"var": variable, "expr": format!("({}) - ({})", gauche.trim(), droite.trim())},
    });
    let brut = match crate::python::pont::ask(&req.to_string()) {
        Ok(b) => b,
        Err(e) => {
            return Some(crate::utils::erreur::bloc(
                &format!("<Montre pour tout>{}", desc),
                &e,
            ))
        }
    };
    let m: Vec<&str> = brut.trim().split('|').collect();
    if m.len() != 3 {
        return None;
    }
    let (difference, canon, constante) = (m[0], m[1], m[2]);
    let enonce = render_inline(desc.trim(), env, toc);
    let g = render_math_terme(gauche.trim());
    let d = render_math_terme(droite.trim());
    let mut html = String::new();
    html.push_str(&phrase(&format!("Montrons que pour tout {}.", enonce)));
    html.push_str(&phrase(&format!(
        "Soit \\({}\\) un réel quelconque. Étudions la différence : \\[{} - \\left({}\\right) = {} = {}\\]",
        variable, g, d, difference, canon
    )));
    let argument = if constante == "0" {
        "Un carré est positif ou nul".to_string()
    } else {
        format!(
            "Un carré est positif ou nul, et la constante \\({}\\) l'est aussi",
            constante
        )
    };
    html.push_str(&phrase(&format!(
        "{} : la différence est positive ou nulle, donc \\({} \\geqslant {}\\).",
        argument, g, d
    )));
    html.push_str(&phrase(&format!(
        "L'élément était quelconque : pour tout {}.",
        enonce
    )));
    Some(html)
}

const BASE: &str = include_str!("../maths/demonstrations.json");

fn normalise(t: &str) -> String {
    let sans_math: String = t.replace('$', " ").replace('\\', " ");
    let plie: String = sans_math
        .chars()
        .map(crate::utils::texte::pli)
        .collect::<String>()
        .to_lowercase();
    let mots: Vec<&str> = plie
        .split(|c: char| !c.is_alphanumeric() && c != '^' && c != '(' && c != ')')
        .filter(|m| !m.is_empty())
        .collect();
    let mut sortie = mots.join(" ");
    for prefixe in ["que ", "qu ", "l ", "le ", "la ", "les "] {
        if let Some(reste) = sortie.strip_prefix(prefixe) {
            sortie = reste.to_string();
            break;
        }
    }
    sortie
}

pub(crate) struct Fiche {
    pub raisonnement: String,
    pub corps: String,
    pub enonce: String,
}

struct FicheBase {
    cles: Vec<String>,
    enonce_normalise: String,
    raisonnement: String,
    corps: String,
    enonce: String,
}

fn fiches() -> &'static [FicheBase] {
    static FICHES: std::sync::OnceLock<Vec<FicheBase>> = std::sync::OnceLock::new();
    FICHES.get_or_init(|| {
        let base: serde_json::Value =
            serde_json::from_str(BASE).expect("demonstrations.json embarqué est invalide");
        base.get("fiches")
            .and_then(|f| f.as_array())
            .map(|liste| {
                liste
                    .iter()
                    .map(|fiche| {
                        let texte = |champ: &str| {
                            fiche
                                .get(champ)
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string()
                        };
                        let enonce = texte("énoncé");
                        FicheBase {
                            cles: fiche
                                .get("clés")
                                .and_then(|c| c.as_array())
                                .map(|cles| {
                                    cles.iter()
                                        .filter_map(|c| c.as_str())
                                        .map(normalise)
                                        .collect()
                                })
                                .unwrap_or_default(),
                            enonce_normalise: normalise(&enonce),
                            raisonnement: texte("raisonnement"),
                            corps: texte("corps"),
                            enonce,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    })
}

pub(crate) fn cherche(desc: &str) -> Option<Fiche> {
    let vise = normalise(desc);
    if vise.is_empty() {
        return None;
    }
    for fiche in fiches() {
        for cle in &fiche.cles {
            if *cle == vise || (cle.len() > 12 && vise.contains(cle.as_str())) {
                return Some(Fiche {
                    raisonnement: fiche.raisonnement.clone(),
                    corps: fiche.corps.clone(),
                    enonce: fiche.enonce.clone(),
                });
            }
        }
    }
    None
}

pub(crate) fn suggestions(desc: &str) -> Vec<String> {
    let vise = normalise(desc);
    let mots: Vec<&str> = vise.split(' ').filter(|m| m.len() > 3).collect();
    let mut notes: Vec<(usize, &str)> = Vec::new();
    for fiche in fiches() {
        let score = mots
            .iter()
            .filter(|m| fiche.enonce_normalise.contains(**m))
            .count();
        if score > 0 {
            notes.push((score, fiche.enonce.as_str()));
        }
    }
    notes.sort_by(|a, b| b.0.cmp(&a.0));
    notes
        .into_iter()
        .take(3)
        .map(|(_, e)| e.to_string())
        .collect()
}

pub(crate) fn depuis_bibliotheque(
    desc: &str,
    modificateur: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
) -> Option<String> {
    let fiche = cherche(desc)?;

    if let Some(message) = desaccord(modificateur, &fiche.raisonnement) {
        return Some(crate::utils::erreur::bloc(
            &format!("<Montre>{} {}", modificateur, desc.trim()),
            &message,
        ));
    }
    let raisonnement = fiche.raisonnement.clone();
    if raisonnement == "existence et unicité" {
        return existence_unicite(&fiche.enonce, &fiche.corps, env, toc);
    }
    let tag = if raisonnement.is_empty() {
        "Montre".to_string()
    } else {
        format!("Montre {}", raisonnement)
    };
    montre(&tag, &fiche.enonce, &fiche.corps, env, toc)
}

pub(crate) fn separe_raisonnement(complement: &str) -> (String, String) {
    let t = complement.trim_start();

    for forme in [
        "par le principe des tiroirs",
        "par disjonction de cas",
        "par analyse-synthèse",
        "par double inclusion",
        "par élément quelconque",
        "par contraposée",
        "par récurrence",
        "par l'absurde",
    ] {
        for variante in [forme.to_string(), forme.replace('\'', "\u{2019}")] {

            let n = variante.chars().count();
            let debut: String = t.chars().take(n).collect::<String>().to_lowercase();
            if debut == variante {
                let reste: String = t.chars().skip(n).collect();
                return (forme.to_string(), reste.trim_start().to_string());
            }
        }
    }
    (String::new(), t.to_string())
}

pub(crate) fn desaccord(demande: &str, fiche: &str) -> Option<String> {
    if demande.is_empty() || demande == fiche {
        return None;
    }
    let attendu = if fiche.is_empty() {
        "directement".to_string()
    } else {
        fiche.to_string()
    };
    Some(format!(
        "cet énoncé se démontre {} — pour le démontrer autrement, écrivez le corps entre accolades",
        attendu
    ))
}

#[cfg(test)]
mod base {

    #[test]
    fn la_base_embarquee_se_charge() {
        let fiches = super::fiches();
        assert!(fiches.len() >= 100, "la base compte {} fiches", fiches.len());
        for fiche in fiches {
            assert!(!fiche.cles.is_empty(), "fiche sans clé : {}", fiche.enonce);
        }
    }
}
