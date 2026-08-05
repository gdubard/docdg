pub(crate) fn apres_cle<'a>(source: &'a str, cle: &str) -> Option<&'a str> {
    let bas = source.to_ascii_lowercase();
    bas.find(cle).map(|i| &source[i + cle.len()..])
}

pub(crate) fn nom_apres(source: &str, cle: &str) -> String {
    match apres_cle(source, cle) {
        Some(r) => r
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphanumeric())
            .collect(),
        None => String::new(),
    }
}

pub(crate) fn pli(c: char) -> char {
    match c {
        'à' | 'â' | 'ä' => 'a',
        'é' | 'è' | 'ê' | 'ë' => 'e',
        'î' | 'ï' => 'i',
        'ô' | 'ö' => 'o',
        'ù' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        'À' | 'Â' | 'Ä' => 'A',
        'É' | 'È' | 'Ê' | 'Ë' => 'E',
        'Î' | 'Ï' => 'I',
        'Ô' | 'Ö' => 'O',
        'Ù' | 'Û' | 'Ü' => 'U',
        'Ç' => 'C',
        _ => c,
    }
}

pub(crate) fn meme_mot(a: &str, b: &str) -> bool {
    let mut x = a.chars().map(pli);
    let mut y = b.chars().map(pli);
    loop {
        match (x.next(), y.next()) {
            (None, None) => return true,
            (Some(p), Some(q)) if p == q => continue,
            _ => return false,
        }
    }
}

pub(crate) fn maj_profondeur(ligne: &str, profondeur: &mut i32) {
    for o in ligne.as_bytes() {
        match o {
            b'{' => *profondeur += 1,
            b'}' => *profondeur -= 1,
            _ => {}
        }
    }
}

pub(crate) fn termes_signes(s: &str) -> Vec<(i128, String)> {
    let mut termes = Vec::new();
    let mut signe = 1i128;
    let mut cur = String::new();
    for c in s.trim().chars() {
        match c {
            '+' | '-' if !cur.trim().is_empty() => {
                termes.push((signe, cur.trim().to_string()));
                signe = if c == '-' { -1 } else { 1 };
                cur.clear();
            }
            '-' if cur.trim().is_empty() => signe = -signe,
            '+' if cur.trim().is_empty() => {}
            c => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        termes.push((signe, cur.trim().to_string()));
    }
    termes
}

pub(crate) fn noms_separes(s: &str) -> Vec<String> {
    s.replace(" et ", ",")
        .split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}
