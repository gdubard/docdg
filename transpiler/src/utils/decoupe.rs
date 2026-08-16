#[derive(Clone, Copy, PartialEq)]
enum Delimiteurs {

    AccoladesEtParentheses,

    Toutes,
}

impl Delimiteurs {
    fn variation(self, c: char) -> i32 {
        match self {
            Delimiteurs::AccoladesEtParentheses => match c {
                '{' | '(' => 1,
                '}' | ')' => -1,
                _ => 0,
            },
            Delimiteurs::Toutes => match c {
                '{' | '(' | '[' => 1,
                '}' | ')' | ']' => -1,
                _ => 0,
            },
        }
    }
}

struct Regles {
    delim: Delimiteurs,

    guillemets: bool,

    tolere_le_deficit: bool,
}

fn coupe(s: &str, sep: char, r: Regles) -> Vec<&str> {
    let mut morceaux = Vec::new();
    let mut profondeur = 0i32;
    let mut debut = 0usize;
    let mut cite = false;
    for (i, c) in s.char_indices() {
        if r.guillemets && c == '"' {
            cite = !cite;
            continue;
        }
        if cite {
            continue;
        }
        let ouvert = if r.tolere_le_deficit {
            profondeur > 0
        } else {
            profondeur != 0
        };
        if c == sep && !ouvert {
            morceaux.push(&s[debut..i]);
            debut = i + c.len_utf8();
            continue;
        }
        profondeur += r.delim.variation(c);
    }
    morceaux.push(&s[debut..]);
    morceaux
}

pub fn coupe_arguments(s: &str) -> Vec<&str> {
    coupe(
        s,
        ';',
        Regles {
            delim: Delimiteurs::Toutes,
            guillemets: true,
            tolere_le_deficit: false,
        },
    )
}

pub fn coupe_elements(s: &str, sep: char) -> Vec<&str> {
    coupe(
        s,
        sep,
        Regles {
            delim: Delimiteurs::AccoladesEtParentheses,
            guillemets: false,
            tolere_le_deficit: false,
        },
    )
}

pub fn coupe_fragments(s: &str, sep: char) -> Vec<&str> {
    coupe(
        s,
        sep,
        Regles {
            delim: Delimiteurs::Toutes,
            guillemets: false,
            tolere_le_deficit: true,
        },
    )
}

pub fn groupe_apparie(s: &str, ouvre: char, ferme: char) -> Option<usize> {
    let mut profondeur = 0i32;
    for (i, c) in s.char_indices() {
        if c == ouvre {
            profondeur += 1;
        } else if c == ferme {
            profondeur -= 1;
            if profondeur == 0 {
                return Some(i);
            }
        }
    }
    None
}

pub fn prend_accolades(s: &str, ouverture: usize) -> Option<(&str, &str)> {
    if !s[ouverture..].starts_with('{') {
        return None;
    }
    let fin = ouverture + groupe_apparie(&s[ouverture..], '{', '}')?;
    Some((&s[ouverture + 1..fin], &s[fin + 1..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_groupe_apparie_saute_les_groupes_internes() {
        assert_eq!(groupe_apparie("(a(b)c)d", '(', ')'), Some(6));
        assert_eq!(groupe_apparie("(sans fin", '(', ')'), None);
    }

    #[test]
    fn la_coupe_respecte_la_profondeur() {
        assert_eq!(
            coupe_arguments("a ; f(1 ; 2) ; b"),
            vec!["a ", " f(1 ; 2) ", " b"]
        );
    }

    #[test]
    fn les_guillemets_ne_protegent_que_les_arguments() {
        assert_eq!(coupe_arguments("v ; \" ; \""), vec!["v ", " \" ; \""]);

        assert_eq!(coupe_elements("v ; \" ; \"", ';').len(), 3);
    }

    #[test]
    fn le_crochet_ne_compte_pas_pour_les_elements() {
        assert_eq!(coupe_arguments("[1 ; 2] ; 3").len(), 2);
        assert_eq!(coupe_elements("[1 ; 2] ; 3", ';').len(), 3);
    }

    #[test]
    fn seuls_les_fragments_tolerent_une_fermante_orpheline() {

        assert_eq!(coupe_fragments("a} ; b", ';').len(), 2);
        assert_eq!(coupe_arguments("a} ; b").len(), 1);
    }

    #[test]
    fn les_accolades_rendent_le_corps_et_la_suite() {
        assert_eq!(prend_accolades("{a{b}c}reste", 0), Some(("a{b}c", "reste")));
        assert_eq!(prend_accolades("pas d'accolade", 0), None);
        assert_eq!(prend_accolades("{jamais refermé", 0), None);
    }

    #[test]
    fn le_parcours_tient_devant_l_utf8() {

        assert_eq!(groupe_apparie("(é🎓)", '(', ')'), Some(7));
        assert_eq!(coupe_elements("é ; 🎓", ';'), vec!["é ", " 🎓"]);
    }
}
