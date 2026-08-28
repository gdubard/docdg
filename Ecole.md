# docdg — L'école élémentaire

La classe, les lignes Seyès, l'écriture cursive et les premiers calculs.

> Ce document rassemble ce que docdg apporte à ce niveau.
> Les fonctionnalités communes à tous les niveaux — la syntaxe, les objets,
> les styles, le langage algorithmique, les graphiques — sont décrites dans le
> [README](README.md), qui reste le manuel de référence.

---

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
