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

fn main() {
    let racine = racine();
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
