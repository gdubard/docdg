use serde::Deserialize;
use std::cell::RefCell;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use tao::window::WindowBuilder;
use muda::{AboutMetadata, Menu, PredefinedMenuItem, Submenu};
use wry::http::Request;
use wry::{WebView, WebViewBuilder};

const INDEX_HTML: &str = include_str!("static/index.html");
const STYLE_CSS: &str = include_str!("static/style.css");
const APP_JS: &str = include_str!("static/app.js");
const KATEX_CSS: &str = include_str!("static/katex/katex.min.css");
const KATEX_JS: &str = include_str!("static/katex/katex.min.js");
const KATEX_AUTO: &str = include_str!("static/katex/auto-render.min.js");

const POLICES: &[(&str, &[u8])] = &[
    ("KaTeX_AMS-Regular", include_bytes!("static/katex/fonts/KaTeX_AMS-Regular.woff2") as &[u8]),
    ("KaTeX_Caligraphic-Bold", include_bytes!("static/katex/fonts/KaTeX_Caligraphic-Bold.woff2") as &[u8]),
    ("KaTeX_Caligraphic-Regular", include_bytes!("static/katex/fonts/KaTeX_Caligraphic-Regular.woff2") as &[u8]),
    ("KaTeX_Fraktur-Bold", include_bytes!("static/katex/fonts/KaTeX_Fraktur-Bold.woff2") as &[u8]),
    ("KaTeX_Fraktur-Regular", include_bytes!("static/katex/fonts/KaTeX_Fraktur-Regular.woff2") as &[u8]),
    ("KaTeX_Main-Bold", include_bytes!("static/katex/fonts/KaTeX_Main-Bold.woff2") as &[u8]),
    ("KaTeX_Main-BoldItalic", include_bytes!("static/katex/fonts/KaTeX_Main-BoldItalic.woff2") as &[u8]),
    ("KaTeX_Main-Italic", include_bytes!("static/katex/fonts/KaTeX_Main-Italic.woff2") as &[u8]),
    ("KaTeX_Main-Regular", include_bytes!("static/katex/fonts/KaTeX_Main-Regular.woff2") as &[u8]),
    ("KaTeX_Math-BoldItalic", include_bytes!("static/katex/fonts/KaTeX_Math-BoldItalic.woff2") as &[u8]),
    ("KaTeX_Math-Italic", include_bytes!("static/katex/fonts/KaTeX_Math-Italic.woff2") as &[u8]),
    ("KaTeX_SansSerif-Bold", include_bytes!("static/katex/fonts/KaTeX_SansSerif-Bold.woff2") as &[u8]),
    ("KaTeX_SansSerif-Italic", include_bytes!("static/katex/fonts/KaTeX_SansSerif-Italic.woff2") as &[u8]),
    ("KaTeX_SansSerif-Regular", include_bytes!("static/katex/fonts/KaTeX_SansSerif-Regular.woff2") as &[u8]),
    ("KaTeX_Script-Regular", include_bytes!("static/katex/fonts/KaTeX_Script-Regular.woff2") as &[u8]),
    ("KaTeX_Size1-Regular", include_bytes!("static/katex/fonts/KaTeX_Size1-Regular.woff2") as &[u8]),
    ("KaTeX_Size2-Regular", include_bytes!("static/katex/fonts/KaTeX_Size2-Regular.woff2") as &[u8]),
    ("KaTeX_Size3-Regular", include_bytes!("static/katex/fonts/KaTeX_Size3-Regular.woff2") as &[u8]),
    ("KaTeX_Size4-Regular", include_bytes!("static/katex/fonts/KaTeX_Size4-Regular.woff2") as &[u8]),
    ("KaTeX_Typewriter-Regular", include_bytes!("static/katex/fonts/KaTeX_Typewriter-Regular.woff2") as &[u8]),
];

fn base64(donnees: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut sortie = String::with_capacity((donnees.len() + 2) / 3 * 4);
    for morceau in donnees.chunks(3) {
        let a = morceau[0] as u32;
        let b = *morceau.get(1).unwrap_or(&0) as u32;
        let c = *morceau.get(2).unwrap_or(&0) as u32;
        let bloc = (a << 16) | (b << 8) | c;
        sortie.push(TABLE[(bloc >> 18) as usize & 63] as char);
        sortie.push(TABLE[(bloc >> 12) as usize & 63] as char);
        sortie.push(if morceau.len() > 1 {
            TABLE[(bloc >> 6) as usize & 63] as char
        } else {
            '='
        });
        sortie.push(if morceau.len() > 2 {
            TABLE[bloc as usize & 63] as char
        } else {
            '='
        });
    }
    sortie
}

fn katex() -> String {
    let mut css = KATEX_CSS.to_string();
    for (nom, octets) in POLICES {
        let ancien = format!(
            "url(fonts/{n}.woff2) format(\"woff2\"),url(fonts/{n}.woff) format(\"woff\"),url(fonts/{n}.ttf) format(\"truetype\")",
            n = nom
        );
        let nouveau = format!(
            "url(data:font/woff2;base64,{}) format(\"woff2\")",
            base64(octets)
        );
        css = css.replace(&ancien, &nouveau);
    }
    format!(
        "<style>{}</style>\n<script>{}</script>\n<script>{}</script>",
        css, KATEX_JS, KATEX_AUTO
    )
}


#[derive(Deserialize)]
struct IpcMessage {
    cmd: String,
    #[serde(default)]
    content: String,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    saisies: std::collections::BTreeMap<String, String>,
}

enum Demande {
    Rendu {
        source: String,
        parallele: bool,
        saisies: std::collections::BTreeMap<String, String>,
    },
    Oublie,
}

enum Reponse {
    Rendu(String),
    Message(String, bool),
    Quitter,
}

fn json_str(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".into())
}


fn chemin_prefs() -> Option<PathBuf> {
    #[cfg(unix)]
    let base = std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config"));
    #[cfg(windows)]
    let base = std::env::var_os("APPDATA").map(PathBuf::from);
    let dossier = base?.join("docdg");
    std::fs::create_dir_all(&dossier).ok()?;
    Some(dossier.join("interface.json"))
}

fn lit_prefs() -> String {
    chemin_prefs()
        .and_then(|c| std::fs::read_to_string(c).ok())
        .filter(|s| serde_json::from_str::<serde_json::Value>(s).is_ok())
        .unwrap_or_else(|| "{}".into())
}

fn ecrit_prefs(contenu: &str) {
    if serde_json::from_str::<serde_json::Value>(contenu).is_err() {
        return;
    }
    if let Some(chemin) = chemin_prefs() {
        let _ = std::fs::write(chemin, contenu);
    }
}

fn build_html() -> String {
    INDEX_HTML
        .replacen("/*__STYLE_CSS__*/", STYLE_CSS, 1)
        .replacen("/*__SCRIPT_JS__*/", APP_JS, 1)
        .replacen(
            "/*__PREFS__*/",
            &format!("window.__PREFS__ = {};", lit_prefs()),
            1,
        )
        .replacen("<!--__MATH__-->", &katex(), 1)
}

fn dossier_travail() -> Result<PathBuf, String> {
    let dossier = std::env::temp_dir().join(format!("docdg-{}", std::process::id()));
    std::fs::create_dir_all(&dossier).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dossier, std::fs::Permissions::from_mode(0o700));
    }
    Ok(dossier)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn cherche_dans_path(nom: &str) -> Option<PathBuf> {
    let chemins = std::env::var_os("PATH")?;
    std::env::split_paths(&chemins)
        .map(|d| d.join(nom))
        .find(|c| c.is_file())
}

fn navigateur() -> Option<PathBuf> {
    static TROUVE: OnceLock<Option<PathBuf>> = OnceLock::new();
    TROUVE
        .get_or_init(|| {
            #[cfg(target_os = "macos")]
            {
                [
                    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                    "/Applications/Chromium.app/Contents/MacOS/Chromium",
                    "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
                    "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
                ]
                    .iter()
                    .map(PathBuf::from)
                    .find(|c| c.is_file())
            }
            #[cfg(target_os = "windows")]
            {
                let sous = [
                    r"Google\Chrome\Application\chrome.exe",
                    r"Microsoft\Edge\Application\msedge.exe",
                    r"BraveSoftware\Brave-Browser\Application\brave.exe",
                    r"Chromium\Application\chrome.exe",
                ];
                ["PROGRAMFILES", "PROGRAMFILES(X86)", "LOCALAPPDATA"]
                    .iter()
                    .filter_map(std::env::var_os)
                    .flat_map(|base| {
                        let base = PathBuf::from(base);
                        sous.iter().map(move |s| base.join(s)).collect::<Vec<_>>()
                    })
                    .find(|c| c.is_file())
            }
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                [
                    "google-chrome-stable",
                    "google-chrome",
                    "chromium-browser",
                    "chromium",
                    "microsoft-edge",
                    "brave-browser",
                ]
                    .iter()
                    .find_map(|n| cherche_dans_path(n))
            }
        })
        .clone()
}

fn url_fichier(chemin: &Path) -> String {
    let brut = chemin.display().to_string().replace('\\', "/");
    if brut.starts_with('/') {
        format!("file://{}", brut)
    } else {
        format!("file:///{}", brut)
    }
}






type Ws = WebSocket<MaybeTlsStream<TcpStream>>;

struct Fournaise {
    _fils: Child,
    ws: Ws,
    dossier: PathBuf,
    derniere: Instant,
}

static IDENTIFIANT: AtomicU64 = AtomicU64::new(1);

fn verrou<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn braise() -> &'static Mutex<Option<Fournaise>> {
    static B: OnceLock<Mutex<Option<Fournaise>>> = OnceLock::new();
    B.get_or_init(|| Mutex::new(None))
}

fn decode_base64(texte: &str) -> Option<Vec<u8>> {
    let mut table = [255u8; 256];
    for (i, c) in
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate()
    {
        table[*c as usize] = i as u8;
    }
    let mut sortie = Vec::with_capacity(texte.len() / 4 * 3);
    let mut tampon = 0u32;
    let mut bits = 0u32;
    for o in texte.bytes() {
        if o == b'=' || o == b'\n' || o == b'\r' {
            continue;
        }
        let v = table[o as usize];
        if v == 255 {
            return None;
        }
        tampon = (tampon << 6) | v as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            sortie.push((tampon >> bits) as u8);
        }
    }
    Some(sortie)
}

fn messages_jusqu_a(
    ws: &mut Ws,
    plainte: &str,
    mut retient: impl FnMut(&serde_json::Value) -> Option<Result<serde_json::Value, String>>,
) -> Result<serde_json::Value, String> {
    let butoir = Instant::now() + Duration::from_secs(30);
    loop {
        if Instant::now() > butoir {
            return Err(plainte.to_string());
        }
        let recu = ws.read().map_err(|e| e.to_string())?;
        let texte = match recu {
            Message::Text(t) => t,
            _ => continue,
        };
        let valeur: serde_json::Value = match serde_json::from_str(&texte) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if let Some(issue) = retient(&valeur) {
            return issue;
        }
    }
}

fn envoie(
    ws: &mut Ws,
    session: Option<&str>,
    methode: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let id = IDENTIFIANT.fetch_add(1, Ordering::Relaxed);
    let mut message = serde_json::json!({"id": id, "method": methode, "params": params});
    if let Some(s) = session {
        message["sessionId"] = serde_json::Value::String(s.to_string());
    }
    ws.send(Message::Text(message.to_string().into()))
        .map_err(|e| e.to_string())?;
    messages_jusqu_a(ws, "le navigateur ne répond plus", |valeur| {
        if valeur.get("id").and_then(|v| v.as_u64()) != Some(id) {
            return None;
        }
        if let Some(erreur) = valeur.get("error") {
            let detail = erreur
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("erreur du protocole de pilotage");
            return Some(Err(detail.to_string()));
        }
        Some(Ok(valeur.get("result").cloned().unwrap_or(serde_json::Value::Null)))
    })
}

fn attend_evenement(ws: &mut Ws, session: &str, nom: &str) -> Result<(), String> {
    messages_jusqu_a(ws, "chargement du document interrompu", |valeur| {
        let atteint = valeur.get("method").and_then(|v| v.as_str()) == Some(nom)
            && valeur.get("sessionId").and_then(|v| v.as_str()) == Some(session);
        atteint.then(|| Ok(serde_json::Value::Null))
    })
        .map(|_| ())
}

fn commande_navigateur(chrome: &Path, profil: &Path) -> Command {
    let mut c = Command::new(chrome);
    c.arg("--headless")
        .arg("--disable-gpu")
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--no-service-autorun")
        .arg("--disable-background-networking")
        .arg("--disable-component-update")
        .arg("--disable-client-side-phishing-detection")
        .arg("--disable-default-apps")
        .arg("--disable-extensions")
        .arg("--disable-sync")
        .arg("--disable-crash-reporter")
        .arg("--disable-logging")
        .arg("--log-level=3")
        .arg("--mute-audio")
        .arg(format!("--user-data-dir={}", profil.display()));
    c
}

fn allume(chrome: &Path, dossier: &Path) -> Result<Fournaise, String> {
    std::fs::create_dir_all(dossier).map_err(|e| e.to_string())?;
    let indicateur = dossier.join("DevToolsActivePort");
    let _ = std::fs::remove_file(&indicateur);
    let fils = commande_navigateur(chrome, dossier)
        .arg("--remote-debugging-port=0")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    let butoir = Instant::now() + Duration::from_secs(20);
    let (port, chemin_ws) = loop {
        if Instant::now() > butoir {
            return Err("le navigateur n'a pas ouvert son port de pilotage".into());
        }
        if let Ok(contenu) = std::fs::read_to_string(&indicateur) {
            let mut lignes = contenu.lines();
            if let (Some(p), Some(c)) = (lignes.next(), lignes.next()) {
                if let Ok(port) = p.trim().parse::<u16>() {
                    break (port, c.trim().to_string());
                }
            }
        }
        std::thread::sleep(Duration::from_millis(25));
    };
    let flux = TcpStream::connect(("127.0.0.1", port)).map_err(|e| e.to_string())?;
    flux.set_nodelay(true).ok();
    flux.set_read_timeout(Some(Duration::from_secs(30))).ok();
    let adresse = format!("ws://127.0.0.1:{}{}", port, chemin_ws);
    let (ws, _) = tungstenite::client(adresse.as_str(), MaybeTlsStream::Plain(flux))
        .map_err(|e| e.to_string())?;
    Ok(Fournaise {
        _fils: fils,
        ws,
        dossier: dossier.to_path_buf(),
        derniere: Instant::now(),
    })
}

fn imprime(f: &mut Fournaise, url: &str) -> Result<Vec<u8>, String> {
    let cible = envoie(
        &mut f.ws,
        None,
        "Target.createTarget",
        serde_json::json!({"url": "about:blank"}),
    )?;
    let cible = cible
        .get("targetId")
        .and_then(|v| v.as_str())
        .ok_or("cible introuvable")?
        .to_string();
    let issue = (|| {
        let attache = envoie(
            &mut f.ws,
            None,
            "Target.attachToTarget",
            serde_json::json!({"targetId": cible, "flatten": true}),
        )?;
        let session = attache
            .get("sessionId")
            .and_then(|v| v.as_str())
            .ok_or("session introuvable")?
            .to_string();
        envoie(&mut f.ws, Some(&session), "Page.enable", serde_json::json!({}))?;
        envoie(
            &mut f.ws,
            Some(&session),
            "Page.navigate",
            serde_json::json!({"url": url}),
        )?;
        attend_evenement(&mut f.ws, &session, "Page.loadEventFired")?;
        let resultat = envoie(
            &mut f.ws,
            Some(&session),
            "Page.printToPDF",
            serde_json::json!({
                "printBackground": true,
                "preferCSSPageSize": true,
                "displayHeaderFooter": false
            }),
        )?;
        let donnees = resultat
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or("réponse sans document")?;
        decode_base64(donnees).ok_or_else(|| "document illisible".to_string())
    })();
    let _ = envoie(
        &mut f.ws,
        None,
        "Target.closeTarget",
        serde_json::json!({"targetId": cible}),
    );
    issue
}

const SOMMEIL: Duration = Duration::from_secs(300);

fn souffle(place: &mut Option<Fournaise>) {
    if let Some(mut f) = place.take() {
        let _ = envoie(&mut f.ws, None, "Browser.close", serde_json::json!({}));
        let _ = f._fils.wait();
        let _ = std::fs::remove_dir_all(&f.dossier);
    }
}

fn veilleur() {
    static LANCE: OnceLock<()> = OnceLock::new();
    LANCE.get_or_init(|| {
        std::thread::spawn(|| loop {
            std::thread::sleep(Duration::from_secs(60));
            let mut place = verrou(braise());
            let dormante = place
                .as_ref()
                .map(|f| f.derniere.elapsed() > SOMMEIL)
                .unwrap_or(false);
            if dormante {
                souffle(&mut place);
            }
        });
    });
}

fn exporte_chaud(chrome: &Path, dossier: &Path, url: &str) -> Result<Vec<u8>, String> {
    veilleur();
    let mut place = verrou(braise());
    if place.is_none() {
        *place = Some(allume(chrome, dossier)?);
    }
    let premier = imprime(place.as_mut().unwrap(), url);
    if let Ok(pdf) = premier {
        place.as_mut().unwrap().derniere = Instant::now();
        return Ok(pdf);
    }
    *place = None;
    *place = Some(allume(chrome, dossier)?);
    let pdf = imprime(place.as_mut().unwrap(), url)?;
    place.as_mut().unwrap().derniere = Instant::now();
    Ok(pdf)
}

fn eteint() {
    souffle(&mut verrou(braise()));
}

static EXPORTS: AtomicUsize = AtomicUsize::new(0);

fn html_vers_pdf(html: &str) -> Result<Vec<u8>, String> {
    let chrome = navigateur().ok_or_else(|| {
        String::from("Aucun navigateur Chromium (Chrome, Chromium, Edge ou Brave) n'a été trouvé.")
    })?;
    let numero = EXPORTS.fetch_add(1, Ordering::Relaxed);
    let dossier = dossier_travail()?.join(format!("export-{}", numero));
    let _ = std::fs::remove_dir_all(&dossier);
    std::fs::create_dir_all(&dossier).map_err(|e| e.to_string())?;
    let source = dossier.join("document.html");
    std::fs::write(&source, html).map_err(|e| e.to_string())?;
    let braise = dossier_travail()?.join("fournaise");
    let issue = match exporte_chaud(&chrome, &braise, &url_fichier(&source)) {
        Ok(pdf) => Ok(pdf),
        Err(_) => pdf_froid(&chrome, &dossier, &source),
    };
    let _ = std::fs::remove_dir_all(&dossier);
    issue
}

fn pdf_froid(chrome: &Path, dossier: &Path, source: &Path) -> Result<Vec<u8>, String> {
    let sortie = dossier.join("document.pdf");
    let journal = dossier.join("chrome.log");
    let trace = std::fs::File::create(&journal).map_err(|e| e.to_string())?;
    match commande_navigateur(chrome, &dossier.join("profil"))
        .arg("--no-pdf-header-footer")
        .arg(format!("--print-to-pdf={}", sortie.display()))
        .arg(url_fichier(&source))
        .stdout(Stdio::null())
        .stderr(Stdio::from(trace))
        .status()
    {
        Ok(e) if e.success() => std::fs::read(&sortie).map_err(|e| e.to_string()),
        Ok(e) => Err(format!(
            "{} s'est terminé avec le code {:?}{}",
            chrome.display(),
            e.code(),
            derniere_plainte(&journal)
        )),
        Err(e) => Err(e.to_string()),
    }
}

fn derniere_plainte(journal: &Path) -> String {
    let texte = match std::fs::read_to_string(journal) {
        Ok(t) => t,
        Err(_) => return String::new(),
    };
    match texte
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .map(str::trim)
    {
        Some(l) => format!(" — {}", l),
        None => String::new(),
    }
}

fn compositeur(proxy: EventLoopProxy<Reponse>) -> Sender<Demande> {
    let (envoi, reception) = channel::<Demande>();
    std::thread::spawn(move || {
        let mut moteur = docdg_transpiler::Engine::new();
        while let Ok(premiere) = reception.recv() {
            let mut demande = premiere;
            while let Ok(suivante) = reception.try_recv() {
                if let Demande::Oublie = demande {
                    moteur.clear_cache();
                }
                demande = suivante;
            }
            match demande {
                Demande::Oublie => moteur.clear_cache(),
                Demande::Rendu { source, parallele, saisies } => {
                    let t0 = Instant::now();
                    moteur.saisies = saisies;
                    let mut resultat = moteur.render(&source, parallele);
                    if resultat.page.cesure {
                        resultat.html = docdg_transpiler::cesure_html(&resultat.html);
                    }
                    let stats = format!(
                        "{} — {:.1} ms",
                        if parallele { "complet" } else { "incrémental" },
                        t0.elapsed().as_secs_f64() * 1000.0
                    );
                    let charge = format!(
                        "{{\"page\":{},\"html\":{},\"stats\":{}}}",
                        resultat.page.to_json(),
                        json_str(&resultat.html),
                        json_str(&stats)
                    );
                    if proxy.send_event(Reponse::Rendu(charge)).is_err() {
                        break;
                    }
                }
            }
        }
    });
    envoi
}

fn menu_edition(fenetre: &tao::window::Window) {
    let _ = fenetre;
    let barre = Menu::new();

    let application = Submenu::new("DocDG", true);
    let _ = application.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("DocDG".into()),
                version: Some(env!("CARGO_PKG_VERSION").into()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::hide(None),
        &PredefinedMenuItem::hide_others(None),
        &PredefinedMenuItem::show_all(None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::quit(Some("Quitter DocDG")),
    ]);

    let edition = Submenu::new("Édition", true);
    let _ = edition.append_items(&[
        &PredefinedMenuItem::cut(Some("Couper")),
        &PredefinedMenuItem::copy(Some("Copier")),
        &PredefinedMenuItem::paste(Some("Coller")),
        &PredefinedMenuItem::select_all(Some("Tout sélectionner")),
    ]);

    let _ = barre.append_items(&[&application, &edition]);

    #[cfg(target_os = "macos")]
    barre.init_for_nsapp();

    #[cfg(target_os = "windows")]
    {
        use tao::platform::windows::WindowExtWindows;
        let _ = unsafe { barre.init_for_hwnd(fenetre.hwnd() as _) };
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        use tao::platform::unix::WindowExtUnix;
        let _ = barre.init_for_gtk_window(fenetre.gtk_window(), fenetre.default_vbox());
    }
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<Reponse>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title("DocDG")
        .with_inner_size(tao::dpi::LogicalSize::new(1200.0, 900.0))
        .build(&event_loop)
        .unwrap();

    menu_edition(&window);

    let proxy = event_loop.create_proxy();
    let vers_compositeur = compositeur(proxy.clone());
    let fenetre: Rc<RefCell<Option<WebView>>> = Rc::new(RefCell::new(None));
    let cible = fenetre.clone();
    let quitte = Rc::new(RefCell::new(false));
    let demande_quitter = quitte.clone();
    let derniere_demande: Rc<RefCell<Option<Instant>>> =
        Rc::new(RefCell::new(None));

    let builder = WebViewBuilder::new()
        .with_html(build_html())
        .with_devtools(cfg!(debug_assertions))
        .with_ipc_handler(move |requete: Request<String>| {
            let msg: IpcMessage = match serde_json::from_str(requete.body()) {
                Ok(m) => m,
                Err(_) => return,
            };
            let repondre = |texte: String, ok: bool| {
                let _ = proxy.send_event(Reponse::Message(texte, ok));
            };

            match msg.cmd.as_str() {
                "render" => {
                    let _ = vers_compositeur.send(Demande::Rendu {
                        source: msg.content,
                        parallele: msg.mode == "full",
                        saisies: msg.saisies,
                    });
                }
                "load" => {
                    if let Some(chemin) = rfd::FileDialog::new()
                        .add_filter("texecole", &["txt", "md", "eco"])
                        .pick_file()
                    {
                        match std::fs::read_to_string(&chemin) {
                            Ok(contenu) => {
                                let dossier = chemin.parent().map(Path::to_path_buf);
                                docdg_transpiler::set_base_dir(dossier);
                                let _ = vers_compositeur.send(Demande::Oublie);
                                if let Some(vue) = cible.borrow().as_ref() {
                                    let _ = vue.evaluate_script(&format!(
                                        "setEditorContent({});",
                                        json_str(&contenu)
                                    ));
                                }
                            }
                            Err(e) => repondre(
                                format!("Lecture impossible — {}", e),
                                false,
                            ),
                        }
                    }
                }
                "save" => {
                    if let Some(chemin) = rfd::FileDialog::new()
                        .set_file_name("document.txt")
                        .save_file()
                    {
                        match std::fs::write(&chemin, &msg.content) {
                            Ok(_) => {
                                let dossier = chemin.parent().map(Path::to_path_buf);
                                docdg_transpiler::set_base_dir(dossier);
                                let _ = vers_compositeur.send(Demande::Oublie);
                                repondre(format!("Enregistré : {}", chemin.display()), true);
                            }
                            Err(e) => repondre(
                                format!("Enregistrement impossible — {}", e),
                                false,
                            ),
                        }
                    }
                }
                "export" => {
                    let destination = rfd::FileDialog::new()
                        .set_file_name("document.pdf")
                        .add_filter("PDF", &["pdf"])
                        .save_file();
                    let destination = match destination {
                        Some(d) => d,
                        None => {
                            repondre(String::from("Export annulé."), true);
                            return;
                        }
                    };
                    let source = msg.content;
                    let proxy_export = proxy.clone();
                    std::thread::spawn(move || {
                        let issue = match html_vers_pdf(&source) {
                            Ok(pdf) => match std::fs::write(&destination, pdf) {
                                Ok(_) => (format!("PDF écrit : {}", destination.display()), true),
                                Err(e) => (format!("Écriture du PDF impossible — {}", e), false),
                            },
                            Err(e) => (format!("Génération du PDF impossible — {}", e), false),
                        };
                        let _ = proxy_export.send_event(Reponse::Message(issue.0, issue.1));
                    });
                }
                "prefs" => {
                    ecrit_prefs(&msg.content);
                }
                "quitter" => {
                    let _ = proxy.send_event(Reponse::Quitter);
                }
                _ => {}
            }
        });

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let webview = builder.build(&window)?;
    #[cfg(all(unix, not(target_os = "macos")))]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        builder.build_gtk(window.default_vbox().unwrap())?
    };

    *fenetre.borrow_mut() = Some(webview);

    event_loop.run(move |evenement, _, flux| {
        *flux = ControlFlow::Wait;
        match evenement {
            Event::UserEvent(Reponse::Rendu(charge)) => {
                if let Some(vue) = fenetre.borrow().as_ref() {
                    let _ = vue.evaluate_script(&format!("window.onTranspiled({});", charge));
                }
            }
            Event::UserEvent(Reponse::Message(texte, ok)) => {
                if let Some(vue) = fenetre.borrow().as_ref() {
                    let _ = vue.evaluate_script(&format!(
                        "window.onMessage({}, {});",
                        json_str(&texte),
                        ok
                    ));
                }
            }
            Event::UserEvent(Reponse::Quitter) => {
                *demande_quitter.borrow_mut() = true;
                eteint();
                *flux = ControlFlow::Exit;
            }
            Event::LoopDestroyed => eteint(),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let demande = Instant::now();
                let precedente = *derniere_demande.borrow();
                let recent = match precedente {
                    Some(t) => demande.duration_since(t).as_secs_f32() < 2.0,
                    None => false,
                };
                let interroge = fenetre
                    .borrow()
                    .as_ref()
                    .map(|vue| {
                        vue.evaluate_script(
                            "if (window.demandeFermeture) { window.demandeFermeture(); }",
                        )
                        .is_ok()
                    })
                    .unwrap_or(false);
                if *demande_quitter.borrow() || recent || !interroge {
                    eteint();
                    *flux = ControlFlow::Exit;
                } else {
                    *derniere_demande.borrow_mut() = Some(demande);
                }
            }
            _ => {}
        }
    });
}
