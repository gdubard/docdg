# *docdg 3.4*

**docdg** is a Rust document class built around a French-only prose
tag language (no anglicisms). Users write a single `.docdg` file where every
command reads as a natural French sentence; the Rust engine compiles it
directly to HTML and PDF, with professional typographic quality. It covers
layout frames and grids, tables, images, a math mini-language, function
plotting, plane and solid geometry, statistics/probability diagrams,
physics and chemistry, and long-form publishing (chapters, title page,
cross-references, bibliography).

**Author / Maintainer:** Gérard Dubard — docdg69@gmail.com
**License:** proprietary freeware — see [CLUF.md](CLUF.md)
**Repository:** https://github.com/gdubard/docdg

---

**Des fiches d'exercices prêtes à imprimer, sans écrire de code.**

> *Un petit langage de balises, cohérent, pour les documents qu'un enseignant fabrique réellement.*

[![Version](https://img.shields.io/badge/version-3.4-2980b9?style=flat-square)](CHANGELOG.md)
[![Engine](https://img.shields.io/badge/moteur-Rust-E95420?style=flat-square)](https://www.rust-lang.org/)
[![Standalone](https://img.shields.io/badge/distribution-autonome-27ae60?style=flat-square)](https://github.com/gdubard/docdg)
[![Licence](https://img.shields.io/badge/licence-propriétaire_freeware-8e44ad?style=flat-square)](CLUF.md)

**[Journal des versions](CHANGELOG.md)**

---


## Avant de commencer : installation

**docdg est une application autonome** — aucune distribution TeX, aucun éditeur spécialisé n'est requis. Néanmoins, pour exploiter tout son potentiel, l'installation de Python et de la bibliothèque SymPy est requise. En effet, ce moteur externe prend en charge l'ensemble des calculs automatiques avancés (tableau de variations, dérivées, primitives, zéros de fonctions, lois de probabilité).

**Télécharger docdg.** Récupérer le binaire correspondant à votre système sur la page des versions du dépôt :

```
https://github.com/gdubard/docdg/releases
```

Sous Windows, l'installeur configure le `PATH` automatiquement. Sous macOS/Linux, rendre le binaire exécutable (`chmod +x docdg`) et le placer dans un dossier du `PATH`.

**Installer Python (facultatif, pour les calculs formels).** Le calcul formel et numérique avancé repose sur **Python 3 avec SymPy**. docdg l'appelle automatiquement dès qu'il le détecte — aucune option à activer. Récupérer l'installeur sur **[python.org](https://www.python.org/downloads/)** (sous Windows, cocher **« Add Python to PATH »**), puis dans un terminal :

```
python3 -m pip install sympy
```

Sous Windows : `py -m pip install sympy`. docdg essaie dans l'ordre `python3`, `python` puis `py`, et garde la première qui répond.

Sans SymPy, docdg fonctionne intégralement avec son moteur de calcul interne — seuls les calculs automatiques (variations, dérivées, zéros, primitives, lois de probabilité) demandent le moteur externe.

## **📚 Sommaire détaillé**

### Les guides par niveau

| document | contenu |
|---|---|
| [Ecole.md](Ecole.md) | lignes Seyès, écriture cursive, premiers calculs |
| [College.md](College.md) | cycles 3 et 4, corpus de mathématiques et de physique-chimie |
| [Lycee.md](Lycee.md) | seconde à terminale, spécialité et enseignement scientifique |
| [Licence.md](Licence.md) | licence 1 à 3, classes préparatoires, outils du supérieur |
| [MasterAgregation.md](MasterAgregation.md) | master, agrégation, rédaction longue et publication |

### Le catalogue

| document | contenu |
|---|---|
| [Redaction.md](Redaction.md) | les 1373 énoncés et les 569 démonstrations du corpus, par matière et par niveau, avec la phrase qui les appelle — plus les 101 démonstrations classiques servies par `<Montre>` |

### Les contrôles

Quatre commandes tiennent le dépôt d'aplomb, et la CI les rejoue à chaque poussée :

| commande | ce qu'elle vérifie |
|---|---|
| `cargo app corpus` | le corpus : références, cycles, monotonie des niveaux, homogénéité des relations |
| `cargo app couverture` | que le moteur, les manuels et les exemples disent la même chose — 278 entrées confrontées, table par table |
| `cargo app recensement` | l'état du corpus : ancrages par niveau, énoncés par domaine, et les creux qui restent à nourrir — `--redaction` réécrit [Redaction.md](Redaction.md) |
| `cargo run -p docdg-bench --bin exemples` | que les 52 exemples composent, et que le compte de marques d'erreur voulues n'a pas bougé |

`couverture` lit les tables du moteur dans le code source, puis cherche chaque
entrée dans un exemple **et** dans un manuel. Une entrée qui doit rester hors
couverture se déclare dans `xtask/src/couverture.rs`, avec sa raison — jamais
en silence.

`Redaction.md` est **engendré depuis le corpus** : il ne peut pas en diverger,
et chacune de ses lignes est un appel qui fonctionne tel quel.

Le présent document reste le **manuel de référence** : syntaxe, objets,
styles, langage algorithmique, graphiques et démonstrations y sont décrits une
fois pour tous les niveaux.

### La référence

1. [Qu'est-ce que docdg ?](#-quest-ce-que-docdg-)
2. [Ce que la 3.4 apporte](#-ce-que-la-34-apporte) — le corpus complet, l'école élémentaire, la vitesse
2. [Ce que la 2.9 apporte](#-ce-que-la-29-apporte) — le corpus de physique-chimie et l'analyse dimensionnelle
3. [Ce que la 2.8 apporte](#-ce-que-la-28-apporte) — le corpus mathématique, de la sixième à l'agrégation
3. [Ce que la 2.7 apporte](#-ce-que-la-27-apporte) — environnements numérotés, frise chronologique
3. [Fonctionnalités clés](#-fonctionnalités-clés)
4. [Pourquoi docdg ?](#-pourquoi-docdg-)
5. [Ce dont vous avez besoin](#ce-dont-vous-avez-besoin)
6. [Premiers documents](#-premiers-documents)
7. [La syntaxe](#-la-syntaxe)
8. [Les objets](#-les-objets) — cadres, tableaux, images, listes, grilles, sections
9. [Les actions](#les-actions) — `Soit`, `Trace` : figures, repère, solides, mode analytique
10. [Les démonstrations](#-les-démonstrations) — les dix raisonnements, la preuve automatique, la bibliothèque
11. [Les saisies interactives](#les-saisies-interactives) — `<Saisis>`, blocage typé, alternative `si … sinon`, le document vivant
12. [Les styles et la mise en forme](#-les-styles-et-la-mise-en-forme)
13. [Le langage algorithmique](#-le-langage-algorithmique) — types, conteneurs, primitives, chaînes, fonctions, p-uplets
14. [Les graphiques de fonctions](#-les-graphiques-de-fonctions)
15. [Les courbes et les coniques](#-les-courbes-et-les-coniques) — paramétrées, polaires, coniques rédigées et tracées
16. [Les fonctions de deux variables](#-les-fonctions-de-deux-variables) — surfaces, lignes de niveau, Lagrange, intégrales multiples
17. [Le plan complexe et les résidus](#-le-plan-complexe-et-les-résidus) — images de domaines, transformations holomorphes, pôles et résidus
18. [Les groupes](#les-groupes) — tables de ℤ/nℤ, générateurs, cycles et signature
19. [Les lois à densité](#-les-lois-à-densité) — densités, loi normale, théorème central limite
20. [Les mathématiques](#-les-mathématiques)
21. [La géométrie](#-la-géométrie)
22. [La géométrie dans l'espace](#-la-géométrie-dans-lespace) — solides, patrons, repère de l'espace, droites et plans, positions relatives
23. [Les statistiques](#-les-statistiques) — diagrammes, arbres de probabilités, droite graduée
24. [L'écriture sur des lignes](#lecriture-sur-des-lignes) — la réglure Seyès et la cursive, pour l'école élémentaire
25. [Le collège, rédigé](#-le-collège-rédigé) — Pythagore, Thalès, trigonométrie, proportionnalité, transformations
26. [Le lycée, couvert](#-le-lycée-couvert) — convexité, asymptotes, espace, graphes, Markov, diophantiennes
27. [Le supérieur (CPGE)](#-le-supérieur-cpge) — séries, réduction, Gram-Schmidt, polynômes formels, Fourier, Laplace
28. [La physique-chimie](#la-physique-chimie) — équations, masses molaires, avancement, unités, constantes, incertitudes
29. [La frise chronologique](#la-frise-chronologique) — l'histoire entre en scène : dates, titres, détails
30. [Rédiger un article, une thèse](#-rédiger-un-article-une-thèse) — chapitres, page de titre, renvois, bibliographie
31. [Les exemples, par niveau](#les-exemples-par-niveau)
32. [Les documents complexes](#-les-documents-complexes)
33. [Comprendre les erreurs et les cas particuliers](#-comprendre-les-erreurs-et-les-cas-particuliers)
34. [Référence complète](#-référence-complète) — couleurs et options de classe
35. [Bonnes pratiques](#-bonnes-pratiques)
36. [Le calcul scientifique étendu](#-le-calcul-scientifique-étendu) — SymPy : ce que débloque le second moteur
37. [Les tournures et le placement](#les-tournures-et-le-placement) — synonymes, bornes naturelles, placement en langage naturel

*Annexes, index alphabétique et index thématique : pas encore rédigés dans cette version, à venir.*

---

## **📖 Qu'est-ce que docdg ?**

**docdg est un langage de composition documentaire.**  
Il permet de **créer simplement des documents de grande qualité** grâce à un **moteur de composition Rust** qui produit directement du HTML et du PDF.

**docdg prend en charge toute la composition** : vous décrivez le document en français, il produit le PDF — sans intermédiaire, sans passer par un autre système.

### **🎯 L'objectif double de docdg**

- ✅ **Produire rapidement de beaux documents** ;
- ✅ **Comprendre progressivement la composition documentaire**.

### **👥 À qui s'adresse docdg ?**

docdg s'adresse **à tous** : Collégiens, Lycéens, Étudiants, Enseignants, Chercheurs, Ingénieurs, Auteurs de documentation, et plus généralement à toute personne souhaitant produire des documents scientifiques, techniques ou pédagogiques.

### **📖 Comment lire ce manuel**

Ce manuel suit une **progression logique** :

1. Les premiers chapitres posent les fondations (le premier document, la syntaxe, les quatre familles de commandes).
2. Les chapitres suivants explorent chaque domaine (styles, tableaux, images, dessins, mathématiques, géométrie, statistiques).
3. Les derniers chapitres forment un **appareil de référence** : la liste complète des commandes, des annexes de tableaux, un index alphabétique et un index thématique.

**Chaque construction est présentée de la même façon :**

1. **L'écriture docdg** (ce que **vous écrivez**) ;
2. **Le HTML et PDF généré** (ce que docdg **produit pour vous**) ;
3. **Une explication** de ce que fait ce code.

> 💡 **Vous pouvez lire ce manuel dans l'ordre**, ou **sauter directement au chapitre qui vous intéresse** et revenir aux fondations en cas de besoin.

### **📜 Une convention de lecture**

Dans tout ce manuel, un bloc marqué **`docdg`** est ce que **vous écrivez**. C'est la seule syntaxe à connaître : docdg génère le HTML et le PDF pour vous, en coulisse.

---

## **🆕 Ce que la 3.4 apporte**

**Le corpus ne déclare plus aucune dette.** 1 373 énoncés et 569 démonstrations,
et pas une seule promesse en attente : chaque énoncé annoncé démontré porte sa
démonstration, rédigée au niveau exact où il est exigible. Les trois cent huit
lacunes que la 3.0 assumait sont soldées — la seconde, la première et la
terminale, les trois années de licence, puis le master et l'agrégation en
mathématiques, en physique et en chimie.

**L'école élémentaire entre dans le corpus.** Du CP au CM2, en mathématiques
comme en sciences : numération de position, compléments à dix, tables de
multiplication, fractions et écriture décimale, aires par pavage, symétrie
axiale, circuits électriques, états de l'eau. Rien ne s'y démontre — à l'école,
tout est admis, et le corpus le dit ainsi. Le collège s'enrichit du même
mouvement de quinze énoncés et deux démonstrations : nombres relatifs,
priorités opératoires, repère du plan, et la propriété qu'une médiane partage
le triangle en deux triangles de même aire, démontrée en cinquième comme le
demande le programme du cycle 4.

**`cargo app recensement`.** L'état du corpus se mesure au lieu de se déclarer :
les ancrages niveau par niveau, les énoncés domaine par domaine, et les creux
nommés — les niveaux les moins fournis, les domaines qui tiennent en moins de
trois énoncés. Avec `--redaction`, la commande réécrit `Redaction.md` depuis les
données : le catalogue ne peut plus diverger de la base.

**La vitesse et la mémoire.** Le cache des segments retient l'environnement
sortant de chacun et la césure déjà composée, si bien qu'une frappe ne
recompose plus que ce qu'elle touche ; `finalise` ne recopie plus le document
que deux fois au lieu de quatre ; le bassin de calcul formel se sème à
l'ouverture de l'application plutôt qu'au premier calcul ; la cursive Marelle
ne voyage plus dans chaque document composé ; et une image ne se relit que si
elle a changé sur le disque.

**SymPy seul.** SciPy n'est plus employé ni requis : le calcul formel comme le
calcul numérique avancé passent par SymPy, et l'installation se réduit à
`python3 -m pip install sympy`.

**L'écriture s'arrête au dernier carreau entier.** Sur la réglure Seyès, la
colonne ne contient pas un nombre entier de carreaux : le reste du bord droit
était mordu par le texte. Il ne l'est plus.

---

## **🆕 Ce que la 3.2 apporte**

**Les divisions de l'ouvrage.** Sept niveaux, du tome à la sous-sous-section :
`<tome>`, `<livre>` et `<partie>` comptent en romain ou en arabe à la façon
des ouvrages composés, ouvrent leur page, entrent dans la table des matières —
et ne se remettent jamais à zéro. La série `publication1` à `publication5`
montre un document par usage réel : l'article, l'exposé, le traité de
typographie, la thèse, l'essai.

**La page de titre se compose, elle ne se déclare pas.** Des styles nommés
(`au centre`, une taille, `petites capitales`), `<Insère l'image …>` pour
l'emblème, `<page suivante>` pour refermer la couverture : le même geste que
partout ailleurs, sans balise spéciale ni clés d'identité.

**Une notion, un mot — appliqué au langage lui-même.** Les clés du bloc
`document { }` ne s'écrivent qu'en toutes lettres accentuées, les alignements
ne connaissent que `à gauche`, `au centre`, `à droite` et `justifié`, le bloc
s'appelle `document` et c'est le seul. Chaque forme retirée est refusée **en
nommant son remplacement** : rien ne casse en silence.

**L'atelier.** Rechercher et remplacer dans la source (Ctrl+F — recherche
exacte, casse et accents compris, remplacement un par un ou d'un coup,
annulable d'un Ctrl+Z), le chemin du fichier en cours affiché dans le
cartouche Fichier, et des dialogues qui proposent `nomdufichier.txt` et
`nomdufichier.pdf`.

## **🆕 Ce que la 3.0 apporte**

**Le master et l'agrégation en physique-chimie.** 163 énoncés et 55
démonstrations portent le corpus à **1310 énoncés et 299 démonstrations**, de la
cinquième à l'agrégation, dans une base unique aux trois matières. Neuf domaines
neufs : mécanique quantique avancée, physique statistique du master, matière
condensée, électrodynamique covariante, physique nucléaire et des particules,
relativité générale et cosmologie, chimie quantique, spectroscopie moléculaire,
catalyse et organométalliques, cinétique du master et photochimie, chimie
macromoléculaire.

Le graphe traverse les matières dans les deux sens : les forces de dispersion se
démontrent par la perturbation au second ordre du corpus de physique, la chaîne
gaussienne d'un polymère par la marche aléatoire de l'équation de Langevin, et
le théorème de Bloch par le théorème spectral du corpus mathématique. Rien n'est
réécrit.

**L'analyse dimensionnelle.** Chaque relation du corpus est vérifiée en
homogénéité au moment de la compilation : sept dimensions de base, exposants
en douzièmes, opérateurs différentiels et intégraux. Les équations de Maxwell,
de Schrödinger et de d'Alembert passent le même contrôle que les autres. Une
loi de vitesse dont la constante ne cadre pas avec l'ordre est refusée.

**Les hypothèses et le domaine de validité.** Un énoncé de physique n'a de sens
que sous des conditions : `<Énonce>` les compose désormais avec la formule et
le domaine de validité, là où un énoncé de mathématiques se suffit de son
texte.

**Les voies de classe préparatoire.** PCSI, PC, MPSI et MP sont des voies et
non des rangs : un même énoncé porte un ancrage de licence et un ancrage de
classe préparatoire, sans corpus parallèle.

**Le rappel sans renumérotation.** `<Rappelle>le théorème de Pythagore` remet
un résultat du corpus sous les yeux, marqué « Rappel » avec son niveau
d'origine — un cours rappelle constamment ce qui a été vu plus tôt, et le
renuméroter serait faux.

## **🆕 Ce que la 2.7 apporte**

**Les environnements numérotés.** Théorème, proposition, propriété, lemme, corollaire, axiome, conjecture, définition, exemple, remarque : dix genres s'énoncent, se numérotent tout seuls, se renvoient l'un à l'autre — et la preuve loge dans l'énoncé, où la machinerie des démonstrations la rédige.

**La frise chronologique.** L'histoire entre dans docdg : un bandeau gradué que referme une grande pointe, les périodes à l'intérieur, les événements à l'extérieur en cartouches reliés à leur date, et des bandes nommées qui partagent la même échelle du temps pour donner à voir la simultanéité. Trois documents suivent les niveaux du programme, du collège au supérieur.

La **2.6** avait apporté ce qui suit, et qui demeure.

**La rédaction du supérieur.** Le vocabulaire et la syntaxe des énoncés ont été repris pour coller à ce qu'attendent les enseignants de mathématiques du supérieur. Le travail ne s'est pas fait au jugé : six ouvrages de référence — 2,1 millions de mots — ont été dépouillés. L'ordre nom-nature de la déclaration y est unanime, 777 occurrences contre 15, chez les six auteurs sans exception. docdg écrit désormais « Soit *f* la fonction définie par *f*(*x*) = … » et non « Soit une fonction *f*(*x*) = … », en phrases plutôt qu'en formules enveloppant du texte.

**Les démonstrations.** Onze raisonnements — direct, contraposée, absurde, récurrence, disjonction de cas, analyse-synthèse, double implication, double inclusion, élément quelconque, principe des tiroirs, existence et unicité — chacun avec sa charpente : l'annonce, les étapes étiquetées, la conclusion quand la logique l'exige. Le moteur fournit la charpente, vous écrivez les mathématiques. Et lorsque l'énoncé est à sa portée, il démontre seul : par le **calcul formel** (SymPy) quand il s'agit de vérifier une identité ou une inégalité, par une **bibliothèque de cent une démonstrations types** quand il s'agit d'une idée que rien ne calcule. Une formule fausse est refusée avec sa raison — le moteur ne démontre pas un mensonge.

**Chaque version a un terme.** docdg reste gratuit, mais une installation est valable un an ; la barre d'outils affiche les jours restants. Vos documents ne sont pas affectés — ce sont vos fichiers, sur votre disque.

## **✨ Fonctionnalités clés**


| **Fonctionnalité**                  | **Description**                                                                                                           | **Exemples**                                       |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| **🏷️ Une seule syntaxe de balise** | `<attributs>{contenu}` couvre texte, tableaux, images, maths, cadres, listes et mises en page pleine page                 | `<Affiche>un cadre avec une bordure bleue{Texte}`, `<Dresse>un tableau [mc]{...}` |
| **🎨 148 couleurs nommées**         | Une palette en **français**, une couleur par nom, accordée selon l'orthographe | `rouge`, `bleu nuit`, `vert forêt` |
| **📐 Un mini-langage mathématique** | Fractions, racines, sommes, produits, intégrales, dérivées, trigonométrie, matrices et systèmes, **le tout dans `$...$`** | `$somme(k=1;n) k^2$`, `$intégrale(x=a;b) f(x)$`    |
| **📦 Cadres et grilles**            | Panneaux `<Affiche>un cadre` colorés et mises en page à zones nommées **façon CSS Grid**                                            | `<Affiche>une grille avec les zones:[...]`, `<Affiche>un cadre avec des coins arrondis de 5 mm`       |
| **📊 Des tableaux sans douleur**    | Placement de cellule en **deux lettres**, fusions, en-têtes, bordures, **aucun `&` à compter**                            | `<Dresse>un tableau [mc ; mg] bordures entête`                |
| **♻️ Alias et macros**              | Factorisez un style **une fois**, nommez-le partout ; **une seule retouche restyle tout le document**                     | `soit titre = <gras 18pt>`, `soit exercice{n} = ...` |
| **🔁 Structures de contrôle**       | Boucles, conditionnelles et interpolation **`#{...}`** dans le corps du document                                          | `pour n de 1 à 5`, `si note >= 10`                 |


---

## **🎯 Pourquoi docdg ?**

De nombreux utilisateurs rédigent aujourd'hui leurs documents à l'aide de **traitements de texte**, de logiciels de présentation ou de publication assistée par ordinateur. Ces outils permettent **d'obtenir rapidement un résultat**. **En revanche**, lorsque les documents deviennent **plus longs ou plus complexes**, leur mise en page demande **beaucoup de manipulation manuelle** (réglages de marges, alignements, numérotations...).

### **💡 docdg adopte une approche radicalement différente**

> **L'utilisateur ne construit pas son document en déplaçant des objets ou en ajustant visuellement la mise en page.** **Il décrit simplement le document qu'il souhaite obtenir.** **La composition est ensuite entièrement prise en charge par docdg.**

### **✅ Les avantages de cette approche**


| **Avantage**                                     | **Bénéfice concret**                              |
| ------------------------------------------------ | ------------------------------------------------- |
| ✅ **Mise en page homogène**                      | Cohérence parfaite sur tout le document           |
| ✅ **Numérotation automatique**                   | Sections, figures, tableaux numérotés sans effort |
| ✅ **Références croisées fiables**                | Liens internes toujours synchronisés              |
| ✅ **Table des matières générée automatiquement** | Mise à jour instantanée                           |
| ✅ **Typographie de qualité professionnelle**     | Rendu PDF soigné pour tous vos documents          |
| ✅ **Maintenance facilitée**                      | Même pour des documents volumineux                |


---

<a id="ce-dont-vous-avez-besoin"></a>

## **🛠️ Ce dont vous avez besoin**

### **Installation**

docdg est un exécutable autonome — aucune dépendance externe obligatoire.

**Téléchargement :** récupérez le binaire pour votre système sur la page des versions :

```
https://github.com/gdubard/docdg/releases
```

Sous Windows, l'installeur configure le `PATH`. Sous macOS/Linux :

```bash
chmod +x docdg
sudo mv docdg /usr/local/bin/
```

Vérifiez l'installation :

```bash
docdg --version
```

### **💻 Éditeurs recommandés**

| **Éditeur**   | **Plateformes**       | **Pourquoi ?**                                                        |
| ------------- | --------------------- | --------------------------------------------------------------------- |
| **docdg**     | Windows, macOS, Linux | ✅ Interface intégrée, prévisualisation en direct, export PDF en un clic |
| VS Code       | Toutes                | Éditeur de texte polyvalent, extension de coloration disponible         |
| Tout éditeur  | Toutes                | Un fichier `.docdg` est du texte brut — n'importe quel éditeur convient |

---

## **🚀 Premiers documents**

### **📄 Le document minimal**

**docdg**

```docdg
Bonjour le monde !
```

> C'est vraiment tout. Enregistrez ce texte dans un fichier `.docdg`, ouvrez-le avec docdg — le PDF est prêt.
### **⚙️ Le bloc document**

Tout ce qui décrit le **corps physique** du document — son format, ses polices, sa conduite — s'écrit dans un bloc `document { }` en tête du fichier. Il est entièrement facultatif : sans lui, des valeurs par défaut raisonnables s'appliquent.

> **📌 L'identité n'est pas un réglage.** Le titre, l'auteur, l'institution, l'emblème, la nature du document : tout cela **se compose** sur la première page, avec les mêmes outils que le reste — des styles (`au centre`, une taille), `<Insère l'image …>` pour l'emblème, et `<page suivante>` pour refermer la couverture. Le bloc `document { }` ne décrit que le papier. Une clé d'identité qui s'y glisse est refusée, et le refus nomme le chemin.

**docdg**

```docdg
document {
  % ===== Marges externes =====
  marges: 20;  % 20 mm sur les quatre côtés

  % ===== Espacements internes =====
  espacements: 2;  % 2 mm entre le contenu et le bord des cadres

  % ===== Polices =====
  script: Georgia;  % Police du texte imprimé — les lettres scriptes
  seyès: Marelle;  % Cursive des lignes réglées — seule la Marelle est embarquée ;
                   % un fichier .ttf voisin est lu et joint au document

  % ===== Taille et espacement =====
  taille: 11;  % Corps de base (pt)
  interligne: 1,2;  % Coefficient d'interligne
  tabulation: 8;  % Largeur tabulation (mm) — le carreau de la Seyès
  hauteur: 8;  % Hauteur saut de ligne (mm) — l'interligne de la Seyès
  décalage: 100;  % Décalage exposants/indices (%)

  % ===== Typographie =====
  césure: oui;  % Coupure des mots en fin de ligne
  orphelines: 2;  % Lignes minimales en bas de page
  veuves: 2;  % Lignes minimales en haut de page

  % ===== Calculs =====
  précision: 3;  % Décimales pour arrondi (-1 = pas d'arrondi)
}
```

> **💡 Virgule décimale.** Dans le bloc, la virgule est la virgule décimale (`interligne: 1,2`), **et elle seule** : `interligne: 1.2` est refusé, et le refus nomme la forme attendue. Tolérer les deux écritures, ce serait les enseigner toutes les deux. Les valeurs à quatre composantes utilisent le point-virgule (`marges: 25;20;25;20`). Partout dans le corps du document, la virgule reste la virgule décimale ; docdg la distingue d'une énumération par l'absence d'espace après elle.

> **⚠️ Une valeur mal écrite se dit.** Un réglage numérique illisible — un point décimal, un mot à la place d'un nombre, deux mesures là où il en faut une ou quatre — ne tombe plus en silence : la clé serait restée à son défaut sans que rien ne le signale, et l'auteur aurait cherché longtemps. La valeur est refusée et nommée, séparément des clés inconnues : ce ne sont pas les mêmes reproches.

> **📌 Un bloc, un nom.** Le bloc s'appelle `document { }`, et c'est le seul. L'ancien `page { }` est refusé, et le refus nomme le remplacement — comme toute clé retirée : une clé sans accent (`cesure`), un ancien alias anglais (`margin`, `padding`) ou une clé d'identité reçoivent la phrase qui remet le document au propre. Rien ne casse en silence.

**Tableau récapitulatif :**

| **Option**   | **Type**           | **Défaut**  | **Description**                |
| ------------ | ------------------ | ----------- | ------------------------------ |
| `orientation`| `portrait`/`paysage` | `portrait` | Orientation de la page       |
| `marges`     | nombre ou 4 valeurs | `20`       | Marges externes (mm) — `20` ou `25;20;25;20` (haut;droite;bas;gauche) |
| `espacements`| nombre ou 4 valeurs | `2`        | Espacements internes des cadres, tableaux et zones (mm) |
| `script`     | texte              | *(celle du système)* | Police du texte imprimé — les lettres scriptes |
| `seyès`      | texte              | `Marelle`   | Cursive des lignes réglées — seule la `Marelle` est embarquée ; un fichier `.ttf` voisin est lu et joint au document |
| `taille`     | nombre             | `11`        | Taille de base (pt)            |
| `interligne` | nombre             | `1,3`       | Coefficient d'interligne       |
| `tabulation` | nombre             | `8`         | Largeur tabulation (mm) — le carreau de la Seyès |
| `hauteur`    | nombre             | `8`         | Hauteur saut de ligne (mm) — l'interligne de la Seyès |
| `décalage`   | nombre             | `100`       | Décalage exposants/indices (%) |
| `césure`     | `oui`/`non`        | `oui`       | Coupure des mots en fin de ligne, selon les motifs français |
| `orphelines` | nombre             | `2`         | Lignes minimales laissées en bas de page |
| `numérotation` | `composée`, `simple` ou `sans` | `composée` | Numéros de page : `1 / 3`, `1` seul, ou aucun |
| `veuves`     | nombre             | `2`         | Lignes minimales reportées en haut de page |
| `précision`  | nombre             | `-1`        | Décimales pour arrondi (`-1` = pas d'arrondi) |

> **🖱️ Sans écrire le bloc.** Le bouton **Paramétrer** de l'application affiche toutes ces options dans un formulaire. À l'application, docdg écrit ou met à jour le bloc `document { }` en tête du fichier, en n'inscrivant que les réglages qui diffèrent du défaut.

> **↦ L'alinéa partout.** Une tabulation en tête de ligne creuse un alinéa de la largeur réglée par `tabulation`, **au fil du texte comme à l'intérieur d'un cadre, d'une grille ou d'un bloc**. La règle est la même partout : une tabulation, un alinéa ; deux tabulations, deux alinéas. Dans un tableau en revanche, la tabulation reste le séparateur de colonnes — elle y garde son rôle de structure.

> **✂️ La césure et les lignes isolées.** Trois réglages, actifs par défaut, tiennent la qualité du gris typographique. La **césure** coupe les mots longs en fin de ligne selon les motifs français de TeX (deux lettres au moins avant la coupure, trois après) : les blancs interlettres d'un texte justifié s'en trouvent régularisés. Elle épargne les mots composés, les sigles, les noms propres, les titres et tout ce qui relève des mathématiques. Les **orphelines** sont les lignes esseulées en bas de page, les **veuves** celles qui échouent seules en haut de la suivante : docdg coupe désormais un paragraphe à une frontière de ligne plutôt que de le reporter en bloc, en garantissant le nombre de lignes demandé de chaque côté. Le réglage `césure: non` rend le texte tel quel, sans coupure.

> **📄 Les blocs qui se coupent.** Un cadre et un tableau sont **insécables par défaut** : un théorème, un énoncé bref, un petit tableau doivent rester d'un seul tenant, et passent entiers à la page suivante s'ils n'y tiennent pas. L'adjectif `sécable` les autorise à se poursuivre d'une page à l'autre — indispensable dès qu'un cadre de démonstration ou un tableau de données dépasse la page. Le filet du cadre devient tireté au point de rupture et son titre ne se répète pas ; la **rangée d'entête du tableau, elle, reparaît en tête de chaque fragment**, sans qu'il y ait rien à écrire pour l'exiger. Un tableau se coupe dès qu'une rangée tient dans la place restante : les règles de veuve et d'orpheline valent pour la prose, non pour les rangées, et continuent de s'appliquer à l'intérieur d'un cadre scindé. L'exemple `publication3.txt` met les cinq réglages à l'épreuve sur un même document.

### **📝 Écrire plusieurs paragraphes**

Un simple passage à la ligne sépare deux paragraphes.

**docdg**

```docdg
Premier paragraphe.
Deuxième paragraphe.
```

### **🏷️ Le titre du document**

**docdg**

```docdg
soit titre = <rouge gras 18pt au centre>

<titre>Ma feuille d'exercices
```
---

## **📝 La syntaxe**

> **La règle d'or, une fois pour toutes.** Une action s'écrit **`<Action>`complément** : le verbe seul dans la balise, le complément à l'extérieur, en français complet — articles compris et nom de l'objet rappelé.
>
> ```docdg
> <Résous>l'équation 3x + 5 = 17
> <Trace>la fonction f
> <Calcule>l'aire du disque de rayon 3
> ```
>
> Quand l'action porte sur un contenu de plusieurs lignes, celui-ci suit entre accolades : `<Action>complément {` … `}`. Les attributs se disent avec leurs articles (« avec **des** bordures », « **un** écart de 3 mm »), et un objet déclaré se rappelle par sa catégorie (« le système S », « la fonction f » — jamais le nom seul). Les options éventuelles d'une action restent dans la balise, après le verbe. Et parce que l'objectif est d'écrire moins : **la rédaction pas à pas est le comportement, pas une option**. `<Résous>le système s` déroule le pivot, `<Calcule>1/2 + 1/3` détaille la mise au même dénominateur — il n'y a rien à demander. L'ancienne écriture `<Action>complément` reste comprise, mais toute la documentation et tous les exemples suivent la règle ci-dessus : une seule forme à apprendre, une seule forme à relire.


La syntaxe de docdg repose sur **quatre familles de commandes** :


| **Famille**    | **Rôle**             | **Exemples**                      |
| -------------- | -------------------- | --------------------------------- |
| **Actions**    | Opérations           | `<Trace>`, `<Soit>`, `<Affiche>`, `<Dresse>`  |
| **Objets**     | Éléments du document | `un cadre`, `un tableau`, `une image`      |
| **Propriétés** | Personnalisation     | `rouge`, `gras`, `14pt`, `au centre` |
| **Valeurs**    | Paramètres           | `oui`, `non`, `30`, `bleu foncé`  |

**Une seule façon de parler à docdg : la phrase.** Une balise se lit dans l'ordre où elle s'écrit — l'**action** d'abord (le verbe, entre chevrons), puis l'**objet** qu'elle concerne, avec son article et ses attributs en prose, puis enfin le **contenu**, entre accolades : `<Action>objet avec propriétés{contenu}`. Le chevron se referme le plus souvent tout de suite après l'objet et ses attributs — `<Affiche>un cadre avec ...{...}` — mais pour les objets qui se nomment en vue d'un calcul ultérieur (une matrice, un système), le chevron peut aussi se refermer tout de suite après le verbe seul, l'objet et son nom suivant en toutes lettres : `<Soit>la matrice M{...}` (voir la section **Matrices et systèmes**) ; les deux s'y compilent de façon identique. Toute balise commence par un **verbe d'action à l'impératif présent** — jamais par l'objet nu (règle n°12). Les attributs se disent **en prose**, introduits par `avec`, avec le nom et son article (`avec une bordure rouge, un fond jaune clair et des coins arrondis de 5 mm`) — l'ancienne écriture en couples `attribut:valeur` n'existe plus, hors des rares réglages techniques signalés comme tels (`x:{...}` des tableaux de signes, par exemple). Convention du manuel : quand un chapitre étudie un objet en particulier, la tête de balise s'écrit soit avec un verbe (`<Dresse>un tableau ...`), soit avec trois points qui tiennent la place de n'importe quel verbe d'action : `<... un tableau ...>`.

### **📌 Les règles de base**

1. **Les balises** sont entre **chevrons** et commencent par un **verbe** : `<Affiche>un cadre`, `<Dresse>un tableau`, `<Insère l'image IMAGES/photo.png avec une largeur de 30 mm>`  (règle n°12)
2. **Le contenu** est entre **accolades** : `<Affiche>un cadre{Du texte ici}`
3. **Les noms composés** utilisent des **espaces** : `<table des matières>`
4. **Les propriétés** s'écrivent **en français** : `<bleu foncé centre 14pt>{Titre}`
5. **Les couleurs** : `rouge`, `bleu foncé` (français)
6. **Les polices** : en **MAJUSCULES** : `<ARIAL>`
7. **Échappement** : doublez les caractères spéciaux pour les afficher littéralement : `<<` → `<`, `>>` → `>`, `{{` → `{`, `}}` → `}`, `##` → `#`, `$$` → `$`.
8. **Les séparateurs** : le point-virgule `;` sépare **les nombres, et rien d'autre** — coordonnées (`A(2;3)`), bornes d'intervalles (`[-5 ; 5]`), séries de données (`{12 ; 15 ; 9}`), arguments numériques (`arrondi(5/3; 2)`). La virgule `,` sépare les fragments de prose (règle n°10) et reste la virgule décimale à l'intérieur d'un nombre (`3,5`). La barre `|` sépare les paires `clé: valeur` des données nommées (`{Marche: 5 | Bus: 3}`) et les colonnes des rangées saisies. Trois séparateurs, trois usages, aucune exception.

9. **Majuscule de phrase** : **Seules** les actions dont l'écriture se lit comme une phrase française (`<Soit>`, `<Trace>`, etc.) commencent par une majuscule. Les **mots-clés techniques** (`soit`, `pour`, `si`, `fonction`) restent en minuscules.
10. **Article + prose** : toute action-phrase (`<Soit>`, `<Trace>`) désigne ses objets avec un article et décrit leurs attributs en prose. En détail :

   - **L'article porte l'intention.** `<Soit>` introduit du nouveau : article **indéfini** (`un point A(2;7)`). Au pluriel, l'article **factorise** : `<Soit>les points A(-1,5;3), B(2;5) et C(0;-4,3)` — une seule phrase pour trois points. Un **nombre** peut tenir lieu d'article pluriel (`<Soit>3 points A(1;2), B(-1;2) et C(-1;-2)`) : docdg vérifie alors que le compte annoncé correspond au nombre de points décrits. Enfin, la **distribution respective** — l'idiome des énoncés du secondaire et du supérieur (« de coordonnées respectives », « d'unités respectives ») — sépare les noms des valeurs et les apparie dans l'ordre : `<Soit>les points A, B et C de coordonnées respectives (1;2), (-1;2) et (-1;-2)`. `<Trace>` prend l'article **défini** pour un objet déjà posé (`le point A`) et l'**indéfini** quand il crée la figure au moment du tracé (`un triangle ABC équilatéral, de côté 5 cm`). Les deux formes produisent le même tracé. L'article se met devant chaque nom — objets compris : `<un cadre avec une bordure rouge...>`, `<une grille...>`.
   - **Les qualificatifs** restent accolés au nom sans « de » : `équilatéral`, `rectangle en A`.
   - **Les propriétés à antonyme court** se disent en prose : `avec les marques` / `sans marques`, `avec les valeurs` / `sans valeurs`. `avec` prend l'article, `sans` ne le prend pas — comme en français (`avec du sucre` / `sans sucre`). `avec` sert aussi à introduire un groupe d'attributs : `<un tableau [...] avec entête>`, `avec un fond jaune clair, des coins arrondis de 5 mm et un titre {Attention}`.
   - **La couleur** se dit avec `en` : `en bleu`, `en rouge foncé`.
   - **L'épaisseur du trait** se dit `de trait N mm` (`de trait 0,5 mm`) — c'est une mesure : son unité est obligatoire, comme toutes les unités de longueur.

11. **Les attributs d'un objet s'ouvrent par « avec »**, se séparent par une virgule, et les deux derniers se relient par « et ». C'est la seule forme admise ; écrire les attributs sans `avec` est refusé avec un message qui rappelle la règle. Chaque attribut porte de préférence son **nom précédé d'un article** — plus proche du langage naturel que l'adjectif seul :

    ```docdg
    <Affiche un cadre avec un fond vert menthe, une bordure bleu canard,
        des coins arrondis de 3 mm et un titre {Théorème}>{ ... }
    ```

    L'adjectif `bordé de` n'existe plus : la couleur de bordure se dit `une bordure bleu canard`, comme le fond se dit `un fond vert menthe` — le nom précédé de son article (`une bordure`, `un fond`) se lit mieux que l'adjectif, et docdg ne garde qu'une forme canonique par concept. Les qualificatifs qui font corps avec l'objet restent des adjectifs (`un triangle rectangle`, `un triangle équilatéral`). Un objet **sans** attribut s'écrit nu (`<Affiche>un cadre{...}`), et un placement seul entre crochets (`<Dresse>un tableau [mc ; mg]{...}`) ne demande pas de `avec`.
   - **Le connecteur est obligatoire** entre deux attributs de prose : une virgule entre fragments, ou `et` au sein d'un fragment — `en rouge et de trait 0,5 mm`, jamais la juxtaposition nue. Seule la charnière vers un réglage `clé:valeur` s'accroche sans connecteur.
   - **Les unités de longueur** (`cm`, `mm`) sont **obligatoires** : `de côté 5,3 cm`, jamais `de côté 5,3`. Une longueur sans unité est une erreur, dite en français. *Seule exception :* dans un repère, les longueurs se comptent en **graduations** et s'écrivent sans unité.
   - **Le clé:valeur** ne garde que les réglages purement techniques sans tournure française courante. Chaque fois qu'un nom français existe, la prose gagne : `avec 200 échantillons` plutôt qu'un réglage abstrait, `un pas de $pi/6$` plutôt qu'un `step:`.

12. **Une balise commence toujours par un verbe d'action à l'impératif présent** — `<Dresse>un tableau ...`, `<Affiche>un cadre ...`, `<Insère l'image ...>`, `<Construis>un arbre ...`, `<Trace>...`, `<Soit>` — **jamais par l'objet nu**. L'objet porte toujours son article, et l'article suit un verbe : `<Affiche>un cadre` et non `<cadre>`. Écrire l'objet nu est refusé avec un message qui rappelle la règle. Trois familles seulement échappent au verbe, parce qu'elles ne posent aucun objet dans le document : les **styles** (`<gras rouge 14pt>`), les **commandes structurelles** (`<section>`, `<sous-section>`, `<table des matières>`) et les **définitions d'alias** (`soit exo{n} = <Affiche>un cadre avec ...`, où le membre droit décrit un objet sans encore l'afficher — le verbe y est le bienvenu mais la définition seule ne produit rien). Les verbes d'objet interchangeables sont `Affiche`, `Dresse`, `Construis` et `Insère` ; le manuel écrit `<... un tableau ...>` quand le verbe importe peu.


### **🔹 Structure d'une balise**

```docdg
<OBJET PROPRIÉTÉS>{
  CONTENU
}
```

**Exemple :**

```docdg
<un cadre avec une bordure rouge, un fond jaune clair, des coins arrondis de 5 mm et un titre {Attention}>{
  <gras 14pt>{Exercice important !}
}
```

---

## **📦 Les objets**

### **1️⃣ `<Affiche>un cadre` — Mettre en valeur du contenu**

**Syntaxe :** `<Affiche>un cadre PROSE{CONTENU}`

**Propriétés complètes :**


| **Propriété**            | **S'écrit**                          | **Défaut**       | **Description**                          |
| ------------------------ | ------------------------------------- | ------------------ | ----------------------------------------- |
| Couleur de la bordure     | `une bordure <couleur>`              | `gris`              | Couleur de la bordure                     |
| Couleur du texte / fond   | `texte <couleur>`, `fond <couleur>`  | `texte noir, fond blanc` | Couleur du texte et du fond du cadre |
| Coins arrondis            | `coins arrondis de <N>mm`            | pas d'arrondi       | Rayon des coins                           |
| Largeur                   | `large de <N>mm` / `large de N%`     | auto                | Largeur du cadre                          |
| Épaisseur de la bordure   | `bordure épaisse de <N>mm`           | `0,4mm`             | Épaisseur du trait                        |
| Marge intérieure          | `marge de <N>mm`                     | `3mm`               | Marge intérieure                          |
| Titre                     | `avec un titre {...}`                        | pas de titre        | Titre affiché en haut du cadre            |
| Couleur du titre          | `titre en <couleur texte> sur fond <couleur fond>` | -      | Couleur du texte et du fond du titre      |
| Coupure entre pages       | `sécable` / `insécable`              | `insécable`         | Autorise le cadre à se poursuivre d'une page à l'autre |


`texte <couleur>` et `fond <couleur>` sont deux fragments indépendants de la même prose, séparés par une virgule (règle n°8) : on peut n'écrire que l'un des deux, l'autre garde sa valeur par défaut. Même principe pour `titre en ... sur fond ...`.

**Un cadre recto/verso.** Une ligne ne contenant que `---` coupe le cadre en deux parties, l'énoncé au-dessus, la solution en dessous — pratique pour une fiche de révision ou une carte question/réponse :

```docdg
<Affiche>un cadre avec une bordure bleu marine et un titre {Question}{
  Quelle est la dérivée de $x \mapsto x^2$ ?
  ---
  $2x$
}
```

**Exemples :**

```docdg
% Cadre simple
<Affiche>un cadre avec une bordure bleue, un fond bleu Alice et des coins arrondis de 4 mm{
  <gras 14pt au centre>{Exercice 1}
  Résoudre $x^2 - 5x + 6 = 0$.
}

% Cadre avec titre
<Affiche un cadre avec une bordure rouge foncé, un fond blanc, des coins arrondis de 6 mm,
  un titre {Théorème de Pythagore} et un titre en blanc sur fond rouge foncé>{
  Dans un triangle rectangle...
}
```

### **2️⃣ `<Dresse>un tableau` — Tableaux simplifiés**

**Trois règles suffisent :**

1. **Hors des crochets, tout est entête** — la rangée du haut comme la colonne de gauche. Rien à déclarer.
2. **Entre crochets, tout est donnée** : `[a ; b ; c]` est une rangée, les cellules séparées par un point-virgule.
3. **Les fusions se déduisent de la forme** : une rangée plus courte que le tableau étend sa dernière cellule jusqu'au bord droit ; une entête suivie de plusieurs rangées descend sur autant de lignes.

```docdg
<Dresse>un tableau [mc ; mg ; md]{
	Jour	Matière	Note
	[Lundi ; Maths ; 15]
	[Mardi ; EPS ; 16]
}
```

La première rangée est hors crochets : c'est l'entête, en gras, sans que le mot `entête` figure nulle part.

**Entêtes verticales et fusions.** Une entête suivie d'un deux-points porte les rangées qui la suivent. Écrite seule sur sa ligne, elle ouvre un groupe que l'indentation dessine :

```docdg
<Dresse>un tableau [mc ; mg ; md]{
	Jour	Matière	Note
	Lundi :
		[Maths ; 15]
		[Français ; 12]
	Mardi :
		[EPS ; 16]
}
```

« Lundi » couvre deux rangées parce qu'il en porte deux. Le deux-points est consommé, jamais imprimé. La forme **aplatie** donne exactement le même tableau quand la place le permet :

```docdg
	Lundi :	[Maths ; 15] [Français ; 12]
```

**Entêtes emboîtées.** Les entêtes verticales se collent aux données : la dernière touche le crochet, celle d'avant la précède, et ainsi de suite. Les tabulations ne servent plus qu'à aligner la source à l'œil, si bien que la source a la forme du tableau :

```docdg
<Dresse>un tableau [mc ; mc ; mg ; md]{
	Semaine	Jour	Matière	Note
	S1 :	Lundi :	[Maths ; 15]
					[Français ; 12]
			Mardi :	[EPS ; 16]
	S2 :	Lundi :	[Maths ; 18]
}
```

« S1 » couvre trois rangées, « Lundi » deux : chacun couvre ce qu'il porte. La forme en plan, où chaque entête s'écrit seule sur sa ligne et s'indente sous la précédente, donne le même tableau et reste acceptée.

**Fusion horizontale.** Une rangée à qui il manque des cellules étend la dernière jusqu'au bord droit — un titre seul couvre toute la largeur :

```docdg
<Dresse>un tableau [mc ; mg ; md]{
	Bulletin du premier trimestre
	Jour	Matière	Note
	Lundi :	[Maths ; 15]
	Moyenne :	[13,7]
}
```

**Codes de placement (2 lettres, ou la prose en toutes lettres) :**

- Vertical : `h` (en haut), `m` (au milieu), `b` (en bas)
- Horizontal : `g` (à gauche), `c` (au centre), `d` (à droite)

| Code | En toutes lettres |
| ---- | ----------------- |
| `hg` | `en haut à gauche`  |
| `hc` | `en haut au centre` |
| `hd` | `en haut à droite`  |
| `mg` | `au milieu à gauche` |
| `mc` | `au milieu au centre` |
| `md` | `au milieu à droite` |
| `bg` | `en bas à gauche`   |
| `bc` | `en bas au centre`  |
| `bd` | `en bas à droite`   |

La forme en toutes lettres n'est jamais moins correcte que le code à deux lettres — c'est un raccourci, pas une syntaxe distincte :

```docdg
<Dresse>un tableau [au milieu au centre, au milieu à gauche, au milieu au centre] bordures{
	Jour	Matière	Note
	[Lundi ; Mathématiques ; 15]
}
```

Le nombre de colonnes est celui de la rangée la plus large. S'il dépasse le nombre de codes annoncés, le dernier code vaut pour les colonnes suivantes ; une rangée plus large que le tableau reste une erreur signalée.

**Options :**

| **Option**              | **S'écrit**                          | **Défaut**    | **Description**                              |
| ----------------------- | -------------------------------------- | ------------- | ---------------------------------------------- |
| `[codes]`               | `[mc ; mc ; ...]`                     | `[mc ; mc ; ...]` | Placement des colonnes (formes longues admises : `[au centre ; en bas, à droite]`) |
| Couleur de la bordure    | `une bordure <couleur>`               | `gris`        | Couleur des bordures                            |
| Couleur des cellules     | `texte <couleur>`, `fond <couleur>`   | -             | Couleur du texte et du fond des cellules        |
| Couleur des entêtes      | `des entêtes en <couleur texte> sur fond <couleur fond>` | -   | Couleur de toutes les cellules d'entête         |
| Écart entre cellules     | `écart de <N>mm`                      | `0mm`         | Espacement entre cellules                       |
| Coupure entre pages      | `sécable` / `insécable`               | `insécable`   | Autorise le tableau à se poursuivre d'une page à l'autre ; la rangée d'entête se répète en tête de chaque fragment |

Même convention que pour `<Affiche>un cadre` : `texte <couleur>` / `fond <couleur>`, chaque partie facultative. Les couleurs d'entête habillent toutes les cellules d'entête, la rangée du haut comme la colonne de gauche.

**Exemple complet :**

```docdg
<Dresse un tableau [mc ; mg ; mc] avec un écart de 3 mm,
  une bordure bleu marine, un fond bleu Alice et un entête en blanc, sur fond bleu marine>{
	Jour	Matière	Note
	Lundi :	[Mathématiques ; 15]
	Mardi :	[Français ; 12]
}
```

Pour écrire des crochets littéraux dans une cellule, les protéger par des dollars : `$[AB]$`.

> **Documents écrits avant la grammaire des crochets.** Un tableau qui ne contient aucun crochet conserve exactement l'ancienne sémantique : les tabulations séparent les cellules, `entête` met la première rangée en gras, `<2 colonnes mc>` et `<2 lignes mc>` déclarent les fusions, un point seul marque une cellule absorbée. Aucun document existant ne bouge. La grammaire des crochets est la seule enseignée depuis.

### **3️⃣ Dénombrement, arithmétique, complexes**

```docdg
<Dénombre>les combinaisons de 3 parmi 10
<Dénombre>les arrangements de 3 parmi 10
<Dénombre>les permutations de 5
<Dresse>le triangle de Pascal jusqu'à la ligne 5
```

Le calcul est posé avant d'être fait : la formule s'affiche, puis le résultat.

```docdg
<Dresse>la décomposition en facteurs premiers de 360
<Applique>l'algorithme d'Euclide à 84 et 60
```

La décomposition donne aussi le nombre de diviseurs ; Euclide déroule ses divisions, conclut sur le PGCD et le PPCM, et fournit une relation de Bézout.

### **3️⃣ Le programme de calcul**

Le collège pose un programme de calcul, le fait tourner sur un nombre, puis le met en formule. Les deux verbes lisent le **même corps**, une étape par ligne :

```docdg
<Applique>le programme de calcul à 5 {
	choisir un nombre
	ajouter 3
	multiplier par 2
	soustraire 4
}

<Exprime>le programme de calcul en fonction de x {
	choisir un nombre
	ajouter 3
	multiplier par 2
	soustraire 4
}
```

`<Applique>` déroule les étapes une à une, en montrant chaque passage ; `<Exprime>` compose l'expression littérale et la réduit.

| Étape | Effet |
|---|---|
| `choisir un nombre` | l'en-tête ; sans effet sur le calcul |
| `ajouter N` | ajoute `N` |
| `soustraire N` | retranche `N` |
| `multiplier par N` | multiplie par `N` |
| `diviser par N` | divise par `N` — la division par zéro est dite, non calculée |
| `élever au carré` | met au carré |
| `élever au cube` | met au cube |

`N` s'écrit en entier, en décimal à virgule ou en fraction. **Une ligne que la table ne reconnaît pas interrompt le programme entier** : le verbe ne rend rien plutôt que de sauter une étape en silence.

### **3️⃣ Périmètres, aires et volumes**

Les mesures usuelles se demandent en toutes lettres, avec leurs dimensions :

```docdg
<Calcule>le périmètre du rectangle de longueur 7 cm et de largeur 4 cm
<Calcule>le périmètre du triangle de côtés 3, 4 et 5
<Calcule>le périmètre du cercle de rayon 5
<Calcule>l'aire du carré de côté 6
<Calcule>l'aire du triangle de base 6 et de hauteur 4
<Calcule>l'aire du disque de rayon 3
<Calcule>le volume du pavé droit de longueur 5, de largeur 3 et de hauteur 2
<Calcule>le volume du cylindre de rayon 2 et de hauteur 5
<Calcule>le volume du cône de rayon 3 et de hauteur 4
```

La formule est posée, puis appliquée. L'unité est facultative ; lorsqu'elle est
donnée, elle suit la dimension — `cm` pour un périmètre, `cm²` pour une aire,
`cm³` pour un volume.

### **3️⃣ Les anneaux de factorisation**

`<Factorise>` travaille par défaut dans $\mathbb{R}$. Trois compléments l'en font sortir :

```docdg
<Factorise>x^4 - 1 dans les complexes
<Factorise>x^4 - 1 dans C[X]
<Factorise>x^2 + 1 dans Q[X]
```

`dans les complexes` et `dans C[X]` sont la même demande. `<Résous>` reçoit les mêmes compléments, plus `dans les entiers` (ou `dans ZZ`) pour une équation diophantienne.

```docdg
<Écris>le complexe z = 1 + i sous ses trois formes
```

Formes algébrique, trigonométrique et exponentielle, avec module exact (`√2` reste `√2`) et argument en fraction de π quand il tombe juste.

### **3️⃣ `<Dresse>la table de Cayley du groupe ...`**

```docdg
<Dresse>la table de Cayley du groupe Z/4Z
<Dresse>la table de Cayley du groupe (Z/8Z)*
```

Addition modulo *n* pour `Z/nZ`, multiplication des inversibles pour `(Z/nZ)*` : la table est calculée, pas saisie.

### **3️⃣ Lycée : vecteurs, lois, suites**

```docdg
<Écris>le vecteur u de coordonnées (3 ; -2)
```

Notation colonne, deux ou trois coordonnées, fractions acceptées. Le nom est facultatif.

```docdg
<Dresse>la loi de probabilité de X{
	valeurs : [1 ; 2 ; 3]
	probabilités : [1/6 ; 1/3 ; 1/2]
}
```

Le tableau se dresse tout seul. La somme des probabilités est vérifiée : une loi qui ne totalise pas 1 est refusée, avec le total fautif dans le message.

```docdg
<Soit>la suite u définie par u(0) = 1 et u(n+1) = 2u(n) + 1

<Calcule>les 6 premiers termes de u
```

La définition s'affiche en notation indicielle, puis les termes se calculent à la demande.

### **3️⃣ `<Étudie>le second degré` — Trinômes**

Le trinôme se donne tel qu'il s'écrit, un par ligne. L'étude sort complète : discriminant, racines, forme factorisée, forme canonique.

```docdg
<Étudie>le second degré{
	2x^2 - 3x + 1
	x^2 - 4x + 4
	x^2 + x + 1
}
```

Les fractions restent exactes tant que le discriminant est un carré parfait ; sinon le radical est conservé et une valeur approchée l'accompagne. La variable se précise si besoin : `<Étudie>le second degré en t`.

### **3️⃣ `<Insère>une image` — Insertion d'images**

**Syntaxe :** `<Insère l'image CHEMIN avec ATTRIBUTS>` — le chemin s'écrit **en entier dans la balise**, relatif au fichier `.docdg` ; rien ne reste à accoler après. Un nom comportant des espaces s'accolade : `l'image {mon chat.png}`. Les dimensions se disent **en prose**, comme tout attribut (règle n°11), l'unité (mm) obligatoire. docdg ne cherche jamais dans un dossier par défaut : omettre le chemin est refusé avec un message qui rappelle la forme.

- `avec une largeur de 30 mm` — 30 mm de large, hauteur proportionnelle ;
- `avec une hauteur de 20 mm` — 20 mm de haut, largeur proportionnelle ;
- `avec une largeur de 40 mm et une hauteur de 25 mm` — l'image tient dans 40 × 25 mm, proportions gardées ;
- `avec un côté de 30 mm` — l'image tient dans un carré de 30 mm ;
- `avec une largeur de 50 %` — la moitié de la largeur de la ligne.

**Autres attributs :**

| **Attribut** | **S'écrit**              | **Description**       | **Obligatoire** |
| ------------ | ------------------------ | ---------------------- | ---------------- |
| Chemin       | `l'image DOSSIER/fichier.png` | Le fichier, depuis la source | oui, toujours |
| Légende      | `la légende {...}`       | Légende sous l'image   | non               |

**Exemples :**

```docdg
<Insère l'image IMAGES/diagramme.png avec une largeur de 50 mm>
<Insère l'image IMAGES/photo.png avec une largeur de 80 mm, une hauteur de 60 mm et la légende {Figure 1}>
<Insère l'image IMAGES/GEOMETRIE/cercle.png avec un côté de 30 mm>
```

### **4️⃣ `<Dresse>une liste ADJECTIF` — Listes variées**

**Styles disponibles :**


| **S'écrit**             | **Résultat**  |
| ------------------------ | ------------- |
| `liste sans puce`        | Texte brut    |
| `liste à puces`          | •             |
| `liste à puces vides`    | ∘             |
| `liste à puces carrées`  | ▪             |
| `liste numérotée`        | 1., 2., 3.    |
| `liste alphabétique`     | a., b., c.    |
| `liste alphabétique majuscule` | A., B., C. |
| `liste en chiffres romains` | i., ii., iii. |
| `liste en chiffres romains majuscules` | I., II., III. |
| `liste à cocher`         | □             |


**Exemples :**

```docdg
<Dresse>une liste à puces{
pommes
poires
<gras>{cerises}
}

<Dresse>une liste numérotée Rouge 14pt{
Premier point
Deuxième point
}
```

### **5️⃣ `<Affiche>une grille` — Mise en page avancée**

**Syntaxe :**

```docdg
<Affiche>une grille avec les zones:[DISPOSITION], les colonnes: ..., les lignes: ... et OPTIONS{
  [NOM PROPRIÉTÉS]{CONTENU}
}
```

**Propriétés :**


| **Propriété**            | **S'écrit**                          | **Défaut** | **Description**                                                  |
| ------------------------- | --------------------------------------| ---------- | ------------------------------------------------------------------|
| Zones                      | `zones:[DISPOSITION]` ou bloc visuel  | -          | Disposition des zones (technique — voir plus bas)                  |
| Colonnes                   | `colonnes: ...`                      | `max`      | Largeur de chaque colonne, séparées par des virgules : `min`, `max`, ou valeur fixe (`3cm`) |
| Lignes                     | `lignes: ...`                        | `max`      | Hauteur de chaque ligne, séparées par des virgules : `min`, `max`, ou valeur fixe (`15cm`) |
| Bordures                   | `bordures` (flag, sans valeur)       | absentes   | Affiche une bordure autour de chaque zone                          |
| Couleur de la bordure      | `une bordure <couleur>`              | `gris`     | Couleur des bordures (par défaut, surchargeable par zone)          |
| Couleur du contenu         | `texte <couleur>`, `fond <couleur>`  | `texte noir, fond blanc` | Couleur du contenu (par défaut, surchargeable par zone) |
| Écart                      | `écart de <N>mm`                     | `0mm`      | Espacement entre zones                                              |
| Largeur totale             | `large de <N>mm`                     | auto       | Largeur totale                                                     |
| Hauteur totale             | `haut de <N>mm`                      | auto       | Hauteur totale                                                     |


`zones:`, `colonnes:` et `lignes:` décrivent le plan de la grille (un peu comme des coordonnées) : ce sont des données de structure, pas des réglages de style, elles restent donc en clé:valeur. `colonnes:` et `lignes:` mélangent des mots-clés (`min`, `max`) et des longueurs (`3cm`) — ce n'est pas une liste purement numérique, donc la règle n°8 impose la **virgule** comme séparateur, et les crochets ne sont pas nécessaires : `colonnes: 3cm, auto`, `lignes: min, 15cm, min`. Les crochets `[...]` restent réservés à `zones:`, seule donnée réellement bidimensionnelle de la grille.

Chaque zone s'écrit `[nom en mc avec attributs]` : le **placement** s'introduit par `en` (`en mc`, `en j` pour justifié, ou en toutes lettres : `en haut, à gauche`), puis les **attributs** par `avec`, séparés par des virgules, `et` avant le dernier — la même convention que `<Affiche>un cadre` et `<Dresse>un tableau` (règle n°11). Une zone peut ainsi surcharger `une bordure <couleur>` et ajouter `un texte <couleur>, un fond <couleur>`.

**Exemple simple :**

```docdg
<Affiche>une grille avec les zones:["titre titre logo", "info info logo", "corps corps corps"], des bordures et un écart de 5 mm{
  [titre en mc avec une bordure bleu marine, un fond bleu Alice et des coins arrondis de 2mm]{
    <bleu marine gras 20pt au centre>Devoir de Maths
  }
  [logo en mc]{
    <Insère l'image IMAGES/logo.png avec une largeur de 40 mm>
  }
  [info]{
    Nom: _______ Prénom: _______
  }
  [corps en j]{
    <section>Exercice 1
    Résoudre $x^2 = 4$.
  }
}
```

**Exemple avec largeurs et hauteurs fixées (`min` / `max` / valeur fixe), et `zones:` en bloc visuel :**

Pour une disposition à plat, `zones:` accepte un **bloc indenté**, sans guillemets ni virgules ni crochets : chaque ligne du bloc est une ligne de la grille, chaque mot séparé par des espaces est une zone — la disposition se lit à l'œil, exactement comme un plan tracé à la main.

```docdg
<Affiche>une grille avec les zones:
          haut haut
          nav  corps
          bas  bas
        les colonnes: 3cm, max
        les lignes: min, 15cm, min
        une hauteur de 20cm et un écart de 5mm{

  [haut]{
    % Contenu du header (prendra sa hauteur minimale : min)
  }

  [nav]{
    % Menu / Navigation (largeur fixe : 3cm)
  }

  [corps]{
    % Zone principale (prend tout l'espace restant : max)
  }

  [bas]{
    % Pied de page
  }
}
```

La forme équivalente en tableau de chaînes reste valable, utile quand la grille est écrite en une seule ligne plutôt qu'en bloc :

```docdg
<Affiche>une grille avec les zones:["haut haut", "nav corps", "bas bas"], les colonnes: 3cm, max et les lignes: min, 15cm, min{ ... }
```

### **5️⃣bis La page de titre**

Il n'y a **pas de balise spéciale** : une couverture se compose avec les outils de tout le monde — des styles nommés, une image, un saut de page. C'est le même geste qu'ailleurs, et il compose aussi bien la première de couverture d'un essai que la page de titre d'une thèse :

```docdg
soit nature = <petites capitales grand au centre>
soit ouvrage = <gras 24pt au centre>
soit mention = <14pt au centre>

<au centre>{<Insère l'image IMAGES/institut.svg avec une largeur de 30 mm>}

<nature>{Thèse de doctorat}

<mention>{Université de Vaubertier}

<ouvrage>{Points fixes attractifs et vitesse de convergence}

<mention>{Théodore MARSAN}

<mention>{septembre 2026}

<page suivante>
```

**Entre accolades, une ligne est une ligne.** Hors des accolades, la prose se
recompose : des lignes qui se suivent font un paragraphe. Entre les accolades
d'un style, c'est l'auteur qui dispose, et docdg compose tel quel — comme dans
tous les corps accolés de docdg, du tableau à la frise. Un jury, une adresse,
une épigraphe s'écrivent donc d'un seul style, sans le répéter ni lui inventer
un nom ; pour plus d'air, on saute une ligne.

```docdg
<12pt italique au centre>{
Devant le jury composé de :
madame le professeur Hélène VIRECOURT, présidente
monsieur le professeur Bastien QUÉRAND, directeur de thèse
}
```

Le `<page suivante>` final referme la couverture ; s'il précède une division majeure — qui ouvre sa propre page —, les deux sauts n'en font qu'un, et aucune page blanche ne s'intercale. Les cinq documents de la série `publication` composent chacun la leur : l'article sobre, l'exposé, la thèse à emblème, l'essai.

---

### **6️⃣ Divisions — Structuration du document**

**Syntaxe :**

```docdg
<tome>Titre du tome
<livre>Titre du livre
<partie>Titre de la partie
<chapitre>Titre du chapitre
<section>Titre principal
<sous-section>Sous-titre
<sous-sous-section>Sous-sous-titre
```

**Table des matières :** `<table des matières>`

Sept niveaux, deux natures. Les trois premiers sont des **divisions majeures** : chacune prend une page à elle seule, y compose son nom au-dessus de son titre, et n'a besoin d'aucun `<page suivante>` pour cela. Les quatre suivantes sont des **divisions courantes** : elles s'insèrent dans le fil du texte, et la numérotation y est séparée du titre par un **trait d'union** — « 1 - Titre », « 1.1 - Sous-titre » —, dans les titres comme dans la table des matières.

| Division | LaTeX | Numérotation | Rendu |
|---|---|---|---|
| `tome` | `\part` + `\renewcommand{\partname}{Tome}` | romaine | Tome I |
| `livre` | `\chapter` + `\renewcommand{\chaptername}{Livre}` | arabe | Livre 1 |
| `partie` | `\part` | romaine | Partie I |
| `chapitre` | `\chapter` | arabe | 1 - Titre |
| `section` | `\section` | arabe, préfixée du chapitre | 1.1 - Titre |
| `sous-section` | `\subsection` | arabe | 1.1.1 - Titre |
| `sous-sous-section` | `\subsubsection` | arabe | 1.1.1.1 - Titre |

Le romain et l'arabe ne se choisissent pas : ils viennent de LaTeX, où `\part` compte en chiffres romains et `\chapter` en chiffres arabes. Un tome et un livre ne sont jamais que l'un ou l'autre que l'on renomme, et ils en gardent la numérotation.

**Aucune division majeure ne remet à zéro celle qui la suit** — c'est la conduite de la classe `book`, où les chapitres se suivent d'une partie à l'autre. Chaque échelle court sur l'ouvrage entier, aucun numéro ne paraît deux fois, et un renvoi n'a jamais à dire dans quelle partie chercher.

Une étiquette posée sur une division se renvoie **par son seul numéro** : `la partie <renvoi>{support}` donne « la partie I », jamais « la partie Partie I ». Le nom appartient à la division, la phrase appartient à l'auteur.

**Exemple :**

```docdg
soit h0 = <bleu marine gras chapitre num>
soit h1 = <bleu gras section num>

<tome>Ce que la matière imposait

<partie>La longueur qu'un support autorise

<h0>Le volumen

<h1>Une longueur qui ne se discutait pas
```

Une division se déclare aussi bien par un style nommé — `soit p = <bleu nuit partie>` — que par son mot nu, comme ci-dessus.

---

## **🧾 Les démonstrations**

Une démonstration s'écrit avec le verbe **`<Montre>`** et lui seul — la règle d'or s'applique ici comme partout : le verbe dans la balise, le raisonnement et l'énoncé en complément, `<Montre>par récurrence que …` — dans les manuels du supérieur, « Montrons que » écrase toutes les autres annonces. Le raisonnement direct est la forme nue ; les neuf autres se nomment après le verbe, d'un seul nom chacun, parce que **chaque raisonnement s'annonce avant de se dérouler** — une analyse-synthèse non annoncée, par exemple, ressemble à une pétition de principe pour le correcteur pressé.

Le moteur fournit la charpente — l'annonce, les étapes nommées, la conclusion quand la logique l'exige — et vous n'écrivez que les mathématiques. Les clôtures suivent l'usage réel : seules celles qui **font partie du raisonnement** sont produites (la récurrence invoque son principe, l'absurde conclut de la contradiction, la disjonction constate que les cas couvrent tout), et jamais de « ce qui achève la démonstration ».

| Raisonnement | Écriture | Étapes attendues |
|---|---|---|
| directe | `<Montre>que …` | — |
| contraposée | `<Montre>par contraposée que …` | `contraposée{…}` |
| absurde | `<Montre>par l'absurde que …` | `absurde{…}`, et `contradiction{…}` en option |
| récurrence | `<Montre>par récurrence que …` | `initialisation{…}`, `hérédité{…}` |
| disjonction de cas | `<Montre>par disjonction de cas que …` | `cas ⟨description⟩ {…}`, répété |
| analyse-synthèse | `<Montre>par analyse-synthèse que …` | `analyse{…}`, `synthèse{…}` |
| double implication | `<Montre>par double implication que …` | `directe{…}`, `réciproque{…}` |
| double inclusion | `<Montre>par double inclusion que …` | `directe{…}`, `réciproque{…}` |
| propriété universelle | `<Montre>par élément quelconque …` | `soit{…}` |
| tiroirs | `<Montre>par le principe des tiroirs que …` | `objets{…}`, `tiroirs{…}` |
| existence et unicité | `<Montre>l'existence et l'unicité de …` | `existence{…}`, `unicité{…}` |

> ⚠️ **Double implication ou double inclusion ?** Deux lettres les séparent, et
> elles ne démontrent pas la même chose.
>
> - La **double implication** démontre une **équivalence entre deux
>   propositions** : « *f* est injective **si et seulement si** son noyau est
>   réduit au vecteur nul ». On prouve une implication, puis sa réciproque.
> - La **double inclusion** démontre une **égalité entre deux ensembles** : « le
>   complémentaire d'une réunion **est** l'intersection des complémentaires ».
>   On prouve *A* ⊂ *B*, puis *B* ⊂ *A*.
>
> Un énoncé en « si et seulement si » ou en « équivaut à » relève de la première.
> docdg le vérifie : écrire `par double inclusion` sur une équivalence est
> refusé, avec le nom du raisonnement qui convient.
>
> **Quel sens est le « direct » ?** Pour « *P* si et seulement si *Q* », le sens
> direct est *P* ⟹ *Q*, et la réciproque *Q* ⟹ *P*. Les étapes portent le nom de
> la **direction**, non celui du rang — `directe{…}` et `réciproque{…}` —, si
> bien qu'inverser les deux temps dans la source inverse aussi les intitulés :
> le document ne peut pas mentir sur ce qu'il démontre.
>
> **Ce que la double implication n'est pas.** Une **chaîne d'équivalences**
> (« *x* ∈ *A* ⟺ … ⟺ *x* ∈ *B* ») ne se coupe pas en deux temps : elle
> s'écrit au fil de la prose, en raisonnement **direct**. Et l'équivalence de
> **trois propositions ou plus** se démontre en cycle — (i) ⟹ (ii) ⟹ (iii) ⟹
> (i) — ce que docdg ne charpente pas encore : écrivez-la en raisonnement
> direct.

**Exemple :**

```docdg
<Montre>par récurrence que pour tout entier $n$, $somme(k=0;n) k = (n(n+1))/2$ {
	initialisation{
		Pour $n = 0$, les deux membres valent $0$.
	}
	hérédité{
		Soit $n$ un entier pour lequel la propriété est vraie.
		Alors $somme(k=0;n+1) k = (n(n+1))/2 + (n+1) = ((n+1)(n+2))/2$.
	}
}
```

Le moteur annonce « Montrons par récurrence que… », étiquette **Initialisation** et **Hérédité**, et conclut : « La propriété est vraie au premier rang et héréditaire : d'après le principe de récurrence, … ». Une étape oubliée est signalée — `il manque l'étape « initialisation{…} »` — et un raisonnement inconnu liste les formes admises.

Le corps d'une étape est du docdg vivant : prose, mathématiques `$…$`, interpolations `#variable` et commandes du langage s'y composent librement.

### **La preuve logée dans l'énoncé**

Les dix formes valent aussi **à l'intérieur d'un environnement numéroté**, sous le mot **`démonstration`** : la preuve suit son énoncé au lieu d'en être séparée, et le renvoi porte sur le numéro. Le raisonnement se nomme après le mot, exactement comme après le verbe.

```docdg
<Énonce>le théorème <étiquette>{carre-impair} {
Le carré d'un entier impair est impair.

démonstration par élément quelconque entier impair $n$, $n^2$ est impair {
soit{
$n$ un entier impair, qui s'écrit donc $n = 2k + 1$ avec $k$ entier
}
Son carré vaut $n^2 = 4k^2 + 4k + 1 = 2(2k^2 + 2k) + 1$, qui est impair.
}
}
```

Le mot **`démonstration` seul** suffit lorsque la preuve n'a pas de charpente à nommer : le moteur la compose sous son intitulé, sans annonce ni conclusion — c'est la forme des corollaires immédiats. Et **`démonstration que …`** restitue la propriété à démontrer quand l'énoncé compte plusieurs phrases.

### **Trois documents, trois usages**

| Document | Ce qu'il montre |
|---|---|
| `exemples/demonstration2.txt` | **l'auteur écrit, le moteur charpente** — les dix raisonnements écrits à la main, plus la preuve logée dans l'énoncé |
| `exemples/demonstration3.txt` | **le moteur démontre seul** au lycée — aucun corps entre accolades |
| `exemples/demonstration4.txt` | **le moteur démontre seul** dans le supérieur — quatre-vingt-dix des cent une fiches de la bibliothèque y passent, les onze autres étant appelées par `demonstration3.txt` |

### **Trois sources, une seule rédaction**

Une démonstration peut venir de trois endroits, et sort toujours dans la même langue :

| Source | Quand | Ce qu'elle apporte |
|---|---|---|
| **Vous** | vous écrivez le corps entre accolades | les mathématiques ; le moteur fournit la charpente |
| **Le calcul formel** (SymPy) | pas de corps, et l'énoncé est calculable | la vérification **et** la rédaction, chaîne d'égalités comprise |
| **La bibliothèque** (`demonstrations.json`) | pas de corps, et l'énoncé y figure | les démonstrations d'idée, que rien ne calcule |

**Sans corps, le moteur démontre lui-même** ce qui est à la portée du calcul formel — comme partout dans le langage, l'absence d'accolades signifie que le moteur fait le travail :

```docdg
<Montre>par récurrence que pour tout entier $n$, $somme(k=0;n) k = (n(n+1))/2$
<Montre>par élément quelconque réel $x$, $x^2 + 1 >= 2x$
```

La première vérifie l'initialisation et l'hérédité par le calcul, puis rédige la démonstration complète ; la seconde établit l'inégalité par la forme canonique. Une formule fausse est **refusée avec la raison** — le moteur ne démontre pas un mensonge.

**Ce que le calcul n'atteint pas, la bibliothèque le fournit.** Une idée ne se calcule pas : cent une démonstrations classiques — treize au lycée, vingt-neuf en L1, vingt-cinq en L2, trente-cinq en classes préparatoires — sont écrites une fois et appelées par leur seul énoncé. La base ne contient **que** cela — tout ce qu'une identité ou une inégalité vérifiable démontre relève de SymPy, jamais d'une fiche.

```docdg
<Montre>que $racine(2)$ est irrationnel
<Montre>qu'il existe une infinité de nombres premiers
<Montre>l'existence et l'unicité de la division euclidienne
```

La base ne contient **que** ce qu'aucun outil ne sait faire : une idée, une construction, un argument de structure. Tout ce qu'une identité ou une inégalité vérifiable démontre relève de SymPy — deux sources pour un même résultat finiraient par diverger. On y trouve donc l'irrationalité de √2, le théorème de Cantor, la divergence de la série harmonique, la caractérisation d'un sous-groupe ; on n'y trouve ni la formule du binôme ni l'inégalité de Bernoulli, qui se vérifient par le calcul.

L'énoncé est reconnu après normalisation — accents, mathématiques et ponctuation n'y font rien. Un raisonnement précisé dans la balise l'emporte sur celui de la fiche, et un énoncé absent fait proposer les plus proches. La base vit dans `transpiler/src/maths/demonstrations.json` et suit les versions du logiciel : chaque fiche porte son identifiant, ses clés, son niveau, son raisonnement et son corps **en docdg**.

<a id="les-actions"></a>

## **✏️ Les actions**

### **`<Soit>` — Poser des hypothèses de départ**

**Rôle :** `<Soit>` déclare un ou plusieurs éléments ou hypothèses d'un coup (points, valeurs, fonctions, champs, systèmes...) *et les affiche* dans le document sous la forme d'un énoncé « Soit ... ». `soit` en minuscule, lui, assigne silencieusement, sans rien afficher (voir « Alias et macros »). C'est le même verbe sous deux formes — la casse suit la règle n°9.

**Syntaxe (plusieurs éléments) :**

```docdg
<Soit>{
	un point A(2;-1,5)
	un point B(-4,1;3,4)
}
```

**Syntaxe (un seul élément) :** comme pour `<Trace>`, le contenu suit directement le chevron fermant sans accolades — celles-ci ne servent qu'à regrouper plusieurs lignes.

```docdg
<Soit>un point A(2;-1,5)
```

**Affiche :**

```
Soit A le point de coordonnées (2 ; -1,5).
```

La forme en bloc, elle, affiche un énoncé par ligne :

```
Soit A le point de coordonnées (2 ; -1,5).
Soit B le point de coordonnées (-4,1 ; 3,4).
```

**L'ordre suit l'usage du supérieur : le nom, puis la nature.** « Soit A le point de coordonnées (2 ; 3) », et non « Soit le point A(2 ; 3) ». Le pluriel accorde l'impératif — « Soient A et B les points de coordonnées … ». Chaque déclaration est une **phrase**, non une formule affichée :

| Déclaration | Rendu |
|---|---|
| `<Soit>une fonction f(x) = x^2 - 2` | Soit *f* la fonction définie par *f*(*x*) = *x*² − 2. |
| `<Soit>les fonctions f(x) = x^2 et g(x) = -x^2` | Soient *f* et *g* les fonctions définies par … |
| `<Soit>un vecteur u(3;-2)` | Soit *u⃗* le vecteur de coordonnées (3 ; −2). |
| `<Soit>la matrice M{…}` | Soit *M* la matrice définie par : puis la matrice hors texte |
| `<Soit>le système s{…}` | Soit (*s*) le système : puis le système hors texte |

Le système garde l'ordre inverse : son nom est une étiquette parenthésée, non le nom propre de l'objet.

Les points ainsi déclarés peuvent ensuite être repris par `<Trace>` (voir plus bas) sans qu'il soit nécessaire de redonner leurs coordonnées.

**La factorisation plurielle, sous quatre formes équivalentes.** Une seule phrase déclare plusieurs points ; l'article pluriel (`les`, `des`) ou un **nombre** ouvre la liste, et les coordonnées se donnent soit accolées à chaque nom, soit regroupées en fin de phrase par la **distribution respective** — la tournure des énoncés de collège, de lycée et du supérieur (« de coordonnées respectives », « de rayons respectifs », « de probabilités respectives ») :

```docdg
<Soit>les points A(1;2), B(-1;2) et C(-1;-2)
<Soit>3 points A(1;2), B(-1;2) et C(-1;-2)
<Soit>les points A, B et C de coordonnées respectives (1;2), (-1;2) et (-1;-2)
<Soit>3 points A, B et C de coordonnées respectives (1;2), (-1;2) et (-1;-2)
```

Les quatre phrases posent exactement les trois mêmes points. Deux garde-fous, dits en français : le **compte** annoncé par le nombre doit correspondre au nombre de points décrits, et la distribution respective exige **autant de couples de coordonnées que de noms**, appariés dans l'ordre. La factorisation vaut aussi pour les **fonctions** — une seule phrase les pose toutes :

```docdg
<Soit>les fonctions f(x) = exp(-x^2), g(x) = -x^4 + 2x^2 + 1 et h(x) = (x+1)/(x-2)
```

et chacune est enregistrée comme si elle avait été posée seule (tableaux de variations, dérivées, zéros, tracé : tout suit). Les coefficients décimaux s'écrivent à la française (`-4,9t^2 + 20t`) : la virgule décimale, encadrée de chiffres, n'est jamais confondue avec la virgule qui sépare les fonctions. Elle vaut de même pour les **phrases calculantes** — `<Calcule>la dérivée de f, g et h` vaut trois calculs, un par fonction, et de même pour la dérivée seconde, la primitive, les zéros... — et pour le **tracé** : `<Représente>graphiquement les fonctions f, g et h pour x dans [-2 ; 2] et y dans [-3 ; 3]` dessine les trois courbes dans le même repère, chacune avec sa couleur et sa légende. La distribution respective sert aussi au repère (`avec des unités respectives de 2 cm et 0,5 cm`, voir `<Trace>`) ; son extension aux autres objets (sphères, événements) suivra au fil des versions.

> **Règle de l'article :** indéfini à la déclaration (`<Soit>` : `un point A`), défini à toute reprise ultérieure (`<Trace>` ou une autre action : `le point A`).

**Le champ de vecteurs.** Pour les classes préparatoires, `<Soit>` sait déclarer un champ :

```docdg
<Soit>un champ de vecteurs F de RR^3 dans RR^3 défini par F(x, y, z) = (x^2, yz, sin(z))
```

qui s'affiche « Soit un champ de vecteurs F : ℝ³ → ℝ³ défini par F(x, y, z) = (x², yz, sin(z)) ». Les doubles lettres `RR`, `NN`, `ZZ`, `QQ`, `CC` produisent les ensembles en gras de tableau, comme partout dans les zones mathématiques. La déclaration est **purement notationnelle** : un champ vectoriel n'a ni courbe ni tableau de variations associables, il n'est donc pas enregistré comme les fonctions scalaires le sont.


### **`<Trace>` — Dessiner tout type de figure**

**Une seule action pour :** figures géométriques, courbes de fonctions, repères, cercles trigonométriques, solides 3D.

#### **🔺 Figures géométriques planes**

**Formes disponibles :**


| **Forme** | **Syntaxe**         | **Propriétés clés**                              |
| --------- | ------------------- | ------------------------------------------------ |
| Triangle  | `le triangle ABC équilatéral`, `le triangle ABC isocèle en A, de côté 5 cm`, `le triangle ABC rectangle en A, de côté AB 3 et de côté AC 4` | qualificatif accolé, attributs en `de...` ; l'isocèle prend `de côté` pour les deux côtés égaux et `de base` pour le troisième, qui vaut les trois quarts du côté s'il est omis |
| Carré     | `le carré ABCD, de côté 4 cm`        | `de côté N`                                         |
| Rectangle | `le rectangle ABCD, de côté AB 3 et de côté BC 5`    | `de côté AB N`, `de côté BC N`                      |
| Losange   | `le losange ABCD, de côté 4 cm et d'angle 60`      | `de côté N`, `d'angle N`                            |
| Cercle    | `le cercle O, de rayon 3 cm`  | `de centre A`, `de rayon N`, `de diamètre N`          |
| Disque    | `le disque O, de rayon 3 cm, rempli`  | `de centre A`, `de rayon N`, `de diamètre N`, `rempli` (flag) |
| Polygone  | `le polygone ABCD, de centre O et de rayon 3 cm` | `de centre A`, `de rayon N`                                         |


**Propriétés communes :**  
en prose — unité accolée au nombre (`5 cm`), couleur avec `en` (`en bleu`), épaisseur avec `de trait N` (`de trait 0,5 mm`), `avec les marques`, `sans labels` ;  
en clé:valeur (le seul réglage technique restant à ce niveau) — `rotation:N`

**Convention pour les longueurs :** toute propriété de longueur (`côté`, `rayon`, `diamètre`, `AB`, etc.) accepte soit un nombre suivi de son unité **obligatoire** (`5 cm`, `30 mm`), soit le nom d'un segment déjà défini entre deux points existants (ex. `de rayon AB`). Une longueur numérique sans unité n'est acceptée qu'à l'intérieur d'un repère, où elle se lit en graduations (règle n°10).

**Exemples :**

```docdg
<Trace>le triangle ABC équilatéral, de côté 5 cm, avec les marques
<Trace>le carré ABCD, de côté 4 cm, avec les marques
<Trace>le triangle ABC rectangle en A, de côté AB 3 mm et de côté AC 4 mm, avec les marques
```

**Le cercle en détail :**

```docdg
<Trace>le cercle O, de rayon 3 cm                 % rayon en cm
<Soit>les points A(0;0) et B(3;0)
<Trace>le cercle O, de rayon AB                  % rayon = longueur du segment AB
<Trace>le cercle O, de diamètre 6 cm             % via le diamètre plutôt que le rayon
<Trace>le cercle, de centre O et de rayon 4 cm   % centre nommé, sans coordonnées
<Trace>le cercle, de centre (2;3) et de rayon 4 cm  % centre positionné explicitement
<Trace>le disque O, de rayon 3 cm, rempli
```

Mêmes options pour `disque` (`de centre A`, `de rayon N` ou `de diamètre N`, `rempli`) et pour `polygone` (`de centre A`, `de rayon N` pour un polygone régulier inscrit).

#### **📊 Repère orthonormé**

**Syntaxe :** `un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5]`. Le repère est **orthonormé** par défaut : une graduation vaut la même longueur sur les deux axes (1 cm si rien n'est précisé), et l'adjectif `orthonormé` peut se dire explicitement.

**L'unité se règle en prose**, et son unité de longueur (cm ou mm) est obligatoire, comme pour toute mesure (règle n°10) :

- `avec une unité de 1,5 cm` — la même unité sur les deux axes, le repère reste orthonormé ;
- `avec des unités respectives de 2 cm et 0,5 cm` — la première pour l'axe des abscisses, la seconde pour celui des ordonnées (**distribution respective**, règle n°10) : le repère est alors **orthogonal**, et se dit tel.

**Exemples :**

```docdg
<Trace>dans un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5] {
	le point A(1;2)
}
<Trace>dans un repère orthonormé où l'abscisse appartient à [-4 ; 4] et l'ordonnée à [-3 ; 3], avec une unité de 1,5 cm {
	le point A(1;2)
}
<Trace>dans un repère orthogonal où l'abscisse appartient à [-3 ; 3] et l'ordonnée à [-3 ; 3], avec des unités graphiques de 2 cm pour l'axe des abscisses et de 0,5 cm pour l'axe des ordonnées {
	le point A(1;2)
}
```

Déclarer un repère `orthonormé` avec deux unités distinctes est une contradiction : docdg la refuse et rappelle la forme `orthogonal`. *(L'ancien réglage `unité:N` en clé:valeur, comme les réglages d'affichage `avec le quadrillage`, `avec les axes` et `avec les noms`, n'existent plus : l'unité est une grandeur qui se dit en prose, et les axes gradués sont constitutifs du repère.)*

#### **🎡 Cercle trigonométrique**

**Syntaxe :** `le cercle trigonométrique, de rayon 4 cm`, suivi du réglage d'affichage `avec les valeurs` / `sans valeurs`, et des réglages techniques `pas:N`, `plage:A;B` en clé:valeur.

**Exemple :**

```docdg
<Trace>le cercle trigonométrique, de rayon 4 cm, avec les valeurs
<Trace>le cercle trigonométrique, de rayon 3,2 cm, avec les valeurs et un pas de $pi/6$
```

#### **🎲 Solides en 3D**

**Formes :** `le cube`, `le pavé`, `la pyramide`, `le cylindre`, `le cône`, `la sphère`

**Attributs constitutifs (en prose « de... ») :** `d'arête N`, `de côté N;N;N`, `de base N`, `de hauteur N`, `de rayon N`, `de diamètre N`, `d'angle N`, `de coefficient N`

Même convention que pour les figures planes : `arête`, `rayon`, `diamètre`, `hauteur` acceptent une valeur en cm ou le nom d'un segment déjà défini.

> ✅ **Arêtes cachées calculées automatiquement !**

**Exemple :**

```docdg
<Trace>{
    le solide cube ABCDEFGH, d'arête 3 cm
    le solide pyramide SABCD, de base 3 cm et de hauteur 4 cm
    le solide cylindre, de rayon 1,5 cm et de hauteur 3 cm
    le solide sphère, de centre (0;0;0) et de rayon 2 cm
    le solide cône, de rayon 2 cm et de hauteur 5 cm
}
```

#### **📍 Points, droites, demi-droites et segments**

Pour tracer un élément isolé, sans ouvrir un repère complet :

```docdg
<Soit>un point A(2;-1,5)
<Place>le point A                    % A doit avoir été défini au préalable (ex. via <Soit>)
<Place>un point B(3 ; 1)             % place B : le déclare et le marque
<Place>les points A, B et C tels que AB = 5 cm, BC = 4 cm et AC = 3 cm
<Trace>la droite (AB)
<Trace>la demi-droite [AB)
<Trace>la demi-droite (AB]
<Trace>le segment de droite [AB]
```

**Placer par les distances.** En géométrie, un point se donne rarement par ses
coordonnées : il se construit au compas, à partir de ses distances aux autres.
`<Place>les points A, B et C tels que AB = 5 cm, BC = 4 cm et AC = 3 cm` pose
les trois points à ces longueurs exactes — le premier à l'origine, le deuxième
sur l'horizontale, les suivants par intersection de deux cercles. Les points
sont déclarés comme par `<Soit>`, donc les figures suivantes peuvent s'y
référer. Trois longueurs incompatibles — un côté plus long que la somme des
deux autres — laissent le triangle introuvable, et docdg le dit.

**Convention pour les droites :** `[` / `]` = point inclus, `(` / `)` = point exclu — la même convention que pour la [droite graduée](#-droite-graduée).

La même prose (`la droite (AB)`, `la demi-droite [AB)`, `le segment de droite [AB]`) fonctionne aussi bien **dans un repère**, dès que les points portent des coordonnées connues (posés par `<Soit>`, ou placés dans le même bloc) : la droite et la demi-droite s'arrêtent alors au bord du repère plutôt qu'à une longueur fixe, et le segment relie exactement les deux points. C'est la même phrase, seul le contexte — avec ou sans repère ouvert — décide de l'échelle.

**Construction avec longueur imposée.** Le geste de base au collège — tracer un segment ou une demi-droite d'une longueur donnée — s'écrit avec la clause `tel que` (ou `telle que`), suivie de la longueur avec son unité :

```docdg
<Trace>le segment de droite [AB] tel que AB = 4 cm
<Trace>le segment [CD] tel que CD = 28 mm
<Trace>la demi-droite (AB] telle que AB = 3,5 cm
```

Le segment est dessiné à l'échelle (1 cm = 1 cm sur le papier), ses extrémités nommées et pointées, la longueur inscrite au-dessus. L'unité (`cm` ou `mm`) est **obligatoire** : `tel que AB = 5` seul est refusé, avec un message qui le rappelle. Sans clause `tel que`, le segment garde une longueur d'affichage par défaut.

#### **📐 Mode analytique**

Pour construire une figure complète dans une partie (ou un cadre) — plusieurs points, droites, courbes, vecteurs liés entre eux — **avec ou sans repère**, on ouvre un bloc `<Trace>{...}` : tout ce qui se trouve entre les accolades sera dessiné dans le même canevas. Si la première phrase du bloc est **le repère lui-même** — `un repère où l'abscisse appartient à [a ; b] et l'ordonnée à [c ; d]` — ses bornes fixent le canevas, et tout ce qui suit s'y inscrit ; sans cette phrase, la figure se construit librement, hors de tout repère. Le repère se dit `orthonormé` explicitement si l'on veut le préciser (c'est le défaut) ; le repère `orthogonal` prend des unités distinctes sur les deux axes : `un repère orthogonal où ..., avec des unités respectives de 2 cm et 0,5 cm` (abscisses puis ordonnées).

**Syntaxe :**

```docdg
<Trace>dans un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5] {
	le point A(-1;0)
	le point B(2;3)
	la droite y = 2x + 1
	la droite passant par A et B
	le vecteur u(1;2)
	le cercle C, de centre (1;-2) et de rayon 1,5
	la région y > x + 1
}
```

**Médiatrice, bissectrice et angle.** Trois constructions de collège s'écrivent dans le repère, en prose ordinaire, à condition que les points concernés y soient posés :

```docdg
<Trace>dans un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5]{
    un point A(-3;1)
    un point B(3;-1)
    un point C(2;4)
    la médiatrice de [AB], en bleu
    la bissectrice de l'angle ABC, avec les marques
    l'angle BAC, en vert
}
```

- **La médiatrice** se dit `la médiatrice de [AB]` — le segment entre crochets, notation française, cohérente avec la droite `(AB)`. Elle se trace comme droite complète, coupée aux bords du repère. `avec les marques` ajoute le codage d'égalité sur les deux moitiés du segment et le petit carré de l'angle droit au milieu.
- **La bissectrice** se dit `la bissectrice de l'angle ABC` — le **sommet est la lettre centrale** (ici `B`), convention géométrique française. Elle se trace comme demi-droite issue du sommet, prolongée jusqu'au bord du repère. `avec les marques` ajoute les deux arcs égaux de part et d'autre.
- **L'angle** seul, `l'angle BAC`, trace la marque d'arc au sommet (lettre centrale) — utile pour désigner un angle sur une figure sans le bissecter. Ne pas confondre avec l'*attribut* `d'angle N` (une mesure, en degrés) du losange ou du parallélogramme.

> Les points posés par un `<Soit>` antérieur — au singulier (`un point A(2;7)`) comme au pluriel factorisé (`les points A(-3;1), B(3;-1) et C(2;4)`) — **persistent** : tout bloc `<Trace>` qui les nomme les reçoit automatiquement. La médiatrice et la bissectrice sur figure métrique (hors repère, `le triangle ABC...`) restent prévues pour une version ultérieure.

**L'angle sous toutes ses formes.** Au-delà de `l'angle BAC` (trois points, sommet central), le repère accepte :

```docdg
<Trace>dans un repère où l'abscisse appartient à [-2 ; 2] et l'ordonnée à [-2 ; 2]{
    un point O(0;0)
    un point A(1;0)
    un point B(0;1)
    un point C(1;1)
    l'angle AOB, en bleu et de mesure 90°
    l'angle orienté (OA;OB), en vert
    l'angle entre OA et OC, en rouge
    le vecteur u(1;0)
    le vecteur v(0;1)
    l'angle entre le vecteur u et le vecteur v, en orange et de mesure 90°
}
```

- **`de mesure 90°`** pose la mesure en étiquette, sur la direction médiane de l'arc, un peu au-delà. La valeur est reprise telle quelle (`90°`, `pi/3`, `60°`...) — docdg ne vérifie pas qu'elle est exacte : c'est une annotation, pas un calcul.
- **`l'angle orienté (OA;OB)`** — deux demi-droites de même origine, notation française du couple — trace l'arc **fléché** de la première vers la seconde. Les deux origines doivent coïncider, sinon l'erreur le dit.
- **`l'angle entre OA et OC`** est la forme non orientée de la même construction.
- **`le vecteur u(1;0)`** pose un vecteur libre, tracé depuis l'origine ; **`l'angle entre le vecteur u et le vecteur v`** marque l'arc entre leurs directions, au point de départ du premier. Les deux vecteurs doivent avoir été tracés dans le même bloc.

> **Périmètre du moteur de figures — et feuille de route.** Le moteur de figures est un moteur **plan**. L'angle dièdre entre deux plans, le plan défini par trois points de l'espace, l'angle entre une droite et un plan — toutes les constructions qui exigent une vraie troisième dimension — sont notés pour une version ultérieure dotée d'un moteur 3D (les solides actuels sont des projections câblées, pas un espace calculé). De même, `la droite d, passant par A et B` comme objet *nommé* réutilisable attend le mécanisme d'alias de figures (`soit d = ...`), prévu avec la persistance des points entre blocs.

> **Un seul verbe, une seule prose.** Le mode analytique n'introduit aucune syntaxe nouvelle : c'est un `<Trace>le repère, ...` ordinaire (règle n°10, article défini, bornes en prose `où l'abscisse appartient à [a ; b] et l'ordonnée à [c ; d]`) dont le contenu entre accolades énumère les objets à inscrire dans ce repère, chacun à son tour avec sa propre prose. Une ébauche antérieure du langage ouvrait ce mode par un réglage `axes:{...}` en clé:valeur ; mais les bornes décrivent *ce que montre* la figure, au même titre qu'un rayon ou un côté — elles sont donc en prose, et le mot `axes:` n'existe pas. On n'écrit jamais `<Trace, où l'abscisse ...>` (virgule après un verbe sans objet, incorrecte en français) : le repère est toujours nommé (`le repère`), exactement comme on nomme `le cercle` ou `la fonction`. Le même repère se dit aussi bien `<Représente>graphiquement le repère, ...` dès que son contenu relève de l'analyse (courbes de fonctions) plutôt que de la géométrie plane — voir l'encadré ci-dessous.

**Le repère est un objet : il accueille des courbes de fonctions.** Une fonction posée par `<Soit>` se trace *dans* un repère qui contient aussi des points, des droites ou des vecteurs — la courbe est un habitant du repère parmi les autres :

```docdg
<Soit>les fonctions f(x) = x^2 - 2 et g(x) = -x^2 + 3

<Représente>graphiquement un repère où l'abscisse appartient à [-3 ; 3] et l'ordonnée à [-4 ; 4]{
    les courbes des fonctions f et g
    la droite d'équation y = x
    le point A(-2 ; 3,5)
}
```

Chaque courbe reçoit sa couleur et son nom en étiquette. Trois proses équivalentes pour la courbe : `la fonction f`, `la courbe de la fonction f`, et au pluriel factorisé `les courbes des fonctions f et g` (ou `les fonctions f et g`). La droite se dit par son équation, `la droite d'équation y = ...`, avec n'importe quelle expression au second membre. La fenêtre et les unités viennent du repère englobant — la courbe n'en redit rien.

**La forme académique du repère.** Les énoncés du secondaire et du supérieur nomment le repère et détaillent chaque axe ; docdg les lit tels quels :

```docdg
<Représente>graphiquement dans un repère orthogonal (O, i, j) pour x appartient à [-3 ; 3] et y à [-9 ; 9] avec des unités graphiques de 1,5 cm pour l'axe des abscisses et de 0,5 cm pour l'axe des ordonnées{
    les courbes des fonctions f et g
    la droite d'équation y = x
    le point A(-2 ; 3,5)
}
```

- **`<Représente>graphiquement ...`** est le verbe de l'**analyse** : fonction, courbe, diagramme en escalier d'une suite, statistique (voir plus loin) — la représentation d'un concept abstrait, avec ou sans repère explicite. `<Trace>` reste le verbe de la **géométrie**, plane ou dans l'espace : triangle, cercle, droite, polygone, médiatrice, bissectrice, solide — le tracé concret d'une figure, **même posée dans un repère** (un point, une droite ou une médiatrice entre coordonnées se trace, elle ne se représente pas graphiquement). Les deux verbes peuvent ouvrir le même mode analytique et les mêmes blocs ; seul le contenu — fonction ou figure — décide lequel s'emploie. Une figure mixte (une droite tracée à côté d'une courbe, par exemple) se range du côté de son objet principal.

| Domaine | Verbe | Exemples |
| --- | --- | --- |
| Analyse | `<Représente>graphiquement` | fonctions, courbes, diagrammes en escalier, statistiques |
| Géométrie plane | `<Trace>` | triangles, cercles, droites, polygones, médiatrices |
| Géométrie dans l'espace | `<Trace>` | cubes, pyramides, sphères |
| Probabilités | `<Construis>` | arbres de probabilités |
| Algèbre linéaire | `<Affiche>` | matrices, tableaux |

- **`dans un repère ...`** : le `dans` de la tournure académique se lit naturellement après le verbe.
- **`(O, i, j)`** nomme le repère — la notation est acceptée et consommée ; elle n'ajoute rien au tracé (l'origine s'appelle toujours O).
- **`pour x appartient à [a ; b] et y à [c ; d]`** est l'équivalent de `où l'abscisse appartient à ... et l'ordonnée à ...` — les deux tournures se valent.
- **`avec des unités graphiques de 1,5 cm pour l'axe des abscisses et de 0,5 cm pour l'axe des ordonnées`** est la forme longue, chaque axe nommé en toutes lettres, de `avec des unités respectives de 1,5 cm et 0,5 cm` — l'ordre des axes peut être inversé, les valeurs se remettent d'elles-mêmes dans le bon ordre.

---

<a id="les-saisies-interactives"></a>

## **🖊️ Les saisies interactives**

**La grande nouveauté de docdg 2.0** : un document docdg peut poser des questions au lecteur, et se construire avec ses réponses. C'est ce qu'aucun document figé — LaTeX, Typst, PDF statique — ne peut faire : puisque docdg compose lui-même son rendu, le document devient **vivant**.

### La balise `<Saisis>`

Une saisie se déclare comme une définition ordinaire, avec le type attendu en toutes lettres et la question entre accolades :

```docdg
soit prénom = <Saisis>une chaîne de caractères{Quel est le prénom de l'élève ?}
soit âge = <Saisis>un entier{Quel est son âge ?}
soit taille = <Saisis>un décimal{Quelle est sa taille, en mètres ?}
soit marié = <Saisis>un booléen{Est-il marié ?}
soit initiale = <Saisis>un caractère{Quelle est sa lettre préférée ?}
```

Cinq types, et cinq seulement : **une chaîne de caractères** (non vide), **un entier** (sans virgule), **un décimal** (écrit avec **la virgule**, jamais le point — `1,65`), **un booléen** (`vrai` ou `faux`, en toutes lettres), **un caractère** (un seul).

### Le blocage typé

**Le document s'arrête à la première question sans réponse valide.** Rien de ce qui suit ne s'affiche — ni le texte, ni les questions suivantes, ni les calculs qui dépendent de la réponse. Tant que la saisie ne correspond pas au type, un message d'erreur temporaire s'affiche sous le champ (« Un nombre décimal est attendu, écrit avec une virgule — par exemple 1,65. ») puis s'estompe. Les questions se posent donc l'une après l'autre, dans l'ordre du document.

Une valeur validée s'affiche en bleu ; **cliquer dessus rouvre la question** — le document se replie jusqu'à elle et se déroule de nouveau avec la nouvelle réponse.

### Les réponses sont des valeurs comme les autres

Une saisie s'affiche avec `#nom` et entre dans les calculs comme n'importe quelle variable :

```docdg
soit prénom = "Iris"
soit âge = 9
soit taille = 1,32
soit a = 3
#prénom a #âge ans et mesure #taille m, soit #{taille * 100} cm.
<Trace>le solide cube, d'arête #a cm
```

Un booléen s'affiche `vrai`/`faux` mais vaut 1/0 dans les conditions ; les descriptions de balises interpolent aussi les `#`, si bien qu'un solide, un calcul ou une figure peuvent dépendre d'une réponse du lecteur.

### L'alternative : le bloc et la ligne

L'alternative existe sous deux formes. **Le bloc**, pour dérouler du contenu :

```docdg
si marié {
	Il est marié.
} sinon {
	Il n'est pas marié.
}
```

**La ligne**, pour produire une valeur — du texte ou un nombre :

```docdg
soit statut = si marié { marié } sinon { célibataire }
soit tarif = si âge moins de 18 { 5 } sinon { 9 }
```

La condition s'écrit nue (`si marié` — vrai si non nul), avec `vaut vrai` / `vaut faux`, ou avec tout comparateur du langage : `vaut`, `moins de`, `plus de`, `au moins`, `au plus`, `différent de`. Une valeur produite en ligne se comporte comme les autres : `#statut` l'affiche, `#{tarif * 2}` la calcule.

---

## **🎨 Les styles et la mise en forme**

### **🎨 Styles en ligne**


| **Style**           | **Résultat**      |
| ------------------- | ----------------- |
| `gras`              | **gras**          |
| `italique`          | *italique*        |
| `souligné`          | ++souligné++      |
| `barré`             | ~~barré~~         |
| `sans empattements` | sans empattements |
| `petites capitales` | Petites Capitales |


**Combinaison :** `<gras italique rouge>{texte}` ou `<g i s>{texte}`

### **🎨 Couleurs et tailles**

**Couleurs :** `rouge`, `bleu foncé`, `vert forêt` — les 148 noms français listés dans la [référence des couleurs](#-référence-des-couleurs-148-disponibles), avec leurs accords. Dans une balise de style, **la couleur nue est celle du texte**, et `fond <couleur>` celle du fond : `<gris clair>` écrit en gris clair, `<fond gris clair>` pose un fond gris clair. Un style nommé les combine — `soit correction = <fond gris clair>`.

**Filet de sécurité — teinte absente de la palette française :** les 148 noms français couvrent l'intégralité de la palette CSS usuelle ; il ne devrait donc jamais être nécessaire d'en sortir. Aucun exemple de ce guide n'utilise de nom anglais — docdg se lit et s'écrit intégralement en français.

**Tailles :** `12pt`, `14pt`

**Polices :** `<ARIAL>`, `<TIMES NEW ROMAN 12pt>`

### **📏 Alignements**

**Horizontal :** `à gauche`, `au centre`, `à droite`, `justifié`

**Vertical (dans cellules) :** `hg`, `hc`, `hd`, `mg`, `mc`, `md`, `bg`, `bc`, `bd` (alias en toutes lettres : voir [Les objets - le tableau](#2-dresse-un-tableau--tableaux-simplifiés))

### **📄 Tabulations et sauts**


L'alinéa s'écrit par une **tabulation en tête de ligne** — une tabulation, un
alinéa ; deux tabulations, deux alinéas. Le saut de ligne s'écrit par une
**ligne vide**. Ni l'un ni l'autre n'ont de balise : le source dit l'espace
comme il dit le texte.

| **Balise**        | **Description**              |
| ----------------- | ---------------------------- |
| `<page suivante>` | Saut de page                 |


### **📌 Exposants et indices**

```docdg
x<exposant>{2}      % x² (décalage par défaut de la classe)
y<indice>{1}        % y₁
H<indice>{2}O       % H₂O
x<exposant 4mm>{2}  % x² avec exposant relevé de 4 mm (mesure explicite)
```

> **Nombre collé ou séparé ?** Deux familles, deux conventions, une seule logique. Quand le nombre **compte des objets** (une multiplicité), il est **collé** au mot : `<4 colonnes>` (voir « Fusion de cellules »). Quand le nombre est une **mesure** avec une unité (mm), il est **séparé** et porte son unité : `<exposant 4mm>`. Sans nombre, `<exposant>` et `<indice>` prennent le décalage par défaut fixé par l'option de classe `décalage` — c'est la forme courante, le nombre n'étant utile que pour un ajustement ponctuel.

### **📎 `<note>` — Notes de bas de page**

Une note de bas de page s'insère au fil du texte avec la balise `<note>` suivie de son contenu entre accolades. L'appel de note (le petit numéro en exposant) se place exactement là où figure la balise ; le texte de la note descend automatiquement en bas de la page, numéroté en continu :

```docdg
Selon Bourdieu<note>{BOURDIEU Pierre, La distinction : critique sociale
du jugement, Paris, Éditions de Minuit, 1979}, les habitus structurent
les pratiques.
```

Trois points d'usage, hérités de la typographie française :

- **Placement de la balise :** collée au mot qu'elle annote, *avant* la ponctuation (`...Bourdieu<note>{...},` et non `...Bourdieu,<note>{...}`) — c'est la convention française de l'appel de note.
- **Contenu :** tout ce que le corps du texte accepte est admis dans la note — styles (`<italique>`), mathématiques (`$...$`), calculs (`#{...}`). Une référence bibliographique s'écrit naturellement : NOM Prénom, *Titre*, Ville, Éditeur, année.
- **Longueur :** une note tient sur son accolade, même répartie sur plusieurs lignes du fichier source (l'accolade fermante marque la fin, comme partout ailleurs — règle n°2).

> **Pourquoi `<note>` et pas « note de bas de page » en toutes lettres ?** Le mot seul suffit : il n'existe qu'une sorte de note dans un document scolaire, et la brièveté compte pour une balise appelée au fil de la phrase. La marge (`marginpar`) et les notes de fin de document, plus rares, ne sont pas retenues.

### **🔖 Alias et macros avec `soit`**

> **⚠️ Distinction importante :**
> - `soit` (minuscule) : **Mot-clé technique** pour les assignations (invisible dans le PDF).
>   Exemple : `soit x = 5`
> - `<Soit>` (majuscule) : **Action-phrase** qui affiche un énoncé dans le PDF.
>   Exemple : `<Soit>un point A`
>
> Cette distinction suit la **règle n°9** : seules les actions-phrases (qui s’écrivent comme des phrases françaises) commencent par une majuscule.

**Mauvaise et bonne pratiques**
```docdg
-- ❌ MAUVAIS : "Soit" en majuscule pour une assignation
Soit titre = <gras 18pt au centre>  % → Erreur : ce n'est pas une action-phrase !

-- ✅ BON : majuscule uniquement pour l'action
soit titre = <gras 18pt au centre>
<Soit>un point A(2;3)  % → Correct : action-phrase
```

**Définir un style :**

```docdg
soit titre = <bleu marine gras 18pt au centre>
soit important = <gras souligné rouge>
```

**Variables numériques :**

```docdg
soit pi = 3,14159
soit rayon = 5
soit aire = #{pi * rayon^2}
L'aire vaut #aire cm².
```

**Macros avec paramètres :**

```docdg
soit bonjour{nom} = Bonjour #nom !
soit carre{x} = #{x^2}

<bonjour{Jean}>    % → Bonjour Jean !
<carre{5}>        % → 25
```

**Exemple complet**
```docdg
-- Assignation silencieuse (mot-clé technique)
soit titre = <gras 18pt au centre>
soit pi = 3,14159
soit bonjour{nom} = Bonjour #nom !

-- Action-phrase (affiche dans le PDF)
<Soit>un point A(2;3)
<Trace>le cercle, de centre A et de rayon 5 cm
```

### **🔁 Structures de contrôle**

> **💡 Les deux branches s'accordent en nature.** Dans `soit statut = si marié { marié } sinon { célibataire }`, si l'une des deux branches est du texte, le résultat est du texte — même lorsqu'un mot de branche porte le nom d'une valeur déjà définie. Deux branches numériques donnent un nombre, calculable comme les autres.


**Boucles `pour` :**

```docdg
pour n de 1 à 5 {
  <gras>Exercice #n
}
```

**Les deux bornes sont incluses** : `pour n de 1 à 5` parcourt 1, 2, 3, 4 **et** 5 (cinq passages), exactement comme on le lirait à voix haute. Contrairement à la plupart des langages de programmation (où une borne « à N » exclut souvent N), docdg ne demande pas à un professeur de compter « il faut mettre 6 pour aller jusqu'à 5 » — la phrase française fait foi.

```docdg
pour f dans {chat.png ; chien.png} {
  <Insère l'image IMAGES/#f avec une largeur de 20 mm>
}
```

**Le pas** se règle en prose, comme tout réglage : `avec un pas de p`. Les bornes et le pas acceptent les **décimaux**, virgule française comprise :

```docdg
pour i de 0,5 à 2,5 avec un pas de 0,5 {
  Valeur : #i
}
```

**Boucles `tant que` :** la condition est vérifiée **avant** chaque passage ; le mot `faire` devant l'accolade est facultatif mais recommandé — il se lit mieux.

```docdg
soit n = 0
tant que n < 3 faire {
  Passage numéro #n
  n = n + 1
}
```

**Boucles `faire ... tant que` :** le corps s'exécute **au moins une fois**, la condition est vérifiée **après** chaque passage.

```docdg
soit n = 0
faire {
  Passage numéro #n
  n = n + 1
} tant que n < 3
```

> **Compteurs et portées.** `soit` **déclare**, une fois ; `n = n + 1` **réaffecte** un nom déjà posé, et c'est ce qui fait avancer la boucle. Le répéter à chaque tour n'apprendrait rien. La condition qui rend cette écriture sûre tient en un mot : le nom doit déjà exister — une ligne de prose contenant un signe égal n'est donc jamais prise pour une affectation. Un nom posé pour la première fois *à l'intérieur* d'un bloc, lui, reste local au bloc (règle des portées, plus bas) : ce qu'on pose en tête sert partout, ce qu'on pose dans un tiroir reste dans le tiroir.

**Conditions `si` :**

```docdg
soit note = 15

si note >= 10 {
  <vert gras>Admis !
} sinon {
  <rouge gras>Échoué.
}
```

Les opérateurs symboliques (`>=`, `<=`, `==`, `!=`) ont chacun un synonyme en toutes lettres, interchangeable au choix :

| Symbole | Synonyme       | Symbole | Synonyme     |
| ------- | -------------- | ------- | ------------ |
| `>=`    | `au moins`     | `<=`    | `au plus`    |
| `==`    | `égal à`       | `!=`    | `différent de` |

```docdg
si note au moins 10 {          % strictement équivalent à : si note >= 10
  <vert gras>Admis !
}
```

Les conditions se **combinent** avec les connecteurs du français, `et` et `ou`, et s'enchaînent avec `sinon si` :

```docdg
soit note = 12
soit rattrapage = 8

si note >= 10 et rattrapage >= 10 {
  <vert gras>Admis sans réserve.
} sinon si note >= 10 ou rattrapage >= 12 {
  <orange gras>Admis de justesse.
} sinon {
  <rouge gras>Échoué.
}
```

### **➗ Calculs dans le texte : `#{...}`**

**Fonctions disponibles :**

- Algébriques : `abs`, `racine`, `defaut`, `signe`, `min`, `max`
- Exponentielles : `exp`, `ln`, `log`, `log2`, `logb(x;base)`
- Trigonométriques : `sin`, `cos`, `tan`, `cot`, `sinus en degrés`, `cosinus en degrés`, `tangente en degrés`
- Inverses : `arcsin`, `arccos`, `arctan`
- Hyperboliques : `sinh`, `cosh`, `tanh`, `coth`
- Arrondi : `arrondi(x;d)`, `defaut(x:pas)`, `exces(x:pas)`
- Constantes : `pi`, `e`

**Exemples :**

```docdg
Le carré de 5 vaut #{5^2}.
La racine de 16 vaut #{racine(16)}.
La valeur de π est environ #{arrondi(pi;4)}.
```

---

## **🧮 Le langage algorithmique**

**Ce chapitre décrit le langage exécuté par le document lui-même** : ce qu'on déclare, ce qu'on range, ce qu'on nomme et ce qu'on répète. Il couvre le programme de NSI du lycée et l'informatique commune des classes préparatoires. Les structures de contrôle sont décrites plus haut, dans [Structures de contrôle](#-structures-de-contrôle) ; ce chapitre part de là.

### **🔢 Les types**

Le type est **toujours écrit**, et se lit en toutes lettres. Il n'est pas décoratif : il est vérifié à chaque instant, et l'erreur porte la ligne fautive.

| Type | Ce qu'il contient |
|---|---|
| `entier` | un élément de ℤ |
| `décimal` | un élément de 𝔻 — développement décimal fini |
| `réel` | un élément de ℝ |
| `complexe` | un couple de deux réels, noté `(a ; b)` |
| `booléen` | vrai ou faux |
| `chaîne de caractères` | du texte ; `chaîne` et `texte` sont synonymes |

L'inclusion ℤ ⊂ 𝔻 ⊂ ℝ est respectée dans les deux sens : `1/3` est un réel légitime mais n'est pas un décimal, et docdg le dit.

### **📦 Les conteneurs**

Quatre conteneurs, une seule grammaire — le nom, le deux-points, le type, la valeur. Les indices commencent à **0**, comme en NSI ; le point-virgule sépare, l'accolade rassemble.

```docdg
soit notes: une liste de décimaux = {12,5 ; 15 ; 9,5}
soit trajets: un dictionnaire de textes et d'entiers = {Marche: 5 ; Bus: 3}
soit A: une matrice 2×2 d'entiers = {{1 ; 2} ; {3 ; 4}}
soit devise: chaîne de caractères = "liberté égalité fraternité"
```

`liste`, `tableau` et `collection` nomment le même type : ce sont les mots du programme, tous acceptés.

**Lire, écrire, découper.** `notes[1]` lit, `notes[1] = 14` écrit, `notes[0 à 2]` découpe une tranche **bornes incluses** — comme « de 1 à 5 » fait cinq tours. Une tranche dont la borne de gauche dépasse celle de droite est simplement **vide**, ce n'est pas une faute : c'est ce qui permet de conclure une fusion de deux listes triées en une ligne. Une matrice se lit par `A[ligne ; colonne]`, un dictionnaire par sa clé.

**Grandir.** Le `+` concatène deux conteneurs ; `ajoute(v ; x)` en ajoute un élément.

### **🧰 Les primitives**

Un **nom** pour ce qui produit une valeur, un **verbe** pour ce qui répond par oui ou non. Chacune rend une **nouvelle** valeur : la série de départ n'est jamais modifiée.

| Primitive | Rend |
|---|---|
| `longueur(v)` | le nombre d'éléments — collection, dictionnaire, matrice ou chaîne |
| `somme(v)`, `min(v)`, `max(v)` | un scalaire |
| `tri(v)`, `inverse(v)` | une nouvelle collection |
| `contient(x ; v)` | vrai ou faux |
| `indice de(x ; v)` | le rang — l'absence est une **faute dite**, non un −1 |
| `insère(v ; i ; x)`, `supprime(v ; i)`, `ajoute(v ; x)` | une nouvelle liste |
| `jonction(v ; séparateur)` | une ligne de texte |
| `découpe(t ; séparateur)` | une liste de chaînes de caractères |

`min(a ; b)` à deux arguments reste la fonction mathématique. Une fonction que vous écrivez **l'emporte sur la primitive du même nom** : l'exercice qui demande d'écrire soi-même `sommet` ou `dépile` reste possible.

**Écrire une suite sur une ligne.** Une boucle qui affiche met chaque tour sur sa ligne ; `jonction` aligne une suite avec le séparateur voulu, sans l'écueil de l'accumulateur qui en laisse un de trop à la fin.

```docdg
#{jonction({2 ; 3 ; 5 ; 7} ; ", ")}
#{découpe("un code" ; " ")}
```

**Trois écritures, au choix de ce qui se lit.** La forme fonctionnelle `insère(notes ; 0 ; 20)`, la forme prépositionnelle `dans notes insère(0 ; 20)`, et pour les questions par oui ou non la forme directe `notes contient(15)`. C'est la phrase qui commande.

**La division euclidienne se dit comme au collège** : `quotient de 17 par 5`, `reste de 17 par 5` — la phrase dit lequel des deux nombres est le dividende. Les formes brèves `quotient(17 ; 5)` et `%` restent admises.

### **🔤 Les chaînes de caractères**

Les guillemets, droits ou français, délimitent sans appartenir à la valeur.

```docdg
soit m: chaîne de caractères = "Bonjour"
```

`m[0]` rend une lettre — qui est une chaîne d'un seul caractère, docdg n'introduit pas de type distinct. `pour c dans m` la parcourt lettre à lettre. S'y ajoutent `majuscule`, `minuscule`, `sans accents`, `élague` (les espaces des deux bouts) et `compacte` (tous les espaces), `code("B")` et `caractère(97)` pour le va-et-vient avec le rang Unicode, et les conversions `texte(42)` et `nombre("1,5")`.

**Les guillemets délimitent, ils n'appartiennent pas à la valeur** — droits ou français. C'est ce qui distingue le texte d'un document, qui s'écrit nu comme en LaTeX, de la chaîne de caractères qui relève de la programmation : elle se cite, y compris dans une liste et en clé de dictionnaire.

```docdg
soit mots: liste de chaînes de caractères = {"chat" ; "chien"}
soit trajets: dictionnaire de chaînes de caractères et d'entiers = {"Marche": 5}
```

`majuscule("été")` rend **ÉTÉ**, non ETE : la typographie française accentue les capitales. Le tri suit la **collation française** — « école » se classe avant « Zoé », alors que le codepoint de « é » vient après celui de « z ».

### **🧩 Les fonctions algorithmiques**

Les chevrons marquent la fonction **mathématique** — objet d'étude qu'on dérive et qu'on trace ; le `soit` nu marque la fonction **algorithmique** — calcul nommé qu'on réutilise. Un même nom ne peut désigner les deux.

```docdg
soit addition(a: entier ; b: entier): entier = a + b
```

Dès que le corps compte plusieurs lignes, la valeur produite se désigne par `retourne`, et rien n'est implicite. Un corps qui ne rend rien est une erreur, pas un silence. `retourne` placé dans une boucle en sort sur-le-champ.

**Un paramètre peut être un conteneur ou une chaîne**, et la valeur rendue aussi. Le passage se fait **par copie** : une fonction reçoit une valeur, elle ne peut rien renvoyer dans la série de l'appelant. C'est ce qui rend saine l'acceptation d'une `liste d'entiers` là où une `liste de réels` est attendue.

La récursivité est admise, bornée à deux cents appels imbriqués — au-delà, docdg s'arrête et le dit plutôt que d'épuiser la pile.

### **🎁 Le p-uplet**

Longueur fixe, types pouvant différer — le programme le distingue explicitement de la liste. Il s'écrit entre parenthèses, comme un point du plan.

```docdg
soit divise(a: entier ; b: entier): (entier ; entier) = (quotient de a par b ; reste de a par b)
soit (q ; r) = divise(17 ; 5)
```

La **déliaison** pose les deux noms d'un coup. Les membres se lisent aussi par leur rang : `c[1]`. L'arité est vérifiée dans les deux sens.

### **🎲 Le hasard, et le cours d'une boucle**

`aléatoire(a ; b)` tire un entier entre les deux bornes, comprises. Deux compilations donnent deux tirages : c'est ce qu'une simulation attend — Monte-Carlo, marche aléatoire, étude de fréquences.

`sortir` arrête une boucle entière ; `continuer` n'arrête que le tour en cours. Les deux valent dans un document comme dans un corps de fonction.

### **🥞 Les piles et les files**

Les structures nommées du programme de terminale, avec leur discipline — c'est tout leur intérêt.

```docdg
soit p: pile d'entiers = {}
p = empile(p ; 1)
p = empile(p ; 2)
#{sommet(p)}    #{dépile(p)}

soit f: file de chaînes de caractères = {"a" ; "b"}
#{tête(f)}      #{défile(f)}
```

`empile`, `dépile`, `sommet` d'un côté ; `enfile`, `défile`, `tête` de l'autre ; `est vide` et `longueur` pour les deux. Chaque opération rend une **nouvelle** structure. Une file n'a pas de sommet, une pile n'a pas de tête, et aucune des deux ne s'indexe ni ne se parcourt : on la vide.

### **🧱 Les objets**

Une classe rassemble des données et ce qu'on sait en faire. Son nom commence par une majuscule ; le type de ses attributs se passe d'article.

```docdg
soit une classe Point {
	abscisse: réel
	ordonnée: réel

	soit norme(): réel = racine(abscisse * abscisse + ordonnée * ordonnée)
}

soit p: Point = Point(3 ; 4)
#{p.abscisse}    #{p.norme()}
p.abscisse = 6
```

Dans le corps d'une méthode, les attributs sont visibles par leur nom : la formule s'écrit comme au tableau. Un attribut se modifie sans `soit`, puisque rien n'est déclaré.

**Les quatre piliers** sont couverts, chacun d'un seul mot :

| Pilier | Écriture |
|---|---|
| **Encapsulation** | `privé solde: réel`, `privé soit taux(): réel = 0,02` |
| **Héritage** | `soit une classe Chien qui hérite de la classe Animal { … }` |
| **Polymorphisme** | la valeur décide de la méthode appelée, non le type écrit |
| **Abstraction** | `soit une classe abstraite Forme { soit aire(): réel }` |

Ce qui est privé reste visible **depuis la classe** ; du dehors, la lecture comme l'écriture sont refusées. Un enfant tient la place de son parent, l'inverse est dit. Une classe abstraite ne s'instancie pas, et une classe concrète ne peut rien laisser sans corps.

**Arbres et graphes** s'écrivent avec ce qui précède, sans type nouveau : un graphe est une liste d'adjacence, un arbre binaire se fait d'objets qui se contiennent — l'arbre vide étant une classe, non une valeur nulle.

### **⚠️ Les fautes sont dites**

Un indice hors bornes, une clé absente, un type qui ne convient pas, une récursion sans arrêt, une boucle qui ne finit pas : chaque cas produit un message français nommant ce qui était attendu, avec la ligne fautive — jamais un silence ni un document faux.

---

## **📊 Les graphiques de fonctions**

### **Déclaration d'une fonction**

Déclarer une fonction, c'est **poser un objet nouveau** — exactement ce que fait `<Soit>` pour un point. On emploie donc la même action, avec l'article indéfini (règle n°10) :

```docdg
<Soit>une fonction f(x) = x^2 - 2
<Soit>une fonction g(t) = sin(t) + cos(t)
```

> **Pourquoi `<Soit>` et non un objet `<fonction>` ?** Une fonction est une **hypothèse de départ** au même titre qu'un point : elle se *pose*, elle ne se *dessine* pas (c'est `<Représente>graphiquement` qui la dessinera ensuite). La formule scolaire canonique est d'ailleurs « **Soit** f la fonction définie par… ». Unifier toute déclaration sous le verbe `<Soit>` (points, valeurs, fonctions) évite d'avoir à retenir un objet spécial `<fonction>` : un seul verbe pour tout ce qu'on pose, un seul verbe de graphage (`<Représente>graphiquement`) pour tout ce qu'on représente dans un repère.

Comme pour un point, plusieurs déclarations se regroupent entre accolades :

```docdg
<Soit>{
    une fonction f(x) = x^2 - 2
    une fonction g(t) = sin(t) + cos(t)
}
```

### **Tracé de la courbe**

```docdg
<Soit>une fonction f(x) = x^2 - 2
<Représente>graphiquement la fonction f
<Représente>graphiquement la fonction f pour x appartient à [-5 ; 5] et y à [-10 ; 10]
<Représente>graphiquement la fonction f, en rouge et de trait 0,5 mm
```

**Bornes d'affichage :** `pour x appartient à [A ; B] et y à [C ; D]`, en prose — même convention que le repère (voir règle n°10). Les variantes `pour x dans [A ; B]` et `sur [A ; B]` (abscisse seule) sont acceptées.

**En prose :** couleur (`en rouge`), épaisseur (`de trait 0,5 mm`), bornes d'affichage (`pour x appartient à [A ; B] et y à [C ; D]`), finesse (`avec 200 échantillons`).

**Plusieurs courbes dans le même repère** (factorisation plurielle, règle n°10) : `<Représente>graphiquement les fonctions f, g et h pour x dans [-2 ; 2] et y dans [-3 ; 3]` — article pluriel (`les`, `des`) ou nombre (`3 fonctions f, g et h`, le compte est vérifié). Chaque courbe reçoit sa couleur et son entrée de légende ; les fonctions doivent avoir été posées par `<Soit>` auparavant.

**En prose également :** finesse (`avec 200 échantillons`), aire sous la courbe (`avec l'aire entre A et B`, ou `area:{a, b}` quand les bornes sont des nombres à donner tels quels), aire entre deux courbes (`avec l'aire jusqu'à g`), et le tracé en escalier d'une suite récurrente : `avec le diagramme en escalier depuis U0 sur N termes` (N vaut 8 si omis).

### **Analyse et algèbre du supérieur**

Chaque construction se pose en une phrase, sur une fonction ou une matrice déjà introduite par `<Soit>` ; le calcul est formel (SymPy), le rendu suit les notations du supérieur.

```docdg
<Soit>une fonction f(x) = sin(x)/x

<Calcule>la limite de f en 0                              % lim f(x) = 1, x -> 0
<Calcule>la limite de f en +infini                        % ... et -infini
<Calcule>la limite de f en 0 à droite                     % ... ou à gauche

<Calcule>le développement limité de f en 0 à l'ordre 4    % avec le o(x^4) français

<Calcule>les racines cinquièmes de l'unité     % S = (1, e^(2ipi/5), ...)
<Calcule>les racines cubiques de 8i            % racines n-ièmes complexes

<Factorise>x^2 - 5x + 6                        % (x - 2)(x - 3)
<Simplifie>(x^2 - 1)/(x - 1)                   % x + 1
<Résous>l'équation x^2 = 2x + 1                % S = (1 - rac(2), 1 + rac(2))

<Soit>la matrice M{
    2 ; 1
    1 ; 1
}
<Calcule>le déterminant de M                              % det(M) = 1 (exact)
<Calcule>l'inverse de M                                  % M^(-1), fractions exactes
```

Le développement limité s'écrit avec le petit o de l'usage français (`+ o(x^n)`), les solutions d'équations sortent en radicaux exacts, le déterminant et l'inverse en fractions. Une matrice non inversible ou une fonction non posée sont dites en français.

### **Poser et résoudre un système**

Le système se pose par `<Soit>`, qui l'affiche et le mémorise, puis se résout par `<Résous>` — en **calcul exact** : coefficients entiers, décimaux ou fractions `p/q`, solutions rendues en fractions irréductibles. Le solveur dit aussi, en français, quand le système n'a aucune solution ou en admet une infinité.

```docdg
<Soit>le système s{
    a + b + c = 2
    4a + 2b + c = 3
    9a + 3b + c = 6
}

<Résous>s              % La solution du système est (1, -2, 3).

<Résous>s, pas à pas    % ... déroulé au pivot de Gauss, étape par étape.
```

> **Les deux moteurs de calcul.** docdg calcule à deux niveaux, sans option à régler : le **moteur interne** — arithmétique exacte et Gauss-Jordan embarqués en Rust pur, rien à installer, `<Résous>` et `#{...}` fonctionnent partout — et **SymPy**, sollicité automatiquement par les constructions de calcul formel (dérivées, zéros exacts, variations automatiques, [sympy.org](https://www.sympy.org)). SymPy nécessite Python 3 installé (`python3 -m pip install sympy`) ; docdg le détecte et l'appelle automatiquement — aucune option à activer, aucune ligne de commande à modifier. docdg trouve la commande Python tout seul (`python3`, `python` puis `py`). Chaque requête passe par un fichier temporaire (jamais par la ligne de commande : pas d'injection), est mise en cache (l'expression répétée est gratuite), et remonte ses erreurs en français — rien n'est avalé.

### **Tableau de variations**

Si Python et SymPy sont installés, le tableau de variations **se calcule tout seul** — dérivée, zéros réels, signes et valeurs exactes. La fonction doit avoir été posée par `<Soit>` auparavant : le tableau retrouve sa définition et l'affiche en tête.

```docdg
<Soit>une fonction q(x) = -x^4 + 2x^2 + 1

<Dresse>le tableau de variations de q
```

C'est la seule forme : on nomme la fonction, docdg fait le reste. `<Étudie>les variations de q` en est le synonyme conjugué.

Les compagnons du calcul formel s'écrivent tous par une **action** — un verbe à l'impératif, jamais un nom seul : `<Calcule>la dérivée de q` (et `<Calcule>la dérivée seconde de q`) affiche la dérivée simplifiée ; `<Détermine>les zéros de q` l'ensemble exact des solutions de q(x) = 0, radicaux compris ; `<Calcule>la primitive de q` une primitive (avec sa constante). Les verbes interchangeables sont `Calcule`, `Détermine`, `Donne`, `Cherche`, `Trouve`, `Établis`, `Évalue`. Ces phrases se **factorisent** au pluriel (règle n°10) : `<Calcule>la dérivée de f, g et h` affiche les trois dérivées, une par ligne — l'argument doit être une pure liste de noms, virgules entre eux, « et » avant le dernier. Le tracé de la même fonction se fait par une phrase en langage naturel :

```docdg
<Soit>une fonction q(x) = x^3 - x
<Représente>graphiquement la fonction q pour x appartient à [-2 ; 2] et y à [-3 ; 3], avec 200 échantillons
```

### **Tableau de signes**

```docdg
<Soit>une fonction f(x) = (x+2)(x-3)
<Dresse>le tableau de signes de f
```

---

## **🔢 Les mathématiques**

> **Le cœur de docdg** : Un mini-langage mathématique puissant.

**Syntaxe :** `$EXPRESSION MATHÉMATIQUE$`

### **Fractions et racines**

```docdg
$1/2 + 1/3$        % Fractions enchaînées
$racine(2)$        % Racine carrée
$racine[3](8)$     % Racine cubique
$|x - 2|$          % Valeur absolue, entre barres
$E(x)$             % Partie entière (ent(x) et floor(x) tolérés)
```

La valeur absolue s'écrit entre barres partout — dans une formule, dans une fonction posée (`<Soit>une fonction k(x) = |x - 2|`) comme dans une équation à résoudre (`<Résous>l'équation |x - 2| = 3`). La partie entière s'écrit `E(x)` — la notation la plus simple — et se rend $E(x)$ (`ent` et `floor` restent compris) ; toutes deux s'évaluent, se tracent et se prêtent aux limites à gauche et à droite (`<Calcule>la limite de pe en 2 à gauche`).

**Le tracé d'une courbe est complet d'office** — même doctrine que la rédaction : dans un repère, `la courbe de la fonction f` embarque ce que dit le tableau de variations. Les tangentes horizontales se tracent aux extremums (avec le point marqué), les asymptotes verticales, horizontales et obliques viennent en tirets, et les fonctions en escalier dessinent leurs marches sans faux traits verticaux, avec le point plein au bord atteint et le cercle ouvert au bord exclu, comme au tableau. Il n'y a rien à demander.

### **Puissances et indices**

```docdg
$x^2$              % x²
$x^(a+b)$          % x^(a+b) - parenthèses importantes !
$u_n$               % uₙ
$u_(n-1)$           % uₙ₋₁
```

### **Ensembles de nombres**

Chaque ensemble s'écrit soit avec l'abréviation à double lettre (usage mathématique courant), soit en toutes lettres — les deux formes sont équivalentes :

```docdg
$NN$ ou $naturels$      % ℕ
$ZZ$ ou $entiers$       % ℤ
$DD$ ou $décimaux$      % 𝔻
$QQ$ ou $rationnels$    % ℚ
$RR$ ou $réels$         % ℝ
$CC$ ou $complexes$     % ℂ
$PP$ ou $probabilité$   % ℙ
$KK$ ou $corps$         % 𝕂
$HH$ ou $quaternions$   % ℍ
```

### **Quantificateurs et logique**

```docdg
$pour tout x dans RR$           % ∀x ∈ ℝ
$il existe x$                   % ∃x
$il n'existe pas de x$          % ∄x
$x appartient à NN$             % x ∈ ℕ
$x n'appartient pas à NN$       % x ∉ ℕ
$(P et Q) ou négation de P$     % (P ∧ Q) ∨ ¬P
```

Dans une zone mathématique, `et` et `ou` sont les connecteurs logiques (∧, ∨) ; `négation de` donne ¬. Le mot `non` sert au **complémentaire** d'un ensemble ou d'un événement : `non(A)` s'affiche $\overline{A}$.

### **Opérations sur les ensembles**

Chaque opération se dit en toutes lettres, en français :

```docdg
$A inclus dans B$               % A ⊂ B
$A inclus dans ou égal à B$     % A ⊆ B
$A union B$                     % A ∪ B
$A inter B$                     % A ∩ B
$A privé de B$                  % A \ B
$vide$                          % ∅
$non(A union B) = non(A) inter non(B)$   % loi de De Morgan
$parties de(A)$                 % 𝒫(A)
$cardinal(A)$                   % card(A)
$a congru à b modulo n$         % a ≡ b [n]
```

Les grands opérateurs indexés se disent `réunion de` et `intersection de` :

```docdg
$réunion de(i=1;n) A_i$         % ⋃ de i=1 à n
$intersection de(i=1;n) A_i$    % ⋂ de i=1 à n
```

### **Probabilités : conditionnement et complémentaire**

```docdg
$PP(A sachant B)$               % P(A | B)
$PP(non(B))$                    % P(B̄)
$répartition de(X; x)$          % F_X(x)
$densité de(X; t)$              % f_X(t)
```

### **Algèbre linéaire**

Les opérateurs suivent les usages du supérieur ; les majuscules sont celles de la tradition (Vect, Tr, Ker...) :

```docdg
$engendré par(u, v)$            % Vect(u, v)
$trace(A)$                      % Tr(A)
$déterminant(A)$                % det(A)
$^t{A}$                         % transposée
$A^{-1}$                        % inverse
$comatrice(A)$                  % Com(A)
$spectre(A)$                    % Sp(A)
$A^*$                           % adjoint
$noyau(A)$                      % Ker(A)
$image(A)$                      % Im(A)
$rang(A)$                       % rg(A)
```

### **Fonctions mathématiques**

```docdg
$sin(x)^2 + cos(x)^2 = 1$
$arcsin(1/2) = pi/6$
$sinus en degrés(30) = 0,5$          % Sinus en degrés
$exp(1) = e$
$ln(e) = 1$
$abs(-5) = 5$
$norme(vecteur(AB))$
```

### **Sommes et produits**

```docdg
$somme(k=1;n) k^2$       % ∑ₖ=₁ⁿ k²
```

**Les opérateurs différentiels, en toutes lettres.** Le gradient, la divergence, le rotationnel et le laplacien s'écrivent comme ils se disent — et sortent dans les notations françaises (`grad`, `div`, `rot`, `Δ`) :

```docdg
$gradient(f)$                    % grad f
$divergence(F)$                  % div F
$rotationnel(F)$                 % rot F
$laplacien f$                    % Δf   (préfixe, sans parenthèses obligatoires)
$partielle f / partielle x$      % ∂f/∂x
$nabla f$                        % ∇f, pour qui préfère la notation nabla
```

Le laplacien suit la convention du symbole préfixe : `laplacien f` se lit comme `nabla f`, les parenthèses éventuelles appartiennent à l'opérande. La dérivée partielle s'obtient par la fraction : le mot `partielle` (ou `partiel`) devant chaque membre — le moteur compose alors la fraction aux ∂ droits. Les notations alternatives que vous citeriez dans un énoncé (« noté ∇f ou grad f ») s'écrivent naturellement : `$nabla f$ ou $gradient(f)$`.

```docdg
$produit(k=1;n) k$      % ∏ₖ=₁ⁿ k
```

### **Limites et intégrales**

```docdg
$lim(x->0) sin(x)/x = 1$          % limₓ→₀
$intégrale(x=a;b) f(x)$         % ∫ₐᵇ f(x) dx
$intégrale(x) f(x)$              % ∫ f(x) dx (primitive)
$intégrale contour(C) f(z)$             % ∮_C
$intégrale valeur principale(x=a;b) f(x)$             % Valeur principale
$intégrale moyenne(x=a;b) f(x)$             % ⨍ₐᵇ (moyenne)
```

**Intégrales multiples :**

```docdg
$intégrale(x=a;b ; y=c;d) f(x;y)$    % ∬ₐᵇᶜᵈ f(x,y) dy dx
$intégrale(x=a;b ; y=c;d ; z=e;g) f$ % ∭ₐᵇᶜᵈᵉᵍ f dz dy dx
$intégrale surface(S)$                     % ∯_S (surface)
$intégrale volume(V)$                      % ∭_V (volume)
```

### **Dérivées**

```docdg
$dy/dx$                          % dy/dx
$(d^2 y)/(dx^2)$                 % d²y/dx²
$partielle f / partielle x$          % ∂f/∂x
$(partielle u)/(partielle t) = (partielle^2 u)/(partielle x^2)$  % Équation chaleur
```

### **Matrices et systèmes**

**Formule compacte, en ligne :**

```docdg
$matrice(1 2 ; 3 4)$      % Matrice avec parenthèses
$matrice crochets(1 2 ; 3 4)$ % Matrice avec crochets
$det(1 2 ; 3 4)$          % Déterminant
```

**Bloc, sur plusieurs lignes — parenthèses ou crochets comme délimiteur :**

```docdg
<Affiche>la matrice(
	1	2	3
	-x	0	x^2
)

<Affiche>la matrice[
	1	0
	0	1
]
```

Parenthèses ou crochets s'écrivent une fois, à l'ouverture, et referment le bloc eux-mêmes — ce que l'on tape est ce que l'on voit au rendu. Le verbe seul, suivi de l'objet en toutes lettres (`<Affiche>la matrice(`, refermé par `)` seul — la même convention qui referme `<Trace>le cercle...`), sépare nettement la consigne de l'objet qu'elle concerne.

La même forme scindée s'applique aux objets **nommés**, gardés pour un calcul ultérieur — la matrice ou le système reçoit alors son nom avant l'ouverture, et les accolades neutres restent le délimiteur (les rangées contiennent déjà des parenthèses ou des crochets, dans une équation ou un coefficient) :

```docdg
<Soit>la matrice M {
	2	1
	1	1
}

<Soit>le système s {
	2x + 3y = 7
	x - y = 1
}
```

équivalent à `<Soit>la matrice M{...}` et `<Soit>le système s{...}`.

Dans ces blocs, la **tabulation** sépare les colonnes et le **saut de ligne** sépare les lignes ; avec parenthèses ou crochets, c'est le **point-virgule** qui sépare les colonnes (voir plus haut). Pour un système, découper la ligne en plusieurs colonnes permet d'aligner sur plusieurs points à la fois (le signe `+` et le signe `=`), pas seulement sur le `=`.

Une balise porte toujours un **verbe d'action** : il n'existe pas de balise `<matrice>` ni `<système>` qui nommerait l'objet sans le poser. C'est `<Soit>` qui déclare, ici comme partout ailleurs.

### **Opérateurs avancés**

```docdg
$grad(f)$        % ∇f (gradient)
$div(F)$         % ∇·F (divergence)
$rot(F)$         % ∇×F (rotationnel)
$laplacien(f)$         % Δf (laplacien)
$dérivée directionnelle(f ; u)$ % ∇ᵤf (dérivée directionnelle)
```

### **Probabilités et statistiques**

```docdg
$C(n ; k)$        % Coefficient binomial ⁽ⁿₖ⁾
$A(n ; k)$        % Arrangement Aₙᵏ
$factorielle(n)$ % Factorielle n!
$n!$             % Factorielle (alternative)
$PP(A)$          % ℙ(A) Probabilité
$PP(A sachant B)$ % ℙ(A∣B) Probabilité conditionnelle
$EE(X)$ ou $espérance(X)$          % 𝔼(X) Espérance
$variance(X)$         % Var(X) Variance
$écart type(X)$   % σ(X) Écart-type
$covariance(X ; Y)$      % Cov(X,Y) Covariance
$normal(mu ; sigma)$     % 𝒩(μ,σ²) Loi normale
$poisson(lambda)$       % 𝒫(λ) Loi de Poisson
$binomiale(n ; p)$        % ℬ(n,p) Loi binomiale
$fonction de répartition(X ; x)$           % F_X(x) Fonction de répartition
$densité(X ; x)$          % f_X(x) Densité
```

---

## **📐 La géométrie**

Toute la géométrie — plane, dans l'espace, avec ou sans repère — se trace avec l'action [**`<Trace>`**](#-les-actions) : figures planes, cercle trigonométrique, solides en 3D, points/droites/segments/médiatrices, y compris posés dans un repère. L'analyse se représente graphiquement avec [**`<Représente>graphiquement`**](#-les-actions) : courbes de fonctions, diagrammes en escalier, statistiques, et le mode analytique dès que son contenu relève de l'analyse. Voir le chapitre **Les actions** pour le détail de chaque cas.

---

## **🧊 La géométrie dans l'espace**

Nouveauté de docdg 2.0, portée par un **moteur de projection cavalière** (fuyante à 0,45/0,35, celle des manuels) : les solides, leurs patrons, le repère de l'espace et la géométrie analytique rédigée.

### Les solides et leurs patrons

Sept solides en perspective cavalière, arêtes cachées en pointillés et cotes en centimètres :

```docdg
<Trace>le solide cube, d'arête 3 cm
<Trace>le solide pavé droit, de longueur 4 cm, de largeur 2 cm et de hauteur 3 cm
<Trace>le solide prisme, de base 3 cm et de longueur 5 cm
<Trace>le solide pyramide, de base 4 cm et de hauteur 3 cm
<Trace>le solide cylindre, de rayon 2 cm et de hauteur 5 cm
<Trace>le solide cône, de rayon 2 cm et de hauteur 4 cm
<Trace>le solide sphère, de rayon 3 cm
```

Les génératrices du cône sont les tangentes exactes à l'ellipse de base depuis le sommet. Chaque solide dépliable a son patron, prêt à découper — `<Trace>le patron du cône de rayon 2 cm et de hauteur 4 cm` — pour le cube, le pavé, la pyramide (apothème coté), le cylindre (rectangle de largeur 2πr) et le cône (secteur d'angle 2πr/g).

### Le repère de l'espace

Les points déclarés se placent dans un repère (O ; x, y, z) gradué, avec le chemin de coordonnées en pointillés rouges depuis l'origine — le dessin canonique de Terminale :

```docdg
<Soit>les points A(1;2;3) et B(3;1;2)
<Trace>dans un repère de l'espace les points A et B, le segment [AB] et le vecteur AB
```

Les segments se citent entre crochets, les vecteurs se dessinent en flèches rouges, et les droites déclarées se tracent en vert, découpées aux bords du repère.

### Les droites, les plans, les positions relatives

Une droite se déclare par un point et un vecteur directeur — en coordonnées ou par référence à des objets déclarés :

```docdg
<Soit>la droite d passant par A(1;0;2) et de vecteur directeur u(1;1;-1)
<Soit>le plan P d'équation 2x + y - z = 3
<Donne>une représentation paramétrique de la droite d
<Donne>le vecteur normal du plan P
<Calcule>la distance du point (1 ; 2 ; 0) au plan P
```

Et les positions relatives se **rédigent pas à pas**, comme une copie modèle — colinéarité des vecteurs, résolution du système, vérification de la troisième équation, conclusion avec le point ou la droite d'intersection :

```docdg
<Soit>la droite d passant par A(1;0;2) et de vecteur directeur u(1;1;-1)
<Soit>la droite d' passant par B(0;1;3) et de vecteur directeur v(2;2;-2)
<Soit>le plan P d'équation 2x + y - z = 3
<Soit>le plan Q d'équation 2x + y - z = 5
<Étudie>la position relative des droites d et d'
<Étudie>la position relative de la droite d et du plan P
<Étudie>la position relative des plans P et Q
```

Chaque cas conclut proprement : droites confondues, strictement parallèles, sécantes en \(I\), ou non coplanaires ; droite incluse, strictement parallèle ou sécante au plan ; plans confondus, strictement parallèles ou sécants selon une droite \(Δ\) donnée en représentation paramétrique.

---

## **📊 Les statistiques**

### **📈 Diagrammes statistiques**

**Syntaxe :** `<Représente>graphiquement une statistique TYPE données:DONNÉES` — le type se met directement, sans préfixe `type:` (comme `<Dresse>une liste à puces` ou les figures de `<Trace>`). `données:` reste en clé:valeur : ce sont de vraies données structurées, pas un réglage de style.


| **Type**             | **Description**      | **Format données**          | **Exemple**                    |
| -------------------- | -------------------- | --------------------------- | ------------------------------ |
| `barres`             | Diagramme en bâtons  | \`clé:valeur                | clé:valeur\`                   |
| `histogramme`        | Histogramme          | `bornes:{...} effectifs:{...}` | `bornes:{0;5;10} effectifs:{3;7}` |
| `camembert`          | Diagramme circulaire | \`clé:valeur                | clé:valeur\`                   |
| `boite a moustaches` | Boîte à moustaches   | `valeur1 ; valeur2 ; ...`     | `données:{12 ; 15 ; 9 ; 21}`            |
| `nuage`              | Nuage de points      | `(x1;y1) (x2;y2) ...`       | `données:{(1;2) (2;3)}`           |


**Option :** `avec ajustement` (trace la droite de régression pour les nuages)

**Exemples :**

```docdg
<Représente>graphiquement une statistique en barres avec les données {1: 4 | 2: 7 | 3: 2}
<Représente>graphiquement une statistique en camembert avec les données {Pommes: 5 | Poires: 3 | Bananes: 2}
<Représente>graphiquement une statistique en histogramme avec les bornes {0 ; 5 ; 10 ; 20} et les effectifs {3 ; 7 ; 2}
```

**Avec variables :**

```docdg
soit notes = {
	Mathématiques: 15,5
	Français: 12
	Histoire: 14
}
<Représente>graphiquement une statistique en barres avec les données notes
<Représente>graphiquement une statistique en camembert avec les données notes
```

### **🌳 Arbres de probabilités**

**Syntaxe :** chaque branche s'écrit `Étiquette probabilité`, ses
sous-branches entre accolades :

```docdg
<Construis>un arbre{
    Étiquette 0,5 {
        Sous-étiquette 0,4
    }
}
```

L'arbre se dessine horizontalement, la racine à gauche, et le produit du
chemin — P(A ∩ B) — s'imprime au bout de chaque feuille.

**Exemples :**

```docdg
<Construis>un arbre{
    A 0,3 {
        B 0,6
        !B 0,4
    }
    !A 0,7 {
        B 0,1
        !B 0,9
    }
}

<Construis>un arbre{
    A 1/3 {
        B 0,6
    }
    !A 2/3
}
```

**Règles :**

- `!` devant une étiquette = événement contraire
- Si une branche unique, le complément est ajouté automatiquement
- Une probabilité s'écrit en nombre — décimal (`0,3`) ou fraction (`1/32`) ; une branche illisible est refusée en nommant la forme

### **▭ Cadre, citation, filet**

Trois objets, **une seule famille d'adjectifs**. Le trait se dit en prose — sa
couleur, son dessin, son épaisseur — et les trois le partagent :

| objet | forme | ce qu'il dit |
| --- | --- | --- |
| `un cadre` | quatre côtés | un bloc détaché : consigne, exercice, encadré |
| `une citation` | un filet à gauche | la parole d'un autre |
| `un filet` | un trait, seul | ce qui sépare, ou souligne |

**Le trait :** `en tirets`, `en pointillés`, `double`, `sans bordure` — le
trait plein est le défaut. Son épaisseur suit le mot : `de 0,6 mm` — **la
virgule sépare les décimales ici comme dans les réglages**, et `0.6 mm` est
refusé en nommant la forme attendue. Sa couleur
se nomme comme partout : `bleu marine`, `vert forêt`.

**La largeur et la place :** `d'une largeur de 90 mm`, puis `au centre`,
`à gauche` ou `à droite` — les mots des alignements, sans en inventer. **Sans
largeur déclarée, l'objet prend toute la colonne** : c'est le comportement
d'un bloc, il n'y a rien à écrire pour l'obtenir.

```docdg
<Affiche>un filet bleu nuit de 0,35 mm, de largeur 110 mm au centre

<Affiche>une citation bleu nuit{
	Le gris d'un texte est ce qui reste quand on cesse de lire.
}

<Affiche>une citation avec un fond bleu clair et une bordure bleu canard{
	Un fond appelle une marge intérieure : la citation la prend d'elle-même.
}

<Affiche>un cadre de largeur 120 mm au centre avec une bordure bleu marine de 0,6 mm, un fond bleu Alice et des coins arrondis de 3 mm{
	Le corps du cadre est de la prose ordinaire : un titre s'y numérote et
	entre au sommaire comme ailleurs.
}
```

L'épaisseur appartient à la proposition qui la précède : elle se place **dans**
le groupe du filet ou de la bordure, avant la virgule qui ouvre la proposition
suivante. La largeur, elle, se dit où l'on veut — `de largeur 90 mm`,
`d'une largeur de 90 mm`, en millimètres ou en centimètres.

**Le fond :** `avec un fond bleu Alice` — le cadre et la citation le portent,
le filet non : un trait n'a pas de surface. **Un fond posé sur une citation lui
donne sa marge intérieure** ; sans fond, la citation garde son seul retrait de
gauche et ne bouge pas. Les `coins arrondis de 2 mm` suivent le fond, là où il
y en a un.

Les intensités se disent : `bleu clair`, `gris foncé`, `vert clair`,
`rouge foncé`, `jaune clair` — et les nuances nommées gardent leur nom,
`bleu canard`, `bleu Alice`, `vert menthe`, `gris perle`.

Le cadre garde ses clés propres — `un titre {…}`, le caractère `sécable`, les
séparateurs `---` — que les deux autres n'ont pas : un filet ne porte rien, une
citation ne se coupe pas en morceaux.

### **📏 Droite graduée**

**Syntaxe résolue (inéquation) :**

```docdg
<Représente>graphiquement la droite graduée sur [-5 ; 5] {
	abs(x - 1/2) >= 5/2
}
```

**Syntaxe déclarée (ensemble donné) :**

```docdg
<Représente>graphiquement la droite graduée sur [-5 ; 5], d'intervalle {[-2 ; 3)} et de points {-4}
```

**Symboles :** `[`/`]` = inclus, `(`/`)` = exclu, `union`, `inf`, `-inf`

---

<a id="lecriture-sur-des-lignes"></a>

## **⚗️ La physique-chimie**

Le corpus couvre la physique et la chimie du CE2 à l'agrégation :
751 énoncés et 215 démonstrations de physique-chimie, 243 grandeurs et 122 cadres
d'hypothèses, répartis en 102 domaines — le catalogue complet, par matière et par
niveau, est dans [Redaction.md](Redaction.md). Les relations y sont vérifiées en homogénéité
au moment de la compilation.

Deux verbes lui sont propres. **`<Équilibre>`** ajuste les coefficients
stœchiométriques d'une équation de réaction — nombres entiers minimaux, ions et
électrons compris :

```docdg
<Équilibre>CH4 + O2 -> CO2 + H2O
<Équilibre>MnO4^- + Fe^2+ + H^+ -> Mn^2+ + Fe^3+ + H2O
```

**`<Propage>`** rédige la propagation des incertitudes sur un modèle : les
dérivées partielles, la formule de composition quadratique et l'application
numérique sortent complètes.

```docdg
<Propage>l'incertitude sur g = 4pi^2 L/T^2 avec u(L) = 0,001, u(T) = 0,01, L = 0,995, T = 2,004
```

Le détail est donné dans les guides par niveau : [Collège](College.md),
[Lycée](Lycee.md), [Licence](Licence.md), [Master et agrégation](MasterAgregation.md).

## **🕰️ La frise chronologique**

L'histoire entre en scène — et c'est une vraie frise, non une ligne du temps : un **bandeau** gradué, les **périodes dedans**, les **événements dehors** en cartouches reliés à leur date. Une ligne par événement : la date, deux-points, le titre, et, s'il éclaire, le détail **entre parenthèses** en fin de ligne.

```docdg
<Construis>la frise chronologique du second XXe siècle {
09/11/1989 : Chute du mur de Berlin
20/07/1969 : Premier pas sur la Lune (Neil Armstrong marche sur la Lune.)
12/07/1998 : Victoire en Coupe du monde (La France remporte sa première étoile.)
}
```

**Le bandeau se referme sur une grande pointe** : le temps ne s'arrête pas au bord de la feuille. La frise occupe **toute la largeur utile** de la page, portrait ou paysage.

**L'échelle du temps est graduée** en années rondes, le pas s'ajustant à l'étendue — de l'année au millénaire. Ses verticales descendent d'un bandeau à l'autre : ce qui tombe dans la même colonne est simultané.

Les cartouches **se rangent d'eux-mêmes dans l'ordre du temps**, quelle que soit la saisie, et **ne se chevauchent jamais** : ils se répartissent **de part et d'autre du bandeau**, en alternance le long du temps, et s'étagent sur autant de rangées qu'il faut de chaque côté, un trait de rappel reliant chacun à sa date. **Rien n'est jamais barré** : les traits se tracent tous avant les cartouches, qui sont opaques, et les verticales de la graduation s'interrompent devant chaque cartouche comme devant chaque bandeau — elles ne subsistent que dans les espaces libres, ce qui suffit à faire lire la colonne sans traverser ce qu'on regarde.

La date se lit en `AAAA`, `MM/AAAA` ou `JJ/MM/AAAA` ; **l'année négative est admise** — l'Antiquité s'écrit `-52 : Alésia` — et `vers` dit l'incertitude des sources sans gêner le placement : la date s'imprime toujours telle qu'elle s'est écrite. Ce qui suit « la frise chronologique » dans la description devient la légende, sous la frise. Un long détail se replie de lui-même sur deux ou trois lignes courtes. La parenthèse s'apparie depuis la fin de la ligne : une parenthèse *dans* la description ne trompe pas la lecture, et une ligne entièrement parenthésée reste un titre.

**Une période est un événement qui dure.** Ses deux dates s'écrivent `de 1914 à 1918`, `1789 - 1799`, `1914 -- 1918` ou `1914 — 1918`, au choix — le tiret simple demande seulement de respirer, pour ne pas se confondre avec le signe d'une année négative —, et elle occupe **l'intérieur du bandeau**, avec son nom, ses bornes et sa description si la place le permet. Les périodes qui se suivent se touchent sans se gêner ; celles qui se chevauchent se partagent la hauteur du bandeau ; celles qui sont trop étroites pour se nommer dedans se nomment dans un cartouche, dehors :

```docdg
<Construis>la frise chronologique du premier XXe siècle {
de 1914 à 1918 : Grande Guerre (Le premier conflit mondial.)
1936 -- 1938 : Front populaire
11/11/1918 : Armistice
06/02/1934 : Crise du 6 février
}
```

**La frise multilinéaire** range ses événements par bandes nommées, qui partagent la même échelle du temps — ce qui met la simultanéité sous les yeux, et permet de confronter deux découpages concurrents :

```docdg
<Construis>la frise chronologique du premier XXe siècle {
politique {
de 1914 à 1918 : Grande Guerre
06/02/1934 : Crise du 6 février
}
économie {
de 1929 à 1939 : Grande Dépression
24/10/1929 : Krach de Wall Street
}
}
```

Une bande porte son nom dans la marge, ses périodes dans son bandeau et ses cartouches de part et d'autre, dans sa couleur. Elle s'ouvre d'un nom suivi d'une accolade — `politique {` — et le deux-points se tolère : `Vie politique: {`. Ce qui est simultané se lit dans une même verticale : c'est ce qui rend une frise multilinéaire argumentative et non plus seulement descriptive.

Trois documents suivent les niveaux du programme. **`histoire2`** (collège) : la frise linéaire simple et la frise par périodes — ordonner des événements, se repérer dans le temps. **`histoire3`** (lycée) : la frise thématique et multilinéaire, et la périodisation des régimes français — articuler cause et effet, problématiser un découpage. **`histoire4`** (supérieur) : les trois temps de Braudel en trois bandes d'échelles différentes, et deux périodisations concurrentes posées l'une sous l'autre — mettre en évidence la simultanéité, questionner la pertinence des découpages.

## **🗂️ Les exemples, par niveau**

Les documents d'exemples, dans le dossier `exemples/`, suivent le niveau scolaire, porté par le suffixe du nom de fichier :

| Suffixe | Niveau | Fichiers |
|---|---|---|
| **1** | la langue et la mise en forme | `basique1` |
| **2** | collège | `algebre2`, `analyse2`, `basique2`, `geometrie2`, `statistiques-probabilites2` |
| **3** | lycée | `algebre3`, `analyse3`, `geometrie3`, `basique3`, `statistiques-probabilites3` |
| **4** | supérieur | `algebre4`, `analyse4`, `geometrie4`, `statistiques-probabilites4` |

S'y ajoutent `physique-chimie3` et `physique-chimie4` (équations chimiques, avancement, conversions, constantes, incertitudes, opérateurs de champs), `factorisation` (une étude complète), et depuis la 2.2 deux documents de publication : **`publication1`** (un article court — page de titre, résumé, mots-clés, notes, tableau de mesures) et **`publication2`** (un mémoire — page de titre, table des matières, chapitres sectionnés, renvois croisés, bibliographie) ; la 2.3 y ajoute **`publication3`** (une démonstration typographique — césure, veuves et orphelines, cadre et tableau sécables). La 2.4 réorganise la série algorithmique en trois niveaux, comme toutes les autres : **`algo2`** (déclarer et initialiser, entrer et sortir, décider, répéter — jusqu'à `sortir` et `continuer`), **`algo3`** (les conteneurs, les chaînes de caractères et les fonctions) et **`algo4`** (récursivité, algorithmes classiques du programme — recherche, dichotomie, tris, fusion — et structures de données avancées : pile, file, p-uplet, simulation). Les nouveautés de la 2.0 vivent dans les fichiers de leurs domaines : les saisies interactives et l'alternative dans `basique2` ; les solides, patrons et volumes dans `geometrie2` ; le repère de l'espace, les droites, les plans et les positions relatives dans `geometrie3` ; les paramétrées, polaires et coniques dans `geometrie4` ; les surfaces, Lagrange, intégrales multiples, le plan complexe et les résidus dans `analyse4` ; les groupes dans `algebre4` ; les lois à densité et le théorème central limite dans `statistiques-probabilites4`. La 2.6 ajoute **`vitrine4`** (la démonstration de force : étude de fonction complète, récurrence vérifiée et rédigée par le moteur, système au pivot, géométrie de l'espace — chaque ligne source y produit une rédaction entière), **`calcul1`** (l'école élémentaire : les quatre opérations, décomposition, fractions de même dénominateur, division euclidienne, conversions), **`geometrie1`** (les figures planes : points placés, droites et segments, triangles, cercles, repère), **`demonstration3`** et **`demonstration4`** (pas une ligne de mathématiques écrite à la main : chaque énoncé, sans accolades, est démontré et rédigé par le moteur — récurrences et propriétés universelles menées par le calcul formel, le reste appelé dans la bibliothèque des classiques, du lycée aux classes préparatoires). La 2.7 ouvre une série d'histoire, réglée elle aussi sur les niveaux : **`histoire2`** (le collège — frise linéaire de la Révolution, grandes périodes de l'histoire : ordonner, se repérer), **`histoire3`** (le lycée — frise thématique et multilinéaire de l'entre-deux-guerres, périodisation des régimes français : articuler cause et effet, problématiser un découpage) et **`histoire4`** (le supérieur — les trois temps de Braudel en trois bandes, deux périodisations concurrentes en regard : la simultanéité, la critique des découpages) ; elle ajoute enfin **`publication4`** (les environnements numérotés : deux chapitres, huit énoncés, six renvois croisés, trois démonstrations dont une récurrence rédigée par le moteur). La 3.1 remet la série de publication d'aplomb, un document par usage réel : **`publication1`** reste l'article court, **`publication2`** devient l'**exposé d'élève en histoire** (deux frises — une linéaire illustrée, une multilinéaire à trois bandes —, un tableau de lecture, un cadre de méthode, une note, une bibliographie), **`publication3`** reste la démonstration typographique, **`publication4`** devient la **mini-thèse** (page de titre à emblème, résumé et mots-clés, quatorze environnements numérotés couvrant les dix genres, neuf renvois croisés, quatre démonstrations dont deux récurrences rédigées par le moteur) et **`publication5`** l'**essai** (les sept divisions : deux tomes, deux livres, trois parties, des chapitres sectionnés jusqu'à la sous-sous-section, un renvoi à une partie et une bibliographie). Chaque fichier se clôt sur sa figure à la demande. La 3.2 ajoute **`couleurs1`** — le nuancier des 148 teintes, une par nom, suivi des trois régimes d'accord et des formes fautives que le moteur refuse en nommant la règle — et **`demonstration2`**, le pendant de `demonstration3` et `demonstration4` : les onze raisonnements **écrits par l'auteur**, du raisonnement direct à la double implication, plus la preuve logée dans l'énoncé. La 3.2 ajoute **`couleurs1`** — le nuancier des 148 teintes, une par nom, suivi des trois régimes d'accord et des formes fautives que le moteur refuse en nommant la règle.

---

## **📄 Les documents complexes**

### **Exemple complet : Fiche d'exercices**

```docdg
document {
  marges: 20;
  taille: 12;
  précision: 2;
}

% ===== Styles réutilisables =====
soit titre = <bleu marine gras 18pt au centre>
soit sous-titre = <bleu marine gras 14pt>
soit exercice = <bleu marine gras sous-section>
soit cadre-ex = <cadre avec une bordure bleu marine, un fond bleu Alice et des coins arrondis de 4mm>
soit important = <gras rouge souligné>
soit indication = <italique bleu>

% ===== En-tête =====
<Affiche>une grille avec les zones:["titre titre logo", "info info logo"] et un écart de 5 mm{
  [titre en mc]{
    <titre>Fiche d'Exercices - Équations et Fonctions
  }
  [logo en mc]{
    <Insère l'image IMAGES/logo.png avec une largeur de 40 mm>
  }
  [info]{
    Nom: _______________ Prénom: _______________

    Classe: _______ Date: ___________
  }
}

% ===== Table des matières =====
<table des matières>

% ===== Exercices =====
<exercice>Exercice 1 : Équations du second degré

<Affiche>un cadre-ex{
  Résoudre :
  <Dresse>une liste numérotée{
    $x^2 - 5x + 6 = 0$
    $2x^2 - 8x = 0$
    $x^2 - 7 = 0$
  }
  <indication>Indice :</indication> Utilisez $x = (-b +- racine(b^2 - 4ac))/(2a)$.
}

<exercice>Exercice 2 : Géométrie

<Affiche>un cadre-ex{
  <Trace>le triangle ABC rectangle en A, de côté AB 3 cm et de côté AC 4 cm, avec les marques
  
  Calculer :
  <Dresse>une liste numérotée{
    La longueur de l'hypoténuse BC = #{racine(3^2 + 4^2)} cm
    Le périmètre = #{3 + 4 + racine(3^2 + 4^2)} cm
    L'aire = #{3*4/2} cm²
  }
}

<exercice>Exercice 3 : Étude de fonction

<Affiche>un cadre-ex{
  <Soit>une fonction f(x) = x^2 - 4x + 3
  
  <Dresse le tableau de variations de f, x:{-infini | 2 | +infini},
      dérivée:{- | +}, variations:{+infini \ -1 / +infini}>
  <Représente>graphiquement la fonction f pour x appartient à [-1 ; 5] et y à [-2 ; 4], en bleu, avec 200 échantillons  % Courbe
}

<exercice>Exercice 4 : Statistiques

<Affiche>un cadre-ex{
  Notes : 12 (3x), 14 (12x), 16 (5x)
  
  <Représente>graphiquement une statistique barres données:{12: 3 | 14: 12 | 16: 5}
  
  Moyenne = #{ (12*3 + 14*12 + 16*5) / (3+12+5) }
}

```

---

## **🧭 Comprendre les erreurs et les cas particuliers**

Ce chapitre rassemble les points de détail qui, une fois compris, évitent la plupart des blocages. Il répond aux questions que se pose tout utilisateur dès son premier document un peu long.

### **🚦 Les messages d'erreur**

Quand docdg rencontre une faute, il l'annonce **en français**, en désignant la balise et la ligne fautives. Les erreurs les plus fréquentes :

| **Ce que vous écrivez**              | **Message docdg**                                              |
| ------------------------------------ | ----------------------------------------------------------------- |
| `<Place>le point A` sans avoir défini A | *Le point A n'a pas été défini. Déclarez-le d'abord avec `<Soit>un point A(...)`.* |
| `<Représente>graphiquement la fonction f` sans `<Soit>une fonction f` | *La fonction f n'a pas été déclarée.*                     |
| `Soit titre = ...` (majuscule pour une assignation) | *« Soit » avec majuscule est une action-phrase. Pour une assignation, écrivez `soit` en minuscule.* |
| `<Affiche>un cadre{...` (accolade non refermée) | *Accolade ouverte à la ligne N jamais refermée.*                  |
| `<Trace>un carré ABCD, de côté 5` (unité manquante) | *Longueur sans unité : écrivez « de côté 5 cm » ou « de côté 50 mm ». L'unité est obligatoire hors d'un repère.* |
| `données:{12, 15}` (virgule dans une liste numérique) | *Liste numérique : utilisez le point-virgule (`12 ; 15`).*  |

> **Pourquoi c'est important.** Des messages en français, qui nomment la cause et proposent la correction, sont ce qui distingue un outil pédagogique d'un compilateur. Tout message d'erreur qui ne serait pas en français est un cas non intercepté : signalez-le, il a vocation à être traduit.

### **🔭 La portée des `soit`**

Une assignation `soit x = ...` est **visible depuis sa déclaration jusqu'à la fin du document**, y compris à l'intérieur des blocs (`<Affiche>un cadre`, boucles `pour`, conditions `si`) ouverts *après* elle. À l'inverse, une variable déclarée **à l'intérieur** d'un bloc ne vit que dans ce bloc :

```docdg
soit tva = 0,2          % visible partout ensuite

<Affiche>un cadre{
  soit prix = 100       % visible seulement dans ce cadre
  Total : #{prix * (1 + tva)} €
}

% Ici, tva existe encore ; prix, non.
```

La variable d'une boucle (`pour n de 1 à 5`) ne vit que le temps de la boucle. Cette règle est celle qu'on attend naturellement : ce qu'on pose « en tête » sert partout, ce qu'on pose « dans un tiroir » reste dans le tiroir.

### **🔢 Calculs `#{...}` et virgule décimale**

Un calcul `#{...}` rend son résultat avec une **virgule décimale** (règle d'affichage française), arrondi selon l'option `précision` de la classe :

```docdg
#{5/2}        % → 2,5
#{1/3}        % → 0,333  (avec précision=3)
#{3*4/2}      % → 6      (résultat entier : pas de virgule)
```

**Attention à la collision avec les séparateurs.** Le résultat `2,5` contient une virgule *sans espace après* : le lexer la reconnaît comme décimale, jamais comme énumération (règle n°8). Vous pouvez donc écrire sans crainte `#{5/2}` dans une phrase. En revanche, si vous insérez un calcul **dans une liste numérique** (séparée par `;`), le résultat garde sa virgule décimale et le point-virgule reste le séparateur de liste — aucune ambiguïté :

```docdg
données:{#{5/2} ; #{7/2} ; 4}   % → 2,5 ; 3,5 ; 4
```

---

## **📚 Référence complète**

### **🎨 Référence des couleurs (148 disponibles)**

Une couleur, un nom. Deux noms français pour la même teinte seraient un
doublet : la table n'en porte aucun, et un test le vérifie à chaque
compilation. Cette liste est **engendrée depuis la table du moteur** — elle
ne peut pas en diverger.

#### L'accord des couleurs

docdg suit l'orthographe, sans tolérance : un document scolaire ne peut pas
enseigner une faute. Trois régimes, et trois seulement.

| Régime | Accord | Exemples |
|---|---|---|
| **L'adjectif simple** — ils sont 8 | genre **et** nombre | `bleu`, `bleue`, `bleus`, `bleues` |
| **L'adjectif en -e** — ils sont 6 | nombre seulement | `rouge`, `rouges` |
| **Tout le reste** — 134 | **invariable** | `bleu marine`, `orange`, `marron` |

Les 8 adjectifs simples : `noir`, `blanc`, `gris`, `cramoisi`, `bleu`, `vert`, `violet`, `brun`.

Les 6 adjectifs déjà terminés par -e : `beige`, `rouge`, `jaune`, `pourpre`, `mauve`, `rose`.

Sont **invariables**, et c'est la règle qui surprend le plus :

- **les couleurs composées** — *une bordure **bleu marine***, jamais « bleue
  marine » ; *des filets **vert foncé***, jamais « verts foncés ». L'adjectif
  de couleur composé ne s'accorde pas, quel que soit le nom qu'il qualifie ;
- **les noms de chose employés comme couleurs** — `orange`, `marron`,
  `turquoise`, `ivoire`, `or`, `argent`, `lavande`, `prune`, `chardon`,
  `biscuit`, `neige`, `lin`. *Des cadres **marron***, jamais « marrons ».
  Font exception, et s'accordent parce que l'usage en a fait de vrais
  adjectifs : `rose`, `mauve`, `pourpre`, `beige`.

Une forme fautive n'est pas rattrapée en silence : elle est **dite**, avec la
règle qu'elle enfreint.

```
<bleue marine>Titre     ⚠ « bleue marine » : un adjectif de couleur composé
                          est invariable — écrivez « bleu marine »
<vert sapin>Titre       ⚠ « vert sapin » : teinte inconnue
```

Auparavant, la première s'imprimait en bleu marine sans rien dire, et la
seconde en vert : le document mentait.

**Noirs, gris et blancs** (30)
```
noir, blanc, gris, gris clair, gris foncé, gris perle, gris anthracite, gris souris,
gris souris clair, gris souris foncé, gris ardoise, gris ardoise clair, gris tourterelle, argent,
blanc fumé, blanc antique, blanc fantôme, ivoire, neige, lin, coquillage, vieille dentelle,
blanc navajo, biscuit, blé, mocassin, soie de maïs, fouet de papaye, pêche duveteuse, beige
```

**Rouges** (14)
```
rouge, rouge clair, rouge foncé, rouge sang, rouge brique, rouge tomate, rouge indien,
rouge orangé, cramoisi, corail, corail clair, saumon, saumon foncé, rose brumeux
```

**Bleus** (36)
```
bleu, bleu clair, bleu foncé, bleu marine, bleu nuit, bleu ciel, bleu ciel profond, bleu roi,
bleu acier, bleu acier clair, bleu dodger, bleu canard, bleu Alice, bleu bleuet, bleu poudre,
bleu pâle, bleu ardoise, bleu ardoise foncé, bleu ardoise moyen, bleu moyen, bleu outremer, indigo,
lavande, lavande rosée, azur, cyan, cyan clair, cyan foncé, turquoise, turquoise clair,
turquoise foncé, turquoise moyen, aigue-marine, aigue-marine moyenne, sarcelle, cadet
```

**Verts** (24)
```
vert, vert clair, vert foncé, vert forêt, vert menthe, vert olive, vert olive terne,
vert olive clair, vert printemps, vert printemps moyen, vert de mer, vert de mer clair,
vert de mer foncé, vert de mer moyen, vert pâle, vert prairie, vert citron, vert lime,
vert chartreuse, vert jaune, vert jaune pâle, vert bouteille, vert d'eau, vert menthe glacée
```

**Jaunes et oranges** (15)
```
jaune, jaune clair, jaune foncé, jaune paille, jaune paille foncé, jaune citron, jaune vif, or,
verge d'or, verge d'or pâle, verge d'or claire, orange, orange foncé, orange sable,
orange mandarine
```

**Violets et magentas** (14)
```
violet, violet clair, violet foncé, violet moyen, violet bleu, pourpre, pourpre foncé, magenta,
magenta moyen, orchidée, orchidée foncée, chardon, prune, mauve
```

**Roses** (6)
```
rose, rose clair, rose vif, rose profond, rose ancien, rose fuchsia
```

**Marrons et terres** (9)
```
marron, brun, brun clair, brun foncé, brun chocolat, terre de Sienne, terre cuite, tan, bronze
```

### **Options de classe**

Toutes s'écrivent dans le bloc `document { }` en tête du fichier — voir [Le bloc document](#-le-bloc-document) pour les explications et les exemples.

| **Option**   | **Type**           | **Défaut**  | **Description**                |
| ------------ | ------------------ | ----------- | ------------------------------ |
| `orientation`| `portrait`/`paysage` | `portrait` | Orientation de la page       |
| `marges`     | nombre ou 4 valeurs | `20`       | Marges externes (mm)           |
| `espacements`| nombre ou 4 valeurs | `2`        | Espacements internes (mm)      |
| `police`     | texte              | *(système)* | Police du texte                |
| `taille`     | nombre             | `11`        | Taille de base (pt)            |
| `interligne` | nombre             | `1,3`       | Coefficient d'interligne       |
| `tabulation` | nombre             | `8`         | Largeur tabulation (mm) — le carreau de la Seyès |
| `hauteur`    | nombre             | `8`         | Hauteur saut de ligne (mm) — l'interligne de la Seyès |
| `décalage`   | nombre             | `100`       | Décalage exposants/indices (%) |
| `précision`  | nombre             | `-1`        | Décimales pour arrondi         |

L'ancien bloc `page { }` n'existe plus : il est refusé en nommant `document { }`.

---

## **💡 Bonnes pratiques**

1. **Structurez** : Définissez vos styles en tête avec `soit`
2. **Factorisez** : Utilisez `soit` pour éviter la répétition
3. **Nommez clairement** : Utilisez des noms explicites
4. **Documentez** : Commentez les parties complexes
5. **Testez** : Vérifiez les calculs `#{...}`
6. **Organisez** : regroupez vos images dans un dossier de votre choix (`IMAGES`, `FIGURES`...), avec des noms clairs, et indiquez ce dossier à chaque insertion — docdg n'en cherche jamais un par défaut

## **🧮 Le calcul scientifique étendu**

Ce chapitre détaille tout ce que débloque le second moteur de calcul,
SymPy — le premier moteur, le moteur interne (sans rien à installer),
est présenté dans
[Poser et résoudre un système](#poser-et-résoudre-un-système) ; il
couvre l'arithmétique exacte, les systèmes linéaires et l'évaluation
numérique `#{...}`.

Dès que Python 3 et SymPy sont installés, docdg confie automatiquement les
calculs symboliques et numériques avancés à SymPy — aucune option à activer, les phrases de calcul formel appellent le
moteur externe d'elles-mêmes. Chaque résultat s'affiche en notation
française, virgule décimale comprise.

### Le calcul formel (SymPy)

| Phrase | Effet |
|---|---|
| `<Factorise>EXPR` | Factorisation |
| `<Développe>EXPR` ou `<Développe>et réduis EXPR` | Développement |
| `<Simplifie>EXPR` | Simplification |
| `<Décompose>en éléments simples EXPR` | Éléments simples |
| `<Calcule>la forme canonique de EXPR` | Trinôme canonique |
| `<Résous>l'équation EXPR = EXPR` | Résolution dans ℝ |
| `<Résous>dans CC l'équation ...` | Domaine : `RR`, `CC`, `ZZ`, `NN`, `QQ` ou `les complexes`, `les entiers`... |
| `<Calcule>la somme de EXPR pour k de a à b` | Forme close de la somme |
| `<Calcule>le produit de EXPR pour k de a à b` | Forme close du produit |
| `<Calcule>la dérivée de f` / `<Calcule>la dérivée seconde de f` | Dérivées (fonction posée par `<Soit>`) |
| `<Calcule>la primitive de f` | Primitive |
| `<Détermine>les zéros de f` | Ensemble des zéros |
| `<Calcule>la limite de f en a` (+ `à droite` / `à gauche`) | Limite |
| `<Calcule>le développement limité de f en a à l'ordre n` | DL avec reste de Landau |
| `<Calcule>les racines n-ièmes de z` | Racines complexes, ordinal en toutes lettres (`carrées`... `douzièmes`), `l'unité` admis pour 1 |
| `<Calcule>l'intégrale numérique de f entre a et b` | Valeur approchée |

### Les équations différentielles

La notation prime se lit telle quelle ; l'inconnue et sa variable se
déclarent au besoin :

```
<Résous>l'équation différentielle y'' + 4y = 0
<Résous>l'équation différentielle N' = -0,12N, d'inconnue N(t)
```

Les équations aux dérivées partielles du premier ordre linéaire
s'écrivent en notation indicée (`u_x`, `u_y`, `u_xx`...) :

```
<Résous>l'équation aux dérivées partielles u_x + u_y = u
```

Au-delà du premier ordre, SymPy ne sait pas résoudre symboliquement et
docdg le dit en français.

### L'algèbre linéaire calculée

Une matrice posée par `<Soit>la matrice M{...}` accepte, outre
`<Calcule>le déterminant de M` et `<Calcule>l'inverse de M` :

```
<Calcule>les valeurs propres de M
<Diagonalise>M
```

Les valeurs propres s'affichent avec leurs multiplicités ; la
diagonalisation donne P et D avec M = P D P⁻¹, ou explique pourquoi la
matrice n'est pas diagonalisable.

### Le calcul numérique (SymPy)

| Phrase | Effet |
|---|---|
| `<Résous>numériquement l'équation EXPR = EXPR sur [a ; b]` | Racine numérique, cherchée à partir du milieu de l'intervalle — l'intervalle guide le point de départ |
| `<Calcule>la probabilité que X <= a pour la loi normale(m ; s)` | Probabilité ; opérateurs `<=`, `>=`, `=`, `<`, `>` |
| `<Calcule>le quantile d'ordre p de la loi ...` | Quantile |
| `<Calcule>l'espérance de la loi ...` / `<Calcule>l'écart type de la loi ...` | Moments |
| `<Ajuste f(x) = a*exp(b*x) aux données {(0;1) (1;2,7) ...}>` | Moindres carrés non linéaires ; paramètres libres a, b, c, d — le tout dans la balise, les données comprises |

Les lois reconnues : `normale(m ; s)`, `binomiale(n ; p)`,
`poisson(l)`, `uniforme(a ; b)`, `exponentielle(l)` (paramètre λ, à la
française), `student(k)`, `khi-deux(k)`.

<a id="les-tournures-et-le-placement"></a>

## **🖋️ Les tournures et le placement**

### Les verbes d'objets

Chaque phrase peut commencer par un verbe : `<Affiche>...`,
`<Dresse>...`, `<Construis>...` et `<Insère>...` s'acceptent devant
tout objet, article compris.

```
<Dresse>un tableau bordures entête{ ... }
<Affiche>une liste à puces{ ... }
<Insère l'image IMAGES/photo.png avec une largeur de 30 mm>
<Dresse>le tableau de signes de f
```

### Les verbes non encore illustrés

Sept verbes du moteur ne paraissaient nulle part dans ce manuel. Ils sont
pourtant éprouvés par les tests et employés dans les exemples livrés :

| verbe | ce qu'il fait | forme |
| --- | --- | --- |
| `<Convertis>` | change d'unité, en respectant la dimension | `<Convertis>90 km/h en m/s` |
| `<Effectue>` | pose une opération et la déroule | `<Effectue>la division euclidienne de 152 par 6` |
| `<Complète>` | remplit ce qui manque, marqué `?` | `<Complète>le tableau de proportionnalité {…}` |
| `<Exprime>` | met en formule un énoncé en prose | `<Exprime>le programme de calcul en fonction de x {…}` |
| `<Orthonormalise>` | applique le procédé de Gram-Schmidt | `<Orthonormalise>la famille u et v` |
| `<Trigonalise>` | réduit une matrice à sa forme triangulaire | `<Trigonalise>D` |
| `<Compose>` | compose une page entière | `<Compose>la page de titre {…}` |

### Les tournures du supérieur

`<On pose>`, `<On considère>` et `<On note>` sont synonymes de `<Soit>`
et s'affichent avec leur verbe. `<Étudie>les variations de f` vaut le
tableau de variations.

### La factorisation des points

Les points se regroupent sous un article pluriel :
`<Soit>les points A(2;3) et B(-1;4)` — la forme « un point... et un
point... » est refusée avec un rappel de la règle.

### Les bornes naturelles

```
<Représente>graphiquement la fonction f sur [-2 ; 5], en ordonnée [0 ; 3], en bleu
<Représente>graphiquement la droite graduée sur [-3 ; 4], d'intervalle {[-2, 3)} et de points {1}
```

### Les réglages en toutes lettres

`avec 200 échantillons`, `avec l'aire entre 0 et 1`,
`avec l'aire jusqu'à g`, `de 4 colonnes`, `de 2 lignes`,
`de bornes {0 ; 5 ; 10}`, `d'effectifs {3 ; 7}`.

### Le placement en langage naturel

Dans les cellules et les zones, les codes deux lettres demeurent
(`mc`, `hg`...) et se doublent des mots : `en haut`, `au milieu`,
`en bas` pour la verticale ; `à gauche`, `au centre`, `à droite` pour
l'horizontale — combinables (`en haut, à gauche`). Hors cellule, un
paragraphe s'aligne par `<à gauche>`, `<au centre>`, `<à droite>`.

### Les zones de grille

Le plan `zones:["titre titre logo", "corps corps logo"]` nomme les
zones ; chaque zone s'écrit `[nom, propriétés]{contenu}` ou
`[nom : propriétés]{contenu}`, les propriétés en toutes lettres :

```
[titre : en haut, à gauche, une bordure bleu marine, un fond bleu Alice]{ ... }
[corps en mc]{ ... }
```

Au niveau de la grille, `bordures`, `une bordure <couleur>`,
`fond <couleur>` et `texte <couleur>` posent les défauts que chaque
zone peut surcharger. Une rangée de cadres côte à côte s'obtient par
`<Affiche>une rangée, écart de 5 mm{ ... }`.

### Les données nommées

Une table de données se déclare et se réutilise par son nom, ses
entrées en `clé: valeur` comme les données en ligne :

```
soit sondage = {
	Jeudi: 8
	Lundi: 5
	Mardi: 3
}

<Représente>graphiquement une statistique en barres horizontal données:sondage
```

---

Les retours d'usage (bugs, tournures manquantes, besoins non couverts) sont les bienvenus sur le dépôt du projet.
