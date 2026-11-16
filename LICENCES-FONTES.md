# Les polices embarquées

## État local de qualification du chantier 8

Les 13 fichiers TrueType attendus par `app/build.rs`, absents pendant les
chantiers précédents, sont restaurés depuis les archives personnelles locales
`docdg-typst0.zip` et `docdg-typst1.zip`. Leurs octets sont identiques entre ces
deux archives ; les noms, versions et empreintes sont consignés dans
`audit_chantier_08/fontes_metriques_et_notices.json` et
`audit_chantier_08/fontes_restauration.json`. La Marelle retenue est la version
DocDG 1.005 de 350 420 octets ; la variante TexDG 1.004 n'est pas utilisée.

La base locale de licence des trois familles de texte est la déclaration
OFL 1.1 ci-dessous, leur copyright incorporé et leur champ de licence pointant
vers openfontlicense.org. Le texte OFL intégral et ces copyrights accompagnent
les fichiers dans `app/polices/OFL-Polices-Texte.txt`. Aucune notice individuelle
amont absente n'est prétendue retrouvée et aucun nom réservé supplémentaire
n'est inventé. Pour Marelle, la notice incorporée correspond à
`app/polices/OFL-Marelle.txt`. Les fontes sont restaurées sans conversion ni
modification d'octets ; aucune police propriétaire du système n'est copiée.

Les déclarations générales de portabilité ci-dessous décrivent le mécanisme
prévu. Leur qualification effective dépend des fontes chargées, des glyphes et
des scénarios testés ; elles ne constituent pas à elles seules une preuve de
métriques identiques sur toutes les machines.

DocDG joint à ses documents une cursive et trois polices de texte, et
s'appuie sur les polices que le compositeur Typst intègre. Aucune ne dépend de
la machine : un document se compose avec les mêmes lettres, donc les
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

Sous **SIL Open Font License, Version 1.1** — <https://openfontlicense.org>,
avec le nom de fonte réservé « Marelle ». La licence permet expressément
l'incorporation d'une fonte dans un document et sa redistribution avec un
logiciel ; son texte intégral accompagne la fonte dans `app/polices/OFL-Marelle.txt`.

*Ce que DocDG en fait :* rien. L'application embarque le fichier TrueType
d'origine, `Marelle-Regular.ttf` en version 1.005, octet pour octet ; le
moteur en garde une copie compressée en WOFF2, sans autre transformation.
Aucun contour, aucune métrique n'est touché : la fonte distribuée est celle
des auteurs, sous son nom. Le calage sur la réglure Seyès — hampe telle que
la fonte la déclare, jambage aux deux tiers, approche de ligne nulle — est
obtenu par la composition (bords de ligne imposés par Typst, `ascent-override`
et `descent-override` côté moteur), jamais en réécrivant la fonte.

## Les polices de texte

Trois noms de police reviennent dans docdg : **Georgia**, sa police de texte par
défaut, **Times New Roman** et **Arial**, que l'on nomme dans un style
(`<TIMES NEW ROMAN 12 pt>`, `<ARIAL>`). Les polices qui portent ces noms
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

*Ce que DocDG en fait :* les quatre styles de chacune (romain, italique, gras,
gras italique) sont pris dans leur version WOFF2 réduite au latin, telle que la
publie Fontsource 5.3.0, et l'application les embarque convertis en TrueType,
format que lit Typst ; aucun contour, aucune métrique n'est touché. Un
caractère que le latin ne couvre pas est demandé à la police suivante de la
liste, comme le veut le CSS.

## Aucune police de la machine, sauf si on la nomme

Les noms **Georgia**, **Times New Roman** et **Arial** — et leurs équivalents
génériques `serif`, `sans-serif`, `monospace`, ainsi que Times, Helvetica et
Courier — désignent toujours les polices jointes, dans le texte comme dans les
figures, même sur une machine où les originales sont installées. Un caractère
qu'elles n'ont pas est demandé, dans cet ordre, à Libertinus Serif, New
Computer Modern Math et DejaVu Sans Mono, toutes embarquées. Une police du
système n'est chargée que si le document la nomme expressément et que DocDG
ne l'embarque pas. Un même document produit donc le même PDF, aux mêmes
lettres, sous macOS, Windows et Linux.

## Les polices du compositeur

Typst, qui compose l'aperçu et le PDF, intègre ses propres polices, que DocDG
utilise en repli et pour les formules :

| Police | Usage dans DocDG | Licence |
|---|---|---|
| **New Computer Modern** et **New Computer Modern Math** | formules mathématiques | GUST Font License |
| **Libertinus Serif** | repli pour les caractères absents des polices jointes | SIL Open Font License 1.1 |
| **DejaVu Sans Mono** | code | licence Bitstream Vera, avec les ajouts DejaVu dans le domaine public |

Elles sont distribuées par le projet Typst (`typst-assets`) sans modification.

