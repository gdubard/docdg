use std::collections::BTreeMap;

fn main() {
    let dir = std::env::args().nth(1).expect("dossier");
    docdg_transpiler::set_base_dir(Some(std::path::PathBuf::from(&dir)));
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false))
        .collect();
    files.sort();
    let mut global: BTreeMap<String, usize> = BTreeMap::new();
    for f in &files {
        let src = std::fs::read_to_string(f).unwrap();
        let mut e = docdg_transpiler::Engine::new();
        let r = e.render(&src, false);
        let mut miss = 0usize;
        let mut idx = 0usize;
        while let Some(i) = r.html[idx..].find("&lt;") {
            let start = idx + i + 4;
            let rest = &r.html[start..];
            let word: String = rest
                .chars()
                .take_while(|c| c.is_alphabetic() || *c == '\'' || *c == 'à')
                .collect();
            if !word.is_empty() && word.chars().next().unwrap().is_uppercase()
                && !r.html[..start].ends_with("prise en charge : &lt;") {
                *global.entry(word).or_insert(0) += 1;
                miss += 1;
            }
            idx = start;
        }
        let non_pris = r.html.matches("calcul-absent").count();
        let erreurs = r.html.matches("calcul-note").count();
        let calculs = r.html.matches("class=\"calcul\"").count();
        println!(
            "{:34} {:4} rendus {:4} refusés {:4} notes {:4} restes",
            f.file_name().unwrap().to_string_lossy(),
            calculs,
            non_pris,
            erreurs,
            miss
        );
    }
    println!("\n--- verbes non traités, tous fichiers ---");
    let mut v: Vec<_> = global.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    for (k, n) in v {
        println!("{:6} {}", n, k);
    }
}
