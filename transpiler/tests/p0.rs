//! Chantier P0 — une fonction reçoit et rend autre chose que des nombres.
//!
//! Jusqu'ici, `appelle` rendait un `f64` et les paramètres étaient liés dans
//! une table de nombres : aucun tri, aucune recherche, aucun texte manipulé ne
//! s'écrivait. Ces tests fixent la levée de cette contrainte.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn une_fonction_recoit_une_collection() {
    let html = rend(
        "soit notes: une liste de réels = {12 ; 15 ; 9}\n\
         soit total(v: une liste de réels): un réel = {\n\
         \tretourne v[0] + v[1] + v[2]\n\
         }\n\
         Total : #{total(notes)}",
    );
    assert!(html.contains("36"), "{}", html);
}

#[test]
fn une_fonction_rend_une_collection() {
    let html = rend(
        "soit doublons(v: une liste d'entiers): une liste d'entiers = {\n\
         \tretourne v + v\n\
         }\n\
         soit s: une liste d'entiers = {1 ; 2}\n\
         Résultat : #{doublons(s)}",
    );
    assert!(html.contains("1 ; 2 ; 1 ; 2"), "{}", html);
}

#[test]
fn une_collection_d_entiers_passe_pour_une_collection_de_reels() {
    // la covariance : le passage se fait par copie, elle est donc saine
    let html = rend(
        "soit premier(v: une liste de réels): un réel = {\n\
         \tretourne v[0]\n\
         }\n\
         soit s: une liste d'entiers = {7 ; 8}\n\
         Premier : #{premier(s)}",
    );
    assert!(html.contains("7"), "{}", html);
}

#[test]
fn l_appelant_n_est_pas_modifie() {
    let html = rend(
        "soit ajoute(v: une liste d'entiers): une liste d'entiers = {\n\
         \tretourne v + {99}\n\
         }\n\
         soit s: une liste d'entiers = {1 ; 2}\n\
         Copie : #{ajoute(s)}\n\
         Origine : #s",
    );
    assert!(html.contains("1 ; 2 ; 99"), "{}", html);
    assert!(html.contains("Origine"), "{}", html);
    // la série de départ n'a pas bougé
    let apres_origine = html.split("Origine").nth(1).unwrap_or("");
    assert!(!apres_origine.contains("99"), "{}", html);
}

#[test]
fn une_fonction_manipule_du_texte() {
    let html = rend(
        "soit joint(a: chaîne de caractères ; b: chaîne de caractères): chaîne de caractères = {\n\
         \tretourne a + b\n\
         }\n\
         Mot : #{joint(bon ; jour)}",
    );
    assert!(html.contains("bonjour"), "{}", html);
}

#[test]
fn un_litteral_passe_directement_en_argument() {
    let html = rend(
        "soit tete(v: une liste d'entiers): un entier = {\n\
         \tretourne v[0]\n\
         }\n\
         Tête : #{tete({4 ; 5 ; 6})}",
    );
    assert!(html.contains("4"), "{}", html);
}

#[test]
fn le_type_d_argument_est_verifie() {
    let html = rend(
        "soit tete(v: une liste d'entiers): un entier = {\n\
         \tretourne v[0]\n\
         }\n\
         Tête : #{tete(3)}",
    );
    // le message nomme le type attendu, avec son article accordé
    assert!(html.contains("une liste d'entiers"), "{}", html);
}

#[test]
fn la_copie_locale_permet_le_tri_par_insertion() {
    let html = rend(
        "soit tri_par_insertion(v: une liste de réels): une liste de réels = {\n\
         \tsoit t = v\n\
         \tt[0] = 42\n\
         \tretourne t\n\
         }\n\
         soit s: une liste d'entiers = {1 ; 2 ; 3}\n\
         Trié : #{tri_par_insertion(s)}\n\
         Origine : #s",
    );
    assert!(html.contains("42"), "{}", html);
}

// ═══════════ les boucles dans un corps de fonction ═══════════
//
// Régression fondatrice : `expand_loops_avec` et son homologue pour
// `tant que` balayaient toutes les lignes du document sans savoir qu'une
// ligne peut appartenir au corps d'une fonction. La coupure tombait au
// milieu d'une déclaration, dont l'accolade fermante restait de l'autre
// côté : la fonction n'était pas enregistrée (`pour`), ou l'était amputée
// de sa boucle (`tant que`).

#[test]
fn une_boucle_pour_de_a_tourne_dans_un_corps() {
    let html = rend(
        "soit somme_jusqu_a(n: entier): entier = {\n\
         \tsoit s = 0\n\
         \tpour i de 1 à n {\n\
         \t\tsoit s = s + i\n\
         \t}\n\
         \tretourne s\n\
         }\n\
         R : #{somme_jusqu_a(10)}",
    );
    assert!(html.contains("55"), "{}", html);
}

#[test]
fn une_boucle_pour_dans_parcourt_une_collection_recue() {
    let html = rend(
        "soit total(v: une liste de réels): un réel = {\n\
         \tsoit s = 0\n\
         \tpour x dans v {\n\
         \t\tsoit s = s + x\n\
         \t}\n\
         \tretourne s\n\
         }\n\
         R : #{total({12 ; 15 ; 9})}",
    );
    assert!(html.contains("36"), "{}", html);
}

#[test]
fn une_boucle_tant_que_tourne_dans_un_corps() {
    let html = rend(
        "soit compte(n: entier): entier = {\n\
         \tsoit s = 0\n\
         \ttant que s moins de n faire {\n\
         \t\tsoit s = s + 1\n\
         \t}\n\
         \tretourne s\n\
         }\n\
         R : #{compte(7)}",
    );
    assert!(html.contains("7"), "{}", html);
}

#[test]
fn longueur_mesure_un_conteneur() {
    let html = rend(
        "soit taille(v: une liste d'entiers): un entier = longueur(v)\n\
         R : #{taille({4 ; 5 ; 6})}",
    );
    assert!(html.contains("3"), "{}", html);
}

#[test]
fn la_moyenne_s_ecrit_enfin() {
    let html = rend(
        "soit moyenne(v: une liste de réels): un réel = {\n\
         \tsoit s = 0\n\
         \tpour x dans v {\n\
         \t\tsoit s = s + x\n\
         \t}\n\
         \tretourne s / longueur(v)\n\
         }\n\
         R : #{moyenne({12 ; 15 ; 9})}",
    );
    assert!(html.contains("12"), "{}", html);
}

#[test]
fn la_recherche_lineaire_s_ecrit_enfin() {
    let html = rend(
        "soit recherche(v: une liste d'entiers ; x: entier): un entier = {\n\
         \tsoit trouve = -1\n\
         \tpour i de 0 à longueur(v) - 1 {\n\
         \t\tsi v[i] vaut x {\n\
         \t\t\tsoit trouve = i\n\
         \t\t}\n\
         \t}\n\
         \tretourne trouve\n\
         }\n\
         R : #{recherche({4 ; 8 ; 15 ; 16} ; 15)}",
    );
    assert!(html.contains("2"), "{}", html);
}

#[test]
fn une_condition_dans_une_boucle_dans_un_corps() {
    let html = rend(
        "soit pairs(n: entier): entier = {\n\
         \tsoit c = 0\n\
         \tpour i de 1 à n {\n\
         \t\tsi i % 2 vaut 0 {\n\
         \t\t\tsoit c = c + 1\n\
         \t\t}\n\
         \t}\n\
         \tretourne c\n\
         }\n\
         R : #{pairs(10)}",
    );
    assert!(html.contains("5"), "{}", html);
}

#[test]
fn les_boucles_du_document_ne_sont_pas_affectees() {
    // le saut ne doit sauter *que* les déclarations de fonction
    let html = rend(
        "soit s: une liste d'entiers = {}\n\
         pour k de 1 à 4 {\n\
         \tsoit s = s + {k}\n\
         }\n\
         R : #s",
    );
    assert!(html.contains("1 ; 2 ; 3 ; 4"), "{}", html);
}

// ═══════════ les primitives de conteneur ═══════════

fn prim(expression: &str) -> String {
    rend(&format!(
        "soit v: une liste d'entiers = {{3 ; 1 ; 2}}\nR : #{{{}}}",
        expression
    ))
}

#[test]
fn les_primitives_de_collection() {
    for (expression, attendu) in [
        ("longueur(v)", "3"),
        ("tri(v)", "1 ; 2 ; 3"),
        ("inverse(v)", "2 ; 1 ; 3"),
        ("somme(v)", "6"),
        ("min(v)", "1"),
        ("max(v)", "3"),
        ("contient(2 ; v)", "vrai"),
        ("indice de(2 ; v)", "2"),
        ("insère(v ; 0 ; 9)", "9 ; 3 ; 1 ; 2"),
        ("supprime(v ; 1)", "3 ; 2"),
        ("ajoute(v ; 7)", "3 ; 1 ; 2 ; 7"),
        // la forme française : le conteneur précède la primitive
        ("v contient(2)", "vrai"),
        ("v indice de(2)", "2"),
        ("v insère(0 ; 9)", "9 ; 3 ; 1 ; 2"),
        ("v ajoute(7)", "3 ; 1 ; 2 ; 7"),
    ] {
        let html = prim(expression);
        assert!(html.contains(attendu), "{} → {}", expression, html);
    }
}

#[test]
fn min_et_max_a_deux_arguments_restent_mathematiques() {
    // la primitive de collection ne doit pas manger la fonction existante
    assert!(prim("min(7 ; 4)").contains('4'));
    assert!(prim("max(7 ; 4)").contains('7'));
}

#[test]
fn une_primitive_accepte_un_litteral() {
    assert!(prim("tri({5 ; 1 ; 3})").contains("1 ; 3 ; 5"));
}

#[test]
fn le_tri_des_textes_suit_la_collation_francaise() {
    // en codepoints bruts, « é » vient après « z » : « Zoé » précéderait
    // « école ». La comparaison porte d'abord sur les lettres dépouillées.
    let html = rend("R : #{tri({Zoé ; école ; avion})}");
    assert!(html.contains("avion ; école ; Zoé"), "{}", html);
}

#[test]
fn la_tranche_a_ses_deux_bornes_incluses() {
    assert!(prim("v[1 à 2]").contains("1 ; 2"));
    assert!(prim("v[0 à 0]").contains('3'));
}

#[test]
fn une_tranche_a_l_envers_est_vide_et_non_fautive() {
    // sans cette règle, la fusion de deux listes triées perd sa dernière ligne
    let html = prim("v[2 à 1]");
    assert!(html.contains("{}"), "{}", html);
    assert!(!html.contains("calcul-absent"), "{}", html);
}

#[test]
fn une_primitive_dit_ce_qu_elle_attendait() {
    let html = prim("indice de(9 ; v)");
    assert!(html.contains("ne figure pas"), "{}", html);
}

#[test]
fn les_primitives_servent_dans_un_corps_de_fonction() {
    let html = rend(
        "soit médiane_basse(v: une liste de réels): un réel = {\n\
         \tsoit t = tri(v)\n\
         \tretourne t[longueur(t) - 3]\n\
         }\n\
         R : #{médiane_basse({7 ; 1 ; 5 ; 3 ; 9})}",
    );
    assert!(html.contains('5'), "{}", html);
}

#[test]
fn la_fusion_de_deux_listes_triees_s_ecrit_enfin() {
    let html = rend(
        "soit fusion(a: une liste de réels ; b: une liste de réels): une liste de réels = {\n\
         \tsoit r: une liste de réels = {}\n\
         \tsoit i = 0\n\
         \tsoit j = 0\n\
         \ttant que i moins de longueur(a) et j moins de longueur(b) faire {\n\
         \t\tsi a[i] au plus b[j] {\n\
         \t\t\tsoit r = r + {a[i]}\n\
         \t\t\tsoit i = i + 1\n\
         \t\t} sinon {\n\
         \t\t\tsoit r = r + {b[j]}\n\
         \t\t\tsoit j = j + 1\n\
         \t\t}\n\
         \t}\n\
         \tretourne r + a[i à longueur(a) - 1] + b[j à longueur(b) - 1]\n\
         }\n\
         R : #{fusion({1 ; 4 ; 7} ; {2 ; 3 ; 8})}",
    );
    assert!(html.contains("1 ; 2 ; 3 ; 4 ; 7 ; 8"), "{}", html);
}


// ═══════════ la chaîne de caractères comme valeur ═══════════

#[test]
fn une_chaine_scalaire_se_declare_et_se_mesure() {
    let html = rend(
        "soit m: chaîne de caractères = \"bonjour tout le monde\"\n#m — #{longueur(m)} lettres",
    );
    assert!(html.contains("bonjour tout le monde"), "{}", html);
    assert!(html.contains("21"), "{}", html);
}

#[test]
fn les_guillemets_delimitent_sans_appartenir_a_la_valeur() {
    let html = rend("soit m: une chaîne de caractères = \"salut\"\n#m");
    assert!(html.contains("salut"), "{}", html);
    assert!(!html.contains('"'), "les guillemets ne doivent pas ressortir : {}", html);
}

#[test]
fn une_chaine_se_passe_en_argument() {
    let html = rend(
        "soit joint(a: chaîne de caractères ; b: chaîne de caractères): chaîne de caractères = a + b\n\
         soit d: une chaîne de caractères = \"bon\"\n\
         R : #{joint(d ; \"jour\")}",
    );
    assert!(html.contains("bonjour"), "{}", html);
}

#[test]
fn liste_est_le_mot_du_programme() {
    for mot in ["une liste d'entiers", "une liste d'entiers"] {
        let html = rend(&format!("soit v: {} = {{1 ; 2}}\n#v", mot));
        assert!(html.contains("1 ; 2"), "{} → {}", mot, html);
    }
}

#[test]
fn la_negation_et_les_booleens_litteraux() {
    assert!(rend("soit a = 1\nsi non (a vaut 2) {\n\tOUI\n}").contains("OUI"));
    assert!(rend("si vrai {\n\tOUI\n}").contains("OUI"));
    assert!(!rend("si faux {\n\tOUI\n}").contains("OUI"));
}

#[test]
fn retourne_est_le_seul_mot_de_retour() {
    // `renvoie` a été retiré : deux mots pour une même action obligent le
    // lecteur à se demander s'ils diffèrent
    let html = rend("soit f(a: entier): entier = {\n\tsoit x = a + 1\n\tretourne x\n}\nR : #{f(3)}");
    assert!(html.contains('4'), "{}", html);
}

#[test]
fn le_quotient_entier_permet_la_dichotomie() {
    let html = rend(
        "soit dichotomie(v: une liste d'entiers ; x: entier): un entier = {\n\
         \tsoit g = 0\n\
         \tsoit d = longueur(v) - 1\n\
         \ttant que g moins de d faire {\n\
         \t\tsoit m = quotient(g + d ; 2)\n\
         \t\tsi v[m] moins de x {\n\
         \t\t\tsoit g = m + 1\n\
         \t\t} sinon {\n\
         \t\t\tsoit d = m\n\
         \t\t}\n\
         \t}\n\
         \tretourne g\n\
         }\n\
         R : #{dichotomie({2 ; 3 ; 5 ; 7 ; 11 ; 13} ; 11)}",
    );
    assert!(html.contains('4'), "{}", html);
}

// ═══════════ les trois verrous d'avant-publication ═══════════

#[test]
fn une_primitive_sert_dans_une_condition_de_document() {
    // `expand_conditions` n'avait accès ni aux conteneurs ni aux fonctions :
    // la même écriture marchait dans un corps de fonction, pas dans le document
    let oui = rend("soit v: une liste d'entiers = {1 ; 2}\nsi v contient(1) {\n\tOUI\n}");
    assert!(oui.contains("OUI"), "{}", oui);
    let non = rend(
        "soit v: une liste d'entiers = {1 ; 2}\nsi v contient(9) {\n\tOUI\n} sinon {\n\tNON\n}",
    );
    assert!(non.contains("NON") && !non.contains("OUI"), "{}", non);
    assert!(rend("soit v: une liste d'entiers = {1 ; 2}\nsi longueur(v) vaut 2 {\n\tOUI\n}")
        .contains("OUI"));
}

#[test]
fn sortir_arrete_une_boucle_de_document() {
    let html = rend("pour k de 1 à 5 {\n\tsi k plus de 2 {\n\t\tsortir\n\t}\n\tk=#k\n}");
    assert!(html.contains("k=1") && html.contains("k=2"), "{}", html);
    assert!(!html.contains("k=3"), "la boucle devait s'arrêter : {}", html);
}

#[test]
fn sortir_arrete_une_boucle_sans_quitter_la_fonction() {
    let html = rend(
        "soit premier_pair(v: une liste d'entiers): un entier = {\n\
         \tsoit t = -1\n\
         \tpour x dans v {\n\
         \t\tsi x % 2 vaut 0 {\n\
         \t\t\tsoit t = x\n\
         \t\t\tsortir\n\
         \t\t}\n\
         \t}\n\
         \tretourne t\n\
         }\n\
         R : #{premier_pair({3 ; 5 ; 8 ; 10})}",
    );
    assert!(html.contains('8'), "{}", html);
    assert!(!html.contains("10"), "sortir devait couper avant 10 : {}", html);
}

// ═══════════ les tournures qui se lisent ═══════════

#[test]
fn la_preposition_dit_qui_recoit_l_operation() {
    let base = "soit notes: une liste d'entiers = {12 ; 15 ; 9}\nR : ";
    for (ecriture, attendu) in [
        ("dans notes insère(0 ; 20)", "20 ; 12 ; 15 ; 9"),
        ("dans notes supprime(1)", "12 ; 9"),
        ("dans notes ajoute(7)", "12 ; 15 ; 9 ; 7"),
        ("dans notes indice de(15)", "1"),
        ("notes contient(15)", "vrai"),
        // la forme fonctionnelle reste admise
        ("insère(notes ; 0 ; 20)", "20 ; 12 ; 15 ; 9"),
    ] {
        let html = rend(&format!("{}#{{{}}}", base, ecriture));
        assert!(html.contains(attendu), "{} → {}", ecriture, html);
    }
}

#[test]
fn la_division_euclidienne_se_dit_comme_au_college() {
    assert!(rend("R : #{quotient de 17 par 5}").contains('3'));
    assert!(rend("R : #{reste de 17 par 5}").contains('2'));
    // la tournure supporte un appel comme dividende
    let html = rend(
        "soit notes: une liste d'entiers = {12 ; 15 ; 9}\nR : #{quotient de longueur(notes) par 2}",
    );
    assert!(html.contains('1'), "{}", html);
    // et la forme brève ne disparaît pas
    assert!(rend("R : #{quotient(17 ; 5)}").contains('3'));
}

// ═══════════ chantier 1/3 — la chaîne de caractères ═══════════

#[test]
fn une_lettre_se_lit_par_son_indice() {
    let m = "soit m: chaîne de caractères = \"Bonjour\"\n";
    assert!(rend(&format!("{}R : #{{m[0]}}", m)).contains('B'));
    assert!(rend(&format!("{}R : #{{m[6]}}", m)).contains('r'));
    // une lettre est une chaîne d'un seul caractère : pas de type distinct
    assert!(rend(&format!("{}R : #{{longueur(m[0])}}", m)).contains('1'));
}

#[test]
fn un_indice_hors_bornes_compte_en_lettres() {
    let html = rend("soit m: une chaîne de caractères = \"Bonjour\"\nR : #{m[9]}");
    assert!(html.contains("lettre(s)"), "{}", html);
}

#[test]
fn une_chaine_se_parcourt_lettre_a_lettre() {
    let doc = rend("soit m: une chaîne de caractères = \"abc\"\npour c dans m {\n\t[#c]\n}");
    assert!(doc.contains("[a]") && doc.contains("[b]") && doc.contains("[c]"), "{}", doc);
    let fonction = rend(
        "soit compte(t: une chaîne de caractères): un entier = {\n\
         \tsoit n = 0\n\
         \tpour c dans t {\n\
         \t\tsoit n = n + 1\n\
         \t}\n\
         \tretourne n\n\
         }\n\
         R : #{compte(\"bonjour\")}",
    );
    assert!(fonction.contains('7'), "{}", fonction);
}

#[test]
fn les_primitives_de_chaine() {
    // la typographie française accentue les capitales
    assert!(rend("R : #{majuscule(\"été\")}").contains("ÉTÉ"));
    assert!(rend("R : #{minuscule(\"BONJOUR\")}").contains("bonjour"));
    assert!(rend("R : #{sans accents(\"élève\")}").contains("eleve"));
    assert!(rend("R : #{code(\"B\")}").contains("66"));
    assert!(rend("R : #{caractère(97)}").contains('a'));
    assert!(rend("soit m: une chaîne de caractères = \"Bonjour\"\nR : #{m contient(\"jour\")}").contains("vrai"));
}

#[test]
fn les_conversions_entre_texte_et_nombre() {
    assert!(rend("R : #{texte(42)}").contains("42"));
    assert!(rend("R : #{nombre(\"1,5\") + 1}").contains("2,5"));
}

#[test]
fn le_chiffre_de_cesar_s_ecrit_enfin() {
    let html = rend(
        "soit césar(t: une chaîne de caractères ; k: entier): une chaîne de caractères = {\n\
         \tsoit r: une chaîne de caractères = \"\"\n\
         \tpour c dans t {\n\
         \t\tsi code(c) au moins code(\"a\") et code(c) au plus code(\"z\") {\n\
         \t\t\tsoit r = r + caractère(code(\"a\") + reste de code(c) - code(\"a\") + k par 26)\n\
         \t\t} sinon {\n\
         \t\t\tsoit r = r + c\n\
         \t\t}\n\
         \t}\n\
         \tretourne r\n\
         }\n\
         R : #{césar(\"zoo\" ; 1)}",
    );
    // le repli en fin d'alphabet tient parce que le reste d'une division par
    // un nombre positif est positif
    assert!(html.contains("app"), "{}", html);
}

#[test]
fn le_palindrome_se_reconnait() {
    let base = "soit palindrome(t: une chaîne de caractères): un booléen = {\n\
                \tsoit u = minuscule(sans accents(t))\n\
                \tsi u vaut inverse(u) { retourne 1 } sinon { retourne 0 }\n\
                }\n";
    assert!(rend(&format!("{}R : #{{palindrome(\"Ressasser\")}}", base)).contains('1'));
    assert!(rend(&format!("{}R : #{{palindrome(\"bonjour\")}}", base)).contains('0'));
}

#[test]
fn une_lecture_indexee_sert_d_argument() {
    // `code(m[0])` et `ajoute(r ; a[i])` : l'argument est lu, non recopié
    assert!(rend("soit m: une chaîne de caractères = \"Bonjour\"\nR : #{code(m[0])}").contains("66"));
    let html = rend(
        "soit copie(a: une liste d'entiers): une liste d'entiers = {\n\
         \tsoit r: une liste d'entiers = {}\n\
         \tsoit i = 0\n\
         \ttant que i moins de longueur(a) faire {\n\
         \t\tsoit r = ajoute(r ; a[i])\n\
         \t\tsoit i = i + 1\n\
         \t}\n\
         \tretourne r\n\
         }\n\
         R : #{copie({4 ; 5 ; 6})}",
    );
    assert!(html.contains("4 ; 5 ; 6"), "{}", html);
}

// ═══════════ chantier 2/3 — la cohérence des conteneurs ═══════════

#[test]
fn une_cle_de_dictionnaire_peut_etre_une_variable() {
    let html = rend(
        "soit total(d: un dictionnaire de chaînes de caractères et d'entiers): un entier = {\n\
         \tsoit s = 0\n\
         \tpour k dans d {\n\
         \t\tsoit s = s + d[k]\n\
         \t}\n\
         \tretourne s\n\
         }\n\
         soit t: un dictionnaire de chaînes de caractères et d'entiers = {a: 1 ; b: 2}\n\
         R : #{total(t)}",
    );
    assert!(html.contains('3'), "{}", html);
}

#[test]
fn un_corps_reduit_a_un_litteral_est_une_valeur() {
    // `= {a: 1}` délimite un littéral, non un bloc d'instructions
    let html = rend("soit f(): un dictionnaire de chaînes de caractères et d'entiers = {a: 1}\nR : #{f()}");
    assert!(html.contains("a: 1"), "{}", html);
    let coll = rend("soit g(): une liste d'entiers = {1 ; 2}\nR : #{g()}");
    assert!(coll.contains("1 ; 2"), "{}", coll);
}

#[test]
fn une_declaration_typee_accepte_une_expression() {
    // le membre droit n'est pas forcément un littéral
    let html = rend(
        "soit empile(p: une liste d'entiers ; x: entier): une liste d'entiers = ajoute(p ; x)\n\
         soit p: une liste d'entiers = empile({} ; 1)\n\
         R : #p",
    );
    assert!(html.contains('1'), "{}", html);
}

#[test]
fn une_valeur_composee_se_pose_sans_type_ecrit() {
    let appel = rend(
        "soit empile(p: une liste d'entiers ; x: entier): une liste d'entiers = ajoute(p ; x)\n\
         soit p = empile({} ; 1)\n\
         R : #p",
    );
    assert!(appel.contains('1'), "{}", appel);
    let primitive = rend("soit v: une liste d'entiers = {3 ; 1}\nsoit w = tri(v)\nR : #w");
    assert!(primitive.contains("1 ; 3"), "{}", primitive);
    let chaine = rend("soit m = \"bonjour\"\nR : #m");
    assert!(chaine.contains("bonjour"), "{}", chaine);
}

#[test]
fn un_conteneur_se_reaffecte() {
    let html = rend("soit v: une liste d'entiers = {3 ; 1}\nsoit v = tri(v)\nR : #v");
    assert!(html.contains("1 ; 3"), "{}", html);
}

#[test]
fn l_accumulateur_par_concatenation_reste_intact() {
    // la réaffectation ne doit pas avoir mangé `soit S = S + {k}`
    let html = rend(
        "soit S: une liste d'entiers = {}\npour k de 1 à 4 {\n\tsoit S = S + {k}\n}\nR : #S",
    );
    assert!(html.contains("1 ; 2 ; 3 ; 4"), "{}", html);
}

#[test]
fn la_pile_et_la_file_s_ecrivent_dans_le_langage() {
    let html = rend(
        "soit empile(p: une liste d'entiers ; x: entier): une liste d'entiers = ajoute(p ; x)\n\
         soit sommet(p: une liste d'entiers): un entier = p[longueur(p) - 1]\n\
         soit dépile(p: une liste d'entiers): une liste d'entiers = p[0 à longueur(p) - 2]\n\
         soit défile(f: une liste d'entiers): une liste d'entiers = f[1 à longueur(f) - 1]\n\
         soit p = empile(empile(empile({} ; 1) ; 2) ; 3)\n\
         R : #p / #{sommet(p)} / #{dépile(p)} / #{défile(p)}",
    );
    assert!(html.contains("1 ; 2 ; 3"), "{}", html);
    assert!(html.contains("1 ; 2"), "{}", html);
    assert!(html.contains("2 ; 3"), "{}", html);
}

// ═══════════ chantier 3/3 — aléatoire, continuer, p-uplet ═══════════

#[test]
fn le_tirage_aleatoire_reste_dans_ses_bornes() {
    assert!(rend("R : #{aléatoire(5 ; 5)}").contains('5'));
    for _ in 0..20 {
        let html = rend("R : #{aléatoire(1 ; 6)}");
        let tire: i32 = html
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        assert!((1..=6).contains(&tire), "tirage hors bornes : {}", html);
    }
    // deux bornes à l'envers sont une faute dite
    assert!(rend("R : #{aléatoire(6 ; 1)}").contains("borne"));
}

#[test]
fn une_simulation_tient_dans_une_fonction() {
    let html = rend(
        "soit lancers(n: entier): un entier = {\n\
         \tsoit six = 0\n\
         \tpour k de 1 à n {\n\
         \t\tsi aléatoire(1 ; 6) vaut 6 {\n\
         \t\t\tsoit six = six + 1\n\
         \t\t}\n\
         \t}\n\
         \tretourne six\n\
         }\n\
         R : #{lancers(60)}",
    );
    assert!(!html.contains("calcul-absent"), "{}", html);
}

#[test]
fn continuer_saute_le_tour_sans_arreter_la_boucle() {
    let doc = rend("pour k de 1 à 4 {\n\tsi k vaut 2 {\n\t\tcontinuer\n\t}\n\t[#k]\n}");
    assert!(doc.contains("[1]") && doc.contains("[3]") && doc.contains("[4]"), "{}", doc);
    assert!(!doc.contains("[2]"), "le tour 2 devait être sauté : {}", doc);

    let fonction = rend(
        "soit impairs(n: entier): un entier = {\n\
         \tsoit s = 0\n\
         \tpour k de 1 à n {\n\
         \t\tsi reste de k par 2 vaut 0 {\n\
         \t\t\tcontinuer\n\
         \t\t}\n\
         \t\tsoit s = s + k\n\
         \t}\n\
         \tretourne s\n\
         }\n\
         R : #{impairs(6)}",
    );
    assert!(fonction.contains('9'), "1+3+5 = 9 : {}", fonction);
}

fn divise() -> &'static str {
    "soit divise(a: entier ; b: entier): (entier ; entier) = (quotient de a par b ; reste de a par b)\n"
}

#[test]
fn une_fonction_rend_deux_valeurs() {
    let html = rend(&format!("{}R : #{{divise(17 ; 5)}}", divise()));
    assert!(html.contains("(3 ; 2)"), "{}", html);
}

#[test]
fn la_deliaison_pose_les_deux_noms() {
    let doc = rend(&format!("{}soit (q ; r) = divise(17 ; 5)\nR : 17 = 5 × #q + #r", divise()));
    assert!(doc.contains("17 = 5 × 3 + 2"), "{}", doc);

    let fonction = rend(&format!(
        "{}soit somme_qr(a: entier ; b: entier): un entier = {{\n\
         \tsoit (q ; r) = divise(a ; b)\n\
         \tretourne q + r\n\
         }}\n\
         R : #{{somme_qr(17 ; 5)}}",
        divise()
    ));
    assert!(fonction.contains('5'), "3 + 2 = 5 : {}", fonction);
}

#[test]
fn un_p_uplet_est_heterogene_et_se_lit_par_son_rang() {
    let html = rend("soit c: (entier ; chaîne de caractères) = (3 ; \"trois\")\nR : #c puis #{c[1]}");
    assert!(html.contains("(3 ; trois)"), "{}", html);
    assert!(html.contains("puis trois"), "{}", html);
}

#[test]
fn le_p_uplet_ne_se_confond_pas_avec_une_collection() {
    // arité fixe : une collection accepte n'importe quelle longueur, pas lui
    let confusion = rend("soit v: une liste d'entiers = (1 ; 2)\nR : #v");
    assert!(confusion.contains('⚠'), "{}", confusion);
    let arite = rend(&format!("{}soit (a ; b ; c) = divise(17 ; 5)\nR : #a", divise()));
    assert!(arite.contains('⚠'), "{}", arite);
}

#[test]
fn les_extrema_en_un_seul_parcours() {
    let html = rend(
        "soit extrema(v: une liste d'entiers): (entier ; entier) = {\n\
         \tsoit petit = v[0]\n\
         \tsoit grand = v[0]\n\
         \tpour x dans v {\n\
         \t\tsi x moins de petit { soit petit = x }\n\
         \t\tsi x plus de grand { soit grand = x }\n\
         \t}\n\
         \tretourne (petit ; grand)\n\
         }\n\
         soit (mini ; maxi) = extrema({12 ; 15 ; 9 ; 18})\n\
         R : #mini et #maxi",
    );
    assert!(html.contains("9 et 18"), "{}", html);
}

#[test]
fn une_declaration_typee_accepte_un_appel_meme_scalaire() {
    let html = rend("soit f(a: entier): entier = a + 1\nsoit x: entier = f(3)\nR : #x");
    assert!(html.contains('4'), "{}", html);
}

// ═══════════ tant que, au niveau du document ═══════════

#[test]
fn un_tant_que_de_document_fait_croitre_un_conteneur() {
    // `subst_var` protège les accolades — et c'est ce qu'il faut, une accolade
    // délimite des données. Mais l'accumulateur d'un `tant que` a besoin de la
    // valeur du tour : la levée ne vaut que pour les lignes de croissance.
    for boucle in [
        "soit k = 0\ntant que k moins de 3 faire {\n\tsoit k = k + 1\n\tsoit v = v + {k}\n}",
        "soit k = 0\nfaire {\n\tsoit k = k + 1\n\tsoit v = v + {k}\n} tant que k moins de 3",
    ] {
        let html = rend(&format!("soit v: liste d'entiers = {{}}\n{}\nR : #v", boucle));
        assert!(html.contains("1 ; 2 ; 3"), "{}", html);
    }
}

#[test]
fn une_accolade_de_prose_reste_intacte() {
    assert!(rend("soit k = 1\nvaleur {k} ici").contains("{k}"));
}

#[test]
fn une_condition_de_tant_que_interroge_un_conteneur_dans_une_fonction() {
    // Limite connue : au **niveau du document**, la condition d'un `tant que`
    // est évaluée avant que les conteneurs n'existent — l'ordre du pipeline
    // place ce déroulement avant la création des boîtes. Dans un corps de
    // fonction, où tout est déroulé au moment de l'appel, elle les voit.
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

// ═══════════ l'affectation sans « soit » ═══════════
//
// `soit` déclare ; le répéter à chaque tour n'apprend rien et ne se lit pas.
// La condition qui rend la levée sûre tient en un mot : **le nom doit déjà
// exister**. Une ligne de prose contenant un signe égal n'est donc jamais
// prise pour une affectation.

#[test]
fn un_nom_deja_pose_se_reaffecte_sans_soit() {
    let html = rend(
        "soit somme = 0\npour k de 1 à 100 {\n\tsomme = somme + k\n}\nLa somme vaut #somme.",
    );
    assert!(html.contains("5050"), "{}", html);
}

#[test]
fn l_affectation_nue_vaut_pour_les_conteneurs_et_les_boucles() {
    let collection = rend("soit v: liste d'entiers = {}\npour k de 1 à 3 {\n\tv = v + {k}\n}\nR : #v");
    assert!(collection.contains("1 ; 2 ; 3"), "{}", collection);

    let tant_que = rend("soit n = 0\ntant que n moins de 3 faire {\n\tn = n + 1\n}\nR : #n");
    assert!(tant_que.contains('3'), "{}", tant_que);

    let fonction = rend(
        "soit f(m: entier): entier = {\n\
         \tsoit n = 0\n\
         \tpour k de 1 à m {\n\
         \t\tn = n + k\n\
         \t}\n\
         \tretourne n\n\
         }\n\
         R : #{f(10)}",
    );
    assert!(fonction.contains("55"), "{}", fonction);
}

#[test]
fn la_prose_qui_contient_un_egal_reste_de_la_prose() {
    // un nom jamais déclaré
    assert!(rend("Le résultat z = 5 est admis.").contains("z = 5"));
    // un nom déclaré, mais une phrase
    assert!(rend("soit x = 3\nOn pose y = 2x + 1 dans la suite.").contains("y = 2x + 1"));
    // une ligne d'affichage : le dièse signe la prose
    let doc = rend("pour k de 1 à 2 {\n\tk=#k\n}");
    assert!(doc.contains("k=1") && doc.contains("k=2"), "{}", doc);
}

#[test]
fn soit_reste_admis_partout() {
    let html = rend("soit somme = 0\npour k de 1 à 5 {\n\tsoit somme = somme + k\n}\nR : #somme");
    assert!(html.contains("15"), "{}", html);
}

// ═══════════ la source d'une boucle ═══════════

#[test]
fn une_chaine_ecrite_sur_place_se_parcourt() {
    let doc = rend("pour c dans \"un code\" {\n\t[#c]\n}");
    for lettre in ["[u]", "[n]", "[ ]", "[c]", "[o]", "[d]", "[e]"] {
        assert!(doc.contains(lettre), "{} manquant : {}", lettre, doc);
    }
}

#[test]
fn une_source_qui_n_est_pas_un_conteneur_est_dite() {
    // `pour c dans un code` faisait un unique tour sur « un code » au lieu
    // d'en lire les lettres : le faux-semblant valait moins que la faute
    for source in ["un code", "truc"] {
        let doc = rend(&format!("pour x dans {} {{\n\t[#x]\n}}", source));
        assert!(doc.contains("n'est pas un conteneur"), "{} → {}", source, doc);
        assert!(doc.contains("guillemets"), "l'issue doit être dite : {}", doc);
    }
}

#[test]
fn les_autres_sources_restent_admises() {
    assert!(rend("pour x dans {1 ; 2} {\n\t[#x]\n}").contains("[1]"));
    assert!(rend("pour x dans [a, b] {\n\t[#x]\n}").contains("[a]"));
    assert!(rend("soit m: chaîne de caractères = \"ab\"\npour c dans m {\n\t[#c]\n}").contains("[a]"));
}

// ═══════════ écrire une suite sur une ligne ═══════════

#[test]
fn jonction_ecrit_une_suite_avec_son_separateur() {
    // l'accumulateur laissait toujours un séparateur de trop à la fin ; il n'y
    // a pas de raison de faire compter l'élève
    assert!(rend("R : #{jonction(\"un code\" ; \" - \")}").contains("u - n -   - c - o - d - e"));
    let v = "soit v: liste d'entiers = {1 ; 2 ; 3}\n";
    assert!(rend(&format!("{}R : #{{jonction(v ; \", \")}}", v)).contains("1, 2, 3"));
    assert!(rend(&format!("{}R : #{{dans v jonction(\" — \")}}", v)).contains("1 — 2 — 3"));
}

#[test]
fn un_separateur_peut_etre_un_point_virgule() {
    // le découpage des arguments doit respecter les guillemets
    let html = rend("soit v: liste d'entiers = {1 ; 2}\nR : #{jonction(v ; \" ; \")}");
    assert!(html.contains("1 ; 2"), "{}", html);
}

#[test]
fn un_texte_calcule_n_est_pas_recalcule() {
    // `jonction(v ; " / ")` rend « 1 / 2 » : sans marque, le résultat serait
    // repris pour une division et vaudrait un demi
    let html = rend("soit v: liste d'entiers = {1 ; 2}\nR : #{jonction(v ; \" / \")}");
    assert!(html.contains("1 / 2"), "{}", html);
    assert!(!html.contains("0,5"), "{}", html);
}

#[test]
fn les_guillemets_gardent_les_espaces() {
    // ils sont là pour cela : un séparateur « - » en dépend
    assert!(rend("soit m: chaîne de caractères = \" a \"\nR : [#m]").contains("[ a ]"));
}

#[test]
fn decoupe_est_le_chemin_inverse() {
    assert!(rend("R : #{découpe(\"un code\" ; \" \")}").contains("{un ; code}"));
    assert!(rend("R : #{découpe(\"abc\" ; \"\")}").contains("{a ; b ; c}"));
    assert!(rend("R : #{jonction(découpe(\"a,b,c\" ; \",\") ; \" + \")}").contains("a + b + c"));
}

#[test]
fn les_nombres_premiers_sur_une_ligne() {
    let html = rend(
        "soit premiers(n: entier): liste d'entiers = {\n\
         \tsoit p: liste d'entiers = {}\n\
         \tpour m de 2 à n {\n\
         \t\tsoit d = 0\n\
         \t\tpour k de 2 à m - 1 {\n\
         \t\t\tsi reste de m par k vaut 0 {\n\
         \t\t\t\td = d + 1\n\
         \t\t\t}\n\
         \t\t}\n\
         \t\tsi d vaut 0 {\n\
         \t\t\tp = ajoute(p ; m)\n\
         \t\t}\n\
         \t}\n\
         \tretourne p\n\
         }\n\
         R : #{jonction(premiers(30) ; \", \")}",
    );
    assert!(html.contains("2, 3, 5, 7, 11, 13, 17, 19, 23, 29"), "{}", html);
}

#[test]
fn un_tour_saute_ne_laisse_pas_de_ligne_vide() {
    // le tour sauté recevait son séparateur alors qu'il ne produit rien :
    // une ligne vide apparaissait là où il n'y a précisément rien
    let doc = rend("pour k de 1 à 6 {\n\tsi reste de k par 2 vaut 0 {\n\t\tcontinuer\n\t}\n\t[#k]\n}");
    assert!(doc.contains("[1]") && doc.contains("[3]") && doc.contains("[5]"), "{}", doc);
    // trois tours retenus, donc deux séparateurs
    assert_eq!(doc.matches("ligne-vide").count(), 2, "{}", doc);
}

// ═══════════ les espaces d'une chaîne ═══════════

#[test]
fn elague_retire_les_espaces_des_deux_bouts() {
    assert!(rend("R : [#{élague(\"  bonjour  \")}]").contains("[bonjour]"));
    // une seule forme : l'infinitif n'est pas une seconde écriture, et
    // l'appel n'aboutit donc pas
    assert!(!rend("R : [#{élaguer(\"  bonjour  \")}]").contains("[bonjour]"));
}

#[test]
fn compacte_les_retire_tous() {
    assert!(rend("R : [#{compacte(\"un code secret\")}]").contains("[uncodesecret]"));
    assert!(rend("R : #{longueur(compacte(\"un code\"))}").contains('6'));
    // la forme prépositionnelle, parenthèse vide
    assert!(rend("soit m: chaîne de caractères = \" a b \"\nR : [#{dans m compacte()}]").contains("[ab]"));
}

// ═══════════ une notion, un mot ═══════════

#[test]
fn le_vocabulaire_est_unique() {
    // les formes retirées ne déclarent plus rien : la ligne reste en prose et
    // le `#nom` sort littéralement, ce qui se voit à la lecture du document
    for ancienne in [
        "soit m: texte = \"a\"\nR [#m]",
        "soit m: chaîne = \"a\"\nR [#m]",
        "soit v: collection d'entiers = {1}\nR [#v]",
        "soit v: tableau d'entiers = {1}\nR [#v]",
    ] {
        let html = rend(ancienne);
        assert!(html.contains("[#"), "forme retirée encore acceptée : {}", html);
    }
    // et la forme retenue fonctionne
    assert!(rend("soit m: chaîne de caractères = \"a\"\nR [#m]").contains("[a]"));
    assert!(rend("soit v: liste d'entiers = {1}\nR [#v]").contains("[{1}]"));
}

#[test]
fn les_guillemets_delimitent_partout() {
    // une chaîne de caractères se cite : dans une liste, en clé de
    // dictionnaire, comme au singulier
    assert!(rend("soit v: liste de chaînes de caractères = {\"a\" ; \"b\"}\nR : #v").contains("{a ; b}"));
    let d = rend(
        "soit d: dictionnaire de chaînes de caractères et d'entiers = {\"Marche\": 5}\nR : #d",
    );
    assert!(d.contains("{Marche: 5}"), "{}", d);
    assert!(rend("soit d: dictionnaire de chaînes de caractères et d'entiers = {\"A\": 1}\nR : #{d[\"A\"]}").contains('1'));
}

// ═══════════ `sortir` et `continuer` dans les boucles à condition ═══════════
//
// La sortie anticipée fonctionnait dans `pour` et dans les corps de fonction,
// mais restait sans effet dans un `tant que` du document : la boucle allait
// jusqu'au bout, sans le moindre message. Une recherche séquentielle — le cas
// d'usage même du `tant que` — sortait donc fausse.

#[test]
fn sortir_arrete_un_tant_que_de_document() {
    let html = rend("soit n = 0\ntant que n < 5 faire {\n\tsi n plus de 2 {\n\t\tsortir\n\t}\n\t[#n]\n\tsoit n = n + 1\n}");
    assert!(html.contains("[0]") && html.contains("[2]"), "{}", html);
    assert!(!html.contains("[3]"), "sortir devait couper avant 3 : {}", html);
    assert!(!html.contains("sortir"), "le mot ne doit pas fuir : {}", html);
}

#[test]
fn continuer_saute_un_tour_de_tant_que() {
    let html = rend("soit n = 0\ntant que n < 4 faire {\n\tsoit n = n + 1\n\tsi n vaut 2 {\n\t\tcontinuer\n\t}\n\t[#n]\n}");
    assert!(html.contains("[1]") && html.contains("[3]") && html.contains("[4]"), "{}", html);
    assert!(!html.contains("[2]"), "le tour 2 devait être sauté : {}", html);
    assert!(!html.contains("continuer"), "le mot ne doit pas fuir : {}", html);
}

#[test]
fn sortir_arrete_un_faire_tant_que() {
    let html = rend("soit n = 0\nfaire {\n\tsi n plus de 2 {\n\t\tsortir\n\t}\n\t[#n]\n\tsoit n = n + 1\n} tant que n < 5");
    assert!(html.contains("[2]"), "{}", html);
    assert!(!html.contains("[3]"), "sortir devait couper avant 3 : {}", html);
}

#[test]
fn un_tant_que_interrompu_n_ajoute_pas_de_ligne_vide() {
    // le tour qui ne fait que sortir ne produit rien : il ne doit pas laisser
    // de séparateur derrière lui
    let avec = rend("soit n = 0\ntant que n < 5 faire {\n\tsi n plus de 2 {\n\t\tsortir\n\t}\n\t[#n]\n\tsoit n = n + 1\n}");
    let sans = rend("soit n = 0\ntant que n < 3 faire {\n\t[#n]\n\tsoit n = n + 1\n}");
    assert_eq!(avec, sans, "la boucle coupée doit rendre le même document que la boucle bornée");
}

#[test]
fn le_si_d_un_tant_que_voit_les_variables_du_tour() {
    // la condition se lit avec la valeur que la variable a *à cet endroit* du
    // tour, non avec celle qu'elle avait en entrant
    let html = rend("soit n = 0\ntant que n < 3 faire {\n\tsoit n = n + 1\n\tsi n vaut 2 {\n\t\tDEUX\n\t} sinon {\n\t\tAUTRE\n\t}\n}");
    assert!(html.contains("DEUX"), "{}", html);
    assert_eq!(html.matches("AUTRE").count(), 2, "{}", html);
}

// ═══════════ l'accord de « Soit » ═══════════
//
// L'impératif s'accorde avec ce qu'il pose : « Soient les points A, B et C ».
// La faute se relève avant même la lecture de la démonstration.

#[test]
fn soit_s_accorde_au_pluriel() {
    let points = rend("<Soit>les points A(1;2), B(-1;2) et C(-1;-2)");
    assert!(points.contains("Soient"), "{}", points);
    assert!(points.contains("les points de coordonnées"), "{}", points);
    let fonctions = rend("<Soit>les fonctions f(x) = x^2 et g(x) = -x^2");
    assert!(fonctions.contains("Soient"), "{}", fonctions);
    assert!(fonctions.contains("les fonctions définies par"), "{}", fonctions);
}

#[test]
fn soit_reste_au_singulier_pour_un_seul_objet() {
    let un = rend("<Soit>un point A(2;3)");
    assert!(!un.contains("Soient"), "{}", un);
}
