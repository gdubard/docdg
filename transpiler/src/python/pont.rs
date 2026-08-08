use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::Duration;

const WORKER_PY: &str = include_str!("cas.py");
const OUVRIERS_MAX: usize = 4;
const ARCHIVE_MAX: u64 = 8 * 1024 * 1024;

struct Ouvrier {
    fils: Child,
    entree: ChildStdin,
    sortie: BufReader<ChildStdout>,
}

impl Drop for Ouvrier {
    fn drop(&mut self) {
        let _ = self.fils.kill();
        let _ = self.fils.wait();
    }
}

struct Bassin {
    envoi: Sender<Ouvrier>,
    reception: Mutex<Receiver<Ouvrier>>,
}

static VIVANTS: AtomicUsize = AtomicUsize::new(0);
static EN_ROUTE: AtomicUsize = AtomicUsize::new(0);
static ECHECS: AtomicUsize = AtomicUsize::new(0);

fn verrou<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn seme() -> Option<Ouvrier> {
    for py in ["python3", "python"] {
        if let Ok(mut fils) = Command::new(py)
            .arg("-c")
            .arg(WORKER_PY)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            let entree = fils.stdin.take()?;
            let sortie = BufReader::new(fils.stdout.take()?);
            return Some(Ouvrier {
                fils,
                entree,
                sortie,
            });
        }
    }
    None
}

fn plafond() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get().min(OUVRIERS_MAX))
        .unwrap_or(1)
}

fn recrute(envoi: Sender<Ouvrier>) {
    EN_ROUTE.fetch_add(1, Ordering::SeqCst);
    std::thread::spawn(move || {
        match seme() {
            Some(o) => {
                VIVANTS.fetch_add(1, Ordering::SeqCst);
                let _ = envoi.send(o);
            }
            None => {
                ECHECS.fetch_add(1, Ordering::SeqCst);
            }
        }
        EN_ROUTE.fetch_sub(1, Ordering::SeqCst);
    });
}

fn bassin() -> &'static Bassin {
    static B: OnceLock<Bassin> = OnceLock::new();
    B.get_or_init(|| {
        let (envoi, reception) = channel();
        for _ in 0..plafond() {
            recrute(envoi.clone());
        }
        Bassin {
            envoi,
            reception: Mutex::new(reception),
        }
    })
}

pub fn prechauffe() {
    let _ = bassin();
}

pub fn ouvriers_vivants() -> usize {
    VIVANTS.load(Ordering::SeqCst) + EN_ROUTE.load(Ordering::SeqCst)
}

pub fn bassin_ouvert() -> bool {
    ouvriers_vivants() > 0
}

fn memo() -> &'static Mutex<HashMap<String, String>> {
    static M: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(charge_archive()))
}

fn dossier_archive() -> Option<PathBuf> {
    let base = match std::env::var_os("XDG_CACHE_HOME") {
        Some(v) => PathBuf::from(v),
        None => {
            let maison = PathBuf::from(std::env::var_os("HOME")?);
            if cfg!(target_os = "macos") {
                maison.join("Library").join("Caches")
            } else {
                maison.join(".cache")
            }
        }
    };
    Some(base.join("docdg"))
}

fn fichier_archive() -> Option<PathBuf> {
    let dossier = dossier_archive()?;
    std::fs::create_dir_all(&dossier).ok()?;
    Some(dossier.join("calcul.jsonl"))
}

fn signature_moteur() -> String {
    use std::hash::{Hash, Hasher};
    let mut h = seahash::SeaHasher::new();
    WORKER_PY.hash(&mut h);
    format!("docdg-cas-{:016x}", h.finish())
}

fn charge_archive() -> HashMap<String, String> {
    let mut memoire = HashMap::new();
    let chemin = match fichier_archive() {
        Some(c) => c,
        None => return memoire,
    };
    let signature = signature_moteur();
    if std::fs::metadata(&chemin).map(|m| m.len()).unwrap_or(0) > ARCHIVE_MAX {
        let _ = std::fs::write(&chemin, format!("{}\n", signature));
        return memoire;
    }
    let contenu = match std::fs::read_to_string(&chemin) {
        Ok(c) => c,
        Err(_) => {
            let _ = std::fs::write(&chemin, format!("{}\n", signature));
            return memoire;
        }
    };
    if contenu.lines().next() != Some(signature.as_str()) {
        let _ = std::fs::write(&chemin, format!("{}\n", signature));
        return memoire;
    }
    for ligne in contenu.lines().skip(1) {
        if let Ok(serde_json::Value::Array(paire)) = serde_json::from_str(ligne) {
            if let (Some(q), Some(r)) = (
                paire.first().and_then(|v| v.as_str()),
                paire.get(1).and_then(|v| v.as_str()),
            ) {
                memoire.insert(q.to_string(), r.to_string());
            }
        }
    }
    memoire
}

fn archive() -> &'static Mutex<Option<std::fs::File>> {
    static A: OnceLock<Mutex<Option<std::fs::File>>> = OnceLock::new();
    A.get_or_init(|| {
        Mutex::new(fichier_archive().and_then(|c| {
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(c)
                .ok()
        }))
    })
}

fn consigne(requete: &str, reponse: &str) {
    let ligne = match serde_json::to_string(&[requete, reponse]) {
        Ok(l) => l,
        Err(_) => return,
    };
    if let Some(fichier) = verrou(archive()).as_mut() {
        let _ = writeln!(fichier, "{}", ligne);
    }
}

pub fn ask(request: &str) -> Result<String, String> {
    if request.contains("__") {
        return Err("expression refusée : elle contient un mot réservé".into());
    }
    if let Some(connu) = verrou(memo()).get(request).cloned() {
        return Ok(connu);
    }
    let chrono = std::time::Instant::now();
    let issue = interroge(request);
    if std::env::var_os("DOCDG_TRACE").is_some() {
        let mut fin = request.len().min(90);
        while !request.is_char_boundary(fin) {
            fin -= 1;
        }
        eprintln!(
            "[{:?}] {:6.0} ms  {}",
            std::thread::current().id(),
            chrono.elapsed().as_secs_f64() * 1000.0,
            &request[..fin]
        );
    }
    if let Ok(reponse) = &issue {
        verrou(memo()).insert(request.to_string(), reponse.clone());
        consigne(request, reponse);
    }
    issue
}

fn emprunte() -> Result<Ouvrier, String> {
    let bassin = bassin();
    let cible = plafond();
    for _ in 0..150 {
        let attente = {
            let file = verrou(&bassin.reception);
            file.recv_timeout(Duration::from_millis(200))
        };
        if let Ok(ouvrier) = attente {
            return Ok(ouvrier);
        }
        let vivants = VIVANTS.load(Ordering::SeqCst);
        let en_route = EN_ROUTE.load(Ordering::SeqCst);
        if vivants == 0 && en_route == 0 && ECHECS.load(Ordering::SeqCst) > 0 {
            return Err("Python 3 est introuvable sur ce système.".into());
        }
        if vivants + en_route < cible {
            recrute(bassin.envoi.clone());
        }
    }
    Err("le moteur de calcul ne répond plus".into())
}

fn interroge(request: &str) -> Result<String, String> {
    let mut ouvrier = emprunte()?;
    if let Some(connu) = verrou(memo()).get(request).cloned() {
        let _ = bassin().envoi.send(ouvrier);
        return Ok(connu);
    }
    if writeln!(ouvrier.entree, "{}", request).is_err() || ouvrier.entree.flush().is_err() {
        VIVANTS.fetch_sub(1, Ordering::SeqCst);
        recrute(bassin().envoi.clone());
        return Err("le moteur de calcul s'est interrompu".into());
    }
    let mut ligne = String::new();
    if ouvrier.sortie.read_line(&mut ligne).unwrap_or(0) == 0 {
        VIVANTS.fetch_sub(1, Ordering::SeqCst);
        recrute(bassin().envoi.clone());
        return Err("le moteur de calcul n'a pas répondu".into());
    }
    let _ = bassin().envoi.send(ouvrier);
    let valeur: serde_json::Value = match serde_json::from_str(&ligne) {
        Ok(v) => v,
        Err(_) => return Err("réponse illisible du moteur de calcul".into()),
    };
    if let Some(ok) = valeur.get("ok").and_then(|v| v.as_str()) {
        return Ok(ok.to_string());
    }
    Err(valeur
        .get("err")
        .and_then(|v| v.as_str())
        .unwrap_or("calcul impossible")
        .to_string())
}
