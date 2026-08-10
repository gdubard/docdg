use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn l_astroide_se_trace() {
    let html = rend("<Trace>la courbe paramétrée x = cos(t)^3 et y = sin(t)^3 pour t dans [0 ; 2*pi]\n");
    assert!(html.contains("<svg"));
    assert!(html.contains("class=\"courbe\""));
}

#[test]
fn la_courbe_parametree_accepte_pi_accole() {
    let html = rend("<Trace>la courbe paramétrée x = 2*cos(t) et y = sin(2*t) pour t dans [0 ; 2pi]\n");
    assert!(html.contains("class=\"courbe\""));
}

#[test]
fn la_cardioide_polaire_se_trace() {
    let html = rend("<Trace>la courbe polaire r = 1 + cos(t) pour t dans [0 ; 2*pi], en rouge\n");
    assert!(html.contains("<svg"));
    assert!(html.contains("class=\"courbe\""));
    assert!(html.contains("#c0392b"));
}

#[test]
fn la_rosace_se_trace_sans_intervalle() {
    let html = rend("<Trace>la courbe polaire r = cos(3*t)\n");
    assert!(html.contains("class=\"courbe\""));
}

#[test]
fn l_ellipse_se_redige_et_se_dessine() {
    let html = rend("<Étudie>la conique d'équation x^2/9 + y^2/4 = 1\n");
    assert!(html.contains("ellipse"));
    assert!(html.contains("a = 3"));
    assert!(html.contains("b = 2"));
    assert!(html.contains("excentricité"));
    assert!(html.contains("foyers"));
    assert!(html.contains("<svg"));
    assert!(html.contains(">F</text>"));
    assert!(html.contains("Ω"));
}

#[test]
fn le_cercle_est_reconnu() {
    let html = rend("<Étudie>la conique d'équation x^2 + y^2 = 4\n");
    assert!(html.contains("cercle"));
    assert!(html.contains("rayon"));
    assert!(html.contains("2"));
}

#[test]
fn l_hyperbole_se_redige_et_se_dessine() {
    let html = rend("<Étudie>la conique d'équation x^2/4 - y^2 = 1\n");
    assert!(html.contains("hyperbole"));
    assert!(html.contains("a = 2"));
    assert!(html.contains("asymptotes"));
    assert!(html.contains("<svg"));
    assert!(html.contains("F'"));
}

#[test]
fn la_parabole_donne_sommet_foyer_directrice() {
    let html = rend("<Étudie>la conique d'équation y^2 = 4x\n");
    assert!(html.contains("parabole"));
    assert!(html.contains("p = 2"));
    assert!(html.contains("(0 ; 0)"));
    assert!(html.contains("(1 ; 0)"));
    assert!(html.contains("directrice"));
    assert!(html.contains("<svg"));
}

#[test]
fn le_terme_croise_declenche_la_rotation() {
    let html = rend("<Étudie>la conique d'équation x^2 + xy + y^2 = 3\n");
    assert!(html.contains("rotation"));
    assert!(html.contains("lambda_1"));
    assert!(html.contains("ellipse"));
}

#[test]
fn la_conique_a_centre_decale_trouve_son_centre() {
    let html = rend("<Étudie>la conique d'équation x^2 + 4y^2 - 2x - 8y + 1 = 0\n");
    assert!(html.contains("(1 ; 1)"));
    assert!(html.contains("ellipse"));
}

#[test]
fn la_conique_degeneree_en_point_est_reconnue() {
    let html = rend("<Étudie>la conique d'équation x^2 + y^2 = 0\n");
    assert!(html.contains("point"));
}

#[test]
fn l_ensemble_vide_est_reconnu() {
    let html = rend("<Étudie>la conique d'équation x^2 + y^2 + 1 = 0\n");
    assert!(html.contains("vide"));
}

#[test]
fn les_saisies_pilotent_aussi_les_courbes() {
    let src = "soit n = <Saisis>un entier{Combien de pétales ?}\n<Trace>la courbe polaire r = cos(#n*t)\n";
    let mut e = Engine::new();
    assert!(!e.render(src, false).html.contains("class=\"courbe\""));
    e.saisies.insert("n".into(), "5".into());
    assert!(e.render(src, false).html.contains("class=\"courbe\""));
}

#[test]
fn la_section_courbes_de_geometrie4_est_complete() {
    let src = std::fs::read_to_string("../exemples/geometrie4.txt").unwrap();
    let mut e = Engine::new();
    let bloque = e.render(&src, false).html;
    assert!(bloque.matches("class=\"courbe\"").count() >= 4);
    assert!(bloque.matches("<svg").count() >= 8);
    assert!(!bloque.contains("non prise en charge"));
    assert!(bloque.contains("Combien de pétales"));
    e.saisies.insert("n".into(), "5".into());
    let complet = e.render(&src, false).html;
    assert!(complet.matches("<svg").count() > bloque.matches("<svg").count());
}
