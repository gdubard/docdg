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
<Soit>les fonctions f(x) = x^2, g(x) = 1/x^2 et s(x) = sin(x)
<Calcule>l'intégrale de f entre 0 et 1
<Détermine>la nature de l'intégrale de g entre 1 et +infini
<Calcule>un équivalent de s en 0
<Détermine>la nature de la série de terme général 1/n^2
```

Intégrales en forme close, impropres avec nature et valeur, équivalents en un point ou en $\pm\infty$ (notation $\sim$), séries avec convergence prouvée et somme quand elle existe ($\pi^2/6$…).

```docdg
<Soit>la matrice M {
	2 ; 1
	1 ; 1
}
<Soit>les vecteurs u(1;0;1), v(0;1;1), a(1;2;0) et b(1;0;0)
<Calcule>le rang de M
<Détermine>le noyau de M
<Détermine>l'image de M
<Calcule>le polynôme caractéristique de M
<Calcule>le polynôme minimal de M
<Trigonalise>M
<Orthonormalise>la famille u et v
<Calcule>le projeté orthogonal de a sur b
```

$\mathrm{Ker}$ et $\mathrm{Im}$ en $\mathrm{Vect}(\dots)$, $\chi_M$ factorisé, $\pi_M$ vérifié par annulation, trigonalisation $M = PTP^{-1}$ (avec signalement quand $T$ est en fait diagonale), Gram-Schmidt exact en dimension quelconque.

```docdg
<Effectue>la division euclidienne de X^3 + 2X - 1 par X^2 + 1
<Calcule>le PGCD de X^3 - 1 et X^2 - 1
<Factorise>X^4 - 1 dans R[X]
<Factorise>X^4 - 1 dans C[X]
```

Les polynômes formels parlent la langue du chapitre : quotient et reste, PGCD unitaire, irréductibles du corps choisi. Les mêmes phrases acceptent les entiers ($47 = 5 \times 9 + 2$).

```docdg
<Soit>une fonction psi(x, y) = x^3 - 3x + y^2

<Calcule>le gradient de psi
<Calcule>la matrice hessienne de psi
<Calcule>la dérivée partielle de psi par rapport à y
<Détermine>les points critiques de psi
```

Les fonctions de plusieurs variables se posent naturellement ; les points critiques viennent avec leur nature par la hessienne (minimum, maximum, point col).

```docdg
<Soit>les fonctions f(x) = x^2, g(x) = x et F(p) = 1/(p^2 + 1)
<Calcule>la série de Fourier de f sur [-pi ; pi] à l'ordre 4
<Calcule>la transformée de Laplace de f
<Calcule>la transformée de Laplace inverse de F
<Calcule>le wronskien de f et g
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
<Trace>la courbe paramétrée x = cos(t)^3 et y = sin(t)^3 pour t dans [0 ; 2*pi]
<Trace>la courbe polaire r = 1 + cos(t) pour t dans [0 ; 2*pi], en rouge
<Trace>la rosace à 4 pétales, en violet       % le moteur choisit le coefficient : cos(2t)
<Trace>la courbe polaire r = cos(3*t), en vert
```

Le repère est isotrope (un cercle reste un cercle), les couleurs sont celles du traceur, et les expressions interpolent les `#` — une rosace peut donc dépendre d'une saisie du lecteur : `r = cos(#n*t)`.

### Les coniques, rédigées

`<Étudie>` prend l'équation générale du second degré — terme croisé compris — et rédige la réduction pas à pas avant de tracer la courbe, ses foyers et son centre :

```docdg
<Étudie>la conique d'équation x^2/9 + y^2/4 = 1
<Étudie>la conique d'équation x^2/4 - y^2 = 1
<Étudie>la conique d'équation y^2 = 4x
<Étudie>la conique d'équation x^2 + xy + y^2 = 3
```

La rédaction suit la méthode : discriminant \\(AC - B^2/4\\), rotation d'angle \\(\\theta\\) et valeurs propres si le terme croisé est présent, centre \\(\\Omega\\), équation réduite, puis les éléments caractéristiques — demi-axes, \\(c\\), excentricité et foyers pour l'ellipse et l'hyperbole (asymptotes comprises), sommet, paramètre \\(p\\), foyer et directrice pour la parabole. Les cas dégénérés (point, droites, ensemble vide) concluent proprement. Le cercle est reconnu comme tel.

---

## **🗻 Les fonctions de deux variables**

Nouveauté de docdg 2.0, pour la deuxième année du supérieur : les nappes, les lignes de niveau, les extremums libres et sous contrainte, les intégrales multiples.

### Les surfaces et les lignes de niveau

Une nappe \\(z = f(x, y)\\) se dessine en perspective cavalière, maillée et ombrée selon l'orientation des facettes ; les lignes de niveau tracent \\(f(x, y) = c\\), étiquetées par leurs valeurs :

```docdg
<Trace>la surface z = x^2 - y^2 pour x dans [-2 ; 2] et y dans [-2 ; 2]
<Trace>la surface z = cos(x)*cos(y) pour x dans [-3 ; 3] et y dans [-3 ; 3], avec 32 mailles
<Trace>les lignes de niveau de z = x^2 + y^2 pour x dans [-2 ; 2] et y dans [-2 ; 2], aux niveaux {1 ; 2 ; 3}
```

Sans liste, neuf niveaux se répartissent d'eux-mêmes entre le minimum et le maximum. Les couleurs sont celles du traceur, et les expressions interpolent les `#` — la nappe peut dépendre d'une saisie du lecteur.

### Les extremums, libres et sous contrainte

Les extremums libres passent par les points critiques et la hessienne — `<Détermine>les points critiques de g` conclut minimum, maximum ou point col. Sous contrainte, docdg introduit le lagrangien, résout \\(\\nabla f = \\lambda\\,\\nabla g\\) avec \\(g = 0\\), et classe les candidats :

```docdg
<Soit>la fonction f(x, y) = x*y
<Détermine>les extremums de f sous la contrainte x^2 + y^2 = 2
```

### Les intégrales multiples

Doubles et triples sur un pavé (Fubini, variable par variable), doubles sur un disque avec le passage en polaires et son jacobien rédigés :

```docdg
<Calcule>l'intégrale double de x*y sur [0 ; 1] × [0 ; 2]
<Calcule>l'intégrale double de x^2 + y^2 sur le disque de rayon 2
<Calcule>l'intégrale triple de x*y*z sur [0 ; 1] × [0 ; 1] × [0 ; 2]
```

---

## **🌀 Le plan complexe et les résidus**

Dernier étage de docdg 2.0, avec un évaluateur complexe intégré (opérations, puissances, `i`, `exp`, `ln`, `sin`, `cos`, `sqrt`) :

```docdg
<Trace>l'image du carré [-1 ; 1] × [-1 ; 1] par w = z^2
<Trace>l'image du cercle unité par w = z + 1/z
<Calcule>les résidus de 1/(z^2 + 1)
```

L'image d'un quadrillage montre la géométrie de la transformation — les deux familles de lignes, bleues et rouges, restent orthogonales là où \\(f\\) est holomorphe de dérivée non nulle ; les singularités coupent proprement les courbes. Les résidus sortent rédigés : pôles, ordres, et \\(\\mathrm{Res}(f, z_0)\\) pour chacun.

---

<a id="les-groupes"></a>

## **🎲 Les lois à densité**

```docdg
<Étudie>la loi de densité f(x) = 3*x^2 sur [0 ; 1]
<Étudie>la loi de densité f(x) = exp(-x) sur [0 ; +infini]
<Calcule>la probabilité d'être entre -1 et 1 pour la loi normale d'espérance 0 et d'écart type 1
<Trace>la densité de la loi normale d'espérance 0 et d'écart type 1
<Trace>l'illustration du théorème central limite avec la somme de 8 dés
```

`<Étudie>` vérifie que l'intégrale vaut 1 (bornes infinies comprises) puis rédige l'espérance et la variance — ou signale que \\(f\\) n'est pas une densité. La loi normale donne ses probabilités exactes (en \\(\\mathrm{erf}\\)) et approchées, et sa cloche se trace. L'illustration du théorème central limite superpose la loi exacte de la somme de \\(n\\) dés (calculée par convolution) et sa densité normale limite — et \\(n\\) peut être une saisie du lecteur.

---


## La physique-chimie en licence

Le corpus couvre la licence 1 à la licence 3 sur dix-sept domaines, avec les
ancrages de classe préparatoire correspondants : 300 énoncés et l'essentiel
des 143 démonstrations. La suite — master 1, master 2 et agrégation — est
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

**Les exemples** `physique4.txt` et `chimie4.txt` montrent un cours rédigé à ce
niveau.
