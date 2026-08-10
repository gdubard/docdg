use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    let mut e = Engine::new();
    e.render(src, false).html
}

fn page_json(src: &str) -> String {
    let mut e = Engine::new();
    e.render(src, false).page.to_json()
}

#[test]
fn la_police_du_document_se_declare_par_son_nom() {
    let j = page_json("page {\n\tpolice: Latin Modern Roman;\n\ttaille: 12;\n}\nBonjour.");
    assert!(j.contains("\"police\":\"Latin Modern Roman\""));
    assert!(j.contains("\"taille\":12"));
}

#[test]
fn taille_et_police_sont_independantes() {
    let j = page_json("page {\n\ttaille: 14;\n}\nBonjour.");
    assert!(j.contains("\"taille\":14"));
    assert!(j.contains("\"police\":\"\""));
}

#[test]
fn la_police_mathematique_est_stockee() {
    let j = page_json("page {\n\tmath: Latin Modern Math;\n}\nBonjour.");
    assert!(j.contains("\"math\":\"Latin Modern Math\""));
}

#[test]
fn les_nouvelles_cles_ont_des_defauts_stables() {
    let j = page_json("Bonjour.");
    assert!(j.contains("\"tabulation\":10"));
    assert!(j.contains("\"hauteur\":5"));
    assert!(j.contains("\"decalage\":100"));
    assert!(j.contains("\"precision\":-1"));
}

#[test]
fn la_tabulation_pilote_l_indentation() {
    let large = rend("page {\n\ttabulation: 20;\n}\n\tTexte indenté.");
    assert!(large.contains("width:2cm"), "{}", large);
    let fine = rend("page {\n\ttabulation: 5;\n}\n\tTexte indenté.");
    assert!(fine.contains("width:0.5cm"), "{}", fine);
    let defaut = rend("\tTexte indenté.");
    assert!(defaut.contains("width:1cm"), "{}", defaut);
}

#[test]
fn la_hauteur_pilote_la_ligne_vide() {
    let haute = rend("page {\n\thauteur: 12;\n}\nUn.\n\nDeux.");
    assert!(haute.contains("height:1.2cm"), "{}", haute);
    let defaut = rend("Un.\n\nDeux.");
    assert!(defaut.contains("height:0.5cm"), "{}", defaut);
}

#[test]
fn la_precision_arrondit_les_valeurs_numeriques() {
    let trois = rend("page {\n\tprécision: 3;\n}\n<Calcule>la masse molaire de H2O");
    assert!(trois.contains("18{,}000"), "{}", trois);
    let deux = rend("page {\n\tprécision: 2;\n}\n<Convertis>1 h en min");
    assert!(deux.contains("60{,}00") || deux.contains("60,00"), "{}", deux);
    let libre = rend("<Calcule>la masse molaire de H2O");
    assert!(libre.contains("= 18\\"), "{}", libre);
}

#[test]
fn une_police_locale_en_majuscules_s_applique() {
    let html = rend("<TIMES NEW ROMAN>{un fragment}");
    assert!(html.contains("font-family:'TIMES NEW ROMAN'"), "{}", html);
    assert!(html.contains(">un fragment</span>"), "{}", html);
}

#[test]
fn la_police_locale_se_combine_aux_autres_styles() {
    let html = rend("<au centre italique 14pt>Je suis<TIMES NEW ROMAN gras>{très}fatigué.");
    assert!(html.contains("text-align:center"), "{}", html);
    assert!(html.contains("font-style:italic"), "{}", html);
    assert!(html.contains("font-size:14pt"), "{}", html);
    assert!(html.contains("font-family:'TIMES NEW ROMAN';font-weight:700"), "{}", html);
}

#[test]
fn la_police_locale_s_utilise_dans_un_style_nomme() {
    let html = rend("soit manuscrit = <SCHOLA italique>\n\n<manuscrit>{écrit à la main}");
    assert!(html.contains("font-family:'SCHOLA'"), "{}", html);
    assert!(html.contains("font-style:italic"), "{}", html);
}

#[test]
fn un_mot_minuscule_inconnu_ne_devient_pas_une_police() {
    let html = rend("<inconnu>du texte");
    assert!(!html.contains("font-family:'inconnu'"), "{}", html);
}

#[test]
fn les_reglages_ne_fuient_pas_entre_documents() {
    let _ = rend("page {\n\tprécision: 0;\n\ttabulation: 30;\n}\n<Calcule>la masse molaire de H2O");
    let apres = rend("\tTexte.\n\n<Calcule>la masse molaire de H2O");
    assert!(apres.contains("width:1cm"), "{}", apres);
    assert!(apres.contains("= 18\\"), "{}", apres);
}

#[test]
fn le_parallele_egale_le_sequentiel_avec_reglages() {
    let src = "page {\n\ttabulation: 8;\n\thauteur: 8;\n\tprécision: 2;\n}\n\tUn paragraphe.\n\n<Calcule>la masse molaire de CO2\n\n<Convertis>90 km/h en m/s";
    let mut a = Engine::new();
    let mut b = Engine::new();
    assert_eq!(a.render(src, true).html, b.render(src, false).html);
}

#[test]
fn les_chapitres_se_numerotent_et_prefixent_les_sections() {
    let src = "soit h0 = <gras chapitre num>\nsoit h1 = <gras section num>\n\n<h0>Premier\n\n<h1>Une\n\n<h1>Deux\n\n<h0>Second\n\n<h1>Trois";
    let html = rend(src);
    assert!(html.contains("id=\"chap-1\""), "{}", html);
    assert!(html.contains("id=\"chap-2\""), "{}", html);
    assert!(html.contains(">1.1</span>"), "{}", html);
    assert!(html.contains(">1.2</span>"), "{}", html);
    assert!(html.contains(">2.1</span>"), "{}", html);
    assert!(html.contains("class=\"sec lvl0\""), "{}", html);
}

#[test]
fn sans_chapitre_la_numerotation_reste_inchangee() {
    let html = rend("soit h1 = <gras section num>\n\n<h1>Seule");
    assert!(html.contains("id=\"sec-1\""), "{}", html);
    assert!(html.contains(">1</span>"), "{}", html);
    assert!(!html.contains("0.1"), "{}", html);
}

#[test]
fn la_page_de_titre_compose_les_metadonnees() {
    let src = "document {\n\ttitre: Essai;\n\tauteur: G. Dubard;\n\tinstitution: Lyon;\n\tdate: 2026;\n}\n<page de titre>\n\nLa suite.";
    let html = rend(src);
    assert!(html.contains("class=\"titre-doc\">Essai<"), "{}", html);
    assert!(html.contains("class=\"auteur-doc\">G. Dubard<"), "{}", html);
    assert!(html.contains("class=\"institution-doc\">Lyon<"), "{}", html);
    assert!(html.contains("class=\"date-doc\">2026<"), "{}", html);
    assert!(html.contains("pagebreak"), "{}", html);
    assert!(!html.contains("document {"), "{}", html);
}

#[test]
fn la_page_de_titre_omet_les_cles_absentes() {
    let src = "document {\n\ttitre: Essai;\n}\n<page de titre>";
    let html = rend(src);
    assert!(html.contains("titre-doc"), "{}", html);
    assert!(!html.contains("auteur-doc"), "{}", html);
}

#[test]
fn changer_les_metadonnees_invalide_le_cache() {
    let mut e = Engine::new();
    let a = e
        .render("document {\n\ttitre: Un;\n}\n<page de titre>", false)
        .html;
    let b = e
        .render("document {\n\ttitre: Deux;\n}\n<page de titre>", false)
        .html;
    assert!(a.contains(">Un<"), "{}", a);
    assert!(b.contains(">Deux<"), "{}", b);
}

#[test]
fn le_parallele_egale_le_sequentiel_avec_chapitres() {
    let src = "document {\n\ttitre: Essai;\n\tauteur: G. D.;\n}\nsoit h0 = <gras chapitre num>\nsoit h1 = <gras section num>\n\n<page de titre>\n\n<table des matières>{Sommaire}\n\n<h0>Alpha\n\n<h1>Un\n\nTexte.\n\n<h0>Bêta\n\n<h1>Deux\n\nTexte.";
    let mut a = Engine::new();
    let mut b = Engine::new();
    assert_eq!(a.render(src, true).html, b.render(src, false).html);
}

#[test]
fn une_etiquette_se_renvoie_par_son_numero() {
    let src = "soit h1 = <gras section num>\n\n<h1>Le modèle <étiquette>{modele}\n\nTexte.\n\n<h1>Suite\n\nVoir la section <renvoi>{modele}.";
    let html = rend(src);
    assert!(html.contains("<a class=\"renvoi\" href=\"#sec-1\">1</a>"), "{}", html);
    assert!(!html.contains('\u{E016}'), "{}", html);
    assert!(!html.contains('\u{E018}'), "{}", html);
}

#[test]
fn le_renvoi_precede_l_etiquette_sans_dommage() {
    let src = "soit h1 = <gras section num>\n\n<h1>Avant\n\nVoir la section <renvoi>{cible}.\n\n<h1>La cible <étiquette>{cible}\n\nTexte.";
    let html = rend(src);
    assert!(html.contains("href=\"#sec-2\">2</a>"), "{}", html);
}

#[test]
fn une_etiquette_sous_chapitre_porte_le_prefixe() {
    let src = "soit h0 = <gras chapitre num>\nsoit h1 = <gras section num>\n\n<h0>Un\n\n<h1>Section <étiquette>{s}\n\n<renvoi>{s}";
    let html = rend(src);
    assert!(html.contains("href=\"#sec-1-1\">1.1</a>"), "{}", html);
}

#[test]
fn un_renvoi_inconnu_s_affiche_en_double_point_d_interrogation() {
    let html = rend("Voir <renvoi>{fantome}.");
    assert!(html.contains("<span class=\"renvoi-absent\">??</span>"), "{}", html);
}

#[test]
fn la_bibliographie_numerote_et_ancre_les_entrees() {
    let src = "Le GUM <cite>{gum} fait foi.\n\n<Dresse>une bibliographie {\n\t[gum] JCGM, Guide, 2008.\n\t[taylor] J. Taylor, Incertitudes, Dunod, 2000.\n}";
    let html = rend(src);
    assert!(html.contains("id=\"bib-gum\""), "{}", html);
    assert!(html.contains("class=\"bib-num\">[1]</span> JCGM"), "{}", html);
    assert!(html.contains("class=\"bib-num\">[2]</span> J. Taylor"), "{}", html);
    assert!(html.contains("<a class=\"renvoi\" href=\"#bib-gum\">[1]</a>"), "{}", html);
}

#[test]
fn une_citation_multiple_se_groupe() {
    let src = "Voir <cite>{a, b}.\n\n<Dresse>une bibliographie {\n\t[a] Premier.\n\t[b] Second.\n}";
    let html = rend(src);
    assert!(html.contains("href=\"#bib-a\">[1]</a>"), "{}", html);
    assert!(html.contains("href=\"#bib-b\">[2]</a>"), "{}", html);
    assert!(html.contains("cite-sep"), "{}", html);
}

#[test]
fn le_parallele_egale_le_sequentiel_avec_renvois() {
    let src = "soit h1 = <gras section num>\n\n<h1>Une <étiquette>{u}\n\nVoir <renvoi>{u} et <cite>{gum}.\n\n<h1>Deux\n\nTexte.\n\n<Dresse>une bibliographie {\n\t[gum] JCGM, Guide, 2008.\n}";
    let mut a = Engine::new();
    let mut b = Engine::new();
    assert_eq!(a.render(src, true).html, b.render(src, false).html);
}

#[test]
fn le_bloc_document_fusionne_metadonnees_et_reglages() {
    let src = "document {\n\ttitre: Essai;\n\tauteur: G. Dubard;\n\tmarges: 25;\n\tpolice: Georgia;\n\ttaille: 12;\n\tprécision: 2;\n}\n<page de titre>\n\n\tTexte.\n\n<Calcule>la masse molaire de H2O";
    let mut e = Engine::new();
    let r = e.render(src, false);
    assert!(r.html.contains("class=\"titre-doc\">Essai<"), "{}", r.html);
    assert!(r.html.contains("18{,}00"), "{}", r.html);
    let j = r.page.to_json();
    assert!(j.contains("\"marges\":[25,25,25,25]"), "{}", j);
    assert!(j.contains("\"police\":\"Georgia\""), "{}", j);
    assert!(j.contains("\"taille\":12"), "{}", j);
    assert!(j.contains("\"titre\":\"Essai\""), "{}", j);
    assert!(j.contains("\"auteur\":\"G. Dubard\""), "{}", j);
}

#[test]
fn les_blocs_page_et_document_restent_acceptes_dans_les_deux_ordres() {
    let a = page_json("page {\n\ttaille: 13;\n}\ndocument {\n\ttitre: T;\n}\nBonjour.");
    assert!(a.contains("\"taille\":13") && a.contains("\"titre\":\"T\""), "{}", a);
    let b = page_json("document {\n\ttitre: T;\n}\npage {\n\ttaille: 13;\n}\nBonjour.");
    assert!(b.contains("\"taille\":13") && b.contains("\"titre\":\"T\""), "{}", b);
}
