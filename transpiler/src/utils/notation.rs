fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '\'' || c == 'é' || c == 'è' || c == 'à' || c == 'ç' || c == '_'
}

fn find_call(s: &str, name: &str) -> Option<(usize, usize, usize)> {
    let mut from = 0;
    while let Some(p) = s[from..].find(name) {
        let i = from + p;
        let before_ok = i == 0 || !is_word(s[..i].chars().last().unwrap());
        let after = i + name.len();
        if before_ok && s[after..].starts_with('(') {
            let mut depth = 0i32;
            for (j, c) in s[after..].char_indices() {
                if c == '(' {
                    depth += 1;
                }
                if c == ')' {
                    depth -= 1;
                    if depth == 0 {
                        return Some((i, after + 1, after + j));
                    }
                }
            }
            return None;
        }
        from = i + name.len().max(1);
    }
    None
}

fn wrap_fn(s: &str, name: &str, f: &dyn Fn(&str) -> String) -> String {
    let mut out = s.to_string();
    loop {
        match find_call(&out, name) {
            Some((start, a0, a1)) => {
                let arg = to_latex_inner(&out[a0..a1]);
                let rep = f(&arg);
                let end = a1 + 1;
                out = format!("{}{}{}", &out[..start], rep, &out[end..]);
            }
            None => break,
        }
    }
    out
}

fn phrase_replace(s: &str, from: &str, to: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    loop {
        match rest.find(from) {
            Some(i) => {

                let precedent = if i > 0 {
                    rest[..i].chars().last()
                } else {
                    out.chars().last()
                };
                let before_ok = precedent.map(|c| !is_word(c) && c != '\\').unwrap_or(true);
                let after = &rest[i + from.len()..];
                let after_ok = after.chars().next().map(|c| !is_word(c)).unwrap_or(true);
                out.push_str(&rest[..i]);
                if before_ok && after_ok {
                    out.push_str(to);
                } else {
                    out.push_str(from);
                }
                rest = after;
            }
            None => {
                out.push_str(rest);
                return out;
            }
        }
    }
}

fn liste_virgules(a: &str) -> String {
    a.split(';')
        .map(|m| m.trim())
        .collect::<Vec<_>>()
        .join("\\,,\\,")
}

fn liste_points_virgules(a: &str) -> String {
    a.split(';')
        .map(|m| m.trim())
        .collect::<Vec<_>>()
        .join("\\,;\\,")
}

fn repere_tex(a: &str) -> String {
    let parts: Vec<&str> = a.split(';').map(|m| m.trim()).collect();
    let mut sortie = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        if i == 0 {
            sortie.push(p.to_string());
        } else {
            sortie.push(format!("\\vec{{{}}}", p));
        }
    }
    format!("\\left({}\\right)", sortie.join("\\,,\\,"))
}

fn composantes(s: &str) -> String {
    let mut out = String::new();
    let mut reste = s;
    loop {
        let (cle, colonne) = match (reste.find("vecteur colonne "), reste.find("vecteur ")) {
            (Some(i), Some(j)) if i <= j => (i, true),
            (_, Some(j)) => (j, false),
            _ => {
                out.push_str(reste);
                return out;
            }
        };
        let mot = if colonne { "vecteur colonne " } else { "vecteur " };
        let apres = &reste[cle + mot.len()..];
        let nom: String = apres.chars().take_while(|c| is_word(*c)).collect();
        let suite = &apres[nom.len()..];
        if nom.is_empty() || !suite.starts_with('(') {
            out.push_str(&reste[..cle + mot.len()]);
            reste = apres;
            continue;
        }
        let fin = match suite.find(')') {
            Some(f) => f,
            None => {
                out.push_str(reste);
                return out;
            }
        };
        let comps: Vec<&str> = suite[1..fin].split(&[';', ','][..]).map(|c| c.trim()).collect();
        out.push_str(&reste[..cle]);
        if colonne {
            out.push_str(&format!(
                "\\vec{{{}}}\\begin{{pmatrix}}{}\\end{{pmatrix}}",
                nom,
                comps.join("\\\\")
            ));
        } else {
            out.push_str(&format!(
                "\\vec{{{}}}\\left({}\\right)",
                nom,
                comps.join("\\ ;\\ ")
            ));
        }
        reste = &suite[fin + 1..];
    }
}

fn simple_arg(a: &str) -> bool {
    a.chars().all(|c| c.is_alphanumeric())
}

fn coupe_au_egal(s: &str) -> (String, String) {
    let c: Vec<char> = s.chars().collect();
    let mut depth = 0i32;
    for i in 0..c.len() {
        match c[i] {
            '(' | '{' | '[' => depth += 1,
            ')' | '}' | ']' => depth -= 1,
            '=' if depth <= 0 => {
                let (a, b): (String, String) = (c[..i].iter().collect(), c[i..].iter().collect());
                return (a.trim().to_string(), b);
            }
            _ => {}
        }
    }
    (s.trim().to_string(), String::new())
}

fn integrales(s: &str) -> String {
    let mut out = s.to_string();
    loop {
        let (start, a0, a1) = match find_call(&out, "intégrale").or_else(|| find_call(&out, "integrale")) {
            Some(t) => t,
            None => break,
        };
        let spec = out[a0..a1].to_string();
        let (corps, suite) = coupe_au_egal(&out[a1 + 1..]);
        let mut var = "x".to_string();
        let bornes: Vec<&str> = spec.splitn(2, ';').collect();
        let bas = match bornes[0].split_once('=') {
            Some((v, b)) => {
                var = v.trim().to_string();
                b.trim().to_string()
            }
            None => bornes[0].trim().to_string(),
        };
        let tete = if bornes.len() == 2 {
            format!("\\int_{{{}}}^{{{}}}", bas, bornes[1].trim())
        } else if bas.is_empty() {
            "\\int".to_string()
        } else {
            format!("\\int_{{{}}}", bas)
        };
        out = format!(
            "{}{} {}\\,\\mathrm{{d}}{} {}",
            &out[..start],
            tete,
            corps,
            var,
            suite
        );
    }
    out
}

fn limites(s: &str) -> String {
    let mut out = s.to_string();
    while let Some((start, a0, a1)) = find_call(&out, "lim") {
        let spec = out[a0..a1].to_string();
        let (corps, suite) = coupe_au_egal(&out[a1 + 1..]);
        out = format!(
            "{}\\lim\\limits_{{{}}} {} {}",
            &out[..start],
            spec,
            corps,
            suite
        );
    }
    out
}

fn differentielles(s: &str) -> String {
    let mut out = s.to_string();
    for (mot, tete) in [
        ("dd", "\\mathrm{d}"),
        ("partielle", "\\partial "),
        ("partiel", "\\partial "),
    ] {
        let mut from = 0usize;
        loop {
            let (start, a0, a1) = match find_call(&out[from..], mot) {
                Some((s0, b0, b1)) => (from + s0, from + b0, from + b1),
                None => break,
            };
            let haut = out[a0..a1].trim().to_string();
            let apres = a1 + 1;
            let reste = &out[apres..];
            let sans_espace = reste.trim_start();
            let barre = apres + (reste.len() - sans_espace.len());
            let mut remplacement = format!("{}{}", tete, haut);
            let mut fin = apres;
            if let Some(den) = sans_espace.strip_prefix('/') {
                let sans_espace2 = den.trim_start();
                let debut = barre + 1 + (den.len() - sans_espace2.len());
                if let Some((0, b0, b1)) = find_call(sans_espace2, mot) {
                    let bas = sans_espace2[b0..b1].trim();
                    remplacement =
                        format!("\\dfrac{{{}{}}}{{{}{}}}", tete, haut, tete, bas);
                    fin = debut + b1 + 1;
                }
            }
            out = format!("{}{}{}", &out[..start], remplacement, &out[fin..]);
            from = start + remplacement.len();
        }
    }
    out
}

const FONCTIONS: &[&str] = &[
    "arccos", "arcsin", "arctan", "argch", "argsh", "argth", "cosh", "sinh", "tanh", "cos", "sin",
    "tan", "exp", "ln", "log", "sh", "ch", "th",
];

const GRECQUES: &[&str] = &[
    "alpha", "beta", "gamma", "delta", "epsilon", "zeta", "eta", "theta", "iota", "kappa",
    "lambda", "mu", "nu", "xi", "rho", "sigma", "tau", "phi", "chi", "psi", "omega", "varphi",
    "pi", "Gamma", "Delta", "Theta", "Lambda", "Xi", "Sigma", "Phi", "Psi", "Omega",
];

fn to_latex_inner(src: &str) -> String {
    let mut s = differentielles(src);
    s = integrales(&s);
    s = limites(&s);
    s = wrap_fn(&s, "intervalle entier", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!("[\\![{},\\,{}]\\!]", parts[0].trim(), parts[1].trim())
        } else {
            format!("[\\![{}]\\!]", a.trim())
        }
    });
    s = wrap_fn(&s, "parties de", &|a| format!("\\mathcal{{P}}({})", a.trim()));
    s = wrap_fn(&s, "abs", &|a| format!("\\lvert {}\\rvert", a));
    s = wrap_fn(&s, "norme", &|a| format!("\\lVert {}\\rVert", a));
    s = composantes(&s);
    s = wrap_fn(&s, "vecteur", &|a| format!("\\vec{{{}}}", a.trim()));
    s = wrap_fn(&s, "produit scalaire", &|a| {
        format!("\\left\\langle {}\\right\\rangle", liste_virgules(a))
    });
    s = wrap_fn(&s, "produit mixte", &|a| {
        format!("\\left[{}\\right]", liste_virgules(a))
    });
    s = wrap_fn(&s, "projeté orthogonal", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!("p_{{{}}}\\left({}\\right)", parts[0].trim(), parts[1].trim())
        } else {
            format!("p\\left({}\\right)", a.trim())
        }
    });
    s = wrap_fn(&s, "orthogonal", &|a| format!("{}^{{\\perp}}", a.trim()));
    s = wrap_fn(&s, "valeur absolue", &|a| format!("\\left|{}\\right|", a.trim()));
    s = wrap_fn(&s, "distance", &|a| {
        format!("d\\left({}\\right)", liste_virgules(a))
    });
    s = wrap_fn(&s, "milieu", &|a| {
        format!("I_{{{}}}", a.split(';').map(|m| m.trim()).collect::<Vec<_>>().join(""))
    });
    s = wrap_fn(&s, "colinéaires", &|a| {
        let parts: Vec<&str> = a.split(';').map(|m| m.trim()).collect();
        if parts.len() == 2 {
            format!("\\vec{{{}}} \\mathbin{{/\\!/}} \\vec{{{}}}", parts[0], parts[1])
        } else {
            a.to_string()
        }
    });
    s = wrap_fn(&s, "repère orthonormé", &repere_tex);
    s = wrap_fn(&s, "repère", &repere_tex);
    s = wrap_fn(&s, "triangle", &|a| format!("\\triangle {}", a.trim()));
    s = wrap_fn(&s, "arc", &|a| format!("\\overset{{\\frown}}{{{}}}", a.trim()));
    s = wrap_fn(&s, "cercle", &|a| {
        format!("\\mathcal{{C}}\\left({}\\right)", liste_points_virgules(a))
    });
    s = wrap_fn(&s, "angle", &|a| format!("\\widehat{{{}}}", a.trim()));
    s = phrase_replace(&s, "angle droit", "\\llcorner");
    s = s.replace('\u{b0}', "^\\circ");
    s = phrase_replace(&s, "sachant", "\\mid ");
    s = phrase_replace(&s, "!parallèle", "\\nparallel");
    s = phrase_replace(&s, "parallèle", "\\mathbin{/\\!/}");
    s = phrase_replace(&s, "perpendiculaire", "\\perp");
    s = phrase_replace(&s, "isométrique", "\\cong");
    s = phrase_replace(&s, "semblable", "\\sim");
    s = s.replace("} . \\vec{", "} \\cdot \\vec{");
    s = s.replace("} ^ \\vec{", "} \\wedge \\vec{");
    s = wrap_fn(&s, "C", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!("\\binom{{{}}}{{{}}}", parts[0].trim(), parts[1].trim())
        } else {
            format!("C\\left({}\\right)", a.trim())
        }
    });
    s = wrap_fn(&s, "A", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!("A_{{{}}}^{{{}}}", parts[0].trim(), parts[1].trim())
        } else {
            format!("A\\left({}\\right)", a.trim())
        }
    });
    s = wrap_fn(&s, "variance", &|a| {
        format!("\\operatorname{{Var}}({})", a.trim())
    });
    s = wrap_fn(&s, "covariance", &|a| {
        format!("\\operatorname{{Cov}}\\left({}\\right)", liste_virgules(a))
    });
    s = wrap_fn(&s, "écart type", &|a| format!("\\sigma({})", a.trim()));
    s = wrap_fn(&s, "répartition de", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!("F_{{{}}}\\left({}\\right)", parts[0].trim(), parts[1].trim())
        } else {
            format!("F\\left({}\\right)", a.trim())
        }
    });
    s = wrap_fn(&s, "densité de", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!("f_{{{}}}\\left({}\\right)", parts[0].trim(), parts[1].trim())
        } else {
            format!("f\\left({}\\right)", a.trim())
        }
    });
    s = wrap_fn(&s, "normal", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!(
                "\\mathcal{{N}}\\left({}\\,,\\,{}^2\\right)",
                parts[0].trim(),
                parts[1].trim()
            )
        } else {
            format!("\\mathcal{{N}}\\left({}\\right)", a.trim())
        }
    });
    s = wrap_fn(&s, "binomiale", &|a| {
        format!("\\mathcal{{B}}\\left({}\\right)", liste_virgules(a))
    });
    s = wrap_fn(&s, "poisson", &|a| {
        format!("\\mathcal{{P}}\\left({}\\right)", liste_virgules(a))
    });
    s = wrap_fn(&s, "exponentielle", &|a| {
        format!("\\mathcal{{E}}\\left({}\\right)", liste_virgules(a))
    });
    s = wrap_fn(&s, "uniforme", &|a| {
        format!("\\mathcal{{U}}\\left({}\\right)", liste_virgules(a))
    });
    s = wrap_fn(&s, "racine", &|a| format!("\\sqrt{{{}}}", a));
    s = wrap_fn(&s, "conjugué", &|a| format!("\\overline{{{}}}", a));
    s = wrap_fn(&s, "moyenne", &|a| format!("\\overline{{{}}}", a));
    s = wrap_fn(&s, "adhérence", &|a| format!("\\overline{{{}}}", a));
    s = wrap_fn(&s, "non", &|a| format!("\\overline{{{}}}", a));
    s = wrap_fn(&s, "défaut", &|a| format!("\\lfloor {}\\rfloor", a));
    s = wrap_fn(&s, "excès", &|a| format!("\\lceil {}\\rceil", a));
    s = wrap_fn(&s, "card", &|a| format!("\\operatorname{{Card}}({})", a.trim()));
    s = wrap_fn(&s, "signe", &|a| format!("\\operatorname{{sgn}}({})", a.trim()));
    s = wrap_fn(&s, "engendré par", &|a| {
        format!("\\operatorname{{Vect}}({})", a.trim())
    });
    s = wrap_fn(&s, "déterminant", &|a| format!("\\det({})", a.trim()));
    s = wrap_fn(&s, "comatrice", &|a| format!("\\operatorname{{Com}}({})", a.trim()));
    s = wrap_fn(&s, "spectre", &|a| format!("\\operatorname{{Sp}}({})", a.trim()));
    s = wrap_fn(&s, "noyau", &|a| format!("\\operatorname{{Ker}}({})", a.trim()));
    s = wrap_fn(&s, "image", &|a| format!("\\operatorname{{Im}}({})", a.trim()));
    s = wrap_fn(&s, "trace", &|a| format!("\\operatorname{{Tr}}({})", a.trim()));
    s = wrap_fn(&s, "rang", &|a| format!("\\operatorname{{rg}}({})", a.trim()));
    s = wrap_fn(&s, "factorielle", &|a| {
        if simple_arg(a.trim()) {
            format!("{}!", a.trim())
        } else {
            format!("\\left({}\\right)!", a.trim())
        }
    });
    s = wrap_fn(&s, "somme", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!("\\sum_{{{}}}^{{{}}}", parts[0].trim(), parts[1].trim())
        } else {
            format!("\\sum_{{{}}}", a.trim())
        }
    });
    for (mot, symbole) in [
        ("produit", "\\prod"),
        ("réunion de", "\\bigcup"),
        ("reunion de", "\\bigcup"),
        ("intersection de", "\\bigcap"),
    ] {
        s = wrap_fn(&s, mot, &|a| {
            let parts: Vec<&str> = a.splitn(2, ';').collect();
            if parts.len() == 2 {
                format!("{}_{{{}}}^{{{}}}", symbole, parts[0].trim(), parts[1].trim())
            } else {
                format!("{}_{{{}}}", symbole, a.trim())
            }
        });
    }
    s = wrap_fn(&s, "dérivée directionnelle", &|a| {
        let parts: Vec<&str> = a.splitn(2, ';').collect();
        if parts.len() == 2 {
            format!("\\nabla_{{{}}} {}", parts[1].trim(), parts[0].trim())
        } else {
            format!("\\nabla {}", a.trim())
        }
    });
    s = wrap_fn(&s, "gradient", &|a| {
        format!("\\operatorname{{grad}}({})", a.trim())
    });
    s = wrap_fn(&s, "divergence", &|a| {
        format!("\\operatorname{{div}}({})", a.trim())
    });
    s = wrap_fn(&s, "rotationnel", &|a| {
        format!("\\operatorname{{rot}}({})", a.trim())
    });
    s = wrap_fn(&s, "laplacien", &|a| format!("\\Delta {}", a.trim()));
    for (mot, symbole) in [
        ("la transformée de Laplace", "\\mathcal{L}"),
        ("la transformée de Fourier", "\\mathcal{F}"),
        ("l'inverse de Laplace", "\\mathcal{L}^{-1}"),
        ("l'inverse de Fourier", "\\mathcal{F}^{-1}"),
    ] {
        s = wrap_fn(&s, mot, &|a| {
            format!("{}\\left\\{{{}\\right\\}}", symbole, a.trim())
        });
    }
    for (from, to) in [
        ("il n'existe pas de", "\\nexists\\, "),
        ("n'appartient pas à", "\\notin "),
        ("il existe", "\\exists\\, "),
        ("pour tout", "\\forall\\, "),
        ("inclus dans ou égal à", "\\subseteq "),
        ("inclus dans", "\\subset "),
        ("privé de", "\\smallsetminus "),
        ("négation de", "\\neg "),
        ("congru à", "\\equiv "),
        ("modulo", "\\ \\mathrm{mod}\\ "),
        ("union", "\\cup "),
        ("inter", "\\cap "),
        ("dans", "\\in "),
        ("vide", "\\varnothing "),
        ("et", "\\wedge "),
        ("ou", "\\vee "),
    ] {
        s = phrase_replace(&s, from, to);
    }
    s = s
        .replace("<=>", "\u{E020}")
        .replace("=>", "\u{E021}")
        .replace("<=", "\u{E022}")
        .replace(">=", "\u{E023}")
        .replace("+-", "\u{E024}")
        .replace("!=", "\u{E025}")
        .replace("^*", "\u{E026}")
        .replace('\u{E020}', "\\Leftrightarrow ")
        .replace('\u{E021}', "\\Rightarrow ")
        .replace('\u{E022}', "\\leqslant ")
        .replace('\u{E023}', "\\geqslant ")
        .replace('\u{E024}', "\\pm ")
        .replace('\u{E025}', "\\neq ")
        .replace('*', "\\times ")
        .replace('\u{E026}', "^{*}");
    for bb in ["NN", "ZZ", "DD", "QQ", "RR", "CC", "PP", "EE", "KK", "HH"] {
        let to = format!("\\mathbb{{{}}}", &bb[..1]);
        s = phrase_replace(&s, bb, &to);
    }
    s = s.replace("->", "\u{E027}").replace("...", "\\dots ");
    for (mot, tex) in [
        ("infini", "\\infty "),
        ("to", "\u{E027}"),
        ("partielle", "\\partial "),
        ("partiel", "\\partial "),
    ] {
        s = phrase_replace(&s, mot, tex);
    }
    for f in FONCTIONS {
        s = phrase_replace(&s, f, &format!("\\{}", f));
    }
    for g in GRECQUES {
        s = phrase_replace(&s, g, &format!("\\{}", g));
    }
    s.replace('\u{E027}', "\\to ")
}

pub fn to_latex(src: &str) -> String {
    fractions(&to_latex_inner(src))
}

fn match_back(c: &[char], close: usize) -> usize {
    let (o, f) = if c[close] == '}' { ('{', '}') } else { ('(', ')') };
    let mut depth = 0i32;
    let mut i = close as isize;
    while i >= 0 {
        let ch = c[i as usize];
        if ch == f {
            depth += 1;
        } else if ch == o {
            depth -= 1;
            if depth == 0 {
                return i as usize;
            }
        }
        i -= 1;
    }
    0
}

fn match_fwd(c: &[char], open: usize) -> usize {
    let (o, f) = if c[open] == '{' { ('{', '}') } else { ('(', ')') };
    let mut depth = 0i32;
    for i in open..c.len() {
        let ch = c[i];
        if ch == o {
            depth += 1;
        } else if ch == f {
            depth -= 1;
            if depth == 0 {
                return i;
            }
        }
    }
    c.len() - 1
}

fn cmd_start(c: &[char], mut i: usize) -> usize {
    let start = i;
    while i > 0 && c[i - 1].is_alphabetic() {
        i -= 1;
    }
    if i > 0 && c[i - 1] == '\\' {
        return i - 1;
    }
    start
}

fn left_atom(c: &[char], before: usize) -> usize {
    let mut j = before as isize;
    while j >= 0 && c[j as usize] == ' ' {
        j -= 1;
    }
    if j < 0 {
        return 0;
    }
    loop {
        let ch = c[j as usize];
        if ch == '}' || ch == ')' {
            let mut open = match_back(c, j as usize);
            loop {
                let cs = cmd_start(c, open);
                if cs < open {
                    open = cs;
                    break;
                }
                if open > 0 && (c[open - 1] == '}' || c[open - 1] == ')') {
                    open = match_back(c, open - 1);
                    continue;
                }
                break;
            }

            while open > 0 && (c[open - 1].is_alphanumeric() || c[open - 1] == '\'') {
                open -= 1;
            }
            j = open as isize;
        } else if ch.is_alphanumeric() || ch == ',' || ch == '.' {
            while j > 0 && (c[(j - 1) as usize].is_alphanumeric() || c[(j - 1) as usize] == ',') {
                j -= 1;
            }
            j = cmd_start(c, j as usize) as isize;
        } else {
            return (j + 1) as usize;
        }
        if j > 0 && (c[(j - 1) as usize] == '^' || c[(j - 1) as usize] == '_') {
            j -= 2;
            if j < 0 {
                return 0;
            }
            continue;
        }
        return j.max(0) as usize;
    }
}

fn right_atom(c: &[char], after: usize) -> usize {
    let mut j = after;
    while j < c.len() && c[j] == ' ' {
        j += 1;
    }
    if j >= c.len() {
        return c.len() - 1;
    }
    if c[j] == '-' || c[j] == '+' {
        j += 1;
    }
    if j < c.len() && c[j] == '\\' {
        j += 1;
        while j < c.len() && c[j].is_alphabetic() {
            j += 1;
        }
        if j < c.len() && (c[j] == '{' || c[j] == '(') {
            j = match_fwd(c, j);
        } else {
            j -= 1;
        }
    } else if j < c.len() && (c[j] == '{' || c[j] == '(') {
        j = match_fwd(c, j);
    } else {
        while j + 1 < c.len() && (c[j + 1].is_alphanumeric() || c[j + 1] == ',') {
            j += 1;
        }
    }
    while j + 2 < c.len() && (c[j + 1] == '^' || c[j + 1] == '_') {
        let k = j + 2;
        if c[k] == '{' || c[k] == '(' {
            j = match_fwd(c, k);
        } else {
            j = k;
        }
    }
    j
}

fn sans_parentheses(s: &str) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.first() != Some(&'(') || c.last() != Some(&')') {
        return s.to_string();
    }
    let mut depth = 0i32;
    for (i, ch) in c.iter().enumerate() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 && i + 1 < c.len() {
                    return s.to_string();
                }
            }
            _ => {}
        }
    }
    c[1..c.len() - 1].iter().collect()
}

pub fn fractions(src: &str) -> String {
    let mut c: Vec<char> = src.chars().collect();
    let mut from = 0usize;
    loop {
        let pos = match (from..c.len()).find(|&i| c[i] == '/') {
            Some(p) => p,
            None => break,
        };
        if pos == 0 || pos + 1 >= c.len() {
            from = pos + 1;
            continue;
        }
        let ls = left_atom(&c, pos - 1);
        let re = right_atom(&c, pos + 1);
        let num: String = c[ls..pos].iter().collect();
        let den: String = c[pos + 1..=re].iter().collect();
        let num = sans_parentheses(num.trim());
        let den = sans_parentheses(den.trim());
        if num.is_empty() || den.is_empty() {
            from = pos + 1;
            continue;
        }
        let repl = format!("\\dfrac{{{}}}{{{}}}", num, den);
        let mut next: Vec<char> = Vec::with_capacity(c.len() + 16);
        next.extend_from_slice(&c[..ls]);
        let inserted = repl.chars().count();
        next.extend(repl.chars());
        next.extend_from_slice(&c[re + 1..]);
        c = next;
        from = ls + inserted;
    }
    c.into_iter().collect()
}
