//! Les environnements numérotés : théorème, proposition, propriété, lemme,
//! corollaire, définition, exemple, remarque.
//!
//! `<Énonce>le théorème de Pythagore <étiquette>{pythagore} { … }` pose
//! « **Théorème 3.2 (Pythagore).** » et déroule son corps à la suite, en
//! italique pour les énoncés, en romain pour les définitions, exemples et
//! remarques — la convention des livres. Chaque genre tient son compteur,
//! remis à zéro au chapitre ; le numéro se préfixe du chapitre quand il y en
//! a un, comme les sections. L'étiquette et le renvoi sont ceux du document
//! long : `voir le théorème <renvoi>{pythagore}` se remplace par le numéro,
//! cliquable. Le titre s'habille comme un titre de section :
//! `soit théorème = <bleu nuit petites capitales>`.
//!
//! **L'énoncé est une citation** : ce qui s'y déclare y demeure. Le corps se
//! rend sur une copie de l'environnement, si bien qu'un `<Soit>` posé dans un
//! théorème n'existe plus après lui — c'est aussi ce qui garantit que
//! `scan_env` prédit le rendu sans avoir à lire le corps.
//!
//! La preuve loge dans l'énoncé : un sous-bloc `démonstration { … }` se
//! compose en vedette — *Démonstration.* — et se referme d'un tombeau. Avec
//! un raisonnement, il appelle la machinerie de `<Montre>` : `démonstration
//! par récurrence que … { initialisation{…} hérédité{…} }` déroule le
//! squelette complet. Sans énoncé restitué, la propriété à démontrer est
//! l'énoncé lui-même, s'il tient en une phrase.

use crate::utils::texte::meme_mot;
use crate::{Def, Env, TocEntry};

/// Présent dans le HTML dès qu'un environnement est posé — c'est la classe
/// ouvrante, guillemet compris, pour qu'une prose contenant le mot
/// « environnement » ne déclenche pas l'injection de la feuille de style.
pub(crate) const CLASSE: &str = "class=\"environnement ";

struct Genre {
    /// Le nom tel qu'on l'énonce, et la clé du style configurable.
    cle: &'static str,
    /// Le radical ASCII des identifiants d'ancre.
    ancre: &'static str,
    /// Le titre imprimé.
    titre: &'static str,
    /// Les énoncés se composent en italique, les discours en romain.
    italique: bool,
}

const GENRES: &[Genre] = &[
    Genre { cle: "théorème", ancre: "theoreme", titre: "Théorème", italique: true },
    Genre { cle: "proposition", ancre: "proposition", titre: "Proposition", italique: true },
    Genre { cle: "propriété", ancre: "propriete", titre: "Propriété", italique: true },
    Genre { cle: "lemme", ancre: "lemme", titre: "Lemme", italique: true },
    Genre { cle: "corollaire", ancre: "corollaire", titre: "Corollaire", italique: true },
    Genre { cle: "axiome", ancre: "axiome", titre: "Axiome", italique: true },
    Genre { cle: "conjecture", ancre: "conjecture", titre: "Conjecture", italique: true },
    Genre { cle: "définition", ancre: "definition", titre: "Définition", italique: false },
    Genre { cle: "exemple", ancre: "exemple", titre: "Exemple", italique: false },
    Genre { cle: "remarque", ancre: "remarque", titre: "Remarque", italique: false },
];

fn genres_en_toutes_lettres() -> String {
    let noms: Vec<&str> = GENRES.iter().map(|g| g.cle).collect();
    let (tete, dernier) = noms.split_at(noms.len() - 1);
    format!("{} ou {}", tete.join(", "), dernier[0])
}

struct Enonce<'a> {
    genre: &'static Genre,
    /// Le nom propre de l'énoncé, articles de liaison retirés : « Pythagore »,
    /// « valeurs intermédiaires ». Vide si l'énoncé n'en a pas.
    nom: String,
    etiquette: Option<String>,
    corps: &'a str,
}

/// Détache une éventuelle `<étiquette>{nom}` de la tête, et la retourne.
fn detache_etiquette(tete: &str) -> (String, Option<String>) {
    let octets = tete.as_bytes();
    let mut i = 0;
    while i < octets.len() {
        if octets[i] == b'<' {
            if let Some(fin) = tete[i + 1..].find('>') {
                let mot = &tete[i + 1..i + 1 + fin];
                if meme_mot(mot, "étiquette") {
                    let apres = tete[i + 1 + fin + 1..].trim_start();
                    if let Some(reste) = apres.strip_prefix('{') {
                        if let Some(ferme) = reste.find('}') {
                            let nom = reste[..ferme].trim().to_string();
                            let consomme = tete.len() - apres.len() + 1 + ferme + 1;
                            let mut nettoye = tete[..i].to_string();
                            nettoye.push_str(&tete[consomme..]);
                            let nom = if nom.is_empty() { None } else { Some(nom) };
                            return (nettoye, nom);
                        }
                    }
                }
                i += 1 + fin + 1;
                continue;
            }
        }
        i += 1;
    }
    (tete.to_string(), None)
}

/// Retire la liaison en tête du nom : « de Pythagore » devient « Pythagore »,
/// « des valeurs intermédiaires » devient « valeurs intermédiaires »,
/// « de l'angle inscrit » devient « angle inscrit ».
fn depouille_liaison(nom: &str) -> String {
    let mut reste = nom.trim();
    for liaison in ["de ", "du ", "des ", "d'", "d’"] {
        if let Some(r) = reste.strip_prefix(liaison) {
            reste = r.trim_start();
            break;
        }
    }
    for article in ["la ", "le ", "les ", "l'", "l’"] {
        if let Some(r) = reste.strip_prefix(article) {
            reste = r.trim_start();
            break;
        }
    }
    reste.to_string()
}

fn analyse(apres: &str) -> Result<Enonce<'_>, String> {
    let (tete, corps) = match crate::layout::rendu::find_body_brace(apres) {
        Some((t, c)) => (t, c),
        None => {
            return Err("il manque le corps { … } de l'énoncé".into());
        }
    };
    let (tete, etiquette) = detache_etiquette(&tete);
    let tete = tete.trim();
    let (article_pris, apres_article) = ["le ", "la ", "l'", "l’", "un ", "une "]
        .iter()
        .find_map(|a| tete.strip_prefix(a).map(|r| (true, r.trim_start())))
        .unwrap_or((false, tete));
    if !article_pris {
        return Err(
            "l'énoncé se dit avec son article : « le théorème », « la définition »…".into(),
        );
    }
    let (mot, nom) = match apres_article.split_once(char::is_whitespace) {
        Some((m, reste)) => (m, reste),
        None => (apres_article, ""),
    };
    let genre = GENRES.iter().find(|g| meme_mot(g.cle, mot));
    let Some(genre) = genre else {
        return Err(format!(
            "« {} » n'est pas un environnement connu — {}",
            mot,
            genres_en_toutes_lettres()
        ));
    };
    // Le corps vit dans la même chaîne que la tête : on le retrouve dans
    // `apres` pour rendre une tranche empruntée plutôt qu'une copie.
    let fin = apres.trim_end();
    let corps_debut = fin.len() - 1 - corps.len();
    Ok(Enonce {
        genre,
        nom: depouille_liaison(nom),
        etiquette,
        corps: &fin[corps_debut..corps_debut + corps.len()],
    })
}

/// Le numéro et l'ancre du prochain énoncé de ce genre — et le compteur
/// avance. Le préfixe de chapitre suit la règle des sections.
fn avance(env: &mut Env, genre: &Genre) -> (String, String) {
    let n = env.environnements.entry(genre.cle.to_string()).or_insert(0);
    *n += 1;
    let num = if env.chapitre > 0 {
        format!("{}.{}", env.chapitre, n)
    } else {
        n.to_string()
    };
    let ancre = format!("{}-{}", genre.ancre, num.replace('.', "-"));
    (num, ancre)
}

/// Détache les sous-blocs `démonstration … { … }` du corps : l'énoncé d'un
/// côté, les preuves de l'autre, dans l'ordre où elles se présentent.
fn separe_demonstrations(corps: &str) -> (String, Vec<(String, String)>) {
    let mut enonce = String::new();
    let mut preuves: Vec<(String, String)> = Vec::new();
    let mut courant: Option<(String, String)> = None;
    let mut profondeur = 0i32;
    for ligne in corps.lines() {
        match courant.as_mut() {
            None => {
                let t = ligne.trim();
                let mot = t
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches('{');
                if profondeur == 0 && t.ends_with('{') && !mot.is_empty() && meme_mot(mot, "démonstration")
                {
                    let tete = t[mot.len()..t.len() - 1].trim().to_string();
                    courant = Some((tete, String::new()));
                    profondeur = 1;
                } else {
                    crate::utils::texte::maj_profondeur(ligne, &mut profondeur);
                    enonce.push_str(ligne);
                    enonce.push('\n');
                }
            }
            Some((_, corps_preuve)) => {
                crate::utils::texte::maj_profondeur(ligne, &mut profondeur);
                if profondeur == 0 {
                    let t = ligne.trim_end();
                    let avant = t.strip_suffix('}').unwrap_or(t);
                    if !avant.trim().is_empty() {
                        corps_preuve.push_str(avant);
                        corps_preuve.push('\n');
                    }
                    let (tete, corps_preuve) = courant.take().unwrap();
                    preuves.push((tete, corps_preuve));
                } else {
                    corps_preuve.push_str(ligne);
                    corps_preuve.push('\n');
                }
            }
        }
    }
    if let Some((tete, corps_preuve)) = courant {
        preuves.push((tete, corps_preuve));
    }
    (enonce, preuves)
}

/// La propriété à démontrer, tirée de l'énoncé lui-même : la phrase, point
/// final déposé, première lettre pliée, précédée de « que ». Un énoncé de
/// plusieurs phrases ne se laisse pas deviner — il se restitue.
fn reclame(enonce: &str) -> Result<String, String> {
    let t = enonce.trim();
    let t = t.strip_suffix('.').unwrap_or(t).trim_end();
    if t.is_empty() {
        return Err("l'énoncé est vide : rien à démontrer".into());
    }
    if t.contains('.') || t.contains('\n') {
        return Err(
            "l'énoncé compte plusieurs phrases : dites laquelle se démontre — « démonstration par récurrence que … »"
                .into(),
        );
    }
    let mut lettres = t.chars();
    let premiere = lettres.next().unwrap();
    let mut plie: String = premiere.to_lowercase().collect();
    plie.push_str(lettres.as_str());
    Ok(format!("que {}", plie))
}

/// La preuve en vedette — *Démonstration.* — refermée d'un tombeau. Avec un
/// raisonnement, c'est la machinerie de `<Montre>` qui déroule le squelette.
fn demonstration_html(
    tete: &str,
    corps: &str,
    enonce: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
) -> String {
    let (raisonnement, reste) = crate::layout::demonstration::separe_raisonnement(tete);
    let reste_t = reste.trim();
    let restitue = reste_t.starts_with("que ")
        || reste_t.starts_with("qu'")
        || reste_t.starts_with("qu’");
    if raisonnement.is_empty() && !reste_t.is_empty() && !restitue {
        // Une forme de raisonnement que la machinerie ne connaît pas : c'est
        // elle qui dresse la liste des formes admises, une seule fois.
        return crate::layout::demonstration::montre(
            &format!("Montre {}", reste_t),
            "",
            corps,
            env,
            toc,
        )
        .unwrap_or_else(|| {
            crate::utils::erreur::bloc(
                &format!("démonstration {}", tete),
                "cette démonstration n'a pas pu se rédiger",
            )
        });
    }
    let interne = if raisonnement.is_empty() && reste_t.is_empty() {
        crate::layout::rendu::render_body(corps, env, toc)
    } else {
        let propriete = if reste_t.is_empty() {
            match reclame(enonce) {
                Ok(p) => p,
                Err(message) => {
                    return crate::utils::erreur::bloc(
                        &format!("démonstration {}", tete),
                        &message,
                    );
                }
            }
        } else {
            reste_t.to_string()
        };
        let tag = if raisonnement.is_empty() {
            "Montre".to_string()
        } else {
            format!("Montre {}", raisonnement)
        };
        match crate::layout::demonstration::montre(&tag, &propriete, corps, env, toc) {
            Some(h) => h,
            None => crate::utils::erreur::bloc(
                &format!("démonstration {}", tete),
                "cette démonstration n'a pas pu se rédiger",
            ),
        }
    };
    let titre = "<span class=\"env-demo-titre\">Démonstration.</span>";
    let mut bloc = String::from("<div class=\"env-demonstration\">");
    if let Some(suite) = interne.strip_prefix("<p>") {
        bloc.push_str("<p>");
        bloc.push_str(titre);
        bloc.push(' ');
        bloc.push_str(suite);
    } else {
        bloc.push_str("<p>");
        bloc.push_str(titre);
        bloc.push_str("</p>");
        bloc.push_str(&interne);
    }
    if bloc.ends_with("</p>") {
        bloc.truncate(bloc.len() - "</p>".len());
        bloc.push_str("<span class=\"env-tombeau\">∎</span></p>");
    } else {
        bloc.push_str("<p><span class=\"env-tombeau\">∎</span></p>");
    }
    bloc.push_str("</div>");
    bloc
}

/// Le miroir de `rend` pour `scan_env` : mêmes conditions, même compteur,
/// pas de HTML. C'est le contrat des tests d'invariants du cache.
pub fn scan(apres: &str, env: &mut Env) {
    if let Ok(enonce) = analyse(apres) {
        avance(env, enonce.genre);
    }
}

pub fn rend(apres: &str, env: &mut Env, toc: &mut Vec<TocEntry>) -> String {
    let enonce = match analyse(apres) {
        Ok(e) => e,
        Err(message) => {
            let tete = apres.lines().next().unwrap_or("").trim_end();
            return crate::utils::erreur::bloc(&format!("<Énonce>{}", tete), &message);
        }
    };
    let (num, ancre) = avance(env, enonce.genre);
    // L'énoncé est une citation : son corps se rend sur une copie de
    // l'environnement, et rien n'en sort — le miroir exact de `scan`, qui
    // n'y entre pas.
    let mut interieur = env.clone();

    let style = match env.defs.get(enonce.genre.cle) {
        Some(Def::Style(mots)) => crate::layout::rendu::style_css(mots).0,
        _ => String::new(),
    };
    let attribut_style = if style.is_empty() {
        String::new()
    } else {
        format!(" style=\"{}\"", style)
    };
    let nom = if enonce.nom.is_empty() {
        String::new()
    } else {
        format!(
            " <span class=\"env-nom\">({})</span>",
            crate::layout::rendu::render_inline(&enonce.nom, &mut interieur, toc)
        )
    };
    let titre = format!(
        "<span class=\"env-titre\"{}>{} {}{}.</span>",
        attribut_style, enonce.genre.titre, num, nom
    );

    let (texte_enonce, preuves) = separe_demonstrations(enonce.corps);

    let mut html = format!(
        "<div class=\"environnement env-{}{}\" id=\"{}\">",
        enonce.genre.ancre,
        if enonce.genre.italique { " env-italique" } else { "" },
        ancre
    );
    if let Some(etiquette) = &enonce.etiquette {
        html.push_str(&crate::layout::rendu::declare_renvoi(etiquette, &num, &ancre));
    }
    let corps = crate::layout::rendu::render_body(&texte_enonce, &mut interieur, toc);
    // Le titre entre en vedette dans le premier paragraphe, à la façon des
    // livres ; si le corps ouvre sur autre chose qu'un paragraphe nu, le
    // titre prend sa ligne.
    if let Some(reste) = corps.strip_prefix("<p>") {
        html.push_str("<p>");
        html.push_str(&titre);
        html.push(' ');
        html.push_str(reste);
    } else {
        html.push_str("<p>");
        html.push_str(&titre);
        html.push_str("</p>");
        html.push_str(&corps);
    }
    for (tete, corps_preuve) in &preuves {
        html.push_str(&demonstration_html(
            tete,
            corps_preuve,
            &texte_enonce,
            &mut interieur,
            toc,
        ));
    }
    html.push_str("</div>");
    html
}

pub fn feuille_de_style() -> String {
    // L'ordre des règles compte : la preuve revient au romain *après* que
    // l'italique de l'énoncé s'est posé — même spécificité, la dernière
    // l'emporte.
    "<style>\
.environnement{margin:0.9em 0}\
.environnement .env-titre{font-weight:700;font-style:normal}\
.environnement .env-nom{font-weight:400}\
.environnement.env-italique p{font-style:italic}\
.environnement .env-demonstration p{font-style:normal}\
.environnement .env-demo-titre{font-style:italic}\
.environnement .env-tombeau{float:right}\
</style>"
        .to_string()
}
