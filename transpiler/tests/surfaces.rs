use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn la_selle_se_dessine_ombree() {
    let html = rend("<Trace>la surface z = x^2 - y^2 pour x dans [-2 ; 2] et y dans [-2 ; 2]\n");
    assert!(html.contains("<svg"));
    assert!(html.matches("fill=\"#").count() > 200);
    assert!(html.contains(">z</text>"));
    assert!(html.contains("url(#fleche)"));
}

#[test]
fn le_nombre_de_mailles_se_regle() {
    let grossier = rend("<Trace>la surface z = x*y pour x dans [-1 ; 1] et y dans [-1 ; 1], avec 8 mailles\n");
    let fin = rend("<Trace>la surface z = x*y pour x dans [-1 ; 1] et y dans [-1 ; 1], avec 32 mailles\n");
    assert!(fin.matches("fill=\"#").count() > grossier.matches("fill=\"#").count());
}

#[test]
fn les_lignes_de_niveau_sortent_avec_leurs_valeurs() {
    let html = rend("<Trace>les lignes de niveau de z = x^2 + y^2 pour x dans [-2 ; 2] et y dans [-2 ; 2], aux niveaux {1 ; 2 ; 3}\n");
    assert!(html.contains("<svg"));
    assert_eq!(html.matches("class=\"courbe\"").count(), 3);
    for e in [">1</text>", ">2</text>", ">3</text>"] {
        assert!(html.contains(e), "manque {e}");
    }
}

#[test]
fn les_niveaux_automatiques_se_deduisent() {
    let html = rend("<Trace>les lignes de niveau de z = x*y pour x dans [-2 ; 2] et y dans [-2 ; 2]\n");
    assert!(html.matches("class=\"courbe\"").count() >= 7);
}

#[test]
fn la_surface_suit_une_saisie() {
    let src = "soit k = <Saisis>un entier{Quel coefficient ?}\n<Trace>la surface z = #k*x*y pour x dans [-1 ; 1] et y dans [-1 ; 1]\n";
    let mut e = Engine::new();
    assert!(!e.render(src, false).html.contains("<svg"));
    e.saisies.insert("k".into(), "2".into());
    assert!(e.render(src, false).html.contains("<svg"));
}

#[test]
fn lagrange_redige_ses_candidats() {
    let src = "<Soit>la fonction f(x, y) = x*y\n<Détermine>les extremums de f sous la contrainte x^2 + y^2 = 2\n";
    let html = rend(src);
    assert!(html.contains("lagrangien"));
    assert!(html.contains("nabla"));
    assert!(html.contains("maximum sous la contrainte"));
    assert!(html.contains("minimum sous la contrainte"));
    assert!(html.contains("calcul-prose"));
}

#[test]
fn l_integrale_double_sur_rectangle_vaut_un() {
    let html = rend("<Calcule>l'intégrale double de x*y sur [0 ; 1] × [0 ; 2]\n");
    assert!(html.contains("Fubini"));
    assert!(html.contains("iint"));
    assert!(html.contains("= 1\\]"));
}

#[test]
fn l_integrale_double_sur_disque_passe_en_polaires() {
    let html = rend("<Calcule>l'intégrale double de x^2 + y^2 sur le disque de rayon 2\n");
    assert!(html.contains("polaires"));
    assert!(html.contains("jacobien"));
    assert!(html.contains("8 \\pi"));
}

#[test]
fn l_integrale_triple_se_calcule() {
    let html = rend("<Calcule>l'intégrale triple de x*y*z sur [0 ; 1] × [0 ; 1] × [0 ; 2]\n");
    assert!(html.contains("iiint"));
    assert!(html.contains("frac{1}{2}"));
}

#[test]
fn les_points_critiques_multivariables_restent_classes()
{
    let src = "<Soit>la fonction f(x, y) = x^2 + y^2 - 2*x\n<Détermine>les points critiques de f\n";
    let html = rend(src);
    assert!(html.contains("minimum local"));
}

#[test]
fn la_section_champs_d_analyse4_est_complete() {
    let src = std::fs::read_to_string("../exemples/analyse4.txt").unwrap();
    let mut e = Engine::new();
    let bloque = e.render(&src, false).html;
    assert!(bloque.matches("<svg").count() >= 4);
    assert!(bloque.contains("lagrangien"));
    assert!(bloque.contains("Fubini"));
    assert!(bloque.contains("polaires"));
    assert!(!bloque.contains("non prise en charge"));
    assert!(bloque.contains("Quel coefficient"));
    e.saisies.insert("k".into(), "3".into());
    let complet = e.render(&src, false).html;
    assert!(complet.matches("<svg").count() > bloque.matches("<svg").count());
}
