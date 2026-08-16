//! Des tests de contenu, pas seulement d'absence d'erreur : deux fautes ont
//! passé l'audit parce qu'un rendu peut être faux sans être en erreur —
//! « Décompose 4 782 » répondait par une décomposition en éléments simples,
//! la rosace à 4 pétales en dessinait 8. Ici, on vérifie ce qui est écrit.

use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    Engine::new().render(src, false).html
}

#[test]
fn la_decomposition_est_positionnelle() {
    let h = rend("<Décompose>4 782");
    for attendu in [
        "(4 \\times 1\\,000)",
        "(7 \\times 100)",
        "(8 \\times 10)",
        "+ 2",
    ] {
        assert!(h.contains(attendu), "« {} » attendu dans : {}", attendu, h);
    }
    // les rangs à zéro s'omettent, les unités s'écrivent seules
    let h = rend("<Décompose>708");
    assert!(h.contains("(7 \\times 100)") && h.contains("+ 8"), "{}", h);
    assert!(!h.contains("0 \\times"), "un rang à zéro s'est écrit : {}", h);
    // Un terme seul n'a rien à grouper : pas de parenthèses.
    let h = rend("<Décompose>50");
    assert!(h.contains("5 \\times 10") && !h.contains("("), "{}", h);
    // la décomposition en éléments simples reste au calcul formel
    let h = rend("<Décompose>en éléments simples (3x + 5)/(x^2 - 1)");
    assert!(!h.contains("positionnelle") && !h.contains("\\times 10"), "{}", h);
}

#[test]
fn la_rosace_a_le_bon_nombre_de_petales() {
    // r = cos(k·t) : k pétales si k impair, 2k si k pair — la rosace à
    // 4 pétales doit donc tracer cos(2t), et celle à 5, cos(5t).
    let quatre = rend("<Trace>la rosace à 4 pétales");
    let cos2 = rend("<Trace>la courbe polaire r = cos(2*t)");
    assert_eq!(quatre, cos2, "4 pétales doivent tracer cos(2t)");
    let cinq = rend("<Trace>la rosace à 5 pétales");
    let cos5 = rend("<Trace>la courbe polaire r = cos(5*t)");
    assert_eq!(cinq, cos5, "5 pétales doivent tracer cos(5t)");
    assert_ne!(quatre, rend("<Trace>la courbe polaire r = cos(4*t)"),
        "cos(4t) dessinerait 8 pétales");
}
