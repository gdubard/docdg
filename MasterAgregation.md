# docdg — Le master et l'agrégation

Le master 1 et 2, l'agrégation externe et interne : la rédaction longue, la publication et les structures algébriques.

> Ce document rassemble ce que docdg apporte à ce niveau.
> Les fonctionnalités communes à tous les niveaux — la syntaxe, les objets,
> les styles, le langage algorithmique, les graphiques — sont décrites dans le
> [README](README.md), qui reste le manuel de référence.

---

## **⚙️ Les groupes**

```docdg
<Dresser>la table de Z/5Z pour l'addition
<Dresser>la table de Z/5Z pour la multiplication
<Déterminer>les générateurs de Z/12Z
<Décomposer>la permutation (2 5 4 1 3) en cycles
```

Les tables de \\(\\mathbb{Z}/n\\mathbb{Z}\\) se dressent pour l'addition et la multiplication ; les générateurs sortent avec l'indicatrice d'Euler \\(\\varphi(n)\\) ; une permutation, donnée par la liste des images, se décompose en cycles à supports disjoints, avec ses points fixes, sa signature \\((-1)^{n-c}\\) (paire ou impaire) et son ordre (ppcm des longueurs).

---

## **📕 Rédiger un article, une thèse**

Nouveauté de docdg 2.2 : ce qu'exige un document long — une structure au-dessus de la section, une page de titre, des renvois, une bibliographie.

### Les chapitres

Le mot de style `chapitre` ouvre un niveau au-dessus de `section` :

```docdg
soit h0: un style = <bleu nuit gras chapitre num>
soit h1: un style = <bleu nuit gras section num>

<h0>Le pendule comme instrument

<h1>Le modèle et ses limites
```

Les chapitres se numérotent seuls, remettent les compteurs de sections à zéro et **les préfixent** : les sections deviennent `1.1`, `1.2`, puis `2.1` au chapitre suivant — la convention des mémoires français. Un document sans chapitre garde exactement sa numérotation d'avant.

### La page de titre

La page de titre **se compose dans le corps du document**, avec les styles nommés et l'insertion d'image — comme tout le reste. Le bloc `document { }` ne porte que le physique : il n'a jamais à connaître le titre de l'ouvrage ni le nom de son auteur.

```docdg
soit ouvrage: un style = <gras 22 pt au centre>
soit mention: un style = <14 pt au centre>
soit maison: un style = <italique au centre>

<ouvrage>{De la mesure en physique}

<mention>{Gérard Dubard}

<maison>{Université de Lyon}

<mention>{7 août 2026}

<page suivante>

<table des matières>{Table des matières}
```

Chaque ligne se règle librement — corps, graisse, couleur, alignement —, et `<page suivante>` clôt la page. L'ancienne balise `<page de titre>` n'existe plus, et les clés d'identité `titre:`, `auteur:`, `institution:`, `logo:` et `date:` sont refusées dans le bloc `document { }`, en nommant cette composition-ci. L'exemple `publication1.txt` en donne la forme complète, `publication4.txt` la variante à emblème.

### Les renvois croisés

Une étiquette se pose dans un titre ou un paragraphe ; un renvoi s'y remplace par le numéro, cliquable :

```docdg
<h1>Le modèle et ses limites <étiquette>{modele}

...

	Le raisonnement prolonge celui de la section <renvoi>{modele}.
```

L'ordre est libre : **un renvoi peut précéder son étiquette**. Un renvoi sans étiquette s'affiche `??` en rouge, à la façon de LaTeX — l'erreur se voit sans casser la composition.

### Le corpus

Nouveauté de docdg 2.8, étendu à la physique et à la chimie en 2.9, porté au master en 3.0, à l'école élémentaire en 3.4 puis aux sciences de l'ingénieur en 3.9 : **1606 énoncés et 602 démonstrations**, du CP à l'agrégation, embarqués dans le binaire. `<Énoncer>` **sans accolades** y puise ; avec accolades, il garde son comportement de la 2.7 — l'auteur écrit son énoncé lui-même.

```docdg
document { niveau: quatrième; }

<Énoncer>le théorème de Pythagore
<Dresser>les prérequis du théorème de Pythagore
<Démontrer>le théorème de Pythagore
```

La balise ne porte que **le nom du résultat, dit en français** : le texte, le genre et le numéro viennent du corpus. « le théorème de Pythagore », « la réciproque du théorème de Pythagore » et « la contraposée du théorème de Pythagore » désignent trois résultats distincts, et le corpus sait lequel est un théorème, lequel un corollaire.

**Le niveau ne restreint jamais la consultation.** `document { niveau: … }` prend les dix-sept niveaux en toutes lettres — CP, CE1, CE2, CM1, CM2, sixième, cinquième, quatrième, troisième, seconde, première, terminale, licence 1 à 3, master 1 et 2 — et fixe seulement la rédaction servie par défaut. Si aucune démonstration n'existe à ce niveau, celle du rang le plus proche est servie, et **le repli est dit au lecteur** : un document de sixième qui demande Pythagore reçoit la rédaction de quatrième, avec sa mention. À distance égale, le rang inférieur l'emporte — une démonstration plus élémentaire reste lisible, une plus avancée peut mobiliser des outils inconnus.

La surcharge se dit en prose, pour l'encadré « pour aller plus loin » :

```docdg
<Démontrer>le lien entre signe de la dérivée et variations, au niveau licence 1
```

**Un résultat admis n'est pas une lacune.** Dans un document de troisième, `<Démontrer>le théorème de Thalès` répond « ce résultat est admis en troisième » : le corpus distingue ce que le programme admet de ce qu'il n'a pas encore rédigé.

**Les prérequis se déduisent du graphe.** `<Dresser>les prérequis de …` lit les dépendances du corpus : la liste ne se rédige pas à la main et ne se désynchronise pas quand la démonstration change.

**Un résultat déjà vu se rappelle sans se renuméroter.** `<Rappeler>le théorème de Pythagore` remet l'énoncé sous les yeux, marqué « Rappel » avec son niveau d'origine : un cours rappelle constamment ce qui a été vu plus tôt ou à un niveau antérieur, et le renuméroter serait faux, le recopier à la main se désynchroniserait.

La clé `voie:` n'existe plus : le niveau suffit au corpus, et le moteur la refuse en le disant. Un ancrage porte son statut — admis ou démontré — au niveau où il est posé, et c'est le niveau déclaré qui décide seul de la rédaction servie.

> **La barre doublée.** `$Z//nZ$`, `$G//H$`, `$L//K$` composent un quotient, non une fraction sur deux étages — l'extension de la convention qui fait déjà `<<` pour `<` et `$$` pour `$` (règle n°7). La barre simple garde son sens de fraction.

### Les environnements numérotés

Nouveauté de docdg 2.7 : le théorème, la proposition, la propriété, le lemme, le corollaire, l'axiome, la conjecture, la définition, l'exemple et la remarque **s'énoncent** — et se numérotent tout seuls :

```docdg
soit théorème: un style = <bleu nuit petites capitales>

<Énoncer>le théorème de Pythagore <étiquette>{pythagore} {
Dans un triangle rectangle, le carré de l'hypoténuse est égal à la somme des carrés des deux autres côtés.
}

	La démonstration s'appuie sur le théorème <renvoi>{pythagore}.
```

Le titre se compose en vedette — **Théorème 3.2 (Pythagore).** — et le corps le suit, en italique pour les énoncés, en romain pour les définitions, les exemples et les remarques : la convention des livres. Ce qui suit le nom du genre devient le nom propre de l'énoncé, débarrassé de sa liaison : `le théorème des valeurs intermédiaires` s'imprime « Théorème 3.4 (valeurs intermédiaires) ».

**Chaque genre tient son compteur.** Le théorème 2 succède au théorème 1 sans regarder les définitions ; le chapitre remet tous les compteurs à zéro et préfixe les numéros, comme il le fait des sections. L'étiquette et le renvoi sont ceux du document long — `voir le théorème <renvoi>{pythagore}` se remplace par le numéro, cliquable — et le titre s'habille comme un titre de section : `soit théorème: un style = <bleu nuit petites capitales>`.

**La preuve loge dans l'énoncé.** Un sous-bloc `démonstration { … }` se compose en vedette — *Démonstration.* — et se referme d'un tombeau, en romain quand l'énoncé est en italique. Avec un raisonnement, c'est la machinerie de `<Montrer>` qui rédige :

```docdg
<Énoncer>la propriété {
La somme des $n$ premiers entiers impairs vaut $n^2$.

démonstration par récurrence que pour tout entier $n$ non nul, la somme des $n$ premiers entiers impairs vaut $n^2$ {
initialisation{
Au rang 1, la somme vaut $1 = 1^2$.
}
hérédité{
Si la somme des $n$ premiers entiers impairs vaut $n^2$, la suivante lui ajoute $2n + 1$ et vaut $n^2 + 2n + 1 = (n+1)^2$.
}
}
}
```

Les dix raisonnements de `<Montrer>` s'écrivent tels quels — `démonstration par l'absurde`, `par disjonction de cas`, `par contraposée`… — avec leurs étapes nommées et leurs conclusions rédigées. Quand l'énoncé tient en une phrase, la propriété à démontrer peut même se taire : `démonstration par récurrence { … }` la lit dans l'énoncé lui-même.

**L'énoncé est une citation.** Ce qui s'y déclare — un `<Soit>`, une donnée — y demeure et n'existe plus après lui : le théorème cite ses objets, il ne les lègue pas au document.

Le document **`publication4`** montre l'ensemble en situation : deux chapitres, huit énoncés, six renvois croisés, trois démonstrations dont une récurrence rédigée par le moteur.

### La physique et la chimie au master

Nouveauté de docdg 3.0 : le corpus expérimental, qui s'arrêtait à la licence 3, monte jusqu'à l'agrégation. **163 énoncés et 96 démonstrations** portent le corpus de physique-chimie de dix-sept à vingt-quatre domaines — la 3.4 a rédigé les trente-et-une démonstrations de chimie qui manquaient encore au master et à l'agrégation.

En physique : la mécanique quantique du formalisme de Dirac aux inégalités de Bell, la physique statistique des ensembles quantiques au groupe de renormalisation, la matière condensée du théorème de Bloch à l'effet Josephson, l'électrodynamique covariante et le rayonnement, la physique nucléaire et des particules, la relativité générale et la cosmologie.

En chimie : la chimie quantique de Born-Oppenheimer à la fonctionnelle de la densité, la spectroscopie moléculaire, la catalyse et les organométalliques, la cinétique du master et la photochimie, la chimie macromoléculaire.

```docdg
document { niveau: master 1; }

<Énoncer>le théorème de Bloch
<Démontrer>le théorème de Bloch
<Dresser>les prérequis du modèle de Debye de la capacité thermique
```

**La base est unique aux trois matières.** Un énoncé de chimie quantique dépend de la perturbation stationnaire du corpus de physique, qui dépend elle-même du théorème spectral du corpus de mathématiques. Rien n'est réécrit : les forces de dispersion se démontrent par le second ordre de perturbation, la chaîne gaussienne par la marche aléatoire de l'équation de Langevin, et le graphe le dit.

**Au master, rien ne reste en suspens.** Les résultats que la licence énonce sans preuve y portent un ancrage « démontré » propre, sans qu'aucun énoncé soit dupliqué : c'est le même énoncé, lu à deux niveaux.

### La bibliographie

```docdg
	Le protocole suit le GUM <cite>{gum2008} ; les manuels classiques <cite>{taylor, gum2008} en donnent la démonstration.

<Dresser>une bibliographie {
	[gum2008] JCGM 100:2008, Évaluation des données de mesure, BIPM, 2008.
	[taylor] J. R. Taylor, Incertitudes et analyse des erreurs, Dunod, 2000.
}
```

Une entrée par ligne, numérotée dans l'ordre, ancrée par sa clé. `<cite>{clé}` s'y remplace par `[n]` lié ; plusieurs clés se groupent d'un seul appel. La citation précède naturellement la bibliographie, qui ferme le document.

### Les polices locales

Une suite de mots **en majuscules** dans une balise de style désigne une police :

```docdg
<au centre italique 14 pt>Je suis<TIMES NEW ROMAN gras>{très}fatigué.
```

La convention vaut en début de ligne, en milieu de paragraphe, et dans les styles nommés (`soit manuscrit: un style = <SCHOLA italique>`).

---

<a id="les-exemples-par-niveau"></a>
