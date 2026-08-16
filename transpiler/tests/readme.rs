//! Le manuel tient parole : chaque bloc docdg du README rend sans erreur.
//!
//! C'est le test le plus important du dépôt pour un lecteur : un exemple du
//! manuel qui échoue coûte la confiance dès la première copie. Les blocs sont
//! extraits du fichier lui-même — modifier le README, c'est modifier le test.

use docdg_transpiler::Engine;

#[test]
fn chaque_bloc_du_manuel_rend_sans_erreur() {
    let manuel = std::fs::read_to_string("../README.md").unwrap();
    let mut blocs: Vec<String> = Vec::new();
    let mut dedans = false;
    let mut courant = String::new();
    for ligne in manuel.lines() {
        if dedans {
            if ligne.trim_end() == "```" {
                blocs.push(std::mem::take(&mut courant));
                dedans = false;
            } else {
                courant.push_str(ligne);
                courant.push('\n');
            }
        } else if ligne.trim_end() == "```docdg" {
            dedans = true;
        }
    }
    assert!(blocs.len() > 140, "le manuel devrait compter plus de 140 blocs");
    let mut echecs = Vec::new();
    for (i, b) in blocs.iter().enumerate() {
        let h = Engine::new().render(b, false).html;
        // Une balise non comprise ne produit pas toujours une erreur : elle
        // peut fuir en prose — le lecteur voit alors « <Insère une image… »
        // en toutes lettres. Le mot « Insère » n'a rien à faire dans un
        // rendu ; sa présence trahit la fuite. Le test des exemples fait la
        // même vérification : le manuel n'a pas à être moins bien gardé.
        if h.contains("Insère une image avec") {
            echecs.push(format!(
                "bloc {} : balise Insère fuie en prose — {}",
                i,
                b.lines().next().unwrap_or("").chars().take(60).collect::<String>()
            ));
        }
        if h.contains("calcul-absent") {
            echecs.push(format!(
                "bloc {} : {}",
                i,
                b.lines().next().unwrap_or("").chars().take(60).collect::<String>()
            ));
        }
    }
    assert!(
        echecs.is_empty(),
        "{} bloc(s) du manuel en échec :\n{}",
        echecs.len(),
        echecs.join("\n")
    );
}
