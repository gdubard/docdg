# docdg — Le lycée

La seconde à la terminale : tronc commun, spécialité et enseignement scientifique, en mathématiques comme en physique-chimie.

> Ce document rassemble ce que docdg apporte à ce niveau.
> Les fonctionnalités communes à tous les niveaux — la syntaxe, les objets,
> les styles, le langage algorithmique, les graphiques — sont décrites dans le
> [README](README.md), qui reste le manuel de référence.

---

## **🎓 Le lycée, couvert**

> De la seconde aux maths expertes : chaque chapitre a sa phrase, en registre de la classe.

```docdg
<Soit>une fonction f(x) = x^3 - 3x^2 + 2

<Étudier>la convexité de f
<Déterminer>les asymptotes de f
<Résoudre>l'équation trigonométrique cos(x) = 1/2
```

La convexité donne $f''$, son tableau de signes en intervalles, les points d'inflexion ; les asymptotes horizontales, obliques et verticales sont énoncées avec leurs équations ; les équations trigonométriques résolues sur $\mathbb{R}$ par les valeurs remarquables.

```docdg
<Soient>les vecteurs u et v de coordonnées respectives (3 ; -2) et (4 ; 6)
<Soit>le plan P d'équation 2x + y - z = 3

<Calculer>le produit scalaire de u et v
<Calculer>la norme de u
<Calculer>l'angle entre u et v
<Étudier>la colinéarité de u et v
<Donner>un vecteur normal de P
<Calculer>la distance du point (1 ; 2 ; 0) au plan P
```

Produit scalaire détaillé (orthogonalité signalée quand il est nul), normes exactes en $k\sqrt{m}$, angles remarquables exacts, colinéarité par déterminant (plan) ou produit vectoriel (espace).

```docdg
<Construire>le graphe G{
	A -> B
	B -> C
	C -> A
}
<Dresser>la matrice d'adjacence de G
<Dénombrer>les chemins de longueur 3 de A à B dans G

<Soit>la matrice M {
	0,5 ; 0,5
	0,2 ; 0,8
}
<Calculer>la puissance 3 de M
<Calculer>l'état stable de M
<Résoudre>l'équation diophantienne 12x + 20y = 8
<Calculer>les racines cinquièmes de l'unité
<Calculer>l'intervalle de fluctuation pour n = 100 et p = 0,3
<Appliquer>l'inégalité de Bienaymé-Tchebychev pour une espérance de 5, une variance de 2 et un écart de 3
```

Les graphes se donnent une arête par ligne (`--` non orienté, `->` orienté) et se tracent seuls ; les chaînes de Markov trouvent leur état stable (l'unique $\pi$ tel que $\pi = \pi M$, matrice stochastique vérifiée) ; les diophantiennes passent par Bézout jusqu'à la solution générale — ou démontrent l'absence de solution.

---


## La physique-chimie au lycée

Le corpus couvre la seconde, la première et la terminale, en tronc commun, en
spécialité et en enseignement scientifique : 206 énoncés, dont 21 démontrés.

**Seconde** — vecteur vitesse, interactions fondamentales, gravitation
universelle, loi de Coulomb, principe d'inertie ; signal sonore, niveau
d'intensité, lentilles, grandissement, réfraction, dispersion, spectres ;
entités chimiques, mole et constante d'Avogadro, masses molaires,
concentrations, structure de l'atome, classification périodique, liaison
covalente, transformations et tableau d'avancement.

**Première spécialité** — champs de gravitation, électrostatique et magnétique,
statique des fluides, Boyle-Mariotte, travail et énergies, théorème de
l'énergie cinétique, premier principe, transferts thermiques, ondes et
photons ; dosages et Beer-Lambert, avancement et titrages, polarité, cristaux,
cohésion et dissolution, chimie organique, oxydoréduction, énergie chimique.

**Terminale spécialité** — lois de Newton, mouvements dans un champ, satellites
et Kepler, Bernoulli, oscillateurs, ondes, interférences, diffraction, effet
Doppler, circuit RC, radioactivité ; spectroscopies infrarouge et par résonance
magnétique nucléaire, cinétique, acides et bases, équilibres, électrochimie,
synthèse organique.

**Enseignement scientifique** — nucléosynthèse, rayonnement solaire,
Stefan-Boltzmann, Wien, albédo, bilan radiatif, effet de serre, Ératosthène,
saisons, son et musique, numérisation ; forçage radiatif, rétroactions,
modèles climatiques, empreinte carbone, induction, transport de l'électricité,
chaînes énergétiques.

### Les équations différentielles, intégrées pas à pas

Décroissance radioactive, charge d'un condensateur, chute avec frottement,
cinétique : docdg intègre toute équation différentielle du programme par la
méthode de `scipy.integrate.solve_ivp` — celle des TP Python —, trace la
solution et donne sa valeur au bout de l'intervalle. L'équation s'écrit
comme elle sort du cours, sans isoler la dérivée :

```docdg
<Résoudre>numériquement l'équation différentielle m v' = m g - k v^2, avec m = 80, g = 9,81, k = 0,25 et v(0) = 0, sur [0 ; 30]
```

Les paramètres se posent dans la phrase (`avec R = 1000, C = 1e-6`) et
peuvent se calculer les uns des autres (`T = 5730, λ = ln(2)/T`). Les exemples
`physique.txt` et `chimie.txt` rédigent ces situations dans leurs
chapitres consacrés au lycée ; le premier trace aussi le spectre d'une note de musique — un fondamental et ses harmoniques, comme en
enseignement scientifique (« son et musique ») :

```docdg
<Tracer>le spectre du signal s(t) = sin(2π·440t) + 0,5sin(2π·880t) + 0,25sin(2π·1320t), échantillonné à 8000 Hz sur [0 ; 0,5], jusqu'à 2000 Hz
```

Le titrage pH-métrique s'exploite de même : docdg interpole les mesures par
une spline, la dérive et repère le volume équivalent au pic de la dérivée
(`chimie.txt`, dernier chapitre) :

```docdg
<Tracer>l'interpolation de pH en fonction de V des données {(0;3) (4;3,8) (8;4,7) (9,5;5,5) (10;7) (10,5;10,4) (12;11,3) (16;11,9)} par une spline cubique, avec sa dérivée
```

L'intervalle de confiance d'une proportion s'obtient en une phrase, avec
l'approximation normale et l'intervalle exact
(`statistiques-probabilites.txt`) :

```docdg
<Calculer>l'intervalle de confiance à 95 % de la proportion pour 42 succès sur 100
```

En NSI, l'algorithme des k plus proches voisins classe un point par le vote
de ses voisins (`algo.txt`) :

```docdg
<Tracer>la classe du point (3 ; 3) par les 3 plus proches voisins parmi {(1;1) A, (2;1) A, (1;2) A, (6;6) B, (7;6) B, (6;7) B, (4;2) A}
```

En sciences de l'ingénieur (`si.txt`), la réponse indicielle d'un moteur ou
d'une suspension se trace avec ses temps caractéristiques :

```docdg
<Tracer>la réponse indicielle de H(p) = K/(1 + τp), avec K = 2 et τ = 0,05
```

La syntaxe complète est dans le [manuel général](README.md), sections « Les
équations différentielles, numériquement », « Le spectre d'un signal »,
« L'interpolation et le lissage » et « Les intervalles de confiance et les
tests statistiques ».

**Les exemples** `physique.txt` et `chimie.txt` montrent un cours rédigé à ce
niveau.
