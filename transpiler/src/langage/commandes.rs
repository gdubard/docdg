use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Obj {
    Function { var: String, expr: String },
    Matrix { rows: Vec<Vec<String>> },
    System { eqs: Vec<String> },
    Sequence { first: String, rec: String },
    Graph { sommets: Vec<String>, arcs: Vec<(usize, usize)> },
    Point { coords: Vec<String> },
    Vecteur { coords: Vec<String> },
    Plan { equation: String },
}

pub type Objects = BTreeMap<String, Obj>;

pub fn objects_json(objs: &Objects) -> Value {
    let mut m = Map::new();
    for (name, o) in objs {
        let v = match o {
            Obj::Function { var, expr } => json!({"kind":"function","var":var,"expr":expr}),
            Obj::Matrix { rows } => json!({"kind":"matrix","rows":rows}),
            Obj::System { eqs } => json!({"kind":"system","eqs":eqs}),
            Obj::Sequence { first, rec } => json!({"kind":"sequence","first":first,"rec":rec}),
            Obj::Graph { sommets, arcs } => json!({"kind":"graph","sommets":sommets,"arcs":arcs}),
            Obj::Point { coords } => json!({"kind":"point","coords":coords}),
            Obj::Vecteur { coords } => json!({"kind":"vecteur","coords":coords}),
            Obj::Plan { equation } => json!({"kind":"plan","equation":equation}),
        };
        m.insert(name.clone(), v);
    }
    Value::Object(m)
}

fn strip_article(s: &str) -> &str {
    let t = s.trim();
    for a in [
        "les ", "la ", "le ", "l'", "un ", "une ", "des ", "du ", "de la ", "de ", "d'",
    ] {
        if let Some(r) = t.strip_prefix(a) {
            return r.trim();
        }
    }
    t
}

fn split_top_level(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            _ => {}
        }
        let decimale = i > 0
            && i + 1 < chars.len()
            && chars[i - 1].is_ascii_digit()
            && chars[i + 1].is_ascii_digit();
        if depth == 0 && c == ',' && !decimale {
            out.push(cur.trim().to_string());
            cur.clear();
            i += 1;
            continue;
        }
        if depth == 0 && chars[i..].starts_with(&[' ', 'e', 't', ' ']) {
            out.push(cur.trim().to_string());
            cur.clear();
            i += 4;
            continue;
        }
        cur.push(c);
        i += 1;
    }
    out.push(cur.trim().to_string());
    out.into_iter().filter(|x| !x.is_empty()).collect()
}

fn split_names(s: &str) -> Vec<String> {
    crate::utils::texte::noms_separes(s)
}

fn is_name(s: &str) -> bool {
    !s.is_empty()
        && s.chars().next().unwrap().is_alphabetic()
        && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn parse_declaration(rest: &str, block: Option<&str>, objs: &mut Objects) -> Option<Vec<String>> {
    let t = rest.trim();
    let low = t.to_lowercase();

    if low.starts_with("la matrice") || low.starts_with("les matrices") {
        let after = strip_article(&t["la matrice".len().min(t.len())..]);
        let name = after.split_whitespace().next().unwrap_or("M").to_string();
        let body = block?;
        let rows: Vec<Vec<String>> = body
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.replace('|', ";")
                    .split(';')
                    .map(|c| c.trim().to_string())
                    .collect()
            })
            .filter(|r: &Vec<String>| !r.is_empty())
            .collect();
        objs.insert(name.clone(), Obj::Matrix { rows });
        return Some(vec![name]);
    }

    if low.starts_with("le système") || low.starts_with("le systeme") {
        let after = t.splitn(3, char::is_whitespace).nth(2).unwrap_or("s");
        let name = after.split_whitespace().next().unwrap_or("s").to_string();
        let body = block?;
        let eqs: Vec<String> = body
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        objs.insert(name.clone(), Obj::System { eqs });
        return Some(vec![name]);
    }

    if low.contains("fonction") {
        let after = match t.find("fonctions") {
            Some(i) => &t[i + "fonctions".len()..],
            None => &t[t.find("fonction")? + "fonction".len()..],
        };
        let mut declared = Vec::new();
        for piece in split_top_level(after) {
            let p = piece.trim();
            if p.is_empty() {
                continue;
            }
            let (head, expr) = p.split_once('=')?;
            let head = head.trim();
            let open = head.find('(')?;
            let name = head[..open].trim().to_string();
            let var = head[open + 1..].trim_end_matches(')').trim().to_string();
            if !is_name(&name) {
                continue;
            }
            objs.insert(
                name.clone(),
                Obj::Function {
                    var,
                    expr: expr.trim().to_string(),
                },
            );
            declared.push(name);
        }
        if declared.is_empty() {
            return None;
        }
        return Some(declared);
    }

    if let Some(noms) = declare_geometrie(t, &low, objs) {
        return Some(noms);
    }

    if low.starts_with("la suite") {
        let name = t
            .split_whitespace()
            .nth(2)
            .unwrap_or("u")
            .trim_end_matches(',')
            .to_string();
        let init = t.find(&format!("{}(0)", name)).and_then(|i| {
            t[i..]
                .split_once('=')
                .map(|(_, r)| r.split(" et ").next().unwrap_or("0").trim().to_string())
        })?;
        let rec_src = t.split("=").last()?.trim().to_string();
        let rec = rec_src.replace(&format!("{}(n)", name), "PREV");
        objs.insert(name.clone(), Obj::Sequence { first: init, rec });
        return Some(vec![name]);
    }

    None
}

fn coordonnees(bloc: &str) -> Vec<Vec<String>> {
    let mut listes = Vec::new();
    let mut reste = bloc;
    while let Some(i) = reste.find('(') {
        let apres = &reste[i + 1..];
        let j = match apres.find(')') {
            Some(j) => j,
            None => break,
        };
        let comps: Vec<String> = apres[..j]
            .split(&[';', ','][..])
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .collect();
        if comps.len() >= 2 {
            listes.push(comps);
        }
        reste = &apres[j + 1..];
    }
    listes
}

fn noms_declares(entete: &str) -> Vec<String> {
    entete
        .split(&[',', ';'][..])
        .flat_map(|m| m.split(" et "))
        .map(|m| first_word(m.split('(').next().unwrap_or(m)))
        .filter(|n| is_name(n) && n.chars().all(|c| !c.is_ascii_digit()))
        .collect()
}

fn declare_geometrie(t: &str, low: &str, objs: &mut Objects) -> Option<Vec<String>> {
    if low.contains("plan") && low.contains("équation") {
        let i = t.find("plan")? + "plan".len();
        let nom = first_word(&t[i..]);
        let equation = t[t.find("équation")? + "équation".len()..]
            .trim()
            .trim_end_matches('.')
            .to_string();
        if !is_name(&nom) || equation.is_empty() {
            return None;
        }
        objs.insert(nom.clone(), Obj::Plan { equation });
        return Some(vec![nom]);
    }
    let vecteur = low.contains("vecteur");
    if !vecteur && !low.contains("point") {
        return None;
    }
    let cle = if vecteur { "vecteur" } else { "point" };
    let i = low.find(cle)? + cle.len();
    let suite = t[i..].trim_start_matches('s').trim();
    let entete = suite
        .split(" de coordonnées")
        .next()
        .unwrap_or(suite)
        .to_string();
    let noms = noms_declares(&entete);
    let listes = coordonnees(suite);
    if noms.is_empty() || noms.len() != listes.len() {
        return None;
    }
    for (nom, coords) in noms.iter().zip(listes) {
        let objet = if vecteur {
            Obj::Vecteur { coords }
        } else {
            Obj::Point { coords }
        };
        objs.insert(nom.clone(), objet);
    }
    Some(noms)
}

fn nom_final(s: &str) -> Option<String> {
    let t = s.trim().trim_end_matches('.');
    let tail = t
        .rsplit_once(" de ")
        .map(|(_, r)| r)
        .or_else(|| t.rsplit_once(" d'").map(|(_, r)| r))
        .unwrap_or(t);
    let n = first_word(tail);
    if is_name(&n) {
        Some(n)
    } else {
        None
    }
}

fn nettoie_expr(s: &str) -> String {
    let mut t = s.trim().trim_end_matches('.').to_string();
    for q in [
        "diophantienne",
        "trigonométrique",
        "trigonometrique",
        "la fraction",
        "numériquement",
        "numeriquement",
    ] {
        t = t.replace(q, " ");
    }
    for d in [" dans R[X]", " dans C[X]", " dans Q[X]", " dans Z[X]"] {
        if let Some(i) = t.find(d) {
            t.truncate(i);
        }
    }
    if let Some(i) = t.find(" sur [") {
        t.truncate(i);
    }
    t.trim().to_string()
}

fn after_key<'a>(s: &'a str, keys: &[&str]) -> Option<&'a str> {
    for k in keys {
        if let Some(i) = s.find(k) {
            return Some(s[i + k.len()..].trim());
        }
    }
    None
}

fn first_word(s: &str) -> String {
    s.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .find(|w| !w.is_empty())
        .unwrap_or("")
        .to_string()
}

fn ordinal(s: &str) -> Option<usize> {
    let l = s.to_lowercase();
    for (w, n) in [
        ("seconde", 2),
        ("second", 2),
        ("troisième", 3),
        ("deuxième", 2),
    ] {
        if l.contains(w) {
            return Some(n);
        }
    }
    None
}

pub fn parse_command(verb: &str, rest: &str) -> Option<Value> {
    let t = rest.trim();
    let low = t.to_lowercase();

    let single = |op: &str, name: &str| Some(json!({"op": op, "args": {"name": name}}));

    match verb {
        "Ajuste" => {
            let (modele, bloc) = after_key(t, &["aux données", "aux donnees"])
                .and_then(|d| {
                    let coupe = low
                        .find("aux données")
                        .or_else(|| low.find("aux donnees"))?;
                    Some((t[..coupe].trim(), d))
                })?;
            let (gauche, droite) = modele.split_once('=')?;
            let nom = gauche.trim();
            let var = nom
                .split_once('(')
                .and_then(|(_, r)| r.split_once(')'))
                .map(|(v, _)| v.trim())
                .filter(|v| is_name(v))
                .unwrap_or("x");
            let points: Vec<[f64; 2]> = crate::maths::statistiques::couples(bloc)
                .into_iter()
                .map(|(x, y)| [x, y])
                .collect();
            if points.len() < 2 {
                return None;
            }
            return Some(json!({"op":"ajuste","args":{
                "expr": nettoie_expr(droite),
                "var": var,
                "nom": nom,
                "points": points}}));
        }
        "Factorise" => {
            let anneau = if low.contains("c[x]") {
                "C"
            } else if low.contains("q[x]") {
                "Q"
            } else {
                "R"
            };
            let annonce = low.contains("[x]");
            return Some(
                json!({"op":"factor","args":{"expr":nettoie_expr(t),"ring":anneau,"annonce":annonce}}),
            );
        }
        "Simplifie" => return Some(json!({"op":"simplify","args":{"expr":nettoie_expr(t)}})),
        "Développe" => {
            let e = after_key(t, &["et réduis", "et reduis"]).unwrap_or(t);
            return Some(json!({"op":"expand","args":{"expr":nettoie_expr(e)}}));
        }
        "Décompose" => {
            let e = after_key(t, &["éléments simples", "elements simples"]).unwrap_or(t);
            return Some(json!({"op":"apart","args":{"expr":nettoie_expr(e)}}));
        }
        "Diagonalise" => return single("diagonalize", &first_word(t)),
        "Trigonalise" => return single("trigonalize", &first_word(t)),
        "Effectue" => {
            let e = after_key(t, &["division euclidienne de"])?;
            let (p, q) = e.split_once(" par ")?;
            return Some(json!({"op":"polydiv",
                "args":{"a":p.trim(),"b":q.trim().trim_end_matches('.')}}));
        }
        "Résous" => {
            if low.contains("dérivées partielles") || low.contains("derivees partielles") {
                let e = after_key(t, &["aux dérivées partielles", "aux derivees partielles"])?;
                return Some(json!({"op":"pde","args":{"expr":e.trim().trim_end_matches('.')}}));
            }
            if low.contains("différentielle") || low.contains("differentielle") {
                let e = after_key(t, &["l'équation différentielle", "l'equation differentielle"])?;
                let (equation, inconnue) = match e.split_once(", d'inconnue") {
                    Some((eq, y)) => (eq.trim(), Some(first_word(y.trim()))),
                    None => (e.trim().trim_end_matches('.'), None),
                };
                return Some(json!({"op":"ode",
                    "args":{"expr":equation,"unknown":inconnue}}));
            }
            let domain = if low.contains("dans cc") || low.contains("dans les complexes") {
                "C"
            } else if low.contains("dans les entiers") || low.contains("dans zz") {
                "Z"
            } else {
                "R"
            };
            if low.contains("trigonométrique") || low.contains("trigonometrique") {
                let e = after_key(t, &["trigonométrique", "trigonometrique"])?;
                return Some(json!({"op":"trig_solve",
                    "args":{"expr":nettoie_expr(e.trim().trim_end_matches('.'))}}));
            }
            if let Some(e) = after_key(t, &["l'équation", "l'equation", "équation"]) {
                if low.contains("numériquement") || low.contains("numeriquement") {
                    let bornes = t.rsplit_once(" sur ").map(|(_, b)| b.trim().to_string());
                    return Some(json!({"op":"solve_num",
                        "args":{"expr":nettoie_expr(e),"range":bornes}}));
                }
                return Some(json!({"op":"solve","args":{"expr":nettoie_expr(e),"domain":domain}}));
            }
            let sans = strip_article(t);
            let sans = sans
                .strip_prefix("système")
                .or_else(|| sans.strip_prefix("systeme"))
                .unwrap_or(sans)
                .trim();
            let name = first_word(sans);
            if is_name(&name) {
                return single("system", &name);
            }
            return None;
        }
        "Détermine" => {
            if low.contains("nature de la série") || low.contains("nature de la serie") {
                let e = after_key(t, &["terme général", "terme general"])?;
                return Some(json!({"op":"series_nature",
                    "args":{"expr":nettoie_expr(e.trim_end_matches('.'))}}));
            }
            if low.contains("nature de l'intégrale") || low.contains("nature de l'integrale") {
                let name = nom_final(t.split("entre").next().unwrap_or(t))?;
                let seg = after_key(t, &["entre"])?;
                let (a, b) = seg.split_once(" et ")?;
                return Some(json!({"op":"integral_nature",
                    "args":{"name":name,"from":a.trim(),"to":b.trim().trim_end_matches('.')}}));
            }
            if low.contains("points critiques") {
                return single("critical", &nom_final(t)?);
            }
            if low.contains("asymptotes") {
                return single("asymptotes", &nom_final(t)?);
            }
            if low.contains("zéros") || low.contains("zeros") {
                let tail = t
                    .trim()
                    .trim_end_matches('.')
                    .rsplit_once(" de ")
                    .map(|(_, r)| r)
                    .or_else(|| t.trim().rsplit_once(" d'").map(|(_, r)| r))?;
                let noms = split_names(tail);
                if noms.is_empty() {
                    return None;
                }
                let reqs: Vec<Value> = noms
                    .iter()
                    .map(|n| json!({"op":"zeros","args":{"name":n}}))
                    .collect();
                return Some(Value::Array(reqs));
            }
            if low.contains("forme canonique") {
                let e = after_key(t, &["canonique de"])?;
                return Some(json!({"op":"canonical","args":{"expr":e}}));
            }
            if low.contains("valeurs propres") {
                return single("eigen", &nom_final(t)?);
            }
            if low.contains("noyau") {
                return single("nullspace", &nom_final(t)?);
            }
            if low.contains("image") {
                return single("colspace", &nom_final(t)?);
            }
            return None;
        }
        "Étudie" | "Etudie" => {
            if low.contains("convexité") || low.contains("convexite") {
                return single("convexity", &nom_final(t)?);
            }
            return None;
        }
        "Calcule" => {}
        _ => return None,
    }

    if low.contains("série de fourier") || low.contains("serie de fourier") {
        let name = after_key(t, &["Fourier de", "fourier de"]).map(first_word)?;
        let seg = after_key(t, &[" sur "])?;
        let (a, b) = seg
            .split_once(']')
            .map(|(d, _)| d)
            .unwrap_or(&seg)
            .trim_start_matches('[')
            .split_once(';')?;
        let order = after_key(t, &["ordre"])
            .and_then(|o| first_word(o).parse::<usize>().ok())
            .unwrap_or(4);
        return Some(json!({"op":"fourier",
            "args":{"name":name,"from":a.trim(),"to":b.trim(),"order":order}}));
    }
    if low.contains("transformée de laplace") || low.contains("transformee de laplace") {
        let inverse = low.contains("inverse");
        let name = nom_final(t)?;
        return Some(json!({"op": if inverse { "laplace_inv" } else { "laplace" },
            "args":{"name":name}}));
    }
    if low.contains("wronskien") {
        let seg = after_key(t, &["wronskien de"])?;
        let noms = split_names(&seg);
        if noms.len() < 2 {
            return None;
        }
        return Some(json!({"op":"wronskian","args":{"a":noms[0],"b":noms[1]}}));
    }
    if low.contains("dérivée partielle") {
        let name = after_key(t, &["partielle de"]).map(first_word)?;
        let var = after_key(t, &["par rapport à"]).map(first_word)?;
        return Some(json!({"op":"partial","args":{"name":name,"var":var}}));
    }
    if low.contains("dérivée") || low.contains("derivee") {
        let order = ordinal(&low).unwrap_or(1);
        let tail = t
            .trim()
            .rsplit_once(" de ")
            .map(|(_, r)| r)
            .or_else(|| t.trim().rsplit_once(" d'").map(|(_, r)| r))?;
        let names = split_names(tail);
        let reqs: Vec<Value> = names
            .iter()
            .map(|n| json!({"op":"derive","args":{"name":n,"order":order}}))
            .collect();
        return Some(Value::Array(reqs));
    }
    if low.contains("primitive") {
        return single("primitive", &nom_final(t)?);
    }
    if low.contains("intégrale numérique") || low.contains("integrale numerique") {
        let name = after_key(t, &["numérique de", "numerique de"]).map(first_word)?;
        let seg = after_key(t, &["entre"])?;
        let (a, b) = seg.split_once(" et ")?;
        return Some(json!({"op":"integral_num","args":{"name":name,"from":a.trim(),"to":b.trim()}}));
    }
    if low.contains("intégrale") || low.contains("integrale") {
        let name = nom_final(t.split("entre").next().unwrap_or(t))?;
        let seg = after_key(t, &["entre"])?;
        let (a, b) = seg.split_once(" et ")?;
        return Some(json!({"op":"integral","args":{"name":name,"from":a.trim(),"to":b.trim()}}));
    }
    if low.contains("limite") {
        let head = t.split(" en ").next().unwrap_or(t);
        let name = nom_final(head)?;
        let at_seg = after_key(t, &[" en "])?;
        let side = if at_seg.contains("à droite") {
            Some("droite")
        } else if at_seg.contains("à gauche") {
            Some("gauche")
        } else {
            None
        };
        let at = at_seg
            .replace("à droite", "")
            .replace("à gauche", "")
            .trim()
            .to_string();
        return Some(json!({"op":"limit","args":{"name":name,"at":at,"side":side}}));
    }
    if low.contains("développement limité") || low.contains("developpement limite") {
        let name = after_key(t, &["limité de", "limite de"]).map(first_word)?;
        let at = after_key(t, &[" en "]).map(first_word).unwrap_or("0".into());
        let order = after_key(t, &["ordre"])
            .and_then(|o| first_word(o).parse::<usize>().ok())
            .unwrap_or(4);
        return Some(json!({"op":"series","args":{"name":name,"at":at,"order":order}}));
    }
    if low.contains("équivalent") || low.contains("equivalent") {
        let name = nom_final(t.split(" en ").next().unwrap_or(t))?;
        let at = after_key(t, &[" en "])?.trim().to_string();
        return Some(json!({"op":"equivalent","args":{"name":name,"at":at}}));
    }
    if low.contains("image de") {
        let value = after_key(t, &["image de"])?;
        let (v, name) = value.split_once(" par ")?;
        return Some(
            json!({"op":"image","args":{"name":first_word(name),"value":v.trim().to_string()}}),
        );
    }
    if low.starts_with("la somme de") || low.starts_with("le produit de") {
        let op = if low.starts_with("la somme") {
            "sum"
        } else {
            "product"
        };
        let expr = after_key(t, &["de "])?;
        let (e, tail) = expr.split_once(" pour ")?;
        let idx = first_word(tail);
        let bounds = after_key(tail, &["de "])?;
        let (a, b) = bounds.split_once(" à ")?;
        return Some(
            json!({"op":op,"args":{"expr":e.trim(),"index":idx,"from":a.trim(),"to":b.trim()}}),
        );
    }
    if low.contains("pgcd") {
        let seg = after_key(t, &["pgcd de", "PGCD de"])?;
        let (a, b) = seg.split_once(" et ")?;
        let (a, b) = (a.trim(), b.trim().trim_end_matches('.'));
        let entiers = |s: &str| s.chars().all(|c| c.is_ascii_digit() || c == '-' || c.is_whitespace());
        if entiers(a) && entiers(b) {
            return Some(json!({"op":"gcd","args":{"a":a,"b":b}}));
        }
        return Some(json!({"op":"polygcd","args":{"a":a,"b":b}}));
    }
    if low.contains("état stable") || low.contains("etat stable") {
        return single("markov", &nom_final(t)?);
    }
    if low.contains("déterminant") || low.contains("determinant") {
        return single("det", &nom_final(t)?);
    }
    if low.contains("l'inverse") {
        return single("inverse", &nom_final(t)?);
    }
    if low.contains("le rang") {
        return single("rank", &nom_final(t)?);
    }
    if low.contains("polynôme caractéristique") {
        return single("charpoly", &nom_final(t)?);
    }
    if low.contains("polynôme minimal") {
        return single("minpoly", &nom_final(t)?);
    }
    if low.contains("puissance") {
        let power = after_key(t, &["puissance"])
            .and_then(|p| first_word(p).parse::<usize>().ok())
            .unwrap_or(2);
        let name = nom_final(t)?;
        return Some(json!({"op":"matpow","args":{"name":name,"power":power}}));
    }
    if low.contains("gradient") {
        return single("gradient", &nom_final(t)?);
    }
    if low.contains("hessienne") {
        return single("hessian", &nom_final(t)?);
    }
    if low.contains("transformée de laplace inverse") {
        return single("laplace_inv", &after_key(t, &["inverse de"]).map(first_word)?);
    }
    if low.contains("transformée de laplace") {
        return single("laplace", &after_key(t, &["laplace de"]).map(first_word)?);
    }
    if low.contains("premiers termes") {
        let count = first_word(t).parse::<usize>().unwrap_or(6);
        let name = nom_final(t)?;
        return Some(json!({"op":"terms","args":{"name":name,"count":count}}));
    }

    if t.chars()
        .all(|c| !c.is_alphabetic() || "eipxnk".contains(c))
        && t.chars().any(|c| c.is_ascii_digit())
    {
        return Some(json!({"op":"arith","args":{"expr":t}}));
    }
    None
}
