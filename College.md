# docdg — Le collège

La sixième à la troisième : la rédaction des énoncés, les figures, le corpus de mathématiques et de physique-chimie du cycle 4.

> Ce document rassemble ce que docdg apporte à ce niveau.
> Les fonctionnalités communes à tous les niveaux — la syntaxe, les objets,
> les styles, le langage algorithmique, les graphiques — sont décrites dans le
> [README](README.md), qui reste le manuel de référence.

---

## **🏫 Le collège, rédigé**

> Les rédactions les plus tapées de France, chacune en une phrase — calcul exact, justification comprise, tracé quand il le faut.

### **Le trio roi : Pythagore, Thalès, trigonométrie**

```docdg
<Calculer>AC dans le triangle ABC rectangle en B, avec AB = 3 et BC = 4
<Vérifier>si le triangle ABC est rectangle, avec AB = 3, BC = 4 et AC = 5
<Calculer>AC par le théorème de Thalès, avec AM = 3, AB = 6 et AN = 4
<Vérifier>si les droites (MN) et (BC) sont parallèles, avec AM = 3, AB = 6, AN = 4 et AC = 8
<Calculer>BC dans le triangle ABC rectangle en B, avec l'angle A = 30 degrés et AC = 8
<Calculer>l'angle A dans le triangle ABC rectangle en B, avec AB = 3 et AC = 6
```

La rédaction produite est la rédaction canonique : théorème énoncé, carrés détaillés, racine exacte (ou approchée), réciproque ou contraposée selon le cas. La trigonométrie choisit seule le bon rapport (cos, sin ou tan) selon les côtés donnés, le nomme (opposé/hypoténuse…), garde les valeurs exactes des angles remarquables et retrouve un angle « à la calculatrice, touche cos⁻¹ ». Thalès travaille dans la configuration universelle du triangle $ABC$, $M$ sur $[AB]$, $N$ sur $[AC]$.

### **Statistiques d'une série**

```docdg
<Calculer>la moyenne de la série 12 ; 15 ; 9 ; 14
<Calculer>la variance de la série 12 ; 15 ; 9 ; 14
<Calculer>l'écart type de la série 12 ; 15 ; 9 ; 14
<Calculer>la covariance des séries 1 ; 2 ; 3 et 2 ; 4 ; 7
<Calculer>la moyenne de la série de valeurs 8 ; 12 ; 15 et d'effectifs 2 ; 5 ; 3
<Calculer>la médiane de la série 9 ; 15 ; 12 ; 14
<Calculer>l'étendue de la série 12 ; 15 ; 9 ; 14
<Calculer>les quartiles de la série 2 ; 4 ; 5 ; 7 ; 8 ; 10 ; 12 ; 15
```

Série rangée, cas pair et impair distingués pour la médiane, rangs $\lceil n/4 \rceil$ affichés pour les quartiles.

### **Proportionnalité, pourcentages, échelles, vitesses**

```docdg
<Calculer>la quatrième proportionnelle de 3, 5 et 12
<Vérifier>si le tableau est de proportionnalité{
	2 ; 3 ; 5
	6 ; 9 ; 15
}
<Compléter>le tableau de proportionnalité{
	2 ; 3 ; ?
	6 ; ? ; 15
}
<Calculer>30 % de 250
<Appliquer>une augmentation de 5 % à 240
<Appliquer>une diminution de 12 % à 60
<Calculer>le taux d'évolution de 250 à 280
<Calculer>l'échelle d'un plan où 2 cm représentent 50 m
<Calculer>la vitesse moyenne pour 150 km en 2 h 30 min
<Calculer>la distance parcourue à 80 km/h pendant 1 h 45 min
<Calculer>la durée du trajet de 210 km à 60 km/h
```

Produit en croix rédigé, coefficient multiplicateur pour les évolutions, durées converties dans les deux sens (« 3,5 h = 3 h 30 min »).

### **Mesures et conversions**

```docdg
<Calculer>le périmètre du cercle de rayon 5
<Calculer>l'aire du triangle de base 6 et de hauteur 4
<Calculer>le volume de la boule de rayon 3
<Convertir>3,5 km en m
<Convertir>2500 cm^2 en m^2
<Convertir>3 L en cm^3
```

Dix figures (du carré à la boule), formule affichée, $\pi$ gardé exact ($36\pi \approx 113{,}1$). Les conversions se font par décalage décimal exact et commenté, sur les longueurs, masses, contenances, aires (avec are et hectare) et volumes (le litre dialogue avec le décimètre cube).

### **Nombres et programmes de calcul**

```docdg
<Soit>une fonction f(x) = 2x + 1
<Calculer>1/2 + 1/3
<Simplifier>la fraction 84/60
<Écrire>le nombre 45 600 en notation scientifique
<Vérifier>si 456 est divisible par 3
<Effectuer>la division euclidienne de 47 par 5
<Calculer>le PGCD de 84 et 60
<Appliquer>le programme de calcul à 5{
	choisir un nombre
	ajouter 3
	multiplier par 2
	soustraire 4
}
<Exprimer>le programme de calcul en fonction de x{
	choisir un nombre
	ajouter 3
	multiplier par 2
	soustraire 4
}
<Calculer>l'image de 3 par f
```

Mise au même dénominateur, inverse pour la division, simplification par le PGCD, critères de divisibilité justifiés (somme des chiffres, chiffre des unités).

### **Transformations du plan**

```docdg
<Soient>les points A, B et C de coordonnées respectives (1 ; 1), (4 ; 1) et (2 ; 3)
<Soit>un vecteur u(2;1)

<Construire>l'image du triangle ABC par la symétrie axiale d'axe l'axe des abscisses
<Construire>l'image du triangle ABC par la symétrie centrale de centre O
<Construire>l'image du triangle ABC par la translation de vecteur u
<Construire>l'image du triangle ABC par la rotation de centre O et d'angle 90 degrés
<Construire>l'image du triangle ABC par l'homothétie de centre O et de rapport 2
```

Cinq transformations sur point, segment, triangle, quadrilatère ou polygone. Chaque action donne les coordonnées des images, puis le tracé sur quadrillage : figure d'origine en bleu, image en rouge avec les labels primes, lignes de construction en pointillé, axe en tirets, centre marqué d'une croix. L'axe accepte aussi `l'axe des ordonnées` et `(PQ)` ; la translation, `qui transforme A en B` ; la rotation, `dans le sens des aiguilles d'une montre`.

---


## La physique-chimie au collège

Le corpus couvre le cycle 4 : 64 énoncés de la cinquième à la troisième.

**Physique** — référentiel et trajectoire, vitesse, mouvement uniforme,
interactions et forces, poids, gravitation, équilibre ; énergie et ses
conversions, puissance, rendement ; circuits en série et en dérivation, loi
des nœuds, additivité des tensions, loi d'Ohm, effet Joule, court-circuit ;
propagation rectiligne, vitesse de la lumière, année-lumière, propagation du
son, fréquence et hauteur, niveau sonore.

**Chimie** — espèces chimiques, corps purs et mélanges, solutions et
solubilité, masse volumique, états de la matière et changements d'état, modèle
particulaire, atomes, molécules et ions, éléments chimiques ; transformations
chimiques, conservation de la masse et des atomes, équations de réaction,
combustions ; tests d'identification, pH, acidité et basicité.

**`<Équilibrer>`** ajuste une équation de réaction, coefficients entiers
minimaux — la conservation des atomes, appliquée plutôt que récitée :

```docdg
<Équilibrer>l'équation C3H8 + O2 -> CO2 + H2O
```

**Les exemples** `physique.txt` et `chimie.txt` montrent un cours rédigé à ce
niveau.
