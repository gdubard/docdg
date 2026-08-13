//! Le parcours des délimiteurs, en un seul endroit.
//!
//! Cinq fonctions du moteur appariaient des accolades ou coupaient une chaîne
//! au premier niveau, chacune dans son fichier et avec ses propres règles :
//! l'une comptait les crochets, l'autre non ; l'une respectait les guillemets,
//! l'autre les traversait ; l'une tolérait une fermante orpheline, les autres
//! non. Un défaut corrigé dans l'une ne l'était pas dans les autres.
//!
//! Ces différences ne sont pas des accidents : découper les arguments d'un
//! appel et découper les cellules d'une rangée ne demandent pas le même jeu de
//! délimiteurs. Ce qui devait cesser, c'est de réécrire le parcours à chaque
//! fois. Le parcours est donc unique, et chaque emploi porte un nom qui dit
//! ses règles — plutôt qu'une liste de drapeaux à déchiffrer sur place.

/// Les délimiteurs qui font profondeur.
#[derive(Clone, Copy, PartialEq)]
enum Delimiteurs {
    /// Accolades et parenthèses seulement.
    AccoladesEtParentheses,
    /// Accolades, parenthèses et crochets — le jeu complet.
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
    /// Une portion citée est traversée sans être lue.
    guillemets: bool,
    /// Une fermante orpheline fait passer la profondeur sous zéro. Coupe-t-on
    /// quand même ? Les fragments d'une ligne le font — une accolade
    /// dépareillée dans une cellule ne doit pas avaler le reste de la rangée.
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

/// Les arguments d'un appel, coupés sur les points-virgules de premier niveau,
/// **guillemets exceptés** : dans `jonction(v ; " ; ")`, le second
/// point-virgule est une valeur, non une coupure.
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

/// Les éléments d'un littéral de conteneur. Le crochet n'y fait pas
/// profondeur : il ouvre une lecture par indice, pas un groupe.
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

/// Les fragments d'une ligne de document — cellules d'une rangée, morceaux
/// d'une prose. La parenthèse compte au même titre que l'accolade et le
/// crochet : sans elle, le point-virgule d'un appel — `contient(2 ; v)` —
/// était pris pour un séparateur de colonne et coupait la cellule en deux.
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

/// La position du délimiteur qui referme celui ouvert en tête.
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

/// Le contenu d'un groupe entre accolades, et ce qui le suit. `ouverture` doit
/// désigner l'accolade ouvrante ; rien n'est rendu si elle n'est pas là, ou si
/// le groupe n'est jamais refermé.
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
        // ailleurs, le point-virgule cité coupe comme les autres : c'est le
        // comportement d'avant, et des documents en dépendent
        assert_eq!(coupe_elements("v ; \" ; \"", ';').len(), 3);
    }

    #[test]
    fn le_crochet_ne_compte_pas_pour_les_elements() {
        assert_eq!(coupe_arguments("[1 ; 2] ; 3").len(), 2);
        assert_eq!(coupe_elements("[1 ; 2] ; 3", ';').len(), 3);
    }

    #[test]
    fn seuls_les_fragments_tolerent_une_fermante_orpheline() {
        // une accolade dépareillée ne doit pas avaler le reste de la rangée
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
        // « é » pèse deux octets et « 🎓 » quatre : la parenthèse fermante
        // tombe donc à l'octet 7, non au septième caractère.
        assert_eq!(groupe_apparie("(é🎓)", '(', ')'), Some(7));
        assert_eq!(coupe_elements("é ; 🎓", ';'), vec!["é ", " 🎓"]);
    }
}
