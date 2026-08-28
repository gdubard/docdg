# docdg — La rédaction

Le catalogue du corpus : chaque énoncé, la phrase qui l'appelle, les niveaux
où il est exigible, et l'état de sa démonstration.

> Ce document est **engendré** et ne se modifie pas à la main :
> `cargo app recensement --redaction` le réécrit depuis `corpus/donnees/`
> et `transpiler/src/maths/demonstrations.json`.

---

## **📊 Ce que le corpus contient**

| | Énoncés | Démonstrations rédigées |
|---|---:|---:|
| Mathématiques | 622 | 354 |
| Physique | 438 | 148 |
| Chimie | 313 | 67 |
| **Total** | **1373** | **569** |

S'y ajoutent **243 grandeurs physiques** et **122 cadres d'hypothèses**, et une
**bibliothèque de démonstrations classiques** servie par `<Montre>`, décrite
en fin de document.

### Par niveau

| Niveau | Cycle | Ancrages | dont mathématiques | physique | chimie |
|---|---|---:|---:|---:|---:|
| CP | école | 7 | 7 | 0 | 0 |
| CE1 | école | 12 | 12 | 0 | 0 |
| CE2 | école | 17 | 13 | 3 | 1 |
| CM1 | école | 23 | 18 | 3 | 2 |
| CM2 | école | 14 | 10 | 3 | 1 |
| sixième | collège | 22 | 22 | 0 | 0 |
| cinquième | collège | 65 | 46 | 9 | 10 |
| quatrième | collège | 54 | 28 | 15 | 11 |
| troisième | collège | 52 | 33 | 10 | 9 |
| seconde | lycée | 83 | 42 | 18 | 23 |
| première | lycée | 120 | 37 | 43 | 40 |
| terminale | lycée | 143 | 61 | 48 | 34 |
| licence 1 | supérieur | 181 | 101 | 45 | 35 |
| licence 2 | supérieur | 189 | 59 | 66 | 64 |
| licence 3 | supérieur | 227 | 100 | 87 | 40 |
| master 1 | supérieur | 299 | 64 | 137 | 98 |
| master 2 | supérieur | 165 | 75 | 78 | 12 |
| **Total** | | **1673** | | | |

---

## **🧭 Comment lire ce catalogue**

Chaque énoncé est rangé **au premier niveau où il est exigible** ; ses
ancrages ultérieurs figurent dans sa ligne. La colonne **Appel** donne la
phrase à écrire — elle fonctionne telle quelle :

```docdg
<Énonce>le théorème de Pythagore
<Démontre>le théorème de Pythagore
```

`<Énonce>` compose l'énoncé, sa formule, ses hypothèses et son domaine de
validité. `<Démontre>` sert la démonstration du corpus, rédigée au niveau le
plus proche de celui du document — et **le dit** quand elle vient d'un autre.

| Colonne | Sens |
|---|---|
| **Type** | axiome, définition, propriété, règle, lemme, théorème, corollaire, proposition, loi, principe, postulat, modèle, relation |
| **Appel** | la phrase qui résout l'énoncé — le titre, avec son article |
| **Ancrages** | les niveaux où l'énoncé est exigible, et la voie s'il y en a une |
| **Démonstration** | ✅ rédigée, au niveau indiqué · 🕐 attendue mais non rédigée (`demonstration_prevue`) · — annoncée démontrée ailleurs · · admis, sans démonstration attendue |

---

## **Mathématiques**

### CP

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Commutativité de l'addition` | CP · CE1 | · |
| règle | `Comparaison de deux entiers` | CP · CE1 | · |
| propriété | `Compléments à dix` | CP | · |
| règle | `Numération de position` | CP · CE2 | · |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Euro et centime` | CP · CE1 | · |
| définition | `Report d'une longueur` | CP · CE1 | · |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Points alignés` | CP · CE1 | · |

### CE1

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Addition posée et retenue` | CE1 | · |
| définition | `Double et moitié` | CE1 | · |
| définition | `Multiplication comme addition réitérée` | CE1 | · |
| définition | `Nombre pair et nombre impair` | CE1 | · |
| règle | `Tables de multiplication` | CE1 · CE2 | · |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Lecture de l'heure` | CE1 · CE2 | · |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Figures planes usuelles` | CE1 | · |

### CE2

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Commutativité de la multiplication` | CE2 | · |
| règle | `Multiplication par dix, cent et mille` | CE2 · CM1 | · |
| définition | `Partage et groupement` | CE2 | · |
| règle | `Soustraction posée` | CE2 | · |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Unités de longueur et conversions` | CE2 · CM1 | · |
| règle | `Unités de masse` | CE2 | · |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Angle droit` | CE2 | · |
| définition | `Axe de symétrie d'une figure` | CE2 · CM1 | · |
| définition | `Solides usuels` | CE2 · CM1 | · |

#### statistiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Tableau à double entrée` | CE2 · CM2 | · |

### CM1

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Fraction, part de l'unité` | CM1 | · |
| définition | `Fractions décimales` | CM1 | · |
| règle | `Multiplication posée` | CM1 | · |
| propriété | `Quotient et reste d'une division` | CM1 · CM2 | · |
| définition | `Écriture décimale d'un nombre` | CM1 · CM2 | · |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Aire par pavage` | CM1 · CM2 | · |
| règle | `Calcul d'une durée` | CM1 · CM2 | · |
| définition | `Périmètre d'un polygone` | CM1 | · |
| règle | `Unités de contenance` | CM1 | · |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Cercle, centre et rayon` | CM1 | · |
| propriété | `Conservation par symétrie axiale` | CM1 · sixième | · |
| définition | `Demi-droite graduée` | CM1 · sixième | · |
| définition | `Droites parallèles` | CM1 | · |
| définition | `Droites perpendiculaires` | CM1 | · |

### CM2

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Comparaison de deux nombres décimaux` | CM2 | · |
| règle | `Division posée` | CM2 | · |
| règle | `Ordre de grandeur d'un calcul` | CM2 | · |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Aire d'un rectangle` | CM2 · sixième | · |
| propriété | `Volume d'un pavé droit` | CM2 · sixième | · |

### Sixième

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Comparaison de deux fractions` | sixième | · |
| propriété | `Égalité de deux fractions` | sixième | · |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Critères de divisibilité par 2, 5 et 10` | sixième · terminale | ✅ terminale |
| propriété | `Critères de divisibilité par 3 et par 9` | sixième · terminale | ✅ terminale |
| définition | `Diviseur et multiple` | sixième | · |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Aire d'un disque` | sixième · terminale | ✅ terminale |
| théorème | `Aire d'un triangle` | sixième | ✅ sixième |
| propriété | `Périmètre d'un cercle` | sixième | · |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Angle plat` | sixième | · |
| définition | `Bissectrice d'un angle` | sixième | · |
| définition | `Hauteur d'un triangle` | sixième | · |
| définition | `Médiatrice d'un segment` | sixième | · |
| propriété | `Propriétés du carré` | sixième | · |
| théorème | `Somme des angles d'un triangle` | sixième · cinquième | ✅ cinquième |

#### proportionnalité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Pourcentage` | sixième | · |
| définition | `Situation de proportionnalité` | sixième | · |

#### statistiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Moyenne d'une série statistique` | sixième | · |
| définition | `Étendue d'une série statistique` | sixième | · |

### Cinquième

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Distributivité simple de la multiplication sur l'addition` | cinquième | · |
| définition | `Opposé d'un nombre` | cinquième | · |
| règle | `Réduction d'une expression littérale` | cinquième | · |
| propriété | `Somme de deux fractions` | cinquième | ✅ cinquième |
| propriété | `Soustraire, c'est ajouter l'opposé` | cinquième | · |
| définition | `Test d'une égalité` | cinquième | · |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Différence de deux nombres relatifs` | cinquième | · |
| définition | `Nombre relatif` | cinquième | · |
| règle | `Priorités opératoires` | cinquième | · |
| règle | `Somme de deux nombres relatifs` | cinquième | · |
| définition | `Écriture fractionnaire` | cinquième | · |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Aire d'un parallélogramme` | cinquième | ✅ cinquième |
| théorème | `Aire d'un trapèze` | cinquième | ✅ cinquième |
| propriété | `Volume d'un prisme droit et d'un cylindre` | cinquième | · |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Angles alternes-internes` | cinquième | · |
| propriété | `Angles correspondants` | cinquième | ✅ cinquième |
| corollaire | `Angles d'un triangle équilatéral` | cinquième | ✅ cinquième |
| propriété | `Angles opposés par le sommet` | cinquième | ✅ cinquième |
| théorème | `Angles à la base d'un triangle isocèle` | cinquième | ✅ cinquième |
| théorème | `Caractérisation de la médiatrice par équidistance` | cinquième | ✅ cinquième |
| théorème | `Caractérisation du parallélogramme par ses diagonales` | cinquième | ✅ cinquième |
| propriété | `Cas d'isométrie des triangles : deux côtés et l'angle compris` | cinquième | · |
| propriété | `Cas d'isométrie des triangles : trois côtés` | cinquième | · |
| propriété | `Cas d'isométrie des triangles : un côté et les deux angles adjacents` | cinquième | · |
| théorème | `Cercle circonscrit à un triangle` | cinquième | ✅ cinquième |
| théorème | `Concours des hauteurs` | cinquième | · |
| théorème | `Concours des médianes` | cinquième · seconde | ✅ seconde |
| propriété | `Conservation par symétrie centrale` | cinquième | · |
| théorème | `Côtés opposés d'un parallélogramme` | cinquième | ✅ cinquième |
| théorème | `Diagonales d'un parallélogramme` | cinquième | ✅ cinquième |
| théorème | `Inégalité triangulaire` | cinquième | · |
| définition | `Médiane d'un triangle` | cinquième | · |
| définition | `Parallélogramme` | cinquième | · |
| propriété | `Propriétés du losange` | cinquième | · |
| propriété | `Propriétés du rectangle` | cinquième | · |
| définition | `Repère du plan` | cinquième | · |
| théorème | `Réciproque : deux angles de même mesure caractérisent le triangle isocèle` | cinquième | · |
| corollaire | `Somme des angles d'un quadrilatère` | cinquième | ✅ cinquième |
| propriété | `Une médiane partage le triangle en deux triangles de même aire` | cinquième | ✅ cinquième |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Probabilité d'un événement` | cinquième | · |
| propriété | `Somme des probabilités des issues` | cinquième | · |

#### proportionnalité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Augmentation et diminution en pourcentage` | cinquième · quatrième | ✅ quatrième |
| définition | `Vitesse moyenne` | cinquième · quatrième | · |
| définition | `Échelle d'un plan` | cinquième | · |

#### statistiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Effectif et fréquence` | cinquième | · |

### Quatrième

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Double distributivité` | quatrième | ✅ quatrième |
| définition | `Inverse d'un nombre non nul` | quatrième | · |
| définition | `Notation scientifique` | quatrième | · |
| propriété | `Opérations conservant les solutions d'une équation` | quatrième | · |
| propriété | `Produit de deux fractions` | quatrième · licence 1 | ✅ licence 1 |
| propriété | `Produit de deux puissances de même base` | quatrième | ✅ quatrième |
| définition | `Puissance d'exposant entier négatif` | quatrième | · |
| définition | `Puissance d'exposant entier positif` | quatrième | · |
| propriété | `Puissance d'une puissance` | quatrième | ✅ quatrième |
| propriété | `Quotient de deux fractions` | quatrième | ✅ quatrième |
| propriété | `Quotient de deux puissances de même base` | quatrième | ✅ quatrième |
| règle | `Règle des signes` | quatrième · licence 1 | ✅ licence 1 |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Volume d'une pyramide et d'un cône` | quatrième · terminale | ✅ terminale |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Conservation par translation et par rotation` | quatrième | · |
| corollaire | `Contraposée du théorème de Pythagore` | quatrième | ✅ quatrième |
| théorème | `Longueur du segment des milieux` | quatrième | ✅ quatrième |
| théorème | `Réciproque : triangle inscrit dans un demi-cercle` | quatrième | ✅ quatrième |
| théorème | `Réciproque du théorème de la droite des milieux` | quatrième | ✅ quatrième |
| théorème | `Réciproque du théorème de Pythagore` | quatrième | ✅ quatrième |
| théorème | `Théorème de la droite des milieux` | quatrième | ✅ quatrième |
| théorème | `Théorème de Pythagore` | quatrième | ✅ quatrième |
| théorème | `Théorème de Thalès (configuration du triangle)` | quatrième · seconde | ✅ seconde |
| théorème | `Triangle rectangle et cercle circonscrit` | quatrième | ✅ quatrième |

#### proportionnalité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Égalité des produits en croix` | quatrième | ✅ quatrième |

#### statistiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Moyenne pondérée` | quatrième | · |

#### trigonométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Cosinus d'un angle aigu` | quatrième | · |

### Troisième

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Existence et unicité de la racine carrée` | troisième · licence 1 | ✅ licence 1 |
| théorème | `Identité remarquable : carré d'une différence` | troisième | ✅ troisième |
| théorème | `Identité remarquable : carré d'une somme` | troisième | ✅ troisième |
| théorème | `Identité remarquable : produit de la somme par la différence` | troisième | ✅ troisième |
| propriété | `La racine carrée n'est pas additive` | troisième | ✅ troisième |
| propriété | `Multiplication d'une inéquation par un nombre négatif` | troisième | · |
| théorème | `Produit nul` | troisième | ✅ troisième |
| propriété | `Racine carrée d'un carré` | troisième | ✅ troisième |
| définition | `Racine carrée d'un nombre positif` | troisième | · |
| propriété | `Racine carrée d'un produit` | troisième | ✅ troisième |
| propriété | `Racine carrée d'un quotient` | troisième | ✅ troisième |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Algorithme d'Euclide` | troisième | ✅ troisième |
| théorème | `Décomposition en produit de facteurs premiers` | troisième · licence 1 · master 2 (agrégation externe) | ✅ licence 1 · master 2 |
| définition | `Nombre premier` | troisième | · |
| définition | `Nombres premiers entre eux` | troisième | · |
| définition | `Plus grand commun diviseur` | troisième | · |
| théorème | `Rendre une fraction irréductible` | troisième | ✅ troisième |

#### fonctions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Fonction` | troisième | · |
| définition | `Fonction affine` | troisième | · |
| définition | `Fonction linéaire` | troisième | · |
| théorème | `Fonctions linéaires et proportionnalité` | troisième | ✅ troisième |

#### grandeurs et mesures

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Effet d'un agrandissement sur les longueurs, aires et volumes` | troisième | ✅ troisième |
| propriété | `Volume d'une boule et aire d'une sphère` | troisième · terminale | ✅ terminale |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Effet d'une homothétie sur les longueurs et les angles` | troisième · seconde | ✅ seconde |
| définition | `Homothétie` | troisième | · |
| théorème | `Réciproque du théorème de Thalès` | troisième · seconde | ✅ seconde |
| théorème | `Théorème de Thalès` | troisième · seconde | ✅ seconde |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Probabilité de l'événement contraire` | troisième | ✅ troisième |

#### statistiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Médiane d'une série statistique` | troisième | · |

#### trigonométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Relation fondamentale de la trigonométrie` | troisième | ✅ troisième |
| définition | `Sinus d'un angle aigu` | troisième | · |
| propriété | `Tangente comme quotient du sinus par le cosinus` | troisième | ✅ troisième |
| définition | `Tangente d'un angle aigu` | troisième | · |

### Seconde

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Inclusions des ensembles de nombres` | seconde | · |
| définition | `Intervalles de nombres réels` | seconde | · |
| propriété | `Inverse d'un irrationnel` | seconde | ✅ seconde |
| théorème | `Irrationalité de la racine carrée de 2` | seconde · master 2 (agrégation externe) | ✅ seconde · master 2 |
| propriété | `Maximum et minimum par la valeur absolue` | seconde | ✅ seconde |
| propriété | `Ordre et opérations` | seconde | · |
| propriété | `Ordre et passage au carré` | seconde | ✅ seconde |
| propriété | `Ordre et passage à l'inverse` | seconde | ✅ seconde |
| théorème | `Somme d'un rationnel et d'un irrationnel` | seconde | ✅ seconde |
| propriété | `Stabilité de l'ensemble des rationnels` | seconde | ✅ seconde |
| définition | `Valeur absolue et distance` | seconde | · |
| théorème | `Équation quotient nul` | seconde | ✅ seconde |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Il n'existe pas de plus grand entier naturel` | seconde | ✅ seconde |

#### fonctions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Ensemble de définition` | seconde | · |
| définition | `Sens de variation d'une fonction` | seconde | · |
| théorème | `Signe d'une fonction affine` | seconde | ✅ seconde |
| propriété | `Variations d'une fonction affine` | seconde | ✅ seconde |
| propriété | `Variations de la fonction carré` | seconde | ✅ seconde |
| propriété | `Variations de la fonction inverse` | seconde | ✅ seconde |
| propriété | `Variations de la fonction racine carrée` | seconde | ✅ seconde |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Coordonnées d'un vecteur` | seconde | · |
| théorème | `Coordonnées du milieu d'un segment` | seconde | ✅ seconde |
| théorème | `Critère de colinéarité par le déterminant` | seconde | ✅ seconde |
| théorème | `Distance entre deux points dans un repère orthonormé` | seconde | ✅ seconde |
| théorème | `Parallélisme et vecteurs directeurs` | seconde | ✅ seconde |
| propriété | `Relation de Chasles` | seconde | · |
| théorème | `Théorème de Thalès, version vectorielle` | seconde | ✅ seconde |
| définition | `Vecteur` | seconde | · |
| définition | `Vecteur directeur d'une droite` | seconde | · |
| définition | `Vecteurs colinéaires` | seconde | · |
| théorème | `Équation cartésienne d'une droite` | seconde | ✅ seconde |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Probabilité d'une réunion` | seconde | ✅ seconde |
| définition | `Événements incompatibles` | seconde | · |

#### statistiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Quartiles` | seconde | · |
| définition | `Variance et écart-type d'une série statistique` | seconde | · |

#### trigonométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Cercle trigonométrique et enroulement de la droite réelle` | seconde | · |
| définition | `Radian` | seconde | · |

### Première

#### acoustique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Construction de la gamme de Pythagore` | première (enseignement scientifique) | · |

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Discriminant et racines d'un trinôme` | première (spécialité mathématiques) | ✅ première |
| corollaire | `Factorisation d'un trinôme` | première (spécialité mathématiques) | ✅ première |
| théorème | `Forme canonique d'un trinôme` | première (spécialité mathématiques) | ✅ première |
| théorème | `Signe d'un trinôme` | première (spécialité mathématiques) | ✅ première |
| propriété | `Somme et produit des racines` | première (spécialité mathématiques) | ✅ première |

#### analyse

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Dérivée d'un quotient` | première | ✅ première |
| théorème | `Dérivée d'une somme et d'un produit` | première | ✅ première |
| théorème | `Dérivées des fonctions usuelles` | première | ✅ première |
| théorème | `Existence et unicité de la fonction exponentielle` | première | · |
| théorème | `Extremum local et dérivée` | première | ✅ première |
| théorème | `L'exponentielle ne s'annule jamais` | première | ✅ première |
| théorème | `L'exponentielle ne s'annule pas et reste strictement positive` | première | ✅ première |
| théorème | `Lien entre signe de la dérivée et variations` | première · licence 1 | ✅ licence 1 |
| définition | `Nombre dérivé` | première | · |
| théorème | `Relation fonctionnelle de l'exponentielle` | première | ✅ première |
| théorème | `Somme des termes d'une suite arithmétique` | première | ✅ première |
| théorème | `Somme des termes d'une suite géométrique` | première | ✅ première |
| définition | `Suite arithmétique` | première | · |
| définition | `Suite géométrique` | première | · |
| définition | `Suite numérique` | première | · |
| théorème | `Terme général d'une suite arithmétique` | première | ✅ première |
| théorème | `Terme général d'une suite géométrique` | première | ✅ première |
| corollaire | `Variations de l'exponentielle` | première | ✅ première |
| théorème | `Équation de la tangente` | première | ✅ première |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Caractérisation de l'orthogonalité` | première | ✅ première |
| définition | `Produit scalaire` | première | · |
| théorème | `Produit scalaire en coordonnées` | première | ✅ première |
| théorème | `Théorème d'Al-Kashi` | première | ✅ première |
| théorème | `Équation cartésienne d'un cercle` | première | ✅ première |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Espérance, variance et écart-type d'une variable aléatoire` | première | · |
| théorème | `Formule de Kœnig-Huygens` | première | ✅ première |
| théorème | `Formule des probabilités totales` | première | ✅ première |
| définition | `Indépendance de deux événements` | première | · |
| théorème | `Linéarité de l'espérance` | première | ✅ première |
| définition | `Probabilité conditionnelle` | première | · |
| définition | `Variable aléatoire et loi de probabilité` | première | · |

### Terminale

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Nombre complexe, forme algébrique` | terminale (mathématiques expertes) | · |

#### analyse

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Continuité` | terminale | · |
| définition | `Convexité` | terminale | · |
| théorème | `Convexité et dérivée seconde` | terminale · licence 1 | ✅ licence 1 |
| corollaire | `Corollaire du théorème des valeurs intermédiaires` | terminale | ✅ terminale |
| théorème | `Croissances comparées` | terminale | ✅ terminale |
| théorème | `Deux primitives diffèrent d'une constante` | terminale | ✅ terminale |
| théorème | `Dérivée d'une fonction composée` | terminale · licence 1 | ✅ licence 1 |
| théorème | `Dérivée du logarithme` | terminale | ✅ terminale |
| définition | `Fonction logarithme népérien` | terminale | · |
| définition | `Intégrale d'une fonction continue` | terminale | · |
| théorème | `Inégalité de Bernoulli` | terminale | ✅ terminale |
| définition | `Limite d'une fonction` | terminale | · |
| définition | `Limite d'une suite` | terminale | · |
| théorème | `Limite d'une suite géométrique` | terminale | ✅ terminale |
| propriété | `Linéarité de l'intégrale` | terminale | ✅ terminale |
| théorème | `Opérations sur les limites` | terminale · licence 1 | ✅ licence 1 |
| propriété | `Positivité et croissance de l'intégrale` | terminale | ✅ terminale |
| définition | `Primitive` | terminale | · |
| propriété | `Relation de Chasles pour l'intégrale` | terminale | ✅ terminale |
| théorème | `Relation fonctionnelle du logarithme` | terminale | ✅ terminale |
| théorème | `Solutions de l'équation différentielle y' = ay + b` | terminale | ✅ terminale |
| théorème | `Théorème d'encadrement` | terminale · licence 1 | ✅ licence 1 |
| théorème | `Théorème de comparaison` | terminale | ✅ terminale |
| théorème | `Théorème de la convergence monotone` | terminale · licence 1 | ✅ licence 1 |
| théorème | `Théorème des valeurs intermédiaires` | terminale · licence 1 | ✅ licence 1 |
| théorème | `Toute fonction dérivable est continue` | terminale · licence 1 | ✅ licence 1 |
| théorème | `Une suite croissante non majorée tend vers l'infini` | terminale | ✅ terminale |
| théorème | `Unicité de la limite` | terminale | ✅ terminale |
| définition | `Valeur moyenne d'une fonction` | terminale | · |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Compatibilité des congruences avec les opérations` | terminale (mathématiques expertes) | ✅ terminale |
| définition | `Congruence modulo n` | terminale (mathématiques expertes) | · |
| théorème | `Petit théorème de Fermat` | terminale (mathématiques expertes) · master 2 (agrégation externe) | ✅ terminale · master 2 |
| théorème | `Théorème de Bézout` | terminale (mathématiques expertes) · master 2 (agrégation externe) | ✅ terminale · master 2 |
| théorème | `Théorème de Gauss` | terminale (mathématiques expertes) | ✅ terminale |

#### complexes

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Conjugué d'un nombre complexe` | terminale (mathématiques expertes) | · |
| définition | `Forme trigonométrique et forme exponentielle` | terminale (mathématiques expertes) | · |
| théorème | `Formule de Moivre` | terminale (mathématiques expertes) | ✅ terminale |
| définition | `Module d'un nombre complexe` | terminale (mathématiques expertes) | · |
| propriété | `Propriétés du conjugué` | terminale (mathématiques expertes) | ✅ terminale |
| propriété | `Propriétés du module` | terminale (mathématiques expertes) | ✅ terminale |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Orthogonalité d'une droite et d'un plan` | terminale | ✅ terminale |
| définition | `Produit scalaire dans l'espace` | terminale | · |
| définition | `Vecteur normal à un plan` | terminale | · |
| définition | `Vecteurs de l'espace, base et repère` | terminale | · |
| théorème | `Équation cartésienne d'un plan` | terminale | ✅ terminale |

#### logique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Principe de raisonnement par récurrence` | terminale | · |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Coefficient binomial` | terminale | · |
| théorème | `Espérance et variance de la loi binomiale` | terminale · licence 1 | ✅ licence 1 |
| théorème | `Formule de Bayes` | terminale | ✅ terminale |
| théorème | `Inégalité de Bienaymé-Tchebychev` | terminale | ✅ terminale |
| corollaire | `Inégalité de concentration` | terminale | ✅ terminale |
| définition | `Loi binomiale` | terminale | · |
| théorème | `Loi des grands nombres` | terminale | ✅ terminale |
| théorème | `Probabilités d'une loi binomiale` | terminale | ✅ terminale |
| théorème | `Relation de Pascal` | terminale | ✅ terminale |

### Licence 1

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Anneau et corps` | licence 1 | · |
| définition | `Anneau intègre et diviseurs de zéro` | licence 1 | · |
| théorème | `Caractérisation d'un sous-groupe` | licence 1 | ✅ licence 1 |
| propriété | `Degré d'un produit de polynômes` | licence 1 | ✅ licence 1 |
| théorème | `Division euclidienne des polynômes` | licence 1 | ✅ licence 1 |
| définition | `Groupe` | licence 1 | · |
| propriété | `Inverse d'un produit` | licence 1 | ✅ licence 1 |
| définition | `Polynôme et degré` | licence 1 | · |
| théorème | `Racines et factorisation` | licence 1 | ✅ licence 1 |
| théorème | `Théorème de d'Alembert-Gauss` | licence 1 · licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Tout corps est intègre` | licence 1 | ✅ licence 1 |
| théorème | `Une puissance irrationnelle peut être rationnelle` | licence 1 | · |
| propriété | `Unicité du neutre et du symétrique` | licence 1 | ✅ licence 1 |

#### algèbre linéaire

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Application linéaire, noyau et image` | licence 1 | · |
| théorème | `Caractérisation d'un sous-espace vectoriel` | licence 1 | ✅ licence 1 |
| définition | `Espace vectoriel` | licence 1 | · |
| théorème | `Espaces de même dimension` | licence 1 | ✅ licence 1 |
| définition | `Famille libre, génératrice, base` | licence 1 | · |
| théorème | `Formule de Grassmann` | licence 1 | ✅ licence 1 |
| théorème | `Injectivité et noyau` | licence 1 | ✅ licence 1 |
| propriété | `Intersection de sous-espaces vectoriels` | licence 1 | ✅ licence 1 |
| théorème | `Le noyau et l'image sont des sous-espaces` | licence 1 | ✅ licence 1 |
| théorème | `Lemme d'échange de Steinitz` | licence 1 | · |
| définition | `Matrice d'une application linéaire` | licence 1 | · |
| théorème | `Rang d'une matrice` | licence 1 | · |
| théorème | `Théorème de la base incomplète` | licence 1 | · |
| théorème | `Théorème du rang` | licence 1 · master 2 (agrégation externe) | ✅ licence 1 · master 2 |

#### analyse

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Caractérisation séquentielle de la borne supérieure` | licence 1 | ✅ licence 1 |
| théorème | `Caractérisation séquentielle de la limite` | licence 1 | ✅ licence 1 |
| théorème | `Changement de variable` | licence 1 | ✅ licence 1 |
| théorème | `Complétude de R` | licence 1 | ✅ licence 1 |
| théorème | `Composée de fonctions continues` | licence 1 | ✅ licence 1 |
| théorème | `Densité de Q dans R` | licence 1 | ✅ licence 1 |
| propriété | `Développements limités usuels en 0` | licence 1 | ✅ licence 1 |
| théorème | `Existence et unicité de la partie entière` | licence 1 | ✅ licence 1 |
| théorème | `Formule de Leibniz` | licence 1 | ✅ licence 1 |
| théorème | `Formule de Taylor-Lagrange` | licence 1 · master 2 (agrégation externe) | ✅ licence 1 · master 2 |
| théorème | `Formule de Taylor-Young` | licence 1 | · |
| théorème | `Intégrabilité des fonctions continues` | licence 1 | · |
| définition | `Intégrale de Riemann` | licence 1 | · |
| théorème | `Intégration par parties` | licence 1 | ✅ licence 1 |
| propriété | `Inégalité de la moyenne` | licence 1 | ✅ licence 1 |
| corollaire | `Inégalité des accroissements finis` | licence 1 | ✅ licence 1 |
| théorème | `Lemme de Cesàro` | licence 1 | ✅ licence 1 |
| définition | `Limite d'une fonction, définition quantifiée` | licence 1 | · |
| propriété | `Produit d'une suite bornée par une suite de limite nulle` | licence 1 | ✅ licence 1 |
| théorème | `Propriété d'Archimède` | licence 1 | ✅ licence 1 |
| axiome | `Propriété de la borne supérieure` | licence 1 | · |
| définition | `Suite de Cauchy` | licence 1 | · |
| théorème | `Suites adjacentes` | licence 1 | ✅ licence 1 |
| théorème | `Théorème de Bolzano-Weierstrass` | licence 1 · master 2 (agrégation externe) | ✅ licence 1 · master 2 |
| théorème | `Théorème de Darboux` | licence 1 | · |
| théorème | `Théorème de la bijection` | licence 1 | ✅ licence 1 |
| théorème | `Théorème de Rolle` | licence 1 · master 2 (agrégation externe) | ✅ licence 1 · master 2 |
| théorème | `Théorème des accroissements finis` | licence 1 | ✅ licence 1 |
| théorème | `Théorème des bornes atteintes` | licence 1 | ✅ licence 1 |
| théorème | `Théorème fondamental de l'analyse` | licence 1 | ✅ licence 1 |
| théorème | `Toute suite convergente est bornée` | licence 1 | ✅ licence 1 |
| propriété | `Toute suite de Cauchy est bornée` | licence 1 | ✅ licence 1 |
| théorème | `Toute suite extraite d'une suite convergente converge` | licence 1 | ✅ licence 1 |
| théorème | `Une fonction de dérivée nulle sur un intervalle est constante` | licence 1 | ✅ licence 1 |
| corollaire | `Une suite non bornée diverge` | licence 1 | ✅ licence 1 |
| théorème | `Unicité de la limite d'une fonction` | licence 1 | ✅ licence 1 |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Division euclidienne dans Z` | licence 1 | ✅ licence 1 |
| théorème | `Infinité des nombres premiers` | licence 1 · master 2 (agrégation externe) | ✅ licence 1 · master 2 |
| théorème | `Irrationalité de la racine d'un entier non carré` | licence 1 | ✅ licence 1 |
| théorème | `Le plus petit diviseur d'un entier est premier` | licence 1 | ✅ licence 1 |
| théorème | `Lemme d'Euclide` | licence 1 | ✅ licence 1 |

#### dénombrement

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Nombre de parties d'un ensemble fini` | licence 1 | ✅ licence 1 |
| théorème | `Principe des tiroirs` | licence 1 | ✅ licence 1 |

#### ensembles

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Cardinal d'un ensemble fini` | licence 1 | · |
| théorème | `Composée d'injections et de surjections` | licence 1 | ✅ licence 1 |
| théorème | `Composée de bijections` | licence 1 | ✅ licence 1 |
| propriété | `Distributivité de la réunion et de l'intersection` | licence 1 | ✅ licence 1 |
| définition | `Ensemble dénombrable` | licence 1 | · |
| définition | `Injection, surjection, bijection` | licence 1 | · |
| théorème | `Injectivité, surjectivité et ensembles finis` | licence 1 | ✅ licence 1 |
| théorème | `L'ensemble des rationnels est dénombrable` | licence 1 | · |
| théorème | `L'ensemble des réels n'est pas dénombrable` | licence 1 | ✅ licence 1 |
| théorème | `Les classes d'équivalence forment une partition` | licence 1 | ✅ licence 1 |
| définition | `Relation d'ordre` | licence 1 | · |
| définition | `Relation d'équivalence` | licence 1 | · |
| théorème | `Théorème de Cantor-Bernstein` | licence 1 | · |

#### fonctions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Décomposition en partie paire et partie impaire` | licence 1 | ✅ licence 1 |

#### logique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Implication, contraposée, réciproque` | licence 1 | ✅ licence 1 |
| propriété | `Lois de De Morgan` | licence 1 | ✅ licence 1 |
| propriété | `Négation des propositions quantifiées` | licence 1 | ✅ licence 1 |
| théorème | `Récurrence forte` | licence 1 | ✅ licence 1 |

### Licence 2

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Signature d'une permutation` | licence 2 | · |
| théorème | `Z/nZ est un corps si et seulement si n est premier` | licence 2 · master 2 (agrégation externe) | ✅ licence 2 · master 2 |

#### algèbre linéaire

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Base duale et bidualité` | licence 2 | ✅ licence 2 |
| théorème | `Critères de diagonalisabilité` | licence 2 | ✅ licence 2 |
| définition | `Déterminant` | licence 2 | · |
| théorème | `Déterminant de Vandermonde` | licence 2 | ✅ licence 2 |
| théorème | `Déterminant et inversibilité` | licence 2 | ✅ licence 2 |
| définition | `Espace vectoriel quotient` | licence 2 | · |
| définition | `Forme linéaire, espace dual` | licence 2 | · |
| théorème | `Inégalité de Cauchy-Schwarz` | licence 2 · master 2 (agrégation externe) | ✅ licence 2 · master 2 |
| corollaire | `Inégalité de Minkowski` | licence 2 | ✅ licence 2 |
| théorème | `Lemme des noyaux` | licence 2 · master 2 (agrégation externe) | ✅ licence 2 · master 2 |
| théorème | `Méthode du pivot de Gauss` | licence 2 | ✅ licence 2 |
| théorème | `Polynôme caractéristique` | licence 2 | ✅ licence 2 |
| théorème | `Procédé d'orthonormalisation de Gram-Schmidt` | licence 2 · master 2 (agrégation externe) | ✅ licence 2 · master 2 |
| définition | `Produit scalaire, espace euclidien` | licence 2 | · |
| théorème | `Projection orthogonale sur un sous-espace` | licence 2 | ✅ licence 2 |
| théorème | `Sous-espaces caractéristiques` | licence 2 | ✅ licence 2 |
| théorème | `Théorème de Cayley-Hamilton` | licence 2 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème spectral` | licence 2 · licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| définition | `Transposée d'une application linéaire` | licence 2 | · |
| théorème | `Trigonalisation` | licence 2 | ✅ licence 2 |
| théorème | `Une famille orthogonale de vecteurs non nuls est libre` | licence 2 | ✅ licence 2 |
| définition | `Valeur propre, vecteur propre, sous-espace propre` | licence 2 | · |

#### analyse

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Caractérisations de la convexité` | licence 2 | ✅ licence 2 |
| théorème | `Comparaison série-intégrale` | licence 2 | ✅ licence 2 |
| théorème | `Condition nécessaire de convergence` | licence 2 | ✅ licence 2 |
| théorème | `Continuité de la limite uniforme` | licence 2 | ✅ licence 2 |
| théorème | `Convergence absolue` | licence 2 | ✅ licence 2 |
| définition | `Convergence simple et convergence uniforme` | licence 2 | · |
| théorème | `Critère de comparaison` | licence 2 | ✅ licence 2 |
| théorème | `Critère spécial des séries alternées` | licence 2 | · |
| théorème | `Divergence de la série harmonique` | licence 2 | ✅ licence 2 |
| théorème | `Dérivation d'une série entière` | licence 2 | · |
| définition | `Dérivées partielles et différentielle` | licence 2 | · |
| définition | `Fonction convexe d'une variable réelle` | licence 2 | · |
| théorème | `Formule de Stirling` | licence 2 | · |
| théorème | `Interversion limite-intégrale (cas uniforme)` | licence 2 | · |
| théorème | `La convergence uniforme entraîne la convergence simple` | licence 2 | ✅ licence 2 |
| théorème | `Lemme d'Abel` | licence 2 | ✅ licence 2 |
| théorème | `Multiplicateurs de Lagrange` | licence 2 | · |
| théorème | `Points critiques et extrema` | licence 2 | ✅ licence 2 |
| théorème | `Règle de d'Alembert` | licence 2 | ✅ licence 2 |
| définition | `Série entière et rayon de convergence` | licence 2 | · |
| théorème | `Série géométrique` | licence 2 | ✅ licence 2 |
| définition | `Série numérique` | licence 2 | · |
| théorème | `Séries de Riemann` | licence 2 | ✅ licence 2 |
| théorème | `Théorème de Heine` | licence 2 · master 2 (agrégation externe) | ✅ licence 2 · master 2 |
| théorème | `Théorème de Schwarz` | licence 2 | · |
| théorème | `Unicité des coefficients d'une série entière` | licence 2 | ✅ licence 2 |
| théorème | `Équations différentielles linéaires du second ordre à coefficients constants` | licence 2 | ✅ licence 2 |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Lois discrètes usuelles` | licence 2 | · |
| théorème | `Variance d'une somme de variables indépendantes` | licence 2 | ✅ licence 2 |

#### topologie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Caractérisation séquentielle des fermés` | licence 2 | ✅ licence 2 |
| définition | `Compacité` | licence 2 | · |
| définition | `Norme et distance associée` | licence 2 | · |
| définition | `Ouverts, fermés, adhérence` | licence 2 | · |
| théorème | `Théorème de Borel-Lebesgue` | licence 2 | · |
| théorème | `Équivalence des normes en dimension finie` | licence 2 | · |

### Licence 3

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Action d'un groupe sur un ensemble` | licence 3 | · |
| théorème | `Critère d'Eisenstein` | licence 3 | ✅ licence 3 |
| définition | `Extension de corps et degré` | licence 3 | · |
| théorème | `Formule de Burnside` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Formule orbite-stabilisateur et équation aux classes` | licence 3 | ✅ licence 3 |
| théorème | `Groupe quotient et théorème d'isomorphisme` | licence 3 | ✅ licence 3 |
| théorème | `Injectivité d'un morphisme et noyau` | licence 3 | ✅ licence 3 |
| propriété | `Intersection de sous-groupes` | licence 3 | ✅ licence 3 |
| définition | `Morphisme de groupes` | licence 3 | · |
| théorème | `Multiplicativité du degré d'une extension` | licence 3 | ✅ licence 3 |
| définition | `Polynômes cyclotomiques` | licence 3 | · |
| théorème | `Structure des corps finis` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Structure des groupes abéliens finis` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Structure des groupes cycliques` | licence 3 | · |
| théorème | `Théorème chinois` | licence 3 | ✅ licence 3 |
| théorème | `Théorème de Cauchy pour les groupes` | licence 3 | · |
| théorème | `Théorème de Lagrange` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Théorème de Wantzel` | licence 3 | ✅ licence 3 |
| théorème | `Théorèmes de Sylow` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Tout groupe d'ordre premier est cyclique` | licence 3 | ✅ licence 3 |
| définition | `Élément algébrique, polynôme minimal` | licence 3 | · |

#### algèbre linéaire

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Conditionnement d'une matrice` | licence 3 | · |
| théorème | `Convergence des méthodes itératives` | licence 3 | ✅ licence 3 |
| théorème | `Décomposition de Dunford` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Décomposition en valeurs singulières` | licence 3 | ✅ licence 3 |
| théorème | `Décomposition LU et Cholesky` | licence 3 | ✅ licence 3 |
| définition | `Exponentielle d'une matrice` | licence 3 | · |
| théorème | `Méthode des moindres carrés` | licence 3 | ✅ licence 3 |
| définition | `Rayon spectral` | licence 3 | · |
| théorème | `Rayon spectral, normes et convergence des puissances` | licence 3 | ✅ licence 3 |
| théorème | `Réduction de Jordan` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de Gershgorin-Hadamard` | licence 3 | ✅ licence 3 |

#### analyse

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Inégalité de Jensen` | licence 3 | ✅ licence 3 |
| théorème | `Lemme de Grönwall` | licence 3 | ✅ licence 3 |
| théorème | `Méthode d'Euler explicite` | licence 3 | ✅ licence 3 |
| théorème | `Méthode de dichotomie` | licence 3 | ✅ licence 3 |
| théorème | `Méthodes des rectangles et des trapèzes` | licence 3 | ✅ licence 3 |
| théorème | `Séries de Fourier` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème d'inversion locale` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de Cauchy-Lipschitz` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Théorème de Dini` | licence 3 | ✅ licence 3 |
| théorème | `Théorème des fonctions implicites` | licence 3 | · |

#### analyse complexe

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Fonction holomorphe` | licence 3 | · |
| théorème | `Formule intégrale de Cauchy` | licence 3 | · |
| théorème | `Théorème de Cauchy` | licence 3 | · |
| théorème | `Théorème de Liouville` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Théorème des résidus` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Équations de Cauchy-Riemann` | licence 3 | ✅ licence 3 |

#### analyse fonctionnelle

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Base hilbertienne` | licence 3 | · |
| théorème | `Décomposition sur une base hilbertienne et égalité de Parseval` | licence 3 | ✅ licence 3 |
| définition | `Espace de Banach` | licence 3 | · |
| définition | `Espace de Hilbert` | licence 3 | · |
| théorème | `Inégalité de Bessel` | licence 3 | ✅ licence 3 |
| théorème | `Théorème de projection sur un convexe fermé` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Théorème de représentation de Riesz` | licence 3 | ✅ licence 3 |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Indicatrice d'Euler et théorème d'Euler` | licence 3 | · |
| théorème | `Théorème de Wilson` | licence 3 | · |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Application affine` | licence 3 | · |
| théorème | `Associativité du barycentre` | licence 3 | ✅ licence 3 |
| théorème | `Caractérisation des points cocycliques` | licence 3 | · |
| théorème | `Classification des isométries du plan` | licence 3 | ✅ licence 3 |
| définition | `Coniques du plan affine euclidien` | licence 3 | · |
| définition | `Espace affine et direction` | licence 3 | · |
| théorème | `Existence et unicité du barycentre` | licence 3 | ✅ licence 3 |
| théorème | `Groupe affine, homothéties et translations` | licence 3 | ✅ licence 3 |
| théorème | `Isométries d'un espace affine euclidien` | licence 3 | · |
| définition | `Partie convexe et enveloppe convexe` | licence 3 | · |
| définition | `Produit vectoriel et produit mixte` | licence 3 | · |
| définition | `Similitude` | licence 3 | · |
| définition | `Sous-espace affine` | licence 3 | · |
| théorème | `Théorème de l'angle inscrit` | licence 3 | ✅ licence 3 |
| théorème | `Écriture complexe des similitudes du plan` | licence 3 | ✅ licence 3 |

#### intégration

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Espaces L^p` | licence 3 | · |
| définition | `Intégrale de Lebesgue` | licence 3 | · |
| théorème | `Inégalité de Hölder` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Lemme de Fatou` | licence 3 | ✅ licence 3 |
| théorème | `Lemme de Riemann-Lebesgue` | licence 3 | ✅ licence 3 |
| définition | `Mesure` | licence 3 | · |
| théorème | `Mesure de Lebesgue` | licence 3 | · |
| théorème | `Théorème de convergence dominée` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |
| théorème | `Théorème de convergence monotone de Lebesgue` | licence 3 | · |
| théorème | `Théorème de Riesz-Fischer` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorèmes de Fubini et Tonelli` | licence 3 | · |
| définition | `Tribu et espace mesurable` | licence 3 | · |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Espace probabilisé` | licence 3 | · |
| définition | `Fonction caractéristique` | licence 3 | · |
| théorème | `Hiérarchie des modes de convergence` | licence 3 | ✅ licence 3 |
| théorème | `Inégalité de Markov` | licence 3 | ✅ licence 3 |
| théorème | `Loi forte des grands nombres` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| définition | `Modes de convergence` | licence 3 | · |
| théorème | `Méthode de Monte-Carlo` | licence 3 | · |
| théorème | `Théorème central limite` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de de Moivre-Laplace` | licence 3 | · |

#### topologie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Connexité et connexité par arcs` | licence 3 | · |
| définition | `Espace métrique complet` | licence 3 | · |
| théorème | `Théorème de Baire` | licence 3 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de Riesz sur la compacité de la boule` | licence 3 | ✅ licence 3 |
| théorème | `Théorème du point fixe de Banach` | licence 3 · master 2 (agrégation externe) | ✅ licence 3 · master 2 |

### Master 1

#### algèbre

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Correspondance de Galois` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| définition | `Extension galoisienne et groupe de Galois` | master 1 | · |
| définition | `Forme quadratique et forme polaire` | master 1 | · |
| théorème | `Lemme de Nakayama` | master 1 | · |
| théorème | `Loi d'inertie de Sylvester` | master 1 · master 2 (agrégation externe) | ✅ master 1 · master 2 |
| théorème | `Modules de type fini sur un anneau principal` | master 1 | · |
| théorème | `Orthogonalité des caractères` | master 1 | · |
| définition | `Représentation linéaire d'un groupe fini` | master 1 | · |
| théorème | `Résolubilité par radicaux` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de l'élément primitif` | master 1 | · |
| théorème | `Théorème de Maschke` | master 1 · master 2 (agrégation externe) | ✅ master 1 · master 2 |
| théorème | `Théorème de Wedderburn` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de Witt` | master 1 | · |

#### algèbre linéaire

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Décomposition polaire` | master 1 · master 2 (agrégation externe) | ✅ master 1 · master 2 |
| théorème | `Théorème de Perron-Frobenius` | master 1 · master 2 (agrégation externe) | ✅ master 2 |

#### analyse

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Convergence de la méthode de Newton` | master 1 · master 2 (agrégation externe) | ✅ master 1 · master 2 |
| théorème | `Formules de Green-Riemann et d'Ostrogradski` | master 1 | · |
| théorème | `Interpolation de Lagrange` | master 1 | ✅ master 1 |
| théorème | `Lemme de Morse` | master 1 | · |
| théorème | `Méthode de Laplace` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Principe du maximum pour les fonctions harmoniques` | master 1 | · |
| théorème | `Théorème d'Abel angulaire` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de comparaison de Sturm` | master 1 | · |
| théorème | `Théorème de Fejér` | master 1 · master 2 (agrégation externe) | ✅ master 2 |

#### analyse complexe

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Théorème de Montel` | master 1 | · |
| théorème | `Théorème de Runge` | master 1 · master 2 (agrégation externe) | ✅ master 2 |

#### analyse fonctionnelle

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Alternative de Fredholm` | master 1 | · |
| définition | `Opérateur compact` | master 1 | · |
| théorème | `Théorie de Riesz-Schauder` | master 1 | · |
| théorème | `Théorème d'Ascoli` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de Banach-Steinhaus` | master 1 · master 2 (agrégation externe) | ✅ master 1 · master 2 |
| théorème | `Théorème de Hahn-Banach` | master 1 | · |
| théorème | `Théorème de Korovkine` | master 1 | · |
| théorème | `Théorème de l'application ouverte` | master 1 | · |
| théorème | `Théorème de Lax-Milgram` | master 1 | ✅ master 1 |
| théorème | `Théorème de Stone-Weierstrass` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème du graphe fermé` | master 1 | ✅ master 1 |
| théorème | `Théorème spectral pour les opérateurs compacts autoadjoints` | master 1 · master 2 (agrégation externe) | ✅ master 2 |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Loi de réciprocité quadratique` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème des deux carrés` | master 1 · master 2 (agrégation externe) | ✅ master 1 · master 2 |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Forme différentielle et différentielle extérieure` | master 1 | · |
| définition | `Sous-variété différentielle` | master 1 | · |
| théorème | `Théorème de Stokes` | master 1 | · |

#### intégration

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Distribution` | master 1 | · |
| définition | `Espaces de Sobolev` | master 1 | · |
| théorème | `Formule sommatoire de Poisson` | master 1 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Les espaces de Sobolev sont des espaces de Hilbert` | master 1 | · |
| théorème | `Théorème de Plancherel` | master 1 | · |
| théorème | `Théorème de prolongement de Carathéodory` | master 1 | · |
| définition | `Transformée de Fourier` | master 1 | · |

#### logique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| axiome | `Axiome du choix et lemme de Zorn` | master 1 | · |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Chaîne de Markov` | master 1 | · |
| théorème | `Espérance conditionnelle` | master 1 | ✅ master 1 |
| théorème | `Inégalité de Cramér-Rao` | master 1 | · |
| théorème | `Lemme de Slutsky` | master 1 | · |
| théorème | `Lemmes de Borel-Cantelli` | master 1 | ✅ master 1 |
| théorème | `Loi du zéro-un de Kolmogorov` | master 1 | · |
| définition | `Martingale` | master 1 | · |
| théorème | `Théorème d'arrêt de Doob` | master 1 | · |
| théorème | `Théorème de convergence des martingales` | master 1 | · |
| théorème | `Théorème ergodique pour les chaînes de Markov` | master 1 | · |

#### topologie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Groupe fondamental` | master 1 | · |
| théorème | `Théorème de Krein-Milman` | master 1 | · |
| théorème | `Théorème du point fixe de Brouwer` | master 1 · master 2 (agrégation externe) | ✅ master 2 |

### Master 2

#### analyse complexe

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Surface de Riemann` | master 2 | · |
| théorème | `Théorème de représentation conforme de Riemann` | master 2 | · |
| théorème | `Théorèmes de Picard` | master 2 | · |

#### analyse fonctionnelle

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Théorème de Hille-Yosida` | master 2 | · |

#### arithmétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Entiers algébriques et anneau des entiers` | master 2 | · |
| définition | `Fonction zêta de Riemann` | master 2 | · |
| théorème | `Théorème de la progression arithmétique de Dirichlet` | master 2 | · |
| théorème | `Théorème des nombres premiers` | master 2 | · |

#### géométrie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Théorème de Hopf-Rinow` | master 2 | · |
| définition | `Variété riemannienne, connexion et courbure` | master 2 | · |

#### intégration

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Injections de Sobolev` | master 2 | · |

#### logique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Théorème de compacité en logique` | master 2 | · |
| théorème | `Théorème de complétude de Gödel` | master 2 | · |
| théorème | `Théorèmes d'incomplétude de Gödel` | master 2 | · |

#### probabilités

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Formule d'Itô` | master 2 | · |
| définition | `Mouvement brownien` | master 2 | · |

#### topologie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Homologie singulière` | master 2 | · |
| théorème | `Théorie des revêtements` | master 2 | · |

## **Physique**

### CE2

#### fluides

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Objets qui flottent et qui coulent` | CE2 · CM2 | · |

#### thermodynamique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Température et thermomètre` | CE2 | · |

#### électromagnétisme

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Aimant et matériaux attirés` | CE2 | · |

### CM1

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Ombre portée` | CM1 · CM2 | · |

#### électricité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Circuit électrique simple` | CM1 | · |
| définition | `Conducteur et isolant` | CM1 · CM2 | · |

### Cinquième

#### mécanique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Mouvement uniforme, accéléré, ralenti` | cinquième | · |
| définition | `Référentiel` | cinquième | · |
| définition | `Trajectoire` | cinquième | · |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Propagation rectiligne de la lumière` | cinquième | · |
| définition | `Source primaire et objet diffusant` | cinquième | · |

#### électricité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Circuit en série et en dérivation` | cinquième | · |
| propriété | `Court-circuit` | cinquième | · |
| loi | `Unicité de l'intensité en série` | cinquième | · |

#### énergie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Énergie` | cinquième | · |

### Quatrième

#### acoustique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Propagation du son` | quatrième | · |
| propriété | `Vitesse du son` | quatrième | · |

#### mécanique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Distinction de la masse et du poids` | quatrième | · |
| définition | `Interaction` | quatrième | · |
| loi | `Interaction gravitationnelle` | quatrième | · |
| modèle | `Modélisation d'une action par une force` | quatrième | · |
| loi | `Poids d'un corps` | quatrième | · |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Année-lumière` | quatrième | · |
| propriété | `Vitesse de la lumière` | quatrième | · |

#### électricité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Additivité des tensions en série` | quatrième | · |
| loi | `Loi des nœuds` | quatrième | · |
| loi | `Unicité de la tension en dérivation` | quatrième | · |

#### énergie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Conversion d'énergie` | quatrième | · |
| définition | `Énergie cinétique` | quatrième | · |
| définition | `Énergie de position` | quatrième | · |

### Troisième

#### acoustique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Fréquence et hauteur d'un son` | troisième | · |
| définition | `Niveau d'intensité sonore` | troisième | · |

#### mécanique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Équilibre sous deux forces` | troisième | · |

#### signaux

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Signal` | troisième | · |

#### électricité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Loi d'Ohm` | troisième · seconde | · |
| loi | `Puissance électrique` | troisième | · |
| loi | `Énergie électrique transférée` | troisième | · |

#### énergie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| principe | `Conservation de l'énergie` | troisième | · |
| définition | `Puissance` | troisième | · |
| définition | `Rendement d'un convertisseur` | troisième | · |

### Seconde

#### acoustique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Signal sonore périodique` | seconde | · |
| relation | `Vitesse de propagation d'un son` | seconde | · |
| définition | `Échelle logarithmique des décibels` | seconde | · |

#### interactions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Les quatre interactions fondamentales` | seconde | · |

#### mécanique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Effet d'une force non compensée` | seconde | · |
| loi | `Loi de la gravitation universelle` | seconde | · |
| relation | `Poids et champ de pesanteur` | seconde | ✅ seconde |
| principe | `Principe d'inertie` | seconde | · |
| propriété | `Variation du vecteur vitesse` | seconde | · |
| définition | `Vecteur vitesse moyenne` | seconde | · |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Dispersion de la lumière` | seconde | · |
| définition | `Grandissement` | seconde | · |
| définition | `Lentille mince convergente` | seconde | · |
| loi | `Loi de Snell-Descartes pour la réfraction` | seconde | · |
| définition | `Spectres d'émission et d'absorption` | seconde | · |

#### électricité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Capteur électrique` | seconde | · |

#### électrostatique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Loi de Coulomb` | seconde | · |

### Première

#### acoustique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Fondamentale et harmoniques` | première (enseignement scientifique) | · |
| définition | `Octave et quinte` | première (enseignement scientifique) | · |

#### astronomie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Fusion nucléaire au cœur du Soleil` | première (enseignement scientifique) | · |
| règle | `Mesure du rayon terrestre par Ératosthène` | première (enseignement scientifique) | · |
| propriété | `Origine des saisons` | première (enseignement scientifique) | · |
| modèle | `Origine stellaire des éléments chimiques` | première (enseignement scientifique) | · |
| définition | `Puissance solaire reçue par une planète` | première (enseignement scientifique) | · |

#### champs

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Champ d'une grandeur physique` | première | · |
| loi | `Champ de gravitation` | première | ✅ première |
| propriété | `Champ entre deux plaques parallèles` | première | · |
| définition | `Champ magnétique` | première | · |
| définition | `Champ électrostatique` | première | · |
| définition | `Lignes de champ` | première | · |
| principe | `Superposition des champs` | première | · |

#### climat

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Albédo` | première (enseignement scientifique) | · |
| loi | `Bilan radiatif de la Terre` | première (enseignement scientifique) | · |
| modèle | `Effet de serre` | première (enseignement scientifique) | · |

#### fluides

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Loi de Boyle-Mariotte` | première | · |
| loi | `Loi fondamentale de la statique des fluides` | première | ✅ première |
| définition | `Pression et force pressante` | première | · |

#### ondes

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Onde mécanique progressive` | première | · |
| relation | `Retard de propagation` | première | · |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Couleur perçue d'un objet` | première | · |
| loi | `Loi de Wien` | première (enseignement scientifique) | · |
| modèle | `Modèle ondulatoire de la lumière` | première | · |
| modèle | `Modèle particulaire de la lumière` | première | · |
| loi | `Relation de conjugaison des lentilles minces` | première | · |
| définition | `Spectre des ondes électromagnétiques` | première | · |

#### quantique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Quantification des niveaux d'énergie` | première | · |

#### signaux

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Choix de la fréquence d'échantillonnage` | première (enseignement scientifique) | · |
| modèle | `Numérisation d'un signal sonore` | première (enseignement scientifique) | · |

#### thermodynamique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Loi de Stefan-Boltzmann` | première (enseignement scientifique) | · |

#### électricité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Effet Joule` | première | ✅ première |
| modèle | `Modèle linéaire d'un générateur` | première | · |

#### énergie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Conservation de l'énergie mécanique` | première | ✅ première |
| propriété | `Dissipation en présence de frottements` | première | · |
| théorème | `Théorème de l'énergie cinétique` | première | · |
| définition | `Travail d'une force constante` | première | · |
| propriété | `Travail d'une force de frottement` | première | · |
| propriété | `Travail du poids` | première | ✅ première |
| définition | `Énergie cinétique d'un point matériel` | première | · |
| définition | `Énergie mécanique` | première | · |
| définition | `Énergie potentielle de pesanteur` | première | · |

### Terminale

#### climat

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Forçage radiatif` | terminale (enseignement scientifique) | · |
| modèle | `Modèles climatiques et scénarios` | terminale (enseignement scientifique) | · |
| modèle | `Rétroactions climatiques` | terminale (enseignement scientifique) | · |

#### fluides

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Conservation du débit volumique` | terminale | · |
| définition | `Débit volumique` | terminale | · |
| propriété | `Effet Venturi` | terminale | ✅ terminale |
| loi | `Relation de Bernoulli` | terminale | · |

#### mécanique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Accélération dans un mouvement circulaire uniforme` | terminale | · |
| propriété | `Amortissement d'un oscillateur` | terminale | · |
| propriété | `Chute libre` | terminale | ✅ terminale |
| principe | `Deuxième loi de Newton` | terminale | · |
| propriété | `Mouvement dans un champ de pesanteur uniforme` | terminale | ✅ terminale |
| propriété | `Mouvement dans un champ électrique uniforme` | terminale | ✅ terminale |
| modèle | `Oscillateur mécanique masse-ressort` | terminale | · |
| principe | `Principe des actions réciproques` | terminale | · |
| propriété | `Période propre d'un oscillateur masse-ressort` | terminale | ✅ terminale |
| propriété | `Période propre d'un pendule simple` | terminale | · |
| loi | `Troisième loi de Kepler` | terminale | ✅ terminale |
| définition | `Vecteur accélération` | terminale | · |
| définition | `Vecteur position` | terminale | · |
| définition | `Vecteur vitesse instantanée` | terminale | · |
| propriété | `Vitesse d'un satellite en orbite circulaire` | terminale | ✅ terminale |
| propriété | `Énergie d'un oscillateur` | terminale | · |

#### noyau

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Activité d'un échantillon radioactif` | terminale | · |
| relation | `Demi-vie d'un nucléide` | terminale | ✅ terminale |
| loi | `Décroissance radioactive` | terminale | · |
| définition | `Défaut de masse et énergie de liaison du noyau` | terminale | · |
| principe | `Équivalence entre masse et énergie` | terminale | · |

#### ondes

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Diffraction` | terminale | · |
| propriété | `Effet Doppler` | terminale | · |
| propriété | `Interférences de deux ondes` | terminale | · |
| définition | `Onde progressive périodique` | terminale | · |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Grossissement d'une lunette astronomique` | terminale | · |

#### thermodynamique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Bilan énergétique d'un système` | terminale | · |
| définition | `Flux thermique` | terminale | · |
| loi | `Loi de Newton du refroidissement` | terminale | · |
| définition | `Modes de transfert thermique` | terminale | · |
| principe | `Premier principe de la thermodynamique` | terminale | · |
| loi | `Résistance thermique d'une paroi` | terminale | · |
| loi | `Variation d'énergie interne d'un corps incompressible` | terminale | · |
| définition | `Énergie interne` | terminale | · |

#### électricité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Condensateur et capacité` | terminale · licence 1 | · |
| propriété | `Constante de temps d'un circuit RC` | terminale | · |
| règle | `Optimisation du transport de l'électricité` | terminale (enseignement scientifique) | · |
| loi | `Production d'électricité par induction` | terminale (enseignement scientifique) | · |
| propriété | `Énergie stockée dans un condensateur` | terminale · licence 1 | ✅ licence 1 |

#### énergie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Rendement d'une chaîne énergétique` | terminale (enseignement scientifique) | ✅ terminale |
| propriété | `Stockage de l'énergie électrique` | terminale (enseignement scientifique) | · |

### Licence 1

#### mécanique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Conservation du moment cinétique en force centrale` | licence 1 | ✅ licence 1 |
| modèle | `Discussion énergétique et portrait de phase` | licence 1 | · |
| définition | `Force conservative et énergie potentielle` | licence 1 | · |
| modèle | `Forces usuelles` | licence 1 | · |
| définition | `Moment cinétique d'un point matériel` | licence 1 | · |
| modèle | `Oscillateur harmonique amorti` | licence 1 · licence 1 (classe préparatoire PCSI) | ✅ licence 1 |
| relation | `Pulsation propre d'un oscillateur` | licence 1 | ✅ licence 1 |
| définition | `Quantité de mouvement` | licence 1 | · |
| définition | `Systèmes de coordonnées et bases mobiles` | licence 1 · licence 1 (classe préparatoire PCSI) | · |
| théorème | `Théorème du centre de masse` | licence 1 | ✅ licence 1 |
| théorème | `Théorème du moment cinétique` | licence 1 · licence 1 (classe préparatoire PCSI) | ✅ licence 1 |
| relation | `Vitesse et accélération en coordonnées polaires` | licence 1 | ✅ licence 1 |

#### ondes

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Analogie entre oscillateur mécanique et circuit électrique` | licence 1 | · |
| modèle | `Oscillations forcées en régime sinusoïdal` | licence 1 · licence 1 (classe préparatoire PCSI) | · |
| propriété | `Résonance d'amplitude` | licence 1 | ✅ licence 1 |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Association de systèmes optiques` | licence 1 | ✅ licence 1 |
| principe | `Chemin optique et principe de Fermat` | licence 1 | · |
| modèle | `Conditions de Gauss et stigmatisme approché` | licence 1 | · |

#### thermodynamique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Enthalpie et transformations isobares` | licence 1 | ✅ licence 1 |
| modèle | `Interprétation microscopique de la température` | licence 1 | · |
| principe | `Premier principe de la thermodynamique en licence` | licence 1 | · |
| relation | `Travail des forces de pression` | licence 1 | ✅ licence 1 |
| définition | `Variables d'état et équilibre thermodynamique` | licence 1 | · |
| loi | `Équation d'état du gaz parfait` | licence 1 · licence 1 (classe préparatoire PCSI) | · |

#### électromagnétisme

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Calcul d'un champ par le théorème de Gauss` | licence 1 | · |
| théorème | `Caractère conservatif du champ électrostatique` | licence 1 | ✅ licence 1 |
| loi | `Champ créé par une charge ponctuelle` | licence 1 · licence 1 (classe préparatoire PCSI) | ✅ licence 1 |
| propriété | `Conducteur en équilibre électrostatique` | licence 1 | ✅ licence 1 |
| définition | `Distributions continues de charge` | licence 1 | · |
| loi | `Force de Laplace` | licence 1 | ✅ licence 1 |
| loi | `Force de Lorentz` | licence 1 · licence 1 (classe préparatoire PCSI) | · |
| théorème | `Potentiel électrostatique` | licence 1 | ✅ licence 1 |
| principe | `Superposition des champs électrostatiques` | licence 1 | · |
| théorème | `Théorème de Gauss pour le champ électrostatique` | licence 1 · licence 1 (classe préparatoire PCSI) | ✅ licence 1 |
| définition | `Énergie potentielle électrostatique` | licence 1 | · |

### Licence 2

#### fluides

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Description eulérienne d'un écoulement` | licence 2 | · |
| loi | `Force de traînée de Stokes` | licence 2 | ✅ licence 2 |
| relation | `Nombre de Reynolds` | licence 2 | ✅ licence 2 |
| théorème | `Poussée d'Archimède` | licence 2 | ✅ licence 2 |
| définition | `Viscosité d'un fluide` | licence 2 | · |

#### mécanique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Chocs élastiques et inélastiques` | licence 2 | · |
| loi | `Composition des vitesses et des accélérations` | licence 2 | · |
| modèle | `Forces d'inertie` | licence 2 · licence 2 (classe préparatoire PC) | ✅ licence 2 |
| définition | `Moment d'inertie` | licence 2 | · |
| théorème | `Mouvement à force centrale newtonienne` | licence 2 | ✅ licence 2 |
| propriété | `Pesanteur apparente et effets terrestres` | licence 2 | · |
| théorème | `Rotation autour d'un axe fixe` | licence 2 | ✅ licence 2 |
| théorème | `Réduction du problème à deux corps` | licence 2 | ✅ licence 2 |
| propriété | `Référentiel barycentrique` | licence 2 | ✅ licence 2 |
| définition | `Solide indéformable et champ des vitesses` | licence 2 | · |
| théorème | `Théorème de Huygens` | licence 2 | ✅ licence 2 |
| théorème | `Théorèmes de König` | licence 2 | ✅ licence 2 |
| relation | `Énergie cinétique d'un solide en rotation` | licence 2 | ✅ licence 2 |

#### ondes

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Approximation acoustique` | licence 2 | · |
| loi | `Célérité du son dans un gaz parfait` | licence 2 | · |
| théorème | `Décomposition spectrale d'un signal périodique` | licence 2 | · |
| loi | `Effet Doppler longitudinal` | licence 2 | ✅ licence 2 |
| relation | `Impédance acoustique` | licence 2 | ✅ licence 2 |
| définition | `Intensité acoustique et niveau sonore en licence` | licence 2 | · |
| théorème | `Interférences à deux ondes cohérentes` | licence 2 | ✅ licence 2 |
| définition | `Onde progressive harmonique` | licence 2 | · |
| théorème | `Ondes stationnaires et modes propres d'une corde` | licence 2 | ✅ licence 2 |
| propriété | `Oscillateurs couplés et battements` | licence 2 | · |
| définition | `Équation de d'Alembert` | licence 2 | · |
| théorème | `Équation de la corde vibrante` | licence 2 | ✅ licence 2 |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Formule de Fresnel des interférences à deux ondes` | licence 2 · licence 2 (classe préparatoire PC) | ✅ licence 2 |
| propriété | `L'optique géométrique comme limite des courtes longueurs d'onde` | licence 2 | · |
| théorème | `Trous d'Young et interfrange` | licence 2 | ✅ licence 2 |
| définition | `Éclairement` | licence 2 | · |

#### quantique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Effet photoélectrique` | licence 2 · licence 2 (classe préparatoire PC) | · |
| loi | `Formule de Rydberg` | licence 2 | · |
| loi | `Relation de de Broglie` | licence 2 | · |

#### thermodynamique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| relation | `Capacités thermiques et relation de Mayer` | licence 2 | ✅ licence 2 |
| loi | `Changement d'état d'un corps pur` | licence 2 | · |
| modèle | `Gaz réels et équation de Van der Waals` | licence 2 | · |
| relation | `Identités thermodynamiques` | licence 2 | ✅ licence 2 |
| loi | `Loi de Fourier` | licence 2 | · |
| principe | `Second principe de la thermodynamique` | licence 2 · licence 2 (classe préparatoire PC) | · |
| théorème | `Théorème de Carnot` | licence 2 | ✅ licence 2 |
| définition | `Énergie libre et enthalpie libre` | licence 2 | · |

#### électromagnétisme

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Approximation des régimes quasi stationnaires` | licence 2 | · |
| propriété | `Champs magnétiques de référence` | licence 2 | · |
| loi | `Conservation du flux magnétique` | licence 2 | · |
| propriété | `Courants de Foucault` | licence 2 | · |
| propriété | `Densité d'énergie électrique` | licence 2 | · |
| définition | `Densité volumique d'énergie` | licence 2 | · |
| modèle | `Dipôle magnétique` | licence 2 | · |
| modèle | `Dipôle électrostatique` | licence 2 | ✅ licence 2 |
| définition | `Inductance propre` | licence 2 | · |
| loi | `Loi de Biot et Savart` | licence 2 | · |
| loi | `Loi de Faraday` | licence 2 · licence 2 (classe préparatoire PC) | · |
| règle | `Loi de Lenz` | licence 2 | · |
| théorème | `Théorème d'Ampère` | licence 2 · licence 2 (classe préparatoire PC) | · |
| relation | `Énergie magnétique d'une bobine` | licence 2 | ✅ licence 2 |
| loi | `Équations de Poisson et de Laplace` | licence 2 | ✅ licence 2 |

### Licence 3

#### fluides

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Loi de Poiseuille` | licence 3 | · |
| loi | `Tension superficielle et loi de Laplace` | licence 3 | · |
| modèle | `Turbulence` | licence 3 | · |
| loi | `Équation de Navier-Stokes` | licence 3 | · |

#### mécanique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Coordonnée cyclique et grandeur conservée` | licence 3 | ✅ licence 3 |
| définition | `Coordonnées généralisées et degrés de liberté` | licence 3 | · |
| définition | `Hamiltonien et équations canoniques` | licence 3 | · |
| définition | `Lagrangien d'un système` | licence 3 | · |
| règle | `Petites oscillations et modes propres` | licence 3 | · |
| principe | `Principe de moindre action` | licence 3 | · |
| théorème | `Équations de Lagrange` | licence 3 | ✅ licence 3 |

#### ondes

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Absorption exponentielle d'une onde` | licence 3 | · |
| modèle | `Guide d'onde et fréquence de coupure` | licence 3 | · |
| propriété | `Réflexion et transmission à une interface` | licence 3 | · |
| définition | `Vitesse de phase et vitesse de groupe` | licence 3 | · |
| propriété | `Étalement d'un paquet d'ondes en milieu dispersif` | licence 3 | · |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Biréfringence` | licence 3 | · |
| propriété | `Cohérence spatiale et contraste` | licence 3 | · |
| propriété | `Cohérence temporelle` | licence 3 | · |
| théorème | `Diffraction à l'infini par une fente` | licence 3 | ✅ licence 3 |
| modèle | `Interféromètre de Michelson` | licence 3 | · |
| loi | `Loi de Malus` | licence 3 | ✅ licence 3 |
| propriété | `Pouvoir de résolution et critère de Rayleigh` | licence 3 | · |
| principe | `Principe de Huygens-Fresnel` | licence 3 | · |
| théorème | `Réseau de diffraction` | licence 3 | · |

#### physique statistique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Distribution des vitesses de Maxwell` | licence 3 | ✅ licence 3 |
| théorème | `Facteur de Boltzmann` | licence 3 | ✅ licence 3 |
| théorème | `Fonction de partition` | licence 3 | · |
| modèle | `Gaz de fermions dégénéré et énergie de Fermi` | licence 3 | · |
| propriété | `Gel des degrés de liberté` | licence 3 | · |
| principe | `Indiscernabilité et statistiques quantiques` | licence 3 | · |
| définition | `Libre parcours moyen` | licence 3 | · |
| loi | `Loi de Planck du rayonnement` | licence 3 | · |
| définition | `Micro-états et macro-états` | licence 3 | · |
| théorème | `Origine cinétique de la pression` | licence 3 | ✅ licence 3 |
| propriété | `Origine statistique de l'irréversibilité` | licence 3 | · |
| postulat | `Postulat fondamental de la physique statistique` | licence 3 | · |
| théorème | `Température statistique` | licence 3 | ✅ licence 3 |
| théorème | `Théorème d'équipartition de l'énergie` | licence 3 | ✅ licence 3 |
| principe | `Troisième principe de la thermodynamique` | licence 3 | ✅ licence 3 |

#### quantique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Atome d'hydrogène` | licence 3 | ✅ licence 3 |
| définition | `Commutateur et observables compatibles` | licence 3 | · |
| propriété | `Effet tunnel` | licence 3 | · |
| postulat | `Fonction d'onde et interprétation probabiliste` | licence 3 | · |
| théorème | `Inégalité de Heisenberg` | licence 3 | ✅ licence 3 |
| propriété | `Marche de potentiel et réflexion quantique` | licence 3 | · |
| propriété | `Normalisation de la fonction d'onde` | licence 3 | · |
| postulat | `Observables et opérateurs hermitiens` | licence 3 | · |
| théorème | `Oscillateur harmonique quantique` | licence 3 | · |
| modèle | `Particule libre et paquet d'ondes` | licence 3 | · |
| principe | `Principe d'exclusion de Pauli` | licence 3 | · |
| théorème | `Puits de potentiel infini` | licence 3 | ✅ licence 3 |
| théorème | `Quantification du moment cinétique` | licence 3 | · |
| postulat | `Réduction du paquet d'ondes` | licence 3 | · |
| postulat | `Résultats possibles d'une mesure` | licence 3 | · |
| postulat | `Spin` | licence 3 | · |
| théorème | `Valeur moyenne d'une observable` | licence 3 | ✅ licence 3 |
| postulat | `Équation de Schrödinger` | licence 3 | · |
| théorème | `États stationnaires` | licence 3 | ✅ licence 3 |

#### relativité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Composition relativiste des vitesses` | licence 3 | · |
| propriété | `Contraction des longueurs` | licence 3 | ✅ licence 3 |
| propriété | `Dilatation des durées` | licence 3 | ✅ licence 3 |
| loi | `Effet Doppler relativiste` | licence 3 | · |
| propriété | `Limite newtonienne de la relativité restreinte` | licence 3 | ✅ licence 3 |
| postulat | `Postulats de la relativité restreinte` | licence 3 | · |
| définition | `Quantité de mouvement relativiste` | licence 3 | · |
| théorème | `Relation énergie-impulsion` | licence 3 | ✅ licence 3 |
| propriété | `Relativité de la simultanéité` | licence 3 | · |
| théorème | `Transformation de Lorentz` | licence 3 | ✅ licence 3 |

#### thermodynamique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Formule de Boltzmann` | licence 3 | · |
| théorème | `Équation de la diffusion thermique` | licence 3 | ✅ licence 3 |

#### électromagnétisme

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Bilan local d'énergie électromagnétique` | licence 3 | ✅ licence 3 |
| propriété | `Effet de peau` | licence 3 | · |
| modèle | `Onde plane progressive monochromatique` | licence 3 | · |
| modèle | `Polarisation d'un milieu diélectrique` | licence 3 | · |
| définition | `Polarisation d'une onde électromagnétique` | licence 3 | · |
| définition | `Potentiels scalaire et vecteur` | licence 3 | · |
| règle | `Relations de passage` | licence 3 | · |
| propriété | `Réflexion sur un conducteur parfait` | licence 3 | · |
| propriété | `Structure de l'onde électromagnétique plane` | licence 3 | ✅ licence 3 |
| définition | `Vecteur de Poynting` | licence 3 | · |
| loi | `Équation de conservation de la charge` | licence 3 | · |
| loi | `Équation de Maxwell-Ampère` | licence 3 | ✅ licence 3 |
| loi | `Équation de Maxwell-Faraday` | licence 3 | · |
| loi | `Équation de Maxwell-Gauss` | licence 3 | ✅ licence 3 |
| loi | `Équation de Maxwell-Thomson` | licence 3 | · |
| théorème | `Équation de propagation dans le vide` | licence 3 | ✅ licence 3 |

### Master 1

#### matière condensée

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Conductivité de Drude et temps de relaxation` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Domaines de Weiss et cycle d'hystérésis` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Effet Hall et signe des porteurs` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Effet Meissner et diamagnétisme parfait` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| loi | `Ferromagnétisme et loi de Curie-Weiss` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Jonction p-n et redressement` | master 1 · master 1 (agrégation externe) | · |
| loi | `Loi d'action de masse dans un semi-conducteur` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Masse effective et dynamique des porteurs` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Modèle de Debye de la capacité thermique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Modèle des liaisons fortes` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| loi | `Paramagnétisme et loi de Curie` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Phonons et relation de dispersion du réseau` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| définition | `Réseau réciproque et zones de Brillouin` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Structure de bandes et ouverture d'une bande interdite` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Surface de Fermi et caractère métallique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Théorème de Bloch pour un potentiel périodique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### mécanique analytique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Théorème de Noether` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### noyau

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Désintégration bêta et interaction faible` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Fission induite et réaction en chaîne contrôlée` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Fusion thermonucléaire et critère de Lawson` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Interaction forte résiduelle et saturation nucléaire` | master 1 · master 1 (agrégation externe) | · |
| modèle | `Modèle de la goutte liquide et formule de Bethe-Weizsäcker` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Modèle en couches et nombres magiques` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Radioactivité alpha et loi de Geiger-Nuttall` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| définition | `Sections efficaces de réaction nucléaire` | master 1 · master 1 (agrégation externe) | · |

#### optique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Cavité laser et modes longitudinaux` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Coefficients d'Einstein et inversion de population` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### physique statistique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Classification des transitions de phase` | master 1 · master 1 (agrégation externe) | · |
| théorème | `Condensation de Bose-Einstein` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Densité d'états et dénombrement dans l'espace des phases` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Distribution de Bose-Einstein` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Distribution de Fermi-Dirac` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| définition | `Ensemble grand canonique et potentiel chimique` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Gaz de photons et pression de radiation` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Modèle d'Ising traité en champ moyen` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Relation d'Einstein entre diffusion et mobilité` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Équation de Langevin et mouvement brownien` | master 1 · master 1 (agrégation externe) | · |
| modèle | `Équation maîtresse et bilan détaillé` | master 1 · master 1 (agrégation externe) | · |
| théorème | `Équivalence des ensembles statistiques` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### quantique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Algèbre du moment cinétique et spectre commun` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Composition de deux moments cinétiques` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Couplage spin-orbite et structure fine` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| définition | `Déterminant de Slater` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Effet Zeeman` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| définition | `Formalisme de Dirac et notation des kets` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Intégrale d'échange et règle de Hund` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Largeur naturelle d'une raie et temps de vie` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Méthode variationnelle` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| définition | `Opérateur densité et mélange statistique` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Opérateurs de création et d'annihilation de l'oscillateur` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Perturbations stationnaires d'un niveau dégénéré` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Perturbations stationnaires d'un niveau non dégénéré` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Points de vue de Schrödinger et de Heisenberg` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| postulat | `Postulat de symétrisation` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Relation de fermeture et changement de représentation` | master 1 | ✅ master 1 |
| théorème | `Règle d'or de Fermi` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Règles de sélection dipolaires électriques` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Symétries continues et générateurs en mécanique quantique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Théorème d'Ehrenfest` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Trou de Fermi et énergie d'échange` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Évolution unitaire et opérateur d'évolution` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### relativité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Quadrivecteurs et invariants relativistes` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### électromagnétisme

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Diffusion Thomson et diffusion Rayleigh` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| loi | `Formule de Larmor` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Jauge de Lorenz et équations des potentiels` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| théorème | `Potentiels retardés` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Rayonnement dipolaire électrique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Tenseur du champ électromagnétique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Transformation des champs par changement de référentiel` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

### Master 2

#### astronomie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Limite de Chandrasekhar` | master 2 · master 2 (agrégation externe) | ✅ master 2 |

#### cosmologie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Fond diffus cosmologique et découplage` | master 2 · master 2 (agrégation externe) | · |
| loi | `Loi de Hubble-Lemaître` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| modèle | `Métrique de Robertson-Walker et facteur d'échelle` | master 2 · master 2 (agrégation externe) | · |
| loi | `Équations de Friedmann` | master 2 · master 2 (agrégation externe) | ✅ master 2 |

#### matière condensée

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Effet Josephson` | master 2 · master 2 (agrégation externe) | · |
| modèle | `Paires de Cooper et théorie BCS` | master 2 · master 2 (agrégation externe) | · |

#### particules

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Brisure spontanée de symétrie et acquisition de masse` | master 2 · master 2 (agrégation externe) | · |
| définition | `Classification des particules élémentaires` | master 2 · master 2 (agrégation externe) | · |
| règle | `Diagrammes de Feynman et calcul des amplitudes` | master 2 · master 2 (agrégation externe) | · |
| principe | `Invariance de jauge et origine des interactions` | master 2 · master 2 (agrégation externe) | · |
| propriété | `Symétries discrètes et leur violation` | master 2 · master 2 (agrégation externe) | · |

#### physique statistique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Entropie de von Neumann` | master 2 · master 2 (agrégation externe) | · |
| propriété | `Exposants critiques et universalité` | master 2 · master 2 (agrégation externe) | · |
| théorème | `Groupe de renormalisation et invariance d'échelle` | master 2 · master 2 (agrégation externe) | · |
| règle | `Simulation de Monte-Carlo et algorithme de Metropolis` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème de fluctuation-dissipation` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Théorème H de Boltzmann` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| modèle | `Équation de Boltzmann et coefficients de transport` | master 2 · master 2 (agrégation externe) | · |
| modèle | `Équation de Fokker-Planck` | master 2 · master 2 (agrégation externe) | ✅ master 2 |

#### quantique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Amplitude de diffusion et section efficace` | master 2 · master 2 (agrégation externe) | · |
| théorème | `Approximation de Born` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| théorème | `Approximation semi-classique et quantification de Bohr-Sommerfeld` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| propriété | `Décohérence par couplage à l'environnement` | master 2 · master 2 (agrégation externe) | · |
| théorème | `Développement en ondes partielles et déphasages` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| définition | `Intrication quantique` | master 2 · master 2 (agrégation externe) | · |
| théorème | `Inégalités de Bell` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| définition | `Seconde quantification et opérateurs de création` | master 2 · master 2 (agrégation externe) | · |
| théorème | `Théorème optique` | master 2 · master 2 (agrégation externe) | ✅ master 2 |

#### relativité

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Décalage gravitationnel vers le rouge` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| définition | `Espace-temps courbe et tenseur métrique` | master 2 · master 2 (agrégation externe) | · |
| théorème | `Géodésiques et mouvement en chute libre` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| propriété | `Limite newtonienne de la gravitation relativiste` | master 2 · master 2 (agrégation externe) | ✅ master 2 |
| propriété | `Métrique de Schwarzschild et rayon gravitationnel` | master 2 · master 2 (agrégation externe) | · |
| propriété | `Ondes gravitationnelles` | master 2 · master 2 (agrégation externe) | · |
| principe | `Principe d'équivalence` | master 2 · master 2 (agrégation externe) | · |
| propriété | `Tests classiques de la relativité générale` | master 2 · master 2 (agrégation externe) | · |
| loi | `Équations du champ d'Einstein` | master 2 · master 2 (agrégation externe) | · |

#### électromagnétisme

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Rayonnement synchrotron` | master 2 · master 2 (agrégation externe) | · |

## **Chimie**

### CE2

#### constitution de la matière

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `États de l'eau` | CE2 | · |

### CM1

#### constitution de la matière

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Fusion et solidification de l'eau` | CM1 | · |
| propriété | `Évaporation et condensation` | CM1 · CM2 | · |

### Cinquième

#### changements d'état

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Conservation de la masse lors d'un changement d'état` | cinquième | · |
| propriété | `Palier de changement d'état` | cinquième | · |

#### constitution de la matière

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Corps pur et mélange` | cinquième | · |
| définition | `Espèce chimique` | cinquième | · |
| définition | `Masse volumique` | cinquième | · |
| modèle | `Modèle particulaire` | cinquième | · |
| définition | `Solution, solvant, soluté` | cinquième | · |
| définition | `États de la matière` | cinquième | · |

#### solutions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Conservation de la masse lors d'une dissolution` | cinquième | · |
| définition | `Solubilité et saturation` | cinquième | · |

### Quatrième

#### solutions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Concentration en masse` | quatrième · seconde | · |

#### tests d'identification

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Test de l'eau` | quatrième | · |
| règle | `Test du dihydrogène` | quatrième | · |
| règle | `Test du dioxyde de carbone` | quatrième | · |
| règle | `Test du dioxygène` | quatrième | · |

#### transformations chimiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Combustion` | quatrième | · |
| propriété | `Combustion complète et incomplète` | quatrième | · |
| loi | `Conservation de la masse au cours d'une réaction` | quatrième | · |
| loi | `Conservation des atomes` | quatrième | · |
| définition | `Transformation chimique` | quatrième | · |
| définition | `Équation de réaction` | quatrième | · |

### Troisième

#### acides et bases

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Effet d'une dilution sur le pH` | troisième | · |
| propriété | `Ions responsables de l'acidité et de la basicité` | troisième | · |
| définition | `pH d'une solution` | troisième | · |

#### constitution de la matière

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Atome` | troisième | · |
| définition | `Ion` | troisième | · |
| définition | `Molécule` | troisième | · |
| définition | `Élément chimique` | troisième | · |

#### tests d'identification

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Tests des ions courants` | troisième | · |

#### transformations chimiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Action d'un acide sur un métal` | troisième | · |

### Seconde

#### constitution de la matière

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Charge des ions monoatomiques` | seconde | · |
| règle | `Classification périodique` | seconde | · |
| définition | `Composition massique d'un mélange` | seconde | · |
| règle | `Configuration électronique` | seconde | · |
| définition | `Entité chimique` | seconde | · |
| définition | `Isotopes` | seconde | · |
| définition | `Mole et constante d'Avogadro` | seconde | · |
| propriété | `Neutralité électrique de l'atome` | seconde | · |
| définition | `Noyau, numéro atomique et nombre de masse` | seconde | · |
| règle | `Stabilité des gaz nobles` | seconde | · |
| définition | `Électrons de valence` | seconde | · |

#### liaison chimique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Liaison covalente et schéma de Lewis` | seconde | · |

#### noyau

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Conservation lors d'une transformation nucléaire` | seconde | · |

#### quantité de matière

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Masse molaire` | seconde | · |

#### solutions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Concentration en quantité de matière` | seconde | · |
| loi | `Conservation de la quantité de matière lors d'une dilution` | seconde | ✅ seconde |
| règle | `Dosage par étalonnage` | seconde | · |
| relation | `Lien entre les deux concentrations` | seconde | ✅ seconde |

#### transformations chimiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Ajustement d'une équation de réaction` | seconde | · |
| définition | `Réactif limitant` | seconde | · |
| modèle | `Tableau d'avancement` | seconde | · |
| définition | `Transformations physique, chimique et nucléaire` | seconde | · |

### Première

#### astronomie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Abondance des éléments chimiques` | première (enseignement scientifique) | · |

#### chimie organique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Groupe caractéristique et famille fonctionnelle` | première | · |
| définition | `Isomérie de constitution` | première | · |
| définition | `Modification de chaîne et de groupe caractéristique` | première | · |
| règle | `Nomenclature en chimie organique` | première | · |
| règle | `Représentation par flèche courbe` | première | · |
| modèle | `Sites donneurs et accepteurs de doublet` | première | · |
| définition | `Squelette carboné` | première | · |

#### cohésion

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Cohésion des solides` | première | · |
| modèle | `Interactions de Van der Waals` | première | · |
| modèle | `Liaison hydrogène` | première | · |

#### cristaux

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Compacité d'un cristal` | première | · |
| définition | `Maille et réseau cristallin` | première | · |
| définition | `Mailles cubiques simple et à faces centrées` | première | · |
| relation | `Masse volumique d'un cristal` | première | ✅ première |
| définition | `Types de cristaux` | première | · |

#### liaison chimique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Géométrie d'une entité par répulsion des doublets` | première | · |
| propriété | `Liaison covalente polarisée` | première | · |
| propriété | `Polarité d'une molécule` | première | · |
| définition | `Électronégativité` | première | · |

#### oxydoréduction

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Couple oxydant-réducteur` | première | · |
| règle | `Demi-équation électronique` | première | · |
| définition | `Oxydant et réducteur` | première | · |
| règle | `Équation d'une réaction d'oxydoréduction` | première | · |

#### solutions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Absorbance d'une solution` | première | · |
| règle | `Choix d'un solvant selon la polarité` | première | · |
| loi | `Loi de Beer-Lambert` | première | · |
| loi | `Loi de Kohlrausch` | première | · |
| règle | `Équation de dissolution` | première | · |
| modèle | `Étapes de la dissolution d'un solide ionique` | première | · |

#### techniques expérimentales

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Extraction liquide-liquide` | première | · |
| relation | `Relation à l'équivalence` | première | ✅ première |
| définition | `Titrage` | première | · |
| définition | `Équivalence d'un titrage` | première | · |

#### transformations chimiques

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Avancement final et avancement maximal` | première | · |
| définition | `Taux d'avancement final` | première | · |
| propriété | `Transformation totale et non totale` | première | · |

#### énergie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Pouvoir calorifique massique` | première | · |
| définition | `Énergie de liaison` | première | · |
| définition | `Énergie molaire de réaction` | première | · |

### Terminale

#### acides et bases

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Acide et base au sens de Brønsted` | terminale | · |
| définition | `Constante d'acidité` | terminale | · |
| règle | `Diagramme de prédominance` | terminale | ✅ terminale |
| définition | `Définition logarithmique du pH` | terminale | · |
| règle | `Indicateur coloré acido-basique` | terminale | · |
| loi | `Produit ionique de l'eau` | terminale | · |

#### chimie organique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Stratégie de synthèse organique` | terminale | · |
| définition | `Sélectivité d'une transformation` | terminale | · |

#### cinétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Catalyse` | terminale | · |
| propriété | `Facteurs cinétiques` | terminale | · |
| définition | `Intermédiaire réactionnel` | terminale | · |
| loi | `Loi de vitesse d'ordre un` | terminale | · |
| modèle | `Mécanisme réactionnel et acte élémentaire` | terminale | · |
| relation | `Temps caractéristique et demi-réaction à l'ordre un` | terminale | ✅ terminale |
| définition | `Temps de demi-réaction` | terminale | · |
| définition | `Vitesse volumique de réaction` | terminale | · |

#### climat

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Empreinte carbone` | terminale (enseignement scientifique) | · |

#### oxydoréduction

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Pile électrochimique` | terminale | · |

#### spectroscopie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Déplacement chimique` | terminale | · |
| règle | `Intégration et multiplicité d'un signal` | terminale | · |
| modèle | `Principe de la résonance magnétique nucléaire` | terminale | · |
| règle | `Spectroscopie infrarouge` | terminale | · |

#### synthèse

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Optimisation d'une étape de synthèse` | terminale | · |
| règle | `Principes de la chimie durable` | terminale | · |
| définition | `Rendement d'une synthèse` | terminale | · |

#### techniques expérimentales

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Titrage suivi par pH-métrie` | terminale | · |

#### électrochimie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Capacité électrique d'une pile` | terminale | · |
| règle | `Polarité et électrodes d'une pile` | terminale | · |
| relation | `Quantité d'électricité débitée` | terminale | · |
| définition | `Électrolyse` | terminale | · |

#### équilibres

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Constante d'équilibre` | terminale | · |
| règle | `Critère d'évolution spontanée` | terminale | · |
| définition | `Quotient de réaction` | terminale | · |
| propriété | `Taux d'avancement et constante d'équilibre` | terminale | · |

### Licence 1

#### acides et bases

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Allure des courbes de titrage acido-basique` | licence 1 | ✅ licence 1 |
| règle | `Calcul du pH d'une solution d'acide` | licence 1 | ✅ licence 1 |
| propriété | `Force d'un acide et nivellement par le solvant` | licence 1 · licence 1 (classe préparatoire PCSI) | · |
| relation | `Relation de Henderson-Hasselbalch` | licence 1 | ✅ licence 1 |
| propriété | `Solution tampon` | licence 1 | ✅ licence 1 |

#### atomistique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Forme des orbitales s, p et d` | licence 1 | · |
| définition | `Orbitales atomiques et nombres quantiques` | licence 1 · licence 1 (classe préparatoire PCSI) | · |
| règle | `Règles de remplissage des orbitales` | licence 1 | · |
| définition | `Échelles d'électronégativité` | licence 1 | · |
| propriété | `Énergie d'ionisation` | licence 1 | · |
| propriété | `Évolution des rayons atomiques et ioniques` | licence 1 | · |

#### chimie organique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Chiralité et carbone asymétrique` | licence 1 · licence 1 (classe préparatoire PCSI) | · |
| règle | `Configuration absolue et règles de priorité` | licence 1 | · |
| modèle | `Effets inductif et mésomère` | licence 1 | · |
| définition | `Nucléophiles et électrophiles` | licence 1 | · |
| règle | `Représentations spatiales des molécules` | licence 1 | · |
| définition | `Énantiomères et diastéréoisomères` | licence 1 | · |

#### cinétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Dégénérescence de l'ordre` | licence 1 | · |
| théorème | `Intégration des lois de vitesse simples` | licence 1 | ✅ licence 1 |
| loi | `Loi d'Arrhenius` | licence 1 | · |
| définition | `Ordre d'une réaction` | licence 1 · licence 1 (classe préparatoire PCSI) | · |
| propriété | `Temps de demi-réaction selon l'ordre` | licence 1 | ✅ licence 1 |

#### cristaux

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Coordinence et sites interstitiels` | licence 1 | ✅ licence 1 |
| propriété | `Empilements compacts de sphères` | licence 1 | ✅ licence 1 |
| définition | `Réseaux de Bravais et systèmes cristallins` | licence 1 · licence 1 (classe préparatoire PCSI) | · |

#### liaison chimique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Moment dipolaire d'une molécule` | licence 1 | · |

#### précipitation

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Produit de solubilité` | licence 1 | · |
| propriété | `Solubilité et effet d'ion commun` | licence 1 | ✅ licence 1 |

#### électrochimie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Potentiel d'électrode et électrode standard à hydrogène` | licence 1 · licence 1 (classe préparatoire PCSI) | · |

### Licence 2

#### acides et bases

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Polyacides et diagrammes de distribution` | licence 2 | · |

#### atomistique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Approximation orbitalaire et effet d'écran` | licence 2 | · |

#### chimie analytique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Chaîne de mesure et étalonnage` | licence 2 | · |
| règle | `Incertitude de mesure et propagation` | licence 2 | · |
| modèle | `Principe de la chromatographie` | licence 2 | · |
| définition | `Sensibilité, limite de détection et de quantification` | licence 2 | · |

#### chimie organique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Activité optique et excès énantiomérique` | licence 2 | · |
| modèle | `Addition nucléophile sur un groupe carbonyle` | licence 2 | · |
| modèle | `Addition électrophile sur une double liaison` | licence 2 | ✅ licence 2 |
| propriété | `Analyse conformationnelle` | licence 2 | · |
| modèle | `Carbocations, carbanions et radicaux` | licence 2 | · |
| modèle | `Organomagnésiens mixtes` | licence 2 | · |
| règle | `Oxydation et réduction en chimie organique` | licence 2 | · |
| modèle | `Substitution nucléophile monomoléculaire et bimoléculaire` | licence 2 | ✅ licence 2 |
| modèle | `Éliminations et régiosélectivité` | licence 2 | · |

#### cinétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Action d'un catalyseur sur la barrière énergétique` | licence 2 | ✅ licence 2 |
| règle | `Approximation de l'état quasi stationnaire` | licence 2 | · |
| définition | `Catalyse homogène, hétérogène et enzymatique` | licence 2 | · |
| modèle | `Complexe activé et coordonnée réactionnelle` | licence 2 | · |
| définition | `Molécularité d'un acte élémentaire` | licence 2 | · |
| règle | `Étape cinétiquement déterminante` | licence 2 | · |

#### complexation

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Complexe, ligand et indice de coordination` | licence 2 | · |
| relation | `Constantes de formation d'un complexe` | licence 2 | ✅ licence 2 |
| règle | `Diagramme de prédominance des complexes` | licence 2 | · |

#### coordination

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Métaux de transition et orbitales d` | licence 2 | · |
| règle | `Nomenclature et géométrie des complexes` | licence 2 | · |

#### cristaux

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Alliages d'insertion et de substitution` | licence 2 | · |
| propriété | `Cristaux covalents et cristaux moléculaires` | licence 2 | · |
| modèle | `Liaison métallique et modèle des électrons libres` | licence 2 | · |
| loi | `Loi de Bragg` | licence 2 | ✅ licence 2 |
| propriété | `Structures types des cristaux ioniques` | licence 2 | · |
| théorème | `Énergie réticulaire et cycle de Born-Haber` | licence 2 | ✅ licence 2 |

#### liaison chimique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Caractère ionique partiel d'une liaison` | licence 2 | · |
| règle | `Diagramme d'orbitales moléculaires d'une molécule diatomique` | licence 2 | · |
| modèle | `Délocalisation électronique et mésomérie` | licence 2 | · |
| modèle | `Hybridation des orbitales atomiques` | licence 2 | · |
| relation | `Ordre de liaison` | licence 2 | ✅ licence 2 |
| modèle | `Recouvrement et orbitales moléculaires` | licence 2 | · |

#### précipitation

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Compétition entre précipitation et complexation` | licence 2 | ✅ licence 2 |
| propriété | `Influence du pH sur la solubilité` | licence 2 | · |

#### solutions

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Activité et écart à l'idéalité des solutions` | licence 2 · licence 3 | · |

#### thermodynamique chimique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Azéotropie et limites de la distillation` | licence 2 | · |
| théorème | `Critère thermodynamique d'évolution spontanée` | licence 2 | ✅ licence 2 |
| modèle | `Diagrammes binaires liquide-vapeur` | licence 2 | · |
| propriété | `Déplacement d'un équilibre par la pression` | licence 2 | ✅ licence 2 |
| théorème | `Enthalpie standard de réaction` | licence 2 | ✅ licence 2 |
| définition | `Entropie standard de réaction` | licence 2 | · |
| loi | `Loi de Henry` | licence 2 | · |
| règle | `Loi de modération` | licence 2 | · |
| loi | `Loi de Raoult` | licence 2 | · |
| définition | `Potentiel chimique` | licence 2 · licence 3 | · |
| propriété | `Propriétés colligatives` | licence 2 | ✅ licence 2 |
| loi | `Relation entre enthalpie libre standard et constante d'équilibre` | licence 2 | · |
| théorème | `Règle des phases de Gibbs` | licence 2 · licence 2 (classe préparatoire PC) | ✅ licence 2 |
| théorème | `Théorème des moments chimiques` | licence 2 | ✅ licence 2 |
| règle | `Équilibres hétérogènes` | licence 2 | · |

#### électrochimie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Dismutation et médiamutation` | licence 2 | · |
| propriété | `Influence du milieu sur le potentiel d'un couple` | licence 2 | · |
| relation | `Lien entre enthalpie libre standard et potentiel standard` | licence 2 | ✅ licence 2 |
| loi | `Lois de Faraday de l'électrolyse` | licence 2 | ✅ licence 2 |
| règle | `Prévision des réactions d'oxydoréduction` | licence 2 | · |
| loi | `Relation de Nernst` | licence 2 · licence 2 (classe préparatoire PC) | ✅ licence 2 |

### Licence 3

#### atomistique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Transitions électroniques et absorption dans le visible` | licence 3 | · |

#### chimie analytique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| définition | `Efficacité et résolution d'une colonne` | licence 3 | · |
| modèle | `Méthodes électrochimiques d'analyse` | licence 3 | · |
| modèle | `Spectrométrie de masse` | licence 3 | · |
| modèle | `Spectroscopies atomiques` | licence 3 | · |
| règle | `Validation d'une méthode analytique` | licence 3 | · |

#### chimie organique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Analyse rétrosynthétique` | licence 3 | · |
| règle | `Détermination de structure par spectroscopies croisées` | licence 3 | · |
| modèle | `Substitution électrophile aromatique` | licence 3 | · |

#### cinétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Cinétique enzymatique de Michaelis-Menten` | licence 3 | ✅ licence 3 |
| propriété | `Contrôle cinétique et contrôle thermodynamique` | licence 3 | · |
| modèle | `Réactions en chaîne` | licence 3 | · |

#### coordination

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Catalyse organométallique` | licence 3 | · |
| règle | `Champ fort, champ faible et propriétés magnétiques` | licence 3 | ✅ licence 3 |
| propriété | `Couleur des complexes` | licence 3 | · |
| propriété | `Effet chélate` | licence 3 | ✅ licence 3 |
| propriété | `Isomérie des complexes` | licence 3 | · |
| règle | `Série spectrochimique` | licence 3 | · |
| modèle | `Théorie du champ cristallin` | licence 3 | · |

#### cristaux

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Défauts cristallins` | licence 3 | · |
| modèle | `Semi-conducteurs et dopage` | licence 3 | · |
| modèle | `Théorie des bandes` | licence 3 | ✅ licence 3 |
| propriété | `Transitions de phase à l'état solide et allotropie` | licence 3 | · |

#### liaison chimique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Aromaticité` | licence 3 | · |
| modèle | `Origine quantique des interactions de Van der Waals` | licence 3 | · |

#### matériaux

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Relation entre structure et propriétés d'un matériau` | licence 3 | · |

#### thermodynamique chimique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Compromis dans l'optimisation industrielle` | licence 3 | · |
| modèle | `Diagrammes binaires solide-liquide` | licence 3 | · |
| loi | `Influence de la température sur un équilibre` | licence 3 | · |
| règle | `Lecture d'un diagramme binaire` | licence 3 | · |
| loi | `Pression osmotique` | licence 3 | · |

#### électrochimie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Accumulateurs` | licence 3 | · |
| modèle | `Corrosion humide d'un métal` | licence 3 | · |
| modèle | `Courbes intensité-potentiel et surtension` | licence 3 | · |
| modèle | `Diagramme potentiel-pH` | licence 3 | · |
| propriété | `Diagramme potentiel-pH de l'eau` | licence 3 | ✅ licence 3 |
| règle | `Protection contre la corrosion` | licence 3 | · |
| règle | `Prévision des réactions d'électrolyse` | licence 3 | · |

### Master 1

#### catalyse

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Désactivation et sélectivité d'un catalyseur` | master 1 · master 1 (agrégation externe) | · |
| loi | `Isotherme de Langmuir` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Mécanisme de Langmuir-Hinshelwood` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| définition | `Performance d'un catalyseur et grandeurs de rotation` | master 1 · master 1 (agrégation externe) | · |

#### chimie organométallique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Addition oxydante et élimination réductrice` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Catalyse asymétrique et induction chirale` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Couplages croisés pallado-catalysés` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Insertion migratoire et bêta-élimination` | master 1 · master 1 (agrégation externe) | · |

#### chimie quantique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Approximation de Born-Oppenheimer` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| règle | `Méthode de Hartree-Fock et champ autocohérent` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| règle | `Méthode de Hückel` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| règle | `Orbitales frontières et réactivité` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Polarisabilité et forces de dispersion` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| règle | `Règles de Woodward-Hoffmann` | master 1 · master 1 (agrégation externe) | · |
| théorème | `Théorème de Jahn-Teller` | master 1 · master 1 (agrégation externe) | · |
| théorème | `Théorème de Koopmans` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| définition | `Énergie de corrélation électronique` | master 1 · master 1 (agrégation externe) | · |

#### cinétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Effet isotopique cinétique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Grandeurs d'activation et molécularité` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| postulat | `Postulat de Hammond` | master 1 · master 1 (agrégation externe) | · |
| loi | `Relation de Hammett` | master 1 · master 1 (agrégation externe) | · |
| théorème | `Théorie de l'état de transition et équation d'Eyring` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### coordination

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| règle | `Règle des dix-huit électrons` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Rétrodonation et ligands pi-accepteurs` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### photochimie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Diagramme de Perrin-Jablonski` | master 1 · master 1 (agrégation externe) | · |
| loi | `Relation de Stern-Volmer` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Rendement quantique photochimique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### polymères

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Chaîne gaussienne et rayon de giration` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Chromatographie d'exclusion stérique` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Masses molaires moyennes et indice de polymolécularité` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| modèle | `Polymérisation en chaîne radicalaire` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| loi | `Polymérisation par étapes et équation de Carothers` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Polymérisation vivante et contrôle des masses` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Stéréorégularité et catalyse de coordination` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Transition vitreuse et états physiques d'un polymère` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Élasticité caoutchoutique d'origine entropique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### spectroscopie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Condition de résonance magnétique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Couplage scalaire et multiplicité des signaux` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Déplacement chimique et blindage électronique` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| règle | `Fragmentation et reconstitution de structure` | master 1 · master 1 (agrégation externe) | · |
| principe | `Principe de Franck-Condon` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Relaxation et acquisition par transformée de Fourier` | master 1 · master 1 (agrégation externe) | · |
| règle | `Règles de sélection en infrarouge et en Raman` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Résonance paramagnétique électronique` | master 1 · master 1 (agrégation externe) | · |
| propriété | `Spectrométrie de masse à haute résolution` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Spectroscopie de rotation et constante rotationnelle` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Spectroscopie de vibration et anharmonicité` | master 1 · master 1 (agrégation externe) | ✅ master 1 |

#### électrochimie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| loi | `Relation de Butler-Volmer` | master 1 · master 1 (agrégation externe) | ✅ master 1 |
| propriété | `Voltampérométrie cyclique` | master 1 · master 1 (agrégation externe) | · |

### Master 2

#### chimie organométallique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Métathèse des oléfines` | master 2 · master 2 (agrégation externe) | · |

#### chimie quantique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Effets relativistes en chimie des éléments lourds` | master 2 · master 2 (agrégation externe) | · |
| théorème | `Théorie de la fonctionnelle de la densité` | master 2 · master 2 (agrégation externe) | · |

#### cinétique

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| théorème | `Théorie de Marcus du transfert d'électron` | master 2 · master 2 (agrégation externe) | · |

#### photochimie

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| propriété | `Photocatalyse et conversion de l'énergie lumineuse` | master 2 · master 2 (agrégation externe) | · |

#### polymères

| Type | Appel | Ancrages | Démonstration |
|---|---|---|---|
| modèle | `Théorie de Flory-Huggins des solutions de polymères` | master 2 · master 2 (agrégation externe) | ✅ master 2 |

---

## **📚 La bibliothèque des classiques**

**101 démonstrations d'idée**, servies par **`<Montre>`** — celles qu'aucun
calcul n'atteint, et qui reposent sur une construction ou un argument de
structure. Elles s'appellent **par leur énoncé**, sans corps :

```docdg
<Montre>que $racine(2)$ est irrationnel
```

Tout ce qu'une identité ou une inégalité vérifiable démontre relève du calcul
formel, non d'une fiche : `<Montre>par récurrence que …` est vérifié puis
rédigé par le moteur, sans passer par ici.

### lycée — 13 démonstrations

| Raisonnement | Appel | Autres formulations reçues |
|---|---|---|
| direct | `<Montre>le lemme de Gauss : si $a$ divise $bc$ et $\mathrm{pgcd}(a,b)=1$, alors $a$ divise $c$` | `lemme de gauss` · `si a divise bc et pgcd(a,b)=1 alors a divise c` |
| direct | `<Montre>le théorème de Bézout : si $a$ et $b$ sont non nuls, il existe $u,v\in\mathbb{Z}$ tels que $au+bv=\mathrm{pgcd}(a,b)$` | `theoreme de bezout` · `identite de bezout` · `pgcd combinaison lineaire` |
| par récurrence | `<Montre>le théorème des valeurs intermédiaires : si $f$ est continue sur $[a;b]$ et $k$ est entre $f(a)$ et $f(b)$, alors il existe $c\in[a;b]$ tel que $f(c)=k$` | `theoreme des valeurs intermediaires` · `tvi` · `si f continue et f(a)f(b)<0 alors il existe c avec f(c)=0` |
| direct | `<Montre>l’inégalité arithmético-géométrique : pour $x_i\ge 0$, $\frac{x_1+\dots+x_n}{n}\ge\sqrt[n]{x_1\cdots x_n}$` | `inegalite de la moyenne` · `moyenne arithmetico-geometrique` · `AM-GM` |
| par l'absurde | `<Montre>qu'il existe une infinité de nombres premiers` | `il existe une infinite de nombres premiers` · `l'ensemble des nombres premiers est infini` · `les nombres premiers sont en nombre infini` |
| par l'absurde | `<Montre>qu'il n'existe pas de plus grand entier naturel` | `il n'existe pas de plus grand entier` · `l'ensemble des entiers n'a pas de maximum` |
| par l'absurde | `<Montre>que $racine(2)$ est irrationnel` | `racine(2) est irrationnel` · `racine de 2 est irrationnel` · `racine(2) n'est pas rationnel` · `2 n'a pas de racine rationnelle` |
| par l'absurde | `<Montre>que $racine(3)$ est irrationnel` | `racine(3) est irrationnel` · `racine de 3 est irrationnel` |
| par le principe des tiroirs | `<Montre>que dans un groupe de $13$ personnes, deux au moins sont nées le même mois` | `dans un groupe de 13 personnes deux au moins sont nees le meme mois` · `13 personnes meme mois` |
| par l'absurde | `<Montre>que la somme d'un rationnel et d'un irrationnel est irrationnelle` | `la somme d'un rationnel et d'un irrationnel est irrationnelle` · `rationnel plus irrationnel est irrationnel` |
| direct | `<Montre>que la somme de deux rationnels est rationnelle` | `la somme de deux rationnels est rationnelle` · `la somme de deux nombres rationnels est un rationnel` |
| par disjonction de cas | `<Montre>que pour tous réels $a$ et $b$, le plus grand des deux vaut $(a + b + \|a - b\|)/2$` | `le maximum de deux reels s'exprime avec la valeur absolue` · `max(a,b) = (a+b+\|a-b\|)/2` |
| direct | `<Montre>qu’une suite croissante majorée de réels converge (vers son supremum)` | `theoreme de convergence monotone` · `suite croissante majoree converge` |

### L1 — 29 démonstrations

| Raisonnement | Appel | Autres formulations reçues |
|---|---|---|
| direct | `<Montre>l'inégalité de Cauchy-Schwarz : $(somme(k=1;n) a_k b_k)^2 <= (somme(k=1;n) a_k^2) * (somme(k=1;n) b_k^2)$` | `l'inegalite de cauchy-schwarz` · `l'inegalite de cauchy schwarz` · `le carre du produit scalaire est majore par le produit des carres des normes` |
| direct | `<Montre>la formule de Taylor-Lagrange : si $f$ est $n+1$ fois dérivable, alors il existe $c$ entre $a$ et $x$ tel que $f(x)=\sum_{k=0}^n\frac{f^{(k)}(a)}{k!}(x-a)^k+\frac{f^{(n+1)}(c)}{(n+1)!}(x-a)^{n+1}$` | `formule de taylor-lagrange` · `reste de lagrange` |
| direct | `<Montre>le théorème de Darboux : si $f$ est dérivable sur un intervalle, alors $f'$ vérifie la propriété des valeurs intermédiaires (même si $f'$ n’est pas continue)` | `theoreme de darboux` · `la derivee verifie la propriete des valeurs intermediaires` |
| direct | `<Montre>le théorème de Rolle : si $f$ est continue sur $[a;b]$, dérivable sur $]a;b[$ et $f(a)=f(b)$, alors il existe $c\in]a;b[$ tel que $f'(c)=0$` | `theoreme de rolle` · `si f(a)=f(b) alors il existe c avec f'(c)=0` |
| direct | `<Montre>le théorème des accroissements finis : si $f$ est continue sur $[a;b]$ et dérivable sur $]a;b[$, alors il existe $c\in]a;b[$ tel que $f(b)-f(a)=(b-a)f'(c)$` | `theoreme des accroissements finis` · `il existe c tel que f(b)-f(a)=(b-a)f'(c)` |
| direct | `<Montre>le théorème fondamental de l’analyse : si $f$ est continue, alors $F(x)=\int_a^x f(t)\,dt$ est dérivable et $F'=f$` | `theoreme fondamental de l'analyse` · `derivee de la fonction integrale` |
| par disjonction de cas | `<Montre>qu'il existe deux irrationnels $a$ et $b$ tels que $a^b$ soit rationnel` | `il existe deux irrationnels dont la puissance est rationnelle` |
| direct | `<Montre>qu'une fonction dérivable de dérivée nulle sur un intervalle y est constante` | `une fonction de derivee nulle sur un intervalle est constante` |
| direct | `<Montre>qu'une fonction dérivable de dérivée positive sur un intervalle y est croissante` | `une fonction de derivee positive est croissante` · `le lien entre signe de la derivee et sens de variation` |
| direct | `<Montre>qu'une partie non vide $H$ d'un groupe $G$, stable par $x y^(-1)$, est un sous-groupe de $G$` | `la caracterisation d'un sous-groupe` · `une partie non vide stable par produit et inverse est un sous-groupe` |
| par l'absurde | `<Montre>que $\mathbb{R}$ (ou $[0;1]$) n’est pas dénombrable` | `argument diagonal de cantor` · `R non denombrable` · `non denombrabilite de R` |
| par double inclusion | `<Montre>que $A inter (B union C) = (A inter B) union (A inter C)$` | `la distributivite de l'intersection sur la reunion` · `A inter (B union C) = (A inter B) union (A inter C)` |
| direct | `<Montre>que dans un groupe, l'inverse du produit $xy$ est $y^(-1) x^(-1)$` | `l'inverse d'un produit est le produit des inverses dans l'ordre inverse` |
| direct | `<Montre>que l'intersection de deux sous-groupes est un sous-groupe` | `l'intersection de deux sous-groupes est un sous-groupe` |
| direct | `<Montre>que l'inverse d'un élément d'un groupe est unique` | `l'unicite de l'inverse dans un groupe` · `l'inverse d'un element d'un groupe est unique` |
| par l'absurde | `<Montre>que l'élément neutre d'un groupe est unique` | `l'unicite de l'element neutre d'un groupe` · `le neutre d'un groupe est unique` |
| direct | `<Montre>que la composée de deux applications injectives est injective` | `la composee de deux injections est injective` · `la composee de deux applications injectives est injective` |
| direct | `<Montre>que la composée de deux applications surjectives est surjective` | `la composee de deux surjections est surjective` |
| direct | `<Montre>que la composée de deux fonctions continues est continue` | `la composee de deux fonctions continues est continue` |
| par l'absurde | `<Montre>que la limite d'une fonction en un point, lorsqu'elle existe, est unique` | `l'unicite de la limite d'une fonction en un point` |
| par l'absurde | `<Montre>que la limite d'une suite convergente est unique` | `l'unicite de la limite d'une suite` · `la limite d'une suite est unique` · `unicite de la limite` |
| par l'absurde | `<Montre>que la racine carrée d'un nombre premier est irrationnelle` | `la racine carree d'un nombre premier est irrationnelle` · `racine(p) est irrationnel pour p premier` |
| par double inclusion | `<Montre>que le complémentaire d'une réunion est l'intersection des complémentaires` | `les lois de de morgan` · `le complementaire d'une reunion est l'intersection des complementaires` |
| direct | `<Montre>que le terme général d'une série convergente tend vers $0$` | `le terme general d'une serie convergente tend vers zero` |
| direct | `<Montre>que si la composée $g rond f$ est injective, alors $f$ est injective` | `si la composee g rond f est injective alors f est injective` |
| direct | `<Montre>que toute fonction continue sur un segment est Riemann-intégrable` | `toute fonction continue est integrable` · `integrale de riemann d'une fonction continue` |
| direct | `<Montre>que toute fonction dérivable en un point y est continue` | `toute fonction derivable est continue` · `une fonction derivable en un point y est continue` · `la derivabilite implique la continuite` |
| direct | `<Montre>que toute suite convergente est bornée` | `toute suite convergente est bornee` · `une suite convergente est bornee` |
| direct | `<Montre>que toute suite de Cauchy est bornée` | `toute suite de cauchy est bornee` · `une suite de cauchy est bornee` |

### L2 — 24 démonstrations

| Raisonnement | Appel | Autres formulations reçues |
|---|---|---|
| direct | `<Montre>la formule de la dimension : $\dim(F+G)=\dim F+\dim G-\dim(F\cap G)$` | `formule de dimension` · `dim(F+G)=dim F + dim G - dim(F cap G)` |
| direct | `<Montre>le lemme de Steinitz : si $(v_1,\dots,v_n)$ engendre $E$ et $(u_1,\dots,u_p)$ est libre, alors $p\le n$ et on peut remplacer $p$ vecteurs de la famille génératrice par les $u_i$` | `lemme de steinitz` · `echange de steinitz` |
| direct | `<Montre>le premier théorème de Weierstrass : toute fonction continue sur un segment est limite uniforme d’une suite de polynômes` | `theoreme de weierstrass` · `toute fonction continue sur un segment est limite uniforme de polynomes` |
| par récurrence | `<Montre>le procédé de Gram-Schmidt : toute famille libre admet une famille orthonormée ayant le même espace engendré` | `procede de gram-schmidt` · `orthogonalisation` |
| par l'absurde | `<Montre>le théorème de Cantor : il n'existe pas de surjection d'un ensemble $E$ sur l'ensemble de ses parties` | `il n'existe pas de surjection d'un ensemble sur l'ensemble de ses parties` · `le theoreme de cantor` |
| direct | `<Montre>le théorème de Cantor-Bernstein : s’il existe une injection de $A$ dans $B$ et une injection de $B$ dans $A$, alors il existe une bijection entre $A$ et $B$` | `theoreme de cantor-bernstein` · `s'il existe une injection de A dans B et de B dans A alors il existe une bijection` |
| direct | `<Montre>le théorème de Cauchy : si $p$ est un nombre premier divisant l’ordre d’un groupe fini $G$, alors $G$ possède un élément d’ordre $p$` | `theoreme de cauchy pour les groupes` · `si p premier divise \|G\| alors il existe element d'ordre p` |
| direct | `<Montre>le théorème de Heine-Borel : un sous-ensemble de $\mathbb{R}$ est compact si et seulement s’il est fermé et borné` | `theoreme de heine-borel` · `compact de R = ferme borne` |
| direct | `<Montre>le théorème de Lagrange : l’ordre d’un sous-groupe d’un groupe fini divise l’ordre du groupe` | `theoreme de lagrange` · `l'ordre d'un sous-groupe divise l'ordre du groupe` |
| direct | `<Montre>le théorème du point fixe de Banach : une contraction d’un espace métrique complet admet un unique point fixe` | `theoreme du point fixe de banach` · `contraction a un unique point fixe` |
| direct | `<Montre>le théorème du rang : pour une application linéaire $f:E\to F$ entre espaces de dimension finie, $\mathrm{rg}(f)=\dim E-\dim(\ker f)$` | `theoreme du rang` · `rg(f)=dim(E)-dim(ker f)` · `relation noyau image` · `dim ker + dim im = dim E` |
| par double implication | `<Montre>qu'une application d'un ensemble fini dans lui-même est injective si et seulement si elle est surjective` | `une application d'un ensemble fini dans lui-meme est injective si et seulement si elle est surjective` |
| par double implication | `<Montre>qu'une application linéaire est injective si et seulement si son noyau est réduit au vecteur nul` | `une application lineaire est injective si et seulement si son noyau est nul` · `injectivite et noyau` · `ker f = 0 equivaut a f injective` |
| direct | `<Montre>qu'une famille orthogonale de vecteurs non nuls est libre` | `une famille orthogonale de vecteurs non nuls est libre` |
| par double implication | `<Montre>que dans un espace métrique, la compacité équivaut à la compacité séquentielle` | `compacite sequentielle` · `dans un espace metrique compact = sequentiellement compact` |
| par récurrence | `<Montre>que dans un espace vectoriel de dimension finie, toute famille libre peut être complétée en une base` | `theoreme de la base incomplete` · `toute famille libre se complete en une base` |
| direct | `<Montre>que deux espaces vectoriels de même dimension finie sont isomorphes` | `deux espaces de meme dimension finie sont isomorphes` · `isomorphisme entre espaces vectoriels` |
| direct | `<Montre>que l'image d'une application linéaire est un sous-espace vectoriel` | `l'image d'une application lineaire est un sous-espace vectoriel` |
| direct | `<Montre>que l'intersection de deux sous-espaces vectoriels d'un même espace est un sous-espace vectoriel` | `l'intersection de deux sous-espaces vectoriels est un sous-espace vectoriel` · `intersection de sous-espaces vectoriels` |
| direct | `<Montre>que le noyau d'une application linéaire est un sous-espace vectoriel` | `le noyau d'une application lineaire est un sous-espace vectoriel` |
| direct | `<Montre>que tout espace vectoriel de dimension finie non nul admet une base` | `existence d'une base` · `tout espace vectoriel de dimension finie admet une base` |
| direct | `<Montre>que tout groupe dont l'ordre est un nombre premier est cyclique` | `tout groupe d'ordre premier est cyclique` · `un groupe dont l'ordre est premier est cyclique` |
| direct | `<Montre>que tout sous-groupe d’un groupe cyclique est cyclique, et qu’il y a exactement un sous-groupe pour chaque diviseur de l’ordre` | `sous-groupes d'un groupe cyclique` · `groupe cyclique d'ordre n` |
| direct | `<Montre>qu’il existe une unique projection orthogonale sur un sous-espace de dimension finie d’un espace préhilbertien` | `existence et unicite de la projection orthogonale` · `projection sur un sous-espace` |

### CPGE — 35 démonstrations

| Raisonnement | Appel | Autres formulations reçues |
|---|---|---|
| existence et unicité | `<Montre>l'existence et l'unicité du quotient et du reste de la division euclidienne d'un entier $a$ par un entier $b$ strictement positif` | `l'existence et l'unicite de la division euclidienne` · `division euclidienne` · `existence et unicite du quotient et du reste` |
| par l'absurde | `<Montre>la propriété d'Archimède : pour tout réel $x$, il existe un entier naturel strictement supérieur à $x$` | `la propriete d'archimede` · `pour tout reel il existe un entier plus grand` |
| direct | `<Montre>le critère de comparaison : si $0\le u_n\le v_n$ et si $\sum v_n$ converge, alors $\sum u_n$ converge` | `critere de comparaison des series` · `si 0 <= u_n <= v_n et somme v converge alors somme u converge` |
| direct | `<Montre>le lemme d’Abel : si la série $\sum a_n z_0^n$ converge, alors $\sum a_n z^n$ converge absolument pour tout $\|z\|<\|z_0\|$` | `rayon de convergence` · `lemme d'abel` |
| direct | `<Montre>le petit théorème de Fermat : si $p$ est premier, alors pour tout entier $a$ on a $a^p\equiv a\pmod{p}$` | `petit theoreme de fermat` · `a^p \equiv a mod p pour p premier` |
| direct | `<Montre>le théorème de Cesàro : si $(u_n)$ converge vers $\ell$, alors la suite des moyennes $\sigma_n=\frac{1}{n}\sum_{k=1}^n u_k$ converge aussi vers $\ell$` | `theoreme de cesaro` · `moyenne de cesaro` |
| par double implication | `<Montre>le théorème de Wilson : $p$ est premier si et seulement si $(p-1)!\equiv-1\pmod{p}$` | `theoreme de wilson` · `(p-1)! \equiv -1 mod p` |
| direct | `<Montre>le théorème des gendarmes : une suite encadrée par deux suites de même limite converge vers cette limite` | `le theoreme des gendarmes` · `le theoreme d'encadrement des suites` |
| direct | `<Montre>le théorème d’Euler : si $\mathrm{pgcd}(a,n)=1$, alors $a^{\varphi(n)}\equiv 1\pmod{n}$` | `theoreme d'euler` · `a^phi(n) equiv 1 mod n` |
| direct | `<Montre>qu'entre deux rationnels distincts se trouve toujours un troisième rationnel` | `entre deux rationnels distincts il existe un rationnel` · `la densite des rationnels dans eux-memes` |
| par récurrence | `<Montre>qu'un ensemble à $n$ éléments admet exactement $2^n$ parties` | `un ensemble a n elements admet 2^n parties` · `le cardinal de l'ensemble des parties est 2^n` · `card(P(E)) = 2^n` |
| direct | `<Montre>que deux suites adjacentes convergent vers une même limite` | `deux suites adjacentes convergent vers la meme limite` · `le theoreme des suites adjacentes` · `suites adjacentes` |
| par contraposée | `<Montre>que l'inverse d'un irrationnel non nul est irrationnel` | `l'inverse d'un irrationnel est irrationnel` |
| par l'absurde | `<Montre>que la série harmonique diverge` | `la serie harmonique diverge` · `la somme des inverses des entiers diverge` |
| direct | `<Montre>que le produit d'une suite bornée par une suite de limite nulle tend vers $0$` | `le produit d'une suite bornee par une suite tendant vers zero tend vers zero` |
| par le principe des tiroirs | `<Montre>que parmi cinq entiers quelconques, deux au moins ont le même reste dans la division par $4$` | `parmi cinq entiers deux ont le meme reste modulo quatre` |
| direct | `<Montre>que pour tous réels $a$ et $b$, $\|a+b\| <= \|a\| + \|b\|$` | `l'inegalite triangulaire` · `\|a+b\| <= \|a\| + \|b\|` · `la valeur absolue d'une somme est majoree par la somme des valeurs absolues` |
| direct | `<Montre>que si un nombre premier $p$ divise un produit $ab$, alors il divise $a$ ou il divise $b$` | `le lemme d'euclide` · `si un nombre premier divise un produit il divise l'un des facteurs` |
| par l'absurde | `<Montre>que tout entier supérieur ou égal à $2$ admet un diviseur premier` | `tout entier superieur ou egal a 2 admet un diviseur premier` · `existence d'un diviseur premier` |
| par l'absurde | `<Montre>que tout entier supérieur ou égal à $2$ s'écrit comme produit de nombres premiers` | `tout entier superieur ou egal a 2 est un produit de nombres premiers` · `l'existence de la decomposition en facteurs premiers` |
| par analyse-synthèse | `<Montre>que toute fonction $f$ de $RR$ dans $RR$ s'écrit de façon unique comme somme d'une fonction paire et d'une fonction impaire` | `toute fonction se decompose en somme d'une fonction paire et d'une fonction impaire` · `decomposition d'une fonction en partie paire et partie impaire` |
| direct | `<Montre>que toute fonction continue sur un segment $[a;b]$ est bornée et atteint ses bornes` | `theoreme des bornes atteintes` · `toute fonction continue sur un segment est bornee et atteint ses bornes` |
| par l'absurde | `<Montre>que toute fonction continue sur un segment est uniformément continue` | `toute fonction continue sur un segment est uniformement continue` · `theoreme de heine` · `heine` |
| par récurrence | `<Montre>que toute suite bornée de réels admet au moins une sous-suite convergente` | `toute suite bornee admet une sous-suite convergente` · `theoreme de bolzano-weierstrass` · `bolzano weierstrass` |
| direct | `<Montre>que toute suite croissante et majorée converge vers la borne supérieure de ses termes` | `toute suite croissante et majoree converge` · `le theoreme de la limite monotone` · `une suite croissante majoree converge vers sa borne superieure` |
| direct | `<Montre>que toute suite croissante et non majorée tend vers $+infini$` | `toute suite croissante non majoree tend vers plus l'infini` · `une suite croissante non majoree diverge vers l'infini` |
| direct | `<Montre>que toute suite extraite d'une suite convergente converge vers la même limite` | `toute suite extraite d'une suite convergente converge vers la meme limite` · `les suites extraites d'une suite convergente` |
| par contraposée | `<Montre>que toute suite non bornée diverge` | `une suite non bornee ne converge pas` · `toute suite non bornee diverge` |
| direct | `<Montre>qu’il existe au plus un prolongement continu d’une fonction définie sur une partie dense` | `unicite du prolongement continu` · `densite et continuite` |
| par double implication | `<Montre>qu’une fonction $f:E\to F$ entre espaces métriques est continue en $a$ si et seulement si pour toute suite $x_n\to a$ on a $f(x_n)\to f(a)$` | `continuite sequentielle` · `f continue ssi image de suite convergente` |
| par l'absurde | `<Montre>qu’une fonction continue sur un compact métrique est uniformément continue` | `continuite uniforme sur un compact` · `heine pour les fonctions` |
| par double implication | `<Montre>qu’une partie $A$ d’un espace métrique est fermée si et seulement si elle contient toutes les limites de ses suites convergentes` | `caracterisation sequentielle des fermes` · `un ensemble est ferme ssi il contient les limites de ses suites` |
| par double implication | `<Montre>qu’une suite réelle est convergente si et seulement si elle est de Cauchy` | `critere de cauchy pour les suites` · `suite de cauchy dans R` |
| direct | `<Montre>qu’une série absolument convergente est convergente` | `convergence absolue implique convergence` · `serie absolument convergente` |
| direct | `<Montre>qu’à l’intérieur du disque de convergence, une série entière est dérivable terme à terme et la série dérivée a le même rayon` | `derivation terme a terme des series entieres` · `serie derivee` |

---

*Engendré depuis `corpus/donnees/` et `transpiler/src/maths/demonstrations.json` par `cargo app recensement --redaction`.*
