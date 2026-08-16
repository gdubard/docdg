use std::time::Instant;

fn mediane(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn chrono(mut f: impl FnMut()) -> f64 {
    let t = Instant::now();
    f();
    t.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    let mut args = std::env::args().skip(1);
    let chemin = args
        .next()
        .expect("usage : froid_chaud FICHIER [ITERATIONS] [PLAFOND_INCR_MS]");
    let iterations: usize = args.next().and_then(|a| a.parse().ok()).unwrap_or(20);
    let plafond: Option<f64> = args.next().and_then(|a| a.parse().ok());
    let src = std::fs::read_to_string(&chemin).unwrap();
    docdg_transpiler::set_base_dir(Some(
        std::path::Path::new(&chemin).parent().unwrap().to_path_buf(),
    ));
    docdg_transpiler::prechauffe();
    std::thread::sleep(std::time::Duration::from_secs(3));

    let neuf = iterations.div_euclid(4).max(3);
    let froid = mediane(
        (0..neuf)
            .map(|_| chrono(|| {
                let _ = docdg_transpiler::Engine::new().render(&src, true);
            }))
            .collect(),
    );
    let froid_seq = mediane(
        (0..neuf)
            .map(|_| chrono(|| {
                let _ = docdg_transpiler::Engine::new().render(&src, false);
            }))
            .collect(),
    );

    let mut moteur = docdg_transpiler::Engine::new();
    let r = moteur.render(&src, true);
    let chaud = mediane(
        (0..iterations)
            .map(|_| chrono(|| {
                let _ = moteur.render(&src, true);
            }))
            .collect(),
    );
    let modifie = format!("{}\n\nUn paragraphe ajouté pour la mesure.\n", src);
    let incr = mediane(
        (0..iterations)
            .flat_map(|_| {
                let a = chrono(|| {
                    let _ = moteur.render(&modifie, true);
                });
                let b = chrono(|| {
                    let _ = moteur.render(&src, true);
                });
                [a, b]
            })
            .collect(),
    );

    println!(
        "{}\tfroid_ms={:.2}\tfroid_seq_ms={:.2}\tchaud_ms={:.3}\tincr_ms={:.3}\thtml_ko={}",
        std::path::Path::new(&chemin)
            .file_name()
            .unwrap()
            .to_string_lossy(),
        froid,
        froid_seq,
        chaud,
        incr,
        r.html.len() / 1024
    );
    if let Some(p) = plafond {
        if incr > p {
            eprintln!(
                "régression : incr {:.3} ms dépasse le plafond {} ms",
                incr, p
            );
            std::process::exit(1);
        }
    }
}
