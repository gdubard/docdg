//! Arbres et graphes — les structures de données du programme.
//!
//! Aucun type nouveau : un graphe est une **liste d'adjacence**, c'est-à-dire
//! un dictionnaire dont les valeurs sont des listes ; un arbre parfait tient
//! dans un tableau, les fils de `i` étant en `2i+1` et `2i+2` ; un arbre
//! chaîné se fait d'objets qui se contiennent. C'est le programme qui le
//! demande ainsi, et c'est ce que le langage portait déjà — à un verrou près,
//! levé ici : un attribut peut désormais contenir un objet.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

fn graphe() -> &'static str {
    "soit g: dictionnaire de chaînes de caractères et de listes de chaînes de caractères = \
     {A: {B ; C} ; B: {D} ; C: {D} ; D: {}}\n"
}

#[test]
fn un_graphe_est_une_liste_d_adjacence() {
    let html = rend(&format!("{}R : #{{longueur(g)}} sommets, voisins de A : #{{g[A]}}", graphe()));
    assert!(html.contains('4'), "{}", html);
    assert!(html.contains("{B ; C}"), "{}", html);
}

#[test]
fn les_sommets_se_parcourent() {
    let doc = rend(&format!("{}pour s dans g {{\n\t[#s]\n}}", graphe()));
    for s in ["[A]", "[B]", "[C]", "[D]"] {
        assert!(doc.contains(s), "{} manquant : {}", s, doc);
    }
}

#[test]
fn le_parcours_en_largeur_s_ecrit() {
    // la file donne l'ordre : A, puis ses voisins, puis les leurs
    let html = rend(&format!(
        "{}soit largeur(g: dictionnaire de chaînes de caractères et de listes de chaînes de caractères ; départ: chaîne de caractères): chaîne de caractères = {{\n\
         \tsoit vus: chaîne de caractères = \"\"\n\
         \tsoit f: file de chaînes de caractères = {{}}\n\
         \tsoit f = enfile(f ; départ)\n\
         \ttant que est vide(f) vaut faux faire {{\n\
         \t\tsoit s = tête(f)\n\
         \t\tsoit f = défile(f)\n\
         \t\tsi vus contient(s) vaut faux {{\n\
         \t\t\tsoit vus = vus + s\n\
         \t\t\tpour v dans g[s] {{\n\
         \t\t\t\tsoit f = enfile(f ; v)\n\
         \t\t\t}}\n\
         \t\t}}\n\
         \t}}\n\
         \tretourne vus\n\
         }}\n\
         R : #{{largeur(g ; \"A\")}}",
        graphe()
    ));
    assert!(html.contains("ABCD"), "{}", html);
}

#[test]
fn le_parcours_en_profondeur_s_ecrit() {
    // la pile donne l'autre ordre : on descend avant d'élargir
    let html = rend(&format!(
        "{}soit profondeur(g: dictionnaire de chaînes de caractères et de listes de chaînes de caractères ; départ: chaîne de caractères): chaîne de caractères = {{\n\
         \tsoit vus: chaîne de caractères = \"\"\n\
         \tsoit p: pile de chaînes de caractères = {{}}\n\
         \tsoit p = empile(p ; départ)\n\
         \ttant que est vide(p) vaut faux faire {{\n\
         \t\tsoit s = sommet(p)\n\
         \t\tsoit p = dépile(p)\n\
         \t\tsi vus contient(s) vaut faux {{\n\
         \t\t\tsoit vus = vus + s\n\
         \t\t\tpour v dans g[s] {{\n\
         \t\t\t\tsoit p = empile(p ; v)\n\
         \t\t\t}}\n\
         \t\t}}\n\
         \t}}\n\
         \tretourne vus\n\
         }}\n\
         R : #{{profondeur(g ; \"A\")}}",
        graphe()
    ));
    assert!(html.contains("ACD") || html.contains("ABD"), "{}", html);
}

#[test]
fn un_arbre_parfait_tient_dans_un_tableau() {
    let html = rend(
        "soit t: liste d'entiers = {1 ; 2 ; 3 ; 4 ; 5}\n\
         soit gauche(i: entier): entier = 2*i + 1\n\
         soit droit(i: entier): entier = 2*i + 2\n\
         R : racine #{t[0]}, fils #{t[gauche(0)]} et #{t[droit(0)]}",
    );
    assert!(html.contains("racine 1"), "{}", html);
    assert!(html.contains("fils 2 et 3"), "{}", html);
}

#[test]
fn un_attribut_peut_contenir_un_objet() {
    // le verrou levé : sans lui, ni arbre chaîné ni liste chaînée
    let html = rend(
        "soit une classe Feuille {\n\tv: entier\n}\n\
         soit une classe Branche {\n\
         \tfils: Feuille\n\
         \tsoit valeur(): entier = fils.v\n\
         }\n\
         soit f: Feuille = Feuille(7)\n\
         soit b: Branche = Branche(f)\n\
         R : #{b.valeur()}",
    );
    assert!(html.contains('7'), "{}", html);
    assert!(!html.contains("calcul-absent"), "{}", html);
}

#[test]
fn une_collection_d_objets_tient_dans_un_objet() {
    let html = rend(
        "soit une classe Sommet {\n\tnom: chaîne de caractères\n}\n\
         soit une classe Graphe {\n\
         \tsommets: liste de Sommet\n\
         \tsoit ordre(): entier = longueur(sommets)\n\
         }\n\
         soit g: Graphe = Graphe({Sommet(\"A\") ; Sommet(\"B\")})\n\
         R : #{g.ordre()}",
    );
    assert!(html.contains('2'), "{}", html);
}

#[test]
fn on_parcourt_ce_qu_une_lecture_a_rendu() {
    // `pour v dans g[s]` : la source d'une boucle n'est pas forcément un nom
    let html = rend(&format!(
        "{}soit degré(g: dictionnaire de chaînes de caractères et de listes de chaînes de caractères ; s: chaîne de caractères): entier = {{\n\
         \tsoit n = 0\n\
         \tpour v dans g[s] {{\n\
         \t\tsoit n = n + 1\n\
         \t}}\n\
         \tretourne n\n\
         }}\n\
         R : #{{degré(g ; \"A\")}}",
        graphe()
    ));
    assert!(html.contains('2'), "{}", html);
}

#[test]
fn un_arbre_binaire_recursif_s_ecrit_sans_valeur_nulle() {
    // L'arbre vide n'a pas besoin d'un « rien » : c'est une classe. La
    // hiérarchie dit tout — un arbre est vide ou c'est un nœud.
    let arbre = "soit une classe abstraite Arbre {\n\
                 \tsoit taille(): entier\n\
                 }\n\
                 soit une classe Vide qui hérite de la classe Arbre {\n\
                 \tsoit taille(): entier = 0\n\
                 }\n\
                 soit une classe Nœud qui hérite de la classe Arbre {\n\
                 \tvaleur: entier\n\
                 \tgauche: Arbre\n\
                 \tdroit: Arbre\n\
                 \n\
                 \tsoit taille(): entier = 1 + gauche.taille() + droit.taille()\n\
                 }\n";
    assert!(rend(&format!("{}soit v: Vide = Vide()\nR : #{{v.taille()}}", arbre)).contains('0'));
    let trois = rend(&format!(
        "{}soit t: Nœud = Nœud(1 ; Nœud(2 ; Vide() ; Vide()) ; Nœud(3 ; Vide() ; Vide()))\nR : #{{t.taille()}}",
        arbre
    ));
    assert!(trois.contains('3'), "{}", trois);
    assert!(!trois.contains("calcul-absent"), "{}", trois);
}
