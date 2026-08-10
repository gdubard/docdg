use docdg_transpiler::Engine;

fn rendu(src: &str) -> String {
    Engine::new().render(src, true).html
}

#[test]
fn moyennes_simple_et_ponderee() {
    let h = rendu(
        "<Calcule>la moyenne de la série 12 ; 15 ; 9 ; 14\n\n\
         <Calcule>la moyenne de la série de valeurs 8 ; 12 ; 15 et d'effectifs 2 ; 5 ; 3\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("\\dfrac{12 + 15 + 9 + 14}{4} = \\dfrac{50}{4} = 12{,}5"));
    assert!(h.contains("\\dfrac{8 \\times 2 + 12 \\times 5 + 15 \\times 3}{10}"));
    assert!(h.contains("\\dfrac{121}{10} = 12{,}1"));
}

#[test]
fn la_mediane_depend_de_la_parite() {
    let h = rendu("<Calcule>la médiane de la série 9 ; 15 ; 12 ; 14\n");
    assert!(h.contains("9 ; 12 ; 14 ; 15 (4 valeurs)"));
    assert!(h.contains("L'effectif est pair"));
    assert!(h.contains("\\dfrac{12 + 14}{2} = 13"));

    let h = rendu("<Calcule>la médiane de la série 7 ; 3 ; 9 ; 5 ; 11\n");
    assert!(h.contains("3 ; 5 ; 7 ; 9 ; 11 (5 valeurs)"));
    assert!(h.contains("L'effectif est impair"));
    assert!(h.contains("m = 7"));
}

#[test]
fn etendue_et_quartiles() {
    let h = rendu("<Calcule>l'étendue de la série 12 ; 15 ; 9 ; 14\n");
    assert!(h.contains("e = 15 - 9 = 6"));

    let h = rendu("<Calcule>les quartiles de la série 2 ; 4 ; 5 ; 7 ; 8 ; 10 ; 12 ; 15\n");
    assert!(h.contains("Q_1 = 4"));
    assert!(h.contains("Q_3 = 10"));
}

#[test]
fn proportionnalite_dans_les_trois_sens() {
    let h = rendu("<Calcule>la quatrième proportionnelle de 3, 5 et 12\n");
    assert!(h.contains("x = \\dfrac{5 \\times 12}{3} = 20"));

    let h = rendu("<Vérifie>si le tableau est de proportionnalité {\n\t2 ; 3 ; 5\n\t6 ; 9 ; 15\n}\n");
    assert!(h.contains("Tous les rapports sont égaux"));
    assert!(h.contains("de coefficient \\(3\\)"));

    let h = rendu("<Vérifie>si le tableau est de proportionnalité {\n\t2 ; 3 ; 5\n\t6 ; 9 ; 14\n}\n");
    assert!(h.contains("\\dfrac{14}{5} = 2{,}8"));
    assert!(h.contains("ce n'est pas un tableau de proportionnalité"));

    let h = rendu("<Complète>le tableau de proportionnalité {\n\t2 ; 3 ; ?\n\t6 ; ? ; 15\n}\n");
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("font-weight=\"bold\"").count(), 2);
    assert!(h.contains("font-weight=\"bold\">9</text>"));
    assert!(h.contains("font-weight=\"bold\">5</text>"));
}

#[test]
fn pourcentages_et_taux() {
    let h = rendu(
        "<Calcule>30 % de 250\n\n<Applique>une augmentation de 5 % à 240\n\n\
         <Applique>une diminution de 12 % à 60\n\n<Calcule>le taux d'évolution de 250 à 280\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("\\dfrac{30}{100} \\times 250 = 75"));
    assert!(h.contains("1 + \\dfrac{5}{100} = 1{,}05"));
    assert!(h.contains("240 \\times 1{,}05 = 252"));
    assert!(h.contains("1 - \\dfrac{12}{100} = 0{,}88"));
    assert!(h.contains("60 \\times 0{,}88 = 52{,}8"));
    assert!(h.contains("une augmentation de \\(12\\,\\%\\)"));
}

#[test]
fn echelles_vitesses_distances_et_durees() {
    let h = rendu("<Calcule>l'échelle d'un plan où 2 cm représentent 50 m\n");
    assert!(h.contains("\\dfrac{1}{2500}"));

    let h = rendu("<Calcule>la vitesse moyenne pour 150 km en 2 h 30 min\n");
    assert!(h.contains("2\\text{ h }30\\text{ min} = 2{,}5\\text{ h}"));
    assert!(h.contains("= 60\\) km/h"));

    let h = rendu("<Calcule>la distance parcourue à 80 km/h pendant 1 h 45 min\n");
    assert!(h.contains("80 \\times 1{,}75 = 140"));

    let h = rendu("<Calcule>la durée du trajet de 210 km à 60 km/h\n");
    assert!(h.contains("\\dfrac{210}{60} = 3{,}5\\text{ h} = 3\\text{ h }30\\text{ min}"));
}

#[test]
fn le_college_ne_laisse_aucun_bloc_absent() {
    let h = rendu(include_str!("statistiques-probabilites2.txt"));
    assert!(!h.contains("calcul-absent"));
}

#[test]
fn les_six_diagrammes_sortent_en_svg() {
    let h = rendu(
        "soit sondage = {\n\tJeudi: 8\n\tLundi: 5\n\tMardi: 3\n}\n\n\
         <Représente>graphiquement une statistique en barres avec les données {1: 4 | 2: 7 | 3: 2 | 4: 5}\n\n\
         <Représente>graphiquement une statistique en camembert avec les données {Marche: 5 | Bus: 3 | Vélo: 2}\n\n\
         <Représente>graphiquement une statistique en barres avec les données sondage et en disposition horizontale\n\n\
         <Représente>graphiquement une statistique en histogramme avec les bornes {0 ; 5 ; 10 ; 20} et les effectifs {3 ; 7 ; 2}\n\n\
         <Représente>graphiquement une statistique en boîte à moustaches avec les données {2 ; 3 ; 5 ; 7 ; 8 ; 11 ; 13}\n\n\
         <Représente>graphiquement une statistique en nuage avec ajustement et les données {(1;2) (2;2,8) (3;4,1) (4;4,9)}\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("<svg").count(), 6);
    assert!(h.contains("Marche (50 %)"));
    assert!(h.contains("Bus (30 %)"));
    assert!(h.contains(">Jeudi</text>"));
    assert!(h.contains(">Me</text>"));
    assert!(h.contains(">Q3</text>"));
    assert_eq!(h.matches("<circle").count(), 4);
}

#[test]
fn les_lois_du_lycee_et_du_superieur() {
    let h = rendu(
        "<Calcule la probabilité que X <= 1,96 pour la loi normale(0 ; 1)>\n\n\
         <Calcule la probabilité que X >= 3 pour la loi binomiale(10 ; 0,5)>\n\n\
         <Calcule>la probabilité que X = 2 pour la loi poisson(1,5)\n\n\
         <Détermine>le quantile d'ordre 0,95 de la loi normale(0 ; 1)\n\n\
         <Calcule>l'espérance de la loi binomiale(20 ; 0,05)\n\n\
         <Calcule>l'écart type de la loi poisson(4)\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("P(X \\leqslant 1{,}96) \\approx 0{,}975"));
    assert!(h.contains("P(X \\geqslant 3) \\approx 0{,}945"));
    assert!(h.contains("P(X = 2) \\approx 0{,}251"));
    assert!(h.contains("x_{0{,}95} \\approx 1{,}645"));
    assert!(h.contains("E(X) \\approx 1\\)"));
    assert!(h.contains("\\sigma(X) \\approx 2\\)"));

    let h = rendu(
        "<Calcule la probabilité que X <= 0,5 pour la loi uniforme(0 ; 2)>\n\n\
         <Calcule la probabilité que X >= 1 pour la loi exponentielle(2)>\n\n\
         <Calcule la probabilité que X <= 2 pour la loi student(10)>\n\n\
         <Calcule la probabilité que X >= 3,84 pour la loi khi-deux(1)>\n\n\
         <Détermine>le quantile d'ordre 0,975 de la loi student(9)\n\n\
         <Détermine>le quantile d'ordre 0,975 de la loi student(100)\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("\\approx 0{,}25\\)"));
    assert!(h.contains("\\approx 0{,}135\\)"));
    assert!(h.contains("\\approx 0{,}963\\)"));
    assert!(h.contains("\\approx 0{,}05\\)"));
    assert!(h.contains("x_{0{,}975} \\approx 2{,}262"));
    assert!(h.contains("x_{0{,}975} \\approx 1{,}984"));
}

#[test]
fn dispersion_des_series_fluctuation_et_tchebychev() {
    let h = rendu(
        "<Calcule>la variance de la série 12 ; 15 ; 9 ; 14\n\n\
         <Calcule>l'écart type de la série 12 ; 15 ; 9 ; 14\n\n\
         <Calcule>la variance de la série de valeurs 8 ; 12 ; 15 et d'effectifs 2 ; 5 ; 3\n\n\
         <Calcule>la covariance des séries 1 ; 2 ; 3 et 2 ; 4 ; 7\n\n\
         <Calcule>l'intervalle de fluctuation pour n = 100 et p = 0,3\n\n\
         <Applique>l'inégalité de Bienaymé-Tchebychev pour une espérance de 5, une variance de 2 \
         et un écart de 3\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("V = 5{,}25"));
    assert!(h.contains("\\sigma = \\sqrt{5{,}25} \\approx 2{,}291"));
    assert!(h.contains("V = 5{,}89"));
    assert!(h.contains("\\dfrac{1}{N} \\sum n_i"));
    assert!(h.contains("\\approx 1{,}667"));
    assert!(h.contains("0{,}2 \\,;\\, 0{,}4"));
    assert!(h.contains("\\dfrac{V}{a^2} = 0{,}222"));
}

#[test]
fn le_tableau_de_proportionnalite_montre_ses_facteurs() {
    let h = rendu("<Complète>le tableau de proportionnalité {\n\t2 ; 3 ; ?\n\t6 ; ? ; 15\n}\n");
    assert_eq!(h.matches("<svg").count(), 1);
    assert!(!h.contains("coefficient de proportionnalité vaut"));
    assert!(h.contains("\u{00d7}3</text>"));
    assert!(h.contains("\u{00f7}3</text>"));
    assert_eq!(h.matches("font-weight=\"bold\"").count(), 2);
    assert_eq!(h.matches("marker-end=\"url(#pointe)\"").count(), 2);
}

#[test]
fn la_boucle_sur_une_liste_se_deroule() {
    let h = rendu(
        "pour n dans [100, 400] {\n\tPour #n lancers, l'écart type vaut #{defaut(1/(2*racine(n)))}.\n}\n",
    );
    assert!(h.contains("Pour 100 lancers"));
    assert!(h.contains("Pour 400 lancers"));
}

#[test]
fn l_arbre_complete_les_branches_et_imprime_les_produits() {
    let h = rendu(
        "<Construis>un arbre avec les produits {\n\tA 0,3 {\n\t\tB 0,6\n\t}\n\
         \t!A 0,7 {\n\t\tB 0,1\n\t}\n}\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert_eq!(h.matches("<svg").count(), 1);
    assert!(h.contains(">0,4</text>"));
    assert!(h.contains(">0,9</text>"));
    assert!(h.contains("= 0,18</text>"));
    assert!(h.contains("= 0,12</text>"));
    assert!(h.contains("= 0,07</text>"));
    assert!(h.contains("= 0,63</text>"));
    assert_eq!(h.matches("text-decoration:overline").count(), 7);
}

#[test]
fn la_loi_de_probabilite_se_dresse_et_se_calcule() {
    let h = rendu(
        "<Dresse>la loi de probabilité de X {\n\tvaleurs : [1 ; 2 ; 3]\n\
         \tprobabilités : [1/6 ; 1/3 ; 1/2]\n}\n\n\
         <Calcule>l'espérance de X\n\n<Calcule>la variance de X\n\n<Calcule>l'écart type de X\n",
    );
    assert!(!h.contains("calcul-absent"));
    assert!(h.contains("P(X = x_i)"));
    assert!(h.contains("\\dfrac{1}{6}"));
    assert!(h.contains("E(X) = \\dfrac{7}{3} \\approx 2{,}3333"));
    assert!(h.contains("V(X) = \\dfrac{5}{9} \\approx 0{,}5556"));
    assert!(h.contains("\\sigma(X) = \\dfrac{\\sqrt{5}}{3} \\approx 0{,}7454"));
}

#[test]
fn une_loi_qui_ne_totalise_pas_un_est_refusee() {
    let h = rendu(
        "<Dresse>la loi de probabilité de Y {\n\tvaleurs : [1 ; 2]\n\
         \tprobabilités : [1/3 ; 1/3]\n}\n",
    );
    assert!(h.contains("au lieu de \\(1\\)"));
    assert!(h.contains("\\dfrac{2}{3}"));
}

#[test]
fn les_trois_fichiers_de_statistiques_sont_complets() {
    for source in [
        include_str!("statistiques-probabilites2.txt"),
        include_str!("statistiques-probabilites3.txt"),
        include_str!("statistiques-probabilites4.txt"),
    ] {
        assert!(!rendu(source).contains("calcul-absent"));
    }
}

#[test]
fn ajuste_un_modele_exponentiel_aux_donnees() {
    let mut e = Engine::new();
    let html = e
        .render(
            "<Ajuste N(t) = a*exp(b*t) aux données {(0;100) (1;61) (2;37) (3;22) (4;14)}>",
            false,
        )
        .html;
    assert!(!html.contains("Commande non prise en charge"), "{}", html);
    assert!(html.contains("100{,}1"), "paramètre a incorrect : {}", html);
    assert!(html.contains("-0{,}4978"), "paramètre b incorrect : {}", html);
    assert!(html.contains("R^2"), "coefficient de détermination absent");
}

#[test]
fn ajuste_accepte_un_modele_affine_et_la_variable_par_defaut() {
    let mut e = Engine::new();
    let html = e
        .render(
            "<Ajuste f(x) = a*x + b aux données {(0;1) (1;3,1) (2;4,9) (3;7,2)}>",
            false,
        )
        .html;
    assert!(!html.contains("Commande non prise en charge"), "{}", html);
    assert!(html.contains("2{,}04"), "pente incorrecte : {}", html);
}

#[test]
fn ajuste_refuse_un_modele_sans_assez_de_mesures() {
    let mut e = Engine::new();
    let html = e
        .render("<Ajuste f(x) = a*x + b aux données {(0;1) (1;3)}>", false)
        .html;
    assert!(html.contains("calcul-absent"), "{}", html);
}
