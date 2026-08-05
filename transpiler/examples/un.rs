fn main() {
    let c = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(&c).unwrap();
    docdg_transpiler::set_base_dir(std::path::Path::new(&c).parent().map(|p| p.to_path_buf()));
    let mut m = docdg_transpiler::Engine::new();
    println!("{}", m.render(&src, true).html.len());
}
