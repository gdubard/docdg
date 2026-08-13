//! Les squelettes rhétoriques des démonstrations.
//!
//! Chaque type de raisonnement du supérieur possède une charpente attendue :
//! une phrase qui l'annonce, des étapes nommées, une phrase qui le referme.
//! Cette charpente n'est pas décorative — un raisonnement correct mais non
//! annoncé peut être compté faux : une analyse-synthèse commence par supposer
//! la conclusion, et le correcteur pressé qui ne lit pas l'annonce y voit une
//! pétition de principe. Le moteur fournit donc la charpente ; l'auteur
//! n'écrit que les mathématiques.
//!
//! Le verbe est « Montre » et lui seul : dans le corpus dépouillé,
//! « Montrons que » (425 occurrences) écrase « Démontrons que » (3). Le
//! raisonnement direct est la forme nue ; les neuf autres se nomment après le
//! verbe, d'un seul nom chacun — une notion, un mot :
//!
//! ```text
//! <Montre>que ...                            (directe)
//! <Montre par contraposée>que ...
//! <Montre par l'absurde>que ...
//! <Montre par récurrence>que ...
//! <Montre par disjonction de cas>que ...
//! <Montre par analyse-synthèse>que ...
//! <Montre par double inclusion>que ...
//! <Montre>par élément quelconque ...
//! <Montre par le principe des tiroirs>que ...
//! <Montre l'existence et l'unicité>de ...
//! ```
//!
//! Les étapes s'écrivent comme tous les blocs du langage — le nom, l'accolade
//! en fin de ligne, la fermante seule sur la sienne :
//!
//! ```text
//! <Montre par récurrence>que pour tout entier $n$, ... {
//!     initialisation{
//!         ...
//!     }
//!     hérédité{
//!         ...
//!     }
//! }
//! ```
//!
//! Les clôtures sont dosées d'après le corpus : les formules d'achèvement en
//! sont quasi absentes (« ce qui achève la démonstration » : 12 occurrences
//! sur 2,1 millions de mots), le moteur n'en sème donc pas. Ne sont refermés
//! que les raisonnements dont la conclusion **fait partie de la logique** :
//! la récurrence invoque son principe, la contraposée revient à l'énoncé
//! direct, l'absurde conclut de la contradiction, la disjonction constate que
//! tous les cas sont couverts, la double inclusion assemble ses deux moitiés.

use super::rendu::{render_body_indent, render_inline};
use crate::{Env, TocEntry};

/// Une étape nommée du corps, ou du texte libre entre deux étapes.
enum Morceau {
    Etape { nom: String, titre: String, corps: String },
    Libre(String),
}

/// Le nom d'étape en tête de ligne, s'il ouvre un bloc — la ligne se termine
/// par l'accolade ouvrante, comme tous les blocs du langage.
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

/// Découpe le corps en étapes nommées et texte libre, dans l'ordre d'écriture.
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

/// L'énoncé sans son « que » introducteur, prêt à être repris dans une
/// clôture : « Donc √2 est irrationnel. »
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

/// Le point d'entrée : `<Montre …>énoncé {corps}`.
///
/// La démonstration rend une **suite de paragraphes**, sans conteneur qui les
/// enveloppe. Un conteneur en ferait un bloc unique aux yeux de la mise en
/// page, qui ne scinde que ce qu'elle sait scinder : une démonstration un peu
/// longue basculerait tout entière à la page suivante en laissant un blanc
/// derrière elle. Sans lui, chaque paragraphe se pose où il tient.
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
        // ————— la démonstration directe : l'annonce, puis les déductions.
        "" => {
            html.push_str(&phrase(&format!("Montrons {}.", enonce_rendu)));
            for p in &parts {
                html.push_str(&rend(p, env, toc));
            }
        }
        // ————— la contraposée s'annonce, s'énonce, et revient à l'implication
        // de départ — sans quoi le lecteur ne sait pas ce qui vient d'être
        // démontré.
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
        // ————— l'absurde suppose la négation, la nomme, et conclut de la
        // contradiction.
        "par l'absurde" => {
            let h = match trouve("absurde") {
                Some(h) => h,
                None => return Some(manque(tag_t, "absurde")),
            };
            let _ = &h;
            html.push_str(&phrase(&format!("Montrons {}.", enonce_rendu)));
            // Les étapes s'annoncent **à leur place** : ce qui est écrit avant
            // l'hypothèse absurde — poser les objets, nommer la suite — doit
            // se lire avant elle, non après.
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
        // ————— la récurrence : initialisation, hérédité, et une conclusion
        // qui invoque le principe — l'oublier coûte des points.
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
        // ————— la disjonction énumère ses cas et constate qu'ils couvrent
        // tout.
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
        // ————— l'analyse-synthèse DOIT s'annoncer : l'analyse suppose la
        // conclusion vraie, et sans l'annonce, un correcteur pressé y lit une
        // pétition de principe.
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
        // ————— la double inclusion prouve l'égalité de deux ensembles moitié
        // par moitié, et l'assemble à la fin.
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
        // ————— la propriété universelle : on fixe un élément quelconque —
        // l'ouverture « Soit… » est systématique — et le choix quelconque
        // fonde la généralisation.
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
        // ————— les tiroirs : nommer les objets, nommer les tiroirs, compter.
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
        // ————— l'existence et l'unicité se traitent séparément : construire,
        // puis confondre deux candidats.
        _ if desc.starts_with("l'existence et l'unicité") && modif.is_empty() => unreachable!(),
        autre => {
            // L'existence et l'unicité se demandent par le complément :
            // `<Montre>l'existence et l'unicité de …`.
            let _ = autre;
            return Some(crate::utils::erreur::bloc(
                &format!("<{}>", tag_t),
                "raisonnement inconnu — les formes admises sont : par contraposée, par l'absurde, par récurrence, par disjonction de cas, par analyse-synthèse, par double inclusion, par élément quelconque, par le principe des tiroirs",
            ));
        }
    }
    Some(html)
}

/// L'existence et l'unicité, demandées par le complément du verbe nu :
/// `<Montre>l'existence et l'unicité de …`.
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

// ═══════════ les démonstrations que le moteur écrit lui-même ═══════════
//
// Ailleurs dans le langage, l'absence de corps signifie « le moteur fait le
// travail » : `<Calcule>la dérivée de f` n'attend pas qu'on la lui donne. Les
// démonstrations suivent la même règle. Quand le raisonnement est à la portée
// du calcul formel, `<Montre par récurrence>que …` sans accolades suffit : le
// moteur vérifie chaque étape par le calcul, puis rédige — et si la formule
// est fausse, il refuse au lieu de démontrer un mensonge.

/// `somme(k=a;n) terme = formule` extrait du complément.
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

/// La récurrence d'une formule sommatoire, démontrée par le moteur.
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

/// Le terme sous le signe somme, rendu en LaTeX par le mini-langage.
fn render_math_terme(terme: &str) -> String {
    crate::utils::notation::to_latex(terme)
}

/// `réel x, A >= B` : la positivité de la différence, lue sur la forme
/// canonique.
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

// ═══════════ la bibliothèque des démonstrations classiques ═══════════
//
// Tout ne se calcule pas. L'irrationalité de √2, l'infinité des nombres
// premiers, l'unicité de la limite : ces démonstrations reposent sur une idée,
// pas sur un calcul, et aucun système de calcul formel ne les trouvera. Elles
// sont donc écrites une fois, en docdg, et rangées dans une base.
//
// Le corps stocké traverse **la même charpente** que celui qu'un auteur
// écrirait à la main : la bibliothèque ne fournit pas du HTML tout fait, elle
// fournit les mathématiques, et la rédaction reste celle du moteur. Une fiche
// corrigée profite donc aussitôt à toutes les formes de rendu.
//
// La base est incorporée au binaire : docdg travaille hors ligne, en classe,
// sans dépendance ni compte à créer.

const BASE: &str = include_str!("../maths/demonstrations.json");

/// La forme sous laquelle un énoncé se compare : minuscules, accents pliés,
/// mathématiques et ponctuation retirées, espaces réduits. « Que $racine(2)$
/// est irrationnel. » et « racine de 2 est irrationnel » se rejoignent.
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

/// Une fiche trouvée : le raisonnement à employer et le corps à rendre.
pub(crate) struct Fiche {
    pub raisonnement: String,
    pub corps: String,
    pub enonce: String,
}

/// La base livrée, une fois pour toutes.
///
/// Elle est incorporée au binaire et suit les versions : une fiche corrigée
/// arrive avec la mise à jour, et il n'existe pas deux sources qui puissent
/// diverger.
fn fiches() -> Option<Vec<serde_json::Value>> {
    let base: serde_json::Value = serde_json::from_str(BASE).ok()?;
    Some(base.get("fiches")?.as_array()?.clone())
}

/// Cherche l'énoncé dans la bibliothèque.
pub(crate) fn cherche(desc: &str) -> Option<Fiche> {
    let vise = normalise(desc);
    if vise.is_empty() {
        return None;
    }
    for fiche in fiches()? {
        let cles = fiche.get("clés")?.as_array()?;
        for cle in cles {
            let cle = normalise(cle.as_str().unwrap_or(""));
            if cle == vise || (cle.len() > 12 && vise.contains(&cle)) {
                return Some(Fiche {
                    raisonnement: fiche
                        .get("raisonnement")
                        .and_then(|r| r.as_str())
                        .unwrap_or("")
                        .to_string(),
                    corps: fiche
                        .get("corps")
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string(),
                    enonce: fiche
                        .get("énoncé")
                        .and_then(|e| e.as_str())
                        .unwrap_or("")
                        .to_string(),
                });
            }
        }
    }
    None
}

/// Les énoncés les plus proches, pour que l'échec enseigne quelque chose :
/// une démonstration absente de la base doit dire ce qu'elle contient.
pub(crate) fn suggestions(desc: &str) -> Vec<String> {
    let vise = normalise(desc);
    let mots: Vec<&str> = vise.split(' ').filter(|m| m.len() > 3).collect();
    let mut notes: Vec<(usize, String)> = Vec::new();
    if let Some(fiches) = fiches() {
        for fiche in fiches {
            let enonce = fiche
                .get("énoncé")
                .and_then(|e| e.as_str())
                .unwrap_or("")
                .to_string();
            let cible = normalise(&enonce);
            let score = mots.iter().filter(|m| cible.contains(**m)).count();
            if score > 0 {
                notes.push((score, enonce));
            }
        }
    }
    notes.sort_by(|a, b| b.0.cmp(&a.0));
    notes.into_iter().take(3).map(|(_, e)| e).collect()
}

/// Le point d'entrée : un énoncé sans corps, cherché dans la bibliothèque puis
/// rendu par la charpente ordinaire.
pub(crate) fn depuis_bibliotheque(
    desc: &str,
    modificateur: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
) -> Option<String> {
    let fiche = cherche(desc)?;
    // Le raisonnement vient de la fiche, sauf si l'auteur en impose un — il a
    // le dernier mot sur la façon dont il veut voir la chose démontrée.
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

/// Le raisonnement se lit **en tête du complément**, non dans la balise.
///
/// La règle d'or du langage veut le verbe seul entre chevrons et le reste en
/// français complet : `<Montre>par récurrence que …`, comme `<Résous>le
/// système s`. C'est aussi la phrase française — « Montre par récurrence
/// que P » — lue dans l'ordre où elle s'écrit. L'ancienne écriture
/// `<Montre par récurrence>` reste comprise, comme toutes les anciennes
/// écritures du langage, mais elle ne s'enseigne plus.
pub(crate) fn separe_raisonnement(complement: &str) -> (String, String) {
    let t = complement.trim_start();
    // Les plus longs d'abord : « par récurrence forte » avant « par récurrence ».
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
            // Comparaison par caractères, non par octets : « par élément »
            // porte des accents, et découper un octet au milieu d'un « é »
            // ferait paniquer le moteur.
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

/// Le raisonnement annoncé par l'auteur contredit-il celui de la fiche ?
///
/// Silencieusement passer outre produirait une démonstration amputée — le
/// corps d'une récurrence n'a pas les étapes d'une preuve par l'absurde. Mieux
/// vaut le dire.
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
