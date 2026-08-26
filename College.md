# docdg — Le collège

La sixième à la troisième : la rédaction des énoncés, les figures, le corpus de mathématiques et de physique-chimie du cycle 4.

> Ce document rassemble ce que docdg apporte à ce niveau.
> Les fonctionnalités communes à tous les niveaux — la syntaxe, les objets,
> les styles, le langage algorithmique, les graphiques — sont décrites dans le
> [README](README.md), qui reste le manuel de référence.

---

<a id="-le-collège-rédigé"></a>

## **🏫 Le collège, rédigé**

> Les rédactions les plus tapées de France, chacune en une phrase — calcul exact, justification comprise, tracé quand il le faut.

### **Le trio roi : Pythagore, Thalès, trigonométrie**

```docdg
<Calcule>AC dans le triangle ABC rectangle en B, avec AB = 3 et BC = 4
<Vérifie>si le triangle ABC est rectangle, avec AB = 3, BC = 4 et AC = 5
<Calcule>AC par le théorème de Thalès, avec AM = 3, AB = 6 et AN = 4
<Vérifie>si les droites (MN) et (BC) sont parallèles, avec AM = 3, AB = 6, AN = 4 et AC = 8
<Calcule>BC dans le triangle ABC rectangle en B, avec l'angle A = 30 degrés et AC = 8
<Calcule>l'angle A dans le triangle ABC rectangle en B, avec AB = 3 et AC = 6
```

La rédaction produite est la rédaction canonique : théorème énoncé, carrés détaillés, racine exacte (ou approchée), réciproque ou contraposée selon le cas. La trigonométrie choisit seule le bon rapport (cos, sin ou tan) selon les côtés donnés, le nomme (opposé/hypoténuse…), garde les valeurs exactes des angles remarquables et retrouve un angle « à la calculatrice, touche cos⁻¹ ». Thalès travaille dans la configuration universelle du triangle $ABC$, $M$ sur $[AB]$, $N$ sur $[AC]$.

### **Statistiques d'une série**

```docdg
<Calcule>la moyenne de la série 12 ; 15 ; 9 ; 14
<Calcule>la variance de la série 12 ; 15 ; 9 ; 14
<Calcule>l'écart type de la série 12 ; 15 ; 9 ; 14
<Calcule>la covariance des séries 1 ; 2 ; 3 et 2 ; 4 ; 7
<Calcule>la moyenne de la série de valeurs 8 ; 12 ; 15 et d'effectifs 2 ; 5 ; 3
<Calcule>la médiane de la série 9 ; 15 ; 12 ; 14
<Calcule>l'étendue de la série 12 ; 15 ; 9 ; 14
<Calcule>les quartiles de la série 2 ; 4 ; 5 ; 7 ; 8 ; 10 ; 12 ; 15
```

Série rangée, cas pair et impair distingués pour la médiane, rangs $\lceil n/4 \rceil$ affichés pour les quartiles.

### **Proportionnalité, pourcentages, échelles, vitesses**

```docdg
<Calcule>la quatrième proportionnelle de 3, 5 et 12
<Vérifie>si le tableau est de proportionnalité{
	2 ; 3 ; 5
	6 ; 9 ; 15
}
<Complète>le tableau de proportionnalité{
	2 ; 3 ; ?
	6 ; ? ; 15
}
<Calcule>30 % de 250
<Applique>une augmentation de 5 % à 240
<Applique>une diminution de 12 % à 60
<Calcule>le taux d'évolution de 250 à 280
<Calcule>l'échelle d'un plan où 2 cm représentent 50 m
<Calcule>la vitesse moyenne pour 150 km en 2 h 30 min
<Calcule>la distance parcourue à 80 km/h pendant 1 h 45 min
<Calcule>la durée du trajet de 210 km à 60 km/h
```

Produit en croix rédigé, coefficient multiplicateur pour les évolutions, durées converties dans les deux sens (« 3,5 h = 3 h 30 min »).

### **Mesures et conversions**

```docdg
<Calcule>le périmètre du cercle de rayon 5
<Calcule>l'aire du triangle de base 6 et de hauteur 4
<Calcule>le volume de la boule de rayon 3
<Convertis>3,5 km en m
<Convertis>2500 cm^2 en m^2
<Convertis>3 L en cm^3
```

Dix figures (du carré à la boule), formule affichée, $\pi$ gardé exact ($36\pi \approx 113{,}1$). Les conversions se font par décalage décimal exact et commenté, sur les longueurs, masses, contenances, aires (avec are et hectare) et volumes (le litre dialogue avec le décimètre cube).

### **Nombres et programmes de calcul**

```docdg
<Soit>une fonction f(x) = 2x + 1
<Calcule>1/2 + 1/3
<Simplifie>la fraction 84/60
<Écris>le nombre 45 600 en notation scientifique
<Vérifie>si 456 est divisible par 3
<Effectue>la division euclidienne de 47 par 5
<Calcule>le PGCD de 84 et 60
<Applique>le programme de calcul à 5{
	choisir un nombre
	ajouter 3
	multiplier par 2
	soustraire 4
}
<Exprime>le programme de calcul en fonction de x{
	choisir un nombre
	ajouter 3
	multiplier par 2
	soustraire 4
}
<Calcule>l'image de 3 par f
```

Mise au même dénominateur, inverse pour la division, simplification par le PGCD, critères de divisibilité justifiés (somme des chiffres, chiffre des unités).

### **Transformations du plan**

```docdg
<Soit>les points A, B et C de coordonnées respectives (1 ; 1), (4 ; 1) et (2 ; 3)
<Soit>un vecteur u(2;1)

<Construis>l'image du triangle ABC par la symétrie axiale d'axe l'axe des abscisses
<Construis>l'image du triangle ABC par la symétrie centrale de centre O
<Construis>l'image du triangle ABC par la translation de vecteur u
<Construis>l'image du triangle ABC par la rotation de centre O et d'angle 90 degrés
<Construis>l'image du triangle ABC par l'homothétie de centre O et de rapport 2
```

Cinq transformations sur point, segment, triangle, quadrilatère ou polygone. Chaque action donne les coordonnées des images, puis le tracé sur quadrillage : figure d'origine en bleu, image en rouge avec les labels primes, lignes de construction en pointillé, axe en tirets, centre marqué d'une croix. L'axe accepte aussi `l'axe des ordonnées` et `(PQ)` ; la translation, `qui transforme A en B` ; la rotation, `dans le sens des aiguilles d'une montre`.

---


## La physique-chimie au collège

Le corpus couvre le cycle 4 : 65 énoncés de la cinquième à la troisième.

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

**`<Équilibre>`** ajuste une équation de réaction, coefficients entiers
minimaux — la conservation des atomes, appliquée plutôt que récitée :

```docdg
<Équilibre>l'équation C3H8 + O2 -> CO2 + H2O
```

**Les exemples** `physique2.txt` et `chimie2.txt` montrent un cours rédigé à ce
niveau.

---

## **📚 Ce que le corpus couvre au collège**

Le corpus porte **207 ancrages** répartis sur les quatre années — mathématiques,
physique et chimie confondues. La 3.3 y a comblé des absences centrales :

| Domaine | Ce qui est entré en 3.3 |
|---|---|
| **Nombres relatifs** | le nombre relatif, la somme, la différence (cinquième), le produit, le quotient (quatrième) |
| **Calcul** | les priorités opératoires, l'écriture fractionnaire (sixième), la réduction d'une expression littérale, le test d'une égalité (cinquième) |
| **Puissances** | la puissance d'exposant entier relatif et ses règles de calcul (quatrième) |
| **Algèbre** | développer et factoriser, l'équation du premier degré (quatrième) |
| **Fonctions** | image et antécédent, la représentation graphique (troisième) |
| **Repérage** | la demi-droite graduée (sixième), le repère du plan (cinquième) |
| **Proportionnalité** | l'échelle d'un plan (sixième), l'augmentation et la diminution en pourcentage (cinquième) |
| **Géométrie** | rayon, diamètre et corde, la distance d'un point à une droite (sixième), la hauteur et la médiane d'un triangle (cinquième) |
| **Statistiques** | l'effectif et la fréquence (sixième) |
| **Probabilités** | l'arbre de probabilités (troisième) |

**Le vocabulaire reste celui du supérieur, avec sa glose.** Un document composé
au niveau cinquième compose « deux triangles isométriques (on dit aussi :
superposables) » ; le même énoncé lu en quatrième dit « isométriques », sans
apposition. Le terme écrit dans le corpus n'a jamais changé. Voir
[Ecole.md](Ecole.md) pour le mécanisme, et
[docs/REGLES-CORPUS.md](docs/REGLES-CORPUS.md) pour le lexique complet.

**`exemples/nombres2.txt`** compose au niveau cinquième et montre la glose à
l'œuvre sur une vingtaine d'énoncés.

---
