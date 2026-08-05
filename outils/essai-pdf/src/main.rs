use printpdf::{Base64OrRaw, GeneratePdfOptions, PdfDocument, PdfSaveOptions};
use std::collections::BTreeMap;
use std::time::Instant;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let source = arguments.next().expect("usage : essai-pdf entree.html sortie.pdf");
    let sortie = arguments.next().unwrap_or_else(|| "essai.pdf".into());
    let html = std::fs::read_to_string(&source).expect("lecture");

    let options = GeneratePdfOptions {
        page_width: Some(210.0),
        page_height: Some(297.0),
        margin_top: Some(0.0),
        margin_right: Some(0.0),
        margin_bottom: Some(0.0),
        margin_left: Some(0.0),
        ..Default::default()
    };
    let images: BTreeMap<String, Base64OrRaw> = BTreeMap::new();
    let polices: BTreeMap<String, Base64OrRaw> = BTreeMap::new();
    let mut alertes = Vec::new();

    let depart = Instant::now();
    let document = match PdfDocument::from_html(&html, &images, &polices, &options, &mut alertes) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("échec de la mise en page : {}", e);
            std::process::exit(1);
        }
    };
    let octets = document.save(&PdfSaveOptions::default(), &mut alertes);
    let duree = depart.elapsed();

    std::fs::write(&sortie, &octets).expect("écriture");
    println!(
        "{} → {} : {} octets en {:.0} ms",
        source,
        sortie,
        octets.len(),
        duree.as_secs_f64() * 1000.0
    );
    if alertes.is_empty() {
        println!("aucune alerte");
    } else {
        println!("{} alertes :", alertes.len());
        for a in alertes.iter().take(25) {
            println!("  {:?}", a);
        }
    }
}
