use docdg_transpiler::Engine;

fn rendu(src: &str) -> String {
    Engine::new().render(src, true).html
}

#[test]
fn pythagore_dans_les_deux_sens() {
    let h = rendu(
        "<Calcule>AC dans le triangle ABC rectangle en B, avec AB = 3 et BC = 4\n\n\
         <Calcule>EF dans le triangle DEF rectangle en F, avec DE = 13 et DF = 5\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("AC^2 = AB^2 + BC^2"));
    assert!(h.contains("AC^2 = 3^2 + 4^2 = 25"));
    assert!(h.contains("AC = 5"));
    assert!(h.contains("EF^2 = DE^2 - DF^2"));
    assert!(h.contains("EF = 12"));
}

#[test]
fn reciproque_de_pythagore_tranche() {
    let h = rendu("<Vérifie>si le triangle ABC est rectangle, avec AB = 3, BC = 4 et AC = 5\n");
    assert!(h.contains("est rectangle en \\(B\\)"));

    let h = rendu("<Vérifie>si le triangle RST est rectangle, avec RS = 4, ST = 5 et RT = 6\n");
    assert!(h.contains("n'est pas rectangle"));
}

#[test]
fn thales_dans_les_deux_configurations() {
    let h = rendu("<Calcule>AC par le théorème de Thalès, avec AM = 3, AB = 6 et AN = 4\n");
    assert!(h.contains("AC = 8"));

    let h = rendu("<Calcule>MN par le théorème de Thalès, avec AM = 3, AB = 6 et BC = 10\n");
    assert!(h.contains("MN = 5"));

    let h = rendu(
        "<Vérifie>si les droites (MN) et (BC) sont parallèles, \
         avec AM = 3, AB = 6, AN = 4 et AC = 8\n",
    );
    assert!(h.contains("sont parallèles"));
}

#[test]
fn trigonometrie_du_triangle_rectangle() {
    let h = rendu(
        "<Calcule>BC dans le triangle ABC rectangle en B, avec l'angle A = 30 degrés et AC = 8\n",
    );
    assert!(h.contains("\\sin"));
    assert!(h.contains("BC = 4"));

    let h = rendu(
        "<Calcule>AB dans le triangle ABC rectangle en B, avec l'angle A = 40 degrés et AC = 10\n",
    );
    assert!(h.contains("\\cos"));
    assert!(h.contains("AB \\approx 7{,}66"));

    let h = rendu("<Calcule>l'angle A dans le triangle ABC rectangle en B, avec AB = 3 et AC = 6\n");
    assert!(h.contains("\\widehat{A} = 60^\\circ"));
}

#[test]
fn mesures_usuelles_et_conversions() {
    let h = rendu(
        "<Calcule>le périmètre du rectangle de longueur 7 et de largeur 4\n\n\
         <Calcule>l'aire du disque de rayon 3\n\n\
         <Calcule>le volume de la boule de rayon 3\n\n\
         <Convertis>3,5 km en m\n\n\
         <Convertis>2500 cm^2 en m^2\n\n\
         <Convertis>3 L en cm^3\n\n\
         <Convertis>1250 g en kg\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("= 22"));
    assert!(h.contains("\\approx 28{,}27"));
    assert!(h.contains("\\approx 113{,}1"));
    assert!(h.contains("3{,}5\\ \\mathrm{km} = 3500\\ \\mathrm{m}"));
    assert!(h.contains("2500\\ \\mathrm{cm^2} = 0{,}25\\ \\mathrm{m^2}"));
    assert!(h.contains("3\\ \\mathrm{L} = 3000\\ \\mathrm{cm^3}"));
    assert!(h.contains("1250\\ \\mathrm{g} = 1{,}25\\ \\mathrm{kg}"));
}

#[test]
fn une_conversion_incoherente_ne_repond_pas() {
    let h = rendu("<Convertis>3 kg en cm\n");
    assert!(h.contains("calcul-absent"));
}

#[test]
fn la_droite_graduee_tient_dans_une_bande() {
    let h = rendu(
        "<Représente>graphiquement la droite graduée sur [-3 ; 4], \
         d'intervalle {[-2, 3)} et de points {1}, en rouge\n",
    );
    assert!(h.contains("viewBox=\"0 0 150 20\""));
}

#[test]
fn vecteurs_du_plan_et_de_l_espace() {
    let h = rendu(
        "<Soit>les vecteurs a et b de coordonnées respectives (3 ; -2) et (4 ; 6)\n\n\
         <Calcule>le produit scalaire de a et b\n\n\
         <Calcule>la norme de a\n\n\
         <Calcule>l'angle entre a et b\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("3 \\times 4 + \\left(-2\\right) \\times 6 = 0"));
    assert!(h.contains("= \\sqrt{13}"));
    assert!(h.contains("90^\\circ"));

    let h = rendu(
        "<Soit>les vecteurs r et s de coordonnées respectives (1 ; 2 ; -1) et (2 ; 4 ; -2)\n\n\
         <Étudie>la colinéarité de r et s\n",
    );
    assert!(h.contains("colinéaires"));
    assert!(!h.contains("ne sont pas colinéaires"));
}

#[test]
fn plans_de_l_espace() {
    let h = rendu(
        "<Soit>le plan P d'équation 2x + y - z = 3\n\n\
         <Donne>un vecteur normal de P\n\n\
         <Calcule>la distance du point (1 ; 2 ; 0) au plan P\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("\\vec{n}\\left(2\\ ;\\ 1\\ ;\\ -1\\right)"));
    assert!(h.contains("\\frac{\\sqrt{6}}{6}"));
}

#[test]
fn gram_schmidt_et_projete_orthogonal() {
    let h = rendu(
        "<Soit>les vecteurs e et f de coordonnées respectives (1 ; 1 ; 0) et (1 ; 0 ; 1)\n\n\
         <Orthonormalise>la famille e et f\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("\\vec{\\varepsilon_1}"));
    assert!(h.contains("\\vec{\\varepsilon_2}"));
    assert!(h.contains("\\frac{\\sqrt{2}}{2}"));

    let h = rendu(
        "<Soit>les vecteurs g et h de coordonnées respectives (2 ; 1) et (1 ; 1)\n\n\
         <Calcule>le projeté orthogonal de g sur h\n",
    );
    assert!(h.contains("\\left(\\frac{3}{2}\\ ;\\ \\frac{3}{2}\\right)"));
}

#[test]
fn racines_de_l_unite() {
    let h = rendu("<Calcule>les racines cinquièmes de l'unité\n");
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("z^{5} = 1"));
    assert!(h.contains("e^{\\frac{2 i \\pi}{5}}"));
}

#[test]
fn transformations_du_plan() {
    let points = "<Soit>les points A, B et C de coordonnées respectives \
                  (1 ; 1), (4 ; 1) et (2 ; 3)\n\n";

    let h = rendu(&format!(
        "{}<Construis>l'image du triangle ABC par la symétrie axiale d'axe l'axe des abscisses\n",
        points
    ));
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("A'\\left(1\\ ;\\ -1\\right)"));
    assert!(h.contains("C'\\left(2\\ ;\\ -3\\right)"));
    assert_eq!(h.matches("<svg").count(), 1);

    let h = rendu(&format!(
        "{}<Construis>l'image du triangle ABC par la symétrie centrale de centre O\n",
        points
    ));
    assert!(h.contains("A'\\left(-1\\ ;\\ -1\\right)"));

    let h = rendu(&format!(
        "{}<Soit>le vecteur u de coordonnées (2 ; -1)\n\n\
         <Construis>l'image du triangle ABC par la translation de vecteur u\n",
        points
    ));
    assert!(h.contains("A'\\left(3\\ ;\\ 0\\right)"));

    let h = rendu(&format!(
        "{}<Construis>l'image du triangle ABC par la rotation de centre O et d'angle 90 degrés\n",
        points
    ));
    assert!(h.contains("B'\\left(-1\\ ;\\ 4\\right)"));

    let h = rendu(&format!(
        "{}<Construis>l'image du triangle ABC par l'homothétie de centre O et de rapport 2\n",
        points
    ));
    assert!(h.contains("C'\\left(4\\ ;\\ 6\\right)"));
}

#[test]
fn symetrie_par_rapport_a_une_droite_nommee() {
    let h = rendu(
        "<Soit>les points A, B et C de coordonnées respectives (1 ; 1), (4 ; 1) et (2 ; 3)\n\n\
         <Soit>les points P et Q de coordonnées respectives (0 ; 0) et (1 ; 1)\n\n\
         <Construis>l'image du segment [AB] par la symétrie axiale d'axe (PQ)\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("A'\\left(1\\ ;\\ 1\\right)"));
    assert!(h.contains("B'\\left(1\\ ;\\ 4\\right)"));
}

#[test]
fn geometrie_college_sans_trou() {
    let h = rendu(include_str!("../../exemples/geometrie2.txt"));
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("<svg").count(), 18);
}

#[test]
fn blocs_de_repere_geometriques() {
    let points = "<Soit>les points A(1;1), B(5;3), C(2;4) et D(4 ; -1)\n\n";

    let h = rendu(&format!(
        "{}<Trace>dans un repère où l'abscisse appartient à [-1 ; 6] et l'ordonnée à [-1 ; 5] {{\n\
         \tla droite (AB)\n\tla médiatrice de [AB]\n\tla bissectrice de l'angle BAC\n\
         \tl'angle BAC\n\tle vecteur w (1;1)\n}}\n",
        points
    ));
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("<svg").count(), 1);
    assert_eq!(h.matches("stroke-dasharray=\"2 1.3\"").count(), 0);
    assert_eq!(h.matches("stroke=\"#c00\" stroke-width=\"0.45\"").count(), 2);
    assert!(h.contains("marker-end=\"url(#pointe)\""));

    let h = rendu(&format!(
        "{}<Trace>dans un repère où l'abscisse appartient à [-2 ; 6] et l'ordonnée à [-2 ; 6] {{\n\
         \tla demi-droite [AB)\n\tla droite (AC)\n\tle segment de droite [CD]\n}}\n",
        points
    ));
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("r=\"0.7\"").count(), 4);
}

#[test]
fn figures_a_l_echelle_du_centimetre() {
    let h = rendu("<Trace>le triangle ABC équilatéral, de côté 5 cm, avec les marques\n");
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains(">A</text>") && h.contains(">C</text>"));

    let h = rendu("<Trace>le carré MNOP, de côté 3 cm, avec les marques\n");
    assert!(h.contains(">M</text>") && h.contains(">P</text>"));

    let h = rendu("<Trace>le losange ABEF, de côté 4 cm, d'angle 60, avec les mesures\n");
    assert_eq!(h.matches("4 cm</text>").count(), 4);

    let h = rendu("<Trace>le triangle ABC équilatéral, de côté 5 cm, avec les marques\n");
    assert_eq!(h.matches("5 cm</text>").count(), 3);

    let h = rendu("<Trace>le cercle C, de centre (0;0) et de rayon 2 cm\n");
    assert!(h.contains(">C</text>"));

    let h = rendu("<Trace>le disque D, de centre (0;0) et de rayon 1,5 cm, en bleu ciel\n");
    assert!(h.contains("1,5 cm</text>"));
    assert!(h.contains("<ellipse"));
}

#[test]
fn cercle_trigonometrique_et_solides() {
    let h = rendu("<Trace>le cercle trigonométrique, avec les valeurs\n");
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("r=\"0.8\"").count(), 16);

    let h = rendu("<Trace>le solide cube, d'arête 3 cm\n");
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("<line").count(), 12);

    let h = rendu("<Trace>le solide pyramide, de base 4 cm et de hauteur 3 cm\n");
    assert_eq!(h.matches("<line").count(), 8);
}

#[test]
fn geometrie_lycee_sans_trou() {
    let h = rendu(include_str!("../../exemples/geometrie3.txt"));
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("<svg").count(), 18);
}

#[test]
fn geometrie_superieur_sans_trou() {
    let h = rendu(include_str!("../../exemples/geometrie4.txt"));
    assert!(!h.contains("calcul-absent"));
}

#[test]
fn le_cercle_trigonometrique_porte_les_angles_signes() {
    let h = rendu("<Trace>le cercle trigonométrique, avec les valeurs\n");
    assert!(h.contains(">-\u{3c0}/6</text>"));
    assert!(h.contains(">5\u{3c0}/6</text>"));
    assert!(!h.contains(">7\u{3c0}/6</text>"));
    assert_eq!(h.matches("\u{221a}3/2</text>").count(), 2);
}

#[test]
fn les_declarations_se_groupent_en_une_phrase() {
    // le nom précède la nature, et une seule phrase pose les quatre points
    let h = rendu("<Soit>les points A(1;1), B(5;3), C(2;4) et D(4 ; -1)\n");
    assert_eq!(h.matches("les points de coordonnées").count(), 1);
    assert!(h.contains("Soient \\(A\\), \\(B\\), \\(C\\) et \\(D\\)"), "{}", h);
    assert!(h.contains("et \\(\\left(4\\ ;\\ -1\\right)\\)"), "{}", h);

    let h = rendu("<Soit>le vecteur u de coordonnées (2 ; -1)\n");
    assert!(h.contains("le vecteur de coordonnées"), "{}", h);
}

#[test]
fn le_produit_scalaire_nul_conclut_a_l_orthogonalite() {
    let h = rendu(
        "<Soit>les vecteurs a et b de coordonnées respectives (3 ; -2) et (4 ; 6)\n\n\
         <Calcule>le produit scalaire de a et b\n\n\
         <Calcule>l'angle entre a et b\n",
    );
    assert!(h.contains("sont orthogonaux"));
    assert!(h.contains("\\frac{\\pi}{2}"));
    assert!(h.contains("90^\\circ"));
}

#[test]
fn la_distance_au_plan_substitue_le_point() {
    let h = rendu(
        "<Soit>le plan P d'équation 2x + y - z = 3\n\n\
         <Calcule>la distance du point (1 ; 2 ; 0) au plan P\n",
    );
    assert!(h.contains("2 \\times 1 + 1 \\times 2"));
    assert!(!h.contains("a x_A + b y_A + c z_A + d = 2 x"));
    assert!(h.contains("\\approx 0{,}4082"));
}

#[test]
fn la_notation_geometrique_est_traduite() {
    let h = rendu(
        "$angle(ABC)$ $angle(A)$ $(AB) parallèle (CD)$ $(AB) perpendiculaire (CD)$\n\n\
         $(AB) !parallèle (CD)$ $ABC isométrique A'B'C'$ $ABC semblable A'B'C'$\n\n\
         $distance(A; B)$ $milieu(A; B)$ $vecteur u(3;5)$ $vecteur colonne u(3;5)$\n\n\
         $repère(O; i; j)$ $triangle(ABC)$ $arc(AB)$ $angle droit$ $cercle(O; r)$\n\n\
         $vecteur(u) . vecteur(v)$ $vecteur(u) ^ vecteur(v)$ $vecteur(u)^2$ $colinéaires(u; v)$\n",
    );
    for attendu in [
        "\\widehat{ABC}",
        "\\widehat{A}",
        "\\mathbin{/\\!/}",
        "\\perp",
        "\\nparallel",
        "\\cong",
        "\\sim",
        "d\\left(A\\,,\\,B\\right)",
        "I_{AB}",
        "\\vec{u}\\left(3\\ ;\\ 5\\right)",
        "\\begin{pmatrix}3\\\\5\\end{pmatrix}",
        "\\left(O\\,,\\,\\vec{i}\\,,\\,\\vec{j}\\right)",
        "\\triangle ABC",
        "\\overset{\\frown}{AB}",
        "\\llcorner",
        "\\mathcal{C}\\left(O\\,;\\,r\\right)",
        "\\vec{u} \\cdot \\vec{v}",
        "\\vec{u} \\wedge \\vec{v}",
        "\\vec{u}^2",
    ] {
        assert!(h.contains(attendu), "manque : {}", attendu);
    }
    assert!(!h.contains("angle("));
    assert!(!h.contains("parallèle ("));
}

#[test]
fn le_degre_devient_un_rond() {
    let h = rendu("$angle(ABC) = 90°$\n");
    assert!(h.contains("90^\\circ"));
    assert!(!h.contains('\u{b0}'));
}

#[test]
fn un_cercle_reste_rond_dans_son_repere() {
    let h = rendu(
        "<Trace>dans un repère où l'abscisse appartient à [-4 ; 4] et l'ordonnée à [-4 ; 4] {\n\
         \tle cercle C, de centre (0;0) et de rayon 3\n}\n",
    );
    let rayons: Vec<&str> = h
        .split("<ellipse")
        .skip(1)
        .map(|s| s.split('>').next().unwrap_or(""))
        .collect();
    assert_eq!(rayons.len(), 1);
    let rx = rayons[0].split("rx=\"").nth(1).unwrap().split('"').next().unwrap();
    let ry = rayons[0].split("ry=\"").nth(1).unwrap().split('"').next().unwrap();
    assert_eq!(rx, ry);
    assert!(h.contains("viewBox=\"0 0 150 150\""));
}

#[test]
fn les_unites_graphiques_fixent_le_cadre() {
    let h = rendu(
        "<Soit>une fonction p(x) = x^2\n\n\
         <Représente>graphiquement dans un repère orthogonal (O, i, j) pour x appartient à \
         [-3 ; 3] et y à [-9 ; 9] avec des unités graphiques de 1,5 cm pour l'axe des abscisses \
         et de 0,5 cm pour l'axe des ordonnées {\n\tla courbe de la fonction p\n\
         \tle point S(0 ; 0)\n}\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("viewBox=\"0 0 150 150\""));
}

#[test]
fn les_etiquettes_fuient_le_centre_de_la_figure() {
    let h = rendu(
        "<Soit>les points A, B et C de coordonnées respectives (1 ; 1), (4 ; 1) et (2 ; 3)\n\n\
         <Construis>l'image du triangle ABC par la translation de vecteur u\n",
    );
    let h = h + &rendu("<Trace>le carré MNOP, de côté 3 cm, avec les marques\n");
    assert!(h.contains("text-anchor=\"end\""));
    assert!(h.contains("text-anchor=\"start\""));
}

#[test]
fn l_axe_des_ordonnees_descend_jusqu_au_bas_du_repere() {
    let h = rendu(
        "<Soit>une fonction p(x) = x^2\n\n\
         <Représente>graphiquement dans un repère orthogonal (O, i, j) pour x appartient à \
         [-3 ; 3] et y à [-9 ; 9] avec des unités graphiques de 1,5 cm pour l'axe des abscisses \
         et de 0,5 cm pour l'axe des ordonnées {\n\tla courbe de la fonction p\n}\n",
    );
    let hauteur: f64 = h
        .split("viewBox=\"0 0 150 ")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let vertical = h
        .split("class=\"axe\"")
        .nth(1)
        .unwrap();
    let bas: f64 = vertical
        .rsplit("y1=\"")
        .next()
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert!((bas - (hauteur - 4.0)).abs() < 0.01, "axe tronqué à {}", bas);
}

#[test]
fn les_cotes_suivent_la_direction_du_cote() {
    let h = rendu("<Trace>le triangle ABC équilatéral, de côté 5 cm, avec les marques\n");
    assert_eq!(h.matches("cm</text>").count(), 3);
    assert_eq!(h.matches("transform=\"rotate(").count(), 3);
    assert!(h.contains("rotate(0.00"));
    assert!(h.contains("rotate(60.00"));
    assert!(h.contains("rotate(-60.00"));

    let h = rendu("<Trace>le carré MNOP, de côté 3 cm, avec les marques\n");
    assert_eq!(h.matches("rotate(-90.00").count(), 2);
    assert_eq!(h.matches("rotate(0.00").count(), 2);
}

#[test]
fn le_cercle_trigonometrique_porte_ses_traits_de_rappel() {
    let h = rendu("<Trace>le cercle trigonométrique, avec les valeurs\n");
    assert_eq!(h.matches("stroke=\"#777\" stroke-width=\"0.45\"").count(), 6);
    assert_eq!(h.matches("stroke-dasharray=\"2 1.3\"").count(), 6);
    assert_eq!(h.matches("1/2</text>").count(), 2);
    assert_eq!(h.matches("\u{221a}2/2</text>").count(), 2);
    assert_eq!(h.matches("\u{221a}3/2</text>").count(), 2);
}

#[test]
fn les_resultantes_portent_leur_expression() {
    let h = rendu(
        "<Trace>dans un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5] {\n\
         \tle vecteur u (3;1)\n\tle vecteur v (1;3)\n\
         \tle vecteur u depuis la pointe de v\n\tle vecteur v depuis la pointe de u\n\
         \tle vecteur u + v\n\tle vecteur u - v depuis la pointe de v\n}\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains(">u + v</text>"));
    assert!(h.contains(">u \u{2212} v</text>"));
    assert!(!h.contains(">w</text>"));
    assert!(!h.contains(">d</text>"));
    assert_eq!(h.matches("class=\"nom\" fill=").count(), 6);
    assert_eq!(h.matches("transform=\"rotate(").count(), 6);
}
