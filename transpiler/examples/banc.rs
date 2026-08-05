fn hwm() -> f64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).map(|v| v.to_string()))
        })
        .and_then(|v| v.parse::<f64>().ok())
        .map(|k| k / 1024.0)
        .unwrap_or(0.0)
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.total_cmp(b));
    v[v.len() / 2]
}

fn main() {
    let chemin = std::env::args().nth(1).expect("chemin");
    let src = std::fs::read_to_string(&chemin).expect("lecture");
    let tours = 7;

    let mut froids = Vec::new();
    let mut octets = 0;
    for _ in 0..tours {
        let mut m = docdg_transpiler::Engine::new();
        let t = std::time::Instant::now();
        let r = m.render(&src, true);
        froids.push(t.elapsed().as_secs_f64() * 1000.0);
        octets = r.html.len();
    }

    let mut moteur = docdg_transpiler::Engine::new();
    let _ = moteur.render(&src, true);
    let segments = moteur.cache_len();
    let mut chauds = Vec::new();
    for _ in 0..tours {
        let t = std::time::Instant::now();
        let _ = moteur.render(&src, true);
        chauds.push(t.elapsed().as_secs_f64() * 1000.0);
    }

    let mut incrs = Vec::new();
    for i in 0..tours {
        let modifie = src.replacen(
            "Mesure de reference du document.",
            &format!("Mesure de reference du document, revision {}.", i),
            1,
        );
        let t = std::time::Instant::now();
        let _ = moteur.render(&modifie, true);
        incrs.push(t.elapsed().as_secs_f64() * 1000.0);
    }

    println!(
        "{:.1} {:.1} {:.1} {} {} {:.1}",
        median(froids),
        median(chauds),
        median(incrs),
        octets,
        segments,
        hwm()
    );
}
