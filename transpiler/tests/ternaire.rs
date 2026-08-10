use docdg_transpiler::Engine;

fn moteur(reponses: &[(&str, &str)]) -> Engine {
    let mut e = Engine::new();
    for (nom, valeur) in reponses {
        e.saisies.insert(nom.to_string(), valeur.to_string());
    }
    e
}

const FICHE: &str = "soit marié = <Saisis>un booléen{Est-il marié ?}\nsoit statut = si marié vaut vrai { Il est marié. } sinon { Il n'est pas marié. }\nÉtat civil : #statut\n";

#[test]
fn le_ternaire_choisit_selon_vaut_vrai() {
    let html = moteur(&[("marié", "vrai")]).render(FICHE, false).html;
    assert!(html.contains("État civil : Il est marié."));
    assert!(!html.contains("Il n'est pas marié."));
    let html = moteur(&[("marié", "faux")]).render(FICHE, false).html;
    assert!(html.contains("État civil : Il n'est pas marié."));
}

#[test]
fn le_ternaire_accepte_la_condition_nue() {
    let src = "soit marié = <Saisis>un booléen{Est-il marié ?}\nsoit statut = si marié { Il est marié. } sinon { Il n'est pas marié. }\n#statut\n";
    let html = moteur(&[("marié", "vrai")]).render(src, false).html;
    assert!(html.contains("Il est marié."));
    let html = moteur(&[("marié", "faux")]).render(src, false).html;
    assert!(html.contains("Il n'est pas marié."));
}

#[test]
fn le_ternaire_reaffecte_la_variable_elle_meme() {
    let src = "soit marié = <Saisis>un booléen{Est-il marié ?}\nsoit marié = si marié vaut vrai { Il est marié. } sinon { Il n'est pas marié. }\n#marié\n";
    let html = moteur(&[("marié", "vrai")]).render(src, false).html;
    assert!(html.contains("Il est marié."));
    let html = moteur(&[("marié", "faux")]).render(src, false).html;
    assert!(html.contains("Il n'est pas marié."));
}

#[test]
fn le_ternaire_produit_aussi_des_nombres() {
    let src = "soit âge = <Saisis>un entier{Quel est son âge ?}\nsoit tarif = si âge moins de 18 { 5 } sinon { 9 }\nEntrée : #tarif euros, soit #{tarif * 2} euros pour deux.\n";
    let html = moteur(&[("âge", "17")]).render(src, false).html;
    assert!(html.contains("Entrée : 5 euros, soit 10 euros pour deux."));
    let html = moteur(&[("âge", "20")]).render(src, false).html;
    assert!(html.contains("Entrée : 9 euros, soit 18 euros pour deux."));
}

#[test]
fn le_ternaire_sans_sinon_donne_le_vide() {
    let src = "soit x = 3\nsoit note = si x vaut 4 { retenu }\n[#note]\n";
    let html = Engine::new().render(src, false).html;
    assert!(html.contains("[#note]") || html.contains("[]"));
}

#[test]
fn le_ternaire_multiligne_est_compris() {
    let src = "soit marié = <Saisis>un booléen{Est-il marié ?}\nsoit statut = si marié vaut vrai {\n\tIl est marié.\n} sinon {\n\tIl n'est pas marié.\n}\n#statut\n";
    let html = moteur(&[("marié", "faux")]).render(src, false).html;
    assert!(html.contains("Il n'est pas marié."));
    assert!(!html.contains("Il est marié."));
}

#[test]
fn le_bloc_si_accepte_vaut_vrai_et_la_condition_nue() {
    let src = "soit marié = <Saisis>un booléen{Est-il marié ?}\nsi marié vaut vrai {\n\tMarié, donc.\n}\nsi marié {\n\tToujours marié.\n} sinon {\n\tCélibataire.\n}\n";
    let html = moteur(&[("marié", "vrai")]).render(src, false).html;
    assert!(html.contains("Marié, donc."));
    assert!(html.contains("Toujours marié."));
    assert!(!html.contains("Célibataire."));
    let html = moteur(&[("marié", "faux")]).render(src, false).html;
    assert!(!html.contains("Marié, donc."));
    assert!(html.contains("Célibataire."));
}

#[test]
fn le_parallele_egale_le_sequentiel_avec_ternaire() {
    let a = moteur(&[("marié", "vrai")]).render(FICHE, true).html;
    let b = moteur(&[("marié", "vrai")]).render(FICHE, false).html;
    assert_eq!(a, b);
}

#[test]
fn une_branche_texte_ne_se_laisse_pas_confondre_avec_une_variable() {
    let mut e = Engine::new();
    let html = e
        .render(
            "soit marié = 1\nsoit statut = si marié plus de 0 { marié } sinon { célibataire }\n\n#statut",
            false,
        )
        .html;
    assert!(html.contains("marié"), "{}", html);
    assert!(!html.contains(">1<"), "{}", html);
}

#[test]
fn la_branche_texte_non_choisie_impose_aussi_le_texte() {
    let mut e = Engine::new();
    let html = e
        .render(
            "soit âge = 12\nsoit statut = si âge moins de 18 { mineur } sinon { âge }\n\n#statut",
            false,
        )
        .html;
    assert!(html.contains("mineur"), "{}", html);
}

#[test]
fn deux_branches_numeriques_restent_numeriques() {
    let mut e = Engine::new();
    let html = e
        .render(
            "soit âge = 38\nsoit taille = 1,84\nsoit tarif = si âge moins de 18 { 5 } sinon { 9 }\nsoit h = si âge plus de 18 { taille } sinon { 0 }\n\n#tarif et #h et #{tarif * 2}",
            false,
        )
        .html;
    assert!(html.contains("9 et 1,84 et 18"), "{}", html);
}

#[test]
fn un_ternaire_sans_sinon_reste_numerique() {
    let mut e = Engine::new();
    let html = e
        .render("soit âge = 38\nsoit t = si âge plus de 18 { 7 }\n\n#{t + 1}", false)
        .html;
    assert!(html.contains('8'), "{}", html);
}
