use crate::{Env, TocEntry};

const CURSIVE_HAUT: f32 = 0.903;

const MONTANTE: f32 = 0.75;

const JAMBAGE: f32 = 2.0 / 3.0;

const TRAIT_FORT: f32 = 0.25;
const TRAIT_FIN: f32 = 0.18;
const TRAIT_MARGE: f32 = 0.5;

const COULEUR_FORTE: &str = "royalblue";
const COULEUR_FINE: &str = "deepskyblue";
const COULEUR_MARGE: &str = "red";

const DECALAGE_MM: f32 = 1.0;

const DEBORD_MM: f32 = 300.0;

const PX_PAR_MM: f32 = 96.0 / 25.4;

pub(crate) const CLASSE: &str = "docdg-lignes";

const MARELLE: &str = include_str!("marelle.woff2.b64");

const MARELLE_HAUT: f32 = 1.44;

const CURSIVE_DEFAUT: &str = "Marelle";

fn fonte(c: &Cursive) -> String {

    format!(
        "@font-face{{font-family:'{nom}';src:{source};\
ascent-override:{haut:.1}%;descent-override:{bas:.1}%;line-gap-override:0%;\
font-display:block}}",
        nom = c.nom,
        source = c.source,
        haut = c.hampe * 100.0,
        bas = c.hampe * JAMBAGE * 100.0,
    )
}

pub fn feuille_de_style() -> String {
    let fonte = avec_cursive(fonte);
    let g = geometrie();

    let coiffe = g.pas / 8.0 + TRAIT_FIN / 2.0;

    let lignes = lignes_tracees(&g);

    let pas_px = (g.pas * PX_PAR_MM).round();
    format!(
        "<style>{fonte}\
docdg-lignes{{display:block;position:relative;z-index:0;\
padding:{coiffe}mm 0 {coiffe}mm {dec}mm;line-height:{pas}mm}}\
docdg-reglure{{display:block;position:absolute;top:0;right:0;bottom:0;left:0;\
overflow:hidden;z-index:-1}}\
docdg-reglure svg{{display:block;height:{haut}mm}}\
docdg-lignes span,docdg-lignes sup,docdg-lignes sub{{line-height:0}}\
docdg-lignes span[style*=\"display:block\"]{{line-height:{pas}mm}}\
@media screen{{\
docdg-lignes,docdg-lignes span[style*=\"display:block\"]{{line-height:{pas_px}px}}\
docdg-reglure svg{{height:{haut_px}px}}}}</style>{symbole}",
        fonte = fonte,
        coiffe = mm(coiffe),
        dec = mm(DECALAGE_MM),
        pas = mm(g.pas),
        haut = mm(lignes as f32 * g.pas),
        pas_px = pas_px,
        haut_px = lignes as f32 * pas_px,
        symbole = symbole(&g),
    )
}

fn symbole(g: &Reglure) -> String {
    let hauteur = lignes_tracees(g) as f32 * g.pas;
    format!(
        "<div class=\"docdg-motif\"><svg xmlns=\"http://www.w3.org/2000/svg\" \
width=\"0\" height=\"0\" style=\"position:absolute\" aria-hidden=\"true\">\
<symbol id=\"docdg-reglure-motif\" viewBox=\"0 0 {l} {h}\" \
preserveAspectRatio=\"none\">{motif}</symbol></svg></div>",
        l = mm(DEBORD_MM),
        h = mm(hauteur),
        motif = motif(g),
    )
}

fn est_sur_des_lignes(desc: &str) -> bool {
    let plie: String = desc
        .trim()
        .trim_end_matches(['.', ':'])
        .chars()
        .map(crate::utils::texte::pli)
        .collect::<String>()
        .to_lowercase();
    plie.split_whitespace().collect::<Vec<_>>() == ["sur", "des", "lignes"]
}

fn carreaux_de_tete(ligne: &str) -> (usize, &str) {
    let reste = ligne.trim_start_matches('\t');
    (ligne.len() - reste.len(), reste)
}

struct Reglure {

    pas: f32,

    carreau: f32,

    corps: f32,

    base: f32,
}

fn geometrie() -> Reglure {
    let r = crate::layout::rendu::reglages_page();
    let pas = r.hauteur_cm * 10.0;

    let corps = MONTANTE * pas / hampe();
    Reglure {
        pas,
        carreau: r.tabulation_cm * 10.0,
        corps,

        base: pas / 2.0 + corps * hampe() * (1.0 - JAMBAGE) / 2.0,
    }
}

fn mm(v: f32) -> String {
    format!("{:.3}", v)
}

fn rectangle(x: f32, y: f32, l: f32, h: f32) -> String {
    format!(
        "M{} {}h{}v{}h-{}z",
        mm(x),
        mm(y),
        mm(l),
        mm(h),
        mm(l)
    )
}

fn motif(g: &Reglure) -> String {

    let premiere = g.pas / 8.0 + TRAIT_FIN / 2.0 + g.base;
    let hauteur = lignes_tracees(g) as f32 * g.pas;

    let mut fine = String::new();
    for j in -3i32.. {
        let y = premiere + j as f32 * g.pas / 4.0;
        if y >= hauteur {
            break;
        }
        if j.rem_euclid(4) != 0 {
            fine.push_str(&rectangle(0.0, y - TRAIT_FIN / 2.0, DEBORD_MM, TRAIT_FIN));
        }
    }

    let mut forte = String::new();
    for k in 0.. {
        let y = premiere + k as f32 * g.pas;
        if y >= hauteur {
            break;
        }
        forte.push_str(&rectangle(0.0, y - TRAIT_FORT / 2.0, DEBORD_MM, TRAIT_FORT));
    }
    for k in 0.. {
        let x = k as f32 * g.carreau;
        if x >= DEBORD_MM {
            break;
        }
        forte.push_str(&rectangle(x, 0.0, TRAIT_FORT, hauteur));
    }

    format!(
        "<path d=\"{fine}\" fill=\"{cfine}\"/>\
<path d=\"{forte}\" fill=\"{cforte}\"/>\
<path d=\"{rouge}\" fill=\"{cmarge}\"/>",
        fine = fine,
        forte = forte,
        rouge = rectangle(0.0, 0.0, TRAIT_MARGE, hauteur),
        cfine = COULEUR_FINE,
        cforte = COULEUR_FORTE,
        cmarge = COULEUR_MARGE,
    )
}

fn reglure(g: &Reglure) -> String {
    let hauteur = lignes_tracees(g) as f32 * g.pas;
    format!(
        "<docdg-reglure><svg xmlns=\"http://www.w3.org/2000/svg\" \
width=\"{l}mm\" height=\"{h}mm\" viewBox=\"0 0 {l} {h}\" \
preserveAspectRatio=\"none\" aria-hidden=\"true\">\
<use href=\"#docdg-reglure-motif\"/></svg></docdg-reglure>",
        l = mm(DEBORD_MM),
        h = mm(hauteur),
    )
}

fn lignes_tracees(g: &Reglure) -> usize {
    (DEBORD_MM / g.pas).ceil() as usize + 1
}

pub(crate) fn ecris(
    desc: &str,
    corps_source: &str,
    env: &mut Env,
    toc: &mut Vec<TocEntry>,
) -> Option<String> {
    if !est_sur_des_lignes(desc) {
        return None;
    }

    let g = geometrie();

    let corps_source = corps_source.strip_prefix('\n').unwrap_or(corps_source);
    let corps_source = corps_source.strip_suffix('\n').unwrap_or(corps_source);

    let mut lignes = String::new();
    for ligne in corps_source.split('\n') {
        let ligne = ligne.strip_suffix('\r').unwrap_or(ligne);
        let (carreaux, texte) = carreaux_de_tete(ligne);

        let retrait = if carreaux > 0 {
            format!(" style=\"text-indent:{}mm\"", mm(carreaux as f32 * g.carreau))
        } else {
            String::new()
        };
        let contenu = if texte.trim().is_empty() {

            "\u{200B}".to_string()
        } else {

            let texte = texte.replace('$', "\u{E005}");

            let texte = texte.replace('\u{2212}', "-");
            crate::layout::rendu::render_inline(&texte, env, toc)
        };
        lignes.push_str(&format!("<div{}>{}</div>", retrait, contenu));
    }

    Some(format!(
        "<docdg-lignes class=\"{}\" style=\"\
font-family:'{}',cursive;font-size:{}mm;\
hyphens:none;-webkit-hyphens:none;\
\">{}{}</docdg-lignes>",
        CLASSE,
        avec_cursive(|c| c.nom.clone()),
        mm(g.corps),
        reglure(&g),
        lignes,
    ))
}

struct Cursive {

    nom: String,

    source: String,

    hampe: f32,
}

fn tables(data: &[u8]) -> Vec<([u8; 4], usize)> {
    let mot = |i: usize| -> u16 {
        u16::from_be_bytes([
            data.get(i).copied().unwrap_or(0),
            data.get(i + 1).copied().unwrap_or(0),
        ])
    };
    let long = |i: usize| -> u32 {
        u32::from_be_bytes([
            data.get(i).copied().unwrap_or(0),
            data.get(i + 1).copied().unwrap_or(0),
            data.get(i + 2).copied().unwrap_or(0),
            data.get(i + 3).copied().unwrap_or(0),
        ])
    };
    (0..mot(4) as usize)
        .filter_map(|k| {
            let e = 12 + k * 16;
            let tag: [u8; 4] = data.get(e..e + 4)?.try_into().ok()?;
            Some((tag, long(e + 8) as usize))
        })
        .collect()
}

fn normalise_metriques(data: &mut [u8]) -> Option<()> {
    let t = tables(data);
    let cherche = |nom: &[u8; 4]| t.iter().find(|(g, _)| g == nom).map(|(_, o)| *o);
    let hhea = cherche(b"hhea")?;
    let os2 = cherche(b"OS/2")?;
    if data.len() < os2 + 78 || data.len() < hhea + 10 {
        return None;
    }
    let asc = i16::from_be_bytes([data[hhea + 4], data[hhea + 5]]);
    if asc <= 0 {
        return None;
    }
    let desc = -((asc as f32 * JAMBAGE).round() as i16);

    let drapeaux = u16::from_be_bytes([data[os2 + 62], data[os2 + 63]]);
    fn pose(data: &mut [u8], offset: usize, valeur: i16) {
        data[offset..offset + 2].copy_from_slice(&valeur.to_be_bytes());
    }

    pose(data, hhea + 4, asc);
    pose(data, hhea + 6, desc);
    pose(data, hhea + 8, 0);

    pose(data, os2 + 68, asc);
    pose(data, os2 + 70, desc);
    pose(data, os2 + 72, 0);
    pose(data, os2 + 74, asc);
    pose(data, os2 + 76, -desc);

    pose(data, os2 + 62, (drapeaux | (1 << 7)) as i16);
    Some(())
}

fn hampe_de_fonte(data: &[u8]) -> Option<f32> {
    let u16a = |i: usize| -> Option<u16> {
        Some(u16::from_be_bytes([*data.get(i)?, *data.get(i + 1)?]))
    };
    let u32a = |i: usize| -> Option<u32> {
        Some(u32::from_be_bytes([
            *data.get(i)?,
            *data.get(i + 1)?,
            *data.get(i + 2)?,
            *data.get(i + 3)?,
        ]))
    };
    let tables = u16a(4)? as usize;
    let mut head = None;
    let mut hhea = None;
    for k in 0..tables {
        let e = 12 + k * 16;
        let tag = data.get(e..e + 4)?;
        let off = u32a(e + 8)? as usize;
        match tag {
            b"head" => head = Some(off),
            b"hhea" => hhea = Some(off),
            _ => {}
        }
    }
    let upm = u16a(head? + 18)? as f32;
    let asc = i16::from_be_bytes([*data.get(hhea? + 4)?, *data.get(hhea? + 5)?]) as f32;
    if upm <= 0.0 || asc <= 0.0 {
        return None;
    }
    Some(asc / upm)
}

fn sur_le_systeme(nom: &str) -> String {
    format!(
        "local('{n}'),local('{n} Regular'),local('{n}-Regular')",
        n = nom
    )
}

fn cursive(police: &str) -> Cursive {
    let ecrit = police.trim().replace('\'', "");
    let ecrit = if ecrit.is_empty() {
        CURSIVE_DEFAUT.to_string()
    } else {
        ecrit
    };
    let plie = ecrit.to_lowercase();
    if plie == "marelle" {
        return Cursive {
            nom: ecrit,
            source: format!("url(data:font/woff2;base64,{}) format('woff2')", MARELLE),
            hampe: MARELLE_HAUT,
        };
    }
    let fichier = ecrit.rsplit('.').next().map(|e| e.to_lowercase());
    let est_fonte = matches!(fichier.as_deref(), Some("ttf" | "otf" | "woff2" | "woff"));

    let nom = if est_fonte {
        std::path::Path::new(&ecrit)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| ecrit.clone())
    } else {
        ecrit.clone()
    };
    if est_fonte && crate::layout::rendu::chemin_sur(&ecrit) {
        if let Some(base) = crate::layout::rendu::base_dir() {
            if let Ok(mut data) = std::fs::read(base.join(&ecrit)) {

                if matches!(fichier.as_deref(), Some("ttf" | "otf")) {
                    normalise_metriques(&mut data);
                }

                let (mime, format) = match fichier.as_deref() {
                    Some("woff2") => ("font/woff2", "woff2"),
                    Some("woff") => ("font/woff", "woff"),
                    Some("otf") => ("font/otf", "opentype"),
                    _ => ("font/ttf", "truetype"),
                };
                let hampe = hampe_de_fonte(&data).unwrap_or(CURSIVE_HAUT);
                return Cursive {
                    nom,
                    source: format!(
                        "url(data:{};base64,{}) format('{}')",
                        mime,
                        crate::layout::rendu::base64(&data),
                        format
                    ),
                    hampe,
                };
            }
        }
    }
    Cursive {
        source: sur_le_systeme(&nom),
        nom,
        hampe: CURSIVE_HAUT,
    }
}

fn avec_cursive<T>(lit: impl FnOnce(&Cursive) -> T) -> T {
    thread_local! {
        static MEMO: std::cell::RefCell<Option<(String, Cursive)>> =
            const { std::cell::RefCell::new(None) };
    }
    let police = crate::layout::rendu::police_seyes();
    MEMO.with(|c| {
        let mut c = c.borrow_mut();
        let connue = matches!(c.as_ref(), Some((nom, _)) if *nom == police);
        if !connue {
            *c = Some((police.clone(), cursive(&police)));
        }
        lit(&c.as_ref().unwrap().1)
    })
}

fn hampe() -> f32 {
    avec_cursive(|c| c.hampe)
}
