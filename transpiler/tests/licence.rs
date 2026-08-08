use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn l_image_du_carre_par_z_carre_se_dessine() {
    let html = rend("<Trace>l'image du carré [-1 ; 1] × [-1 ; 1] par w = z^2\n");
    assert!(html.contains("<svg"));
    assert!(html.contains("#c0392b"));
    assert!(html.contains(">Im</text>"));
    assert!(html.matches("<path").count() > 15);
}

#[test]
fn l_image_du_cercle_par_joukovski_se_dessine() {
    let html = rend("<Trace>l'image du cercle unité par w = z + 1/z\n");
    assert!(html.contains("<svg"));
    let html = rend("<Trace>l'image du cercle de rayon 2 par w = 1/z, en vert\n");
    assert!(html.contains("<svg"));
    assert!(html.contains("#1e7d32"));
}

#[test]
fn l_evaluateur_complexe_gere_les_singularites() {
    let html = rend("<Trace>l'image du carré [-1 ; 1] × [-1 ; 1] par w = 1/z\n");
    assert!(html.contains("<svg"));
}

#[test]
fn les_residus_se_calculent() {
    let html = rend("<Calcule>les résidus de 1/(z^2 + 1)\n");
    assert!(html.contains("pôle"));
    assert!(html.contains("Res"));
    assert!(html.contains("- \\frac{i}{2}") || html.contains("\\frac{i}{2}"));
    assert!(html.contains("calcul-prose"));
}

#[test]
fn les_tables_de_znz_se_dressent() {
    let html = rend("<Dresse>la table de Z/5Z pour l'addition\n<Dresse>la table de Z/5Z pour la multiplication\n");
    assert_eq!(html.matches("<table").count(), 2);
    assert!(html.contains("+"));
    assert!(html.contains("×"));
    assert!(html.matches("<tr>").count() == 12);
}

#[test]
fn les_generateurs_donnent_l_indicatrice_d_euler() {
    let html = rend("<Détermine>les générateurs de Z/12Z\n");
    assert!(html.contains("premier avec"));
    assert!(html.contains("varphi(12) = 4"));
    assert!(html.contains("1\\,;\\,5\\,;\\,7\\,;\\,11"));
}

#[test]
fn la_permutation_se_decompose_avec_signature_et_ordre() {
    let html = rend("<Décompose>la permutation (2 5 4 1 3) en cycles\n");
    assert!(html.contains("(1\\;2\\;5\\;3\\;4)"));
    assert!(html.contains("+1"));
    assert!(html.contains("paire"));
    assert!(html.contains("5"));
    let html = rend("<Décompose>la permutation (2 1 3) en cycles\n");
    assert!(html.contains("(1\\;2)"));
    assert!(html.contains("impaire"));
    assert!(html.contains("points fixes"));
}

#[test]
fn la_densite_se_verifie_avec_esperance_et_variance() {
    let html = rend("<Étudie>la loi de densité f(x) = 3*x^2 sur [0 ; 1]\n");
    assert!(html.contains("densité de probabilité"));
    assert!(html.contains("E(X)"));
    assert!(html.contains("frac{3}{4}"));
    assert!(html.contains("frac{3}{80}"));
    let html = rend("<Étudie>la loi de densité f(x) = x sur [0 ; 1]\n");
    assert!(html.contains("neq 1"));
}

#[test]
fn la_densite_a_support_infini_est_comprise() {
    let html = rend("<Étudie>la loi de densité f(x) = exp(-x) sur [0 ; +infini]\n");
    assert!(html.contains("densité de probabilité"));
    assert!(html.contains("E(X) ="));
}

#[test]
fn la_loi_normale_donne_sa_probabilite() {
    let html = rend("<Calcule>la probabilité d'être entre -1 et 1 pour la loi normale d'espérance 0 et d'écart type 1\n");
    assert!(html.contains("mathcal{N}"));
    assert!(html.contains("erf"));
    assert!(html.contains("0{,}68"));
}

#[test]
fn la_densite_normale_se_trace() {
    let html = rend("<Trace>la densité de la loi normale d'espérance 2 et d'écart type 0,5\n");
    assert!(html.contains("<svg"));
    assert!(html.contains("class=\"courbe\""));
    assert!(html.contains("class=\"repere\""));
}

#[test]
fn le_theoreme_central_limite_s_illustre() {
    let html = rend("<Trace>l'illustration du théorème central limite avec la somme de 8 dés\n");
    assert!(html.contains("<svg"));
    assert!(html.matches("<rect").count() > 20);
    assert!(html.contains("class=\"courbe\""));
}

#[test]
fn la_somme_de_des_est_une_vraie_loi() {
    let html = rend("<Trace>l'illustration du théorème central limite avec la somme de 2 dés\n");
    assert!(html.matches("<rect").count() == 11);
}

#[test]
fn la_table_a_la_demande_suit_la_saisie() {
    let src = "soit n = <Saisis>un entier{Quel module ?}\n<Dresse>la table de Z/#nZ pour l'addition\n";
    let mut e = Engine::new();
    assert!(!e.render(src, false).html.contains("<table"));
    e.saisies.insert("n".into(), "7".into());
    let html = e.render(src, false).html;
    assert_eq!(html.matches("<tr>").count(), 8);
    assert!(!html.contains("non prise en charge"));
}

#[test]
fn les_saisies_pilotent_le_dernier_etage() {
    let src = "soit n = <Saisis>un entier{Combien de dés ?}\n<Trace>l'illustration du théorème central limite avec la somme de #n dés\n";
    let mut e = Engine::new();
    assert!(!e.render(src, false).html.contains("<svg"));
    e.saisies.insert("n".into(), "12".into());
    assert!(e.render(src, false).html.contains("<svg"));
}

#[test]
fn les_sections_de_licence_des_domaines_sont_completes() {
    for (fichier, saisie, attendu) in [
        ("analyse4", ("k", "3"), "Res"),
        ("algebre4", ("n", "6"), "varphi"),
        ("statistiques-probabilites4", ("n", "10"), "erf"),
    ] {
        let src =
            std::fs::read_to_string(format!("../exemples/{fichier}.txt")).unwrap();
        let mut e = Engine::new();
        let bloque = e.render(&src, false).html;
        assert!(
            !bloque.contains("non prise en charge"),
            "{fichier} : commande refusée"
        );
        assert!(bloque.contains(attendu), "{fichier} : manque {attendu}");
        e.saisies.insert(saisie.0.into(), saisie.1.into());
        let complet = e.render(&src, false).html;
        assert!(complet.len() >= bloque.len(), "{fichier} : rendu réduit");
        assert!(
            !complet.contains("non prise en charge"),
            "{fichier} : commande refusée après saisie"
        );
    }
}
