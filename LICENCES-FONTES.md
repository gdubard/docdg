# Les polices embarquées

docdg joint à ses documents une cursive et trois polices de texte. Aucune ne
dépend de la machine : un document se compose avec les mêmes lettres, donc les
mêmes coupures de lignes et de pages, sous macOS, Windows et Linux, dans
l'aperçu comme dans le PDF.

## La cursive

docdg joint une seule cursive à ses documents : la **Marelle**. Elle est la
seule dont la licence permet expressément l'incorporation dans un document et
la redistribution avec un logiciel ; sa notice voyage dans le fichier de fonte
lui-même. Aucune autre cursive n'est embarquée — nommée, une cursive est
cherchée sur le système, ou jointe au document par son fichier lorsque
l'auteur la possède, sous sa propre responsabilité de licence.

## Marelle

Copyright 2026 Ministère de l'Éducation nationale, de l'Enseignement supérieur
et de la Recherche, Laurent Bourcellier, Jonathan Fabreguettes et Rosalie
Wagner. Publiée par la Forge des communs numériques éducatifs.

Sous **SIL Open Font License, Version 1.1** — <https://openfontlicense.org>.
La licence permet expressément l'incorporation d'une fonte dans un document
et sa redistribution avec un logiciel.

*Ce que docdg en fait :* la fonte est convertie en WOFF2 et ses métriques
verticales sont réécrites sur la réglure Seyès — hampe telle que la fonte la
déclare, jambage aux deux tiers, approche de ligne nulle. Aucun contour n'est
touché : le dessin des lettres est celui des auteurs, à l'unité près.

## Les polices de texte

Trois noms de police reviennent dans docdg : **Georgia**, sa police de texte par
défaut, **Times New Roman** et **Arial**, que l'on nomme dans un style
(`<TIMES NEW ROMAN 12pt>`, `<ARIAL>`). Les polices qui portent ces noms
appartiennent à leurs éditeurs et ne se redistribuent pas ; surtout, elles ne
sont pas sur toutes les machines, et un système qui ne les a pas en choisit une
autre, aux chasses différentes. docdg joint donc sous ces trois noms trois
polices libres dessinées pour en reprendre exactement les chasses :

| Nom dans docdg  | Police jointe | Auteurs                                            |
|-----------------|---------------|----------------------------------------------------|
| Georgia         | **Gelasio**   | The Gelasio Project Authors (Eben Sorkin)          |
| Times New Roman | **Tinos**     | The Tinos Project Authors (Steve Matteson)         |
| Arial           | **Arimo**     | The Arimo Project Authors (Steve Matteson)         |

Toutes trois sont sous **SIL Open Font License, Version 1.1** —
<https://openfontlicense.org>, qui en permet l'incorporation dans un document
et la redistribution avec un logiciel.

*Ce que docdg en fait :* les quatre styles de chacune (romain, italique, gras,
gras italique) sont pris dans leur version WOFF2 réduite au latin, telle que la
publie Fontsource 5.3.0 ; aucun contour, aucune métrique n'est touché. Un
caractère que le latin ne couvre pas est demandé à la police suivante de la
liste, comme le veut le CSS.
