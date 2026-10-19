# docdg 3.9.0 — tronc unique et recette de publication

## Tronc unique

Cette arborescence est **la seule base de la 3.9.0**. Les branches de travail
antérieures — `doccl1`, `docch1`, la consolidation `docch2` — sont closes.

Tout apport ultérieur venu d'une autre branche s'importe **fichier par
fichier, correctif par correctif**, jamais par fusion globale d'arborescence :
une fusion d'ensemble réintroduit silencieusement des régressions déjà
corrigées ici (troncatures par `%`, cycles du corpus, messages de saisie
anonymes, espace fine des formules). Chaque import se justifie dans le
CHANGELOG.

## Modèle de titres — figé

La numérotation décimale hiérarchique est automatique. `num`, `roman`,
`Roman`, `alpha`, `Alpha` choisissent la forme ; `sans numéro` la supprime, en
produisant une forme absente (`None`) qui ne consomme aucun compteur. Les
contradictions sont refusées, `Num` et `NUM` n'existent pas. Ce modèle est
arrêté : il ne faut pas réintroduire de variante d'énumération pour
représenter l'absence de numéro. Voir `CONVENTION_TITRES_3_9.md`.

## Contrôles passés hors Cargo

```sh
python3 tests/controle_reference.py
python3 tests/consolidation3_9.py
python3 tests/regression_v4.py
python3 tests/qualification_multiplateforme.py
python3 tests/distribution.py
python3 outils/qualifier_hybride.py --navigateur
python3 outils/valider_synchronisation_scroll.py  # Playwright + Chromium
```

## Qualification native — à faire sur macOS, Windows 11 et Fedora

```sh
cargo test --workspace --locked
cargo test -p docdg-transpiler --test numerotation
cargo test -p docdg-transpiler --test notation
cargo test -p docdg-transpiler --test boucles_typage
cargo test -p docdg-transpiler --test exemples -- --nocapture
cargo test -p docdg-transpiler --test diagnostic_alertes_publics -- --nocapture
cargo app lint
cargo build --release --locked -p docdg
```

Le diagnostic exécuté le 17 octobre sur macOS montre **zéro alerte** dans
`algo.txt` et **une alerte volontaire** dans `vitrine.txt` : la relation
`E = m * c`, choisie pour démontrer la détection d’une formule non homogène.
Ces résultats doivent rester stables dans les trois environnements.
Les trois sauts de page de la vitrine (« grammaire vectorielle », « sciences
de l’ingénieur », « algèbre »), retirés lors d’un correctif antérieur, ont
été **rétablis explicitement par l’auteur** dans ses sources finales. Ils sont
donc conservés dans les exemples livrés et les tests sont mis à jour pour
correspondre à cette pagination choisie. Les 23 PDF fournis à titre de
référence visuelle sont disponibles dans `references-pdf/` (archive source).
La suppression d’un saut explicite dans l’éditeur déclenche une repagination
complète sans reprise de pages caduques ; recette Chromium ajoutée.

## Validation visuelle — PDF et PNG

À l'écran puis à l'export, sur chaque système, en vérifiant que le PDF est
identique au rendu affiché :

- `basique.txt` — les six saisies, puis la modification d'une réponse
  antérieure
- `algo.txt` — boucles, matrices, conteneurs
- `vitrine.txt` — la page de démonstration des fonctionnalités
- `mathlang.txt` — notations du supérieur, espace fine des formules
- `statistiques-probabilites.txt` — graphiques et tableaux

Vérifier en outre un document portant des filets, des fonds, des pages
d'objets seuls et des sauts explicites : aucune page ne doit disparaître,
aucun titre ne doit rester seul en bas de page.

## Empaquetage

```sh
python3 outils/creer_distribution.py
```

Un nom dupliqué dans l'archive fait échouer la fabrication en nommant le
chemin fautif. L'avertissement `Duplicate name` que produisait la
qualification venait d'une fixture de test qui fabrique volontairement un
doublon pour vérifier qu'il est refusé ; il est désormais tu sur place et ne
doit plus être pris pour un défaut d'empaquetage.

Ne pas présenter l'archive comme une version publiée avant la qualification
native sur les trois systèmes.

## Vérification Source / Aperçu (avant publication)

- Faire défiler **lentement puis rapidement** la Source avec le pavé tactile
  ou la molette, puis reprendre le pilotage dans l'Aperçu : aucun bloc ne doit
  sauter ni rattraper son retard après l'arrêt du geste.
- Vérifier `basique.txt`, `mathlang.txt` et `vitrine.txt` notamment aux
  **changements de page**, autour des **grands tableaux, formules et dessins**.
- Vérifier les **première et dernière positions**, les deux dispositions de
  volets, ainsi que **zoom** et **redimensionnement** en cours de session.
- La source ne peut pas représenter pixel pour pixel un objet très haut
  généré par une seule instruction ; l'alignement est **sémantique**, et non
  une égalité des pourcentages de défilement.
