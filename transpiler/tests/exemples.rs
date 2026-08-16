//! Chaque fichier d'exemple rend sans erreur : ce sont les premiers documents
//! qu'un nouvel utilisateur ouvre, et un exemple qui échoue coûte la
//! confiance aussi sûrement qu'un bloc du manuel.

use docdg_transpiler::Engine;

#[test]
fn chaque_exemple_rend_sans_erreur() {
    let mut fichiers: Vec<_> = std::fs::read_dir("../exemples")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false))
        .collect();
    fichiers.sort();
    assert!(fichiers.len() >= 28, "il devrait y avoir au moins 28 exemples");
    // Deux exemples démontrent le canal d'erreur lui-même — indice hors
    // bornes, attribut privé, classe abstraite : leurs erreurs sont la
    // matière de la leçon, et leur nombre exact est verrouillé pour que
    // toute dérive, en plus ou en moins, se voie.
    let attendues = |nom: &str| match nom {
        "algo3.txt" => 7,
        "algo4.txt" => 3,
        _ => 0,
    };
    let mut echecs = Vec::new();
    for f in &fichiers {
        let nom = f.file_name().unwrap().to_string_lossy().to_string();
        let src = std::fs::read_to_string(f).unwrap();
        let h = Engine::new().render(&src, false).html;
        let n = h.matches("calcul-absent").count();
        // Une balise non comprise ne produit pas toujours une erreur : elle
        // peut fuir en prose — le lecteur voit alors « <Insère une image… »
        // en toutes lettres. Le mot « Insère » n'a rien à faire dans un
        // rendu ; sa présence trahit la fuite.
        if h.contains("Insère une image avec") {
            echecs.push(format!("{} : balise Insère fuie en prose", nom));
        }
        if n != attendues(&nom) {
            echecs.push(format!(
                "{} : {} erreur(s), {} attendue(s)",
                nom,
                n,
                attendues(&nom)
            ));
        }
    }
    assert!(echecs.is_empty(), "{}", echecs.join("\n"));
}
