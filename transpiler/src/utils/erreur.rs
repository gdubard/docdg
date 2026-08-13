//! La forme des messages d'erreur, en un seul endroit.
//!
//! Le moteur avait deux façons de dire à l'utilisateur qu'il s'était trompé :
//! un `<rouge gras>` précédé d'un triangle pour les conteneurs, un filet rouge
//! en marge pour les calculs. Deux présentations pour la même chose — et
//! l'enseignant devait deviner, à la couleur du message, quelle partie du
//! moteur l'avait produit.
//!
//! Il n'y en a plus qu'une : un filet rouge en marge, un triangle, la source
//! fautive puis la raison. Le triangle est gardé — la couleur seule ne dit
//! rien à l'impression en noir et blanc, ni à qui la distingue mal.

fn echappe(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// L'erreur qui occupe sa propre ligne : une instruction refusée, une
/// commande inconnue, un calcul qui n'aboutit pas.
pub fn bloc(source: &str, message: &str) -> String {
    let source = source.trim();
    if source.is_empty() {
        return format!("<div class=\"calcul-absent\">⚠ {}</div>", echappe(message));
    }
    format!(
        "<div class=\"calcul-absent\">⚠ {} — {}</div>",
        echappe(source),
        echappe(message)
    )
}

/// L'erreur écrite dans la source du document plutôt que dans son HTML.
///
/// Une instruction de conteneur rend du docdg, que le moteur relit ensuite :
/// y glisser du HTML le ferait échapper et l'utilisateur lirait sa balise en
/// clair. Ce canal-là garde donc la mise en forme du langage. Le vocabulaire,
/// lui, est celui d'ici — c'est ce qui compte pour qui lit le message.
pub fn source(ligne: &str, message: &str) -> String {
    format!("<rouge gras>{{⚠ {} — {}}}\n", ligne.trim(), message)
}

/// L'erreur qui reste au fil du texte, là où un `#{...}` n'a pas pu être
/// calculé : elle ne peut pas casser le paragraphe qui la porte.
pub fn en_ligne(message: &str) -> String {
    format!("<span class=\"calcul-absent\">⚠ {}</span>", echappe(message))
}
