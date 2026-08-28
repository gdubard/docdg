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

<Étudie>la convexité de f
<Détermine>les asymptotes de f
<Résous>l'équation trigonométrique cos(x) = 1/2
```

La convexité donne $f''$, son tableau de signes en intervalles, les points d'inflexion ; les asymptotes horizontales, obliques et verticales sont énoncées avec leurs équations ; les équations trigonométriques résolues sur $\mathbb{R}$ par les valeurs remarquables.

```docdg
<Soit>les vecteurs u et v de coordonnées respectives (3 ; -2) et (4 ; 6)
<Soit>le plan P d'équation 2x + y - z = 3

<Calcule>le produit scalaire de u et v
<Calcule>la norme de u
<Calcule>l'angle entre u et v
<Étudie>la colinéarité de u et v
<Donne>un vecteur normal de P
<Calcule>la distance du point (1 ; 2 ; 0) au plan P
```

Produit scalaire détaillé (orthogonalité signalée quand il est nul), normes exactes en $k\sqrt{m}$, angles remarquables exacts, colinéarité par déterminant (plan) ou produit vectoriel (espace).

```docdg
<Construis>le graphe G{
	A -> B
	B -> C
	C -> A
}
<Dresse>la matrice d'adjacence de G
<Dénombre>les chemins de longueur 3 de A à B dans G

<Soit>la matrice M {
	0,5 ; 0,5
	0,2 ; 0,8
}
<Calcule>la puissance 3 de M
<Calcule>l'état stable de M
<Résous>l'équation diophantienne 12x + 20y = 8
<Calcule>les racines cinquièmes de l'unité
<Calcule>l'intervalle de fluctuation pour n = 100 et p = 0,3
<Applique>l'inégalité de Bienaymé-Tchebychev pour une espérance de 5, une variance de 2 et un écart de 3
```

Les graphes se donnent une arête par ligne (`--` non orienté, `->` orienté) et se tracent seuls ; les chaînes de Markov trouvent leur état stable (l'unique $\pi$ tel que $\pi = \pi M$, matrice stochastique vérifiée) ; les diophantiennes passent par Bézout jusqu'à la solution générale — ou démontrent l'absence de solution.

---


## La physique-chimie au lycée

Le corpus couvre la seconde, la première et la terminale, en tronc commun, en
spécialité et en enseignement scientifique : 215 énoncés.

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

**Les exemples** `physique3.txt` et `chimie3.txt` montrent un cours rédigé à ce
niveau.
