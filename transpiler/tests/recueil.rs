//! Le recueil « Le langage naturel des mathématiques » fait loi : ces tests
//! vérifient que le rendu du moteur, les exemples et la bibliothèque des
//! démonstrations respectent ses règles vérifiables — symboles logiques hors
//! de la prose (§2.3, §8), accord de « Soient » (§9), nom puis nature (§2.2),
//! déclarations sans « tel que » parasite (§8).
use docdg_transpiler::Engine;

/// La prose en ligne \( … \) d'un HTML, concaténée — les formules hors texte
/// \[ … \] et les tableaux sont tolérés par le recueil (§2.3).
fn prose_et_inline(h: &str) -> String {
    let mut out = String::new();
    let mut reste = h;
    // retirer les tableaux
    let mut nettoye = String::new();
    while let Some(i) = reste.find("<table") {
        nettoye.push_str(&reste[..i]);
        match reste[i..].find("</table>") {
            Some(j) => reste = &reste[i + j + 8..],
            None => { reste = ""; break; }
        }
    }
    nettoye.push_str(reste);
    // retirer les hors-texte \[ … \]
    let mut r: &str = &nettoye;
    while let Some(i) = r.find("\\[") {
        out.push_str(&r[..i]);
        match r[i..].find("\\]") {
            Some(j) => r = &r[i + j + 2..],
            None => { r = ""; break; }
        }
    }
    out.push_str(r);
    out
}

#[test]
fn aucun_symbole_logique_dans_la_prose() {
    // §2.3 et §8 : ∀, ∃, ⇒, ⇔ sont proscrits dans le fil du texte.
    let mut fichiers: Vec<_> = std::fs::read_dir("../exemples").unwrap()
        .filter_map(|e| e.ok()).map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false)).collect();
    fichiers.sort();
    let interdits = ["\\forall", "\\exists", "\\Rightarrow", "\\Leftrightarrow",
                     "\\implies", "\\iff", "∀", "∃", "⇒", "⇔"];
    // Le critère exact du recueil : la faute est le MÉLANGE, dans une même
    // phrase, de mots français et de symboles logiques. Une formule seule
    // dans son paragraphe — un cadre de notations, un récapitulatif — est
    // une formule isolée, tolérée.
    let melange = |h: &str| -> bool {
        for p in h.split("<p>").skip(1) {
            let p = p.split("</p>").next().unwrap_or("");
            let mut hors_math = String::new();
            let mut reste = p;
            let mut a_symbole = false;
            while let Some(i) = reste.find("\\(") {
                hors_math.push_str(&reste[..i]);
                match reste[i..].find("\\)") {
                    Some(j) => {
                        let formule = &reste[i..i + j];
                        if interdits.iter().any(|s| formule.contains(s)) {
                            a_symbole = true;
                        }
                        reste = &reste[i + j + 2..];
                    }
                    None => { reste = ""; break; }
                }
            }
            hors_math.push_str(reste);
            // épurer les balises : leurs attributs ne sont pas des mots
            let mut texte = String::new();
            let mut dans_balise = false;
            for c in hors_math.chars() {
                match c {
                    '<' => dans_balise = true,
                    '>' => { dans_balise = false; texte.push(' ') }
                    c if !dans_balise => texte.push(c),
                    _ => {}
                }
            }
            let a_mots = texte.chars().filter(|c| c.is_alphabetic()).count() > 2;
            if a_symbole && a_mots {
                return true;
            }
        }
        false
    };
    let mut fautes = Vec::new();
    for f in &fichiers {
        let src = std::fs::read_to_string(f).unwrap();
        let h = Engine::new().render(&src, false).html;
        let h = prose_et_inline(&h);
        if melange(&h) {
            fautes.push(format!("{} : mélange mots + symbole logique", f.file_name().unwrap().to_string_lossy()));
        }
    }
    // la bibliothèque des démonstrations, fiche par fiche
    let base: serde_json::Value =
        serde_json::from_str(include_str!("../src/maths/demonstrations.json")).unwrap();
    for fiche in base["fiches"].as_array().unwrap() {
        let cle = fiche["clés"][0].as_str().unwrap();
        let h = Engine::new().render(&format!("<Montre>{}", cle), false).html;
        let h = prose_et_inline(&h);
        if melange(&h) {
            fautes.push(format!("fiche {} : mélange mots + symbole logique", fiche["id"]));
        }
    }
    assert!(fautes.is_empty(), "symboles proscrits en prose :\n{}", fautes.join("\n"));
}

fn rendu_texte(src: &str) -> String {
    let h = Engine::new().render(src, false).html;
    let mut o = String::new(); let mut d = false;
    for c in h.chars() {
        match c { '<' => d = true, '>' => { d = false; o.push(' ') }, c if !d => o.push(c), _ => {} }
    }
    o.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn les_phrases_types_du_recueil() {
    for (src, attendu) in [
        // §9 : l'accord du pluriel
        ("<Soit>les points A(1;2) et B(3;4)", "Soient"),
        // §3.2 : la suite définie par récurrence, en phrase
        ("<Soit>la suite u définie par u(0) = 1 et u(n+1) = 2u(n) + 1", "la suite définie par"),
        // §2.2 : le nom, puis la nature
        ("<Soit>une fonction f(x) = x^2", "la fonction définie par"),
        ("<Soit>un vecteur u(3;-2)", "le vecteur de coordonnées"),
    ] {
        let t = rendu_texte(src);
        assert!(t.contains(attendu), "« {} » attendu dans : {}", attendu, t);
    }
}

#[test]
fn pas_de_tel_que_redondant_dans_les_declarations() {
    // §8, premier piège : la propriété fait partie de la déclaration.
    for src in ["<Soit>une fonction f(x) = x^2",
                "<Soit>les points A(1;2) et B(3;4)",
                "<Soit>un vecteur u(3;-2)",
                "<Soit>la suite u définie par u(0) = 1 et u(n+1) = 2u(n) + 1"] {
        let t = rendu_texte(src);
        assert!(!t.contains("tel que") || src.contains("tel que"), "« tel que » parasite : {}", t);
    }
}
