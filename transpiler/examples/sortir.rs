fn main() {
    let c = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(&c).unwrap();
    let mut m = docdg_transpiler::Engine::new();
    let r = m.render(&src, true);
    std::fs::write(std::env::args().nth(2).unwrap(), &r.html).unwrap();
    println!("{} octets", r.html.len());
}
