/// La réglure ne demande rien : le carreau et l'interligne valent huit
/// millimètres par défaut, et la Marelle embarquée écrit sans qu'on la nomme.
/// Ici une cursive est nommée — la Schola, non embarquée, donc cherchée sur
/// le système avec la hampe supposée de 0,903 : le cas général d'un auteur
/// qui désigne la sienne.
const PAGE: &str = "page {\n\tseyès: Schola;\n}\n\n";

fn rend(src: &str) -> String {
    let mut e = docdg_transpiler::Engine::new();
    e.render(&format!("{}{}", PAGE, src), false).html
}

fn rend_parallele(src: &str) -> String {
    let mut e = docdg_transpiler::Engine::new();
    e.render(&format!("{}{}", PAGE, src), true).html
}

#[test]
fn le_bloc_pose_une_reglure_et_ecrit_en_cursive() {
    let h = rend("<Écris>sur des lignes{\nLéa mange une pomme.\n}");
    assert!(h.contains("docdg-lignes"), "{}", h);
    assert!(h.contains("font-family:'Schola'"), "{}", h);
    assert!(h.contains("Léa mange une pomme."), "{}", h);
}

#[test]
fn sans_cursive_declaree_docdg_emploie_la_sienne() {
    // docdg porte ses cursives : il n'a plus à en exiger une. Tant qu'il
    // dépendait de ce qui était installé sur la machine, il fallait que
    // l'auteur la nomme ; embarquée, elle est toujours là.
    let mut e = docdg_transpiler::Engine::new();
    let h = e.render("<Écris>sur des lignes{\nBonjour.\n}", false).html;
    assert!(!h.contains("aucune police manuscrite"), "{}", h);
    assert!(h.contains("font-family:'Marelle'"), "{}", h);
    assert!(h.contains("@font-face"), "la cursive par défaut doit voyager : {}", h);
    // et ses proportions à elle, non celles de la Schola
    assert!(h.contains("ascent-override:144.0%"), "{}", h);
    assert!(h.contains("descent-override:96.0%"), "{}", h);
}

#[test]
fn le_rendu_parallele_voit_la_cursive_comme_le_rendu_sequentiel() {
    // Les réglages de page vivent dans un `thread_local` : les fils de rayon
    // ne voient pas ceux du fil appelant. Sans les leur reposer, le même
    // document rendait une réglure en séquentiel et une erreur en parallèle.
    let corps = "<Écris>sur des lignes{\nUn.\n}\n\n<Écris>sur des lignes{\nDeux.\n}";
    assert!(!rend_parallele(corps).contains("aucune police manuscrite"));
    assert_eq!(
        rend(corps).matches("<docdg-lignes").count(),
        rend_parallele(corps).matches("<docdg-lignes").count()
    );
}

#[test]
fn la_taille_se_deduit_de_la_reglure() {
    // La hampe vaut trois interlignes, le jambage deux : la Schola les pose
    // à 0,903 et 0,602 em, ce qui fixe le corps à 6,645 mm pour un pas de
    // 8 mm — l'auteur n'a aucun calcul à faire.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    assert!(h.contains("font-size:6.645mm"), "{}", h);
    assert!(h.contains("line-height:8.000mm"), "{}", h);
}

#[test]
fn la_ligne_de_base_tombe_sur_le_trait_fort() {
    // La première ligne de base tombe à 6,090 mm du haut du bloc : la coiffe
    // (un huitième de pas, plus la demi-épaisseur de l'interligne qui
    // l'affleure) et les cinq huitièmes du pas qui placent la base dans sa
    // ligne. Le trait la chevauche, d'où son bord haut une demi-épaisseur
    // plus haut.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    assert!(h.contains("M0.000 5.965h300.000v0.250h-300.000z"), "{}", h);
}

#[test]
fn chaque_trait_chevauche_sa_position() {
    // Le moteur cale chaque ligne écrite sur le pixel entier ; un pas de
    // 8 mm en vaut 30,236, si bien que l'écriture décrit une dent de scie
    // d'un pixel autour de sa ligne. Un trait posé sous la ligne de base
    // laissait paraître le jour ; centré sur elle, il l'absorbe.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    // ligne de base à 6,090 : le trait de 0,250 va de 5,965 à 6,215
    assert!(h.contains("M0.000 5.965h300.000v0.250h"), "{}", h);
    // interligne à 8,090 : le trait de 0,180 va de 8,000 à 8,180
    assert!(h.contains("M0.000 8.000h300.000v0.180h"), "{}", h);
}

#[test]
fn aucun_interligne_ne_tombe_sur_une_ligne_forte() {
    // Sur la feuille, il y a trois interlignes entre deux lignes fortes, pas
    // quatre. Tant que le trait fort pendait sous sa ligne il couvrait
    // l'intrus ; maintenant qu'il la chevauche, l'intrus dépasserait en
    // cyan sous le bleu. Il n'est plus tracé du tout.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    let fins = h.split("fill=\"deepskyblue\"").next().unwrap();
    assert!(fins.contains("M0.000 4.000h300.000v0.180h"), "{}", fins);
    assert!(fins.contains("M0.000 8.000h300.000v0.180h"), "{}", fins);
    // 6,000 serait l'interligne qui tombe sur la première ligne forte
    assert!(!fins.contains("M0.000 6.000h300.000v0.180h"), "interligne sous la ligne forte");
}

#[test]
fn la_reglure_se_repete_au_pas_et_au_quart_du_pas() {
    // Le trait fort tous les 8 mm, les interlignes fins tous les 2 mm, les
    // carreaux verticaux tous les 8 mm.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    assert!(h.contains("M0.000 0.000h300.000v0.180h-300.000zM0.000 2.000h300.000v0.180h-300.000z"), "{}", h);
    assert!(h.contains("M0.000 5.965h300.000v0.250h-300.000zM0.000 13.965h300.000v0.250h-300.000z"), "{}", h);
    assert!(h.contains("M0.000 0.000h0.250v312.000h-0.250zM8.000 0.000h0.250v312.000h-0.250z"), "{}", h);
}

#[test]
fn la_reglure_est_dessinee_et_non_peinte() {
    // Une image de fond est rastérisée par Chromium à l'export : le PDF la
    // porte en pixels rééchantillonnés et compressés avec perte — traits
    // magenta, épaisseurs doublées. Un tracé dans le document en sort en
    // vectoriel. Aucun fond, donc, nulle part dans le bloc.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    let bloc = h.split("<docdg-lignes").nth(1).unwrap();
    assert!(!bloc.contains("background"), "{}", bloc);
    assert!(!bloc.contains("url(data:image"), "{}", bloc);
    assert!(!h.contains("linear-gradient"), "{}", h);
    assert!(bloc.contains("<svg"), "{}", bloc);
}

#[test]
fn la_feuille_est_vraie() {
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    // Les trois couches dans l'ordre de la feuille : les interlignes
    // dessous, les lignes fortes et les carreaux au-dessus, la marge par
    // -dessus tout.
    let fine = h.find("deepskyblue").unwrap();
    let forte = h.find("\"royalblue\"").unwrap();
    let marge = h.find("\"red\"").unwrap();
    assert!(fine < forte && forte < marge, "{} {} {}", fine, forte, marge);
    // La feuille commence au trait rouge : rien n'est tracé à sa gauche, et
    // le bloc ne déborde plus dans la marge de la page.
    assert!(h.contains("M0.000 0.000h0.500v312.000h-0.500z"), "{}", h);
    assert!(!h.contains("margin-left:-"), "{}", h);
    assert!(!h.contains("border-left"), "{}", h);
    // Une seule réglure pour tout le bloc : elle le couvre entièrement,
    // quel que soit le nombre de lignes que le texte occupe une fois coupé.
    assert_eq!(h.matches("<docdg-reglure>").count(), 1, "{}", h);
    assert!(h.contains("position:absolute;top:0;right:0;bottom:0;left:0"), "{}", h);
    assert!(h.contains("overflow:hidden"), "{}", h);
    // Trente-neuf lignes de 8 mm couvrent la plus grande page ; le bloc
    // coupe le reste.
    assert!(h.contains("height:312.000mm"), "{}", h);
    assert!(h.contains("viewBox=\"0 0 300.000 312.000\" preserveAspectRatio=\"none\""), "{}", h);
    // Trois interlignes au-dessus de la première ligne forte, deux
    // au-dessous de la dernière : les coiffes sont les garnitures du bloc.
    assert!(h.contains("padding:1.090mm 0 1.090mm 1.000mm"), "{}", h);
}

#[test]
fn chaque_moteur_recoit_le_pas_qu_il_sait_tenir() {
    // L'aperçu et le PDF ne sont pas composés par le même moteur : l'aperçu
    // vit dans la vue web du système — WebKit sous Linux —, le PDF sort de
    // Chromium. Or 8 mm valent 30,236 pixels, et WebKit arrondit la hauteur
    // de ligne au pixel entier quand Chromium la garde entière : la réglure
    // prenait un quinzième de millimètre d'avance à chaque ligne dans
    // l'aperçu, et rien à l'impression. À l'écran, le pas est donc dit en
    // pixels entiers — il n'y a plus rien à arrondir — et la réglure est
    // étirée d'autant.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    assert!(h.contains("line-height:8.000mm"), "{}", h);
    assert!(h.contains("height:312.000mm"), "{}", h);
    assert!(
        h.contains("@media screen{docdg-lignes,docdg-lignes span[style*=\"display:block\"]\
{line-height:30px}docdg-reglure svg{height:1170px}}"),
        "{}",
        h
    );
    // 39 lignes de 30 px : la réglure et le texte avancent du même pas.
    assert_eq!(39 * 30, 1170);
}

#[test]
fn la_reglure_couvre_les_lignes_repliees() {
    // Une ligne de source qui se replie occupe plusieurs lignes écrites. La
    // réglure ne peut donc pas être portée par la ligne de source : elle
    // couvre le bloc, et le bloc la coupe à sa hauteur réelle. Le motif est
    // déclaré une fois en tête ; chaque bloc n'émet qu'un renvoi.
    let h = rend("<Écris>sur des lignes{\nUn texte assez long pour se replier.\n}");
    assert_eq!(h.matches("<symbol id=\"docdg-reglure-motif\"").count(), 1, "{}", h);
    assert_eq!(h.matches("<use href=\"#docdg-reglure-motif\"").count(), 1, "{}", h);
    let bloc = h.split("<docdg-lignes").nth(1).unwrap();
    assert!(bloc.find("<docdg-reglure>").unwrap() < bloc.find("<div").unwrap(), "{}", bloc);
}

#[test]
fn le_motif_ne_voyage_qu_une_fois() {
    // Trois blocs, un seul motif : la géométrie de la réglure ne dépend que
    // du pas et du carreau, constants pour tout le document — la répéter par
    // bloc, c'était la transporter par élève dans un jeu de modèles
    // nominatifs pour toute une classe.
    let corps = "<Écris>sur des lignes{\nUn.\n}\n\n<Écris>sur des lignes{\nDeux.\n}\n\n<Écris>sur des lignes{\nTrois.\n}";
    let h = rend(corps);
    assert_eq!(h.matches("<symbol id=\"docdg-reglure-motif\"").count(), 1, "{}", h);
    assert_eq!(h.matches("<use href=\"#docdg-reglure-motif\"").count(), 3, "{}", h);
    assert_eq!(h.matches("fill=\"red\"").count(), 1, "le trait rouge est retracé par bloc");
}

#[test]
fn le_trait_rouge_n_est_pas_saucissonne() {
    // Un seul rectangle, de la tête au pied du motif, tracé en dernier : le
    // trait court d'un bout à l'autre sans être coupé par ce qu'il croise.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    assert_eq!(h.matches("h0.500v312.000h-0.500z").count(), 1, "{}", h);
    assert!(h.contains("h0.500v312.000h-0.500z\" fill=\"red\"/></symbol></svg>"), "{}", h);
}

#[test]
fn l_ecriture_ne_touche_pas_le_trait_rouge() {
    // La main ne pose pas sa première lettre sur la marge : le bloc écarte
    // son texte d'un millimètre, et la réglure — posée sur la boîte de
    // remplissage — ne bouge pas pour autant.
    let h = rend("<Écris>sur des lignes{\nBonjour.\n}");
    assert!(h.contains("padding:1.090mm 0 1.090mm 1.000mm"), "{}", h);
}

#[test]
fn un_exposant_ne_decroche_pas_la_reglure() {
    // Un exposant est un fragment de ligne comme un autre : il n'a pas voix
    // au chapitre sur la hauteur de sa ligne. L'application le neutralisait
    // de son côté ; le cahier n'a plus à en dépendre.
    let h = rend("<Écris>sur des lignes{\nLe 21<exposant>{è} siècle et H<indice>{2}O.\n}");
    assert!(h.contains("<sup>"), "{}", h);
    assert!(h.contains("<sub>"), "{}", h);
    assert!(
        h.contains("docdg-lignes span,docdg-lignes sup,docdg-lignes sub{line-height:0}"),
        "{}",
        h
    );
}

#[test]
fn un_style_en_ligne_ne_decroche_pas_la_reglure() {
    let h = rend("<Écris>sur des lignes{\nÀ <ARIAL gras 14pt>{Paris}, il pleut.\n}");
    assert!(h.contains("font-size:14pt"), "{}", h);
    // La hauteur de réglure est portée par le bloc et par lui seul : aucun
    // élément du corps ne la reprend, si bien que le mot écrit plus gros
    // déborde sans faire enfler sa ligne.
    let corps = h.split("</style>").nth(1).unwrap();
    assert_eq!(corps.matches("line-height").count(), 0, "{}", corps);
}

#[test]
fn une_tabulation_vaut_un_carreau() {
    let h = rend("<Écris>sur des lignes{\n\tUn carreau.\n\t\t\tTrois carreaux.\n}");
    assert!(h.contains("text-indent:8.000mm"), "{}", h);
    assert!(h.contains("text-indent:24.000mm"), "{}", h);
}

#[test]
fn le_retrait_ne_porte_que_sur_la_premiere_ligne() {
    // `text-indent` et non une marge : le retour automatique revient à la
    // marge, comme sur un vrai cahier. (Le seul `margin-left` admis est le
    // débord négatif de la bande de marge, dans la feuille de style.)
    let h = rend("<Écris>sur des lignes{\n\tUn texte long.\n}");
    assert!(h.contains("text-indent"), "{}", h);
    let bloc = h.split("<docdg-lignes").nth(1).unwrap();
    assert!(!bloc.contains("margin-left"), "{}", bloc);
}

#[test]
fn une_ligne_vide_est_une_ligne_a_remplir() {
    let h = rend("<Écris>sur des lignes{\nLe modèle.\n\n\n}");
    let bloc = h.split("<docdg-lignes").nth(1).unwrap();
    assert_eq!(bloc.matches("<div").count(), 3, "{}", bloc);
    assert_eq!(bloc.matches('\u{200B}').count(), 2, "{}", bloc);
}

#[test]
fn seule_la_marelle_voyage_avec_le_document() {
    // La Marelle est la seule cursive embarquée — sa licence, la SIL OFL,
    // permet expressément l'incorporation et la redistribution ; celles des
    // autres cursives, Schola comprise, ne le permettent pas. Nommée, une
    // autre cursive est cherchée sur le système ou jointe par son fichier.
    let mut e = docdg_transpiler::Engine::new();
    let h = e.render("<Écris>sur des lignes{\nBonjour.\n}", false).html;
    assert_eq!(h.matches("@font-face").count(), 1, "fonte absente ou en double");
    assert!(h.contains("base64"), "la Marelle doit voyager : {}", &h[..h.len().min(300)]);
    let nommee = rend("<Écris>sur des lignes{\nBonjour.\n}");
    assert!(!nommee.contains("base64"), "une cursive non libre a été embarquée");
    let sans = rend("Une phrase ordinaire.");
    assert!(!sans.contains("@font-face"), "fonte jointe pour rien");
}

#[test]
fn une_cursive_etrangere_est_cherchee_sur_le_systeme() {
    let mut e = docdg_transpiler::Engine::new();
    let h = e
        .render("page {\n\tseyès: Belle Allure;\n}\n\n<Écris>sur des lignes{\nBonjour.\n}", false)
        .html;
    assert!(h.contains("font-family:'Belle Allure'"), "{}", h);
    // Cherchée sur le système, non embarquée : seule la Marelle voyage.
    assert!(h.contains("src:local('Belle Allure')"), "{}", h);
    assert!(!h.contains("base64"), "fonte embarquée pour une autre cursive");
}

#[test]
fn une_cursive_se_nomme_par_son_fichier_et_livre_ses_proportions() {
    // Une cursive installée sur le système, docdg la nomme sans pouvoir
    // l'ouvrir : il suppose alors les proportions de la Schola. Nommée par
    // son fichier, elle est jointe au document **et mesurée** — c'est la
    // seule façon de connaître sa hampe, et elle varie beaucoup d'une
    // cursive à l'autre.
    let mut e = docdg_transpiler::Engine::new();
    let h = e
        .render("page {\n\tseyès: Introuvable.ttf;\n}\n\n<Écris>sur des lignes{\nBonjour.\n}", false)
        .html;
    // Fichier absent : la cursive est cherchée sur le système sous le nom du
    // fichier, sans son extension, et sous ses trois formes usuelles.
    assert!(h.contains("font-family:'Introuvable'"), "{}", h);
    assert!(
        h.contains("src:local('Introuvable'),local('Introuvable Regular'),local('Introuvable-Regular')"),
        "{}",
        h
    );
    assert!(!h.contains("Introuvable.ttf'"), "l'extension a fui dans le nom : {}", h);
}

#[test]
fn les_metriques_de_la_cursive_sont_imposees() {
    // La place de la ligne de base dans sa ligne dépend de ce que le moteur
    // lit dans la fonte, et les moteurs n'en font pas tous le même usage.
    // Les surcharges leur retirent ce qui restait à leur discrétion — et
    // elles valent pour toute cursive, installée ou embarquée.
    for police in ["Schola", "Belle Allure"] {
        let mut e = docdg_transpiler::Engine::new();
        let h = e
            .render(&format!("page {{\n\tseyès: {};\n}}\n\n<Écris>sur des lignes{{\nBonjour.\n}}", police), false)
            .html;
        assert!(h.contains("ascent-override:90.3%"), "{} : {}", police, h);
        // Le jambage se déduit de la hampe : deux interlignes contre trois.
        assert!(h.contains("descent-override:60.2%"), "{} : {}", police, h);
        assert!(h.contains("line-gap-override:0%"), "{} : {}", police, h);
    }
}

#[test]
fn un_mot_ne_se_coupe_jamais_sur_des_lignes() {
    let h = rend("<Écris>sur des lignes{\nanticonstitutionnellement\n}");
    let coupe = docdg_transpiler::cesure_html(&h);
    let bloc = coupe.split("<docdg-lignes").nth(1).unwrap();
    assert!(!bloc.contains('\u{00AD}'), "césure dans un cahier : {}", bloc);
}

#[test]
fn le_verbe_ecris_garde_son_emploi_mathematique() {
    let h = rend("<Écris>le nombre 45 600 en notation scientifique");
    assert!(!h.contains("docdg-lignes"), "{}", h);
    assert!(!h.contains("non prise en charge"), "{}", h);
}

#[test]
fn un_alignement_ne_fait_pas_flotter_sa_ligne() {
    // Les alignements — `<centre>`, `<gauche>`, `<droite>` — produisent un
    // span qui passe en bloc. La règle anti-gonflement, faite pour les
    // fragments de ligne, lui retirait toute hauteur : son texte flottait un
    // demi-pas au-dessus de la réglure, et tout ce qui suivait glissait d'un
    // pas entier. Un span en bloc *est* une ligne : il en reprend la hauteur.
    let h = rend("<Écris>sur des lignes{\n<centre>Lorem Ipsum\n}");
    assert!(h.contains("display:block;text-align:center;"), "{}", h);
    assert!(
        h.contains("docdg-lignes span[style*=\"display:block\"]{line-height:8.000mm}"),
        "{}",
        h
    );
    // et la règle générale reste en place pour les fragments de ligne
    assert!(h.contains("docdg-lignes span,docdg-lignes sup,docdg-lignes sub{line-height:0}"), "{}", h);
}

#[test]
fn un_paragraphe_se_justifie_sur_le_cahier() {
    // Le manuel annonçait `justifie` parmi les alignements horizontaux ; le
    // style en ligne ne connaissait que `centre`, `gauche` et `droite`. Il
    // passe par le même span en bloc, donc par la même hauteur de ligne.
    let h = rend("<Écris>sur des lignes{\n<justifié>Un texte assez long pour être justifié sur la largeur.\n}");
    assert!(h.contains("display:block;text-align:justify;"), "{}", h);
    let sans_accent = rend("<Écris>sur des lignes{\n<justifie>Un texte.\n}");
    assert!(sans_accent.contains("text-align:justify;"), "{}", sans_accent);
}

#[test]
fn un_mot_plus_gros_ne_gonfle_pas_sa_ligne() {
    // Même avec un interligne absolu, un span à grande taille gonfle sa boîte
    // de ligne par ses propres métriques — mesuré : 8,14 mm au lieu de 8 mm,
    // et toute la suite du bloc décrochait de la réglure. La règle
    // `span{line-height:0}` retire aux mots le droit de fixer la hauteur :
    // seul l'étai du bloc la donne.
    let h = rend("<Écris>sur des lignes{\nÀ <ARIAL 14pt>{Paris}, il pleut.\n}");
    assert!(
        h.contains("docdg-lignes span,docdg-lignes sup,docdg-lignes sub{line-height:0}"),
        "la règle anti-gonflement manque : {}",
        &h[..h.len().min(400)]
    );
}

#[test]
fn le_mode_mathematique_ne_s_ouvre_pas_sur_le_cahier() {
    let h = rend("<Écris>sur des lignes{\nLe carré : $x^2$ et c'est tout.\n}");
    let bloc = h.split("<docdg-lignes").nth(1).unwrap();
    assert!(bloc.contains("$x^2$"), "le dollar doit rester un caractère : {}", bloc);
    assert!(!bloc.contains("\\("), "une formule s'est composée sur le cahier : {}", bloc);
}

#[test]
fn le_moins_typographique_devient_le_trait_du_cahier() {
    // U+2212 est absent des cursives scolaires : un glyphe absent sort en
    // police de secours au milieu de la cursive, et les métriques de cette
    // police gonflent la boîte de ligne — mesuré à 8,084 mm au lieu de 8.
    let h = rend("<Écris>sur des lignes{\n10 \u{2212} 2 = 8\n}");
    let bloc = h.split("<docdg-lignes").nth(1).unwrap();
    assert!(bloc.contains("10 - 2 = 8"), "{}", bloc);
    assert!(!bloc.contains('\u{2212}'), "{}", bloc);
}
