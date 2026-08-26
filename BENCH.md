# Banc d'essai

Chaque version consigne ici les temps de rendu de référence, mesurés par le
crate `bench/` sur les deux documents les plus exigeants du dépôt :
`vitrine4.txt` (la démonstration de force — chaque ligne source produit une
rédaction complète) et `algo4.txt` (le langage algorithmique au complet).

Le protocole, à lancer sur une machine au repos :

```bash
cargo run --release -p docdg-bench --bin froid_chaud -- exemples/vitrine4.txt 50
cargo run --release -p docdg-bench --bin froid_chaud -- exemples/algo4.txt 50
```

`froid_chaud` mesure en médianes et accepte un troisième argument, le plafond
d'`incr` en millisecondes, au-delà duquel il sort en erreur — c'est lui que la
CI exécute. Le binaire par défaut `docdg-bench` donne une mesure rapide en
moyennes, sans plafond.

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
| 2.7.0 → 3.1.0 | *(non relevé)* | | | | | | |
| 3.2.0 | MacBook Air M5⁵ | vitrine4.txt | 1,54 | 0,96 | 0,232 | 0,187 | 12 |
| 3.2.0 | MacBook Air M5⁵ | algo4.txt | 8,44 | 8,83 | 0,712 | 0,695 | 18 |
| 3.2.0 | MacBook Air M5⁵ | demonstration4.txt³ | 4,16 | 12,81 | 0,298 | 0,295 | 68 |
| 3.2.0 | MacBook Air M5⁵ | couleurs1.txt⁴ | 1,22 | 1,80 | 0,387 | 0,314 | 40 |

**Six versions manquent à cette table.** Le protocole exige un relevé par
version ; de la 2.7 à la 3.1, aucun n'a été consigné. Les lignes vides
au-dessus le disent plutôt que de le taire : une table qui saute six versions
ne mesure plus de non-régression, elle enregistre un souvenir.

³ `demonstration4.txt` appelle depuis la 3.2 quatre-vingt-dix des cent une
fiches de la bibliothèque, contre 48 auparavant : son temps n'est pas
comparable aux relevés antérieurs, et la ligne repart de zéro.

⁵ Premier relevé de publication sur machine de développement, cargo 1.96.0,
IDE fermé. La comparaison à la 2.6.0 n'est qu'indicative — les deux machines
diffèrent —, mais le rapport ENTRE documents, lui, se compare : l'`incr`
d'`algo4.txt` valait 8,6 fois celui de `vitrine4.txt` en 2.6.0 (2,358/0,275),
contre 3,7 fois ici (0,695/0,187). C'est le résultat attendu de la garde
posée sur `subst_var` cette version : elle ne prétendait pas égaler les deux
documents — `algo4.txt` reste plus long et plus bouclé —, seulement retirer
le gaspillage qui s'ajoutait à cet écart sans raison.

Sur `algo4.txt`, `froid_seq` est mesuré plus lent que `froid` (8,83 contre
8,44 ms) — l'inverse de l'écart attendu sur une machine à plusieurs cœurs,
et l'inverse de ce que dit la note ¹ pour un conteneur à un seul cœur. L'écart
est faible (4 %) et n'a pas été confirmé sur un second relevé indépendant :
à surveiller à la prochaine version plutôt qu'à investiguer ici.

## Ce que la 3.2 change dans la mesure

Quatre réglages touchent directement les chiffres de cette table. Le relevé
3.2 les mesure ensemble ; les isoler demanderait quatre relevés, ce que le
protocole n'exige pas.

1. **L'environnement n'est rehaché et recopié que s'il a changé.** `inerte`
   laissait passer tout segment portant une balise — un titre, une ligne de
   style —, et chacun déclenchait un parcours complet de l'environnement pour
   le hachage, puis une copie complète pour l'instantané. La comparaison qui
   les remplace s'arrête au premier écart et n'alloue rien. C'est le poste
   attendu comme le plus rentable sur `algo4.txt`, dont l'environnement est le
   plus lourd du dépôt — et dont l'`incr` était huit fois celui de `vitrine4`.
2. **Les `Arc` des conteneurs et des fonctions ne se refont que sur
   changement.** Ils étaient reconstruits à chaque segment, ce qui annulait
   leur partage.
3. **Le seuil de parallélisme compte le travail, en octets de source, et non
   les segments.** Deux segments d'une ligne ne valent pas l'ordonnancement
   d'un pool : `froid` était plus lent que `froid_seq` sur une machine à un
   cœur, et le seuil en était la cause.
4. **L'attente d'un ouvrier Python sort du verrou.** Le `recv_timeout` se
   faisait verrou tenu : les fils de rayon se bloquaient par tranches de deux
   cents millisecondes, et le parallélisme s'annulait là où SymPy coûte le
   plus cher. Attendu surtout sur `vitrine4.txt` à mémo froid.

⁴ `couleurs1.txt` compose les 148 teintes en nuancier — c'est le document le
plus dense en balises de style du dépôt, et donc la meilleure mesure du coût
de `lit_style`.

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
