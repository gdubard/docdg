pub fn color(name: &str) -> Option<&'static str> {
    let n = name.trim().to_lowercase();
    let c = match n.as_str() {
        "bleu marine" | "bleue marine" => "navy",
        "bleu alice" => "aliceblue",
        "bleu canard" => "#0e7a80",
        "bleu lavande" => "lavender",
        "bleu indigo" => "indigo",
        "bleu ciel" => "skyblue",
        "bleu roi" => "royalblue",
        "bleu acier" => "steelblue",
        "bleu dodger" => "dodgerblue",
        "bleu nuit" => "midnightblue",
        "bleu" | "bleue" => "#1a5fb4",
        "vert menthe" | "verte menthe" => "#dff2e4",
        "vert honeydew" => "honeydew",
        "vert foncé" | "verte foncée" => "darkgreen",
        "vert forêt" => "forestgreen",
        "vert olive" => "olive",
        "vert" | "verte" => "#26a269",
        "rose brumeux" => "mistyrose",
        "rose" => "pink",
        "cramoisi" | "cramoisie" => "crimson",
        "vieille dentelle" => "oldlace",
        "orange foncé" | "orange foncée" => "darkorange",
        "orange" => "orange",
        "blanc fumé" => "whitesmoke",
        "blanc" | "blanche" => "white",
        "gris anthracite" | "grise anthracite" => "#36454f",
        "gris perle" | "grise perle" => "#e8e8e6",
        "gris clair" => "#d3d3d3",
        "gris" | "grise" => "gray",
        "pourpre" => "purple",
        "violet" | "violette" => "#8f3fbf",
        "rouge tomate" => "tomato",
        "rouge brique" => "firebrick",
        "rouge" => "#c01c28",
        "noir" | "noire" => "black",
        "jaune" => "#e5a50a",
        "or" | "doré" | "dorée" => "goldenrod",
        "marron" => "#8b5a2b",
        "turquoise" => "turquoise",
        "corail" => "coral",
        "saumon" => "salmon",
        "beige" => "beige",
        "ivoire" => "ivory",
        _ => return None,
    };
    Some(c)
}

pub fn parse_color_at(words: &[&str]) -> Option<(&'static str, usize)> {
    for n in (1..=3.min(words.len())).rev() {
        let joined = words[..n].join(" ");
        if let Some(c) = color(&joined) {
            return Some((c, n));
        }
    }
    None
}
