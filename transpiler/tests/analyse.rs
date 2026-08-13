use docdg_transpiler::Engine;

#[test]
fn analyse_college_complet() {
    let src = include_str!("analyse2.txt");
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(!r.html.contains("calcul-absent"));
    // la déclaration nomme avant de qualifier, et se lit comme une phrase
    assert!(r.html.contains("Soit \\(f\\) la fonction définie par \\(f(x) = 2x + 1\\)."));
    assert!(r.html.contains("f(3) = 2\\times 3 + 1 = 7"));
    assert!(r.html.contains("\\mathscr{S} = \\left\\{3\\right\\}"));
    assert!(r.html.contains("g(5) = -3\\times 5 = -15"));
    assert!(!r.html.contains("est posée"));
}

#[test]
fn mini_langage_analyse() {
    let mut e = Engine::new();
    let r = e.render(
        "$produit(k=1;n) k$ et $intégrale(x=0;1) x^2 = 1/3$ et $réunion de(i=1;n) A_i$",
        true,
    );
    assert!(r.html.contains("\\prod_{k=1}^{n} k"));
    assert!(r.html.contains("\\int_{0}^{1} x^2\\,\\mathrm{d}x = \\dfrac{1}{3}"));
    assert!(r.html.contains("\\bigcup_{i=1}^{n} A_i"));

    let r = e.render("$lim(x->0) sin(x)/x = 1$ et $tan(x) = sin(x)/cos(x)$", true);
    assert!(r.html.contains("\\lim\\limits_{x\\to 0} \\dfrac{\\sin(x)}{x}"));
    assert!(r.html.contains("\\dfrac{\\sin(x)}{\\cos(x)}"));

    let r = e.render("$dd(f)/dd(x)$ et $partielle(f)/partielle(x)$", true);
    assert!(r.html.contains("\\dfrac{\\mathrm{d}f}{\\mathrm{d}x}"));
    assert!(r.html.contains("\\dfrac{\\partial f}{\\partial x}"));

    let r = e.render(
        "$laplacien(f)$, $dérivée directionnelle(f; v)$, $la transformée de Laplace(f)$",
        true,
    );
    assert!(r.html.contains("\\Delta f"));
    assert!(r.html.contains("\\nabla_{v} f"));
    assert!(r.html.contains("\\mathcal{L}\\left\\{f\\right\\}"));

    let r = e.render("$somme(k=1;n) k = (n(n+1))/2$", true);
    assert!(r.html.contains("\\dfrac{n(n+1)}{2}"));
}

#[test]
fn analyse_lycee_complet() {
    let src = include_str!("analyse3.txt");
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(!r.html.contains("calcul-absent"));

    assert_eq!(r.html.matches("<table class=\"signes\">").count(), 7);
    assert!(r.html.contains("<th>\\(x + 2\\)</th>"));
    assert!(r.html.contains("\\nearrow"));
    assert!(r.html.contains("\\searrow"));
    assert!(r.html.contains("class=\"haut var\""));
    assert!(r.html.contains("class=\"bas var\""));

    assert!(r.html.contains("h''(x) = 6 x - 6"));
    assert!(r.html.contains("convexe sur \\([1\\,;\\,+\\infty[\\)"));
    assert!(r.html.contains("Point d'inflexion en \\(x = 1\\)"));
    assert!(r.html.contains("La courbe n'a pas de point d'inflexion."));

    assert!(r.html.contains("\\(y = 2 x + 2\\) est asymptote oblique"));
    assert!(r.html.contains("\\(x = 1\\) est asymptote verticale"));
    assert!(r.html.contains("\\(y = 3\\) est asymptote horizontale"));

    assert!(r.html.contains("valeur remarquable \\(\\cos\\left(\\frac{\\pi}{3}\\right)"));
    assert!(r.html.contains("x = \\frac{\\pi}{3} + 2k\\pi"));
    assert!(r.html.contains("x = \\pi - \\frac{\\pi}{4} + 2k\\pi"));
    assert!(r.html.contains("x = \\frac{\\pi}{3} + k\\pi"));

    assert_eq!(r.html.matches("<svg").count(), 9);
    assert!(r.html.contains("class=\"aire\""));
    assert!(r.html.contains("class=\"escalier\""));

    let dessins: Vec<&str> = r
        .html
        .split("<svg")
        .skip(1)
        .map(|s| s.split("</style>").last().unwrap_or(""))
        .collect();
    assert_eq!(dessins.len(), 9);

    let details: Vec<(usize, usize)> = dessins
        .iter()
        .map(|d| {
            (
                d.matches("class=\"tangente\"").count(),
                d.matches("class=\"asymptote\"").count(),
            )
        })
        .collect();

    assert_eq!(
        details,
        vec![(2, 1), (0, 0), (0, 0), (0, 0), (0, 0), (2, 0), (0, 2), (0, 0), (1, 0)]
    );

    let table = r
        .html
        .split("<table class=\"signes\">")
        .nth(4)
        .unwrap_or("")
        .split("</table>")
        .next()
        .unwrap_or("");
    assert!(table.contains("- \\frac{1}{3}"));
    assert!(table.contains("\\frac{\\sqrt{3}}{3}"));
    assert!(!table.contains("\\infty i"));
    assert_eq!(table.matches("\\|").count(), 2);
    assert!(r.html.contains("\\begin{cases}"));
}

#[test]
fn analyse_superieur_complet() {
    let src = include_str!("analyse4.txt");
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(!r.html.contains("calcul-absent"));

    assert!(r.html.contains("y(x) = C_{1} \\sin{\\left(2 x \\right)} + C_{2} \\cos{\\left(2 x \\right)}"));
    assert!(r.html.contains("N(t) = C_{1} e^{- 0{,}12 t}"));
    assert!(r.html.contains("u(x, y) = F{\\left(x - y \\right)}"));

    assert!(r.html.contains("L'intégrale \\(\\int_{1}^{+\\infty} p(x)\\,\\mathrm{d}x\\) converge, et vaut \\(1\\)"));
    assert!(r.html.contains("q(x)\\,\\mathrm{d}x\\) diverge"));

    assert!(r.html.contains("m(x) \\underset{x \\to 0}{\\sim} - \\frac{x^{3}}{6}"));
    assert!(r.html.contains("n(x) \\underset{x \\to +\\infty}{\\sim} x"));

    assert!(r.html.contains("converge, et \\(\\sum_{n=1}^{+\\infty} \\frac{1}{n^{2}} = \\frac{\\pi^{2}}{6}\\)"));
    assert!(r.html.contains("\\nabla \\psi"));
    assert!(r.html.contains("\\mathrm{H}_{\\psi}"));
    assert!(r.html.contains("\\dfrac{\\partial \\psi}{\\partial y} = 2 y"));
    assert!(r.html.contains("point col"));
    assert!(r.html.contains("minimum local"));

    assert!(r.html.contains("la série de Fourier de \\(cr\\), tronquée à l'ordre 4"));
    assert!(r.html.contains("\\mathcal{L}[ex](p) = \\int_{0}^{+\\infty}"));
    assert!(r.html.contains("L'originale de \\(Fp\\) est"));
    assert!(r.html.contains("la famille \\((co, si)\\) est libre"));
    assert!(r.html.contains("le wronskien est identiquement nul"));
}

#[test]
fn aire_entre_deux_courbes() {

    let src = "<On pose>une fonction p(x) = -x^2/4 + 3\n\n\
               <On pose>une fonction v(x) = x/2\n\n\
               <Représente>graphiquement la fonction p sur [-1 ; 4] en abscisse \
               et [-1 ; 4] en ordonnée, avec l'aire jusqu'à v et area:{0, 3}\n";
    let mut e = Engine::new();
    let r = e.render(src, true);
    let dessin = r.html.split("</style>").last().unwrap_or("");
    assert_eq!(dessin.matches("class=\"aire\"").count(), 1);
    assert_eq!(dessin.matches("class=\"courbe\"").count(), 2);

    let r2 = e.render(
        &src.replace("et area:{0, 3}", "entre 0 et 3"),
        true,
    );
    assert_eq!(
        r2.html.split("</style>").last().unwrap_or("").matches("class=\"aire\"").count(),
        1
    );
}

#[test]
fn partie_entiere_tracee_en_marches() {
    let src = "<Soit>une fonction pe(x) = E(x)\n\n\
               <Trace>dans un repère où l'abscisse appartient à [-3 ; 4] \
               et l'ordonnée à [-4 ; 5] {\n\tla courbe de la fonction pe\n}\n";
    let mut e = Engine::new();
    let r = e.render(src, true);
    let dessin = r.html.split("</style>").last().unwrap_or("");

    assert_eq!(dessin.matches("class=\"courbe\"").count(), 7);
    for chemin in dessin.split("<path d=\"").skip(1) {
        let points: Vec<(f64, f64)> = chemin
            .split('"')
            .next()
            .unwrap_or("")
            .split_whitespace()
            .filter_map(|p| p.trim_start_matches(['M', 'L']).split_once(','))
            .filter_map(|(a, b)| Some((a.parse::<f64>().ok()?, b.parse::<f64>().ok()?)))
            .collect();
        let hauteurs: Vec<f64> = points.iter().map(|(_, y)| *y).collect();
        let ecart = hauteurs
            .iter()
            .cloned()
            .fold(f64::MIN, f64::max)
            - hauteurs.iter().cloned().fold(f64::MAX, f64::min);
        assert!(ecart < 0.5, "marche non horizontale : écart {}", ecart);
    }

    assert_eq!(dessin.matches("class=\"point\"").count(), 7);
    assert_eq!(dessin.matches("class=\"point creux\"").count(), 7);

    let abscisses: Vec<(f64, f64)> = dessin
        .split("<path d=\"")
        .skip(1)
        .filter_map(|chemin| {
            let points = chemin.split('"').next()?;
            let bout = |p: &str| p.trim_start_matches(['M', 'L']).split_once(',')?.0.parse().ok();
            Some((
                bout(points.split_whitespace().next()?)?,
                bout(points.split_whitespace().last()?)?,
            ))
        })
        .collect();
    for paire in abscisses.windows(2) {
        assert!(
            (paire[1].0 - paire[0].1).abs() < 0.01,
            "trou entre deux marches : {} puis {}",
            paire[0].1,
            paire[1].0
        );
    }
}

#[test]
fn le_tableau_se_tient_dans_le_domaine() {
    let mut e = Engine::new();

    let r = e.render(
        "<Soit>une fonction q(x) = |x|racine((1-x)/(1+3x))\n\n\
         <Dresse>le tableau de variations de q\n",
        true,
    );
    assert!(!r.html.contains("calcul-absent"));
    assert!(r.html.contains("- \\frac{1}{3}"));
    assert!(r.html.contains("\\frac{\\sqrt{3}}{3}"));

    assert!(!r.html.contains("-\\infty"));
    assert!(!r.html.contains("\\infty i"));
    assert_eq!(r.html.matches("\\|").count(), 2);

    let r = e.render(
        "<Soit>une fonction u(x) = racine(x^2 - 1)\n\n\
         <Dresse>le tableau de variations de u\n",
        true,
    );
    assert!(!r.html.contains("calcul-absent"));
    assert_eq!(r.html.matches("class=\"hachure").count(), 2);

    let r = e.render(
        "<Soit>une fonction v(x) = x^3 - 3x + 1\n\n\
         <Dresse>le tableau de variations de v\n",
        true,
    );
    assert!(r.html.contains("-\\infty"));
    assert!(r.html.contains("+\\infty"));
    assert_eq!(r.html.matches("class=\"borne\"").count(), 8);
}

#[test]
fn la_convexite_se_tient_dans_le_domaine() {
    let mut e = Engine::new();

    let r = e.render(
        "<Soit>une fonction r(x) = ln(x)\n\n<Étudie>la convexité de r\n",
        true,
    );
    assert!(!r.html.contains("calcul-absent"));
    assert!(r.html.contains("concave sur \\(]0\\,;\\,+\\infty[\\)"));
    assert!(!r.html.contains("-\\infty"));

    let r = e.render(
        "<Soit>une fonction c(x) = |x - 2|\n\n<Étudie>la convexité de c\n",
        true,
    );
    assert!(!r.html.contains("calcul-absent"));
    assert!(r.html.contains("affine sur"));
    assert!(!r.html.contains("convexe"));

    let r = e.render(
        "<Soit>une fonction q(x) = |x|racine((1-x)/(1+3x))\n\n\
         <Étudie>la convexité de q\n",
        true,
    );
    assert!(!r.html.contains("calcul-absent"));
    assert!(r.html.contains("convexe sur \\(]- \\frac{1}{3}\\,;\\,0[\\)"));
    assert!(r.html.contains("concave sur \\(]0\\,;\\,1]\\)"));
}

#[test]
fn un_pole_ne_recoit_pas_de_rond() {
    let src = "<Soit>une fonction g(x) = (x+1)/(x-2)\n\n\
               <Trace>dans un repère où l'abscisse appartient à [-4 ; 8] \
               et l'ordonnée à [-6 ; 8] {\n\tla courbe de la fonction g\n}\n";
    let mut e = Engine::new();
    let r = e.render(src, true);
    let dessin = r.html.split("</style>").last().unwrap_or("");

    assert_eq!(dessin.matches("class=\"point").count(), 0);
    assert_eq!(dessin.matches("class=\"courbe\"").count(), 2);
}

#[test]
fn tableau_de_signes_marque_la_valeur_interdite() {
    let src = "<Soit>une fonction g(x) = (x+1)/(x-2)\n\n<Dresse>le tableau de signes de g\n";
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(r.html.contains("<th>\\(x + 1\\)</th>"));
    assert!(r.html.contains("<th>\\(x - 2\\)</th>"));
    assert!(r.html.contains("\\(\\|\\)"));
}

#[test]
fn racine_cubique_definie_sur_les_negatifs() {
    let trace = "<Trace>dans un repère où l'abscisse appartient à [-5 ; 5] \
                 et l'ordonnée à [-5 ; 5] {\n\tla courbe de la fonction cr\n}\n";
    let pose = "<Soit>une fonction cr(x) = racine[3](x)\n\n";
    let mut e = Engine::new();

    let r = e.render(&format!("{}{}", pose, trace), true);
    assert!(r.html.contains("class=\"courbe\""));
    assert!(!r.html.contains("<line") || !r.html.contains("class=\"tangente\" marker-start"));

    let r = e.render(
        &format!("{}<Dresse>le tableau de variations de cr\n\n{}", pose, trace),
        true,
    );
    let dessin = r.html.split("</style>").last().unwrap_or("");
    assert!(dessin.contains("class=\"tangente\""), "{}", dessin);
}

#[test]
fn le_pluriel_traite_toutes_les_fonctions() {
    fn rendu(src: &str) -> String {
        Engine::new().render(src, true).html
    }
    let h = rendu(
        "<Soit>les fonctions f(x) = exp(-x^2), g(x) = -x^4 + 2x^2 + 1 et h(x) = (x+1)/(x-2)\n\n\
         <Dresse>les tableaux de variations de f, g, et h\n\n\
         <Dresse>les tableaux de signes de f, g, et h\n\n\
         <Détermine>les zéros de f, g et h\n\n\
         <Représente>graphiquement les fonctions f, g et h pour x dans [-2 ; 2] \
         et y dans [-3 ; 3], avec 200 échantillons\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("<svg").count(), 3);
    assert!(h.contains("f(x) = 0 \\iff"));
    assert!(h.contains("g(x) = 0 \\iff"));
    assert!(h.contains("h(x) = 0 \\iff"));
    assert!(h.contains("f'(x)"));
    assert!(h.contains("g'(x)"));
    assert!(h.contains("h'(x)"));
    assert!(h.contains(">h(x)</span>") || h.contains("h(x)"));
}

#[test]
fn les_deux_modes_de_rendu_donnent_le_meme_document() {
    for source in [
        include_str!("factorisation.txt"),
        include_str!("analyse3.txt"),
        include_str!("geometrie3.txt"),
        include_str!("statistiques-probabilites3.txt"),
    ] {
        let sequentiel = Engine::new().render(source, false).html;
        let parallele = Engine::new().render(source, true).html;
        assert_eq!(sequentiel, parallele);
    }
}

#[test]
fn les_courbes_etudiees_portent_tangentes_et_asymptotes() {
    for parallele in [false, true] {
        let h = Engine::new()
            .render(include_str!("factorisation.txt"), parallele)
            .html;
        assert_eq!(h.matches("class=\"tangente\"").count(), 4);
        assert_eq!(h.matches("class=\"asymptote\"").count(), 3);
    }
}
