use docdg_transpiler::Engine;
#[test]
fn la_numerotation_se_choisit(){
    let r = Engine::new().render("page {\n\tnumérotation: simple;\n}\nBonjour.", false);
    assert_eq!(r.page.numerotation, "simple", "parse simple");
    let r = Engine::new().render("page {\n\tnumérotation: sans;\n}\nBonjour.", false);
    assert_eq!(r.page.numerotation, "sans", "parse sans");
    let r = Engine::new().render("Bonjour.", false);
    assert!(r.page.to_json().contains("\"numerotation\":\"composee\""), "défaut composée");
    println!("NUMEROTATION OK");
}
