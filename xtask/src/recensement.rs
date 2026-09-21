use docdg_corpus::{Corpus, Demonstration, Enonce, Matiere, Niveau, Statut};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const NIVEAU_MAIGRE: usize = 30;
const DOMAINE_MAIGRE: usize = 3;

struct Compte {
    ancrages: usize,
    matieres: BTreeMap<Matiere, usize>,
    admis: usize,
    demontres: usize,
    preuves: usize,
    lacunes: usize,
}

impl Compte {
    fn neuf() -> Self {
        Compte {
            ancrages: 0,
            matieres: BTreeMap::new(),
            admis: 0,
            demontres: 0,
            preuves: 0,
            lacunes: 0,
        }
    }

    fn matiere(&self, m: Matiere) -> usize {
        self.matieres.get(&m).copied().unwrap_or(0)
    }
}

fn capitale(s: &str) -> String {
    let mut lettres = s.chars();
    match lettres.next() {
        Some(p) => p.to_uppercase().collect::<String>() + lettres.as_str(),
        None => String::new(),
    }
}

fn cycle(niveau: Niveau) -> &'static str {
    match niveau {
        Niveau::CP | Niveau::CE1 | Niveau::CE2 | Niveau::CM1 | Niveau::CM2 => "école",
        Niveau::Sixième | Niveau::Cinquième | Niveau::Quatrième | Niveau::Troisième => "collège",
        Niveau::Seconde | Niveau::Première | Niveau::Terminale => "lycée",
        Niveau::Licence1 | Niveau::Licence2 | Niveau::Licence3 => "supérieur",
        Niveau::Master1 | Niveau::Master2 => "supérieur",
    }
}

const ORDRE_BIBLIOTHEQUE: [&str; 4] = ["lycée", "L1", "L2", "CPGE"];

const MATIERES: [Matiere; 3] = [Matiere::Mathématiques, Matiere::Physique, Matiere::Chimie];

fn par_niveau(corpus: &Corpus) -> BTreeMap<Niveau, Compte> {
    let mut table: BTreeMap<Niveau, Compte> =
        Niveau::TOUS.iter().map(|n| (*n, Compte::neuf())).collect();
    for e in corpus.enonces() {
        for a in &e.ancrages {
            let Some(c) = table.get_mut(&a.niveau) else {
                continue;
            };
            c.ancrages += 1;
            *c.matieres.entry(e.matiere).or_insert(0) += 1;
            match a.statut {
                Statut::Admis => c.admis += 1,
                Statut::Démontré => c.demontres += 1,
            }
            if a.demonstration_prevue {
                c.lacunes += 1;
            }
        }
    }
    for d in corpus.demonstrations() {
        if let Some(c) = table.get_mut(&d.niveau_redaction) {
            c.preuves += 1;
        }
    }
    table
}

fn par_domaine(corpus: &Corpus) -> BTreeMap<(Matiere, String), usize> {
    let mut table = BTreeMap::new();
    for e in corpus.enonces() {
        for d in &e.domaine {
            *table.entry((e.matiere, d.clone())).or_insert(0) += 1;
        }
    }
    table
}

pub fn recensement() {
    let corpus = Corpus::charger();
    let niveaux = par_niveau(&corpus);
    let domaines = par_domaine(&corpus);

    println!(
        "Recensement du corpus — {} énoncés, {} démonstrations, {} grandeurs, {} cadres",
        corpus.enonces().len(),
        corpus.demonstrations().len(),
        corpus.grandeurs().len(),
        corpus.cadres().len()
    );

    println!();
    println!(
        "{:<12}{:>9}{:>8}{:>7}{:>7}{:>8}{:>10}{:>9}{:>11}",
        "niveau", "ancrages", "maths", "phys", "chim", "admis", "démontré", "preuves", "à rédiger"
    );
    let mut total = 0;
    for niveau in Niveau::TOUS {
        let c = &niveaux[&niveau];
        total += c.ancrages;
        println!(
            "{:<12}{:>9}{:>8}{:>7}{:>7}{:>8}{:>10}{:>9}{:>11}",
            niveau.nom(),
            c.ancrages,
            c.matiere(Matiere::Mathématiques),
            c.matiere(Matiere::Physique),
            c.matiere(Matiere::Chimie),
            c.admis,
            c.demontres,
            c.preuves,
            c.lacunes
        );
    }
    println!("{:<12}{:>9}", "total", total);

    println!();
    println!("{} domaines de rangement", domaines.len());
    for matiere in MATIERES {
        let mut liste: Vec<(&str, usize)> = domaines
            .iter()
            .filter(|((m, _), _)| *m == matiere)
            .map(|((_, d), n)| (d.as_str(), *n))
            .collect();
        liste.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        let enonces: usize = liste.iter().map(|(_, n)| *n).sum();
        println!(
            "  {} — {} domaines, {} énoncés",
            matiere.nom(),
            liste.len(),
            enonces
        );
        for (domaine, n) in liste.iter().filter(|(_, n)| *n < DOMAINE_MAIGRE) {
            println!("    {:<46}{:>3}", domaine, n);
        }
    }

    println!();
    println!("Ce qui reste à nourrir");
    let mut rien = true;
    for niveau in Niveau::TOUS {
        let c = &niveaux[&niveau];
        if c.ancrages < NIVEAU_MAIGRE {
            rien = false;
            println!(
                "  {:<12} {} ancrages, sous le seuil de {}",
                niveau.nom(),
                c.ancrages,
                NIVEAU_MAIGRE
            );
        }
        if c.lacunes > 0 {
            rien = false;
            println!(
                "  {:<12} {} démonstrations annoncées et non rédigées",
                niveau.nom(),
                c.lacunes
            );
        }
    }
    let maigres = domaines.values().filter(|n| **n < DOMAINE_MAIGRE).count();
    if maigres > 0 {
        rien = false;
        println!(
            "  {} domaines tiennent en moins de {} énoncés",
            maigres, DOMAINE_MAIGRE
        );
    }
    if rien {
        println!("  rien : tous les niveaux et tous les domaines passent les seuils.");
    }

    monotonie(&corpus);
}

fn monotonie(corpus: &Corpus) {
    let mut entorses: Vec<(u32, String, &'static str, String, &'static str)> = Vec::new();
    let mut assumees: Vec<(u32, String, &'static str, String, &'static str)> = Vec::new();
    for e in corpus.enonces() {
        let Some(ici) = e.premier_niveau() else {
            continue;
        };
        for dep in &e.depend_de {
            let Some(amont) = corpus.enonce(dep) else {
                continue;
            };
            let Some(la_bas) = amont.premier_niveau() else {
                continue;
            };
            if la_bas.rang() > ici.rang() {
                let ligne = (
                    la_bas.rang() - ici.rang(),
                    e.id.clone(),
                    ici.nom(),
                    amont.id.clone(),
                    la_bas.nom(),
                );
                if e.depend_en_avance.iter().any(|c| c == dep) {
                    assumees.push(ligne);
                } else {
                    entorses.push(ligne);
                }
            }
        }
    }
    println!();
    println!(
        "Prérequis en avance assumés : {} — le corpus les déclare, la validation les laisse passer.",
        assumees.len()
    );
    assumees.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.3.cmp(&b.3)));
    for (ecart, id, niveau, dep, niveau_dep) in assumees.iter().take(10) {
        println!(
            "  +{:<2} {:<38} ({}) s'appuie sur {} ({})",
            ecart, id, niveau, dep, niveau_dep
        );
    }
    if assumees.len() > 10 {
        println!("  … et {} autres", assumees.len() - 10);
    }
    println!();
    println!("Prérequis en avance NON déclarés");
    if entorses.is_empty() {
        println!("  rien : toute dépendance en avance est assumée par une marque.");
        return;
    }
    entorses.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.3.cmp(&b.3)));
    let total = entorses.len();
    for (ecart, id, niveau, dep, niveau_dep) in entorses.iter().take(20) {
        println!(
            "  +{:<2} {:<38} ({}) s'appuie sur {} ({})",
            ecart, id, niveau, dep, niveau_dep
        );
    }
    if total > 20 {
        println!("  … et {} autres", total - 20);
    }
    println!(
        "  {} prérequis en avance non déclarés — la validation du corpus les refuse.",
        total
    );
}

fn echappe(s: &str) -> String {
    s.replace('|', "\\|")
}

fn ancrages_dits(e: &Enonce) -> String {
    let mut v: Vec<String> = Vec::new();
    for a in &e.ancrages {
        let dit = match &a.voie {
            Some(voie) => format!("{} ({})", a.niveau.nom(), voie.0),
            None => a.niveau.nom().to_string(),
        };
        if !v.contains(&dit) {
            v.push(dit);
        }
    }
    v.join(" · ")
}

fn demonstration_dite(e: &Enonce, preuves: &[&Demonstration]) -> String {
    if e.ancrages.iter().any(|a| a.demonstration_prevue) {
        return "🕐 à rédiger".to_string();
    }
    if preuves.is_empty() {
        if e.ancrages.iter().any(|a| a.statut == Statut::Démontré) {
            return "—".to_string();
        }
        return "·".to_string();
    }
    let mut niveaux: Vec<&str> = Vec::new();
    for d in preuves {
        let nom = d.niveau_redaction.nom();
        if !niveaux.contains(&nom) {
            niveaux.push(nom);
        }
    }
    format!("✅ {}", niveaux.join(" · "))
}

fn entete(corpus: &Corpus, niveaux: &BTreeMap<Niveau, Compte>) -> String {
    let mut enonces: BTreeMap<Matiere, usize> = BTreeMap::new();
    for e in corpus.enonces() {
        *enonces.entry(e.matiere).or_insert(0) += 1;
    }
    let mut demos: BTreeMap<Matiere, usize> = BTreeMap::new();
    for d in corpus.demonstrations() {
        if let Some(e) = corpus.enonce(&d.enonce_id) {
            *demos.entry(e.matiere).or_insert(0) += 1;
        }
    }

    let mut s = String::new();
    s.push_str("# docdg — La rédaction\n\n");
    s.push_str(
        "Le catalogue du corpus : chaque énoncé, la phrase qui l'appelle, les niveaux\n\
         où il est exigible, et l'état de sa démonstration.\n\n",
    );
    s.push_str(
        "> Ce document est **engendré** et ne se modifie pas à la main :\n\
         > `cargo app recensement --redaction` le réécrit depuis `corpus/corpus.db`\n\
         > et `transpiler/src/maths/demonstrations.json`.\n\n",
    );
    s.push_str("---\n\n## **📊 Ce que le corpus contient**\n\n");
    s.push_str("| | Énoncés | Démonstrations rédigées |\n|---|---:|---:|\n");
    for matiere in MATIERES {
        s.push_str(&format!(
            "| {} | {} | {} |\n",
            capitale(matiere.nom()),
            enonces.get(&matiere).copied().unwrap_or(0),
            demos.get(&matiere).copied().unwrap_or(0)
        ));
    }
    s.push_str(&format!(
        "| **Total** | **{}** | **{}** |\n\n",
        corpus.enonces().len(),
        corpus.demonstrations().len()
    ));
    s.push_str(&format!(
        "S'y ajoutent **{} grandeurs physiques** et **{} cadres d'hypothèses**, et une\n\
         **bibliothèque de démonstrations classiques** servie par `<Montre>`, décrite\n\
         en fin de document.\n\n",
        corpus.grandeurs().len(),
        corpus.cadres().len()
    ));

    s.push_str("### Par niveau\n\n");
    s.push_str("| Niveau | Cycle | Ancrages | dont mathématiques | physique | chimie |\n");
    s.push_str("|---|---|---:|---:|---:|---:|\n");
    let mut total = 0;
    for niveau in Niveau::TOUS {
        let c = &niveaux[&niveau];
        if c.ancrages == 0 {
            continue;
        }
        total += c.ancrages;
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            niveau.nom(),
            cycle(niveau),
            c.ancrages,
            c.matiere(Matiere::Mathématiques),
            c.matiere(Matiere::Physique),
            c.matiere(Matiere::Chimie)
        ));
    }
    s.push_str(&format!("| **Total** | | **{}** | | | |\n", total));
    s
}

fn lecture() -> String {
    let mut s = String::new();
    s.push_str("\n---\n\n## **🧭 Comment lire ce catalogue**\n\n");
    s.push_str(
        "Chaque énoncé est rangé **au premier niveau où il est exigible** ; ses\n\
         ancrages ultérieurs figurent dans sa ligne. La colonne **Appel** donne la\n\
         phrase à écrire — elle fonctionne telle quelle :\n\n",
    );
    s.push_str("```docdg\n<Énonce>le théorème de Pythagore\n<Démontre>le théorème de Pythagore\n```\n\n");
    s.push_str(
        "`<Énonce>` compose l'énoncé, sa formule, ses hypothèses et son domaine de\n\
         validité. `<Démontre>` sert la démonstration du corpus, rédigée au niveau le\n\
         plus proche de celui du document — et **le dit** quand elle vient d'un autre.\n\n",
    );
    s.push_str("| Colonne | Sens |\n|---|---|\n");
    s.push_str("| **Type** | axiome, définition, propriété, règle, lemme, théorème, corollaire, proposition, loi, principe, postulat, modèle, relation |\n");
    s.push_str("| **Appel** | la phrase qui résout l'énoncé — le titre, avec son article |\n");
    s.push_str("| **Ancrages** | les niveaux où l'énoncé est exigible, et la voie s'il y en a une |\n");
    s.push_str("| **Démonstration** | ✅ rédigée, au niveau indiqué · 🕐 attendue mais non rédigée (`demonstration_prevue`) · — annoncée démontrée ailleurs · · admis, sans démonstration attendue |\n");
    s.push_str("\n---\n");
    s
}

fn catalogue(corpus: &Corpus, sortie: &mut String) {
    let mut preuves: BTreeMap<&str, Vec<&Demonstration>> = BTreeMap::new();
    for d in corpus.demonstrations() {
        preuves.entry(d.enonce_id.as_str()).or_default().push(d);
    }
    for liste in preuves.values_mut() {
        liste.sort_by_key(|d| d.niveau_redaction);
    }

    let mut rangement: BTreeMap<(Matiere, Niveau, &str), Vec<&Enonce>> = BTreeMap::new();
    for e in corpus.enonces() {
        let Some(niveau) = e.premier_niveau() else {
            continue;
        };
        let domaine = e.domaine.first().map(|d| d.as_str()).unwrap_or("divers");
        rangement
            .entry((e.matiere, niveau, domaine))
            .or_default()
            .push(e);
    }
    for liste in rangement.values_mut() {
        liste.sort_by(|a, b| a.titre.to_lowercase().cmp(&b.titre.to_lowercase()));
    }

    let vides: Vec<&Demonstration> = Vec::new();
    for matiere in MATIERES {
        sortie.push_str(&format!("\n## **{}**\n", capitale(matiere.nom())));
        for niveau in Niveau::TOUS {
            let domaines: Vec<&str> = rangement
                .keys()
                .filter(|(m, n, _)| *m == matiere && *n == niveau)
                .map(|(_, _, d)| *d)
                .collect();
            if domaines.is_empty() {
                continue;
            }
            sortie.push_str(&format!("\n### {}\n", capitale(niveau.nom())));
            for domaine in domaines {
                sortie.push_str(&format!("\n#### {}\n\n", domaine));
                sortie.push_str("| Type | Appel | Ancrages | Démonstration |\n|---|---|---|---|\n");
                for e in &rangement[&(matiere, niveau, domaine)] {
                    let siennes = preuves.get(e.id.as_str()).unwrap_or(&vides);
                    sortie.push_str(&format!(
                        "| {} | `{}` | {} | {} |\n",
                        echappe(&e.genre),
                        echappe(&e.titre),
                        echappe(&ancrages_dits(e)),
                        demonstration_dite(e, siennes)
                    ));
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct Fiche {
    niveau: String,
    raisonnement: String,
    #[serde(rename = "énoncé")]
    enonce: String,
    #[serde(rename = "clés")]
    cles: Vec<String>,
}

#[derive(Deserialize)]
struct Bibliotheque {
    fiches: Vec<Fiche>,
}

fn classiques(racine: &Path) -> String {
    let chemin = racine.join("transpiler/src/maths/demonstrations.json");
    let Ok(brut) = fs::read_to_string(&chemin) else {
        return String::new();
    };
    let bibliotheque: Bibliotheque =
        serde_json::from_str(&brut).expect("demonstrations.json illisible");
    if bibliotheque.fiches.is_empty() {
        return String::new();
    }

    let mut par_niveau: BTreeMap<(usize, &str), Vec<&Fiche>> = BTreeMap::new();
    for f in &bibliotheque.fiches {
        let rang = ORDRE_BIBLIOTHEQUE
            .iter()
            .position(|n| *n == f.niveau)
            .unwrap_or(ORDRE_BIBLIOTHEQUE.len());
        par_niveau
            .entry((rang, f.niveau.as_str()))
            .or_default()
            .push(f);
    }

    let mut s = String::new();
    s.push_str("\n---\n\n## **📚 La bibliothèque des classiques**\n\n");
    s.push_str(&format!(
        "**{} démonstrations d'idée**, servies par **`<Montre>`** — celles qu'aucun\n\
         calcul n'atteint, et qui reposent sur une construction ou un argument de\n\
         structure. Elles s'appellent **par leur énoncé**, sans corps :\n\n",
        bibliotheque.fiches.len()
    ));
    s.push_str("```docdg\n<Montre>que $racine(2)$ est irrationnel\n```\n\n");
    s.push_str(
        "Tout ce qu'une identité ou une inégalité vérifiable démontre relève du calcul\n\
         formel, non d'une fiche : `<Montre>par récurrence que …` est vérifié puis\n\
         rédigé par le moteur, sans passer par ici.\n",
    );

    for ((_, niveau), liste) in &par_niveau {
        s.push_str(&format!(
            "\n### {} — {} démonstrations\n\n",
            niveau,
            liste.len()
        ));
        s.push_str("| Raisonnement | Appel | Autres formulations reçues |\n|---|---|---|\n");
        let mut ordonnee = liste.clone();
        ordonnee.sort_by(|a, b| a.enonce.to_lowercase().cmp(&b.enonce.to_lowercase()));
        for f in ordonnee {
            let autres: Vec<String> = f
                .cles
                .iter()
                .map(|c| format!("`{}`", echappe(c)))
                .collect();
            let raisonnement = if f.raisonnement.is_empty() {
                "direct"
            } else {
                &f.raisonnement
            };
            s.push_str(&format!(
                "| {} | `<Montre>{}` | {} |\n",
                echappe(raisonnement),
                echappe(&f.enonce),
                autres.join(" · ")
            ));
        }
    }
    s
}

pub fn redaction(racine: &Path) {
    let corpus = Corpus::charger();
    let niveaux = par_niveau(&corpus);
    let mut sortie = entete(&corpus, &niveaux);
    sortie.push_str(&lecture());
    catalogue(&corpus, &mut sortie);
    sortie.push_str(&classiques(racine));
    sortie.push_str(
        "\n---\n\n*Engendré depuis `corpus/corpus.db` et \
         `transpiler/src/maths/demonstrations.json` par `cargo app recensement --redaction`.*\n",
    );
    let chemin = racine.join("Redaction.md");
    fs::write(&chemin, sortie).expect("écriture de Redaction.md");
    println!("{} réécrit depuis le corpus.", chemin.display());
}
