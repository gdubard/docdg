fn echappe(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

pub fn bloc(source: &str, message: &str) -> String {
    let source = source.trim();
    if source.is_empty() {
        return format!("<div class=\"calcul-absent\">⚠ {}</div>", echappe(message));
    }
    format!(
        "<div class=\"calcul-absent\">⚠ {} — {}</div>",
        echappe(source),
        echappe(message)
    )
}

pub fn source(ligne: &str, message: &str) -> String {
    format!("<rouge gras>{{⚠ {} — {}}}\n", ligne.trim(), message)
}

pub fn en_ligne(message: &str) -> String {
    format!("<span class=\"calcul-absent\">⚠ {}</span>", echappe(message))
}
