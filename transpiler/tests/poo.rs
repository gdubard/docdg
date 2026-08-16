//! Version 2.5 — la programmation orientée objet.
//!
//! Décision fondatrice : **une classe est une fonction qui construit.** En
//! Python, `Point(3, 4)` *est* un appel de la classe. En enregistrant le
//! constructeur comme une entrée ordinaire de la table des fonctions, et les
//! méthodes sous la clé `Point.norme`, la vérification d'arité et de types
//! est celle qui existait déjà : aucune signature du moteur n'a changé.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

fn point() -> &'static str {
    "soit une classe Point {\n\
     \tabscisse: un réel\n\
     \tordonnée: un réel\n\
     \n\
     \tsoit norme(): un réel = racine(abscisse * abscisse + ordonnée * ordonnée)\n\
     \tsoit translaté(dx: un réel ; dy: un réel): un Point = Point(abscisse + dx ; ordonnée + dy)\n\
     }\n"
}

#[test]
fn une_classe_se_declare_et_s_instancie() {
    let html = rend(&format!("{}soit p: un Point = Point(3 ; 4)\nR : #p", point()));
    assert!(html.contains("abscisse: 3"), "{}", html);
    assert!(html.contains("ordonnée: 4"), "{}", html);
}

#[test]
fn un_attribut_se_lit_par_le_point() {
    let html = rend(&format!(
        "{}soit p: un Point = Point(3 ; 4)\nR : #{{p.abscisse}} et #{{p.ordonnée}}",
        point()
    ));
    assert!(html.contains("3 et 4"), "{}", html);
}

#[test]
fn une_methode_voit_les_attributs_de_son_objet() {
    let html = rend(&format!("{}soit p: un Point = Point(3 ; 4)\nR : #{{p.norme()}}", point()));
    assert!(html.contains('5'), "3-4-5 : {}", html);
}

#[test]
fn une_methode_prend_des_arguments_et_rend_un_objet() {
    let html = rend(&format!(
        "{}soit p: un Point = Point(3 ; 4)\nsoit q = p.translaté(1 ; 1)\nR : #q",
        point()
    ));
    assert!(html.contains("abscisse: 4"), "{}", html);
    assert!(html.contains("ordonnée: 5"), "{}", html);
}

#[test]
fn un_objet_se_pose_sans_type_ecrit() {
    let html = rend(&format!("{}soit p = Point(3 ; 4)\nR : #{{p.abscisse}}", point()));
    assert!(html.contains('3'), "{}", html);
}

#[test]
fn l_arite_du_constructeur_est_verifiee() {
    let html = rend(&format!("{}soit p: un Point = Point(3)\nR : #p", point()));
    assert!(html.contains("attend 2 argument"), "{}", html);
}

#[test]
fn le_type_d_un_attribut_est_verifie() {
    let classe = "soit une classe Compteur {\n\tvaleur: un entier\n}\n";
    let html = rend(&format!("{}soit c: un Compteur = Compteur(2,5)\nR : #c", classe));
    assert!(html.contains("n'est pas un entier"), "{}", html);
}

#[test]
fn un_attribut_absent_est_une_faute_dite() {
    let html = rend(&format!("{}soit p: un Point = Point(3 ; 4)\nR : #{{p.couleur}}", point()));
    assert!(html.contains("n'a pas d'attribut couleur"), "{}", html);
}

#[test]
fn une_methode_absente_est_une_faute_dite() {
    let html = rend(&format!("{}soit p: un Point = Point(3 ; 4)\nR : #{{p.tourne()}}", point()));
    assert!(html.contains("n'est pas une méthode"), "{}", html);
}

#[test]
fn un_objet_ne_se_confond_pas_avec_un_autre() {
    let deux = format!(
        "{}soit une classe Cercle {{\n\trayon: un réel\n}}\nsoit c: un Point = Cercle(2)\nR : #c",
        point()
    );
    let html = rend(&deux);
    assert!(html.contains('⚠'), "{}", html);
}

#[test]
fn un_objet_entre_dans_une_collection() {
    let html = rend(&format!(
        "{}soit a: un Point = Point(1 ; 2)\nsoit b: un Point = Point(3 ; 4)\nR : #{{a.norme()}} #{{b.abscisse}}",
        point()
    ));
    assert!(!html.contains("calcul-absent"), "{}", html);
}

// ═══════════ l'encapsulation ═══════════
//
// Le défaut est la visibilité, comme en Scala : un seul mot la retire, et il
// se place en tête, comme `private` en C#. Un mot, un défaut — rien à retenir
// pour le cas courant, qui est celui de l'élève.

fn compte() -> &'static str {
    "soit une classe Compte {\n\
     \ttitulaire: chaîne de caractères\n\
     \tprivé solde: un réel\n\
     \n\
     \tprivé soit taux(): un réel = 0,02\n\
     \tsoit intérêts(): un réel = solde * taux()\n\
     \tsoit résumé(): une chaîne de caractères = titulaire\n\
     }\n\
     soit c: un Compte = Compte(\"Léa\" ; 250)\n"
}

#[test]
fn un_attribut_public_se_lit_du_dehors() {
    assert!(rend(&format!("{}R : #{{c.titulaire}}", compte())).contains("Léa"));
}

#[test]
fn un_attribut_prive_ne_se_lit_pas_du_dehors() {
    let html = rend(&format!("{}R : #{{c.solde}}", compte()));
    assert!(html.contains("attribut privé"), "{}", html);
    assert!(html.contains("depuis la classe"), "l'issue doit être dite : {}", html);
}

#[test]
fn une_methode_privee_ne_s_appelle_pas_du_dehors() {
    let html = rend(&format!("{}R : #{{c.taux()}}", compte()));
    assert!(html.contains("méthode privée"), "{}", html);
}

#[test]
fn une_methode_publique_voit_ce_qui_est_prive() {
    // 250 × 0,02 = 5 : la méthode publique lit l'attribut privé *et* appelle
    // la méthode privée. C'est tout l'objet de l'encapsulation.
    let html = rend(&format!("{}R : #{{c.intérêts()}}", compte()));
    assert!(html.contains('5'), "{}", html);
    assert!(!html.contains("calcul-absent"), "{}", html);
}

#[test]
fn un_attribut_prive_reste_visible_dans_sa_classe() {
    assert!(rend(&format!("{}R : #{{c.résumé()}}", compte())).contains("Léa"));
}

#[test]
fn le_feminin_du_mot_prive_est_accepte() {
    let classe = "soit une classe Boîte {\n\tprivée clé: un entier\n\tsoit ouvre(): un entier = clé\n}\n\
                  soit b: un Boîte = Boîte(7)\n";
    assert!(rend(&format!("{}R : #{{b.ouvre()}}", classe)).contains('7'));
    assert!(rend(&format!("{}R : #{{b.clé}}", classe)).contains("privé"));
}

// ═══════════ l'héritage ═══════════
//
// La lignée voyage **avec la valeur** : `Valeur::Objet` porte ses ancêtres.
// C'est ce qui permet à un chien de tenir la place d'un animal sans que le
// vérificateur de types ait à consulter la table des classes.

fn animaux() -> &'static str {
    "soit une classe Animal {\n\
     \tnom: chaîne de caractères\n\
     \tpattes: entier\n\
     \n\
     \tsoit cri(): chaîne de caractères = \"...\"\n\
     \tsoit carte(): chaîne de caractères = nom\n\
     }\n\
     soit une classe Chien hérite de Animal {\n\
     \trace: chaîne de caractères\n\
     \n\
     \tsoit cri(): chaîne de caractères = \"ouaf\"\n\
     }\n\
     soit c: Chien = Chien(\"Rex\" ; 4 ; \"berger\")\n"
}

#[test]
fn les_attributs_du_parent_viennent_en_tete() {
    let html = rend(&format!(
        "{}R : #{{c.nom}} #{{c.pattes}} #{{c.race}}",
        animaux()
    ));
    assert!(html.contains("Rex"), "{}", html);
    assert!(html.contains("berger"), "{}", html);
}

#[test]
fn une_methode_du_parent_revient_a_l_enfant() {
    assert!(rend(&format!("{}R : #{{c.carte()}}", animaux())).contains("Rex"));
}

#[test]
fn l_enfant_redefinit_ce_qu_il_veut() {
    assert!(rend(&format!("{}R : #{{c.cri()}}", animaux())).contains("ouaf"));
    // et le parent garde la sienne
    let parent = rend(&format!("{}soit a: Animal = Animal(\"Loup\" ; 4)\nR : #{{a.cri()}}", animaux()));
    assert!(parent.contains("..."), "{}", parent);
}

#[test]
fn un_enfant_tient_la_place_de_son_parent() {
    // la substitution : c'est ce que l'héritage apporte au-delà du partage
    let html = rend(&format!(
        "{}soit décrit(x: Animal): chaîne de caractères = x.carte()\nR : #{{décrit(c)}}",
        animaux()
    ));
    assert!(html.contains("Rex"), "{}", html);
}

#[test]
fn un_parent_ne_tient_pas_la_place_de_son_enfant() {
    let html = rend(&format!("{}soit a: Chien = Animal(\"Loup\" ; 4)\nR : #a", animaux()));
    assert!(html.contains("un Animal n'est pas un Chien"), "{}", html);
}

#[test]
fn ce_qui_est_prive_chez_le_parent_le_reste_chez_l_enfant() {
    let src = "soit une classe Base {\n\
               \tprivé secret: entier\n\
               \tsoit lit(): entier = secret\n\
               }\n\
               soit une classe Dérivée hérite de Base {\n\
               \tétiquette: chaîne de caractères\n\
               }\n\
               soit d: Dérivée = Dérivée(7 ; \"a\")\n";
    assert!(rend(&format!("{}R : #{{d.lit()}}", src)).contains('7'));
    assert!(rend(&format!("{}R : #{{d.secret}}", src)).contains("attribut privé"));
}

#[test]
fn le_type_se_passe_du_determinant() {
    // à ce niveau la prose n'est pas nécessaire : `x: réel` suffit
    let html = rend("soit une classe P {\n\tx: réel\n\tsoit deux(): réel = x * 2\n}\nsoit p: P = P(21)\nR : #{p.deux()}");
    assert!(html.contains("42"), "{}", html);
}

// ═══════════ le polymorphisme ═══════════
//
// La liaison est **dynamique par construction** : `resoudre_points` lit le nom
// de classe dans la *valeur*, non dans le type déclaré. Un chien rangé parmi
// des animaux reste un chien.

fn ferme() -> &'static str {
    "soit une classe Animal {\n\
     \tnom: chaîne de caractères\n\
     \n\
     \tsoit cri(): chaîne de caractères = \"...\"\n\
     }\n\
     soit une classe Chien qui hérite de la classe Animal {\n\
     \tsoit cri(): chaîne de caractères = \"ouaf\"\n\
     }\n\
     soit une classe Chat qui hérite de la classe Animal {\n\
     \tsoit cri(): chaîne de caractères = \"miaou\"\n\
     }\n"
}

#[test]
fn la_liaison_est_dynamique() {
    // le paramètre est déclaré Animal, la valeur est un Chien : c'est la
    // valeur qui décide
    let html = rend(&format!(
        "{}soit fait(x: Animal): chaîne de caractères = x.cri()\nsoit c: Chien = Chien(\"Rex\")\nR : #{{fait(c)}}",
        ferme()
    ));
    assert!(html.contains("ouaf"), "{}", html);
}

#[test]
fn une_collection_accueille_les_descendants() {
    let html = rend(&format!(
        "{}soit z: liste d'Animal = {{Chien(\"Rex\") ; Chat(\"Mia\")}}\nR : #{{longueur(z)}}",
        ferme()
    ));
    assert!(html.contains('2'), "{}", html);
}

#[test]
fn parcourir_une_collection_appelle_chacun_selon_sa_classe() {
    let doc = rend(&format!(
        "{}soit z: liste d'Animal = {{Chien(\"Rex\") ; Chat(\"Mia\")}}\npour a dans z {{\n\t[#{{a.cri()}}]\n}}",
        ferme()
    ));
    assert!(doc.contains("[ouaf]") && doc.contains("[miaou]"), "{}", doc);

    let fonction = rend(&format!(
        "{}soit tous(v: liste d'Animal): chaîne de caractères = {{\n\
         \tsoit r: chaîne de caractères = \"\"\n\
         \tpour a dans v {{\n\
         \t\tsoit r = r + a.cri()\n\
         \t}}\n\
         \tretourne r\n\
         }}\n\
         R : #{{tous({{Chien(\"Rex\") ; Chat(\"Mia\")}})}}",
        ferme()
    ));
    assert!(fonction.contains("ouafmiaou"), "{}", fonction);
}

#[test]
fn les_deux_tournures_de_l_heritage_se_valent() {
    // deux, non quatre : la brève et la parlée
    for tournure in ["hérite de", "qui hérite de la classe"] {
        let src = format!(
            "soit une classe A {{\n\tn: entier\n}}\n\
             soit une classe B {} A {{\n\tm: entier\n}}\n\
             soit x: B = B(1 ; 2)\nR : #{{x.n}}",
            tournure
        );
        assert!(rend(&src).contains('1'), "tournure « {} » : {}", tournure, rend(&src));
    }
}

// ═══════════ l'abstraction ═══════════

fn formes() -> &'static str {
    "soit une classe abstraite Forme {\n\
     \tnom: chaîne de caractères\n\
     \n\
     \tsoit aire(): réel\n\
     \tsoit carte(): chaîne de caractères = nom\n\
     }\n\
     soit une classe Carré qui hérite de la classe Forme {\n\
     \tcôté: réel\n\
     \n\
     \tsoit aire(): réel = côté * côté\n\
     }\n\
     soit c: Carré = Carré(\"c\" ; 3)\n"
}

#[test]
fn une_classe_abstraite_ne_s_instancie_pas() {
    let html = rend(&format!("{}soit x: Forme = Forme(\"f\")\nR : #x", formes()));
    assert!(html.contains("classe abstraite"), "{}", html);
    assert!(html.contains("classes filles"), "l'issue doit être dite : {}", html);
}

#[test]
fn la_fille_fournit_le_corps_qui_manquait() {
    let html = rend(&format!("{}R : #{{c.aire()}} #{{c.carte()}}", formes()));
    assert!(html.contains('9'), "{}", html);
    assert!(html.contains(" c"), "{}", html);
}

#[test]
fn une_methode_abstraite_traverse_le_polymorphisme() {
    let html = rend(&format!(
        "{}soit mesure(s: Forme): réel = s.aire()\nR : #{{mesure(c)}}",
        formes()
    ));
    assert!(html.contains('9'), "{}", html);
}

#[test]
fn une_classe_concrete_ne_laisse_rien_sans_corps() {
    let html = rend(
        "soit une classe abstraite A {\n\tsoit f(): réel\n}\n\
         soit une classe B qui hérite de la classe A {\n\tn: entier\n}\nR : fin",
    );
    assert!(html.contains("sans corps"), "{}", html);
    assert!(html.contains("abstraite"), "l'issue doit être dite : {}", html);
}

// ═══════════ la modification d'un attribut ═══════════
//
// docdg n'a pas de références : une boîte n'est jamais partagée. Muter la
// boîte et lui réaffecter une copie modifiée sont donc indiscernables — il n'y
// a rien qu'un marqueur de référence pourrait distinguer, et le langage reste
// entièrement par valeur.

fn point_mutable() -> &'static str {
    "soit une classe Point {\n\tx: réel\n\ty: réel\n}\nsoit p: Point = Point(1 ; 2)\n"
}

#[test]
fn un_attribut_se_modifie() {
    let html = rend(&format!("{}p.x = 5\nR : #{{p.x}} et #{{p.y}}", point_mutable()));
    assert!(html.contains("5 et 2"), "{}", html);
}

#[test]
fn la_modification_est_typee() {
    let html = rend(&format!("{}p.x = \"trois\"\nR : #{{p.x}}", point_mutable()));
    assert!(html.contains("ne se lit pas comme"), "{}", html);
}

#[test]
fn on_ne_modifie_pas_un_attribut_qui_n_existe_pas() {
    let html = rend(&format!("{}p.z = 5\nR : #{{p.x}}", point_mutable()));
    assert!(html.contains("n'a pas d'attribut z"), "{}", html);
}

#[test]
fn on_ne_modifie_pas_du_dehors_ce_qui_est_prive() {
    let html = rend("soit une classe C {\n\tprivé s: réel\n}\nsoit c: C = C(1)\nc.s = 9\nR : fin");
    assert!(html.contains("attribut privé"), "{}", html);
    assert!(html.contains("ne s'écrit que"), "{}", html);
}

#[test]
fn une_chaine_attribut_ne_se_confond_pas_avec_une_variable() {
    // régression : l'attribut valant « c » était relu comme la variable c,
    // puisque les attributs voyagent sous forme écrite jusqu'à l'appel
    let html = rend(&format!("{}R : #{{c.carte()}}", formes()));
    assert!(!html.contains("calcul-absent"), "{}", html);
}

#[test]
fn un_attribut_textuel_s_affiche_sans_guillemets() {
    // La forme *relisible* — celle qui porte les guillemets — sert à faire
    // voyager une valeur jusqu'à un appel. À l'affichage, c'est la forme lue
    // qui convient : une chaîne se cite quand on l'écrit, pas quand on la
    // montre.
    let src = "soit une classe Compte {\n\
               \ttitulaire: chaîne de caractères\n\
               \tsoit nom(): chaîne de caractères = titulaire\n\
               }\n\
               soit c: Compte = Compte(\"Léa\")\n";
    let direct = rend(&format!("{}R : [#{{c.titulaire}}]", src));
    assert!(direct.contains("[Léa]"), "{}", direct);
    assert!(!direct.contains("&quot;"), "les guillemets ne doivent pas ressortir : {}", direct);
    // et la lecture par méthode donne la même chose
    assert!(rend(&format!("{}R : [#{{c.nom()}}]", src)).contains("[Léa]"));
}
