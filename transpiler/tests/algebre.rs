use docdg_transpiler::Engine;

#[test]
fn algebre_college_complet() {
    let src = include_str!("algebre2.txt");
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(!r.html.contains("calcul-absent"));
    assert!(r.html.contains("On met les fractions au même dénominateur, 6"));
    assert!(r.html.contains("\\dfrac{3 + 2}{6} = \\dfrac{5}{6}"));
    assert!(r.html.contains("Diviser par une fraction, c'est multiplier par son inverse"));
    assert!(r.html.contains("\\dfrac{2}{3}"));
    assert!(r.html.contains("4{,}56 \\times 10^{4}"));
    assert!(r.html.contains("3{,}2 \\times 10^{-4}"));
    assert!(r.html.contains("La somme des chiffres de 456"));
    assert!(r.html.contains("47 = 5 \\times 9 + 2"));
    assert!(r.html.contains("On part de \\(5\\)"));
    assert!(r.html.contains("Le programme renvoie \\(12\\)"));
    assert!(r.html.contains("2 \\times \\left(x + 3\\right) - 4"));
    assert!(r.html.contains("\\operatorname{PGCD}(84\\,;\\,60) = 12"));
    assert!(r.html.contains("\\mathscr{S} = \\left\\{4\\right\\}"));
    assert!(r.html.contains("\\mathscr{S} = \\left\\{6\\right\\}"));
}

#[test]
fn algebre_lycee_complet() {
    let src = include_str!("algebre3.txt");
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(!r.html.contains("calcul-absent"));
    // la déclaration est une phrase, et non une formule enveloppant du texte
    assert!(r.html.contains("Soit \\((s)\\) le système"));
    assert!(r.html.contains("Étape 1. Pivot sur \\(x\\)"));
    assert!(r.html.contains("L_{2} \\leftarrow L_{2} - 4\\,L_{1}"));
    assert!(r.html.contains("L_{3} \\leftarrow L_{3} - \\left(-6\\right)\\,L_{2}"));
    assert!(r.html.contains("La solution du système est \\(\\left(x = 2,\\ y = 1\\right)\\)"));
    assert!(r.html.contains("La solution du système est \\(\\left(x = 1,\\ y = -2,\\ z = 3\\right)\\)"));
    assert!(r.html.contains("\\Delta = \\left(-3\\right)^{2} - 4 \\times 2 \\times 1 = 1"));
    assert!(r.html.contains("x_1 = \\dfrac{1}{2}"));
    assert!(r.html.contains("racine double"));
    assert!(r.html.contains("aucune racine réelle"));
    assert!(r.html.contains("\\dfrac{0 \\pm \\sqrt{8}}{2}"));
    assert!(r.html.contains("u_{0} = 1,\\ u_{1} = 3"));
    assert!(r.html.contains("u_{5} = 63"));
    assert!(r.html.contains("arithmétique de premier terme"));
    assert!(r.html.contains("géométrique de premier terme"));
    assert!(r.html.contains("3x \\equiv 4 \\pmod{7}"));
    assert!(r.html.contains("\\operatorname{Vect}(u, v)"));
    assert!(r.html.contains("\\operatorname{Tr}(A)"));
    assert!(r.html.contains("\\operatorname{Ker}(A)"));
    assert!(r.html.contains("A^{*}"));
    assert!(r.html.contains("\\begin{array}{c|cccc}+ & 0 & 1 & 2 & 3"));
    assert!(r.html.contains("\\dbinom{10}{3}"));
    assert!(r.html.contains("360 = 2^{3} \\times 3^{2} \\times 5"));
    assert!(r.html.contains("relation de Bézout"));
    assert!(r.html.contains("\\sqrt{2}\\,\\mathrm{e}^{\\mathrm{i}\\frac{\\pi}{4}}"));
    assert!(r.html.contains("<svg"));
    assert!(r.html.contains("La matrice d'adjacence de \\(G\\)"));
    assert!(r.html.contains("chemin de longueur 3"));
    assert!(r.html.contains("(4 + 5k\\,;\\,-2 - 3k)"));
    assert!(r.html.contains("n'a aucune solution entière"));
}

#[test]
fn algebre_superieur_complet() {
    let src = include_str!("algebre4.txt");
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(!r.html.contains("calcul-absent"));
    assert!(r.html.contains("\\operatorname{Sp}(A) = \\left\\{1, 3\\right\\}"));
    assert!(r.html.contains("\\operatorname{Sp}(B) = \\left\\{3\\ (\\times 2)\\right\\}"));
    assert!(r.html.contains("A = PDP^{-1}"));
    assert!(r.html.contains("\\operatorname{rg}(C) = 2"));
    assert!(r.html.contains("\\operatorname{Ker} C = \\operatorname{Vect}"));
    assert!(r.html.contains("\\operatorname{Im} C = \\operatorname{Vect}"));
    assert!(r.html.contains("\\chi_{D}(X) = \\left(X - 3\\right)^{2} \\left(X - 2\\right)"));
    assert!(r.html.contains("\\pi_{D}(X) = \\left(X - 3\\right)^{2} \\left(X - 2\\right)"));
    assert!(r.html.contains("triangulaire supérieure"));
    assert!(r.html.contains("La division euclidienne donne \\(X^{3} + 2 X - 1 = (X^{2} + 1)(X) + X - 1\\)"));
    assert!(r.html.contains("(X^{3} - 1) \\wedge (X^{2} - 1) = X - 1"));
    assert!(r.html.contains("Dans \\(\\mathbb{R}[X]\\)"));
    assert!(r.html.contains("\\left(X^{2} + 1\\right)"));
    assert!(r.html.contains("Dans \\(\\mathbb{C}[X]\\)"));
    assert!(r.html.contains("\\left(X - i\\right)"));
}

#[test]
fn etat_stable_markov_exact() {
    let src = "<Soit>la matrice M {\n\t0,5 ; 0,5\n\t0,25 ; 0,75\n}\n\n<Calcule>l'état stable de M\n";
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(r.html.contains("L'état stable de la chaîne de matrice de transition"));
    assert!(r.html.contains("\\frac{1}{3} & \\frac{2}{3}"));
    assert!(r.html.contains("\\pi = \\pi M"));
}

#[test]
fn systeme_sans_solution() {
    let src = "<Soit>le système s {\n\tx + y = 1\n\tx + y = 2\n}\n\n<Résous>le système s\n";
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(r.html.contains("Le système n'a pas de solution."));
}

#[test]
fn commande_au_fil_du_texte() {
    let mut e = Engine::new();
    let r = e.render(
        "La solution est : <Résous>l'équation 3x + 5 = 17",
        true,
    );
    assert!(r.html.contains(
        "La solution est : <span class=\"calcul-en-ligne\">\\(\\mathscr{S} = \\left\\{4\\right\\}\\)</span>"
    ));
    assert!(!r.html.contains("calcul-absent"), "{}", r.html);

    let r = e.render("Le produit vaut <Calcule>3/4 * 2/5", true);
    assert!(r.html.contains("\\(\\dfrac{3}{4} \\times \\dfrac{2}{5} = \\dfrac{3}{10}\\)"));

    let r = e.render("Au dixième près, <Écris>le nombre 0,00032 en notation scientifique", true);
    assert!(r.html.contains("\\(0{,}00032 = 3{,}2 \\times 10^{-4}\\)"));

    let r = e.render("On a <Effectue>la division euclidienne de 47 par 5", true);
    assert!(r.html.contains("\\(47 = 5 \\times 9 + 2\\)"));

    let r = e.render("Il y a <Dénombre>les combinaisons de 3 parmi 10 poignées.", true);
    assert!(r.html.contains("\\(\\dbinom{10}{3} = 120\\)"));

    let src = "<Soit>la matrice M {\n\t2 ; 1\n\t1 ; 1\n}\n\nLe déterminant vaut <Calcule>le déterminant de M et <Calcule>l'inverse de M";
    let r = e.render(src, true);
    assert!(r.html.contains("\\(\\det(M) = 1\\)"));
    assert!(r.html.contains("\\(M^{-1} ="));
}

#[test]
fn commande_sans_forme_compacte_bascule_en_bloc() {
    let mut e = Engine::new();
    let r = e.render("Un cas : <Vérifie>si 456 est divisible par 3", true);
    assert!(r.html.contains("<p>Un cas :</p>"));
    assert!(r.html.contains("La somme des chiffres de 456"));
}

#[test]
fn commande_tabulee_garde_son_decalage() {
    let mut e = Engine::new();
    let r = e.render("\t<Calcule>7/6 - 3/4", true);
    assert!(r.html.contains("margin-left:1cm"), "{}", r.html);
    assert!(r.html.contains("On met les fractions au même dénominateur, 12"));
}
