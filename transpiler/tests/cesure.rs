use docdg_transpiler::cesure_html;

const COUPE: char = '\u{00AD}';

#[test]
fn prose_coupee_selon_les_motifs_francais() {
    let sortie = cesure_html("La factorisation plurielle fonctionne.");
    assert_eq!(
        sortie.replace(COUPE, "|"),
        "La fac|to|ri|sa|tion plu|rielle fonc|tionne."
    );
}

#[test]
fn zones_mathematiques_intactes() {
    let source = "Calcul \\(\\operatorname{PGCD}(84\\,;\\,60) = 12\\) magistralement rédigé.";
    let sortie = cesure_html(source);
    assert!(sortie.contains("\\(\\operatorname{PGCD}(84\\,;\\,60) = 12\\)"));
    assert!(sortie.contains("ma\u{00AD}gis\u{00AD}tra\u{00AD}le\u{00AD}ment"));
    let bloc = cesure_html("\\[\\dfrac{denominateur}{numerateur}\\]");
    assert!(!bloc.contains(COUPE));
}

#[test]
fn balises_protegees_intactes() {
    let source = "<h2 class=\"sec\">Introduction fondamentale</h2><svg><text>dimensionnelle</text></svg><code>interminable</code><p>Paragraphe recommence.</p>";
    let sortie = cesure_html(source);
    assert!(sortie.contains("Introduction fondamentale"));
    assert!(sortie.contains("<text>dimensionnelle</text>"));
    assert!(sortie.contains("<code>interminable</code>"));
    assert!(sortie.contains("Pa\u{00AD}ra\u{00AD}graphe"));
}

#[test]
fn mots_particuliers_epargnes() {
    let sortie = cesure_html("porte-monnaie PGCD Paris quatrième l'exemple");
    assert!(sortie.contains("porte-monnaie"));
    assert!(sortie.contains("PGCD"));
    assert!(sortie.contains("Paris"));
    assert!(sortie.contains("qua\u{00AD}trième"));
    assert!(sortie.contains("l'exemple"));
}

#[test]
fn entites_soudees_non_coupees() {
    let sortie = cesure_html("R&eacute;solution &amp; suite");
    assert!(sortie.contains("R&eacute;solution"));
}

#[test]
fn blocs_secables_marques() {
    let mut e = docdg_transpiler::Engine::new();
    let src = include_str!("../../exemples/publication3.txt");
    let r = e.render(src, true);
    assert!(!r.html.contains("calcul-absent"));
    assert!(r.page.cesure);
    assert_eq!(r.page.veuves, 2);
    assert_eq!(r.page.orphelines, 2);
    assert_eq!(r.html.matches("cadre secable").count(), 1);
    let tab = &r.html[r.html.find("<table").unwrap()..];
    assert!(!tab[..tab.find("\">").unwrap()].contains("background:navy"));
    assert_eq!(r.html.matches("tab secable").count(), 1);
    assert_eq!(r.html.matches("tab-entete").count(), 1);
    let sortie = cesure_html(&r.html);
    assert!(sortie.contains("\\(\\operatorname{PGCD}(84\\,;\\,60) = 12\\)"));
    assert!(sortie.matches(COUPE).count() > 300);
}

#[test]
fn fond_des_entetes_ne_deteint_pas() {
    let mut e = docdg_transpiler::Engine::new();
    let src = "<Dresse>un tableau sécable [mc, md] avec une bordure bleu marine et des entêtes en blanc sur fond bleu marine{\n\tRang\tTerme\n\t[0 ; 1]\n}";
    let r = e.render(src, false);
    let tab = &r.html[r.html.find("<table").unwrap()..];
    let ouverture = &tab[..tab.find("\">").unwrap()];
    assert!(ouverture.contains("background:transparent"), "{}", ouverture);
    assert!(ouverture.contains("border:0.3mm solid navy"), "{}", ouverture);
    assert!(r.html.contains("background:navy;font-weight:700"), "{}", r.html);
}

#[test]
fn adjectif_secable_et_son_contraire() {
    let mut e = docdg_transpiler::Engine::new();
    let s = e.render("<Affiche>un cadre sécable avec un fond jaune{\n\tTexte.\n}", false);
    assert!(s.html.contains("cadre secable"));
    let i = e.render("<Affiche>un cadre insécable avec un fond jaune{\n\tTexte.\n}", false);
    assert!(!i.html.contains("secable"));
    let d = e.render("<Affiche>un cadre avec un fond jaune{\n\tTexte.\n}", false);
    assert!(!d.html.contains("secable"));
}

#[test]
fn alinea_rendu_dans_les_blocs() {
    let mut e = docdg_transpiler::Engine::new();
    let cadre = e.render("<Affiche>un cadre avec un fond jaune{\n\tAlinéa dans le cadre.\n}", false);
    assert!(cadre.html.contains("width:0.8cm"), "{}", cadre.html);
    let libre = e.render("\tAlinéa au fil du texte.", false);
    assert!(libre.html.contains("width:0.8cm"), "{}", libre.html);
    let plat = e.render("<Affiche>un cadre avec un fond jaune{\nSans alinéa.\n}", false);
    assert!(!plat.html.contains("width:0.8cm"), "{}", plat.html);
}

#[test]
fn boucle_filtree_par_intervalle_dans_un_tableau() {
    let mut e = docdg_transpiler::Engine::new();
    let src = "<Dresse>un tableau [mc, mc, mc, mc]{\n\tRang\tTerme\tÉcart\tRapport(croissant)\n\tpour n de 14 à 20 {\n\t\tsi (n appartient à [16;20]) {\n\t\t\t[#n ; #{2^(n+1) - 1} ; #{2^n} ; 2,000]\n\t\t}\n\t}\n}";
    let r = e.render(src, false);
    assert_eq!(r.html.matches("<tr").count(), 6, "{}", r.html);
    assert!(r.html.contains(">16<"));
    assert!(r.html.contains(">131071<"));
    assert!(r.html.contains(">2097151<"));
    assert!(!r.html.contains(">14<"));
    assert!(!r.html.contains(">15<"));
}

#[test]
fn si_hors_boucle_voit_une_affectation_du_meme_segment() {
    let mut e = docdg_transpiler::Engine::new();
    let a = e.render("soit n = 18\nsi n plus de 16 {\n\tOui.\n}", false);
    assert!(a.html.contains("Oui."), "{}", a.html);
    let mut e2 = docdg_transpiler::Engine::new();
    let b = e2.render("soit n = 5\nsi n plus de 16 {\n\tOui.\n}\nsinon {\n\tNon.\n}", false);
    assert!(b.html.contains("Non."), "{}", b.html);
}

#[test]
fn comparateur_appartient_a_un_intervalle() {
    let mut e = docdg_transpiler::Engine::new();
    let a = e.render("soit n = 18\nsi n appartient à [16;20] {\n\tDedans.\n}\nsinon {\n\tDehors.\n}", false);
    assert!(a.html.contains("Dedans."), "{}", a.html);
    let mut e2 = docdg_transpiler::Engine::new();
    let b = e2.render("soit n = 25\nsi n appartient à [16;20] {\n\tDedans.\n}\nsinon {\n\tDehors.\n}", false);
    assert!(b.html.contains("Dehors."), "{}", b.html);
    let mut e3 = docdg_transpiler::Engine::new();
    let c = e3.render("soit n = 25\nsi n n'appartient pas à [16;20] {\n\tHors bornes.\n}", false);
    assert!(c.html.contains("Hors bornes."), "{}", c.html);
}

#[test]
fn algo2_bases_boucle_condition_pas_et_saisie() {
    // La saisie arrête le document : sans réponse, rien de ce qui la suit
    // n'existe. C'est le comportement enseigné par le document lui-même.
    let mut sans_reponse = docdg_transpiler::Engine::new();
    let src = include_str!("../../exemples/algo2.txt");
    let r0 = sans_reponse.render(src, true);
    assert!(r0.html.contains("180"));
    assert!(r0.html.contains("class=\"saisie\""));
    assert!(!r0.html.contains("Ligne 1 : le carré de 1 vaut 1."));

    let mut jeune = docdg_transpiler::Engine::new();
    jeune.saisies.insert("âge".to_string(), "16".to_string());
    let r1 = jeune.render(src, true);
    assert!(r1.html.contains("Tu es mineur(e)."), "{}", &r1.html[..r1.html.len().min(400)]);
    assert!(r1.html.contains("Ligne 1 : le carré de 1 vaut 1."));
    assert!(r1.html.contains("Admis avec 12"));
    // Le pas négatif : k va de 2 à 0 par -0,25, et l'étiquette remonte.
    assert!(r1.html.contains("Terme 1 : 2."));
    assert!(r1.html.contains("Terme 9 : 0."));

    let mut majeur = docdg_transpiler::Engine::new();
    majeur.saisies.insert("âge".to_string(), "20".to_string());
    let r2 = majeur.render(src, true);
    assert!(r2.html.contains("Tu es majeur(e)."));
}

#[test]
fn algo2_boucles_filtrees_diviseurs_ternaire_et_ou() {
    let mut e = docdg_transpiler::Engine::new();
    e.saisies.insert("âge".to_string(), "16".to_string());
    let src = include_str!("../../exemples/algo2.txt");
    let r = e.render(src, true);
    assert!(r.html.contains("Multiple"));
    assert_eq!(r.html.matches("<table").count(), 4);
    for d in [1, 2, 3, 4, 6, 7, 12, 14, 21, 28, 42, 84] {
        assert!(r.html.contains(&format!(">{}<", d)), "diviseur {} manquant", d);
    }
    assert!(r.html.contains(">16<") && r.html.contains(">20<"));
    assert!(r.html.contains("17 est impair."));
    assert!(r.html.contains("multiple de 15"));
    assert!(r.html.contains("Retenu par l'exception."));
    assert!(!r.html.contains("Écarté."));
    assert!(!r.html.contains("sinon {"));
    assert!(!r.html.contains("vaut 0 {"));
    assert!(r.html.contains("\\(d\\)"));
    assert!(r.html.contains("5050"), "somme 1..100 non accumulée");
    assert!(r.html.contains("120"), "factorielle 5 non accumulée");
    assert!(r.html.contains("84 possède 12 diviseurs"));
    assert!(r.html.contains("83 en possède 2"));
    assert!(!r.html.contains("#somme") && !r.html.contains("#diviseurs"));
    assert!(r.html.contains("1024") && r.html.contains("rang 10"));
    assert!(r.html.contains("n valant 10") && !r.html.contains("n valant 11"));
    assert!(!r.html.contains("**"));
}

#[test]
fn boucles_conditionnelles() {
    let mut e = docdg_transpiler::Engine::new();
    let a = e.render("soit n = 0\ntant que n < 3 faire {\n\tPassage #n\n\tsoit n = n + 1\n}", false);
    assert!(a.html.contains("Passage 0") && a.html.contains("Passage 2"), "{}", a.html);
    assert!(!a.html.contains("Passage 3"), "{}", a.html);

    let mut e2 = docdg_transpiler::Engine::new();
    let b = e2.render("soit n = 10\ntant que n < 3 faire {\n\tJamais #n\n}", false);
    assert!(!b.html.contains("Jamais"), "tant que doit pouvoir ne faire aucun tour : {}", b.html);

    let mut e3 = docdg_transpiler::Engine::new();
    let c = e3.render("soit n = 10\nfaire {\n\tUne fois #n\n\tsoit n = n + 1\n} tant que n < 3", false);
    assert!(c.html.contains("Une fois 10"), "faire doit s'exécuter au moins une fois : {}", c.html);
    assert!(!c.html.contains("Une fois 11"), "{}", c.html);
}

#[test]
fn accumulateur_traverse_une_boucle() {
    let mut e = docdg_transpiler::Engine::new();
    let a = e.render("soit s = 0\npour k de 1 à 5 {\n\tsoit s = s + k\n}\nSomme : #s", false);
    assert!(a.html.contains("Somme : 15"), "{}", a.html);
    let mut e2 = docdg_transpiler::Engine::new();
    let b = e2.render("soit p = 1\npour k de 1 à 6 {\n\tsoit p = p * k\n}\nProduit : #p", false);
    assert!(b.html.contains("Produit : 720"), "{}", b.html);
    let mut e3 = docdg_transpiler::Engine::new();
    let c = e3.render("soit c = 0\npour d de 1 à 12 {\n\tsi 12 % d vaut 0 {\n\t\tsoit c = c + 1\n\t}\n}\nDiviseurs : #c", false);
    assert!(c.html.contains("Diviseurs : 6"), "{}", c.html);
}

#[test]
fn conditions_composees_et_ou() {
    let mut e = docdg_transpiler::Engine::new();
    let a = e.render("soit n = 15\nsi n plus de 10 et n moins de 20 {\n\tDedans.\n}\nsinon {\n\tDehors.\n}", false);
    assert!(a.html.contains("Dedans."), "{}", a.html);
    let mut e2 = docdg_transpiler::Engine::new();
    let b = e2.render("soit n = 5\nsi n moins de 3 ou n plus de 4 {\n\tRetenu.\n}\nsinon {\n\tÉcarté.\n}", false);
    assert!(b.html.contains("Retenu."), "{}", b.html);
    let mut e3 = docdg_transpiler::Engine::new();
    let c = e3.render("soit n = 5\nsi (n plus de 10 et n moins de 20) ou n vaut 5 {\n\tException.\n}\nsinon {\n\tÉcarté.\n}", false);
    assert!(c.html.contains("Exception."), "{}", c.html);
}

#[test]
fn accumulateur_traverse_la_boucle() {
    let mut e = docdg_transpiler::Engine::new();
    let somme = e.render("soit s = 0\npour k de 1 à 100 {\n\tsoit s = s + k\n}\nTotal : #s", false);
    assert!(somme.html.contains("Total : 5050"), "{}", somme.html);

    let mut e2 = docdg_transpiler::Engine::new();
    let fact = e2.render("soit p = 1\npour k de 1 à 5 {\n\tsoit p = p * k\n}\n#p", false);
    assert!(fact.html.contains("120"), "{}", fact.html);

    let mut e3 = docdg_transpiler::Engine::new();
    let compte = e3.render("soit c = 0\npour d de 1 à 84 {\n\tsi 84 % d vaut 0 {\n\t\tsoit c = c + 1\n\t}\n}\n#c diviseurs.", false);
    assert!(compte.html.contains("12 diviseurs."), "{}", compte.html);

    let mut e4 = docdg_transpiler::Engine::new();
    let manuel = e4.render("soit s = 0\nsoit s = s + 5\nsoit s = s + 7\n#s", false);
    assert!(manuel.html.contains("12"), "{}", manuel.html);

    let mut e5 = docdg_transpiler::Engine::new();
    let croise = e5.render("soit tva = 0,2\nsoit prix = 150\nsoit ttc = prix * (1 + tva)\n#ttc", false);
    assert!(croise.html.contains("180"), "{}", croise.html);
}

#[test]
fn algo2_accumulateurs_et_compteurs() {
    let mut e = docdg_transpiler::Engine::new();
    e.saisies.insert("âge".to_string(), "16".to_string());
    let r = e.render(include_str!("../../exemples/algo2.txt"), true);
    assert!(r.html.contains("La somme des entiers de 1 à 100 vaut 5050."));
    assert!(r.html.contains("La factorielle de 5 vaut 120."));
    assert!(r.html.contains("84 possède 12 diviseurs"));
    assert!(r.html.contains("83 en possède 2"));
    // Les boucles conditionnelles et les ruptures, ajoutées au document.
    assert!(r.html.contains("est 1024, atteinte au rang 10"));
    assert!(r.html.contains("Passage unique, avec n valant 10."));
    assert!(r.html.contains("Le plus petit diviseur de 91 autre que 1 est 7."));
    assert!(!r.html.contains("soit "));
}

#[test]
fn algo3_les_conteneurs() {
    let mut e = docdg_transpiler::Engine::new();
    let src = include_str!("../../exemples/algo3.txt");
    let r = e.render(src, true);
    assert!(r.html.contains("{12,5 ; 15 ; 14}"), "écriture indexée");
    assert!(r.html.contains("indice hors bornes"));
    assert!(r.html.contains("{1 ; 4 ; 9 ; 16 ; 25}"), "carrés par concaténation");
    assert!(r.html.contains("{Marche: 5 ; Bus: 3 ; Vélo: 7}"));
    assert!(r.html.contains("clé absente"));
    assert!(r.html.contains("Marche : 5 minutes"));
    assert!(r.html.contains("{{1 ; 2} ; {3 ; 4}}"), "matrice littérale");
    assert!(r.html.contains("{{5 ; 6} ; {7 ; 8}}"), "matrice bloc");
    assert!(r.html.contains("{{1 ; 2 ; 3} ; {2 ; 4 ; 6} ; {3 ; 6 ; 9}}"), "remplissage indexé");
    assert!(r.html.contains("centrale : 4"));
    assert!(r.html.contains("{{0 ; 0 ; 0 ; 0} ; {0 ; 1 ; 2 ; 3} ; {0 ; 2 ; 4 ; 6}}"), "composition");
    assert!(r.html.contains("2,5 n'est pas un entier"));
    for fuite in ["#notes", "#S", "#trajets", "#A", "#B", "#T", "#M", "notes[2] = 14"] {
        assert!(!r.html.contains(fuite), "fuite {}", fuite);
    }
}

#[test]
fn conteneurs_gestes_elementaires() {
    let mut e = docdg_transpiler::Engine::new();
    let a = e.render("soit notes: une liste de décimaux = {12,5 ; 15 ; 9,5}\nDeuxième : #notes[1], double : #{notes[1] * 2}", false);
    assert!(a.html.contains("Deuxième : 15, double : 30"), "{}", a.html);
    let mut e2 = docdg_transpiler::Engine::new();
    let b = e2.render("soit S: une liste d'entiers = {}\npour k de 1 à 5 {\n\tsoit S = S + {k}\n}\nS : #S", false);
    assert!(b.html.contains("S : {1 ; 2 ; 3 ; 4 ; 5}"), "{}", b.html);
    let mut e3 = docdg_transpiler::Engine::new();
    let c = e3.render("soit mots: une liste de chaînes de caractères = {chat ; chien ; cheval}\npour m dans mots {\n\tMot : #m\n}", false);
    assert!(c.html.contains("Mot : chat") && c.html.contains("Mot : cheval"), "{}", c.html);
}

#[test]
fn les_quatre_types_de_nombres() {
    let mut e = docdg_transpiler::Engine::new();
    let a = e.render("soit d: une liste de décimaux = {12,5 ; 0,25 ; 15}\n#d", false);
    assert!(a.html.contains("{12,5 ; 0,25 ; 15}"), "{}", a.html);

    let mut e2 = docdg_transpiler::Engine::new();
    let b = e2.render("soit d: une liste de décimaux = {1/3}\n#d", false);
    assert!(b.html.contains("n'est pas un décimal"), "D doit refuser 1/3 : {}", b.html);

    let mut e3 = docdg_transpiler::Engine::new();
    let c = e3.render("soit r: une liste de réels = {1/3 ; racine(2)}\n#r", false);
    assert!(!c.html.contains("n'est pas"), "R doit accepter 1/3 : {}", c.html);

    let mut e4 = docdg_transpiler::Engine::new();
    let d = e4.render("soit z: une liste de complexes = {(1 ; 2) ; (0 ; -1)}\nTous : #z — second : #z[1]", false);
    assert!(d.html.contains("Tous : {(1 ; 2) ; (0 ; -1)}"), "{}", d.html);
    assert!(d.html.contains("second : (0 ; -1)"), "{}", d.html);

    let mut e5 = docdg_transpiler::Engine::new();
    let f = e5.render("soit z: une liste de complexes = {(1 ; 2 ; 3)}\n#z", false);
    assert!(f.html.contains("couple de deux réels"), "{}", f.html);
}

#[test]
fn fonctions_algorithmiques() {
    let mut e = docdg_transpiler::Engine::new();
    let a = e.render("soit addition(a: entier ; b: entier): entier = a + b\nR : #{addition(3 ; 7)}", false);
    assert!(a.html.contains("R : 10"), "{}", a.html);

    let mut e2 = docdg_transpiler::Engine::new();
    let b = e2.render("soit factorielle(n: entier): entier = si n vaut 0 { 1 } sinon { n * factorielle(n - 1) }\n5! = #{factorielle(5)}", false);
    assert!(b.html.contains("5! = 120"), "récursivité : {}", b.html);

    let mut e3 = docdg_transpiler::Engine::new();
    let c = e3.render("soit h(a: réel ; b: réel): réel = {\n\tsoit c = a*a + b*b\n\tretourne racine(c)\n}\nH : #{h(3 ; 4)}", false);
    assert!(c.html.contains("H : 5"), "corps multiple : {}", c.html);

    let mut e4 = docdg_transpiler::Engine::new();
    let d = e4.render("soit add(a: entier ; b: entier): entier = a + b\n#{add(2,5 ; 7)}", false);
    assert!(d.html.contains("n'est pas un entier"), "type d'argument : {}", d.html);

    let mut e5 = docdg_transpiler::Engine::new();
    let f = e5.render("soit add(a: entier ; b: entier): entier = a + b\n#{add(3)}", false);
    assert!(f.html.contains("attend 2 argument"), "arité : {}", f.html);

    let mut e6 = docdg_transpiler::Engine::new();
    let g = e6.render("soit f(a: entier): entier = { soit x = a }\n#{f(3)}", false);
    assert!(g.html.contains("il manque"), "retourne absent : {}", g.html);

    let mut e7 = docdg_transpiler::Engine::new();
    let h = e7.render("soit carré(n: entier): entier = n * n\nsoit S: une liste d'entiers = {}\npour k de 1 à 4 {\n\tsoit S = S + {carré(k)}\n}\nS : #S", false);
    assert!(h.html.contains("S : {1 ; 4 ; 9 ; 16}"), "appel dans un littéral : {}", h.html);
}

#[test]
fn algo3_les_fonctions() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(include_str!("../../exemples/algo3.txt"), true);
    assert!(r.html.contains("{1 ; 4 ; 9 ; 16 ; 25 ; 36}"));
    assert!(r.html.contains("n'est pas un entier") && r.html.contains("attend 2 argument"));
    for d in [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60] {
        assert!(r.html.contains(&format!(">{}<", d)), "diviseur {}", d);
    }
    assert!(!r.html.contains("#somme"));
    // la moyenne d'une collection reçue en argument
    assert!(r.html.contains("13,2"));
}

#[test]
fn algo4_recursivite_et_algorithmes() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(include_str!("../../exemples/algo4.txt"), true);
    // récursivité : factorielle et Fibonacci
    assert!(r.html.contains("720") && r.html.contains("89"));
    // les tris et la fusion
    assert!(r.html.contains("1 ; 2 ; 5 ; 5 ; 6 ; 9"));
    assert!(r.html.contains("1 ; 2 ; 3 ; 4 ; 7 ; 8 ; 9"));
    // les structures de données et le p-uplet
    assert!(r.html.contains("sommet"));
    assert!(r.html.contains("17 = 5"));
    // les quatre piliers de la programmation orientée objet
    assert!(r.html.contains("distance à l'origine 5"));
    assert!(r.html.contains("ouaf") && r.html.contains("miaou"));
    assert!(r.html.contains("3 nœuds"));
    assert!(r.html.contains("ABCD"));
    // trois fautes, et trois seulement : les démonstrations volontaires
    // — attribut privé, méthode privée, classe abstraite
    assert_eq!(r.html.matches("calcul-absent").count(), 3);
}
