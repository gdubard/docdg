//! La frise chronologique : les dates aux trois précisions, l'année
//! négative, « vers », le tri chronologique, les deux séparateurs du
//! détail, et les erreurs qui nomment leur ligne.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn la_frise_range_les_evenements_dans_l_ordre_du_temps() {
    let h = rend(
        "<Construis>la frise chronologique {\n09/11/1989 : Chute du mur de Berlin\n20/07/1969 : Premier pas sur la Lune\n12/07/1998 : Victoire en Coupe du monde\n}\n",
    );
    let un = h.find("20/07/1969").expect("1969 absent");
    let deux = h.find("09/11/1989").expect("1989 absent");
    let trois = h.find("12/07/1998").expect("1998 absent");
    assert!(un < deux && deux < trois, "{}", h);
    assert!(h.contains("frise-bandeau\" d="), "{}", h);
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn le_detail_se_met_entre_parentheses() {
    let h = rend(
        "<Construis>la frise chronologique {\n1969 : Premier pas sur la Lune (Neil Armstrong marche sur la Lune.)\n1998 : Victoire en Coupe du monde (La France remporte sa première étoile.)\n}\n",
    );
    assert!(h.contains("Neil Armstrong marche sur la Lune."), "{}", h);
    assert!(h.contains("La France remporte sa première étoile."), "{}", h);
    assert!(h.contains("frise-boite"), "l'événement se pose hors du bandeau : {}", h);
}

#[test]
fn la_parenthese_interne_ne_trompe_pas_la_lecture() {
    let h = rend(
        "<Construis>la frise chronologique {\n1969 : Premier pas sur la Lune (Neil Armstrong (Apollo 11) y marche.)\n1998 : Victoire\n}\n",
    );
    assert!(h.contains("Neil Armstrong (Apollo 11) y marche."), "{}", h);
    assert!(h.contains(">Premier pas sur la Lune<"), "le titre s'arrête à la parenthèse ouvrante : {}", h);
}

#[test]
fn la_ligne_entierement_parenthesee_reste_un_titre() {
    let h = rend("<Construis>la frise chronologique {\n1969 : (Premier pas sur la Lune)\n1998 : Victoire\n}\n");
    assert!(!h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("(Premier pas sur la Lune)"), "{}", h);
}

#[test]
fn l_annee_seule_le_mois_et_le_jour_se_lisent() {
    let h = rend(
        "<Construis>la frise chronologique {\n1515 : Marignan\n06/1944 : Débarquement\n14/07/1789 : Prise de la Bastille\n}\n",
    );
    assert!(!h.contains("calcul-absent"), "{}", h);
    let bastille = h.find("14/07/1789").unwrap();
    let marignan = h.find("1515").unwrap();
    assert!(bastille < marignan || marignan < bastille, "les deux se placent");
    assert!(h.find("Prise de la Bastille").unwrap() < h.find("Débarquement").unwrap(), "{}", h);
}

#[test]
fn l_antiquite_s_ecrit_en_annees_negatives() {
    let h = rend(
        "<Construis>la frise chronologique {\n-52 : Alésia\n-753 : Fondation de Rome\nvers -509 : Naissance de la République\n}\n",
    );
    assert!(!h.contains("calcul-absent"), "{}", h);
    let rome = h.find("Fondation de Rome").unwrap();
    let republique = h.find("Naissance de la République").unwrap();
    let alesia = h.find("Alésia").unwrap();
    assert!(rome < republique && republique < alesia, "{}", h);
    assert!(h.contains("vers -509"), "« vers » s'imprime tel quel : {}", h);
}

#[test]
fn la_legende_reprend_le_complement() {
    let h = rend(
        "<Construis>la frise chronologique du second XXe siècle {\n1969 : Lune\n1989 : Berlin\n}\n",
    );
    assert!(
        h.contains("Frise chronologique du second XXe siècle"),
        "{}",
        h
    );
}

#[test]
fn le_long_detail_se_replie() {
    let h = rend(
        "<Construis>la frise chronologique {\n-52 : Alésia (Vercingétorix dépose les armes devant César après un long siège que les sources anciennes racontent en détail.)\n1515 : Marignan\n}\n",
    );
    assert!(h.contains("Vercingétorix dépose les armes devant"), "{}", h);
    assert!(h.contains("César"), "le détail se replie en lignes courtes : {}", h);
}

#[test]
fn la_date_illisible_se_signale_et_se_nomme() {
    let h = rend("<Construis>la frise chronologique {\nbientôt : La suite\n}\n");
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("bientôt"), "{}", h);
    assert!(h.contains("année négative admise"), "{}", h);
}

#[test]
fn la_ligne_sans_date_se_signale() {
    let h = rend("<Construis>la frise chronologique {\nChute du mur de Berlin\n}\n");
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("n'a pas de date"), "{}", h);
}

#[test]
fn la_frise_vide_se_signale() {
    let h = rend("<Construis>la frise chronologique {\n}\n");
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("la frise est vide"), "{}", h);
}

#[test]
fn l_evenement_seul_se_centre_sans_diviser_par_zero() {
    let h = rend("<Construis>la frise chronologique {\n1969 : Lune\n}\n");
    assert!(!h.contains("calcul-absent"), "{}", h);
    assert!(!h.contains("NaN"), "{}", h);
}

#[test]
fn la_frise_ne_gene_pas_les_graphes() {
    let h = rend(
        "<Construis>le graphe G {\n\tA -> B\n\tB -> C\n}\n\n<Construis>la frise chronologique {\n1969 : Lune\n1989 : Berlin\n}\n",
    );
    assert!(h.contains("frise-bandeau\" d="), "{}", h);
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn les_genres_axiome_et_conjecture_s_enoncent() {
    let h = rend(
        "<Énonce>l'axiome du choix {\nTout produit d'ensembles non vides est non vide.\n}\n\n<Énonce>la conjecture de Goldbach {\nTout entier pair supérieur à 2 est somme de deux nombres premiers.\n}\n",
    );
    assert!(h.contains("Axiome 1"), "{}", h);
    assert!(h.contains("(choix)"), "l'article de liaison se dépouille : {}", h);
    assert!(h.contains("Conjecture 1"), "{}", h);
    assert!(h.contains("(Goldbach)"), "{}", h);
    assert!(h.contains("env-axiome env-italique"), "{}", h);
}

fn hauteurs(h: &str, classe: &str) -> Vec<String> {
    let motif = format!("{}\" x=\"", classe);
    h.match_indices(&motif)
        .filter_map(|(i, _)| {
            let suite = &h[i..];
            let y = suite.find(" y=\"")? + 4;
            let fin = suite[y..].find('"')? + y;
            Some(suite[y..fin].to_string())
        })
        .collect()
}

#[test]
fn les_cartouches_serres_s_etagent_avec_leur_trait_de_rappel() {
    let h = rend(
        "<Construis>la frise chronologique {\n28/06/1914 : Attentat de Sarajevo\n03/08/1914 : Entrée en guerre de la France\n11/11/1918 : Armistice\n06/02/1934 : Crise du 6 février\n1936 : Congés payés\n1938 : Accords de Munich\n}\n",
    );
    let titres = hauteurs(&h, "frise-titre");
    let distinctes: std::collections::BTreeSet<_> = titres.iter().collect();
    assert!(
        distinctes.len() >= 3,
        "les serrés doivent monter d'un étage — hauteurs : {:?}\n{}",
        distinctes,
        h
    );
    assert!(h.contains("frise-rappel\" x1"), "{}", h);
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn la_periode_se_lit_en_trois_ecritures() {
    let h = rend(
        "<Construis>la frise chronologique {\nde 1914 à 1918 : Grande Guerre\n1936 -- 1938 : Front populaire\n1922 — 1926 : Cartel des gauches\n}\n",
    );
    assert_eq!(h.matches("frise-periode\" x=").count(), 3, "{}", h);
    for nom in ["Grande Guerre", "Front populaire", "Cartel des gauches"] {
        assert!(h.contains(nom), "la période {} habite le bandeau : {}", nom, h);
    }
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn les_periodes_chevauchantes_s_empilent() {
    let h = rend(
        "<Construis>la frise chronologique {\n1929 — 1939 : Grande Dépression\n1936 -- 1938 : Front populaire\n1950 : Un événement pour l'étendue\n}\n",
    );
    let rangs: std::collections::BTreeSet<_> =
        hauteurs(&h, "frise-periode").into_iter().collect();
    assert_eq!(
        rangs.len(),
        2,
        "les périodes qui se chevauchent se partagent la hauteur : {:?}\n{}",
        rangs,
        h
    );
}

#[test]
fn la_periode_seule_fait_une_frise() {
    let h = rend("<Construis>la frise chronologique {\nde 1914 à 1918 : Grande Guerre\n}\n");
    assert!(!h.contains("calcul-absent"), "{}", h);
    assert!(!h.contains("NaN"), "{}", h);
    assert!(h.contains("Grande Guerre"), "{}", h);
}

#[test]
fn la_periode_se_detaille_aussi() {
    let h = rend(
        "<Construis>la frise chronologique {\nde 1914 à 1918 : Grande Guerre (Le premier conflit mondial.)\n1925 : Un événement\n}\n",
    );
    assert!(h.contains("frise-periode-detail"), "{}", h);
    assert!(h.contains("Le premier conflit mondial."), "{}", h);
}

#[test]
fn la_periode_inversee_se_signale() {
    let h = rend("<Construis>la frise chronologique {\n1918 -- 1914 : Grande Guerre\n}\n");
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("finit avant de commencer"), "{}", h);
}

#[test]
fn la_frise_multilineaire_range_ses_evenements_par_bandes() {
    let h = rend(
        "<Construis>la frise chronologique du XXe siècle {\npolitique {\nde 1914 à 1918 : Grande Guerre\n1958 : Ve République\n}\néconomie {\n1929 : Krach de Wall Street\n}\n}\n",
    );
    assert!(h.contains(">politique<"), "{}", h);
    assert!(h.contains(">économie<"), "{}", h);
    assert_eq!(
        h.matches("frise-bandeau\" d=").count(),
        2,
        "un bandeau par bande : {}",
        h
    );
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn l_echelle_du_temps_est_commune_aux_bandes() {
    let h = rend(
        "<Construis>la frise chronologique {\nun {\n1900 : Début\n1936 : Simultané\n}\ndeux {\n1936 : Simultané aussi\n2000 : Fin\n}\n}\n",
    );
    // Les traits de rappel qui descendent du bandeau vers les cartouches :
    // celui de 1936 tombe à la même abscisse dans les deux bandes — c'est la
    // simultanéité rendue visible.
    let x: Vec<String> = h
        .match_indices("frise-rappel\" x1=\"")
        .map(|(i, m)| {
            let s = &h[i + m.len()..];
            s[..s.find('"').unwrap()].to_string()
        })
        .collect();
    let distinctes: std::collections::BTreeSet<_> = x.iter().collect();
    assert_eq!(
        distinctes.len(),
        3,
        "quatre événements, trois dates distinctes : {:?}\n{}",
        x,
        h
    );
}

#[test]
fn une_bande_porte_ses_periodes_dans_son_bandeau() {
    let h = rend(
        "<Construis>la frise chronologique {\nrégimes {\nde 1852 à 1870 : Second Empire\nde 1870 à 1940 : Troisième République\n}\nrepères {\n1789 : Révolution\n}\n}\n",
    );
    assert_eq!(h.matches("frise-periode\" x=").count(), 2, "{}", h);
    assert!(h.contains(">régimes<"), "{}", h);
}

#[test]
fn la_bande_vide_se_signale() {
    let h = rend("<Construis>la frise chronologique {\npolitique {\n}\néconomie {\n1929 : Krach\n}\n}\n");
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("politique"), "{}", h);
    assert!(h.contains("est vide"), "{}", h);
}

#[test]
fn la_frise_lineaire_garde_son_axe_unique() {
    let h = rend("<Construis>la frise chronologique {\n1789 : Révolution\n1914 : Sarajevo\n}\n");
    assert_eq!(h.matches("frise-bandeau\" d=").count(), 1, "{}", h);
    assert!(!h.contains("class=\"frise-bande\" x="), "{}", h);
}

#[test]
fn le_bandeau_se_referme_sur_une_grande_pointe() {
    let h = rend("<Construis>la frise chronologique {\n1789 : Révolution\n1914 : Sarajevo\n}\n");
    assert!(h.contains("frise-pointe"), "{}", h);
    // Le tracé du bandeau passe par la pointe : cinq sommets, dont celui qui
    // dépasse à droite.
    let d = h
        .split("frise-bandeau\" d=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .unwrap_or("");
    assert_eq!(d.matches(" L").count(), 4, "cinq sommets : {}", d);
}

#[test]
fn les_cartouches_se_repartissent_de_part_et_d_autre() {
    let h = rend(
        "<Construis>la frise chronologique {\n1789 : Un\n1800 : Deux\n1850 : Trois\n1900 : Quatre\n}\n",
    );
    let ys: Vec<f64> = hauteurs(&h, "frise-boite")
        .iter()
        .filter_map(|y| y.parse().ok())
        .collect();
    let bandeau: f64 = h
        .split("frise-bandeau\" d=\"M")
        .nth(1)
        .and_then(|s| s.split(',').nth(1))
        .and_then(|s| s.split(' ').next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);
    assert_eq!(ys.len(), 4, "{:?}", ys);
    assert!(
        ys.iter().any(|y| *y < bandeau) && ys.iter().any(|y| *y > bandeau),
        "les cartouches alternent de part et d'autre du bandeau (à {}) : {:?}",
        bandeau,
        ys
    );
}

#[test]
fn aucun_trait_ne_traverse_un_cartouche() {
    // Les traits de rappel se tracent tous avant les boîtes : celles-ci sont
    // opaques et masquent ce qui les traverserait.
    let h = rend(
        "<Construis>la frise chronologique {\n1789 : Un événement au titre assez long\n1791 : Un deuxième au titre long\n1793 : Un troisième au titre long\n1795 : Un quatrième\n}\n",
    );
    let dernier_trait = h.rfind("<line class=\"frise-rappel\"").unwrap();
    let premiere_boite = h.find("<rect class=\"frise-boite\"").unwrap();
    assert!(dernier_trait < premiere_boite, "{}", h);
}

#[test]
fn le_tiret_simple_espace_borne_une_periode() {
    let h = rend(
        "<Construis>la frise chronologique {\n1789 - 1799 : La Révolution française (Bouleversements)\n1799 - 1814 : Le Consulat et l'Empire\n}\n",
    );
    assert_eq!(h.matches("frise-periode\" x=").count(), 2, "{}", h);
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn le_deux_points_apres_le_nom_d_une_bande_se_tolere() {
    let h = rend(
        "<Construis>la frise chronologique {\nVie politique: {\n1848 : IIe République\n}\nÉconomie & Société: {\n1864 : Droit de grève\n}\n}\n",
    );
    assert!(h.contains(">Vie politique<"), "{}", h);
    assert!(h.contains("Économie &amp; Société"), "l'esperluette s'échappe : {}", h);
    assert_eq!(h.matches("frise-bandeau\" d=").count(), 2, "{}", h);
}

#[test]
fn aucun_pointille_ne_traverse_un_cartouche_ni_le_bandeau() {
    let h = rend(
        "<Construis>la frise chronologique {\nun {\n1789 : Révolution française\n1815 : Waterloo\n}\ndeux {\n1848 : Printemps des peuples\n1870 : Sedan\n}\n}\n",
    );
    let nombre = |s: &str, avant: &str| -> Vec<f64> {
        s.match_indices(avant)
            .filter_map(|(i, m)| {
                let reste = &s[i + m.len()..];
                reste[..reste.find('"')?].parse().ok()
            })
            .collect()
    };
    let gx = nombre(&h, "frise-grille\" x1=\"");
    // On relit chaque segment de grille et chaque cartouche, puis on vérifie
    // qu'aucun des deux ne recouvre l'autre.
    let segments: Vec<(f64, f64, f64)> = h
        .match_indices("<line class=\"frise-grille\" x1=\"")
        .filter_map(|(i, _)| {
            let bloc = &h[i..h[i..].find("/>").map(|j| i + j)?];
            let val = |cle: &str| -> Option<f64> {
                let d = bloc.find(cle)? + cle.len();
                bloc[d..].split('"').next()?.parse().ok()
            };
            Some((val("x1=\"")?, val("y1=\"")?, val("y2=\"")?))
        })
        .collect();
    let boites: Vec<(f64, f64, f64, f64)> = h
        .match_indices("<rect class=\"frise-boite\" x=\"")
        .filter_map(|(i, _)| {
            let bloc = &h[i..h[i..].find("/>").map(|j| i + j)?];
            let val = |cle: &str| -> Option<f64> {
                let d = bloc.find(cle)? + cle.len();
                bloc[d..].split('"').next()?.parse().ok()
            };
            Some((val("x=\"")?, val("y=\"")?, val("width=\"")?, val("height=\"")?))
        })
        .collect();
    // Les bandeaux : le corps de la frise, que rien ne doit barrer non plus.
    let bandeaux: Vec<f64> = h
        .match_indices("<path class=\"frise-bandeau\" d=\"M")
        .filter_map(|(i, m)| {
            let reste = &h[i + m.len()..];
            reste[..reste.find('"')?]
                .split(',')
                .nth(1)?
                .split(' ')
                .next()?
                .parse()
                .ok()
        })
        .collect();
    assert!(!gx.is_empty() && !boites.is_empty() && !bandeaux.is_empty(), "{}", h);
    for (x, y1, y2) in &segments {
        for (bx, by, bw, bh) in &boites {
            let dedans = *x > bx - 0.1 && *x < bx + bw + 0.1;
            let recouvre = y1 < &(by + bh) && y2 > by;
            assert!(
                !(dedans && recouvre),
                "un pointillé traverse un cartouche : segment x={} de {} à {}",
                x,
                y1,
                y2
            );
        }
        for haut in &bandeaux {
            assert!(
                !(y1 < &(haut + 13.0) && y2 > haut),
                "un pointillé traverse le corps de la frise (bandeau à {}) : segment de {} à {}",
                haut,
                y1,
                y2
            );
        }
    }
}
