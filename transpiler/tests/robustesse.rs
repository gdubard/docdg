use docdg_transpiler::Engine;

#[test]
fn les_verbes_sans_accent_sont_acceptes() {
    let mut e = Engine::new();
    let avec = e.render("<Développe>(x+2)(x-3)", false);
    let mut e = Engine::new();
    let sans = e.render("<Developpe>(x+2)(x-3)", false);
    assert_eq!(avec.html, sans.html);
    assert!(avec.html.contains("class=\"calcul\""));
}

#[test]
fn les_notes_se_numerotent_identiquement_dans_les_deux_modes() {
    let src = "Premier paragraphe<note>{une}.\n\nDeuxième<note>{deux}.\n\nTroisième<note>{trois}.\n\nQuatrième<note>{quatre}.\n\nCinquième<note>{cinq}.";
    let mut a = Engine::new();
    let mut b = Engine::new();
    let paral = a.render(src, true);
    let sequ = b.render(src, false);
    assert_eq!(paral.html, sequ.html);
    for n in 1..=5 {
        assert!(
            paral.html.contains(&format!("<sup class=\"note-ref\">{}</sup>", n)),
            "appel de note {} absent",
            n
        );
        assert!(
            paral.html.contains(&format!("data-num=\"{}\"", n)),
            "corps de note {} absent",
            n
        );
    }
}

#[test]
fn une_expression_tres_imbriquee_ne_fait_pas_tomber_le_moteur() {
    let mut e = Engine::new();
    let src = format!("#{{{}1{}}}", "(".repeat(5000), ")".repeat(5000));
    let r = e.render(&src, false);
    assert!(!r.html.contains("50000"));
}

#[test]
fn une_boucle_demesuree_est_arretee() {
    let mut e = Engine::new();
    let r = e.render("pour n de 1 à 500000 {\n\tligne #n\n}", false);
    assert!(r.html.contains("La boucle a été arrêtée après 2000 tours."));
    assert!(r.html.len() < 200_000);
}

#[test]
fn le_retrait_avec_espaces_insecables_ne_panique_pas() {
    let mut e = Engine::new();
    let src = "pour n dans [1, 2] {\n\u{a0}\u{a0}ligne #n\n   suite #n\n}";
    let r = e.render(src, false);
    assert!(r.html.contains("ligne 1"));
    assert!(r.html.contains("ligne 2"));
}

#[test]
fn une_image_hors_du_dossier_est_refusee() {
    let mut e = Engine::new();
    let r = e.render("<Insère>une image{../../../etc/passwd}", false);
    assert!(r.html.contains("img-absente"));
    assert!(!r.html.contains("base64"));
}

#[test]
fn le_cache_reste_borne_apres_de_nombreuses_editions() {
    let mut e = Engine::new();
    for i in 0..3000 {
        let src = format!("Paragraphe numéro {}.\n\nSecond paragraphe.", i);
        let _ = e.render(&src, false);
    }
    assert!(e.cache_len() <= 4096, "cache non borné : {}", e.cache_len());
}

#[test]
fn le_saut_de_page_tolere_la_casse_et_les_espaces() {
    let attendu = "<div class=\"pagebreak\"></div>";
    for balise in [
        "<page suivante>",
        "<Page suivante>",
        "<PAGE SUIVANTE>",
        "<page  suivante>",
        "<page suivante >",
    ] {
        let mut e = Engine::new();
        let r = e.render(balise, false);
        assert_eq!(r.html, attendu, "balise refusée : {}", balise);
    }
}

#[test]
fn le_saut_de_page_majuscule_porte_aussi_un_style() {
    let mut e = Engine::new();
    let r = e.render(
        "soit h1 = <section grand gras>\n\n<Page suivante h1>Chapitre deux",
        false,
    );
    assert!(r.html.contains("pagebreak"));
    assert!(r.html.contains("class=\"sec lvl1\""));
    assert!(!r.html.contains("calcul-absent"));
}

#[test]
fn le_cadre_du_graphe_epouse_les_sommets() {
    let mut e = Engine::new();
    let r = e.render(
        "<Construis>le graphe G {\n\tA -> B\n\tB -> C\n\tC -> A\n\tA -> C\n}",
        false,
    );
    let debut = r.html.find("viewBox=\"").expect("viewBox absente") + 9;
    let fin = debut + r.html[debut..].find('"').unwrap();
    let bornes: Vec<f32> = r.html[debut..fin]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    assert!(bornes[3] < 140.0, "hauteur excessive : {}", bornes[3]);
    assert!(bornes[2] > bornes[3], "le cadre devrait être plus large que haut");
}

#[test]
fn le_saut_de_page_reste_au_premier_niveau_meme_indente() {
    for entree in [
        "Avant.\n\n<page suivante>test",
        "Avant.\n\n\t<page suivante>test",
        "Avant.\n\n\t\t<page suivante>test",
        "Avant.\n\n    <page suivante>test",
    ] {
        let mut e = Engine::new();
        let html = e.render(entree, false).html;
        let saut = html.find("pagebreak").expect("saut absent");
        let marge = html.find("margin-left");
        assert!(
            marge.map(|m| m > saut).unwrap_or(true),
            "le saut est imbriqué dans la marge : {}",
            html
        );
        assert!(html.contains("test"));
    }
}

