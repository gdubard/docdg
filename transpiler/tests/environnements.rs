//! Les environnements numérotés : chaque genre tient son compteur, le
//! chapitre les remet à zéro, l'étiquette et le renvoi se répondent, et le
//! titre s'habille comme un titre de section.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn chaque_genre_tient_son_compteur() {
    let h = rend(
        "<Énonce>le théorème {\nUn.\n}\n\n<Énonce>la définition {\nDeux.\n}\n\n<Énonce>le théorème {\nTrois.\n}\n",
    );
    assert!(h.contains("Théorème 1."), "{}", h);
    assert!(h.contains("Définition 1."), "{}", h);
    assert!(h.contains("Théorème 2."), "{}", h);
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn le_chapitre_prefixe_et_remet_a_zero() {
    let h = rend(
        "soit h0 = <gras chapitre num>\n\n<page suivante h0>Premier\n\n<Énonce>le théorème {\nUn.\n}\n\n<Énonce>le lemme {\nDeux.\n}\n\n<page suivante h0>Second\n\n<Énonce>le théorème {\nTrois.\n}\n",
    );
    assert!(h.contains("Théorème 1.1."), "{}", h);
    assert!(h.contains("Lemme 1.1."), "{}", h);
    assert!(h.contains("Théorème 2.1."), "{}", h);
}

#[test]
fn le_nom_se_depouille_de_sa_liaison() {
    let h = rend(
        "<Énonce>le théorème de Pythagore {\nA.\n}\n\n<Énonce>le théorème des valeurs intermédiaires {\nB.\n}\n\n<Énonce>le théorème de l'angle inscrit {\nC.\n}\n",
    );
    assert!(h.contains("(Pythagore)"), "{}", h);
    assert!(h.contains("(valeurs intermédiaires)"), "{}", h);
    assert!(h.contains("(angle inscrit)"), "{}", h);
}

#[test]
fn l_etiquette_et_le_renvoi_se_repondent_dans_les_deux_sens() {
    let h = rend(
        "\tD'abord le renvoi au théorème <renvoi>{cle}.\n\n<Énonce>le théorème <étiquette>{cle} {\nCorps.\n}\n\n\tPuis le renvoi au théorème <renvoi>{cle}, et un renvoi <renvoi>{fantome} sans étiquette.\n",
    );
    assert_eq!(
        h.matches("<a class=\"renvoi\" href=\"#theoreme-1\">1</a>").count(),
        2,
        "{}",
        h
    );
    assert!(h.contains("renvoi-absent"), "{}", h);
    assert!(h.contains("id=\"theoreme-1\""), "{}", h);
}

#[test]
fn le_style_se_configure_comme_un_titre() {
    let h = rend("soit théorème = <rouge gras>\n\n<Énonce>le théorème {\nCorps.\n}\n");
    assert!(
        h.contains("class=\"env-titre\" style=\"color:#e03131;font-weight:700;\"")
            || h.contains("env-titre\" style=\"color:"),
        "{}",
        h
    );
}

#[test]
fn l_enonce_en_italique_le_discours_en_romain() {
    let h = rend("<Énonce>la proposition {\nA.\n}\n\n<Énonce>la remarque {\nB.\n}\n");
    assert!(h.contains("env-proposition env-italique"), "{}", h);
    assert!(h.contains("env-remarque\""), "{}", h);
    assert!(!h.contains("env-remarque env-italique"), "{}", h);
}

#[test]
fn le_titre_entre_en_vedette_dans_le_premier_paragraphe() {
    let h = rend("<Énonce>la définition {\nUn triangle rectangle possède un angle droit.\n}\n");
    assert!(h.contains("<p><span class=\"env-titre\""), "{}", h);
}

#[test]
fn le_corps_est_du_docdg_entier() {
    let h = rend("<Énonce>le théorème {\nPour tout $x$, on a $x^2 >= 0$.\n}\n");
    assert!(h.contains("\\("), "les formules du corps se composent : {}", h);
}

#[test]
fn la_feuille_de_style_ne_part_qu_avec_un_environnement() {
    let avec = rend("<Énonce>le lemme {\nA.\n}\n");
    assert!(avec.contains(".environnement{"), "{}", avec);
    let sans = rend("\tUn document sur l'environnement et la nature.\n");
    assert!(!sans.contains(".environnement{"), "{}", sans);
}

#[test]
fn l_article_est_obligatoire() {
    let h = rend("<Énonce>théorème {\nCorps.\n}\n");
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("article"), "{}", h);
}

#[test]
fn le_genre_inconnu_se_signale_avec_la_liste() {
    let h = rend("<Énonce>le scholie {\nCorps.\n}\n");
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("scholie"), "{}", h);
    assert!(h.contains("théorème"), "{}", h);
    assert!(h.contains("remarque"), "{}", h);
}

#[test]
fn le_corps_manquant_se_signale() {
    let h = rend("<Énonce>le théorème de Pythagore\n");
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("corps"), "{}", h);
}

#[test]
fn sans_accents_l_enonce_s_ecrit_aussi() {
    let h = rend("<Enonce>le theoreme <etiquette>{t} {\nCorps.\n}\n\n\tVoir <renvoi>{t}.\n");
    assert!(h.contains("Théorème 1."), "{}", h);
    assert!(h.contains("href=\"#theoreme-1\""), "{}", h);
}

#[test]
fn deux_rendus_du_meme_moteur_coincident() {
    let src = "soit h0 = <gras chapitre num>\n\n<page suivante h0>Un\n\n<Énonce>le théorème <étiquette>{t} {\nCorps.\n}\n\n\tVoir <renvoi>{t}.\n";
    let mut moteur = Engine::new();
    let premier = moteur.render(src, false).html;
    let second = moteur.render(src, false).html;
    assert_eq!(premier, second, "le cache doit reproduire le premier rendu");
}

#[test]
fn la_demonstration_se_referme_d_un_tombeau() {
    let h = rend(
        "<Énonce>le théorème {\nToute suite croissante et majorée converge.\n\ndémonstration {\nSoit $l$ la borne supérieure des termes : la croissance y range la suite.\n}\n}\n",
    );
    assert!(h.contains("env-demo-titre\">Démonstration.</span>"), "{}", h);
    assert!(h.contains("env-tombeau\">∎</span></p>"), "{}", h);
    assert!(h.contains(".env-demonstration p{font-style:normal}"), "{}", h);
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn la_recurrence_du_dedans_appelle_la_machinerie_de_montre() {
    let h = rend(
        "<Énonce>la propriété {\nLa somme des $n$ premiers entiers impairs vaut $n^2$.\n\ndémonstration par récurrence que pour tout entier $n$ non nul, la somme des $n$ premiers entiers impairs vaut $n^2$ {\ninitialisation{\nAu rang 1, la somme vaut $1 = 1^2$.\n}\nhérédité{\nDe $n^2$ on passe à $n^2 + 2n + 1 = (n+1)^2$.\n}\n}\n}\n",
    );
    assert!(h.contains("Montrons par récurrence"), "{}", h);
    assert!(h.contains("<strong>Initialisation.</strong>"), "{}", h);
    assert!(h.contains("<strong>Hérédité.</strong>"), "{}", h);
    assert!(h.contains("principe de récurrence"), "{}", h);
    assert!(h.contains("env-tombeau\">∎"), "{}", h);
}

#[test]
fn l_enonce_d_une_phrase_fournit_la_propriete_a_demontrer() {
    let h = rend(
        "<Énonce>le lemme {\nLe produit de deux entiers consécutifs est pair.\n\ndémonstration par disjonction de cas {\ncas $n$ pair {\nUn.\n}\ncas $n$ impair {\nDeux.\n}\n}\n}\n",
    );
    assert!(
        h.contains("Montrons que le produit de deux entiers consécutifs est pair."),
        "la réclamation se dérive de l'énoncé, première lettre pliée : {}",
        h
    );
}

#[test]
fn l_enonce_de_plusieurs_phrases_exige_la_restitution() {
    let h = rend(
        "<Énonce>le théorème {\nToute suite croissante et majorée converge. Toute suite décroissante et minorée converge.\n\ndémonstration par récurrence {\ninitialisation{\nA.\n}\nhérédité{\nB.\n}\n}\n}\n",
    );
    assert!(h.contains("calcul-absent"), "{}", h);
    assert!(h.contains("plusieurs phrases"), "{}", h);
}

#[test]
fn l_etape_manquante_se_signale_aussi_du_dedans() {
    let h = rend(
        "<Énonce>la propriété {\nLa propriété tient.\n\ndémonstration par récurrence {\nhérédité{\nB.\n}\n}\n}\n",
    );
    assert!(h.contains("il manque l'étape"), "{}", h);
}

#[test]
fn le_raisonnement_inconnu_recoit_la_liste_des_formes() {
    let h = rend(
        "<Énonce>le lemme {\nUn énoncé.\n\ndémonstration par magie {\nAbracadabra.\n}\n}\n",
    );
    assert!(h.contains("raisonnement inconnu"), "{}", h);
    assert!(h.contains("par récurrence"), "{}", h);
}

#[test]
fn deux_demonstrations_se_succedent() {
    let h = rend(
        "<Énonce>le théorème {\nUn énoncé.\n\ndémonstration {\nPremière voie.\n}\n\ndémonstration {\nSeconde voie.\n}\n}\n",
    );
    assert_eq!(
        h.matches("env-demo-titre\">Démonstration.").count(),
        2,
        "{}",
        h
    );
    assert_eq!(h.matches("env-tombeau\">∎").count(), 2, "{}", h);
}

#[test]
fn l_enonce_est_une_citation_rien_n_en_sort() {
    let h = rend(
        "<Énonce>le théorème {\n<Soit>une fonction f(x) = x^2\n}\n\n<Calcule>l'image de 2 par f\n",
    );
    assert!(
        h.contains("n'a pas été déclarée"),
        "une déclaration posée dans un énoncé n'existe plus après lui : {}",
        h
    );
}
