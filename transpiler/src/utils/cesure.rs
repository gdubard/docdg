use hyphenation::{Hyphenator, Language, Load, Standard};
use std::sync::OnceLock;

const DICO_FR: &[u8] = include_bytes!("fr.standard.bincode");
const CESURE: char = '\u{00AD}';
const LONGUEUR_MINIMALE: usize = 6;

fn dictionnaire() -> Option<&'static Standard> {
    static DICO: OnceLock<Option<Standard>> = OnceLock::new();
    DICO.get_or_init(|| Standard::from_reader(Language::French, &mut &DICO_FR[..]).ok())
        .as_ref()
}

fn coupe_mot(mot: &str, sortie: &mut String) {
    let dico = match dictionnaire() {
        Some(d) => d,
        None => {
            sortie.push_str(mot);
            return;
        }
    };
    let lettres = mot.chars().count();
    let majuscules = mot.chars().filter(|c| c.is_uppercase()).count();
    if lettres < LONGUEUR_MINIMALE || majuscules > 1 || mot.contains('-') || mot.contains(CESURE) {
        sortie.push_str(mot);
        return;
    }
    let coupes = dico.hyphenate(mot);
    let mut precedent = 0;
    for &indice in &coupes.breaks {
        sortie.push_str(&mot[precedent..indice]);
        sortie.push(CESURE);
        precedent = indice;
    }
    sortie.push_str(&mot[precedent..]);
}

fn coupe_texte(texte: &str, tete_soudee: bool, queue_soudee: bool, sortie: &mut String) {
    let mut tete = tete_soudee;
    let mut mot = String::new();
    let mut curseur = texte.chars().peekable();
    while let Some(c) = curseur.next() {
        if c.is_alphabetic() || c == '-' || c == CESURE {
            mot.push(c);
            if curseur.peek().is_none() {
                if tete || queue_soudee {
                    sortie.push_str(&mot);
                } else {
                    coupe_mot(&mot, sortie);
                }
                mot.clear();
            }
        } else {
            if !mot.is_empty() {
                if tete {
                    sortie.push_str(&mot);
                } else {
                    coupe_mot(&mot, sortie);
                }
                mot.clear();
            }
            tete = false;
            sortie.push(c);
        }
    }
}

pub fn cesure_html(html: &str) -> String {
    let mut sortie = String::with_capacity(html.len() + html.len() / 16);
    let mut reste = html;
    let mut ignores: Vec<String> = Vec::new();
    let mut soude = false;
    while !reste.is_empty() {
        if let Some(debut) = reste.find(|c| c == '<' || c == '&' || c == '\\') {
            let (texte, suite) = reste.split_at(debut);
            let entite_apres = suite.starts_with('&');
            if ignores.is_empty() {
                coupe_texte(texte, soude, entite_apres, &mut sortie);
            } else {
                sortie.push_str(texte);
            }
            soude = false;
            match suite.as_bytes()[0] {
                b'<' => {
                    let fin = suite.find('>').map(|i| i + 1).unwrap_or(suite.len());
                    let balise = &suite[..fin];
                    sortie.push_str(balise);
                    let interieur = balise.trim_start_matches('<').trim_end_matches('>');
                    let fermante = interieur.starts_with('/');
                    let nom: String = interieur
                        .trim_start_matches('/')
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric())
                        .collect::<String>()
                        .to_ascii_lowercase();
                    let protegee = matches!(
                        nom.as_str(),
                        "script"
                            | "style"
                            | "svg"
                            | "code"
                            | "pre"
                            | "textarea"
                            | "math"
                            | "h1"
                            | "h2"
                            | "h3"
                            | "h4"
                            | "h5"
                            | "h6"

                            | "docdg"
                    );
                    if protegee && !interieur.ends_with('/') {
                        if fermante {
                            if let Some(pos) = ignores.iter().rposition(|n| *n == nom) {
                                ignores.truncate(pos);
                            }
                        } else {
                            ignores.push(nom);
                        }
                    }
                    reste = &suite[fin..];
                }
                b'&' => {
                    let fin = suite
                        .char_indices()
                        .take(10)
                        .find(|&(_, c)| c == ';')
                        .map(|(i, _)| i + 1)
                        .unwrap_or(1);
                    sortie.push_str(&suite[..fin]);
                    soude = true;
                    reste = &suite[fin..];
                }
                _ => {
                    let fermeture = match suite.as_bytes().get(1) {
                        Some(b'(') => Some("\\)"),
                        Some(b'[') => Some("\\]"),
                        _ => None,
                    };
                    match fermeture {
                        Some(f) => {
                            let fin = suite[2..].find(f).map(|i| i + 2 + f.len()).unwrap_or(suite.len());
                            sortie.push_str(&suite[..fin]);
                            reste = &suite[fin..];
                        }
                        None => {
                            sortie.push('\\');
                            reste = &suite[1..];
                        }
                    }
                }
            }
        } else {
            if ignores.is_empty() {
                coupe_texte(reste, soude, false, &mut sortie);
            } else {
                sortie.push_str(reste);
            }
            reste = "";
        }
    }
    sortie
}
