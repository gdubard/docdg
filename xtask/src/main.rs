mod couverture;
mod recensement;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn racine() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn cargo(racine: &Path, args: &[&str]) {
    let etat = Command::new(env!("CARGO"))
        .current_dir(racine)
        .args(args)
        .status()
        .expect("cargo introuvable");
    if !etat.success() {
        std::process::exit(etat.code().unwrap_or(1));
    }
}

fn copie(de: &Path, vers: &Path) {
    fs::create_dir_all(vers.parent().unwrap()).unwrap();
    fs::copy(de, vers).unwrap_or_else(|e| panic!("{} → {} : {}", de.display(), vers.display(), e));
}

fn empaquete_macos(racine: &Path) {
    let app = racine.join("docdg.app");
    let _ = fs::remove_dir_all(&app);
    let macos = app.join("Contents/MacOS");
    let ressources = app.join("Contents/Resources");
    fs::create_dir_all(&macos).unwrap();
    fs::create_dir_all(&ressources).unwrap();

    copie(&racine.join("target/release/docdg"), &macos.join("docdg"));
    copie(&racine.join("app/macos/Info.plist"), &app.join("Contents/Info.plist"));
    copie(&racine.join("app/icones/docdg.icns"), &ressources.join("docdg.icns"));
    fs::write(app.join("Contents/PkgInfo"), "APPL????").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let bin = macos.join("docdg");
        let mut p = fs::metadata(&bin).unwrap().permissions();
        p.set_mode(0o755);
        fs::set_permissions(&bin, p).unwrap();
    }

    let _ = Command::new("codesign")
        .current_dir(racine)
        .args(["--force", "--deep", "--sign", "-", "docdg.app"])
        .status();
    let _ = Command::new("touch").arg(&app).status();

    println!("docdg.app est prêt : {}", app.display());
    println!("Double-cliquez-le dans le Finder, ou glissez-le dans /Applications.");
}

fn installe_linux(racine: &Path) {
    let binaire = racine.join("target/release/docdg");
    let icone = racine.join("app/icones/docdg.svg");
    let Some(maison) = std::env::var_os("HOME").map(PathBuf::from) else {
        println!("docdg est prêt : {}", binaire.display());
        println!("HOME introuvable : entrée de bureau non installée.");
        return;
    };

    let icones = maison.join(".local/share/icons/hicolor/scalable/apps");
    fs::create_dir_all(&icones).unwrap();
    copie(&icone, &icones.join("docdg.svg"));

    let entree = format!(
        "[Desktop Entry]\n\
Type=Application\n\
Version=1.0\n\
Name=docdg\n\
GenericName=Préparation de documents\n\
Comment=Composer des documents de mathématiques en français\n\
Exec={} %f\n\
Icon=docdg\n\
Terminal=false\n\
StartupNotify=true\n\
StartupWMClass=docdg\n\
Categories=Education;Math;\n\
MimeType=text/plain;\n",
        binaire.display()
    );
    let applications = maison.join(".local/share/applications");
    fs::create_dir_all(&applications).unwrap();
    let cible = applications.join("docdg.desktop");
    fs::write(&cible, entree).unwrap();

    let _ = Command::new("update-desktop-database").arg(&applications).status();
    let _ = Command::new("gtk-update-icon-cache")
        .args(["-f", "-t"])
        .arg(maison.join(".local/share/icons/hicolor"))
        .status();

    println!("docdg est prêt : {}", binaire.display());
    println!("Entrée de bureau installée : {}", cible.display());
    println!("docdg apparaît dans le menu des applications, avec son logo.");
}

fn valide_corpus(racine: &Path) {
    println!("Validation du corpus…");
    let etat = Command::new(env!("CARGO"))
        .current_dir(racine)
        .args(["check", "-p", "docdg-corpus"])
        .status();
    match etat {
        Ok(s) if s.success() => println!("Corpus valide."),
        _ => {
            eprintln!("Corpus invalide : voir les erreurs ci-dessus.");
            std::process::exit(1);
        }
    }
}

fn main() {
    let racine = racine();

    if std::env::args().nth(1).as_deref() == Some("personnelle") {
        let chemin = std::env::args()
            .nth(2)
            .map(std::path::PathBuf::from)
            .unwrap_or_else(docdg_corpus::chemin_personnelle);
        if !chemin.is_file() {
            println!("{} n'existe pas encore — rien n'a été enregistré.", chemin.display());
            return;
        }
        match docdg_corpus::scellement::ouvre_scelle_en_memoire(&chemin) {
            Ok(conn) => match docdg_corpus::basedonnees::lit(&conn) {
                Ok(c) => {
                    println!(
                        "{} — {} énoncé(s), {} démonstration(s), {} grandeur(s), {} cadre(s)\n",
                        chemin.display(),
                        c.enonces.len(),
                        c.demonstrations.len(),
                        c.grandeurs.len(),
                        c.cadres.len()
                    );
                    for e in &c.enonces {
                        println!("[[enonce]]");
                        println!("id = {:?}", e.id);
                        println!("type = {:?}", e.genre);
                        println!("titre = {:?}", e.titre);
                        println!("matiere = {:?}", e.matiere.nom());
                        println!("domaine = {:?}", e.domaine);
                        if !e.formule.is_empty() {
                            println!("formule = {:?}", e.formule);
                        }
                        if !e.relation.is_empty() {
                            println!("relation = {:?}", e.relation);
                        }
                        if !e.symboles.is_empty() {
                            println!("symboles = {:?}", e.symboles);
                        }
                        if !e.hypotheses.is_empty() {
                            println!("hypotheses = {:?}", e.hypotheses);
                        }
                        if !e.validite.is_empty() {
                            println!("validite = {:?}", e.validite);
                        }
                        if !e.depend_de.is_empty() {
                            println!("depend_de = {:?}", e.depend_de);
                        }
                        if !e.alias.is_empty() {
                            println!("alias = {:?}", e.alias);
                        }
                        if !e.notes.is_empty() {
                            println!("notes = {:?}", e.notes);
                        }
                        println!("enonce = {:?}", e.texte);
                        for a in &e.ancrages {
                            println!(
                                "[[enonce.ancrage]] niveau = {:?} statut = {:?} voie = {:?} demonstration_prevue = {}",
                                a.niveau.nom(),
                                match a.statut {
                                    docdg_corpus::Statut::Démontré => "démontré",
                                    docdg_corpus::Statut::Admis => "admis",
                                },
                                a.voie.as_ref().map(|v| v.0.as_str()),
                                a.demonstration_prevue
                            );
                        }
                        println!();
                    }
                    for d in &c.demonstrations {
                        println!("[[demonstration]]");
                        println!("id = {:?}", d.id);
                        println!("enonce_id = {:?}", d.enonce_id);
                        if !d.titre.is_empty() {
                            println!("titre = {:?}", d.titre);
                        }
                        if !d.methode.is_empty() {
                            println!("methode = {:?}", d.methode);
                        }
                        println!("niveau_redaction = {:?}", d.niveau_redaction.nom());
                        if !d.depend_de.is_empty() {
                            println!("depend_de = {:?}", d.depend_de);
                        }
                        if !d.mentions.is_empty() {
                            println!("mentions = {:?}", d.mentions);
                        }
                        if !d.notes.is_empty() {
                            println!("notes = {:?}", d.notes);
                        }
                        println!("corps = {:?}", d.corps);
                        println!();
                    }
                    let mut stmt = conn.prepare("SELECT espece, id FROM supprime").unwrap();
                    let mut lignes = stmt.query([]).unwrap();
                    let mut premiere = true;
                    while let Some(l) = lignes.next().unwrap() {
                        if premiere {
                            println!("[[supprime]]");
                            premiere = false;
                        }
                        let espece: String = l.get(0).unwrap();
                        let id: String = l.get(1).unwrap();
                        println!("{{ espece = {espece:?}, id = {id:?} }}");
                    }
                }
                Err(e) => {
                    eprintln!("lecture : {e}");
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("descellement : {e}");
                std::process::exit(1);
            }
        }
        return;
    }

    if std::env::args().nth(1).as_deref() == Some("corpus") {
        valide_corpus(&racine);
        return;
    }

    if std::env::args().nth(1).as_deref() == Some("couverture") {
        couverture::couverture(&racine);
        return;
    }

    if std::env::args().nth(1).as_deref() == Some("recensement") {
        if std::env::args().nth(2).as_deref() == Some("--redaction") {
            recensement::redaction(&racine);
        } else {
            recensement::recensement();
        }
        return;
    }

    cargo(&racine, &["build", "--release", "-p", "docdg"]);

    if cfg!(target_os = "macos") {
        empaquete_macos(&racine);
    } else if cfg!(windows) {
        println!("docdg.exe est prêt : target/release/docdg.exe");
        println!("L'icone et l'absence de console sont gravees dans l'executable.");
    } else {
        installe_linux(&racine);
    }
}
