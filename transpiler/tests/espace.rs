use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn les_anciens_solides_restent_compris() {
    let html = rend("<Trace>le solide cube, d'arête 3 cm\n<Trace>le solide pyramide, de base 4 cm et de hauteur 3 cm\n");
    assert_eq!(html.matches("<svg").count(), 2);
    assert!(html.contains("stroke-dasharray"));
    assert!(html.contains("3 cm"));
    assert!(html.contains("4 cm"));
}

#[test]
fn le_pave_droit_porte_ses_trois_cotes() {
    let html = rend("<Trace>le solide pavé droit, de longueur 4 cm, de largeur 2 cm et de hauteur 3 cm\n");
    assert!(html.contains("<svg"));
    for mesure in ["4 cm", "2 cm", "3 cm"] {
        assert!(html.contains(mesure), "manque {mesure}");
    }
    assert!(html.contains("stroke-dasharray"));
    assert!(html.matches("transform=\"rotate(").count() >= 3);
}

#[test]
fn le_prisme_se_dessine() {
    let html = rend("<Trace>le solide prisme, de base 3 cm et de longueur 5 cm\n");
    assert!(html.contains("<svg"));
    assert!(html.contains("3 cm"));
    assert!(html.contains("5 cm"));
    assert!(html.contains("stroke-dasharray"));
}

#[test]
fn cylindre_cone_et_sphere_se_dessinent() {
    let html = rend("<Trace>le solide cylindre, de rayon 2 cm et de hauteur 5 cm\n<Trace>le solide cône, de rayon 2 cm et de hauteur 4 cm\n<Trace>le solide sphère, de rayon 3 cm\n");
    assert_eq!(html.matches("<svg").count(), 3);
    assert_eq!(html.matches("<circle").count() >= 3, true);
    assert!(html.contains("stroke-dasharray"));
    assert!(html.contains("2 cm"));
    assert!(html.contains("5 cm"));
}

#[test]
fn les_dimensions_acceptent_la_virgule() {
    let html = rend("<Trace>le solide cube, d'arête 2,5 cm\n");
    assert!(html.contains("2,5 cm"));
}

#[test]
fn les_patrons_se_deplient() {
    let html = rend("<Trace>le patron du cube d'arête 3 cm\n<Trace>le patron du pavé droit de longueur 4 cm, de largeur 2 cm et de hauteur 3 cm\n<Trace>le patron du cylindre de rayon 2 cm et de hauteur 5 cm\n<Trace>le patron du cône de rayon 2 cm et de hauteur 4 cm\n<Trace>le patron de la pyramide de base 4 cm et de hauteur 3 cm\n");
    assert_eq!(html.matches("<svg").count(), 5);
    assert!(html.contains("<circle"));
}

#[test]
fn le_repere_de_l_espace_place_les_points() {
    let src = "<Soit>les points A(1;2;3) et B(3;1;2)\n<Trace>dans un repère de l'espace les points A et B, le segment [AB] et le vecteur AB\n";
    let html = rend(src);
    assert!(html.contains("<svg"));
    assert!(html.contains(">A</text>"));
    assert!(html.contains(">B</text>"));
    assert!(html.contains(">x</text>"));
    assert!(html.contains(">z</text>"));
    assert!(html.contains("class=\"repere\""));
    assert!(html.contains("url(#pointe)"));
}

#[test]
fn le_repere_exige_des_points_declares() {
    let html = rend("<Trace>dans un repère de l'espace les points A et B\n");
    assert!(!html.contains("class=\"repere\""));
}

#[test]
fn les_calculs_de_l_espace_suivent_les_saisies() {
    let src = "soit a = <Saisis>un entier{Quelle arête pour le cube ?}\n<Trace>le solide cube, d'arête #a cm\n";
    let mut e = Engine::new();
    let bloque = e.render(src, false).html;
    assert!(bloque.contains("saisie-champ"));
    assert!(!bloque.contains("<svg"));
    e.saisies.insert("a".into(), "3".into());
    let html = e.render(src, false).html;
    assert!(html.contains("<svg"));
    assert!(html.contains("3 cm"));
}

#[test]
fn les_sections_espace_des_exemples_sont_completes() {
    let deux = std::fs::read_to_string("../exemples/geometrie2.txt").unwrap();
    let mut e = Engine::new();
    let bloque = e.render(&deux, false).html;
    assert!(bloque.matches("<svg").count() >= 12);
    assert!(bloque.contains("Quelle arête pour le cube"));
    e.saisies.insert("a".into(), "5".into());
    let complet = e.render(&deux, false).html;
    assert!(complet.matches("<svg").count() > bloque.matches("<svg").count());
    assert!(complet.contains("5 cm"));
    assert!(!complet.contains("non prise en charge"));
    assert!(complet.contains("125"));
    let trois = std::fs::read_to_string("../exemples/geometrie3.txt").unwrap();
    let html = Engine::new().render(&trois, false).html;
    assert!(html.matches("class=\"repere\"").count() >= 5);
    assert!(html.contains(">E</text>"));
    assert!(html.contains("url(#pointe)"));
    assert!(html.contains("x = 1 + t"));
    assert!(html.contains("#1e7d32"));
    assert!(html.contains("strictement parallèles"));
    assert!(html.contains("confondus"));
    assert!(html.contains("sécants"));
}

const DECLARATIONS: &str = "<Soit>la droite d passant par A(1;0;2) et de vecteur directeur u(1;1;-1)\n<Soit>la droite d' passant par B(0;1;3) et de vecteur directeur v(2;2;-2)\n<Soit>la droite g passant par C(3;2;0) et de vecteur directeur w(1;0;0)\n<Soit>la droite p passant par O(0;0;0) et de vecteur directeur m(1;1;0)\n<Soit>le plan P d'équation 2x + y - z = 3\n<Soit>le plan Q d'équation 4x + 2y - 2z = 6\n<Soit>le plan R d'équation x - y + z = 0\n";

#[test]
fn la_representation_parametrique_se_redige() {
    let src = format!("{DECLARATIONS}<Donne>une représentation paramétrique de la droite d\n");
    let html = rend(&src);
    assert!(html.contains("vecteur directeur"));
    assert!(html.contains("x = 1 + t"));
    assert!(html.contains("y = t"));
    assert!(html.contains("z = 2 - t"));
    assert!(html.contains("mathbb{R}"));
}

#[test]
fn deux_droites_paralleles_sont_reconnues() {
    let src = format!("{DECLARATIONS}<Étudie>la position relative des droites d et d'\n");
    let html = rend(&src);
    assert!(html.contains("colinéaires"));
    assert!(html.contains("strictement parallèles"));
}

#[test]
fn deux_droites_secantes_donnent_leur_point() {
    let src = format!("{DECLARATIONS}<Étudie>la position relative des droites d et g\n");
    let html = rend(&src);
    assert!(html.contains("sécantes au point"));
    assert!(html.contains("(3 ; 2 ; 0)"));
}

#[test]
fn deux_droites_non_coplanaires_sont_reconnues() {
    let src = "<Soit>la droite d passant par A(0;0;0) et de vecteur directeur u(1;0;0)\n<Soit>la droite h passant par B(0;0;1) et de vecteur directeur v(0;1;0)\n<Étudie>la position relative des droites d et h\n";
    let html = rend(src);
    assert!(html.contains("pas coplanaires"));
}

#[test]
fn droite_et_plan_secants_donnent_leur_point() {
    let src = format!("{DECLARATIONS}<Étudie>la position relative de la droite d et du plan R\n");
    let html = rend(&src);
    assert!(html.contains("cdot"));
    assert!(html.contains("sécants"));
    assert!(html.contains("coupe le plan"));
}

#[test]
fn droite_parallele_au_plan_est_reconnue() {
    let src = format!("{DECLARATIONS}<Étudie>la position relative de la droite p et du plan R\n");
    let html = rend(&src);
    assert!(html.contains("parallèle au plan"));
    assert!(html.contains("incluse"));
}

#[test]
fn plans_confondus_paralleles_et_secants() {
    let confondus = rend(&format!("{DECLARATIONS}<Étudie>la position relative des plans P et Q\n"));
    assert!(confondus.contains("confondus"));
    let secants = rend(&format!("{DECLARATIONS}<Étudie>la position relative des plans P et R\n"));
    assert!(secants.contains("sécants"));
    assert!(secants.contains("Delta"));
    assert!(secants.contains("wedge"));
}

#[test]
fn la_droite_se_trace_dans_le_repere() {
    let src = format!("{DECLARATIONS}<Trace>dans un repère de l'espace la droite d\n");
    let html = rend(&src);
    assert!(html.contains("<svg"));
    assert!(html.contains("#1e7d32"));
    assert!(html.contains(">d</text>"));
}
