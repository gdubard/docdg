use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const EXCEPTIONS: &[(&str, &str)] = &[
    ("faire{", "forme collée de « faire { » : tolérance d'espacement, pas une notion"),
];

fn exception(cle: &str) -> bool {
    EXCEPTIONS.iter().any(|(c, _)| *c == cle)
}

fn plie(s: &str) -> String {
    s.chars()
        .flat_map(|c| c.to_lowercase())
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            '\u{2019}' => '\'',
            c => c,
        })
        .collect()
}

fn litteraux_du_tableau(source: &str, nom: &str) -> Vec<String> {
    let Some(debut) = source.find(&format!("const {}", nom)) else {
        return Vec::new();
    };
    let reste = &source[debut..];
    let Some(fin) = reste.find("];") else {
        return Vec::new();
    };
    litteraux(&reste[..fin])
}

fn litteraux(fragment: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut reste = fragment;
    while let Some(i) = reste.find('"') {
        let apres = &reste[i + 1..];
        match apres.find('"') {
            Some(j) => {
                out.push(apres[..j].to_string());
                reste = &apres[j + 1..];
            }
            None => break,
        }
    }
    out
}

fn noms_des_bras(source: &str, ancre: &str, fin: &str) -> Vec<String> {
    let Some(d) = source.find(ancre) else { return Vec::new() };
    let reste = &source[d + ancre.len()..];
    let Some(f) = reste.find(fin) else { return Vec::new() };
    let mut out = Vec::new();
    for ligne in reste[..f].lines() {
        let Some((gauche, _)) = ligne.split_once("=>") else { continue };
        if let Some(premier) = litteraux(gauche).into_iter().next() {
            out.push(premier);
        }
    }
    out
}

struct Pole {
    exemples: String,
    manuels: String,
}

impl Pole {
    fn lit(racine: &Path) -> Self {
        let mut exemples = String::new();
        ramasse(&racine.join("exemples"), "txt", &mut exemples);
        let mut manuels = String::new();
        for m in [
            "README.md", "Ecole.md", "College.md", "Lycee.md",
            "Licence.md", "MasterAgregation.md", "Redaction.md",
            "docs/REGLES-CORPUS.md",
        ] {
            if let Ok(t) = fs::read_to_string(racine.join(m)) {
                manuels.push_str(&t);
                manuels.push('\n');
            }
        }
        Pole { exemples: plie(&exemples), manuels: plie(&manuels) }
    }
    fn dans_exemples(&self, cle: &str) -> bool {
        self.exemples.contains(&plie(cle))
    }
    fn dans_manuels(&self, cle: &str) -> bool {
        self.manuels.contains(&plie(cle))
    }
}

fn ramasse(dossier: &Path, extension: &str, dest: &mut String) {
    let Ok(entrees) = fs::read_dir(dossier) else { return };
    for e in entrees.flatten() {
        let chemin = e.path();
        if chemin.is_dir() {
            ramasse(&chemin, extension, dest);
        } else if chemin.extension().and_then(|x| x.to_str()) == Some(extension) {
            if let Ok(t) = fs::read_to_string(&chemin) {
                dest.push_str(&t);
                dest.push('\n');
            }
        }
    }
}

struct Rapport {
    manques: Vec<String>,
}

impl Rapport {
    fn table(&mut self, pole: &Pole, titre: &str, cles: &[String]) {
        let mut sans_exemple = Vec::new();
        let mut sans_manuel = Vec::new();
        for c in cles {
            if exception(c) {
                continue;
            }
            if !pole.dans_exemples(c) {
                sans_exemple.push(c.clone());
            }
            if !pole.dans_manuels(c) {
                sans_manuel.push(c.clone());
            }
        }
        println!(
            "  {:34} {:3} entrées   exemples {:3}   manuels {:3}",
            titre,
            cles.len(),
            cles.len() - sans_exemple.len(),
            cles.len() - sans_manuel.len()
        );
        for c in &sans_exemple {
            self.manques.push(format!("{titre} — « {c} » n'est écrit dans aucun exemple"));
        }
        for c in &sans_manuel {
            self.manques.push(format!("{titre} — « {c} » n'est expliqué dans aucun manuel"));
        }
    }
}

pub fn couverture(racine: &Path) {
    let src = racine.join("transpiler/src");
    let lit = |c: &str| fs::read_to_string(src.join(c)).unwrap_or_default();
    let rendu = lit("layout/rendu.rs");
    let couleurs = lit("utils/couleurs.rs");
    let fonctions = lit("langage/fonctions.rs");
    let environnements = lit("layout/environnements.rs");
    let demonstration = lit("layout/demonstration.rs");

    let pole = Pole::lit(racine);
    let mut r = Rapport { manques: Vec::new() };

    println!("Couverture des tables du moteur");

    let verbes: Vec<String> = litteraux_du_tableau(&rendu, "VERBES: &[(&str, u8)]")
        .into_iter()
        .step_by(1)
        .filter(|s| s.chars().next().is_some_and(char::is_uppercase))
        .collect();
    r.table(&pole, "verbes", &verbes);

    let mots_style: Vec<String> = litteraux_du_tableau(&rendu, "MOTS_DE_STYLE")
        .into_iter()
        .filter(|s| !s.contains(':'))
        .collect();
    r.table(&pole, "mots de style", &mots_style);

    let locutions: Vec<String> = litteraux_du_tableau(&rendu, "LOCUTIONS_DE_STYLE")
        .into_iter()
        .filter(|s| !s.contains(':'))
        .collect();
    let locutions: Vec<String> = locutions
        .chunks(2)
        .map(|p| p.join(" "))
        .collect();
    r.table(&pole, "locutions de style", &locutions);

    let cles_document = noms_des_bras(&rendu, "fn cle_connue(k: &str) -> bool {", "\n}");
    let cles_document: Vec<String> = if cles_document.is_empty() {
        litteraux(
            rendu
                .split("fn cle_connue")
                .nth(1)
                .and_then(|r| r.split("\n}").next())
                .unwrap_or(""),
        )
    } else {
        cles_document
    };
    r.table(&pole, "clés du bloc document", &cles_document);

    let noms_couleurs = litteraux_du_tableau(&couleurs, "NOMS: &[&str]");
    r.table(&pole, "couleurs", &noms_couleurs);

    let primitives: Vec<String> = litteraux_du_tableau(&fonctions, "PRIMITIVES");
    r.table(&pole, "primitives", &primitives);

    let genres: Vec<String> = litteraux_du_tableau(&environnements, "GENRES: &[Genre]")
        .into_iter()
        .step_by(3)
        .collect();
    r.table(&pole, "genres d'environnement", &genres);

    let etapes: Vec<String> = demonstration
        .split("fn etape_ouverte")
        .nth(1)
        .and_then(|r| r.split("if let Some(titre)").next())
        .map(litteraux)
        .unwrap_or_default();
    r.table(&pole, "étapes de démonstration", &etapes);

    let raisonnements: Vec<String> = demonstration
        .split("for forme in [")
        .nth(1)
        .and_then(|r| r.split(']').next())
        .map(litteraux)
        .unwrap_or_default();
    r.table(&pole, "raisonnements", &raisonnements);

    let connues: BTreeSet<String> = noms_couleurs.iter().map(|n| plie(n)).collect();
    let mut fantomes = Vec::new();
    if let Ok(manuel) = fs::read_to_string(racine.join("README.md")) {
        if let Some(section) = manuel
            .split("Référence des couleurs")
            .nth(1)
            .and_then(|r| r.split("### **Options de classe**").next())
        {
            for bloc in section.split("```").skip(1).step_by(2) {

                if bloc.contains('<') || bloc.contains('⚠') {
                    continue;
                }
                for nom in bloc.split(',') {
                    let nom = nom.trim();
                    if nom.is_empty() {
                        continue;
                    }
                    if !connues.contains(&plie(nom)) {
                        fantomes.push(nom.to_string());
                    }
                }
            }
        }
    }
    if !fantomes.is_empty() {
        for f in &fantomes {
            r.manques
                .push(format!("couleurs — « {f} » est annoncé au manuel et n'existe pas dans le moteur"));
        }
    }
    println!(
        "  {:34} {:3} noms annoncés au manuel, {} sans couleur",
        "couleurs du manuel",
        connues.len(),
        fantomes.len()
    );

    println!();
    if r.manques.is_empty() {
        println!("Les trois pôles concordent.");
        if !EXCEPTIONS.is_empty() {
            println!("Exceptions déclarées :");
            for (c, raison) in EXCEPTIONS {
                println!("  « {c} » — {raison}");
            }
        }
        return;
    }
    eprintln!("{} écart(s) :", r.manques.len());
    for m in &r.manques {
        eprintln!("  {m}");
    }
    std::process::exit(1);
}
