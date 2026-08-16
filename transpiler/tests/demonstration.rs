//! Les squelettes rhétoriques des démonstrations : chaque raisonnement
//! s'annonce, déroule ses étapes nommées, et se referme quand sa logique
//! l'exige.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn la_directe_annonce_puis_deroule() {
    let h = rend("<Montre>que la somme de deux entiers pairs est paire {\nSoient $a$ et $b$ deux entiers pairs.\n}");
    assert!(h.contains("Montrons que la somme de deux entiers pairs est paire."), "{}", h);
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn la_recurrence_invoque_son_principe() {
    let h = rend("<Montre par récurrence>que pour tout entier $n$, $P(n)$ {\ninitialisation{\nvrai au rang 0.\n}\nhérédité{\nla propriété se transmet.\n}\n}");
    assert!(h.contains("Montrons par récurrence"), "{}", h);
    assert!(h.contains("<strong>Initialisation.</strong>"), "{}", h);
    assert!(h.contains("<strong>Hérédité.</strong>"), "{}", h);
    assert!(h.contains("principe de récurrence"), "{}", h);
}

#[test]
fn la_recurrence_exige_ses_deux_etapes() {
    let h = rend("<Montre par récurrence>que rien {\nhérédité{\nx\n}\n}");
    assert!(h.contains("il manque l'étape « initialisation{…} »"), "{}", h);
}

#[test]
fn la_contraposee_revient_a_l_enonce() {
    let h = rend("<Montre par contraposée>que si $n^2$ est pair, alors $n$ est pair {\ncontraposée{\nsi $n$ est impair, alors $n^2$ est impair\n}\ncorps.\n}");
    assert!(h.contains("Raisonnons par contraposée : montrons que"), "{}", h);
    assert!(h.contains("Par contraposition,"), "{}", h);
}

#[test]
fn l_absurde_suppose_puis_conclut() {
    let h = rend("<Montre par l'absurde>que $racine(2)$ est irrationnel {\nabsurde{\n$racine(2)$ est rationnel\n}\ncorps.\ncontradiction{\nimpossible\n}\n}");
    assert!(h.contains("Raisonnons par l'absurde : supposons que"), "{}", h);
    assert!(h.contains("Contradiction : impossible."), "{}", h);
    assert!(h.contains("Donc \\(\\sqrt{2}\\) est irrationnel."), "{}", h);
}

#[test]
fn la_disjonction_numerote_et_recouvre() {
    let h = rend("<Montre par disjonction de cas>que $E$ {\ncas $n$ pair {\nun.\n}\ncas $n$ impair {\ndeux.\n}\n}");
    assert!(h.contains("<strong>Premier cas</strong>"), "{}", h);
    assert!(h.contains("<strong>Deuxième cas</strong>"), "{}", h);
    assert!(h.contains("Dans tous les cas,"), "{}", h);
}

#[test]
fn l_analyse_synthese_s_annonce() {
    // L'annonce n'est pas une politesse : l'analyse suppose la conclusion
    // vraie, et sans elle un correcteur pressé lit une pétition de principe.
    let h = rend("<Montre par analyse-synthèse>que $E$ {\nanalyse{\nconditions nécessaires.\n}\nsynthèse{\nvérification.\n}\n}");
    assert!(h.contains("Raisonnons par analyse-synthèse."), "{}", h);
    assert!(h.contains("<strong>Analyse.</strong>"), "{}", h);
    assert!(h.contains("<strong>Synthèse.</strong>"), "{}", h);
}

#[test]
fn la_double_inclusion_assemble_ses_moities() {
    let h = rend("<Montre par double inclusion>que $A = B$ {\ndirecte{\nun sens.\n}\nréciproque{\nl'autre.\n}\n}");
    assert!(h.contains("Montrons par double inclusion"), "{}", h);
    assert!(h.contains("Par double inclusion,"), "{}", h);
}

#[test]
fn le_pour_tout_fixe_un_element_quelconque() {
    let h = rend("<Montre>par élément quelconque réel $x$, $x^2 >= 0$ {\nsoit{\n$x$ un réel quelconque\n}\ncorps.\n}");
    assert!(h.contains("Montrons que pour tout réel"), "{}", h);
    assert!(h.contains("Soit \\(x\\) un réel quelconque."), "{}", h);
    assert!(h.contains("L'élément était quelconque"), "{}", h);
}

#[test]
fn les_tiroirs_nomment_objets_et_tiroirs() {
    let h = rend("<Montre par le principe des tiroirs>que $E$ {\nobjets{\ntreize personnes\n}\ntiroirs{\ndouze mois\n}\n}");
    assert!(h.contains("Les objets sont treize personnes ; les tiroirs sont douze mois."), "{}", h);
    assert!(h.contains("par le principe des tiroirs,"), "{}", h);
}

#[test]
fn l_existence_et_l_unicite_se_traitent_separement() {
    let h = rend("<Montre>l'existence et l'unicité du milieu {\nexistence{\non construit.\n}\nunicité{\ndeux candidats se confondent.\n}\n}");
    assert!(h.contains("Montrons l'existence et l'unicité du milieu."), "{}", h);
    assert!(h.contains("<strong>Existence.</strong>"), "{}", h);
    assert!(h.contains("<strong>Unicité.</strong>"), "{}", h);
}

#[test]
fn un_raisonnement_inconnu_liste_les_formes_admises() {
    let h = rend("<Montre par magie>que rien {\nx\n}");
    assert!(h.contains("raisonnement inconnu"), "{}", h);
    assert!(h.contains("par récurrence"), "{}", h);
}

#[test]
fn le_corps_reste_du_docdg_vivant() {
    // Une étape peut contenir des commandes du langage, pas seulement de la
    // prose : le corps est rendu par le moteur, non recopié.
    let h = rend("soit a = 6\n<Montre>que tout va bien {\nLa valeur vaut #a.\n}");
    assert!(h.contains("La valeur vaut 6."), "{}", h);
}

// ═══════════ les démonstrations que le moteur écrit lui-même ═══════════

#[test]
fn la_recurrence_sans_corps_se_demontre_seule() {
    let h = rend("<Montre par récurrence>que pour tout entier $n$, $somme(k=0;n) k = (n(n+1))/2$");
    assert!(h.contains("Montrons par récurrence"), "{}", h);
    assert!(h.contains("la propriété est vraie au premier rang"), "{}", h);
    assert!(h.contains("hypothèse de récurrence"), "{}", h);
    assert!(h.contains("principe de récurrence"), "{}", h);
    assert!(!h.contains("calcul-absent"), "{}", h);
}

#[test]
fn le_moteur_refuse_de_demontrer_une_formule_fausse() {
    let h = rend("<Montre par récurrence>que pour tout entier $n$, $somme(k=0;n) k = n^2$");
    assert!(h.contains("ne se transmet pas au rang suivant"), "{}", h);
}

#[test]
fn le_pour_tout_sans_corps_passe_par_la_forme_canonique() {
    let h = rend("<Montre>par élément quelconque réel $x$, $x^2 + 1 >= 2x$");
    assert!(h.contains("Soit \\(x\\) un réel quelconque."), "{}", h);
    assert!(h.contains("Un carré est positif ou nul"), "{}", h);
    assert!(h.contains("L'élément était quelconque"), "{}", h);
}

#[test]
fn le_pour_tout_refuse_une_inegalite_fausse() {
    let h = rend("<Montre>par élément quelconque réel $x$, $x >= 0$");
    assert!(h.contains("ne se lit pas sur une forme canonique"), "{}", h);
}

// ═══════════ la bibliothèque des démonstrations classiques ═══════════

#[test]
fn toutes_les_fiches_rendent_sans_erreur() {
    // Une fiche qui ne rend pas est pire qu'une fiche absente : elle promet
    // une démonstration et livre un message d'erreur.
    let base: serde_json::Value =
        serde_json::from_str(include_str!("../src/maths/demonstrations.json")).unwrap();
    let fiches = base["fiches"].as_array().unwrap();
    assert!(fiches.len() >= 100);
    for f in fiches {
        for cle in f["clés"].as_array().unwrap() {
            let h = rend(&format!("<Montre>{}", cle.as_str().unwrap()));
            assert!(!h.contains("calcul-absent"), "{} : {}", f["id"], h);
            assert!(h.contains("Montrons"), "{} : {}", f["id"], h);
        }
    }
}

#[test]
fn les_cles_et_les_identifiants_sont_uniques() {
    let base: serde_json::Value =
        serde_json::from_str(include_str!("../src/maths/demonstrations.json")).unwrap();
    // Deux espaces de noms distincts : un identifiant peut ressembler à une
    // clé sans la contredire, mais deux fiches ne peuvent pas se disputer la
    // même clé — la première trouvée gagnerait, en silence.
    let mut ids = std::collections::HashSet::new();
    let mut cles = std::collections::HashSet::new();
    for f in base["fiches"].as_array().unwrap() {
        assert!(ids.insert(f["id"].as_str().unwrap().to_string()), "identifiant en double");
        for cle in f["clés"].as_array().unwrap() {
            assert!(cles.insert(cle.as_str().unwrap().to_string()), "clé en double : {}", cle);
        }
    }
}

#[test]
fn les_neuf_raisonnements_sont_representes() {
    // La base sert aussi d'anthologie : chaque raisonnement doit y trouver au
    // moins un exemple, sans quoi une forme du langage n'est illustrée nulle
    // part.
    let base: serde_json::Value =
        serde_json::from_str(include_str!("../src/maths/demonstrations.json")).unwrap();
    let mut vus = std::collections::HashSet::new();
    for f in base["fiches"].as_array().unwrap() {
        vus.insert(f["raisonnement"].as_str().unwrap().to_string());
    }
    for r in [
        "",
        "par contraposée",
        "par l'absurde",
        "par récurrence",
        "par disjonction de cas",
        "par analyse-synthèse",
        "par double inclusion",
        "par le principe des tiroirs",
        "existence et unicité",
    ] {
        assert!(vus.contains(r), "aucune fiche pour « {} »", r);
    }
}

#[test]
fn la_bibliotheque_repond_a_un_enonce_reformule() {
    // L'énoncé se compare après normalisation : accents, mathématiques et
    // ponctuation ne doivent pas empêcher la reconnaissance.
    let h = rend("<Montre>que $racine(2)$ est irrationnel.");
    assert!(h.contains("Raisonnons par l'absurde"), "{}", h);
    assert!(h.contains("irréductible"), "{}", h);
}

#[test]
fn un_raisonnement_en_desaccord_avec_la_fiche_est_refuse() {
    // Passer outre en silence produirait une démonstration amputée : le corps
    // d'une preuve directe n'a pas les étapes d'un raisonnement par l'absurde.
    let h = rend("<Montre>par l'absurde que la somme de deux rationnels est rationnelle");
    assert!(h.contains("cet énoncé se démontre directement"), "{}", h);
    let h = rend("<Montre>par récurrence que $racine(2)$ est irrationnel");
    assert!(h.contains("se démontre par l'absurde"), "{}", h);
}

#[test]
fn le_texte_libre_precede_l_etape_qu_il_introduit() {
    // Poser la suite avant de supposer l'absurde : l'ordre d'écriture est
    // l'ordre de lecture.
    let h = rend("<Montre>que la limite d'une suite est unique");
    let pose = h.find("une suite convergente").unwrap();
    let absurde = h.find("Raisonnons par l'absurde").unwrap();
    assert!(pose < absurde, "{}", h);
}

#[test]
fn un_enonce_absent_propose_les_plus_proches() {
    let h = rend("<Montre>que toute suite bornée est convergente");
    assert!(h.contains("ni calculable ni en bibliothèque"), "{}", h);
    assert!(h.contains("suite convergente"), "{}", h);
}

#[test]
fn chaque_fiche_porte_les_etapes_de_son_raisonnement() {
    // Une fiche qui annonce une récurrence sans initialisation ne rend qu'un
    // message d'erreur. Le contrôle vaut pour toutes les clés, non pour la
    // première seulement — c'est par là qu'une incohérence était passée.
    let base: serde_json::Value =
        serde_json::from_str(include_str!("../src/maths/demonstrations.json")).unwrap();
    let exige: &[(&str, &[&str])] = &[
        ("par récurrence", &["initialisation", "hérédité"]),
        ("par contraposée", &["contraposée"]),
        ("par l'absurde", &["absurde"]),
        ("par analyse-synthèse", &["analyse", "synthèse"]),
        ("par double inclusion", &["directe", "réciproque"]),
        ("par élément quelconque", &["soit"]),
        ("par le principe des tiroirs", &["objets", "tiroirs"]),
        ("existence et unicité", &["existence", "unicité"]),
    ];
    for f in base["fiches"].as_array().unwrap() {
        let r = f["raisonnement"].as_str().unwrap();
        let corps = f["corps"].as_str().unwrap();
        if let Some((_, etapes)) = exige.iter().find(|(nom, _)| *nom == r) {
            for e in *etapes {
                assert!(
                    corps.lines().any(|l| {
                        let t = l.trim_start();
                        t.starts_with(e) && t.trim_end().ends_with('{')
                    }),
                    "{} annonce « {} » sans l'étape « {} »",
                    f["id"], r, e
                );
            }
        }
    }
}

#[test]
fn aucune_cle_ne_contient_d_accolade() {
    // L'accolade ouvre un corps en docdg : une clé qui en porte une ne peut
    // pas être écrite après le verbe, la fiche serait inatteignable.
    let base: serde_json::Value =
        serde_json::from_str(include_str!("../src/maths/demonstrations.json")).unwrap();
    for f in base["fiches"].as_array().unwrap() {
        for cle in f["clés"].as_array().unwrap() {
            let c = cle.as_str().unwrap();
            assert!(!c.contains('{') && !c.contains('}'), "{} : clé « {} »", f["id"], c);
        }
    }
}
