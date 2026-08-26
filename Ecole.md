# docdg — L'école élémentaire

La classe, les lignes Seyès, l'écriture cursive et les premiers calculs.

> Ce document rassemble ce que docdg apporte à ce niveau.
> Les fonctionnalités communes à tous les niveaux — la syntaxe, les objets,
> les styles, le langage algorithmique, les graphiques — sont décrites dans le
> [README](README.md), qui reste le manuel de référence.

---

<a id="-lécriture-sur-des-lignes"></a>

## **✏️ L'écriture sur des lignes**

> La réglure Seyès de l'école élémentaire, et de la vraie cursive dessus — modèles d'écriture, lignes à recopier, fiches de copie.

**Syntaxe :** `<Écris>sur des lignes{CONTENU}`

Le verbe seul dans la balise, le support en complément, le texte entre accolades — la règle n°12, sans exception. **Rien ne se déclare** : docdg porte ses cursives et écrit avec la sienne.

```docdg
<Écris>sur des lignes{
Léa mange une pomme, Lili un abricot et Murielle des fraises des bois.
Élodie a vu 3 zèbres et 12 girafes au zoo de Vincennes.
}
```

La cursive par défaut est la **Marelle**, celle du ministère de l'Éducation nationale, publiée sous licence SIL Open Font License. Elle voyage avec le document : la fiche s'imprime sur une machine où elle n'est pas installée. C'est la **seule** cursive embarquée — sa licence permet expressément l'incorporation et la redistribution, ce que les licences des autres cursives scolaires, Schola comprise, ne permettent pas. Toute autre cursive se demande par son nom, et docdg la cherche sur le système — ou la joint au document si on la nomme par son fichier.

**Rien d'autre ne se règle.** Le carreau et l'interligne valent 8 mm — la réglure de l'école élémentaire — et la taille de l'écriture s'en déduit : une police dessinée pour la Seyès y pose ses hampes sur trois interlignes et ses jambages sur deux, ce qui fixe le corps sans qu'aucun calcul soit demandé à l'auteur. Un cahier à la réglure plus large se dit `hauteur: 12;`, et tout suit.

**Une autre cursive.** Trois façons de la désigner, et elles ne se valent pas.

```docdg
seyès: Marelle;                   // embarquée : toujours là, toujours juste
seyès: Schola-Regular.ttf;        // lue à côté du document, et jointe à lui
seyès: Belle Allure;              // cherchée sur le système
```

Nommée par son **fichier**, la fonte est jointe au document et docdg **y lit ses proportions**. C'est ce qui importe : toutes les cursives ne sont pas dessinées à la même échelle dans leur cadratin. La hampe de la Schola monte à 0,90 ; celle de la Marelle monte à 1,44, et il faut la composer à quatre millimètres là où la Schola en demande six et demi. Nommez le fichier, docdg mesure ; nommez seulement la police, il suppose les proportions de la Schola — et une cursive qui s'en écarte sortira trop grande ou trop petite.

**Ce qui vit sur les lignes, et ce qui n'y vit pas.** Le corps du bloc est du docdg ordinaire — styles en ligne, alignements, interpolations, tabulations. Deux précisions :

```docdg
<Écris>sur des lignes{
Le 21<exposant>{è} siècle, H<indice>{2}O, x<exposant>{2}.
3 × 4 = 12 s'écrit tel quel : sur un cahier, c'est une ligne qu'on trace.
}
```

Les **exposants** et les **indices** s'écrivent comme partout ailleurs et ne décrochent pas la réglure : ce sont des fragments de ligne, et ils n'ont pas voix au chapitre sur sa hauteur.

Le **mode mathématique ne s'ouvre pas** : entre les accolades, `$` est un caractère et rien d'autre. Une formule composée sortirait en police mathématique au milieu de la cursive, ce qui n'est pas ce qu'on écrit dans un cahier — « 3 × 4 = 12 » est une ligne que l'enfant trace, non une expression à composer. Écrivez-la donc directement.

**Les marges ne dérangent rien.** Le carreau mesure 8 mm quelles que soient `marges` et `espacements` : la feuille se déplace avec la zone d'écriture, elle ne s'étire pas. Le trait rouge se cale contre le bord gauche du contenu — marge plus espacement — et la première ligne d'écriture reste sur son trait.

**Ce que `taille` et `interligne` ne règlent pas.** Ils gouvernent les scriptes du document — le texte imprimé — et n'ont aucune prise sur le cahier : sur des lignes réglées, le corps et l'interligne ne se choisissent pas, ils se déduisent de la réglure. C'est `hauteur` qui commande, et elle seule.

### **Une ligne de source est une ligne écrite**

Le retour à la ligne du fichier est celui du cahier. Une **ligne vide** est une ligne réglée qu'on laisse à remplir : c'est ainsi qu'on prépare un modèle à recopier.

```docdg
<Écris>sur des lignes{
Zéphyr, le chat gris, dort près de la fenêtre.


}
```

Trois lignes réglées : le modèle, puis deux vides.

### **Une tabulation est un carreau**

Le retrait se dessine dans la source même — l'enseignant qui dit « saute deux carreaux » écrit deux tabulations, et voit sa fiche dans son fichier avant de l'imprimer. Seul le début de la ligne compte : un retour automatique revient à la marge, comme sur un vrai cahier.

```docdg
<Écris>sur des lignes{
				3 × 4 = 12
				5 + 9 = 14
	Victor, Inès et Zineb jouent à cache-cache dans le jardin de l'école.
}
```

### **Le reste de docdg fonctionne**

Styles en ligne, interpolations, calculs : le corps du bloc est du docdg ordinaire. Un mot écrit en scriptes ou plus gros **ne décroche pas la réglure** — l'interligne est posé en millimètres, non en coefficient, si bien qu'un mot plus grand déborde de son interligne, comme le ferait une main, sans que la ligne de base bouge.

```docdg
<Écris>sur des lignes{
À <ARIAL gras>{Paris}, il y a 2,2 millions d'habitants et plus de 130 musées.
	On dit souvent : « Mieux vaut tard que jamais. »
}
```

Et comme l'interpolation vaut là comme ailleurs, un jeu de modèles nominatifs pour toute la classe tient en quatre lignes :

```docdg
pour prenom dans {Léa ; Hugo ; Anaïs} {
	<Écris>sur des lignes{
#prenom mange une pomme.


	}
}
```

La feuille est véritable : trois interlignes fins au-dessus de la première ligne forte — la hauteur d'une majuscule —, deux au-dessous de la dernière — la profondeur d'un jambage —, la réglure qui traverse la marge rouge et continue jusqu'au bord, les carreaux verticaux au bleu des lignes fortes, qui ne commencent qu'à la marge.

> **Ce que le bloc ne fait pas.** Un mot ne se coupe **jamais** en fin de ligne : sur un cahier, la césure n'existe pas, et elle est désactivée dans le bloc même quand le document la demande partout ailleurs. Le mode mathématique ne s'y ouvre pas non plus — `3 × 4 = 12` est une ligne que l'enfant trace, non une formule à composer.

---

## **📚 Le corpus de l'école élémentaire**

> Ce que docdg **énonce** du CP au CM2 — et le fait avec les mots qui serviront jusqu'en licence.

Le corpus descend désormais jusqu'au cours préparatoire. Les cinq niveaux de
l'école y sont des niveaux comme les autres — `CP`, `CE1`, `CE2`, `CM1`, `CM2` —
et s'écrivent dans l'entête du document :

```docdg
document {
	niveau: CM2;
}

<Énonce>Comparaison de deux nombres décimaux
<Énonce>Somme de deux fractions de même dénominateur
```

**Le principe qui gouverne tout ce corpus : la difficulté s'adapte, le
vocabulaire non.** Un terme juste appris au cours élémentaire n'aura pas à être
désappris en licence. C'est pourquoi l'addition nomme ses **termes** et sa
**somme** dès le CP, la division son **quotient** et son **reste** dès le CE2,
et pourquoi deux segments n'y sont jamais « égaux » : ce sont leurs **longueurs**
qui sont de même mesure. La virgule décimale est la seule séparation admise, à
l'école comme à l'agrégation.

Quarante énoncés couvrent les trois domaines du programme : **nombres et
calculs** — chiffre et nombre, unité, dizaine, centaine, millier, les quatre
opérations et leurs propriétés, quotient et reste, unités fractionnaires,
fractions, fractions décimales, nombres décimaux, comparaison ; **grandeurs et
mesures** — longueurs, masses, contenances, aires, volumes, conversions ;
**espace et géométrie** — segment et droite, angle droit, perpendiculaires,
parallèles, polygones, périmètre, polyèdres, patrons, agrandissement et
réduction.

S'y ajoutent les résultats déjà présents dans le corpus qui relèvent du cours
moyen et qui y sont maintenant ancrés : les propriétés du rectangle, du losange
et du carré, la symétrie axiale, l'aire du rectangle, le volume du pavé droit,
les multiples et les critères de divisibilité, l'égalité et la comparaison de
fractions, la proportionnalité et le pourcentage.

### **La glose : le mot juste, expliqué**

Le corpus n'écrit jamais qu'un terme, le bon. Mais aux niveaux où ce terme est
neuf, il paraît accompagné d'une **apposition** qui l'explique — et cette
apposition s'efface d'elle-même dès que le terme est acquis.

| niveau du document | ce que compose `<Énonce>` |
|---|---|
| CP | l'énoncé, puis *Vocabulaire : successeur — le nombre juste après.* |
| CE1 et au-delà | l'énoncé seul |
| cinquième | l'énoncé, puis *Vocabulaire : isométriques — on dit aussi superposables.* |
| quatrième et au-delà | l'énoncé seul |

**La note est à côté, jamais dedans.** L'énoncé se cite tel quel à tous les
niveaux — c'est lui, la source.

C'est l'inverse de ce que font les manuels, qui enseignent « superposables » en
cinquième et le font désapprendre en seconde. Ici l'élève lit le mot exact dès le
premier jour, avec l'aide dont il a besoin ce jour-là — et rien à défaire
ensuite.

La glose ne se demande pas : elle vient du niveau déclaré dans l'entête du
document. Le lexique complet est dans
[docs/REGLES-CORPUS.md](docs/REGLES-CORPUS.md).

**Tout y est `admis`.** Aucune démonstration n'est ancrée à l'école : ce n'est
pas le lieu de la preuve rédigée, et le corpus ne prétend pas le contraire. Un
énoncé consulté à un niveau où il est admis le dit, plutôt que de servir la
preuve d'un niveau supérieur.

Le catalogue complet, niveau par niveau, se lit dans
[Redaction.md](Redaction.md) ; les règles qui le gouvernent dans
[docs/REGLES-CORPUS.md](docs/REGLES-CORPUS.md).

---

## **🗂️ Les exemples de l'école**

| Fichier | Ce qu'il montre |
|---|---|
| `seyes1` | la réglure Seyès et la cursive — modèles d'écriture, lignes à recopier |
| `seyes2` | la réglure sur du texte long — justification, styles, tabulations |
| `calcul1` | les quatre opérations, décomposition, fractions de même dénominateur, division euclidienne, conversions |
| `geometrie1` | les figures planes — points placés, droites et segments, triangles, cercles, repère |
| `histoire1` | les grandes périodes de l'histoire, en frise |
| `basique1` | la langue et la mise en forme |
| `couleurs1` | le nuancier des 148 teintes et les trois régimes d'accord |

Chacun a son PDF de référence dans `pdf/`.

---
