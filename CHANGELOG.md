# Journal des versions

Ce projet suit un versionnage simple : le premier chiffre marque un changement
de nature (ce qu'on peut faire avec docdg), le second une extension dans le
même esprit.

## 2.0 — le document devient vivant

Changement de nature plutôt que d'ampleur : docdg peut désormais interroger
son lecteur et se recomposer avec ses réponses, ce qu'aucun document figé
(LaTeX, Typst, PDF statique) ne permet.

### Ajouté

**Les saisies interactives.**
- La balise `<Saisis>` : cinq types (texte, entier, décimal à la virgule,
  booléen `vrai`/`faux`, caractère).
- Blocage typé : le document s'arrête à la première réponse manquante ou
  invalide ; rien de ce qui suit — texte, questions, calculs — ne s'affiche
  tant qu'elle n'est pas conforme. Message d'erreur temporaire sous le champ.
  Le champ de saisie prend automatiquement le focus.
- Cliquer sur une réponse déjà donnée (affichée en bleu) rouvre la question
  et redéroule le document à partir de là.
- Les réponses sont des valeurs comme les autres : elles s'affichent par
  `#nom`, entrent dans les calculs `#{...}`, et peuvent alimenter n'importe
  quelle description de balise (`<Trace>le solide cube, d'arête #a cm`).

**L'opérateur ternaire.**
- En bloc : `si <condition> { ... } sinon { ... }`.
- En ligne, comme valeur : `soit x = si <condition> { A } sinon { B }`,
  texte ou nombre.
- Condition nue (`si marié`, vraie si non nulle), `vaut vrai` / `vaut faux`,
  et tous les comparateurs existants (`vaut`, `moins de`, `plus de`,
  `au moins`, `au plus`, `différent de`).

**La géométrie dans l'espace**, portée par un nouveau moteur de projection
cavalière (fuyante 0,45/0,35) :
- sept solides en perspective cavalière, arêtes cachées en pointillés :
  cube, pavé droit, prisme, pyramide, cylindre, cône, sphère ;
- cinq patrons dépliés (cube, pavé, pyramide, cylindre, cône) ;
- le repère de l'espace (O ; x, y, z), gradué, avec points, segments,
  vecteurs et droites tracés, chemin de coordonnées en pointillés rouges ;
- les droites de l'espace comme objets (`<Soit>la droite d passant par ...
  et de vecteur directeur ...`), leur représentation paramétrique rédigée ;
- les positions relatives rédigées pas à pas (`<Étudie>la position
  relative ...`) : droites/droites, droite/plan, plans/plans — tous les cas
  (confondues, parallèles, sécantes, non coplanaires) conclus proprement.

**Les courbes et les coniques.**
- Courbes paramétrées et polaires (`<Trace>la courbe paramétrée ...`,
  `<Trace>la courbe polaire ...`), repère isotrope, couleurs du traceur.
- Coniques rédigées (`<Étudie>la conique d'équation ...`) : réduction
  complète (discriminant, rotation si terme croisé, centre, équation
  réduite), éléments caractéristiques (demi-axes, excentricité, foyers,
  asymptotes pour l'hyperbole, sommet/foyer/directrice pour la parabole),
  cas dégénérés reconnus, tracé avec centre et foyers marqués.

**Les fonctions de deux variables.**
- Surfaces `z = f(x, y)` en perspective cavalière, maillées et ombrées
  selon l'orientation des facettes.
- Lignes de niveau par *marching squares*, niveaux automatiques ou choisis.
- Extremums sous contrainte par multiplicateurs de Lagrange, candidats
  classés (minimum/maximum).
- Intégrales doubles et triples sur un pavé (Fubini) et sur un disque
  (passage en polaires rédigé, jacobien explicite).

**Le plan complexe et les résidus.**
- Évaluateur complexe intégré (`i`, `exp`, `ln`, `sin`, `cos`, `sqrt`,
  puissances).
- Image d'un domaine par une transformation holomorphe (`<Trace>l'image du
  carré ... par w = ...`, `<Trace>l'image du cercle ... par w = ...`).
- Résidus rédigés (`<Calcule>les résidus de ...`) : pôles, ordres,
  `Res(f, z₀)` pour chacun.

**Les groupes.**
- Tables de `ℤ/nℤ` pour l'addition et la multiplication
  (`<Dresse>la table de Z/5Z pour l'addition`).
- Générateurs avec l'indicatrice d'Euler (`<Détermine>les générateurs de
  Z/12Z`).
- Décomposition d'une permutation en cycles à supports disjoints, points
  fixes, signature, ordre (`<Décompose>la permutation (...) en cycles`).

**Les lois à densité.**
- Vérification qu'une fonction est une densité de probabilité (intégrale
  1, bornes finies ou infinies), espérance et variance rédigées
  (`<Étudie>la loi de densité ...`).
- Probabilités exactes (en erf) et approchées pour la loi normale ; tracé
  de sa densité.
- Illustration du théorème central limite : loi exacte de la somme de *n*
  dés (par convolution) superposée à sa limite normale.

### Corrigé

- `exp(x)` s'affiche désormais `e^x`, notation francophone, au lieu de
  `\exp(x)`.
- L'interpolation `#nom` replie sur le plus long préfixe de variable connu
  quand le nom lu gloutonnement n'existe pas (`Z/#nZ` avec `n` défini donne
  `Z/7Z` et non un `#nZ` littéral).
- Les cotes et mesures des solides et patrons (module espace) sont
  désormais placées à l'extérieur de la figure, parallèles au segment
  mesuré — reprise du principe déjà appliqué en géométrie plane.
- L'interpolation `#` fonctionne désormais dans la description de toute
  balise (`<Trace>`, `<Calcule>`, `<Étudie>`...), pas seulement `<Trace>`.

### Changé

- Réorganisation des exemples par domaine plutôt que par chantier : les
  nouveautés ci-dessus vivent dans les fichiers `geometrie2/3/4`,
  `analyse4`, `algebre4`, `statistiques-probabilites4`, `basique3`,
  suivant la logique déjà en place pour la 1.0.
- Suppression de `outils/`, `transpiler/examples/` et `transpiler/tests/`
  du dépôt public.

## 1.0 — première publication

Version initiale : docdg couvre le programme de mathématiques du CP aux
classes préparatoires (MPSI/MP), avec la syntaxe en prose française
imperative, la rédaction pas à pas par défaut, et l'export PDF modifiable.

### Points marquants

- Objets et actions de base : `<Soit>`, `<Trace>`, `<Affiche>`, `<Dresse>`,
  `<Calcule>`, `<Étudie>`, `<Résous>`, `<Détermine>`, `<Donne>`,
  `<Applique>`, `<Construis>`, `<Dénombre>`, `<Pose>`, `<Insère>`.
- Collège : trio Pythagore/Thalès/trigonométrie, statistiques d'une série,
  proportionnalité et pourcentages, mesures et conversions, programmes de
  calcul, transformations du plan.
- Lycée : convexité, asymptotes, équations trigonométriques, vecteurs et
  produit scalaire du plan et de l'espace, graphes et matrices
  d'adjacence, chaînes de Markov et état stable, équations diophantiennes,
  racines de l'unité, intervalle de fluctuation, inégalité de
  Bienaymé-Tchebychev.
- Supérieur (CPGE) : algèbre linéaire, réduction, polynômes, séries,
  analyse (limites, dérivées, intégrales, développements limités),
  physique-chimie (unités, conversions, Fourier, Laplace).
- Géométrie plane complète, figures avec repère orthonormé et cercle
  trigonométrique, solides en 3D de base, mode analytique.
- Statistiques : diagrammes, arbres de probabilités, droite graduée.
- Tables de Cayley pour `ℤ/nℤ` et `(ℤ/nℤ)*`.
- Mise en forme : styles en ligne, couleurs, tabulations, notes de bas de
  page, alias `soit`, structures de contrôle, calculs `#{...}`.
- Export PDF modifiable — la différence structurelle avec LaTeX et Typst.
- Publication sur CTAN aux côtés de texecole et scholatex.
