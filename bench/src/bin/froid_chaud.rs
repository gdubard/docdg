use std::time::Instant;

fn main() {
    let chemin = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(&chemin).unwrap();
    docdg_transpiler::set_base_dir(Some(
        std::path::Path::new(&chemin).parent().unwrap().to_path_buf(),
    ));
    docdg_transpiler::prechauffe();
    std::thread::sleep(std::time::Duration::from_secs(3));
    let t0 = Instant::now();
    let mut engine = docdg_transpiler::Engine::new();
    let r = engine.render(&src, true);
    println!(
        "{}\tbassin_chaud_froid_ms={:.1}\thtml_ko={}",
        std::path::Path::new(&chemin).file_name().unwrap().to_string_lossy(),
        t0.elapsed().as_secs_f64() * 1000.0,
        r.html.len() / 1024
    );
}
