use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
enum T {
    Num(f64),
    Id(String),
    Op(char),
    LPar,
    RPar,
    Semi,
}

fn norm_fn(name: &str) -> String {
    let l = name.to_lowercase();
    match l.as_str() {
        "défaut" | "defaut" => "floor".into(),
        "excès" | "exces" => "ceil".into(),
        "arrondi" => "round".into(),
        "racine" => "sqrt".into(),
        "valeurabsolue" | "abs" => "abs".into(),
        _ => l,
    }
}

fn lex(s: &str) -> Option<Vec<T>> {
    let cs: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() {
            let mut n = String::new();
            while i < cs.len() && cs[i].is_ascii_digit() {
                n.push(cs[i]);
                i += 1;
            }
            if i + 1 < cs.len() && (cs[i] == ',' || cs[i] == '.') && cs[i + 1].is_ascii_digit() {
                n.push('.');
                i += 1;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    n.push(cs[i]);
                    i += 1;
                }
            }
            out.push(T::Num(n.parse().ok()?));
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let mut w = String::new();
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
                w.push(cs[i]);
                i += 1;
            }
            out.push(T::Id(w));
            continue;
        }
        match c {
            '+' | '-' | '*' | '/' | '^' | '%' => out.push(T::Op(c)),
            '(' => out.push(T::LPar),
            ')' => out.push(T::RPar),
            ';' => out.push(T::Semi),
            _ => return None,
        }
        i += 1;
    }
    Some(out)
}

const PROFONDEUR_MAX: u32 = 64;

struct E<'a> {
    t: Vec<T>,
    i: usize,
    profondeur: u32,
    vars: &'a BTreeMap<String, f64>,
}

impl<'a> E<'a> {
    fn peek(&self) -> Option<&T> {
        self.t.get(self.i)
    }

    fn expr(&mut self) -> Option<f64> {
        self.profondeur += 1;
        if self.profondeur > PROFONDEUR_MAX {
            return None;
        }
        let mut v = self.term()?;
        loop {
            match self.peek() {
                Some(T::Op('+')) => {
                    self.i += 1;
                    v += self.term()?;
                }
                Some(T::Op('-')) => {
                    self.i += 1;
                    v -= self.term()?;
                }
                _ => {
                    self.profondeur -= 1;
                    return Some(v);
                }
            }
        }
    }

    fn term(&mut self) -> Option<f64> {
        let mut v = self.factor()?;
        loop {
            match self.peek() {
                Some(T::Op('*')) => {
                    self.i += 1;
                    v *= self.factor()?;
                }
                Some(T::Op('/')) => {
                    self.i += 1;
                    let d = self.factor()?;
                    if d == 0.0 {
                        return None;
                    }
                    v /= d;
                }
                Some(T::Op('%')) => {
                    self.i += 1;
                    let d = self.factor()?;
                    if d == 0.0 {
                        return None;
                    }
                    v = v.rem_euclid(d);
                }
                _ => return Some(v),
            }
        }
    }

    fn factor(&mut self) -> Option<f64> {
        if self.profondeur > PROFONDEUR_MAX {
            return None;
        }
        match self.peek() {
            Some(T::Op('-')) => {
                self.i += 1;
                Some(-self.factor()?)
            }
            Some(T::Op('+')) => {
                self.i += 1;
                self.factor()
            }
            _ => {
                let b = self.atom()?;
                if let Some(T::Op('^')) = self.peek() {
                    self.i += 1;
                    let e = self.factor()?;
                    Some(b.powf(e))
                } else {
                    Some(b)
                }
            }
        }
    }

    fn args(&mut self) -> Option<Vec<f64>> {
        let mut out = Vec::new();
        if matches!(self.peek(), Some(T::RPar)) {
            self.i += 1;
            return Some(out);
        }
        loop {
            out.push(self.expr()?);
            match self.peek() {
                Some(T::Semi) => {
                    self.i += 1;
                }
                Some(T::RPar) => {
                    self.i += 1;
                    return Some(out);
                }
                _ => return None,
            }
        }
    }

    fn atom(&mut self) -> Option<f64> {
        match self.peek().cloned() {
            Some(T::Num(n)) => {
                self.i += 1;
                Some(n)
            }
            Some(T::LPar) => {
                self.i += 1;
                let v = self.expr()?;
                if matches!(self.peek(), Some(T::RPar)) {
                    self.i += 1;
                    Some(v)
                } else {
                    None
                }
            }
            Some(T::Id(w)) => {
                self.i += 1;
                if matches!(self.peek(), Some(T::LPar)) {
                    self.i += 1;
                    let a = self.args()?;
                    let f = norm_fn(&w);
                    match (f.as_str(), a.len()) {
                        ("floor", 1) => Some(a[0].floor()),
                        ("ceil", 1) => Some(a[0].ceil()),
                        ("round", 1) => Some(round_half_up(a[0], 0)),
                        ("round", 2) => Some(round_half_up(a[0], a[1] as i32)),
                        ("sqrt", 1) => Some(a[0].sqrt()),
                        ("abs", 1) => Some(a[0].abs()),
                        ("min", 2) => Some(a[0].min(a[1])),
                        ("max", 2) => Some(a[0].max(a[1])),
                        ("cos", 1) => Some(a[0].cos()),
                        ("sin", 1) => Some(a[0].sin()),
                        ("tan", 1) => Some(a[0].tan()),
                        ("exp", 1) => Some(a[0].exp()),
                        ("ln", 1) => Some(a[0].ln()),
                        ("log", 1) => Some(a[0].log10()),
                        ("pgcd", 2) => Some(gcd(a[0].round() as i64, a[1].round() as i64) as f64),
                        ("ppcm", 2) => {
                            let g = gcd(a[0].round() as i64, a[1].round() as i64);
                            if g == 0 {
                                None
                            } else {
                                Some((a[0].round() as i64 / g * a[1].round() as i64).abs() as f64)
                            }
                        }
                        _ => None,
                    }
                } else if w == "pi" {
                    Some(std::f64::consts::PI)
                } else {
                    self.vars.get(&w).copied()
                }
            }
            _ => None,
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn round_half_up(v: f64, n: i32) -> f64 {
    let p = 10f64.powi(n);
    (v * p + 0.5f64.copysign(v)).trunc() / p
}

pub fn eval(src: &str, vars: &BTreeMap<String, f64>) -> Option<f64> {
    let toks = lex(src)?;
    if toks.is_empty() {
        return None;
    }
    let mut e = E {
        t: toks,
        i: 0,
        profondeur: 0,
        vars,
    };
    let v = e.expr()?;
    if e.i == e.t.len() && v.is_finite() {
        Some(v)
    } else {
        None
    }
}

pub fn format_number(v: f64) -> String {
    if !v.is_finite() {
        return "…".into();
    }
    let precision = crate::layout::rendu::reglages_page().precision;
    if precision >= 0 {
        let f = 10f64.powi(precision);
        let r = (v * f).round() / f;
        return if precision == 0 {
            format!("{}", r.round() as i64)
        } else {
            format!("{:.*}", precision as usize, r).replace('.', ",")
        };
    }
    let r = (v * 1e9).round() / 1e9;
    if (r - r.round()).abs() < 1e-9 {
        format!("{}", r.round() as i64)
    } else {
        let mut s = format!("{:.9}", r);
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
        s.replace('.', ",")
    }
}

pub fn eval_display(src: &str, vars: &BTreeMap<String, f64>) -> String {
    match eval(src, vars) {
        Some(v) => format_number(v),
        None => "…".into(),
    }
}
