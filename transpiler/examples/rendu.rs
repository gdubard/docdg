fn main() {
    let path = std::env::args().nth(1).expect("chemin du fichier source");
    let src = std::fs::read_to_string(&path).expect("lecture");
    let mut e = docdg_transpiler::Engine::new();
    let r = e.render(&src, false);
    println!("{}", r.html);
}
