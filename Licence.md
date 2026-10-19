# docdg — La licence et les classes préparatoires

La licence 1 à la licence 3, PCSI, PC, MPSI et MP : les outils du supérieur et le corpus scientifique de licence.

> Ce document rassemble ce que docdg apporte à ce niveau.
> Les fonctionnalités communes à tous les niveaux — la syntaxe, les objets,
> les styles, le langage algorithmique, les graphiques — sont décrites dans le
> [README](README.md), qui reste le manuel de référence.

---

## **🎓 Le supérieur (CPGE)**

> Le registre des classes préparatoires s'écrit tel quel. Les résultats sont exacts ; les impossibilités renvoient honnêtement au raisonnement à rédiger.

```docdg
<Soient>les fonctions f(x) = x^2, g(x) = 1/x^2 et s(x) = sin(x)
<Calculer>l'intégrale de f entre 0 et 1
<Déterminer>la nature de l'intégrale de g entre 1 et +infini
<Calculer>un équivalent de s en 0
<Déterminer>la nature de la série de terme général 1/n^2
```

Intégrales en forme close, impropres avec nature et valeur, équivalents en un point ou en $\pm\infty$ (notation $\sim$), séries avec convergence prouvée et somme quand elle existe ($\pi^2/6$…).

```docdg
<Soit>la matrice M {
	2 ; 1
	1 ; 1
}
<Soient>les vecteurs u(1;0;1), v(0;1;1), a(1;2;0) et b(1;0;0)
<Calculer>le rang de M
<Déterminer>le noyau de M
<Déterminer>l'image de M
<Calculer>le polynôme caractéristique de M
<Calculer>le polynôme minimal de M
<Trigonaliser>M
<Orthonormaliser>la famille u et v
<Calculer>le projeté orthogonal de a sur b
```

$\mathrm{Ker}$ et $\mathrm{Im}$ en $\mathrm{Vect}(\dots)$, $\chi_M$ factorisé, $\pi_M$ vérifié par annulation, trigonalisation $M = PTP^{-1}$ (avec signalement quand $T$ est en fait diagonale), Gram-Schmidt exact en dimension quelconque.

```docdg
<Effectuer>la division euclidienne de X^3 + 2X - 1 par X^2 + 1
<Calculer>le PGCD de X^3 - 1 et X^2 - 1
<Factoriser>X^4 - 1 dans R[X]
<Factoriser>X^4 - 1 dans C[X]
```

Les polynômes formels parlent la langue du chapitre : quotient et reste, PGCD unitaire, irréductibles du corps choisi. Les mêmes phrases acceptent les entiers ($47 = 5 \times 9 + 2$).

```docdg
<Soit>une fonction psi(x, y) = x^3 - 3x + y^2

<Calculer>le gradient de psi
<Calculer>la matrice hessienne de psi
<Calculer>la dérivée partielle de psi par rapport à y
<Déterminer>les points critiques de psi
```

Les fonctions de plusieurs variables se posent naturellement ; les points critiques viennent avec leur nature par la hessienne (minimum, maximum, point col).

```docdg
<Soient>les fonctions f(x) = x^2, g(x) = x et F(p) = 1/(p^2 + 1)
<Calculer>la série de Fourier de f sur [-pi ; pi] à l'ordre 4
<Calculer>la transformée de Laplace de f
<Calculer>la transformée de Laplace inverse de F
<Calculer>le wronskien de f et g
```

Le pont vers la physique : séries de Fourier tronquées à l'ordre voulu, transformées de Laplace en variable $p$ (convention des physiciens) avec la définition intégrale affichée, wronskien avec conclusion rigoureuse (non nul $\Rightarrow$ famille libre ; nul : « ce qui, en général, ne suffit pas à conclure »).

---

---

<a id="la-physique-chimie"></a>

## **➰ Les courbes et les coniques**

Nouveauté de docdg 2.0, dans le prolongement du traceur de fonctions : les courbes paramétrées, les courbes polaires, et les coniques rédigées puis tracées.

### Paramétrées et polaires

Une courbe paramétrée se donne par ses deux coordonnées, une polaire par son rayon — l'intervalle de \\(t\\) accepte `pi` (`[0 ; 2*pi]`), et se tait pour valoir \\([0 ; 2\\pi]\\) :

```docdg
<Tracer>la courbe paramétrée x = cos(t)^3 et y = sin(t)^3 pour t dans [0 ; 2*pi]
<Tracer>la courbe polaire r = 1 + cos(t) pour t dans [0 ; 2*pi], en rouge
<Tracer>la rosace à 4 pétales, en violet
<Tracer>la courbe polaire r = cos(3*t), en vert
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `<Tracer>la rosace à 4 pétales, en violet` | le moteur choisit le coefficient : cos(2t) |


Le repère est isotrope (un cercle reste un cercle), les couleurs sont celles du traceur, et les expressions interpolent les `#` — une rosace peut donc dépendre d'une saisie du lecteur : `<Tracer>la rosace à #n pétales`. Écrire `r = cos(#n*t)` à la place donnerait deux fois trop de pétales dès que la saisie est paire — c'est le moteur qui doit choisir le coefficient.

### Les coniques, rédigées

`<Étudier>` prend l'équation générale du second degré — terme croisé compris — et rédige la réduction pas à pas avant de tracer la courbe, ses foyers et son centre :

```docdg
<Étudier>la conique d'équation x^2/9 + y^2/4 = 1
<Étudier>la conique d'équation x^2/4 - y^2 = 1
<Étudier>la conique d'équation y^2 = 4x
<Étudier>la conique d'équation x^2 + xy + y^2 = 3
```

La rédaction suit la méthode : discriminant \\(AC - B^2/4\\), rotation d'angle \\(\\theta\\) et valeurs propres si le terme croisé est présent, centre \\(\\Omega\\), équation réduite, puis les éléments caractéristiques — demi-axes, \\(c\\), excentricité et foyers pour l'ellipse et l'hyperbole (asymptotes comprises), sommet, paramètre \\(p\\), foyer et directrice pour la parabole. Les cas dégénérés (point, droites, ensemble vide) concluent proprement. Le cercle est reconnu comme tel.

---

## **🗻 Les fonctions de deux variables**

Nouveauté de docdg 2.0, pour la deuxième année du supérieur : les nappes, les lignes de niveau, les extremums libres et sous contrainte, les intégrales multiples.

### Les surfaces et les lignes de niveau

Une nappe \\(z = f(x, y)\\) se dessine en perspective cavalière, maillée et ombrée selon l'orientation des facettes ; les lignes de niveau tracent \\(f(x, y) = c\\), étiquetées par leurs valeurs :

```docdg
<Tracer>la surface z = x^2 - y^2 pour x dans [-2 ; 2] et y dans [-2 ; 2]
<Tracer>la surface z = cos(x)*cos(y) pour x dans [-3 ; 3] et y dans [-3 ; 3], avec 32 mailles
<Tracer>les lignes de niveau de z = x^2 + y^2 pour x dans [-2 ; 2] et y dans [-2 ; 2], aux niveaux {1 ; 2 ; 3}
```

Sans liste, neuf niveaux se répartissent d'eux-mêmes entre le minimum et le maximum. Les couleurs sont celles du traceur, et les expressions interpolent les `#` — la nappe peut dépendre d'une saisie du lecteur.

### Les extremums, libres et sous contrainte

Les extremums libres passent par les points critiques et la hessienne — `<Déterminer>les points critiques de g` conclut minimum, maximum ou point col. Sous contrainte, docdg introduit le lagrangien, résout \\(\\nabla f = \\lambda\\,\\nabla g\\) avec \\(g = 0\\), et classe les candidats :

```docdg
<Soit>la fonction f(x, y) = x*y
<Déterminer>les extremums de f sous la contrainte x^2 + y^2 = 2
```

### Les intégrales multiples

Doubles et triples sur un pavé (Fubini, variable par variable), doubles sur un disque avec le passage en polaires et son jacobien rédigés :

```docdg
<Calculer>l'intégrale double de x*y sur [0 ; 1] × [0 ; 2]
<Calculer>l'intégrale double de x^2 + y^2 sur le disque de rayon 2
<Calculer>l'intégrale triple de x*y*z sur [0 ; 1] × [0 ; 1] × [0 ; 2]
```

---

## **🌀 Le plan complexe et les résidus**

Dernier étage de docdg 2.0, avec un évaluateur complexe intégré (opérations, puissances, `i`, `exp`, `ln`, `sin`, `cos`, `sqrt`) :

```docdg
<Tracer>l'image du carré [-1 ; 1] × [-1 ; 1] par w = z^2
<Tracer>l'image du cercle unité par w = z + 1/z
<Calculer>les résidus de 1/(z^2 + 1)
```

L'image d'un quadrillage montre la géométrie de la transformation — les deux familles de lignes, bleues et rouges, restent orthogonales là où \\(f\\) est holomorphe de dérivée non nulle ; les singularités coupent proprement les courbes. Les résidus sortent rédigés : pôles, ordres, et \\(\\mathrm{Res}(f, z_0)\\) pour chacun.

---

<a id="les-groupes"></a>

## **🎲 Les lois à densité**

```docdg
<Étudier>la loi de densité f(x) = 3*x^2 sur [0 ; 1]
<Étudier>la loi de densité f(x) = exp(-x) sur [0 ; +infini]
<Calculer>la probabilité d'être entre -1 et 1 pour la loi normale d'espérance 0 et d'écart type 1
<Tracer>la densité de la loi normale d'espérance 0 et d'écart type 1
<Tracer>l'illustration du théorème central limite avec la somme de 8 dés
```

`<Étudier>` vérifie que l'intégrale vaut 1 (bornes infinies comprises) puis rédige l'espérance et la variance — ou signale que \\(f\\) n'est pas une densité. La loi normale donne ses probabilités exactes (en \\(\\mathrm{erf}\\)) et approchées, et sa cloche se trace. L'illustration du théorème central limite superpose la loi exacte de la somme de \\(n\\) dés (calculée par convolution) et sa densité normale limite — et \\(n\\) peut être une saisie du lecteur.

---


## La physique-chimie en licence

Le corpus couvre la licence 1 à la licence 3 sur dix-sept domaines, avec les
ancrages de classe préparatoire correspondants : 313 énoncés et 98
démonstrations. La suite — master 1, master 2 et agrégation — est
décrite dans [Master et agrégation](MasterAgregation.md).

**Physique** — mécanique du point, du solide et analytique jusqu'à Lagrange et
Hamilton ; électromagnétisme de l'électrostatique aux équations de Maxwell,
aux ondes et à l'effet de peau ; thermodynamique, machines, transferts ;
ondes, acoustique, dispersion ; optique géométrique et ondulatoire,
interférences, diffraction, polarisation ; mécanique quantique des postulats à
l'atome d'hydrogène ; physique statistique ; mécanique des fluides ;
relativité restreinte.

**Chimie** — atomistique et orbitales moléculaires ; cinétique et mécanismes ;
solutions aqueuses, complexation et précipitation ; électrochimie et diagrammes
potentiel-pH ; chimie organique, stéréochimie et mécanismes ; chimie du solide
et théorie des bandes ; thermodynamique chimique et diagrammes binaires ;
chimie de coordination ; chimie analytique instrumentale.

**L'analyse dimensionnelle** vérifie 233 relations à la compilation, opérateurs
différentiels compris : `div(E) = rho / epsilon_0`,
`lap(E) - d2_dt2(E) / c^2 = 0`, `hbar * d_dt(psi) = hbar^2 * lap(psi) / m + V * psi`.

**Les exemples** `physique.txt` et `chimie.txt` montrent un cours rédigé à ce
niveau.

### Les systèmes dynamiques, intégrés numériquement

Pendule pesant aux grandes amplitudes, oscillateurs amorti et de Van der Pol,
Lotka-Volterra, Lorenz, réactions successives, cinétique raide de Robertson :
docdg confie toute équation ou tout système à `scipy.integrate.solve_ivp`,
embarqué — RK45 par défaut, `par la méthode de Radau`, `BDF` ou `LSODA` pour
les problèmes raides, tolérances réglables dans la phrase.

```docdg
<Résoudre>numériquement l'équation différentielle x'' - μ(1 - x^2)x' + x = 0, avec μ = 2, x(0) = 0,1 et x'(0) = 0, sur [0 ; 30], le portrait de phase
```

Un système s'écrit dans un bloc, une équation `x' = …` par ligne ; les
tracés donnent l'évolution temporelle avec légende, une trajectoire
(`en traçant z en fonction de x`) ou le portrait de phase. Les exemples
`physique.txt`, `chimie.txt` et `analyse.txt` rédigent ces problèmes dans
leur dernier chapitre ; la syntaxe complète est dans le
[manuel général](README.md).

Des conditions aux deux bouts de l'intervalle font un problème aux limites,
résolu par `scipy.integrate.solve_bvp` : ailette de refroidissement
(`physique.txt`), chaînette (`analyse.txt`). `jusqu'à ce que …` arrête
l'intégration sur un événement — temps de vol, apogée (`physique.txt`).

### Signaux et spectres

`<Tracer>le spectre du signal …` calcule la transformée de Fourier rapide par
`scipy.fft` et relève les raies : harmoniques d'un signal carré, bandes
latérales d'une modulation d'amplitude, effet d'une fenêtre de Hann ou de
Blackman sur les fuites spectrales.

```docdg
<Tracer>le spectre du signal s(t) = 4/π (sin(2π f t) + sin(6π f t)/3 + sin(10π f t)/5), échantillonné à 2 kHz sur [0 ; 1], avec f = 50, jusqu'à 300 Hz
```

### Géométrie algorithmique et classification

Enveloppe convexe, paire la plus proche, triangulation de Delaunay et
diagramme de Voronoï pour l'informatique des classes préparatoires
(`algo.txt`) ; k-moyennes et classification hiérarchique avec son
dendrogramme pour l'analyse de données (`statistiques-probabilites.txt`).

```docdg
<Tracer>le dendrogramme des points {(0;0) (1;0,2) (0,3;1) (5;5) (5,5;4,8) (4,9;5,6) (9;0) (9,5;0,4)} en 3 groupes
```

### Constantes physiques et fonctions spéciales

Les constantes CODATA 2022, exactes ou mesurées avec leur incertitude
(`physique.txt`) ; les fonctions de Bessel et leurs zéros pour les modes
d'une membrane (`si.txt`) ; les polynômes orthogonaux, Γ, ζ, Airy et les
intégrales elliptiques (`analyse.txt`) ; la période exacte du pendule aux
grandes amplitudes, confrontée à l'intégration numérique (`physique.txt`).

```docdg
<Calculer>la période du pendule simple d'amplitude 2,5 rad, avec L = 1 et g = 9,81
```

### Algèbre linéaire numérique

Décompositions LU (avec pivot), QR et de Cholesky, valeurs singulières,
conditionnement, exponentielle de matrice, valeurs propres numériques :
`scipy.linalg` complète le calcul exact (`algebre.txt`). La matrice de
Hilbert d'ordre 4 y montre ce que coûte un mauvais conditionnement.

```docdg
<Soit>la matrice J {
	0 ; -1
	1 ; 0
}
<Calculer>l'exponentielle de J
```

### Intégrales multiples numériques

À côté du calcul exact, `numériquement` confie une intégrale double ou triple
à `scipy.integrate` ; une borne peut dépendre des variables précédentes, ce
qui décrit tout domaine élémentaire (`analyse.txt`).

```docdg
<Calculer>numériquement l'intégrale double de exp(-(x^2 + y^2)) sur [0 ; 1] × [0 ; 1]
```

### Statistique inférentielle

Intervalles de confiance, tests de Student (un échantillon, Welch, séries
appariées), du \(\chi^2\) d'adéquation et d'indépendance, de normalité de
Shapiro-Wilk : `scipy.stats` rédige les hypothèses, la p-valeur et la
décision, et trace la zone de rejet (`statistiques-probabilites.txt`).

```docdg
<Calculer>le test du khi-deux d'indépendance du tableau {20 ; 30 ; 25 | 15 ; 25 ; 35}
```

### Interpolation : Lagrange et les splines

Polynôme de Lagrange, splines cubique et naturelle, interpolations monotone
(PCHIP) et d'Akima, spline de lissage : `scipy.interpolate` au service du
cours d'analyse numérique. Le phénomène de Runge se montre en une phrase, et
la spline cubique sur les mêmes points en une autre (`analyse.txt`).

```docdg
<Tracer>l'interpolation des données {(-1;0,0385) (-0,6;0,1) (-0,2;0,5) (0;1) (0,2;0,5) (0,6;0,1) (1;0,0385)} par le polynôme de Lagrange
```

### Sciences de l'ingénieur

Réponses indicielle et impulsionnelle avec leurs caractéristiques (dépassement,
temps de montée, temps de réponse à 5 %), diagrammes de Bode, de Nyquist et de
Black avec les marges de stabilité : l'automatique des classes PTSI et PT se
traite en phrases, comme la mécanique vibratoire et le filtrage (`si.txt`) ;
`si.txt` pousse jusqu'aux modes propres et à la flèche d'une poutre.

```docdg
<Tracer>le diagramme de Nyquist de T(p) = K/(p(1 + 0,1p)(1 + 0,01p)), avec K = 20, entre 0,1 et 1000 rad/s
```

### Optimisation et ajustement

La programmation linéaire se résout par `scipy.optimize.linprog`, avec le
polygone des contraintes à deux variables ; l'ajustement d'un modèle à des
mesures, par `scipy.optimize.curve_fit`, donne chaque paramètre avec son
incertitude-type — la présentation attendue en travaux pratiques.

```docdg
<Résoudre>le programme linéaire : minimiser C = 2a + 3b sous les contraintes a + b ≥ 5, a - b ≤ 1, b ≤ 4, a ≥ 0 et b ≥ 0
```

### Filtrage et asservissements

Le diagramme de Bode d'une fonction de transfert — gain statique, coupure à
−3 dB, résonance, marge de phase d'une boucle ouverte — et le filtrage
numérique d'un signal (Butterworth ou Tchebychev, passe-bas, passe-haut,
passe-bande, coupe-bande, sans déphasage ou en temps réel) sortent de
`scipy.signal`. C'est le cœur des sciences de l'ingénieur et de
l'électronique en classes préparatoires.

```docdg
<Tracer>le diagramme de Bode de T(p) = 10/(p(1 + p)(1 + 0,1p)), entre 0,01 et 1000 rad/s
```

La marge de phase de cette boucle ouverte, à peine plus d'un degré,
annonce un système bouclé tout juste stable.
