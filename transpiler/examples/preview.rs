fn json_str(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".into())
}

fn main() {
    let src: String = match std::env::args().nth(2) {
        Some(chemin) => std::fs::read_to_string(chemin).expect("lecture"),
        None => include_str!("../tests/basique3.txt").to_string(),
    };
    let src = src.as_str();
    let index = include_str!("../../app/src/static/index.html");
    let css = include_str!("../../app/src/static/style.css");
    let js = include_str!("../../app/src/static/app.js");
    let math = format!(
        "<style>{}</style>\n<script>{}</script>\n<script>{}</script>",
        include_str!("../../app/src/static/katex/katex.min.css"),
        include_str!("../../app/src/static/katex/katex.min.js"),
        include_str!("../../app/src/static/katex/auto-render.min.js")
    );

    docdg_transpiler::set_base_dir(Some(std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests"
    ))));
    let mut moteur = docdg_transpiler::Engine::new();
    let resultat = moteur.render(src, true);
    let charge = format!(
        "{{\"page\":{},\"html\":{},\"stats\":\"pont simulé\"}}",
        resultat.page.to_json(),
        json_str(&resultat.html)
    );

    let pont = format!(
        "window.ipc = {{ postMessage: function (brut) {{\n  var m = JSON.parse(brut);\n  if (m.cmd === 'render') {{ setTimeout(function () {{ window.onTranspiled({}); }}, 0); }}\n  else {{ document.getElementById('status').textContent = 'commande ' + m.cmd + ' (simulée)'; }}\n}} }};\n",
        charge
    );

    let script = format!("{}\n{}", pont, js);
    let sortie = index
        .replacen("/*__STYLE_CSS__*/", css, 1)
        .replacen("/*__SCRIPT_JS__*/", &script, 1)
        .replacen("<!--__MATH__-->", &math, 1)
        .replacen(
            "requestTranspile('full');",
            &format!("editor.value = {};\nrequestTranspile('full');", json_str(src)),
            1,
        );

    let dest = std::env::args().nth(1).unwrap_or_else(|| "apercu.html".into());
    std::fs::write(&dest, sortie).expect("écriture");
    println!("écrit: {}", dest);
}
