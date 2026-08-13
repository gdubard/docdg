use crate::langage::commandes::Obj;
use crate::Env;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Q {
    n: i128,
    d: i128,
}

fn pgcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn ppcm(a: i128, b: i128) -> i128 {
    (a / pgcd(a, b)).abs() * b.abs()
}

fn pgcd_etendu(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, u, v) = pgcd_etendu(b, a % b);
    (g, v, u - (a / b) * v)
}

impl Q {
    fn new(n: i128, d: i128) -> Q {
        let s = if d < 0 { -1 } else { 1 };
        let g = pgcd(n, d).max(1);
        Q {
            n: s * n / g,
            d: s * d / g,
        }
    }
    fn ent(n: i128) -> Q {
        Q { n, d: 1 }
    }
    fn zero() -> Q {
        Q::ent(0)
    }
    fn est_entier(&self) -> bool {
        self.d == 1
    }
    fn est_nul(&self) -> bool {
        self.n == 0
    }
    fn signe(&self) -> i128 {
        self.n.signum()
    }
    fn abs(&self) -> Q {
        Q::new(self.n.abs(), self.d)
    }
    fn neg(&self) -> Q {
        Q::new(-self.n, self.d)
    }
    fn add(&self, o: Q) -> Q {
        Q::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }
    fn sub(&self, o: Q) -> Q {
        self.add(o.neg())
    }
    fn mul(&self, o: Q) -> Q {
        Q::new(self.n * o.n, self.d * o.d)
    }
    fn div(&self, o: Q) -> Q {
        Q::new(self.n * o.d, self.d * o.n)
    }
    fn approx(&self) -> f64 {
        self.n as f64 / self.d as f64
    }
    fn tex(&self) -> String {
        if self.d == 1 {
            return self.n.to_string();
        }
        let s = if self.n < 0 { "-" } else { "" };
        format!("{}\\dfrac{{{}}}{{{}}}", s, self.n.abs(), self.d)
    }
    fn tex_paren(&self) -> String {
        if self.d == 1 && self.n >= 0 {
            self.n.to_string()
        } else {
            format!("\\left({}\\right)", self.tex())
        }
    }
}

fn entier(s: &str) -> Option<i128> {
    let t: String = s
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '\u{00A0}' && *c != '\u{202F}')
        .collect();
    let t = t.trim_end_matches(|c: char| !c.is_ascii_digit());
    t.parse::<i128>().ok()
}

fn rationnel(s: &str) -> Option<Q> {
    let t = s.trim().trim_end_matches('.');
    if let Some((a, b)) = t.split_once('/') {
        return Some(Q::new(entier(a)?, entier(b)?));
    }
    if let Some((a, b)) = t.split_once(',') {
        let ent = entier(a)?;
        let dec = b.trim();
        if dec.chars().all(|c| c.is_ascii_digit()) && !dec.is_empty() {
            let p = 10i128.pow(dec.len() as u32);
            let frac: i128 = dec.parse().ok()?;
            let signe = if t.trim_start().starts_with('-') { -1 } else { 1 };
            return Some(Q::new(ent * p + signe * frac, p));
        }
        return None;
    }
    entier(t).map(Q::ent)
}

fn premier_entier_apres(s: &str, cle: &str) -> Option<i128> {
    let i = s.to_lowercase().find(&cle.to_lowercase())?;
    let chars: Vec<char> = s[i + cle.len()..].chars().collect();
    let debut = chars.iter().position(|c| c.is_ascii_digit())?;
    let mut nombre = String::new();
    if debut > 0 && chars[debut - 1] == '-' {
        nombre.push('-');
    }
    let mut j = debut;
    while j < chars.len() {
        let c = chars[j];
        if c.is_ascii_digit() {
            nombre.push(c);
            j += 1;
        } else if (c == ' ' || c == '\u{00A0}' || c == '\u{202F}')
            && chars.get(j + 1).map(|x| x.is_ascii_digit()).unwrap_or(false)
        {
            j += 1;
        } else {
            break;
        }
    }
    nombre.parse().ok()
}

fn nombre_a_chiffres(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_digit() || *c == ',')
        .collect()
}

pub(crate) fn bloc_prose(lignes: &[String]) -> String {
    let mut out = String::from("<div class=\"calcul-prose\">");
    for l in lignes {
        if let Some(interieur) = l.strip_prefix("\\[").and_then(|x| x.strip_suffix("\\]")) {
            out.push_str(&format!("<div class=\"calcul\">\\[{}\\]</div>", interieur));
        } else {
            out.push_str(&format!("<p>{}</p>", l));
        }
    }
    out.push_str("</div>");
    out
}

fn prose(lignes: &[String]) -> Option<String> {
    Some(bloc_prose(lignes))
}

fn calcul(inner: &str) -> Option<String> {
    Some(crate::layout::rendu::bloc_calcul(inner))
}

fn approx_fr(v: f64) -> String {
    format!("{:.3}", v).replace('.', "{,}")
}

fn liste_fr(mots: &[String]) -> String {
    match mots.len() {
        0 => String::new(),
        1 => mots[0].clone(),
        _ => format!(
            "{} et {}",
            mots[..mots.len() - 1].join(", "),
            mots[mots.len() - 1]
        ),
    }
}

/// La décomposition positionnelle de l'école élémentaire :
/// « 4 782 = 4 × 1 000 + 7 × 100 + 8 × 10 + 2 ». Les rangs à zéro s'omettent,
/// le chiffre des unités s'écrit seul.
fn decomposition_positionnelle(desc: &str) -> Option<String> {
    let compact: String = desc
        .trim()
        .trim_end_matches('.')
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '\u{202f}' && *c != '\u{a0}')
        .collect();
    if compact.is_empty() || !compact.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let n: u64 = compact.parse().ok()?;
    if n < 10 {
        return None;
    }
    let chiffres: Vec<u32> = compact.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut termes: Vec<String> = Vec::new();
    let total = chiffres.len();
    for (i, c) in chiffres.iter().enumerate() {
        if *c == 0 {
            continue;
        }
        let rang = total - 1 - i;
        if rang == 0 {
            // Les unités ne forment pas un produit : elles s'écrivent seules.
            termes.push(format!("{}", c));
        } else {
            // Chaque produit est parenthésé : l'élève voit des paquets, non
            // une file d'opérations — c'est tout l'objet de l'exercice.
            let puissance = 10u64.pow(rang as u32);
            termes.push(format!(
                "({} \\times {})",
                c,
                groupe_milliers(puissance)
            ));
        }
    }
    // Un terme seul n'a rien à grouper : « 50 = 5 × 10 », sans parenthèses.
    if termes.len() == 1 {
        termes[0] = termes[0].trim_start_matches('(').trim_end_matches(')').to_string();
    }
    Some(bloc_prose(&[format!(
        "\\[{} = {}\\]",
        groupe_milliers(n),
        termes.join(" + ")
    )]))
}

/// Les milliers se séparent d'une espace fine, à la française.
fn groupe_milliers(n: u64) -> String {
    let brut = n.to_string();
    let mut out = String::new();
    for (i, c) in brut.chars().enumerate() {
        if i > 0 && (brut.len() - i) % 3 == 0 {
            out.push_str("\\,");
        }
        out.push(c);
    }
    out
}

pub(crate) fn commande_en_ligne(verbe: &str, desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    match verbe {
        "Calcule" => {
            let (a, b, op, c, d) = frac_binaire(desc)?;
            let r = match op {
                '+' => Q::new(a, b).add(Q::new(c, d)),
                '-' => Q::new(a, b).sub(Q::new(c, d)),
                '*' | '×' => Q::new(a, b).mul(Q::new(c, d)),
                ':' | '÷' if c != 0 => Q::new(a, b).div(Q::new(c, d)),
                _ => return None,
            };
            let symbole = match op {
                '+' => "+",
                '-' => "-",
                '*' | '×' => "\\times",
                _ => "\\div",
            };
            Some(format!(
                "{} {} {} = {}",
                tex_frac_brute(a, b),
                symbole,
                tex_frac_brute(c, d),
                r.tex()
            ))
        }
        "Simplifie" if bas.contains("la fraction") => {
            let (a, b) = fraction_parties(desc)?;
            let r = Q::new(a, b);
            if r.n == a && r.d == b {
                return Some(tex_frac_brute(a, b));
            }
            Some(format!("{} = {}", tex_frac_brute(a, b), r.tex()))
        }
        "Écris" if bas.contains("notation scientifique") => {
            let (echo, mantisse, exposant) = scientifique_parties(desc)?;
            Some(format!("{} = {} \\times 10^{{{}}}", echo, mantisse, exposant))
        }
        "Effectue" if bas.contains("division euclidienne") => {
            let (a, b, q, r) = division_parties(desc)?;
            Some(format!("{} = {} \\times {} + {}", a, b, q, r))
        }
        "Dénombre" => {
            if bas.contains("combinaison") {
                let (n, k, v) = parmi_valeur(desc, "combinaisons de", true)?;
                return Some(format!("\\dbinom{{{}}}{{{}}} = {}", n, k, v));
            }
            if bas.contains("arrangement") {
                let (n, k, v) = parmi_valeur(desc, "arrangements de", false)?;
                return Some(format!("A_{{{}}}^{{{}}} = {}", n, k, v));
            }
            if bas.contains("permutation") {
                let n = premier_entier_apres(desc, "permutations de")?;
                return Some(format!("{}! = {}", n, factorielle(n)?));
            }
            None
        }
        _ => None,
    }
}

pub fn commande(verbe: &str, desc: &str, corps: Option<&str>, env: &mut Env) -> Option<String> {
    let bas = desc.to_lowercase();
    match verbe {
        // Décomposer un nombre entier, au sens de l'école élémentaire :
        // « 4 782 = 4 × 1 000 + 7 × 100 + 8 × 10 + 2 ». La décomposition en
        // éléments simples, elle, se demande avec son complément et continue
        // vers le calcul formel.
        "Décompose" => {
            if let Some(html) = decomposition_positionnelle(desc) {
                return Some(html);
            }
            None
        }
        "Écris" => {
            if bas.contains("notation scientifique") {
                return notation_scientifique(desc);
            }
            if bas.contains("vecteur") {
                return vecteur(desc);
            }
            if bas.contains("complexe") {
                return complexe(desc);
            }
            None
        }
        "Vérifie" if bas.contains("divisible") => divisibilite(desc),
        "Effectue" if bas.contains("division euclidienne") => division_entiere(desc),
        "Applique" => {
            if bas.contains("programme") {
                return programme_applique(desc, corps?);
            }
            if bas.contains("euclide") {
                return euclide(desc);
            }
            None
        }
        "Exprime" if bas.contains("programme") => programme_exprime(desc, corps?),
        "Étudie" => {
            if bas.contains("second degré") || bas.contains("second degre") {
                return second_degre(corps?);
            }
            if bas.contains("arithmétique") || bas.contains("arithmetique") {
                return suite(desc, true);
            }
            if bas.contains("géométrique") || bas.contains("geometrique") {
                return suite(desc, false);
            }
            if bas.contains("congruence") {
                return congruence(desc);
            }
            None
        }
        "Dresse" => {
            if bas.contains("cayley") {
                return cayley(desc);
            }
            if bas.contains("pascal") {
                return pascal(desc);
            }
            if bas.contains("facteurs premiers") {
                return facteurs_premiers(desc);
            }
            if bas.contains("adjacence") {
                return adjacence(desc, env);
            }
            None
        }
        "Dénombre" => {
            if bas.contains("combinaison") {
                return combinaisons(desc);
            }
            if bas.contains("arrangement") {
                return arrangements(desc);
            }
            if bas.contains("permutation") {
                return permutations(desc);
            }
            if bas.contains("chemins") {
                return chemins(desc, env);
            }
            None
        }
        "Construis" if bas.contains("graphe") => graphe(desc, corps?, env),
        "Calcule" => fraction_pas_a_pas(desc),
        "Simplifie" if bas.contains("la fraction") => simplifie_fraction(desc),
        "Résous" if bas.contains("diophantienne") => diophantienne(desc),
        _ => None,
    }
}

pub fn declare_graphe(desc: &str, corps: &str, env: &mut Env) {
    let _ = graphe(desc, corps, env);
}

fn tex_frac_brute(n: i128, d: i128) -> String {
    let s = if n < 0 { "-" } else { "" };
    format!("{}\\dfrac{{{}}}{{{}}}", s, n.abs(), d)
}

fn frac_binaire(t: &str) -> Option<(i128, i128, char, i128, i128)> {
    let mut reste = t.trim();
    let lire_frac = |s: &mut &str| -> Option<(i128, i128)> {
        *s = s.trim_start();
        let fin = s
            .char_indices()
            .find(|(i, c)| !(c.is_ascii_digit() || (*i == 0 && *c == '-')))
            .map(|(i, _)| i)
            .unwrap_or(s.len());
        let num: i128 = s[..fin].parse().ok()?;
        *s = s[fin..].trim_start();
        if !s.starts_with('/') {
            return None;
        }
        *s = s[1..].trim_start();
        let fin = s
            .char_indices()
            .find(|(_, c)| !c.is_ascii_digit())
            .map(|(i, _)| i)
            .unwrap_or(s.len());
        let den: i128 = s[..fin].parse().ok()?;
        *s = &s[fin..];
        Some((num, den))
    };
    let (a, b) = lire_frac(&mut reste)?;
    reste = reste.trim_start();
    let op = reste.chars().next()?;
    if !"+-*×:÷".contains(op) {
        return None;
    }
    reste = &reste[op.len_utf8()..];
    let (c, d) = lire_frac(&mut reste)?;
    if !reste.trim().is_empty() || b <= 0 || d <= 0 {
        return None;
    }
    Some((a, b, op, c, d))
}

fn fraction_pas_a_pas(desc: &str) -> Option<String> {
    let (a, b, op, c, d) = frac_binaire(desc)?;
    let (fa, fb) = (tex_frac_brute(a, b), tex_frac_brute(c, d));
    let mut lignes = Vec::new();
    match op {
        '+' | '-' => {
            let m = ppcm(b, d);
            let (na, nc) = (a * (m / b), c * (m / d));
            if b != d {
                lignes.push(format!(
                    "On met les fractions au même dénominateur, {} : \\({} = {}\\) et \\({} = {}\\).",
                    m,
                    fa,
                    tex_frac_brute(na, m),
                    fb,
                    tex_frac_brute(nc, m)
                ));
            } else {
                lignes.push(format!("Les deux fractions ont déjà le même dénominateur, {}.", b));
            }
            let somme = if op == '+' { na + nc } else { na - nc };
            let r = Q::new(somme, m);
            let mut fin = format!(
                "\\dfrac{{{} {} {}}}{{{}}} = {}",
                na,
                if op == '+' { "+" } else { "-" },
                if nc < 0 { format!("({})", nc) } else { nc.to_string() },
                m,
                tex_frac_brute(somme, m)
            );
            if r.d != m || r.n != somme {
                fin.push_str(&format!(" = {}", r.tex()));
            }
            lignes.push(format!("Donc \\({} {} {} = {}\\).", fa, op, fb, fin));
        }
        '*' | '×' => {
            let (pn, pd) = (a * c, b * d);
            let r = Q::new(pn, pd);
            let mut fin = format!(
                "\\dfrac{{{} \\times {}}}{{{} \\times {}}} = {}",
                a,
                if c < 0 { format!("({})", c) } else { c.to_string() },
                b,
                d,
                tex_frac_brute(pn, pd)
            );
            if r.d != pd || r.n != pn {
                fin.push_str(&format!(" = {}", r.tex()));
            }
            lignes.push(format!(
                "On multiplie les numérateurs entre eux et les dénominateurs entre eux : \\({} \\times {} = {}\\).",
                fa, fb, fin
            ));
        }
        ':' | '÷' => {
            if c == 0 {
                lignes.push("On ne peut pas diviser par la fraction nulle.".into());
                return prose(&lignes);
            }
            let (pn, pd) = (a * d, b * c);
            let r = Q::new(pn, pd);
            let mut fin = format!(
                "\\dfrac{{{} \\times {}}}{{{} \\times {}}} = {}",
                a,
                d,
                b,
                if c < 0 { format!("({})", c) } else { c.to_string() },
                tex_frac_brute(pn, pd)
            );
            if r.d != pd || r.n != pn {
                fin.push_str(&format!(" = {}", r.tex()));
            }
            lignes.push(format!(
                "Diviser par une fraction, c'est multiplier par son inverse : \\({} \\div {} = {} \\times {} = {}\\).",
                fa,
                fb,
                fa,
                tex_frac_brute(d, c.abs()).replace(
                    "\\dfrac",
                    if c < 0 { "-\\dfrac" } else { "\\dfrac" }
                ),
                fin
            ));
        }
        _ => return None,
    }
    prose(&lignes)
}

fn fraction_parties(desc: &str) -> Option<(i128, i128)> {
    let i = desc.to_lowercase().find("la fraction")?;
    let expr = desc[i + "la fraction".len()..].trim().trim_end_matches('.');
    let (a, b) = expr.split_once('/')?;
    let (a, b) = (entier(a)?, entier(b)?);
    if b == 0 {
        return None;
    }
    Some((a, b))
}

fn simplifie_fraction(desc: &str) -> Option<String> {
    let (a, b) = fraction_parties(desc)?;
    let g = pgcd(a, b);
    if g <= 1 {
        return prose(&[format!(
            "\\(\\operatorname{{PGCD}}({}\\,;\\,{}) = 1\\) : la fraction \\({}\\) est déjà irréductible.",
            a,
            b,
            tex_frac_brute(a, b)
        )]);
    }
    prose(&[format!(
        "\\(\\operatorname{{PGCD}}({}\\,;\\,{}) = {}\\), donc \\({} = \\dfrac{{{} \\times {}}}{{{} \\times {}}} = {}\\), irréductible.",
        a,
        b,
        g,
        tex_frac_brute(a, b),
        a / g,
        g,
        b / g,
        g,
        Q::new(a, b).tex()
    )])
}

fn scientifique_parties(desc: &str) -> Option<(String, String, i64)> {
    let bas = desc.to_lowercase();
    let debut = bas.find("nombre").map(|i| i + "nombre".len()).unwrap_or(0);
    let fin = bas.find(" en notation").unwrap_or(desc.len());
    let brut = desc.get(debut..fin)?.trim();
    let negatif = brut.starts_with('-');
    let chiffres = nombre_a_chiffres(brut);
    let (avant, apres) = match chiffres.split_once(',') {
        Some((a, b)) => (a.to_string(), b.to_string()),
        None => (chiffres.clone(), String::new()),
    };
    let tous: String = format!("{}{}", avant, apres);
    if tous.is_empty() {
        return None;
    }
    let premier = tous.find(|c: char| c != '0')?;
    let exposant = avant.len() as i64 - 1 - premier as i64;
    let mantisse: String = tous[premier..].trim_end_matches('0').to_string();
    let mantisse = if mantisse.is_empty() { "0".into() } else { mantisse };
    let mantisse_tex = if mantisse.len() > 1 {
        format!("{}{{,}}{}", &mantisse[..1], &mantisse[1..])
    } else {
        mantisse.clone()
    };
    let echo = brut
        .chars()
        .map(|c| match c {
            ' ' | '\u{00A0}' | '\u{202F}' => "\\,".to_string(),
            ',' => "{,}".to_string(),
            c => c.to_string(),
        })
        .collect::<String>();
    let signe = if negatif { "-" } else { "" };
    Some((echo, format!("{}{}", signe, mantisse_tex), exposant))
}

fn notation_scientifique(desc: &str) -> Option<String> {
    let (echo, mantisse, exposant) = scientifique_parties(desc)?;
    prose(&[format!(
        "\\({} = {} \\times 10^{{{}}}\\) (un seul chiffre non nul avant la virgule).",
        echo, mantisse, exposant
    )])
}

fn divisibilite(desc: &str) -> Option<String> {
    let n = premier_entier_apres(desc, "si")?;
    let k = premier_entier_apres(desc, "par")?;
    if k <= 0 || n < 0 {
        return None;
    }
    if k == 3 || k == 9 {
        let chiffres: Vec<i128> = n
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i128))
            .collect();
        let somme: i128 = chiffres.iter().sum();
        let addition = chiffres
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" + ");
        if somme % k == 0 {
            return prose(&[format!(
                "La somme des chiffres de {} vaut \\({} = {}\\), qui est divisible par {} : donc {} est divisible par {}.",
                n, addition, somme, k, n, k
            )]);
        }
        return prose(&[format!(
            "La somme des chiffres de {} vaut \\({} = {}\\), qui n'est pas divisible par {} : donc {} n'est pas divisible par {}.",
            n, addition, somme, k, n, k
        )]);
    }
    let (q, r) = (n / k, n % k);
    if r == 0 {
        return prose(&[format!(
            "\\({} = {} \\times {}\\) : le reste est nul, donc {} est divisible par {}.",
            n, k, q, n, k
        )]);
    }
    prose(&[format!(
        "\\({} = {} \\times {} + {}\\) : le reste n'est pas nul, donc {} n'est pas divisible par {}.",
        n, k, q, r, n, k
    )])
}

fn division_parties(desc: &str) -> Option<(i128, i128, i128, i128)> {
    let bas = desc.to_lowercase();
    let i = bas.find("division euclidienne de")? + "division euclidienne de".len();
    let reste = &desc[i..];
    let (ga, gb) = reste.split_once(" par ")?;
    let a = entier(ga)?;
    let b = entier(gb)?;
    if b <= 0 {
        return None;
    }
    Some((a, b, a.div_euclid(b), a.rem_euclid(b)))
}

fn division_entiere(desc: &str) -> Option<String> {
    let (a, b, q, r) = division_parties(desc)?;
    prose(&[format!(
        "La division euclidienne de {} par {} s'écrit \\({} = {} \\times {} + {}\\), avec \\(0 \\leqslant {} &lt; {}\\).",
        a, b, a, b, q, r, r, b
    )])
}

enum Etape {
    Ajoute(Q),
    Soustrait(Q),
    Retranche(Q),
    Multiplie(Q),
    Divise(Q),
    Carre,
    Cube,
}

fn lire_programme(corps: &str) -> Option<Vec<Etape>> {
    let mut etapes = Vec::new();
    for ligne in corps.lines() {
        let l = ligne.trim();
        if l.is_empty() {
            continue;
        }
        let bas = l.to_lowercase();
        if bas.starts_with("choisir") {
            continue;
        }
        let valeur = |cle: &str| -> Option<Q> { rationnel(bas.strip_prefix(cle)?.trim()) };
        if bas.starts_with("ajouter") {
            etapes.push(Etape::Ajoute(valeur("ajouter")?));
        } else if bas.starts_with("soustraire") {
            etapes.push(Etape::Soustrait(valeur("soustraire")?));
        } else if bas.starts_with("retrancher") {
            etapes.push(Etape::Retranche(valeur("retrancher")?));
        } else if bas.starts_with("multiplier par") {
            etapes.push(Etape::Multiplie(valeur("multiplier par")?));
        } else if bas.starts_with("diviser par") {
            etapes.push(Etape::Divise(valeur("diviser par")?));
        } else if bas.starts_with("élever au carré") || bas.starts_with("elever au carre") {
            etapes.push(Etape::Carre);
        } else if bas.starts_with("élever au cube") || bas.starts_with("elever au cube") {
            etapes.push(Etape::Cube);
        } else {
            return None;
        }
    }
    Some(etapes)
}

fn programme_applique(desc: &str, corps: &str) -> Option<String> {
    let i = desc.rfind(" à ").or_else(|| desc.rfind(" a "))?;
    let depart = rationnel(&desc[i + 3..])?;
    let etapes = lire_programme(corps)?;
    let mut lignes = vec![format!("On part de \\({}\\).", depart.tex())];
    let mut cur = depart;
    for e in &etapes {
        let (phrase, suivant) = match e {
            Etape::Ajoute(v) => (format!("On ajoute \\({}\\)", v.tex()), cur.add(*v)),
            Etape::Soustrait(v) => (format!("On soustrait \\({}\\)", v.tex()), cur.sub(*v)),
            Etape::Retranche(v) => (format!("On retranche \\({}\\)", v.tex()), cur.sub(*v)),
            Etape::Multiplie(v) => (format!("On multiplie par \\({}\\)", v.tex()), cur.mul(*v)),
            Etape::Divise(v) => {
                if v.est_nul() {
                    lignes.push("On ne peut pas diviser par zéro.".into());
                    return prose(&lignes);
                }
                (format!("On divise par \\({}\\)", v.tex()), cur.div(*v))
            }
            Etape::Carre => ("On élève au carré".to_string(), cur.mul(cur)),
            Etape::Cube => ("On élève au cube".to_string(), cur.mul(cur).mul(cur)),
        };
        lignes.push(format!(
            "{} : \\({} \\to {}\\).",
            phrase,
            cur.tex(),
            suivant.tex()
        ));
        cur = suivant;
    }
    lignes.push(format!("Le programme renvoie \\({}\\).", cur.tex()));
    prose(&lignes)
}

fn programme_exprime(desc: &str, corps: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let variable = bas
        .find("en fonction de")
        .map(|i| desc[i + "en fonction de".len()..].trim())
        .and_then(|r| r.split_whitespace().next())
        .unwrap_or("x")
        .to_string();
    let etapes = lire_programme(corps)?;
    #[derive(PartialEq)]
    enum Forme {
        Atome,
        Somme,
        Produit,
    }
    let mut expr = variable.clone();
    let mut forme = Forme::Atome;
    for e in &etapes {
        match e {
            Etape::Ajoute(v) => {
                expr = if v.signe() < 0 {
                    format!("{} - {}", expr, v.abs().tex())
                } else {
                    format!("{} + {}", expr, v.tex())
                };
                forme = Forme::Somme;
            }
            Etape::Soustrait(v) | Etape::Retranche(v) => {
                expr = if v.signe() < 0 {
                    format!("{} + {}", expr, v.abs().tex())
                } else {
                    format!("{} - {}", expr, v.tex())
                };
                forme = Forme::Somme;
            }
            Etape::Multiplie(v) => {
                let base = if forme == Forme::Somme {
                    format!("\\left({}\\right)", expr)
                } else {
                    expr.clone()
                };
                expr = format!("{} \\times {}", v.tex(), base);
                forme = Forme::Produit;
            }
            Etape::Divise(v) => {
                expr = format!("\\dfrac{{{}}}{{{}}}", expr, v.tex());
                forme = Forme::Atome;
            }
            Etape::Carre | Etape::Cube => {
                let base = if forme == Forme::Atome {
                    expr.clone()
                } else {
                    format!("\\left({}\\right)", expr)
                };
                let exp = if matches!(e, Etape::Carre) { 2 } else { 3 };
                expr = format!("{}^{{{}}}", base, exp);
                forme = Forme::Atome;
            }
        }
    }
    prose(&[format!(
        "En partant d'un nombre \\({}\\), le programme renvoie \\({}\\).",
        variable, expr
    )])
}

fn lire_trinome(ligne: &str) -> Option<(Q, Q, Q)> {
    let termes = crate::utils::texte::termes_signes(ligne);
    let (mut a, mut b, mut c) = (Q::zero(), Q::zero(), Q::zero());
    for (s, t) in termes {
        let (coef_brut, degre) = if let Some(i) = t.find('x') {
            let suite = t[i + 1..].trim();
            let deg = if suite.is_empty() {
                1
            } else if let Some(exp) = suite.strip_prefix('^') {
                exp.trim().parse::<u32>().ok()?
            } else {
                return None;
            };
            (t[..i].trim().to_string(), deg)
        } else {
            (t.trim().to_string(), 0)
        };
        let coef = if coef_brut.is_empty() {
            Q::ent(1)
        } else {
            rationnel(coef_brut.trim_end_matches(|c: char| c == '*' || c.is_whitespace()))?
        };
        let coef = if s < 0 { coef.neg() } else { coef };
        match degre {
            2 => a = a.add(coef),
            1 => b = b.add(coef),
            0 => c = c.add(coef),
            _ => return None,
        }
    }
    if a.est_nul() {
        return None;
    }
    Some((a, b, c))
}

fn terme_en_x(coef: Q, puissance: &str, premier: bool) -> String {
    if coef.est_nul() {
        return String::new();
    }
    let corps = if coef.abs() == Q::ent(1) && !puissance.is_empty() {
        puissance.to_string()
    } else if puissance.is_empty() {
        coef.abs().tex()
    } else {
        format!("{}{}", coef.abs().tex(), puissance)
    };
    if premier {
        if coef.signe() < 0 {
            format!("-{}", corps)
        } else {
            corps
        }
    } else if coef.signe() < 0 {
        format!(" - {}", corps)
    } else {
        format!(" + {}", corps)
    }
}

fn trinome_tex(a: Q, b: Q, c: Q) -> String {
    let mut out = terme_en_x(a, "x^{2}", true);
    out.push_str(&terme_en_x(b, "x", out.is_empty()));
    out.push_str(&terme_en_x(c, "", out.is_empty()));
    if out.is_empty() {
        out.push('0');
    }
    out
}

fn racine_carree_exacte(q: Q) -> Option<Q> {
    if q.signe() < 0 {
        return None;
    }
    let rn = isqrt(q.n);
    let rd = isqrt(q.d);
    if rn * rn == q.n && rd * rd == q.d {
        Some(Q::new(rn, rd))
    } else {
        None
    }
}

fn isqrt(n: i128) -> i128 {
    if n < 2 {
        return n.max(0);
    }
    let mut x = (n as f64).sqrt() as i128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn facteur_dominant(a: Q) -> String {
    if a == Q::ent(1) {
        String::new()
    } else if a == Q::ent(-1) {
        "-".into()
    } else {
        format!("{}\\,", a.tex())
    }
}

fn facteur_racine(r: Q) -> String {
    if r.est_nul() {
        "x".into()
    } else if r.signe() < 0 {
        format!("\\left(x + {}\\right)", r.abs().tex())
    } else {
        format!("\\left(x - {}\\right)", r.tex())
    }
}

fn forme_canonique(a: Q, alpha: Q, beta: Q) -> String {
    let carre = if alpha.est_nul() {
        "x^{2}".to_string()
    } else {
        format!("{}^{{2}}", facteur_racine(alpha))
    };
    let mut out = format!("{}{}", facteur_dominant(a), carre);
    if !beta.est_nul() {
        if beta.signe() < 0 {
            out.push_str(&format!(" - {}", beta.abs().tex()));
        } else {
            out.push_str(&format!(" + {}", beta.tex()));
        }
    }
    out
}

fn second_degre(corps: &str) -> Option<String> {
    let mut lignes = Vec::new();
    for ligne in corps.lines() {
        let l = ligne.trim();
        if l.is_empty() {
            continue;
        }
        let (a, b, c) = lire_trinome(l)?;
        lignes.push(format!("Soit \\(P(x) = {}\\).", trinome_tex(a, b, c)));
        lignes.push("Le discriminant vaut \\(\\Delta = b^{2} - 4ac\\).".to_string());
        let delta = b.mul(b).sub(Q::ent(4).mul(a).mul(c));
        lignes.push(format!(
            "Ici, \\(\\Delta = {}^{{2}} - 4 \\times {} \\times {} = {}\\).",
            b.tex_paren(),
            a.tex_paren(),
            c.tex_paren(),
            delta.tex()
        ));
        let alpha = b.neg().div(Q::ent(2).mul(a));
        let beta = c.sub(b.mul(b).div(Q::ent(4).mul(a)));
        if delta.signe() > 0 {
            if let Some(s) = racine_carree_exacte(delta) {
                let deux_a = Q::ent(2).mul(a);
                let mut r1 = b.neg().sub(s).div(deux_a);
                let mut r2 = b.neg().add(s).div(deux_a);
                if r1.approx() > r2.approx() {
                    std::mem::swap(&mut r1, &mut r2);
                }
                lignes.push(format!(
                    "Comme \\(\\Delta > 0\\), le trinôme admet deux racines : \\(x_1 = {}\\) et \\(x_2 = {}\\).",
                    r1.tex(),
                    r2.tex()
                ));
                lignes.push(format!(
                    "Il se factorise en \\(P(x) = {}{}{}\\).",
                    facteur_dominant(a),
                    facteur_racine(r1),
                    facteur_racine(r2)
                ));
            } else {
                let deux_a = Q::ent(2).mul(a);
                let g = (b.neg().approx() - delta.approx().sqrt()) / deux_a.approx();
                let h = (b.neg().approx() + delta.approx().sqrt()) / deux_a.approx();
                let (g, h) = if g <= h { (g, h) } else { (h, g) };
                lignes.push(format!(
                    "Comme \\(\\Delta > 0\\), le trinôme admet deux racines : \\(x_{{1,2}} = \\dfrac{{{} \\pm \\sqrt{{{}}}}}{{{}}}\\), soit environ \\({}\\) et \\({}\\).",
                    b.neg().tex(),
                    delta.tex(),
                    deux_a.tex(),
                    approx_fr(g),
                    approx_fr(h)
                ));
            }
        } else if delta.est_nul() {
            lignes.push(format!(
                "Comme \\(\\Delta = 0\\), le trinôme admet une racine double \\(x_0 = {}\\).",
                alpha.tex()
            ));
            lignes.push(format!(
                "Il se factorise en \\(P(x) = {}{}^{{2}}\\).",
                facteur_dominant(a),
                facteur_racine(alpha)
            ));
        } else {
            lignes.push(format!(
                "Comme \\(\\Delta &lt; 0\\), le trinôme n'admet aucune racine réelle : il garde partout le signe de \\(a = {}\\).",
                a.tex()
            ));
        }
        lignes.push(format!(
            "Sa forme canonique est \\(P(x) = {}\\).",
            forme_canonique(a, alpha, beta)
        ));
    }
    if lignes.is_empty() {
        return None;
    }
    prose(&lignes)
}

fn vecteur(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("vecteur")? + "vecteur".len();
    let nom = desc[i..].split_whitespace().next()?.to_string();
    let ouvre = desc.find('(')?;
    let ferme = desc.rfind(')')?;
    let cellules: Vec<String> = desc[ouvre + 1..ferme]
        .split(';')
        .map(|c| decimales_fr(&crate::utils::notation::to_latex(c.trim())))
        .collect();
    if cellules.is_empty() {
        return None;
    }
    let fleche = if nom.chars().count() > 1 {
        format!("\\overrightarrow{{{}}}", nom)
    } else {
        format!("\\vec{{{}}}", nom)
    };
    calcul(&format!(
        "{}\\begin{{pmatrix}}{}\\end{{pmatrix}}",
        fleche,
        cellules.join(" \\\\ ")
    ))
}

pub(crate) fn decimales_fr(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    for (i, c) in chars.iter().enumerate() {
        if *c == ','
            && i > 0
            && chars[i - 1].is_ascii_digit()
            && chars.get(i + 1).map(|x| x.is_ascii_digit()).unwrap_or(false)
        {
            out.push_str("{,}");
        } else {
            out.push(*c);
        }
    }
    out
}

fn suite(desc: &str, arithmetique: bool) -> Option<String> {
    let bas = desc.to_lowercase();
    let nom = bas
        .find("la suite")
        .map(|i| desc[i + "la suite".len()..].trim())
        .and_then(|r| r.split_whitespace().next())
        .filter(|m| {
            let b = m.to_lowercase();
            !b.starts_with("arithm") && !b.starts_with("géom") && !b.starts_with("geom")
        })
        .unwrap_or("u")
        .to_string();
    let i = bas.find("premier terme")? + "premier terme".len();
    let fin = bas[i..].find(" et").map(|j| i + j).unwrap_or(desc.len());
    let premier = rationnel(&desc[i..fin])?;
    let j = bas.find("raison")? + "raison".len();
    let raison = rationnel(desc[j..].trim().trim_end_matches('.'))?;
    let mut lignes = Vec::new();
    if arithmetique {
        lignes.push(format!(
            "La suite \\({}\\) est arithmétique de premier terme \\({}_0 = {}\\) et de raison \\(r = {}\\).",
            nom,
            nom,
            premier.tex(),
            raison.tex()
        ));
        let mut terme = String::new();
        if !premier.est_nul() || raison.est_nul() {
            terme.push_str(&premier.tex());
        }
        if !raison.est_nul() {
            ajoute_signe(&mut terme, raison.signe() < 0, &coef_devant(raison, "n"));
        }
        lignes.push(format!(
            "Son terme général est \\({}_n = {}\\).",
            nom, terme
        ));
        if raison.signe() > 0 {
            lignes.push("Elle est croissante, et diverge vers \\(+\\infty\\).".into());
        } else if raison.signe() < 0 {
            lignes.push("Elle est décroissante, et diverge vers \\(-\\infty\\).".into());
        } else {
            lignes.push("Elle est constante.".into());
        }
        lignes.push(format!(
            "La somme des \\(n + 1\\) premiers termes vaut \\(S_n = \\dfrac{{(n + 1)\\,({}_0 + {}_n)}}{{2}}\\).",
            nom, nom
        ));
    } else {
        lignes.push(format!(
            "La suite \\({}\\) est géométrique de premier terme \\({}_0 = {}\\) et de raison \\(q = {}\\).",
            nom,
            nom,
            premier.tex(),
            raison.tex()
        ));
        let base = if raison.est_entier() && raison.signe() > 0 {
            raison.tex()
        } else {
            format!("\\left({}\\right)", raison.tex())
        };
        let facteur = if premier == Q::ent(1) {
            String::new()
        } else {
            format!("{} \\times ", premier.tex())
        };
        lignes.push(format!(
            "Son terme général est \\({}_n = {}{}^{{n}}\\).",
            nom, facteur, base
        ));
        let sens = premier.signe();
        let q = raison;
        if premier.est_nul() {
            lignes.push("Elle est nulle.".into());
        } else if q == Q::ent(1) {
            lignes.push("Elle est constante.".into());
        } else if q.est_nul() {
            lignes.push("Elle est nulle à partir du rang 1.".into());
        } else if q.signe() > 0 && q.approx() > 1.0 {
            let (mono, limite) = if sens > 0 {
                ("croissante", "+\\infty")
            } else {
                ("décroissante", "-\\infty")
            };
            lignes.push(format!(
                "Elle est {}. Comme \\(q > 1\\), elle diverge vers \\({}\\).",
                mono, limite
            ));
        } else if q.signe() > 0 {
            let mono = if sens > 0 { "décroissante" } else { "croissante" };
            lignes.push(format!(
                "Elle est {}. Comme \\(|q| &lt; 1\\), elle converge vers 0.",
                mono
            ));
        } else if q.approx() > -1.0 {
            lignes.push(
                "Elle n'est pas monotone. Comme \\(|q| &lt; 1\\), elle converge vers 0.".into(),
            );
        } else {
            lignes.push("Elle n'est pas monotone, et ne converge pas.".into());
        }
        if q == Q::ent(1) {
            lignes.push(format!(
                "La somme des \\(n + 1\\) premiers termes vaut \\(S_n = {}\\,(n + 1)\\).",
                premier.tex()
            ));
        } else {
            let facteur_somme = if premier == Q::ent(1) {
                String::new()
            } else {
                format!("{}\\,", premier.tex())
            };
            lignes.push(format!(
                "La somme des \\(n + 1\\) premiers termes vaut \\(S_n = {}\\dfrac{{1 - {}^{{n+1}}}}{{1 - {}}}\\).",
                facteur_somme, base, raison.tex()
            ));
        }
    }
    prose(&lignes)
}

fn congruence(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("congruence")? + "congruence".len();
    let expr = desc[i..].trim().trim_end_matches('.');
    let (gauche, reste) = expr
        .split_once('≡')
        .or_else(|| expr.split_once('='))?;
    let (milieu, module) = reste.split_once('[')?;
    let n = entier(module.trim_end_matches(']'))?;
    let b = entier(milieu)?;
    let gauche = gauche.trim();
    let coef = gauche.trim_end_matches(|c: char| c.is_alphabetic()).trim();
    let variable: String = gauche
        .chars()
        .rev()
        .take_while(|c| c.is_alphabetic())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    let variable = if variable.is_empty() { "x".to_string() } else { variable };
    let a = match coef {
        "" => 1,
        "-" => -1,
        c => entier(c)?,
    };
    if n <= 0 {
        return None;
    }
    let lhs = |a: i128| -> String {
        match a {
            1 => variable.clone(),
            -1 => format!("-{}", variable),
            a => format!("{}{}", a, variable),
        }
    };
    let mut lignes = vec![format!(
        "On cherche les entiers \\({}\\) tels que \\({} \\equiv {} \\pmod{{{}}}\\).",
        variable,
        lhs(a),
        b,
        n
    )];
    let d = pgcd(a, n);
    let (mut a2, mut b2, mut n2) = (a, b, n);
    if b % d != 0 {
        lignes.push(format!(
            "Le PGCD de {} et {} vaut {}, qui ne divise pas {} : la congruence n'a aucune solution.",
            a.abs(),
            n,
            d,
            b
        ));
        return prose(&lignes);
    }
    if d > 1 {
        a2 = a / d;
        b2 = b / d;
        n2 = n / d;
        lignes.push(format!(
            "Le PGCD de {} et {} vaut {}, qui divise {} : on simplifie en \\({} \\equiv {} \\pmod{{{}}}\\).",
            a.abs(),
            n,
            d,
            b,
            lhs(a2),
            b2,
            n2
        ));
    }
    let a_mod = a2.rem_euclid(n2);
    let x0;
    if a_mod == 1 {
        x0 = b2.rem_euclid(n2);
        if d <= 1 {
            lignes.push(format!(
                "Le coefficient de \\({}\\) vaut 1, d'où \\({} \\equiv {} \\pmod{{{}}}\\).",
                variable, variable, x0, n2
            ));
        } else {
            lignes.push(format!(
                "D'où \\({} \\equiv {} \\pmod{{{}}}\\).",
                variable, x0, n2
            ));
        }
    } else {
        let inv = (1..n2).find(|t| (a_mod * t).rem_euclid(n2) == 1)?;
        x0 = (b2.rem_euclid(n2) * inv).rem_euclid(n2);
        lignes.push(format!(
            "Un inverse de {} modulo {} est {}, d'où \\({} \\equiv {} \\pmod{{{}}}\\).",
            a_mod, n2, inv, variable, x0, n2
        ));
    }
    lignes.push(format!(
        "Les solutions sont donc les entiers \\({} = {} + {}k\\), soit {}, {}, {}, etc.",
        variable,
        x0,
        n2,
        x0,
        x0 + n2,
        x0 + 2 * n2
    ));
    prose(&lignes)
}

fn groupe_zn(desc: &str) -> Option<(i128, bool)> {
    let bas = desc.replace(' ', "");
    let i = bas.find("Z/")?;
    let fin = bas[i + 2..].find('Z')? + i + 2;
    let n: i128 = bas[i + 2..fin].parse().ok()?;
    let etoile = bas[fin..].contains('*');
    if n < 2 {
        return None;
    }
    Some((n, etoile))
}

fn cayley(desc: &str) -> Option<String> {
    let (n, etoile) = groupe_zn(desc)?;
    let elements: Vec<i128> = if etoile {
        (1..n).filter(|k| pgcd(*k, n) == 1).collect()
    } else {
        (0..n).collect()
    };
    if elements.len() > 24 {
        return None;
    }
    let op = if etoile { "\\times" } else { "+" };
    let spec = format!("c|{}", "c".repeat(elements.len()));
    let entete = format!(
        "{} & {}",
        op,
        elements
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" & ")
    );
    let mut rangs = Vec::new();
    for x in &elements {
        let cellules: Vec<String> = elements
            .iter()
            .map(|y| {
                if etoile {
                    (x * y).rem_euclid(n).to_string()
                } else {
                    (x + y).rem_euclid(n).to_string()
                }
            })
            .collect();
        rangs.push(format!("{} & {}", x, cellules.join(" & ")));
    }
    calcul(&format!(
        "\\begin{{array}}{{{}}}{} \\\\ \\hline {}\\end{{array}}",
        spec,
        entete,
        rangs.join(" \\\\ ")
    ))
}

fn plainte_graphe(nom: &str) -> Option<String> {
    prose(&[format!("Le graphe {} n'a pas été construit.", nom)])
}

fn adjacence_du_graphe<'a>(nom: &str, env: &'a Env) -> Option<(&'a [String], Vec<Vec<u128>>)> {
    match env.objects.get(nom) {
        Some(Obj::Graph { sommets, arcs }) => {
            let n = sommets.len();
            let mut m = vec![vec![0u128; n]; n];
            for (a, b) in arcs {
                m[*a][*b] = 1;
            }
            Some((sommets, m))
        }
        _ => None,
    }
}

fn factorielle(n: i128) -> Option<u128> {
    let mut r: u128 = 1;
    for k in 2..=n.max(0) as u128 {
        r = r.checked_mul(k)?;
    }
    Some(r)
}

fn parmi_valeur(desc: &str, cle: &str, divise: bool) -> Option<(i128, i128, u128)> {
    let k = premier_entier_apres(desc, cle)?;
    let n = premier_entier_apres(desc, "parmi")?;
    if k < 0 || n < k {
        return None;
    }
    let mut v: u128 = 1;
    for i in 0..k as u128 {
        v = v.checked_mul(n as u128 - i)?;
        if divise {
            v /= i + 1;
        }
    }
    Some((n, k, v))
}

fn combinaisons(desc: &str) -> Option<String> {
    let (n, k, v) = parmi_valeur(desc, "combinaisons de", true)?;
    calcul(&format!(
        "\\dbinom{{{}}}{{{}}} = \\dfrac{{{}!}}{{{}!\\,({} - {})!}} = {}",
        n, k, n, k, n, k, v
    ))
}

fn arrangements(desc: &str) -> Option<String> {
    let (n, k, v) = parmi_valeur(desc, "arrangements de", false)?;
    calcul(&format!(
        "A_{{{}}}^{{{}}} = \\dfrac{{{}!}}{{({} - {})!}} = {}",
        n, k, n, n, k, v
    ))
}

fn permutations(desc: &str) -> Option<String> {
    let n = premier_entier_apres(desc, "permutations de")?;
    let v = factorielle(n)?;
    calcul(&format!("{}! = {}", n, v))
}

fn pascal(desc: &str) -> Option<String> {
    let n = premier_entier_apres(desc, "ligne")?;
    if !(0..=20).contains(&n) {
        return None;
    }
    let mut rangs = Vec::new();
    let mut ligne: Vec<u128> = vec![1];
    for _ in 0..=n {
        rangs.push(
            ligne
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" \\quad "),
        );
        let mut suivante = vec![1u128];
        for i in 1..ligne.len() {
            suivante.push(ligne[i - 1] + ligne[i]);
        }
        suivante.push(1);
        ligne = suivante;
    }
    calcul(&format!(
        "\\begin{{array}}{{c}}{}\\end{{array}}",
        rangs.join(" \\\\ ")
    ))
}

fn facteurs_premiers(desc: &str) -> Option<String> {
    let n = premier_entier_apres(desc, "de")?;
    if n < 2 {
        return None;
    }
    let mut reste = n;
    let mut facteurs: Vec<(i128, u32)> = Vec::new();
    let mut p = 2;
    while p * p <= reste {
        if reste % p == 0 {
            let mut e = 0;
            while reste % p == 0 {
                reste /= p;
                e += 1;
            }
            facteurs.push((p, e));
        }
        p += 1;
    }
    if reste > 1 {
        facteurs.push((reste, 1));
    }
    if facteurs.len() == 1 && facteurs[0].1 == 1 {
        return prose(&[format!("\\({}\\) est premier, avec 2 diviseurs.", n)]);
    }
    let produit = facteurs
        .iter()
        .map(|(p, e)| {
            if *e == 1 {
                p.to_string()
            } else {
                format!("{}^{{{}}}", p, e)
            }
        })
        .collect::<Vec<_>>()
        .join(" \\times ");
    let diviseurs: u128 = facteurs.iter().map(|(_, e)| (*e as u128) + 1).product();
    prose(&[format!(
        "\\({} = {}\\), soit {} diviseurs.",
        n, produit, diviseurs
    )])
}

fn bezout_tex(u: i128, a: i128, v: i128, b: i128, g: i128) -> String {
    let terme = |c: i128, val: i128| -> String {
        if c < 0 {
            format!("\\left({}\\right) \\times {}", c, val)
        } else {
            format!("{} \\times {}", c, val)
        }
    };
    let premier = if u < 0 {
        format!("{} \\times {}", u, a)
    } else {
        terme(u, a)
    };
    format!("{} + {} = {}", premier, terme(v, b), g)
}

fn euclide(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("euclide à").map(|j| j + "euclide à".len())
        .or_else(|| bas.find("euclide a").map(|j| j + "euclide a".len()))?;
    let (ga, gb) = desc[i..].split_once(" et ")?;
    let a = entier(ga)?;
    let b = entier(gb)?;
    if a <= 0 || b <= 0 {
        return None;
    }
    let mut lignes = Vec::new();
    let (mut x, mut y) = (a.max(b), a.min(b));
    while y != 0 {
        let (q, r) = (x / y, x % y);
        lignes.push(format!("\\({} = {} \\times {} + {}\\)", x, y, q, r));
        x = y;
        y = r;
    }
    let g = x;
    lignes.push(format!(
        "Le dernier reste non nul donne \\(\\operatorname{{PGCD}}({}\\,;\\,{}) = {}\\), d'où \\(\\operatorname{{PPCM}} = {}\\).",
        a,
        b,
        g,
        ppcm(a, b)
    ));
    let (_, u, v) = pgcd_etendu(a, b);
    lignes.push(format!(
        "Une relation de Bézout est \\({}\\).",
        bezout_tex(u, a, v, b, g)
    ));
    prose(&lignes)
}

fn lire_complexe(expr: &str) -> Option<(Q, Q)> {
    let (mut re, mut im) = (Q::zero(), Q::zero());
    let termes = crate::utils::texte::termes_signes(expr);
    if termes.is_empty() {
        return None;
    }
    for (s, t) in termes {
        let imaginaire = t.contains('i');
        let coef_brut: String = t
            .chars()
            .filter(|c| *c != 'i' && *c != '*' && !c.is_whitespace())
            .collect();
        let coef = if coef_brut.is_empty() {
            Q::ent(1)
        } else {
            rationnel(&coef_brut)?
        };
        let coef = if s < 0 { coef.neg() } else { coef };
        if imaginaire {
            im = im.add(coef);
        } else {
            re = re.add(coef);
        }
    }
    Some((re, im))
}

fn complexe_tex(re: Q, im: Q) -> String {
    if re.est_nul() && im.est_nul() {
        return "0".into();
    }
    let mut out = String::new();
    if !re.est_nul() {
        out.push_str(&re.tex());
    }
    if !im.est_nul() {
        ajoute_signe(&mut out, im.signe() < 0, &coef_devant(im, "\\mathrm{i}"));
    }
    out
}

fn coef_devant(q: Q, base: &str) -> String {
    if q.abs() == Q::ent(1) {
        base.to_string()
    } else {
        format!("{}{}", q.abs().tex(), base)
    }
}

fn ajoute_signe(out: &mut String, negatif: bool, corps: &str) {
    if out.is_empty() {
        if negatif {
            out.push('-');
        }
        out.push_str(corps);
    } else {
        out.push_str(if negatif { " - " } else { " + " });
        out.push_str(corps);
    }
}

fn racine_simplifiee(q: Q) -> (Q, i128) {
    let radicande = q.n * q.d;
    let mut dehors: i128 = 1;
    let mut dedans: i128 = radicande;
    let mut p = 2;
    while p * p <= dedans {
        while dedans % (p * p) == 0 {
            dedans /= p * p;
            dehors *= p;
        }
        p += 1;
    }
    (Q::new(dehors, q.d), dedans)
}

fn module_tex(re: Q, im: Q) -> String {
    let carre = re.mul(re).add(im.mul(im));
    let (coef, radicande) = racine_simplifiee(carre);
    if radicande == 1 {
        return coef.tex();
    }
    if coef == Q::ent(1) {
        return format!("\\sqrt{{{}}}", radicande);
    }
    format!("{}\\sqrt{{{}}}", coef.tex(), radicande)
}

fn angle_tex(num: i128, den: i128, affiche: bool) -> String {
    let frac = if affiche { "\\dfrac" } else { "\\frac" };
    match (num, den) {
        (0, _) => "0".into(),
        (1, 1) => "\\pi".into(),
        (-1, 1) => "-\\pi".into(),
        (1, d) => format!("{}{{\\pi}}{{{}}}", frac, d),
        (-1, d) => format!("-{}{{\\pi}}{{{}}}", frac, d),
        (n, d) if n > 0 => format!("{}{{{}\\pi}}{{{}}}", frac, n, d),
        (n, d) => format!("-{}{{{}\\pi}}{{{}}}", frac, -n, d),
    }
}

fn argument(re: Q, im: Q) -> Option<(i128, i128)> {
    let (sa, sb) = (re.signe(), im.signe());
    if sb == 0 {
        return Some(if sa >= 0 { (0, 1) } else { (1, 1) });
    }
    if sa == 0 {
        return Some(if sb > 0 { (1, 2) } else { (-1, 2) });
    }
    if re.abs() == im.abs() {
        return Some(match (sa, sb) {
            (1, 1) => (1, 4),
            (-1, 1) => (3, 4),
            (-1, -1) => (-3, 4),
            _ => (-1, 4),
        });
    }
    None
}

fn complexe(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("complexe")? + "complexe".len();
    let reste = desc[i..].trim();
    let (nom, expr) = reste.split_once('=')?;
    let nom = nom.trim().to_string();
    let expr = match expr.to_lowercase().find(" sous") {
        Some(j) => &expr[..j],
        None => expr,
    };
    let (re, im) = lire_complexe(expr)?;
    if re.est_nul() && im.est_nul() {
        return prose(&[format!(
            "Forme algébrique : \\({} = 0\\). Le complexe nul n'a ni argument ni forme exponentielle.",
            nom
        )]);
    }
    let mut lignes = vec![format!(
        "Forme algébrique : \\({} = {}\\).",
        nom,
        complexe_tex(re, im)
    )];
    let module = module_tex(re, im);
    match argument(re, im) {
        Some((p, q)) => {
            lignes.push(format!(
                "Module et argument : \\(|{}| = {}\\) et \\(\\arg({}) = {}\\).",
                nom,
                module,
                nom,
                angle_tex(p, q, true)
            ));
            let theta = angle_tex(p, q, true);
            let theta_paren = if p < 0 {
                format!("\\left({}\\right)", theta)
            } else {
                theta.clone()
            };
            let prefixe = if module == "1" {
                String::new()
            } else {
                format!("{}\\,", module)
            };
            lignes.push(format!(
                "Forme trigonométrique : \\({} = {}\\left(\\cos {} + \\mathrm{{i}}\\,\\sin {}\\right)\\).",
                nom, prefixe, theta_paren, theta_paren
            ));
            let expo = angle_tex(p, q, false);
            let exposant = if p < 0 {
                format!("-\\mathrm{{i}}{}", angle_tex(-p, q, false))
            } else {
                format!("\\mathrm{{i}}{}", expo)
            };
            lignes.push(format!(
                "Forme exponentielle : \\({} = {}\\mathrm{{e}}^{{{}}}\\).",
                nom, prefixe, exposant
            ));
        }
        None => {
            let theta = im.approx().atan2(re.approx());
            lignes.push(format!(
                "Module et argument : \\(|{}| = {}\\) et \\(\\arg({}) \\approx {}\\).",
                nom,
                module,
                nom,
                approx_fr(theta)
            ));
        }
    }
    prose(&lignes)
}

fn graphe(desc: &str, corps: &str, env: &mut Env) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("graphe")? + "graphe".len();
    let nom = desc[i..].split_whitespace().next().unwrap_or("G").to_string();
    let mut sommets: Vec<String> = Vec::new();
    let mut arcs: Vec<(usize, usize)> = Vec::new();
    let indice = |sommets: &mut Vec<String>, s: &str| -> usize {
        match sommets.iter().position(|x| x == s) {
            Some(i) => i,
            None => {
                sommets.push(s.to_string());
                sommets.len() - 1
            }
        }
    };
    for ligne in corps.lines() {
        let l = ligne.trim();
        if l.is_empty() {
            continue;
        }
        let (de, vers) = l.split_once("->")?;
        let a = indice(&mut sommets, de.trim());
        let b = indice(&mut sommets, vers.trim());
        arcs.push((a, b));
    }
    if sommets.is_empty() {
        return None;
    }
    let svg = dessine_graphe(&sommets, &arcs);
    env.objects.insert(
        nom,
        Obj::Graph {
            sommets,
            arcs,
        },
    );
    Some(svg)
}

fn dessine_graphe(sommets: &[String], arcs: &[(usize, usize)]) -> String {
    let n = sommets.len();
    let (cx, cy, rayon, r_noeud) = (85.0f32, 85.0f32, 58.0f32, 13.0f32);
    let pos: Vec<(f32, f32)> = (0..n)
        .map(|i| {
            let depart = if n == 2 {
                0.0
            } else {
                -std::f32::consts::FRAC_PI_2
            };
            let angle = depart + i as f32 * 2.0 * std::f32::consts::PI / n.max(1) as f32;
            (cx + rayon * angle.cos(), cy + rayon * angle.sin())
        })
        .collect();
    let mut corps = String::from(
        "<defs><marker id=\"flg\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" \
         markerWidth=\"7\" markerHeight=\"7\" orient=\"auto-start-reverse\">\
         <path d=\"M 0 0 L 10 5 L 0 10 z\" fill=\"#333\"/></marker></defs>",
    );
    for (a, b) in arcs {
        if a == b {
            continue;
        }
        let (x1, y1) = pos[*a];
        let (x2, y2) = pos[*b];
        let (dx, dy) = (x2 - x1, y2 - y1);
        let long = (dx * dx + dy * dy).sqrt().max(1.0);
        let (ux, uy) = (dx / long, dy / long);
        let double = arcs.contains(&(*b, *a));
        let (ox, oy) = if double { (uy * 4.0, -ux * 4.0) } else { (0.0, 0.0) };
        let (sx, sy) = (x1 + ux * (r_noeud + 2.0) + ox, y1 + uy * (r_noeud + 2.0) + oy);
        let (ex, ey) = (x2 - ux * (r_noeud + 5.0) + ox, y2 - uy * (r_noeud + 5.0) + oy);
        corps.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
             stroke=\"#333\" stroke-width=\"1.3\" marker-end=\"url(#flg)\"/>",
            sx, sy, ex, ey
        ));
    }
    for (i, nom) in sommets.iter().enumerate() {
        let (x, y) = pos[i];
        corps.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"white\" \
             stroke=\"#333\" stroke-width=\"1.2\"/>\
             <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" dy=\"0.35em\" \
             font-size=\"13\" font-style=\"italic\" fill=\"#333\">{}</text>",
            x, y, r_noeud, x, y, nom
        ));
    }
    let marge = 6.0f32;
    let borne = |f: fn(f32, f32) -> f32, axe: fn(&(f32, f32)) -> f32, depart: f32| {
        pos.iter().map(axe).fold(depart, f)
    };
    let x0 = borne(f32::min, |p| p.0, f32::INFINITY) - r_noeud - marge;
    let x1 = borne(f32::max, |p| p.0, f32::NEG_INFINITY) + r_noeud + marge;
    let y0 = borne(f32::min, |p| p.1, f32::INFINITY) - r_noeud - marge;
    let y1 = borne(f32::max, |p| p.1, f32::NEG_INFINITY) + r_noeud + marge;
    let (largeur, hauteur) = (x1 - x0, y1 - y0);
    format!(
        "<div class=\"calcul\"><svg width=\"{:.0}\" height=\"{:.0}\" \
         viewBox=\"{:.1} {:.1} {:.1} {:.1}\" style=\"display:block\" \
         xmlns=\"http://www.w3.org/2000/svg\">{}</svg></div>",
        largeur, hauteur, x0, y0, largeur, hauteur, corps
    )
}

fn adjacence(desc: &str, env: &Env) -> Option<String> {
    let nom = desc
        .rsplit_once(" de ")
        .map(|(_, r)| r)
        .or_else(|| desc.rsplit_once(" d'").map(|(_, r)| r))?
        .trim()
        .trim_end_matches('.')
        .to_string();
    let (sommets, m) = match adjacence_du_graphe(&nom, env) {
        Some(x) => x,
        None => return plainte_graphe(&nom),
    };
    let corps = m
        .iter()
        .map(|r| {
            r.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" & ")
        })
        .collect::<Vec<_>>()
        .join(" \\\\ ");
    let noms: Vec<String> = sommets.iter().map(|s| format!("\\({}\\)", s)).collect();
    prose(&[format!(
        "La matrice d'adjacence de \\({}\\), les sommets rangés dans l'ordre {}, est \\(M_{{{}}} = \\begin{{pmatrix}}{}\\end{{pmatrix}}\\).",
        nom,
        liste_fr(&noms),
        nom,
        corps
    )])
}

fn chemins(desc: &str, env: &Env) -> Option<String> {
    let long = premier_entier_apres(desc, "longueur")?;
    let apres = crate::utils::texte::apres_cle(desc, "longueur")?;
    let (avant_dans, nom) = apres.rsplit_once(" dans ")?;
    let nom = nom.trim().trim_end_matches('.').to_string();
    let seg = avant_dans.trim().trim_start_matches(|c: char| c.is_ascii_digit() || c.is_whitespace());
    let seg = seg.strip_prefix("de ").unwrap_or(seg);
    let (source, cible) = seg.split_once(" à ").or_else(|| seg.split_once(" a "))?;
    let (source, cible) = (source.trim().to_string(), cible.trim().to_string());
    let (sommets, base) = match adjacence_du_graphe(&nom, env) {
        Some(x) => x,
        None => return plainte_graphe(&nom),
    };
    let i = sommets.iter().position(|s| *s == source)?;
    let j = sommets.iter().position(|s| *s == cible)?;
    let n = sommets.len();
    let mut m = base.clone();
    for _ in 1..long.max(1) {
        let mut suivante = vec![vec![0u128; n]; n];
        for x in 0..n {
            for y in 0..n {
                let mut s = 0u128;
                for k in 0..n {
                    s += m[x][k] * base[k][y];
                }
                suivante[x][y] = s;
            }
        }
        m = suivante;
    }
    let v = if long == 0 { u128::from(i == j) } else { m[i][j] };
    let pluriel = if v > 1 { "s" } else { "" };
    prose(&[format!(
        "Le coefficient de la ligne \\({}\\), colonne \\({}\\) de \\(M_{{{}}}^{{{}}}\\) vaut \\({}\\) : il y a {} chemin{} de longueur {} de \\({}\\) à \\({}\\) dans \\({}\\).",
        source, cible, nom, long, v, v, pluriel, long, source, cible, nom
    )])
}

fn lire_terme_lineaire(t: &str) -> Option<(i128, String)> {
    let t = t.trim();
    let i = t.find(|c: char| c.is_alphabetic())?;
    let variable: String = t[i..].chars().take_while(|c| c.is_alphabetic()).collect();
    let coef = t[..i].trim();
    let c = match coef {
        "" => 1,
        "-" => -1,
        c => entier(c)?,
    };
    Some((c, variable))
}

fn diophantienne(desc: &str) -> Option<String> {
    let bas = desc.to_lowercase();
    let i = bas.find("diophantienne")? + "diophantienne".len();
    let expr = desc[i..].trim().trim_end_matches('.');
    let (gauche, droite) = expr.split_once('=')?;
    let c = entier(droite)?;
    let mut termes: Vec<(i128, String)> = Vec::new();
    let mut signe = 1i128;
    let mut cur = String::new();
    for ch in gauche.trim().chars() {
        match ch {
            '+' | '-' if !cur.trim().is_empty() => {
                let (co, v) = lire_terme_lineaire(&cur)?;
                termes.push((signe * co, v));
                signe = if ch == '-' { -1 } else { 1 };
                cur.clear();
            }
            '-' if cur.trim().is_empty() => signe = -signe,
            '+' if cur.trim().is_empty() => {}
            ch => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        let (co, v) = lire_terme_lineaire(&cur)?;
        termes.push((signe * co, v));
    }
    if termes.len() != 2 {
        return None;
    }
    let (a, vx) = termes[0].clone();
    let (b, vy) = termes[1].clone();
    let terme_tex = |c: i128, v: &str, premier: bool| -> String {
        let corps = match c.abs() {
            1 => v.to_string(),
            m => format!("{}{}", m, v),
        };
        if premier {
            if c < 0 {
                format!("-{}", corps)
            } else {
                corps
            }
        } else if c < 0 {
            format!(" - {}", corps)
        } else {
            format!(" + {}", corps)
        }
    };
    let mut lignes = vec![format!(
        "On cherche les couples d'entiers \\(({}\\,;\\,{})\\) tels que \\({}{} = {}\\).",
        vx,
        vy,
        terme_tex(a, &vx, true),
        terme_tex(b, &vy, false),
        c
    )];
    let d = pgcd(a, b);
    if d == 0 {
        return None;
    }
    if c % d != 0 {
        lignes.push(format!(
            "Comme \\(\\operatorname{{PGCD}}({}\\,;\\,{}) = {}\\) ne divise pas {}, l'équation n'a aucune solution entière.",
            a.abs(),
            b.abs(),
            d,
            c
        ));
        return prose(&lignes);
    }
    lignes.push(format!(
        "\\(\\operatorname{{PGCD}}({}\\,;\\,{}) = {}\\) divise {} : l'équation a des solutions.",
        a.abs(),
        b.abs(),
        d,
        c
    ));
    let (_, u0, v0) = pgcd_etendu(a.abs(), b.abs());
    let (u, v) = (u0 * a.signum(), v0 * b.signum());
    let (x0, y0) = (u * (c / d), v * (c / d));
    lignes.push(format!(
        "La relation de Bézout \\({}\\) donne la solution particulière \\(({}_0\\,;\\,{}_0) = ({}\\,;\\,{})\\).",
        bezout_tex(u, a, v, b, d),
        vx,
        vy,
        x0,
        y0
    ));
    let (pas_x, pas_y) = (b / d, -a / d);
    let solution = |base: i128, pas: i128| -> String {
        let mut s = String::new();
        if base != 0 || pas == 0 {
            s.push_str(&base.to_string());
        }
        if pas != 0 {
            let corps = match pas.abs() {
                1 => "k".to_string(),
                m => format!("{}k", m),
            };
            if s.is_empty() {
                if pas < 0 {
                    s.push_str(&format!("-{}", corps));
                } else {
                    s.push_str(&corps);
                }
            } else if pas < 0 {
                s.push_str(&format!(" - {}", corps));
            } else {
                s.push_str(&format!(" + {}", corps));
            }
        }
        s
    };
    lignes.push(format!(
        "Les solutions sont les couples \\(({}\\,;\\,{})\\), \\(k \\in \\mathbb{{Z}}\\).",
        solution(x0, pas_x),
        solution(y0, pas_y)
    ));
    prose(&lignes)
}