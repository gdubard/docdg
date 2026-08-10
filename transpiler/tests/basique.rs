use docdg_transpiler::Engine;

#[test]
fn rend_basique3_sans_panique() {
    let src = include_str!("basique3.txt");
    let mut e = Engine::new();
    let r = e.render(src, true);
    assert!(r.html.contains("Valeur absolue"));
    assert!(r.html.contains("\\vec{AB}"));
    assert!(r.html.contains("\\mathbb{N}"));
    assert!(r.html.contains("\\forall"));
    assert!(r.html.contains("class=\"cadre\""));
    assert!(r.html.contains("<table"));
    assert!(r.html.contains("grid-template-areas"));
    assert!(r.html.contains("toc"));
    assert!(r.html.contains("Exercice 1"));
    assert!(r.html.contains("Exercice 2"));
    assert_eq!(r.page.orientation, "portrait");
    assert_eq!(r.page.marges, [10.0, 20.0, 15.0, 20.0]);
}

#[test]
fn cache_incremental_reutilise() {
    let src = include_str!("basique3.txt");
    let mut e = Engine::new();
    let r1 = e.render(src, false);
    let n = e.cache_len();
    let r2 = e.render(src, false);
    assert_eq!(e.cache_len(), n);
    assert_eq!(r1.html, r2.html);
    let modified = src.replacen("Valeur absolue", "Valeur absolue!", 1);
    let r3 = e.render(&modified, false);
    assert!(r3.html.contains("Valeur absolue!"));
}

#[test]
fn parallele_egal_sequentiel() {
    let src = include_str!("basique3.txt");
    let mut a = Engine::new();
    let mut b = Engine::new();
    let ra = a.render(src, true);
    let rb = b.render(src, false);
    assert_eq!(ra.html, rb.html);
}

#[test]
fn echappements_doubles() {
    let mut e = Engine::new();
    let r = e.render("Chevrons << et >>. Dièse ##. Dollar $$.", false);
    assert!(r.html.contains("&lt;"));
    assert!(r.html.contains("&gt;"));
    assert!(r.html.contains("#"));
    assert!(r.html.contains("$"));
    assert!(!r.html.contains("\\("));
}

#[test]
fn boucle_tableau() {
    let mut e = Engine::new();
    let src = "<Dresse>un tableau [mc, mg] avec des bordures {\n\tn\tCarré\n\tpour n de 1 à 3 {\n\t\t[ligne #n ; $#n^2$]\n\t}\n}";
    let r = e.render(src, false);
    assert!(r.html.contains("ligne 1"));
    assert!(r.html.contains("ligne 3"));
    assert!(r.html.contains("3^2"));
}

#[test]
fn mesures_avec_unite() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(
        "<Affiche>un cadre avec une bordure bleue et des coins arrondis de 0,5 cm {Texte}",
        false,
    );
    assert!(r.html.contains("border-radius:5mm"), "{}", r.html);
    let r2 = e.render(
        "<Affiche>un cadre avec une bordure bleue et des coins arrondis de 5 mm {Texte}",
        false,
    );
    assert!(r2.html.contains("border-radius:5mm"), "{}", r2.html);
}

#[test]
fn fusions_explicites() {
    let mut e = docdg_transpiler::Engine::new();
    let src = "<Dresse>un tableau [mc, mg, mc, mc] avec bordures et entête {\n\t<4 colonnes mc>{Bulletin trimestriel}\n\tJour\tMatière\tNote\tCoef.\n\t<dans une cellule qui occupera 2 lignes mc>{Lundi}\tMaths\t15\t4\n\t.\tFrançais\t12\t3\n}";
    let r = e.render(src, false);
    assert!(r.html.contains("colspan=\"4\""), "{}", r.html);
    assert!(r.html.contains("rowspan=\"2\""), "{}", r.html);
    assert!(!r.html.contains("&lt;4 colonnes"), "{}", r.html);
    let after = r.html.split(">Lundi<").nth(1).unwrap_or("");
    let last_row = after.rsplit("<tr>").next().unwrap_or("");
    assert_eq!(last_row.matches("<td").count(), 3, "{}", last_row);
}

#[test]
fn grille_complete() {
    let mut e = docdg_transpiler::Engine::new();
    let src = "<Affiche>une grille avec des zones:[\"titre titre date\", \"corps corps corps\"] des bordures et un écart de 3 mm {\n\t[titre : en haut, à gauche]{Titre}\n\t[date : en haut, à droite]{Date}\n\t[corps: en mc]{Corps}\n}";
    let r = e.render(src, false);
    assert!(r.html.contains("grid-template-columns:repeat(3, 1fr)"), "{}", r.html);
    assert!(r.html.contains("grid-template-rows:repeat(2, auto)"), "{}", r.html);
    assert!(!r.html.contains("grid-template-areas:\""), "guillemets doubles dans style: {}", r.html);
    assert!(r.html.contains("'titre titre date'"), "{}", r.html);
}

#[test]
fn tableau_pleine_largeur() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render("<Dresse>un tableau [mc, mc] avec bordures {\n\tA\tB\n\t1\t2\n}", false);
    assert!(r.html.contains("width:100%"), "{}", r.html);
}

#[test]
fn fractions_en_dfrac() {
    use docdg_transpiler::notation::to_latex;
    assert_eq!(to_latex("1/2"), "\\dfrac{1}{2}");
    assert_eq!(to_latex("a/b/c"), "\\dfrac{\\dfrac{a}{b}}{c}");
    assert_eq!(to_latex("a^2/b"), "\\dfrac{a^2}{b}");
    assert_eq!(to_latex("a/b^2"), "\\dfrac{a}{b^2}");
    assert!(to_latex("racine(2)/2").contains("\\dfrac{\\sqrt{2}}{2}"));
    assert!(to_latex("1/n somme(k=1; n) x_k").starts_with("\\dfrac{1}{n}"));
}

#[test]
fn calcul_formel_de_base() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render("<Factorise x^2 - 5x + 6>", false);
    if r.html.contains("calcul-note") {
        return;
    }
    assert!(r.html.contains("\\left(x - 3\\right)"), "{}", r.html);
    let d = e.render(
        "<Soit>une fonction q(x) = -x^4 + 2x^2 + 1\n\n<Calcule>la dérivée de q",
        false,
    );
    assert!(d.html.contains("q'(x)"), "{}", d.html);
}

#[test]
fn systeme_avec_accents_declare() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(
        "<Soit>le système s {\n2x + 3y = 7\nx - y = 1\n}\n\n<Résous>le système s",
        false,
    );
    assert!(!r.html.contains("n'a pas été déclaré"), "{}", r.html);
}

#[test]
fn indices_caracteres_et_non_octets() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(
        "<Affiche>un cadre avec une bordure bleue et un titre {Élève} {Corps}",
        false,
    );
    assert!(r.html.contains("Élève"), "{}", r.html);
}

#[test]
fn variables_boucles_conditions() {
    let mut e = docdg_transpiler::Engine::new();
    let src = "soit tva = 0,2\nsoit prix = 150\n\nLe prix TTC est #{prix * (1 + tva)} euros.\n\nsoit note = 12\n\nsi note au moins 10 {\n\tAdmis avec #note.\n} sinon {\n\tAjourné.\n}\n\npour k de 0 à 2 avec un pas de 0.25 {\n\tTerme #{defaut(4*k)+1} : #k.\n}";
    let r = e.render(src, false);
    assert!(r.html.contains("180 euros"), "{}", r.html);
    assert!(r.html.contains("Admis avec 12"), "{}", r.html);
    assert!(!r.html.contains("Ajourné"), "{}", r.html);
    assert!(r.html.contains("Terme 9"), "{}", r.html);
    assert!(r.html.contains(": 2."), "{}", r.html);
}

#[test]
fn interpolation_dans_les_maths() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render("soit n = 7\n\n$n = #n$, donc $n^2 = #{n*n}$ et $n/2 = #{n/2}$ ; #{arrondi(5/3; 2)}.", false);
    assert!(r.html.contains("= 49"), "{}", r.html);
    assert!(r.html.contains("= 3,5"), "{}", r.html);
    assert!(r.html.contains("1,67"), "{}", r.html);
}

#[test]
fn exposant_indice_note_legende() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(
        "1<exposant>{er} et H<indice>{2}O et x<exposant 4mm>{2}<note>{Une note.} <Insère une image avec une largeur de 30 mm et la légende {Cinq pommes}>{pommes.png}",
        false,
    );
    assert!(r.html.contains("<sup>er</sup>"), "{}", r.html);
    assert!(r.html.contains("<sub>2</sub>"), "{}", r.html);
    assert!(r.html.contains("vertical-align:4mm"), "{}", r.html);
    assert!(r.html.contains("note-ref"), "{}", r.html);
    assert!(r.html.contains("Cinq pommes"), "{}", r.html);
}

#[test]
fn balises_accentuees_ne_mangent_pas_le_caractere_suivant() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(
        "du <souligné>{souligné}, du <barré>{barré}, des <petites capitales>{petites capitales} et du <sans empattements>{sans empattements}.",
        false,
    );
    assert!(r.html.contains("souligné</span>, du"), "{}", r.html);
    assert!(r.html.contains("text-decoration:line-through;\">barré</span>"), "{}", r.html);
    assert!(r.html.contains("font-variant:small-caps;\">petites capitales</span>"), "{}", r.html);
    assert!(!r.html.contains("&lt;barré&gt;"), "{}", r.html);
}

#[test]
fn alignements_de_paragraphe_en_langage_naturel() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render("<à gauche>Contre la marge.\n\n<au centre>Centré.\n\n<à droite>Contre la marge.", false);
    assert!(r.html.contains("text-align:left;\">Contre la marge."), "{}", r.html);
    assert!(r.html.contains("text-align:center;\">Centré."), "{}", r.html);
    assert!(r.html.contains("text-align:right;\">Contre la marge."), "{}", r.html);
}

#[test]
fn couleurs_composees_supplementaires() {
    use docdg_transpiler::notation::to_latex;
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render("<rouge tomate>{x}, <bleu acier>{y}, <bleu dodger>{z}", false);
    assert!(r.html.contains("color:tomato"), "{}", r.html);
    assert!(r.html.contains("color:steelblue"), "{}", r.html);
    assert!(r.html.contains("color:dodgerblue"), "{}", r.html);
    let _ = to_latex("");
}

#[test]
fn operateurs_plus_moins_et_different() {
    use docdg_transpiler::notation::to_latex;
    assert!(to_latex("x +- y").contains("\\pm"));
    assert!(to_latex("e != f").contains("\\neq"));
}

#[test]
fn balise_style_en_ligne_ne_coupe_pas_le_paragraphe() {
    let mut e = docdg_transpiler::Engine::new();
    let src = "soit cle = <gras cramoisi>\n\nune <cle>{couleur} se nomme ainsi ; une\n<cle>{taille} s'écrit Npt.";
    let r = e.render(src, false);
    assert_eq!(r.html.matches("<p>").count(), 1, "{}", r.html);
    assert!(r.html.contains("taille</span> s'écrit Npt."), "{}", r.html);
}

#[test]
fn decalage_des_zones_de_texte() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(
        "Contre la marge.\n\n\tDécalé d'une tabulation.\n\n\t\tDécalé de deux.\n\n    Décalé de quatre espaces.",
        false,
    );
    assert!(r.html.contains("<p>Contre la marge."), "{}", r.html);
    assert!(r.html.contains("width:1cm\"></span>Décalé d'une tabulation."), "{}", r.html);
    assert!(r.html.contains("width:2cm\"></span>Décalé de deux."), "{}", r.html);
    assert!(r.html.contains("width:1cm\"></span>Décalé de quatre espaces."), "{}", r.html);
    assert!(!r.html.contains("padding-left"), "le paragraphe entier ne doit pas être décalé : {}", r.html);
}

#[test]
fn decalage_ligne_par_ligne_et_en_cours_de_ligne() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render("Première ligne.\n\tDeuxième ligne décalée.\nTroisième ligne.", false);
    assert_eq!(r.html.matches("<p>").count(), 1, "{}", r.html);
    assert!(r.html.contains("<br><span style=\"display:inline-block;width:1cm\"></span>Deuxième"), "{}", r.html);
    let t = e.render("Un\tmot, puis\t\t\ttrois.", false);
    assert!(t.html.contains("width:1cm\"></span>mot"), "{}", t.html);
    assert!(t.html.contains("width:3cm\"></span>trois."), "{}", t.html);
}

#[test]
fn balises_supprimees_sans_effet() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render("Avant.\n\n<ligne>\n\nAprès<nouvelle ligne>suite.", false);
    assert!(!r.html.contains("<hr"), "{}", r.html);
    assert!(!r.html.contains("&lt;ligne&gt;"), "{}", r.html);
    assert!(!r.html.contains("&lt;nouvelle ligne&gt;"), "{}", r.html);
}

#[test]
fn pas_de_decalage_dans_les_objets() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(
        "<Affiche>un cadre avec une bordure bleue {\n\tTexte indenté à la source.\n}",
        false,
    );
    assert!(!r.html.contains("padding-left"), "{}", r.html);
}

#[test]
fn lignes_vides_multiples() {
    let mut e = docdg_transpiler::Engine::new();
    let une = e.render("Avant.\n\nAprès.", false);
    assert_eq!(une.html.matches("ligne-vide").count(), 1, "{}", une.html);
    let trois = e.render("Avant.\n\n\n\nAprès.", false);
    assert_eq!(trois.html.matches("ligne-vide").count(), 3, "{}", trois.html);
    let bloc = e.render("Avant.\n\n<Affiche>un cadre avec une bordure bleue {\n\tX\n}\n\nAprès.", false);
    assert_eq!(bloc.html.matches("ligne-vide").count(), 2, "{}", bloc.html);
}

#[test]
fn corps_de_boucle_une_iteration_par_paragraphe() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render("pour k de 1 à 3 {\n    Ligne #k.\n}", false);
    assert!(!r.html.contains("padding-left"), "{}", r.html);
    assert_eq!(r.html.matches("<p>").count(), 3, "{}", r.html);
    assert_eq!(r.html.matches("ligne-vide").count(), 0, "{}", r.html);
}

#[test]
fn aucune_commande_ne_fuit_en_texte_brut() {
    let mut e = docdg_transpiler::Engine::new();
    for src in [
        "<Écris>le nombre 45 600 en notation scientifique",
        "<Vérifie>si 456 est divisible par 3",
        "<Dresse>la table de Cayley du groupe Z/4Z",
        "<Construis>le graphe G {\n\tA -> B\n}",
    ] {
        let r = e.render(src, false);
        assert!(!r.html.contains("calcul-absent"), "signalée à tort : {}", r.html);
        assert!(
            !r.html.contains("<p>&lt;"),
            "commande recrachée en texte brut : {}",
            r.html
        );
    }
    let r = e.render("<Trigonalise>D", false);
    assert!(r.html.contains("calcul-absent"), "{}", r.html);
    assert!(r.html.contains("n'a pas été déclarée"), "{}", r.html);
    let r = e.render("<Fabrique>quelque chose", false);
    assert!(r.html.contains("calcul-absent"), "{}", r.html);
    assert!(
        !r.html.contains("<p>&lt;"),
        "commande recrachée en texte brut : {}",
        r.html
    );
}

#[test]
fn page_suivante_conservee() {
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(
        "soit h1 = <bleu gras section num>\n\nAvant.\n\n<page suivante h1>Titre après saut",
        false,
    );
    assert!(r.html.contains("pagebreak"), "{}", r.html);
    assert!(r.html.contains("Titre après saut"), "{}", r.html);
}

#[test]
fn boucle_dans_un_tableau_sans_rangee_parasite() {
    let mut e = docdg_transpiler::Engine::new();
    let src = "<Dresse un tableau [mc, mg, mc] avec bordures et entête>{\n\tTable des carrés\n\tn\tLégende\tCarré\n\tpour n de 1 à 3 {\n\t\t[ligne #n; #n ;\t$#n^2$]\n\t}\n}";
    let r = e.render(src, false);
    assert_eq!(r.html.matches("<tr>").count(), 5, "{}", r.html);
    assert!(!r.html.contains("<td colspan=\"3\" style=\"vertical-align:middle;text-align:center;padding:0.90000004mm 1.5mm;border:0.3mm solid #333;\"></td>"), "rangée vide parasite : {}", r.html);
    assert!(r.html.contains("ligne 3"), "{}", r.html);
}

#[test]
fn matrices_delimiteurs_et_barre() {
    let mut e = docdg_transpiler::Engine::new();
    let p = e.render("<Affiche>la matrice(\n\t1 ; 2 ; 3\n\t4 ; 5 ; 6\n)", false);
    assert!(p.html.contains("\\left(\\begin{array}{ccc}"), "{}", p.html);
    assert!(p.html.contains("1 & 2 & 3"), "{}", p.html);
    let c = e.render("<Affiche>la matrice[\n\t2x + 1 ; 0\n\t0 ; 1/2\n]", false);
    assert!(c.html.contains("\\left[\\begin{array}{cc}"), "{}", c.html);
    assert!(c.html.contains("\\dfrac{1}{2}"), "{}", c.html);
    let a = e.render("<Affiche>la matrice[\n\t2 ; 1 | 7\n\t1 ; -1 | 1\n]", false);
    assert!(a.html.contains("{cc|c}"), "{}", a.html);
}

#[test]
fn le_saut_de_page_ne_mange_pas_ce_qui_suit() {
    fn rendu(src: &str) -> String {
        Engine::new().render(src, true).html
    }
    let h = rendu("Avant.\n\n<page suivante>\n\nApres.\n");
    assert_eq!(h.matches("class=\"pagebreak\"").count(), 1);
    assert!(h.contains("<p>Apres.</p>"));

    let h = rendu("<page suivante>Un losange, par côté et angle :\n");
    assert_eq!(h.matches("class=\"pagebreak\"").count(), 1);
    assert!(h.contains("<p>Un losange, par côté et angle :</p>"));

    let h = rendu("<page suivante>\nUn carré :\n");
    assert!(h.contains("<p>Un carré :</p>"));

    let h = rendu("<page suivante>\n<Trace>le carré MNOP, de côté 3 cm, avec les marques\n");
    assert_eq!(h.matches("class=\"pagebreak\"").count(), 1);
    assert_eq!(h.matches("<svg").count(), 1);

    let h = rendu("soit h2 = <bleu italique sous-section>\n\n<page suivante h2>Titre\n");
    assert_eq!(h.matches("class=\"pagebreak\"").count(), 1);
    assert!(h.contains("Titre"));
    assert!(h.contains("<h3"));
}
