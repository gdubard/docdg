//! Piles et files — les structures nommées du programme de terminale.
//!
//! Elles s'écrivent aussi à la main, et `algo4` montre comment : le programme
//! demande que l'élève les implémente avant de les employer. Ce qui suit est
//! l'emploi.
//!
//! Chaque opération rend une **nouvelle** structure : `soit p = empile(p ; 3)`
//! montre que la pile d'après n'est pas celle d'avant. Rien n'est modifié sur
//! place, comme partout ailleurs dans docdg.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn une_pile_se_declare_et_se_lit() {
    let html = rend("soit p: pile d'entiers = {1 ; 2 ; 3}\nR : #p, sommet #{sommet(p)}");
    assert!(html.contains("{1 ; 2 ; 3}"), "{}", html);
    // le sommet est le dernier écrit : dernier entré, premier sorti
    assert!(html.contains("sommet 3"), "{}", html);
}

#[test]
fn empiler_et_depiler() {
    let html = rend(
        "soit p: pile d'entiers = {}\n\
         soit p = empile(p ; 1)\n\
         soit p = empile(p ; 2)\n\
         R : #p sommet #{sommet(p)} puis #{dépile(p)}",
    );
    assert!(html.contains("{1 ; 2}"), "{}", html);
    assert!(html.contains("sommet 2"), "{}", html);
    assert!(html.contains("puis {1}"), "{}", html);
}

#[test]
fn enfiler_et_defiler() {
    let html = rend(
        "soit f: file d'entiers = {}\n\
         soit f = enfile(f ; 1)\n\
         soit f = enfile(f ; 2)\n\
         R : #f tête #{tête(f)} puis #{défile(f)}",
    );
    // la tête est le premier entré
    assert!(html.contains("tête 1"), "{}", html);
    assert!(html.contains("puis {2}"), "{}", html);
}

#[test]
fn est_vide_repond_par_oui_ou_non() {
    assert!(rend("soit p: pile d'entiers = {}\nR : #{est vide(p)}").contains("vrai"));
    assert!(rend("soit p: pile d'entiers = {1}\nR : #{est vide(p)}").contains("faux"));
}

#[test]
fn la_forme_prepositionnelle_vaut_aussi_ici() {
    assert!(rend("soit p: pile d'entiers = {1}\nR : #{dans p empile(9)}").contains("{1 ; 9}"));
}

#[test]
fn la_discipline_est_tenue() {
    // une file n'a pas de sommet, une pile n'a pas de tête
    assert!(rend("soit p: pile d'entiers = {1 ; 2}\nR : #{tête(p)}").contains("attend une file"));
    assert!(rend("soit f: file d'entiers = {1}\nR : #{sommet(f)}").contains("attend une pile"));
    // et ni l'une ni l'autre ne s'indexe : c'est tout leur intérêt
    assert!(rend("soit p: pile d'entiers = {1 ; 2}\nR : #{p[0]}").contains("ne s'indexe pas"));
    // ni ne se confond avec une liste
    assert!(rend("soit v: liste d'entiers = {1}\nsoit p: pile d'entiers = v\nR : #p").contains('⚠'));
}

#[test]
fn une_structure_vide_n_a_ni_sommet_ni_tete() {
    assert!(rend("soit p: pile d'entiers = {}\nR : #{sommet(p)}").contains("n'a pas de sommet"));
    assert!(rend("soit p: pile d'entiers = {}\nR : #{dépile(p)}").contains("vide"));
}

#[test]
fn une_pile_se_passe_en_argument_et_se_mesure() {
    let html = rend(
        "soit hauteur(p: pile d'entiers): entier = longueur(p)\n\
         soit p: pile d'entiers = {1 ; 2 ; 3}\n\
         R : #{hauteur(p)}",
    );
    assert!(html.contains('3'), "{}", html);
}

#[test]
fn une_pile_se_vide_dans_une_fonction() {
    let html = rend(
        "soit total(p: pile d'entiers): entier = {\n\
         \tsoit s = 0\n\
         \ttant que est vide(p) vaut faux faire {\n\
         \t\tsoit s = s + sommet(p)\n\
         \t\tsoit p = dépile(p)\n\
         \t}\n\
         \tretourne s\n\
         }\n\
         R : #{total({1 ; 2 ; 3})}",
    );
    assert!(html.contains('6'), "{}", html);
}

#[test]
fn une_pile_traverse_une_boucle_de_document() {
    let html = rend(
        "soit p: pile d'entiers = {1 ; 2 ; 3}\npour k de 1 à 2 {\n\tsoit p = dépile(p)\n}\nR : #p",
    );
    assert!(html.contains("{1}"), "{}", html);
}

#[test]
fn une_fonction_ecrite_a_la_main_l_emporte_sur_la_primitive() {
    // l'exercice du programme : écrire sa propre pile sur une liste. Les noms
    // se recouvrent avec ceux du langage, et ce sont ceux de l'élève qui
    // valent — sans quoi l'exercice deviendrait impossible à écrire.
    let html = rend(
        "soit sommet(p: liste d'entiers): entier = p[longueur(p) - 1]\n\
         soit dépile(p: liste d'entiers): liste d'entiers = p[0 à longueur(p) - 2]\n\
         soit p: liste d'entiers = {1 ; 2 ; 3}\n\
         R : #{sommet(p)} puis #{dépile(p)}",
    );
    assert!(html.contains('3'), "{}", html);
    assert!(html.contains("{1 ; 2}"), "{}", html);
    assert!(!html.contains("calcul-absent"), "{}", html);
}

#[test]
fn on_ne_parcourt_pas_une_pile() {
    // le silence serait pire que le refus : l'élève croirait la pile vide
    let html = rend(
        "soit f(p: pile d'entiers): entier = {\n\
         \tsoit n = 0\n\
         \tpour x dans p {\n\
         \t\tsoit n = n + 1\n\
         \t}\n\
         \tretourne n\n\
         }\n\
         R : #{f({1 ; 2})}",
    );
    assert!(html.contains("on ne parcourt pas une pile"), "{}", html);
    assert!(html.contains("dépilant"), "l'issue doit être dite : {}", html);
}
