use docdg_transpiler::Engine;

#[test]
fn un_document_sans_calcul_ne_lance_aucun_python() {
    let mut e = Engine::new();
    let src = "soit h1 = <bleu marine gras section num>\n\n<h1>Titre\n\n\
               Un paragraphe ordinaire avec une formule $a^2 + b^2 = c^2$.\n\n\
               <Calcule>la moyenne de la série 12 ; 15 ; 9 ; 14\n\n\
               <Dresse>un tableau [mc, mc] {\n\t[a ; b]\n}";
    let html = e.render(src, false).html;
    assert!(html.contains("moyenne") || html.contains("12{,}5"), "{}", html);
    assert!(
        !docdg_transpiler::bassin_ouvert(),
        "un ouvrier Python a été lancé alors qu'aucun calcul formel n'est demandé"
    );
}
