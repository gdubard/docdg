use docdg_transpiler::Engine;

fn rend(src: &str) -> String {
    let mut e = Engine::new();
    e.render(src, false).html
}

#[test]
fn une_combustion_s_equilibre() {
    let html = rend("<Équilibre>l'équation CH4 + O2 -> CO2 + H2O");
    assert!(html.contains("\\mathrm{CH}_{4} + 2\\,\\mathrm{O}_{2}"));
    assert!(html.contains("\\mathrm{CO}_{2} + 2\\,\\mathrm{H}_{2}\\mathrm{O}"));
    assert!(!html.contains("calcul-absent"));
}

#[test]
fn une_demi_equation_redox_conserve_la_charge() {
    let html = rend("<Équilibre>l'équation Fe^2+ + MnO4^- + H^+ -> Fe^3+ + Mn^2+ + H2O");
    assert!(html.contains("5\\,\\mathrm{Fe}^{2+}"));
    assert!(html.contains("8\\,\\mathrm{H}^{+}"));
    assert!(html.contains("4\\,\\mathrm{H}_{2}\\mathrm{O}"));
}

#[test]
fn la_fleche_unicode_est_acceptee() {
    let a = rend("<Équilibre>l'équation H2 + O2 -> H2O");
    let b = rend("<Équilibre>l'équation H2 + O2 → H2O");
    assert_eq!(a, b);
    assert!(a.contains("2\\,\\mathrm{H}_{2} + \\mathrm{O}_{2}"));
}

#[test]
fn une_equation_impossible_est_refusee() {
    let html = rend("<Équilibre>l'équation H2 + O2 -> Fe");
    assert!(html.contains("calcul-absent"));
}

#[test]
fn la_masse_molaire_se_detaille() {
    let html = rend("<Calcule>la masse molaire de Fe2(SO4)3");
    assert!(html.contains("2\\,M(\\mathrm{Fe})"));
    assert!(html.contains("12\\,M(\\mathrm{O})"));
    assert!(html.contains("3\\,M(\\mathrm{S})"));
    assert!(html.contains("399{,}9\\ \\mathrm{g\\,mol^{-1}}"));
}

#[test]
fn la_masse_molaire_de_l_eau() {
    let html = rend("<Calcule>la masse molaire de H2O");
    assert!(html.contains("18\\ \\mathrm{g\\,mol^{-1}}"));
}

#[test]
fn le_tableau_d_avancement_trouve_le_reactif_limitant() {
    let html = rend(
        "<Dresse>un tableau d'avancement pour CH4 + O2 -> CO2 + H2O avec n(CH4) = 0,5 et n(O2) = 0,8",
    );
    assert!(html.contains("État initial"));
    assert!(html.contains("En cours"));
    assert!(html.contains("État final"));
    assert!(html.contains("0{,}5 - x"));
    assert!(html.contains("0{,}8 - 2x"));
    assert!(html.contains("2x"));
    assert!(html.contains("réactif limitant"));
    assert!(html.contains("\\mathrm{O}_{2}"));
    assert!(html.contains("x_{\\max}"));
    assert!(html.contains("0{,}4"));
}

#[test]
fn le_tableau_d_avancement_sans_quantites_reste_litteral() {
    let html = rend("<Dresse>un tableau d'avancement pour N2 + H2 -> NH3");
    assert!(html.contains("n_0 - x"));
    assert!(html.contains("n_0 - 3x"));
    assert!(html.contains("2x"));
    assert!(!html.contains("réactif limitant"));
}

#[test]
fn les_vitesses_se_convertissent() {
    let html = rend("<Convertis>340 m/s en km/h");
    assert!(html.contains("340\\ \\mathrm{m\\,s^{-1}} = 1224\\ \\mathrm{km\\,h^{-1}}"));
}

#[test]
fn les_temperatures_se_convertissent_dans_les_deux_sens() {
    let a = rend("<Convertis>25 °C en K");
    assert!(a.contains("298{,}15"));
    let b = rend("<Convertis>310 K en °C");
    assert!(b.contains("36{,}85"));
}

#[test]
fn l_electronvolt_vaut_sa_definition() {
    let html = rend("<Convertis>1 eV en J");
    assert!(html.contains("1{,}602176634 \\times 10^{-19}"));
}

#[test]
fn les_conversions_de_l_ecole_primaire_survivent() {
    let html = rend("<Convertis>3 km en m");
    assert!(html.contains("3000"));
    assert!(!html.contains("calcul-absent"));
}

#[test]
fn les_constantes_fondamentales_s_affichent() {
    let h = rend("<Donne>la valeur de la constante de Planck");
    assert!(h.contains("6{,}62607015 \\times 10^{-34}"));
    assert!(h.contains("\\mathrm{J\\,s}"));
    let na = rend("<Donne>la valeur du nombre d'Avogadro");
    assert!(na.contains("6{,}02214076 \\times 10^{23}"));
    let c = rend("<Donne>la valeur de la vitesse de la lumière");
    assert!(c.contains("2{,}99792458 \\times 10^{8}"));
}

#[test]
fn le_gradient_le_laplacien_et_les_champs_se_calculent() {
    let g = rend("<Calcule>le gradient de f(x, y, z) = x^2yz + sin(x)");
    assert!(g.contains("\\overrightarrow{\\nabla} f"));
    assert!(g.contains("2 x y z + \\cos{\\left(x \\right)}"));
    let d = rend("<Calcule>la divergence du champ F(x, y, z) = (x^2 ; xy ; z)");
    assert!(d.contains("\\operatorname{div}\\vec{F} = 3 x + 1"));
    let r = rend("<Calcule>le rotationnel du champ F(x, y, z) = (y ; -x ; 0)");
    assert!(r.contains("\\overrightarrow{\\mathrm{rot}}"));
    assert!(r.contains("-2"));
    let l = rend("<Calcule>le laplacien de f(x, y) = x^2 + y^2");
    assert!(l.contains("\\Delta f = 4"));
}

#[test]
fn l_incertitude_se_propage_avec_application_numerique() {
    let html = rend(
        "<Propage>l'incertitude sur R = U/I avec u(U) = 0,05, u(I) = 0,002, U = 5,12, I = 0,254",
    );
    assert!(html.contains("\\dfrac{\\partial R}{\\partial U}"));
    assert!(html.contains("\\dfrac{\\partial R}{\\partial I}"));
    assert!(html.contains("u(I)^{2}"));
    assert!(html.contains("20{,}16"));
    assert!(html.contains("0{,}25"));
}

#[test]
fn l_incertitude_sans_valeurs_reste_symbolique() {
    let html = rend("<Propage>l'incertitude sur g = 4pi^2 L/T^2 avec u(L) = 0,001, u(T) = 0,01");
    assert!(html.contains("\\dfrac{\\partial g}{\\partial L}"));
    assert!(html.contains("\\dfrac{\\partial g}{\\partial T}"));
    assert!(!html.contains("\\quad ; \\quad u(g)"));
}

#[test]
fn le_parallele_egale_le_sequentiel_en_physique_chimie() {
    let src = "<Équilibre>l'équation C3H8 + O2 -> CO2 + H2O\n\n<Calcule>la masse molaire de CaCO3\n\n<Convertis>90 km/h en m/s\n\n<Donne>la valeur de la constante de Boltzmann";
    let mut a = Engine::new();
    let mut b = Engine::new();
    assert_eq!(a.render(src, true).html, b.render(src, false).html);
}
