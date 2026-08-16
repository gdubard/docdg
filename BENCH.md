# Banc d'essai

Chaque version consigne ici les temps de rendu de référence, mesurés par le
crate `bench/` sur les deux documents les plus exigeants du dépôt :
`vitrine4.txt` (la démonstration de force — chaque ligne source produit une
rédaction complète) et `algo4.txt` (le langage algorithmique au complet).

Le protocole, à lancer sur une machine au repos :

```bash
cargo run --release -p docdg-bench -- exemples/vitrine4.txt 50
cargo run --release -p docdg-bench -- exemples/algo4.txt 50
```

Quatre temps sortent par document :

- **froid** — premier rendu, cache vide, en parallèle ;
- **froid_seq** — même chose sans rayon, pour mesurer ce que le parallélisme
  rapporte réellement ;
- **chaud** — rendu sur cache plein : c'est le coût d'assemblage pur ;
- **incr** — un paragraphe ajouté puis retiré : c'est le temps que ressent
  l'auteur à chaque frappe, le chiffre qui compte.

Un chiffre qui se dégrade d'une version à l'autre sans raison assumée est un
bug de performance : la CI rejoue le banc à chaque poussée pour que la
dégradation se voie au commit fautif, pas à la publication.

## Relevés

| Version | Machine | Document | froid (ms) | froid_seq (ms) | chaud (ms) | incr (ms) | html (Ko) |
|---|---|---|---:|---:|---:|---:|---:|
| 2.6.0 | conteneur d'audit, 1 vCPU¹ | vitrine4.txt | 1,43 | 1,33 | 0,283 | 0,275 | 12 |
| 2.6.0 | conteneur d'audit, 1 vCPU¹ | algo4.txt | 20,13 | 19,93 | 2,337 | 2,358 | 17 |
| 2.6.0 | conteneur d'audit, 1 vCPU¹ | demonstration4.txt | 1,85 | 1,84 | 0,179 | 0,165 | 8 |
| 2.6.0 | conteneur d'audit, 1 vCPU¹ | seyes1.txt² | 0,40 | 0,38 | 0,240 | 0,244 | 133 |
| 2.6.0 | *(à relever sur machine de développement)* | vitrine4.txt | | | | | |
| 2.6.0 | *(à relever sur machine de développement)* | algo4.txt | | | | | |

Les relevés antérieurs à l'implémentation du protocole — le banc ne mesurait
alors qu'un rendu unique, bassin de calcul froid compris — ne se comparent pas
à ceux-ci ; la table repart de la première mesure conforme.

¹ Sur un seul cœur, le rendu parallèle paie l'ordonnancement de rayon sans
rien en tirer, et les rendus froids profitent de l'archive de calcul formel
déjà constituée : « froid » y mesure le transpileur, non SymPy. Les chiffres
qui comptent — chaud et incr, le ressenti de la frappe — restent
significatifs. Un relevé sur une machine de développement multicœur fait
référence.

² L'écriture sur des lignes est le bloc le plus léger du dépôt à composer —
la réglure est un motif déclaré une fois, que chaque bloc référence — mais le
plus lourd à transporter : les 133 Ko sont pour l'essentiel la Marelle jointe
au document, une fois, et non le balisage (~12 Ko, réglure comprise).
