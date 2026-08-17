//! La frise chronologique — l'histoire entre dans docdg.
//!
//! Une vraie frise, non une ligne du temps : un **bandeau** gradué que
//! referme une grande pointe, les **périodes** à l'intérieur, les
//! **événements** à l'extérieur en cartouches reliés à leur date.
//!
//! ```text
//! <Construis>la frise chronologique du premier XXe siècle {
//! de 1914 à 1918 : Grande Guerre (Le premier conflit mondial.)
//! 1936 - 1938 : Front populaire
//! 11/11/1918 : Armistice
//! 06/02/1934 : Crise du 6 février
//! }
//! ```
//!
//! Une ligne par événement : la date, deux-points, le titre — et, s'il
//! éclaire, le détail entre parenthèses en fin de ligne. La date se lit en
//! `AAAA`, `MM/AAAA` ou `JJ/MM/AAAA`, l'année négative est admise
//! (l'Antiquité s'écrit `-52 : Alésia`) et « vers » dit l'incertitude des
//! sources sans gêner le placement ; elle s'imprime toujours telle qu'elle
//! s'est écrite. Ce qui suit « la frise chronologique » dans la description
//! devient la légende.
//!
//! **Une période est un événement qui dure** : ses deux dates s'écrivent
//! `de 1914 à 1918`, `1789 - 1799`, `1914 -- 1918` ou `1914 — 1918`, au
//! choix — le tiret simple demande seulement de respirer, pour ne pas se
//! confondre avec le signe d'une année négative. Elle occupe l'intérieur du
//! bandeau, avec son nom, ses bornes et sa description si la place le
//! permet. Les périodes qui se suivent se touchent sans se gêner, celles
//! qui se chevauchent se partagent la hauteur du bandeau, et celles qui
//! sont trop étroites pour se nommer dedans se nomment dans un cartouche.
//!
//! Les cartouches se rangent dans l'ordre du temps et **ne se chevauchent
//! jamais** : ils se répartissent de part et d'autre du bandeau, en
//! alternance le long du temps, et s'étagent sur autant de rangées qu'il
//! faut de chaque côté, un trait de rappel reliant chacun à sa date. Rien
//! n'est jamais barré : les traits se tracent tous avant les cartouches,
//! qui sont opaques, et les verticales de la graduation s'interrompent
//! devant chaque cartouche comme devant chaque bandeau.
//!
//! **La frise multilinéaire** range ses événements par bandes nommées, qui
//! partagent la même échelle du temps — ce qui met la simultanéité sous les
//! yeux, et permet de confronter deux découpages concurrents :
//!
//! ```text
//! <Construis>la frise chronologique du XXe siècle {
//! politique {
//! de 1914 à 1918 : Grande Guerre
//! 04/10/1958 : Ve République
//! }
//! culture {
//! 1936 : Congés payés (Le Front populaire ouvre les vacances.)
//! }
//! }
//! ```
//!
//! Une bande s'ouvre d'un nom suivi d'une accolade — le deux-points se
//! tolère, `Vie politique: {` — et se referme d'une accolade seule. Elle
//! porte son nom dans la marge, ses périodes dans son bandeau et ses
//! cartouches de part et d'autre, dans sa couleur. Sans bande nommée, la
//! frise reste linéaire et n'a qu'un bandeau.

use crate::utils::erreur;
use crate::utils::texte::echappe;

struct Evenement {
    instant: f64,
    date: String,
    titre: Vec<String>,
    detail: Vec<String>,
}

/// Une bande de la frise : la linéaire n'en a qu'une, sans nom.
struct Bande {
    nom: Option<String>,
    evenements: Vec<Evenement>,
    periodes: Vec<Periode>,
}

struct Periode {
    debut: f64,
    fin: f64,
    date: String,
    titre: String,
    detail: Option<String>,
}

/// La date en nombre : l'année, affinée du mois puis du jour. Seul l'ordre
/// compte — 372 majore les jours d'une année, la monotonie suffit.
fn lit_date(texte: &str) -> Option<f64> {
    let t = texte.trim();
    let t = t.strip_prefix("vers ").or_else(|| t.strip_prefix("Vers ")).unwrap_or(t);
    let morceaux: Vec<&str> = t.trim().split('/').collect();
    let entier = |s: &str| -> Option<i32> { s.trim().parse::<i32>().ok() };
    match morceaux[..] {
        [annee] => entier(annee).map(|a| a as f64),
        [mois, annee] => {
            let m = entier(mois).filter(|m| (1..=12).contains(m))?;
            let a = entier(annee)?;
            Some(a as f64 + (m - 1) as f64 / 12.0)
        }
        [jour, mois, annee] => {
            let j = entier(jour).filter(|j| (1..=31).contains(j))?;
            let m = entier(mois).filter(|m| (1..=12).contains(m))?;
            let a = entier(annee)?;
            Some(a as f64 + (m - 1) as f64 / 12.0 + (j - 1) as f64 / 372.0)
        }
        _ => None,
    }
}

/// Les deux dates d'une période : `de 1914 à 1918`, `1914 -- 1918` ou
/// `1914 — 1918`. Hors de ces trois écritures, ce n'est pas une période.
fn lit_bornes(texte: &str) -> Option<(String, String)> {
    let t = texte.trim();
    if let Some(reste) = t.strip_prefix("de ").or_else(|| t.strip_prefix("De ")) {
        if let Some((a, b)) = reste.split_once(" à ") {
            return Some((a.trim().to_string(), b.trim().to_string()));
        }
    }
    for separateur in ["--", "—", "–"] {
        if let Some((a, b)) = t.split_once(separateur) {
            return Some((a.trim().to_string(), b.trim().to_string()));
        }
    }
    // Le tiret simple, à condition qu'il respire : « 1789 - 1799 ». Les
    // espaces lèvent l'ambiguïté avec l'année négative, qui colle son signe
    // — « -52 » reste une date, « -753 - -509 » une période.
    if let Some((a, b)) = t.split_once(" - ") {
        return Some((a.trim().to_string(), b.trim().to_string()));
    }
    None
}

/// Le titre d'un côté, le détail de l'autre : la description se met entre
/// parenthèses, en fin de ligne. L'appariement se fait par équilibrage
/// depuis la fin, si bien qu'une parenthèse dans la description ne trompe
/// pas la lecture ; une ligne entièrement parenthésée reste un titre.
fn titre_et_detail(reste: &str) -> (String, Option<String>) {
    let t = reste.trim();
    if t.ends_with(')') {
        let octets = t.as_bytes();
        let mut profondeur = 0i32;
        let mut ouvrante = None;
        for i in (0..octets.len()).rev() {
            match octets[i] {
                b')' => profondeur += 1,
                b'(' => {
                    profondeur -= 1;
                    if profondeur == 0 {
                        ouvrante = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(i) = ouvrante {
            let titre = t[..i].trim();
            let detail = t[i + 1..t.len() - 1].trim();
            if !titre.is_empty() && !detail.is_empty() {
                return (titre.to_string(), Some(detail.to_string()));
            }
        }
    }
    (t.to_string(), None)
}

/// Le texte se replie en lignes courtes — le SVG ne connaît pas la césure.
fn replie(texte: &str, largeur: usize, lignes_max: usize) -> Vec<String> {
    let mut lignes = Vec::new();
    let mut courante = String::new();
    for mot in texte.split_whitespace() {
        if !courante.is_empty() && courante.chars().count() + 1 + mot.chars().count() > largeur {
            lignes.push(std::mem::take(&mut courante));
        }
        if !courante.is_empty() {
            courante.push(' ');
        }
        courante.push_str(mot);
    }
    if !courante.is_empty() {
        lignes.push(courante);
    }
    lignes.truncate(lignes_max);
    lignes
}

/// Le corps se lit ligne à ligne. Une ligne « nom { » ouvre une bande, une
/// accolade seule la referme ; sans bande nommée, tout tombe dans la bande
/// anonyme, et la frise reste linéaire.
fn lit_lignes(corps: &str) -> Result<Vec<Bande>, String> {
    let mut bandes: Vec<Bande> = vec![Bande { nom: None, evenements: Vec::new(), periodes: Vec::new() }];
    let mut courante = 0usize;
    for ligne in corps.lines() {
        let l = ligne.trim();
        if l.is_empty() {
            continue;
        }
        if l == "}" {
            courante = 0;
            continue;
        }
        if let Some(nom) = l.strip_suffix('{') {
            // « Vie politique: { » se tolère, même si la maison écrit plutôt
            // « Vie politique { », comme les étapes d'une démonstration.
            let nom = nom.trim().trim_end_matches(':').trim();
            if !nom.is_empty() && !nom.contains(':') {
                bandes.push(Bande {
                    nom: Some(nom.to_string()),
                    evenements: Vec::new(),
                    periodes: Vec::new(),
                });
                courante = bandes.len() - 1;
                continue;
            }
        }
        let Some((date_txt, reste)) = l.split_once(':') else {
            return Err(format!(
                "la ligne « {} » n'a pas de date — « JJ/MM/AAAA : titre (détail) »",
                l
            ));
        };
        let date_txt = date_txt.trim();
        let (titre, detail) = titre_et_detail(reste);
        if titre.is_empty() {
            return Err(format!("l'événement du {} n'a pas de titre", date_txt));
        }
        if let Some((borne_a, borne_b)) = lit_bornes(date_txt) {
            let Some(debut) = lit_date(&borne_a) else {
                return Err(format!(
                    "la date « {} » ne se lit pas — AAAA, MM/AAAA ou JJ/MM/AAAA, année négative admise, « vers » toléré",
                    borne_a
                ));
            };
            let Some(fin) = lit_date(&borne_b) else {
                return Err(format!(
                    "la date « {} » ne se lit pas — AAAA, MM/AAAA ou JJ/MM/AAAA, année négative admise, « vers » toléré",
                    borne_b
                ));
            };
            if fin < debut {
                return Err(format!(
                    "la période « {} » finit avant de commencer",
                    date_txt
                ));
            }
            bandes[courante].periodes.push(Periode {
                debut,
                fin,
                date: format!("{} – {}", borne_a, borne_b),
                titre,
                detail,
            });
            continue;
        }
        let Some(instant) = lit_date(date_txt) else {
            return Err(format!(
                "la date « {} » ne se lit pas — AAAA, MM/AAAA ou JJ/MM/AAAA, année négative admise, « vers » toléré",
                date_txt
            ));
        };
        // Une bande nommée serre ses lignes : le titre s'y replie plus tôt.
        let large = bandes[courante].nom.is_none();
        bandes[courante].evenements.push(Evenement {
            instant,
            date: date_txt.to_string(),
            titre: replie(&titre, if large { 30 } else { 24 }, 2),
            detail: detail.map(|d| replie(&d, if large { 42 } else { 34 }, 3)).unwrap_or_default(),
        });
    }
    // La bande anonyme ne survit que si elle a servi : une frise
    // multilinéaire n'a pas de bandeau sans nom.
    if bandes[0].evenements.is_empty() && bandes[0].periodes.is_empty() && bandes.len() > 1 {
        bandes.remove(0);
    }
    if bandes.iter().all(|b| b.evenements.is_empty() && b.periodes.is_empty()) {
        return Err("la frise est vide — une ligne par événement : « JJ/MM/AAAA : titre (détail) »".into());
    }
    if let Some(vide) = bandes.iter().find(|b| b.evenements.is_empty() && b.periodes.is_empty()) {
        return Err(format!(
            "la bande « {} » est vide",
            vide.nom.clone().unwrap_or_default()
        ));
    }
    for b in &mut bandes {
        b.evenements.sort_by(|a, c| a.instant.partial_cmp(&c.instant).unwrap_or(std::cmp::Ordering::Equal));
        b.periodes.sort_by(|a, c| a.debut.partial_cmp(&c.debut).unwrap_or(std::cmp::Ordering::Equal));
    }
    Ok(bandes)
}

/// La largeur d'écriture d'une ligne, en unités de la frise : un caractère
/// sérif occupe environ la moitié de son corps.
fn largeur_ligne(texte: &str, corps: f64) -> f64 {
    texte.chars().count() as f64 * corps * 0.52
}
/// Un cartouche d'événement, hors du bandeau : sa place et son encombrement.
struct Boite {
    /// L'abscisse de la date sur le bandeau — où s'accroche le trait de rappel.
    x: f64,
    /// L'abscisse du cartouche, qui peut s'écarter pour tenir dans la page.
    cx: f64,
    largeur: f64,
    hauteur: f64,
    rangee: usize,
    /// Au-dessus du bandeau, ou au-dessous : les cartouches alternent.
    dessus: bool,
    lignes: Vec<(String, bool)>,
    couleur: String,
}

const LARGEUR: f64 = 240.0;
const BANDE_H: f64 = 13.0;
const MARGE_D: f64 = 4.0;
/// La pointe qui referme le bandeau : le temps continue au-delà de la frise.
const POINTE: f64 = 11.0;

/// La première rangée libre : celle dont l'occupant le plus à droite laisse
/// la place. C'est l'étagement qui interdit les chevauchements. Le jeu exigé
/// diffère selon ce qu'on range : deux cartouches doivent respirer, deux
/// périodes qui se suivent se touchent sans se gêner — l'une finit où
/// l'autre commence, et elles restent sur le même rang.
fn premiere_rangee_libre_avec(bords: &mut Vec<f64>, gauche: f64, droite: f64, jeu: f64) -> usize {
    for (n, bord) in bords.iter_mut().enumerate() {
        if gauche >= *bord + jeu {
            *bord = droite;
            return n;
        }
    }
    bords.push(droite);
    bords.len() - 1
}

fn premiere_rangee_libre(bords: &mut Vec<f64>, gauche: f64, droite: f64) -> usize {
    premiere_rangee_libre_avec(bords, gauche, droite, 1.5)
}

/// Le pas de la graduation : un nombre rond qui donne entre six et douze
/// repères — l'échelle du temps, celle qui apprend à s'y repérer.
fn pas_graduation(etendue: f64) -> f64 {
    const PAS: [f64; 13] = [
        1.0, 2.0, 5.0, 10.0, 20.0, 25.0, 50.0, 100.0, 200.0, 250.0, 500.0, 1000.0, 2000.0,
    ];
    for p in PAS {
        if etendue / p <= 9.0 {
            return p;
        }
    }
    5000.0
}

/// L'année s'écrit telle qu'un historien la lit : négative pour l'avant.
fn annee(v: f64) -> String {
    format!("{}", v.round() as i64)
}

struct PlanBande {
    boites: Vec<Boite>,
    rangees_dessus: Vec<f64>,
    rangees_dessous: Vec<f64>,
    hauteur_dessus: f64,
    hauteur_dessous: f64,
    /// Le rang de chaque période dans le bandeau, et la hauteur d'un rang.
    rangs: Vec<usize>,
    h_rang: f64,
    /// Les périodes trop étroites pour se nommer dedans : elles se nomment
    /// dans un cartouche, comme un événement.
    dehors: Vec<bool>,
}

fn plan_bande(
    bande: &Bande,
    gauche: f64,
    droite: f64,
    position: &dyn Fn(f64) -> f64,
) -> PlanBande {
    // Les périodes d'abord : leur rang dans le bandeau, et celles dont le nom
    // n'y tient pas.
    let mut bords_periodes: Vec<f64> = Vec::new();
    let rangs: Vec<usize> = bande
        .periodes
        .iter()
        .map(|p| {
            premiere_rangee_libre_avec(
                &mut bords_periodes,
                position(p.debut).clamp(gauche, droite),
                position(p.fin).clamp(gauche, droite),
                -0.05,
            )
        })
        .collect();
    let h_rang = BANDE_H / bords_periodes.len().max(1) as f64;

    // Tout ce qui se pose hors du bandeau — périodes trop étroites et
    // événements — se range dans l'ordre du temps, pour que l'alternance
    // dessus/dessous suive la frise et non l'ordre de saisie.
    struct Aposer {
        x: f64,
        lignes: Vec<(String, bool)>,
        couleur: String,
    }
    let mut a_poser: Vec<Aposer> = Vec::new();
    let mut dehors = Vec::new();
    for (i, p) in bande.periodes.iter().enumerate() {
        let x1 = position(p.debut).clamp(gauche, droite);
        let x2 = position(p.fin).clamp(gauche, droite);
        let tient = largeur_ligne(&p.titre, 2.7) + 2.0 < (x2 - x1).max(0.8);
        dehors.push(!tient);
        if !tient {
            a_poser.push(Aposer {
                x: (x1 + x2) / 2.0,
                lignes: vec![(p.titre.clone(), true), (p.date.clone(), false)],
                couleur: COULEURS[i % COULEURS.len()].to_string(),
            });
        }
    }
    for e in &bande.evenements {
        let mut lignes: Vec<(String, bool)> = vec![(e.date.clone(), true)];
        for t in &e.titre {
            lignes.push((t.clone(), false));
        }
        for d in &e.detail {
            lignes.push((d.clone(), false));
        }
        a_poser.push(Aposer {
            x: position(e.instant),
            lignes,
            couleur: String::new(),
        });
    }
    a_poser.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));

    let mut boites: Vec<Boite> = Vec::new();
    let mut bords_dessus: Vec<f64> = Vec::new();
    let mut bords_dessous: Vec<f64> = Vec::new();
    for (i, item) in a_poser.iter().enumerate() {
        let mut largeur: f64 = 0.0;
        for (texte, _) in &item.lignes {
            largeur = largeur.max(largeur_ligne(texte, 2.7));
        }
        let largeur = largeur.min(60.0) + 4.0;
        let hauteur = 2.6 + 3.3 * item.lignes.len() as f64;
        let cx = item
            .x
            .clamp(MARGE_D + largeur / 2.0, LARGEUR - MARGE_D - largeur / 2.0);
        let dessus = i % 2 == 1;
        let bords = if dessus { &mut bords_dessus } else { &mut bords_dessous };
        let rangee = premiere_rangee_libre(bords, cx - largeur / 2.0, cx + largeur / 2.0);
        boites.push(Boite {
            x: item.x,
            cx,
            largeur,
            hauteur,
            rangee,
            dessus,
            lignes: item.lignes.clone(),
            couleur: item.couleur.clone(),
        });
    }

    let hauteurs = |dessus: bool| -> Vec<f64> {
        let mut h: Vec<f64> = Vec::new();
        for b in boites.iter().filter(|b| b.dessus == dessus) {
            if h.len() <= b.rangee {
                h.resize(b.rangee + 1, 0.0);
            }
            h[b.rangee] = h[b.rangee].max(b.hauteur);
        }
        h
    };
    let rangees_dessus = hauteurs(true);
    let rangees_dessous = hauteurs(false);
    let cumul = |r: &[f64]| -> f64 {
        if r.is_empty() {
            0.0
        } else {
            r.iter().map(|h| h + 2.2).sum::<f64>() + 3.0
        }
    };
    PlanBande {
        hauteur_dessus: cumul(&rangees_dessus),
        hauteur_dessous: cumul(&rangees_dessous),
        boites,
        rangees_dessus,
        rangees_dessous,
        rangs,
        h_rang,
        dehors,
    }
}

const COULEURS: [&str; 6] = [
    "#4c6ef5", "#37b24d", "#f08c00", "#ae3ec9", "#e03131", "#0ca678",
];

/// Le bandeau d'une bande : le trait épais, et les périodes qui l'habitent.
fn dessine_bandeau(
    bande: &Bande,
    plan: &PlanBande,
    haut: f64,
    gauche: f64,
    droite: f64,
    couleur_bande: &str,
    position: &dyn Fn(f64) -> f64,
    svg: &mut String,
) {
    // Le bandeau se referme sur une grande pointe : le temps ne s'arrête pas
    // au bord de la feuille, il continue.
    let bas = haut + BANDE_H;
    let milieu = haut + BANDE_H / 2.0;
    svg.push_str(&format!(
        "<path class=\"frise-pointe\" d=\"M{droite:.1},{haut:.1} L{:.1},{milieu:.1} L{droite:.1},{bas:.1} Z\" fill=\"{couleur_bande}\"/>",
        droite + POINTE
    ));
    svg.push_str(&format!(
        "<path class=\"frise-bandeau\" d=\"M{gauche:.1},{haut:.1} L{droite:.1},{haut:.1} L{:.1},{milieu:.1} \
         L{droite:.1},{bas:.1} L{gauche:.1},{bas:.1} Z\"/>",
        droite + POINTE
    ));
    let h_rang = plan.h_rang;
    for (i, p) in bande.periodes.iter().enumerate() {
        let x1 = position(p.debut).clamp(gauche, droite);
        let x2 = position(p.fin).clamp(gauche, droite);
        let large = (x2 - x1).max(0.8);
        let couleur = COULEURS[i % COULEURS.len()];
        let haut = haut + plan.rangs[i] as f64 * h_rang;
        svg.push_str(&format!(
            "<rect class=\"frise-periode\" x=\"{x1:.1}\" y=\"{haut:.1}\" width=\"{large:.1}\" height=\"{h_rang:.1}\" \
             fill=\"{couleur}\" fill-opacity=\"0.22\" stroke=\"{couleur}\" stroke-width=\"0.35\"/>"
        ));
        if plan.dehors[i] {
            // Trop étroite pour se nommer dedans : son nom l'attend dehors.
            continue;
        }
        // La période s'habite : son nom, ses bornes, et ce qui la caractérise
        // si la place le permet.
        let milieu = (x1 + x2) / 2.0;
        let colonne = ((large / 1.45) as usize).max(6);
        let mut lignes: Vec<(String, bool)> = replie(&p.titre, colonne, 2)
            .into_iter()
            .map(|l| (l, true))
            .collect();
        if largeur_ligne(&p.date, 2.5) + 2.0 < large {
            lignes.push((p.date.clone(), false));
        }
        if let Some(detail) = &p.detail {
            for ligne in replie(detail, colonne, 2) {
                lignes.push((ligne, false));
            }
        }
        // Ce qui déborde du rang ne s'y imprime pas.
        let tenables = ((h_rang - 1.4) / 3.0) as usize;
        lignes.truncate(tenables.max(1));
        let depart = haut + h_rang / 2.0 - 1.4 * (lignes.len() as f64 - 1.0) + 1.0;
        for (k, (ligne, gras)) in lignes.iter().enumerate() {
            svg.push_str(&format!(
                "<text class=\"{}\" x=\"{milieu:.1}\" y=\"{:.1}\" fill=\"{}\">{}</text>",
                if *gras { "frise-periode-titre" } else { "frise-periode-detail" },
                depart + 3.0 * k as f64,
                if *gras { couleur } else { "#333" },
                echappe(ligne)
            ));
        }
    }
}

/// Les cartouches d'une bande, de part et d'autre du bandeau, chacun relié à
/// sa date. Les traits se tracent tous avant les boîtes : celles-ci sont
/// opaques, et masquent donc ce qui les traverserait.
/// L'ordonnée du sommet d'un cartouche, le bandeau étant posé à `haut`.
fn ordonnee_boite(plan: &PlanBande, haut: f64, b: &Boite) -> f64 {
    let rangees = if b.dessus { &plan.rangees_dessus } else { &plan.rangees_dessous };
    let repli: f64 = rangees[..b.rangee].iter().map(|h| h + 2.2).sum();
    if b.dessus {
        haut - 3.0 - repli - b.hauteur
    } else {
        haut + BANDE_H + 3.0 + repli
    }
}

fn dessine_boites(plan: &PlanBande, haut: f64, couleur: &str, svg: &mut String) {
    let bas_bandeau = haut + BANDE_H;
    let ordonnee = |b: &Boite| -> f64 { ordonnee_boite(plan, haut, b) };
    for b in &plan.boites {
        let y = ordonnee(b);
        let (depart, arrivee) = if b.dessus {
            (haut, y + b.hauteur)
        } else {
            (bas_bandeau, y)
        };
        svg.push_str(&format!(
            "<line class=\"frise-rappel\" x1=\"{:.1}\" y1=\"{depart:.1}\" x2=\"{:.1}\" y2=\"{arrivee:.1}\"/>",
            b.x, b.x
        ));
        if (b.cx - b.x).abs() > 0.4 {
            svg.push_str(&format!(
                "<line class=\"frise-rappel\" x1=\"{:.1}\" y1=\"{arrivee:.1}\" x2=\"{:.1}\" y2=\"{arrivee:.1}\"/>",
                b.x, b.cx
            ));
        }
    }
    for b in &plan.boites {
        let y = ordonnee(b);
        let teinte = if b.couleur.is_empty() { couleur } else { b.couleur.as_str() };
        svg.push_str(&format!(
            "<rect class=\"frise-boite\" x=\"{:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"0.8\" stroke=\"{teinte}\"/>",
            b.cx - b.largeur / 2.0,
            b.largeur,
            b.hauteur
        ));
        let mut ligne_y = y + 3.4;
        for (texte, gras) in &b.lignes {
            svg.push_str(&format!(
                "<text class=\"{}\" x=\"{:.1}\" y=\"{ligne_y:.1}\">{}</text>",
                if *gras { "frise-date" } else { "frise-titre" },
                b.cx,
                echappe(texte)
            ));
            ligne_y += 3.3;
        }
    }
}

pub fn commande(
    verbe: &str,
    desc: &str,
    corps: Option<&str>,
    _env: &mut crate::Env,
) -> Option<String> {
    if verbe != "Construis" || !desc.to_lowercase().contains("frise") {
        return None;
    }
    let etiquette = format!("Construis>{}", desc);
    let Some(corps) = corps else {
        return Some(erreur::bloc(
            &etiquette,
            "le corps { … } manque — une ligne par événement : « JJ/MM/AAAA : titre (détail) »",
        ));
    };
    let bandes = match lit_lignes(corps) {
        Ok(b) => b,
        Err(message) => return Some(erreur::bloc(&etiquette, &message)),
    };
    let multilineaire = bandes.iter().any(|b| b.nom.is_some());

    let bas_desc = desc.to_lowercase();
    let apres = bas_desc
        .find("frise chronologique")
        .map(|i| i + "frise chronologique".len())
        .or_else(|| bas_desc.find("frise").map(|i| i + "frise".len()))
        .unwrap_or(desc.len());
    let complement = desc[apres..].trim();
    let titre = (!complement.is_empty()).then(|| format!("Frise chronologique {}", complement));

    // L'échelle du temps est commune à toutes les bandes : c'est elle qui met
    // la simultanéité sous les yeux.
    let mut premier = f64::INFINITY;
    let mut dernier = f64::NEG_INFINITY;
    for b in &bandes {
        for e in &b.evenements {
            premier = premier.min(e.instant);
            dernier = dernier.max(e.instant);
        }
        for p in &b.periodes {
            premier = premier.min(p.debut);
            dernier = dernier.max(p.fin);
        }
    }
    if !premier.is_finite() || !dernier.is_finite() {
        return Some(erreur::bloc(&etiquette, "la frise est vide"));
    }
    // Une frise d'un seul instant n'a pas d'étendue : on lui en donne une.
    if (dernier - premier).abs() < 1e-9 {
        premier -= 1.0;
        dernier += 1.0;
    }
    let pas = pas_graduation(dernier - premier);
    // Les bornes s'arrondissent au pas : la frise commence et finit sur un
    // repère, comme celles qu'on affiche en classe.
    let debut = (premier / pas).floor() * pas;
    let fin = (dernier / pas).ceil() * pas;
    let etendue = (fin - debut).max(f64::EPSILON);

    // La marge de gauche loge le nom des bandes ; celle de droite, la flèche.
    let gauche = if multilineaire {
        bandes
            .iter()
            .filter_map(|b| b.nom.as_ref())
            .map(|n| largeur_ligne(n, 3.1) + 5.0)
            .fold(10.0f64, f64::max)
            .min(48.0)
    } else {
        10.0
    };
    let droite = LARGEUR - POINTE - 3.0;
    let position = move |instant: f64| -> f64 {
        gauche + (instant - debut) / etendue * (droite - gauche)
    };

    let plans: Vec<PlanBande> = bandes
        .iter()
        .map(|b| plan_bande(b, gauche, droite, &position))
        .collect();

    let haut_graduation = 7.0;
    let mut hauteur = haut_graduation + 1.0;
    let mut hauts = Vec::new();
    for plan in &plans {
        hauteur += plan.hauteur_dessus;
        hauts.push(hauteur);
        hauteur += BANDE_H + plan.hauteur_dessous + if multilineaire { 3.0 } else { 1.0 };
    }
    let bas_figure = hauteur + if titre.is_some() { 1.0 } else { 0.0 };
    let hauteur_svg = bas_figure + if titre.is_some() { 5.0 } else { 1.0 };

    let mut svg = format!(
        "<div class=\"trace frise\"><svg viewBox=\"0 0 {LARGEUR} {hauteur_svg:.1}\" \
         preserveAspectRatio=\"xMidYMin meet\" xmlns=\"http://www.w3.org/2000/svg\">\
         <style>\
         .frise-bandeau{{fill:none;stroke:#333;stroke-width:0.5}}\
         .frise-pointe{{fill-opacity:0.22}}\
         .frise-echelle{{stroke:#333;stroke-width:0.35}}\
         .frise-grille{{stroke:#adb5bd;stroke-width:0.2;stroke-dasharray:1.2 1.2}}\
         .frise-rappel{{stroke:#868e96;stroke-width:0.3}}\
         .frise-boite{{fill:#fff;stroke-width:0.4}}\
         .frise-an{{font-size:2.8px;fill:#333;text-anchor:middle;font-family:serif}}\
         .frise-date{{font-size:2.7px;fill:#333;text-anchor:middle;font-family:serif;font-weight:700}}\
         .frise-titre{{font-size:2.7px;fill:#333;text-anchor:middle;font-family:serif}}\
         .frise-periode-titre{{font-size:2.7px;text-anchor:middle;font-family:serif;font-weight:600}}\
         .frise-periode-detail{{font-size:2.4px;text-anchor:middle;font-family:serif}}\
         .frise-bande{{font-size:3.1px;text-anchor:end;font-family:serif;font-weight:600}}\
         .frise-legende{{font-size:3.2px;fill:#333;text-anchor:middle;font-family:serif;font-style:italic}}\
         </style>"
    );

    // La graduation, et les verticales qui la prolongent : ce qui tombe dans
    // la même colonne est simultané, d'une bande à l'autre.
    let dernier_bandeau = hauts.last().copied().unwrap_or(haut_graduation) + BANDE_H;
    // La grille descend jusqu'au dernier bandeau : ce qui tombe dans la même
    // colonne est simultané.
    // Ce que la grille doit contourner : les cartouches, et le corps même de
    // la frise. Elle ne subsiste donc que dans les espaces libres — entre la
    // graduation et le premier bandeau, et d'une bande à l'autre —, ce qui
    // suffit à faire lire la verticale sans jamais barrer ce qu'on regarde.
    let mut obstacles: Vec<(f64, f64, f64, f64)> = Vec::new();
    for (plan, haut) in plans.iter().zip(&hauts) {
        obstacles.push((
            gauche - 1.0,
            droite + POINTE + 1.0,
            *haut,
            *haut + BANDE_H,
        ));
        for b in &plan.boites {
            let y = ordonnee_boite(plan, *haut, b);
            obstacles.push((
                b.cx - b.largeur / 2.0,
                b.cx + b.largeur / 2.0,
                y,
                y + b.hauteur,
            ));
        }
    }
    let mut v = debut;
    while v <= fin + 1e-6 {
        let x = position(v);
        // Le trait s'interrompt devant chaque cartouche et reprend après :
        // rien ne traverse une boîte, pas même en pointillé.
        let mut coupures: Vec<(f64, f64)> = obstacles
            .iter()
            .filter(|(x1, x2, _, _)| x >= *x1 - 0.8 && x <= *x2 + 0.8)
            .map(|(_, _, y1, y2)| (*y1 - 0.8, *y2 + 0.8))
            .collect();
        coupures.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        let mut y = haut_graduation - 1.0;
        for (a, b) in coupures {
            if a - y > 1.2 {
                svg.push_str(&format!(
                    "<line class=\"frise-grille\" x1=\"{x:.1}\" y1=\"{y:.1}\" x2=\"{x:.1}\" y2=\"{a:.1}\"/>"
                ));
            }
            y = y.max(b);
        }
        if dernier_bandeau - y > 1.2 {
            svg.push_str(&format!(
                "<line class=\"frise-grille\" x1=\"{x:.1}\" y1=\"{y:.1}\" x2=\"{x:.1}\" y2=\"{dernier_bandeau:.1}\"/>"
            ));
        }
        svg.push_str(&format!(
            "<line class=\"frise-echelle\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\"/>",
            haut_graduation - 2.4,
            haut_graduation
        ));
        svg.push_str(&format!(
            "<text class=\"frise-an\" x=\"{x:.1}\" y=\"{:.1}\">{}</text>",
            haut_graduation - 3.6,
            annee(v)
        ));
        v += pas;
    }

    for (i, ((bande, plan), haut)) in bandes.iter().zip(&plans).zip(&hauts).enumerate() {
        let couleur = COULEURS[i % COULEURS.len()];
        dessine_bandeau(bande, plan, *haut, gauche, droite, couleur, &position, &mut svg);
        if let Some(nom) = &bande.nom {
            svg.push_str(&format!(
                "<text class=\"frise-bande\" x=\"{:.1}\" y=\"{:.1}\" fill=\"{couleur}\">{}</text>",
                gauche - 2.5,
                haut + BANDE_H / 2.0 + 1.1,
                echappe(nom)
            ));
        }
        dessine_boites(plan, *haut, couleur, &mut svg);
    }

    if let Some(titre) = &titre {
        svg.push_str(&format!(
            "<text class=\"frise-legende\" x=\"{:.1}\" y=\"{:.1}\">{}</text>",
            LARGEUR / 2.0,
            hauteur_svg - 1.4,
            echappe(titre)
        ));
    }
    svg.push_str("</svg></div>");
    Some(svg)
}
