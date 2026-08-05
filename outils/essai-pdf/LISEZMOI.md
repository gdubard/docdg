# essai-pdf

Épreuve de `printpdf::from_html` sur une page produite par docdg, pour juger si
une génération de PDF entièrement en Rust tient la qualité.

Cet outil est délibérément hors de l'espace de travail : il n'est compilé que si
on le demande, et n'ajoute aucune dépendance au projet.

## Procédure

    cargo run -q -p docdg-transpiler --example preview -- epreuve.html mon-document.txt
    cd outils/essai-pdf && cargo run --release -- ../../epreuve.html epreuve.pdf

Puis comparer `epreuve.pdf` au PDF produit par le chemin Chromium sur le même
document. Ce qu'il faut regarder, dans cet ordre :

1. les formules — KaTeX positionne ses glyphes au moyen de marges négatives et
   d'alignements verticaux en em ; c'est le point qui cassera en premier ;
2. les tableaux, en particulier les fusions déduites de la forme ;
3. les cadres colorés, les coins arrondis et les bordures ;
4. les figures SVG et les images en base64 ;
5. la pagination — printpdf repagine lui-même, il ne reprend pas le découpage
   déjà fait par l'aperçu.

Si le rendu tient sur ces cinq points, la voie est bonne. Sinon, le moteur de
mise en page sous-jacent (azul-layout, en version 0.0.13) n'est pas assez mûr et
il faut rester sur Chromium.
