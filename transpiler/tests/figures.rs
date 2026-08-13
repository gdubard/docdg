//! Les figures planes de `<Trace>` : le vocabulaire du manuel, tenu par le
//! moteur.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

fn sans_erreur(src: &str) {
    let h = rend(src);
    assert!(!h.contains("calcul-absent"), "{} : {}", src.lines().next().unwrap_or(""), h);
}

#[test]
fn le_triangle_rectangle_se_place_a_l_angle_droit() {
    sans_erreur("<Trace>le triangle ABC rectangle en A, de côté AB 3 cm et de côté AC 4 cm");
    sans_erreur("<Trace>le triangle ABC rectangle en A, de côté AB 30 mm et de côté AC 40 mm, avec les marques");
}

#[test]
fn l_isocele_a_deux_cotes_egaux() {
    sans_erreur("<Trace>le triangle ABC isocèle en A, de côté 5 cm");
}

#[test]
fn le_segment_et_la_demi_droite_portent_leur_longueur() {
    sans_erreur("<Trace>le segment [AB] tel que AB = 4 cm");
    sans_erreur("<Trace>le segment de droite [CD] tel que CD = 28 mm");
    sans_erreur("<Trace>la demi-droite (AB] telle que AB = 3,5 cm");
}

#[test]
fn le_cercle_se_donne_par_rayon_diametre_ou_reference() {
    sans_erreur("<Trace>le cercle O, de rayon 3 cm");
    sans_erreur("<Trace>le cercle O, de diamètre 6 cm");
    sans_erreur("<Soit>les points A(0;0) et B(3;0)\n<Trace>le cercle O, de rayon AB");
}

#[test]
fn le_rayon_par_reference_exige_ses_points() {
    let h = rend("<Trace>le cercle O, de rayon AB");
    assert!(h.contains("demande les points A et B"), "{}", h);
}

#[test]
fn le_point_seul_reprend_sa_declaration() {
    sans_erreur("<Soit>un point A(2;3)\n<Trace>le point A");
    let h = rend("<Trace>le point A");
    assert!(h.contains("n'a pas été déclaré"), "{}", h);
}

#[test]
fn le_bloc_groupe_sans_changer_le_vocabulaire() {
    sans_erreur("<Trace>{\n\tle point A(1;2)\n\tle point B(3;1)\n\tle segment [AB]\n}");
    sans_erreur("<Trace>{\n\tle solide cube ABCDEFGH, d'arête 3 cm\n\tle solide sphère, de rayon 2 cm\n}");
}

#[test]
fn le_repere_plan_accueille_les_elements() {
    sans_erreur("<Trace>dans un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5] {\n\tun point A(-3;1)\n\tun point B(3;-1)\n\tla médiatrice de [AB], en bleu\n}");
    sans_erreur("<Trace>dans un repère orthonormé où l'abscisse appartient à [-4 ; 4] et l'ordonnée à [-3 ; 3], avec une unité de 1,5 cm {\n\tle point A(1;2)\n}");
}
