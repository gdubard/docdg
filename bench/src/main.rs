use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let chemin = &args[1];
    let iters: u32 = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(20);
    let src = std::fs::read_to_string(chemin).unwrap();
    docdg_transpiler::set_base_dir(Some(std::path::PathBuf::from(
        std::path::Path::new(chemin).parent().unwrap(),
    )));

    let t0 = Instant::now();
    let mut engine = docdg_transpiler::Engine::new();
    let r = engine.render(&src, true);
    let froid = t0.elapsed();

    let t1 = Instant::now();
    for _ in 0..iters {
        let _ = engine.render(&src, true);
    }
    let chaud = t1.elapsed() / iters;

    let mut edite = src.clone();
    edite.push_str("\nParagraphe ajouté en fin de document.\n");
    let t2 = Instant::now();
    for _ in 0..iters {
        let _ = engine.render(&edite, true);
        let _ = engine.render(&src, true);
    }
    let incremental = t2.elapsed() / (iters * 2);

    let t3 = Instant::now();
    for _ in 0..iters {
        let mut e2 = docdg_transpiler::Engine::new();
        let _ = e2.render(&src, false);
    }
    let froid_seq = t3.elapsed() / iters;

    println!(
        "{}\tfroid_ms={:.2}\tfroid_seq_ms={:.2}\tchaud_ms={:.3}\tincr_ms={:.3}\thtml_ko={}",
        std::path::Path::new(chemin).file_name().unwrap().to_string_lossy(),
        froid.as_secs_f64() * 1000.0,
        froid_seq.as_secs_f64() * 1000.0,
        chaud.as_secs_f64() * 1000.0,
        incremental.as_secs_f64() * 1000.0,
        r.html.len() / 1024
    );
}
