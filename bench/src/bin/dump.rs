fn main() {
    let chemin = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(&chemin).unwrap();
    docdg_transpiler::set_base_dir(Some(std::path::Path::new(&chemin).parent().unwrap().to_path_buf()));
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(&src, true);
    for pat in ["inconnu", "erreur", "katex-error", "??"] {
        eprintln!("{} : {}", pat, r.html.matches(pat).count());
    }
    println!("{}", r.html);
}
