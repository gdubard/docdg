use docdg_transpiler::Engine;

const SRC: &str = "soit prénom = <Saisis>un texte{Quel est le prénom de l'élève ?}\nsoit âge = <Saisis>un entier{Quel est son âge ?}\nsoit taille = <Saisis>un décimal{Quelle est sa taille, en mètres ?}\nsoit marié = <Saisis>un booléen{Est-il marié ?}\nsoit initiale = <Saisis>un caractère{Quelle est sa lettre préférée ?}\n\n#prénom a #âge ans, mesure #taille m ; sa lettre préférée est #initiale.\n\nsi marié vaut 1 {\n\tIl est marié.\n} sinon {\n\tIl n'est pas marié.\n}\n";

fn moteur(reponses: &[(&str, &str)]) -> Engine {
    let mut e = Engine::new();
    for (nom, valeur) in reponses {
        e.saisies.insert(nom.to_string(), valeur.to_string());
    }
    e
}

fn rend(reponses: &[(&str, &str)]) -> String {
    moteur(reponses).render(SRC, false).html
}

#[test]
fn sans_reponse_le_document_s_arrete_a_la_premiere_question() {
    let html = rend(&[]);
    assert!(html.contains("Quel est le prénom de l'élève ?"));
    assert!(html.contains("saisie-champ"));
    assert!(html.contains("data-type=\"texte\""));
    assert!(!html.contains("Quel est son âge ?"));
    assert!(!html.contains("mesure"));
    assert!(!html.contains("marié"));
}

#[test]
fn les_questions_se_posent_l_une_apres_l_autre() {
    let html = rend(&[("prénom", "Émile")]);
    assert!(html.contains("saisie-faite"));
    assert!(html.contains("Émile"));
    assert!(html.contains("Quel est son âge ?"));
    assert!(html.contains("data-type=\"entier\""));
    assert!(!html.contains("Quelle est sa taille"));
    assert_eq!(html.matches("saisie-champ").count(), 1);
}

#[test]
fn une_reponse_du_mauvais_type_laisse_le_document_bloque() {
    for brut in ["dix-sept", "17,5", "17.0", ""] {
        let html = rend(&[("prénom", "Émile"), ("âge", brut)]);
        assert!(html.contains("Quel est son âge ?"), "bloqué pour {brut:?}");
        assert!(html.contains("saisie-champ"), "champ présent pour {brut:?}");
        assert!(!html.contains("Quelle est sa taille"), "suite cachée pour {brut:?}");
    }
}

#[test]
fn le_decimal_exige_la_virgule_et_refuse_le_point() {
    let bloque = rend(&[("prénom", "Émile"), ("âge", "17"), ("taille", "1.65")]);
    assert!(bloque.contains("Quelle est sa taille"));
    assert!(bloque.contains("saisie-champ"));
    let passe = rend(&[("prénom", "Émile"), ("âge", "17"), ("taille", "1,65")]);
    assert!(passe.contains("Est-il marié ?"));
    assert!(passe.contains(">1,65<"));
}

#[test]
fn le_booleen_s_ecrit_vrai_ou_faux_et_pilote_la_condition() {
    let complet = &[
        ("prénom", "Émile"),
        ("âge", "17"),
        ("taille", "1,65"),
        ("marié", "vrai"),
        ("initiale", "É"),
    ];
    let html = rend(complet);
    assert!(!html.contains("saisie-champ"));
    assert!(html.contains("Il est marié."));
    assert!(!html.contains("Il n'est pas marié."));
    let mut autre = complet.to_vec();
    autre[3] = ("marié", "faux");
    let html = rend(&autre);
    assert!(html.contains("Il n'est pas marié."));
    assert!(!html.contains("Il est marié."));
    let mut faux = complet.to_vec();
    faux[3] = ("marié", "oui");
    assert!(rend(&faux).contains("saisie-champ"));
}

#[test]
fn le_caractere_est_unique() {
    let base = &[("prénom", "Émile"), ("âge", "17"), ("taille", "1,65"), ("marié", "faux")];
    let mut deux = base.to_vec();
    deux.push(("initiale", "ab"));
    assert!(rend(&deux).contains("saisie-champ"));
    let mut un = base.to_vec();
    un.push(("initiale", "É"));
    let html = rend(&un);
    assert!(!html.contains("saisie-champ"));
    assert!(html.contains("sa lettre préférée est É."));
}

#[test]
fn les_valeurs_saisies_s_interpolent_et_se_calculent() {
    let html = rend(&[
        ("prénom", "Émile"),
        ("âge", "17"),
        ("taille", "1,65"),
        ("marié", "faux"),
        ("initiale", "É"),
    ]);
    assert!(html.contains("Émile a 17 ans, mesure 1,65 m"));
    let mut e = moteur(&[("taille", "1,65")]);
    let r = e.render(
        "soit taille = <Saisis>un décimal{Quelle est sa taille, en mètres ?}\nEn centimètres : #{taille * 100}.",
        false,
    );
    assert!(r.html.contains("En centimètres : 165."));
}

#[test]
fn le_booleen_s_affiche_vrai_ou_faux() {
    let mut e = moteur(&[("marié", "vrai")]);
    let r = e.render(
        "soit marié = <Saisis>un booléen{Est-il marié ?}\nRéponse donnée : #marié.",
        false,
    );
    assert!(r.html.contains("Réponse donnée : vrai."));
    assert!(r.html.contains(">vrai<"));
}

#[test]
fn une_nouvelle_reponse_invalide_le_cache_et_deroule_la_suite() {
    let mut e = Engine::new();
    let bloque = e.render(SRC, false).html;
    assert!(bloque.contains("saisie-champ"));
    e.saisies.insert("prénom".into(), "Léa".into());
    let suite = e.render(SRC, false).html;
    assert!(suite.contains("Léa"));
    assert!(suite.contains("Quel est son âge ?"));
}

#[test]
fn le_parallele_egale_le_sequentiel_avec_saisies() {
    let reponses = &[("prénom", "Émile"), ("âge", "17"), ("taille", "1,65")];
    let a = moteur(reponses).render(SRC, true).html;
    let b = moteur(reponses).render(SRC, false).html;
    assert_eq!(a, b);
}

#[test]
fn l_exemple_basique3_se_bloque_puis_se_deroule() {
    let src = std::fs::read_to_string("../exemples/basique3.txt").unwrap();
    let mut e = Engine::new();
    let bloque = e.render(&src, false).html;
    assert!(bloque.contains("Quel est le prénom"));
    assert!(bloque.contains("saisie-champ"));
    assert!(!bloque.contains("Quel est son âge"));
    for (nom, valeur) in [
        ("prénom", "Émile"),
        ("nom", "Zola"),
        ("âge", "17"),
        ("taille", "1,65"),
        ("marié", "faux"),
        ("initiale", "É"),
    ] {
        e.saisies.insert(nom.into(), valeur.into());
    }
    let html = e.render(&src, false).html;
    assert!(!html.contains("saisie-champ"));
    for attendu in [
        "Émile Zola a 17 ans",
        "165",
        "27",
        "1,77",
        "1650",
        "Il est célibataire",
        "coûte 5 euros, soit 10 euros pour deux",
    ] {
        assert!(html.contains(attendu), "manque {attendu:?}");
    }
}
