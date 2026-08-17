# Journal des versions

Ce projet suit un versionnage simple : le premier chiffre marque un changement
de nature (ce qu'on peut faire avec docdg), le second une extension dans le
même esprit.

## 2.7 — les énoncés numérotés et l'histoire

### Ajouté

**Les environnements numérotés.** Le théorème, la proposition, la propriété,
le lemme, le corollaire, la définition, l'exemple et la remarque s'énoncent :

```
<Énonce>le théorème de Pythagore <étiquette>{pythagore} {
Dans un triangle rectangle, le carré de l'hypoténuse est égal à la somme
des carrés des deux autres côtés.
}
```

Le titre se compose en vedette — « **Théorème 3.2 (Pythagore).** » — et le
corps le suit, en italique pour les énoncés, en romain pour les définitions,
les exemples et les remarques. Chaque genre tient son compteur, remis à zéro
au chapitre, qui préfixe les numéros comme il le fait des sections. Rien
d'inventé : l'étiquette et le renvoi sont ceux du document long, le style du
titre se configure comme celui d'une section (`soit théorème = <bleu nuit
petites capitales>`).

**La preuve loge dans l'énoncé.** Un sous-bloc `démonstration { … }` se
compose en vedette — *Démonstration.* — en romain sous l'énoncé en italique,
et se referme d'un tombeau. Avec un raisonnement, c'est la machinerie de
`<Montre>` qui rédige : `démonstration par récurrence que … {
initialisation{…} hérédité{…} }` déroule le squelette complet, étapes
nommées et conclusion comprises, et les dix raisonnements s'écrivent tels
quels. Quand l'énoncé tient en une phrase, la propriété à démontrer peut se
taire : la démonstration la lit dans l'énoncé lui-même.

**L'énoncé est une citation.** Ce qui s'y déclare y demeure : un `<Soit>`
posé dans un théorème n'existe plus après lui. C'est aussi ce qui permet au
cache incrémental de prédire le rendu sans lire les corps — l'invariant est
prouvé par un test dédié.

Le document d'exemple `publication4` montre l'ensemble en situation.
Dix genres s'énoncent : théorème, proposition, propriété, lemme,
corollaire, axiome, conjecture, définition, exemple, remarque.

**La frise chronologique — l'histoire entre en scène.** Une vraie frise,
non une ligne du temps : un bandeau gradué en années rondes que referme
une grande pointe, les périodes à l'intérieur avec leur nom et leurs
bornes, les événements à l'extérieur en cartouches reliés à leur date.
Une ligne par événement — la date, deux-points, le titre, et, s'il
éclaire, le détail entre parenthèses — et la frise prend toute la largeur
utile de la page, portrait ou paysage.

Les bornes d'une période s'écrivent de quatre façons au choix,
`de 1789 à 1799` comme `1789 - 1799`, `1789 -- 1799` ou `1789 — 1799`.
Les périodes qui se suivent se touchent sans se gêner, celles qui se
chevauchent se partagent la hauteur du bandeau, et celles qui sont trop
étroites pour se nommer dedans se nomment dans un cartouche.

Les cartouches se rangent d'eux-mêmes dans l'ordre du temps et ne se
chevauchent jamais : ils se répartissent de part et d'autre du bandeau,
en alternance le long du temps, et s'étagent sur autant de rangées qu'il
faut de chaque côté. Rien n'est jamais barré — les traits de rappel se
tracent tous avant les cartouches, qui sont opaques, et les verticales de
la graduation s'interrompent devant chaque cartouche comme devant chaque
bandeau, ne subsistant que dans les espaces libres.

La date se lit en `AAAA`, `MM/AAAA` ou `JJ/MM/AAAA`, l'année négative est
admise — l'Antiquité s'écrit `-52 : Alésia` — et « vers » dit
l'incertitude des sources. La date s'imprime telle qu'elle s'est écrite,
et la légende reprend ce qui suit « la frise chronologique ».

**La frise multilinéaire.** Une bande nommée s'ouvre d'un `politique {`
— le deux-points se tolère, `Vie politique: {` — et se referme d'une
accolade seule ; elle porte son nom dans la marge, ses périodes dans son
bandeau et ses cartouches de part et d'autre, dans sa couleur. Toutes les bandes partagent la même échelle du temps : ce qui
est simultané se lit dans une même verticale, et deux découpages
concurrents se confrontent d'un coup d'œil.

Les documents d'exemple suivent les niveaux du programme : `histoire2`
(collège — frise linéaire, frise par périodes), `histoire3` (lycée —
frise thématique et multilinéaire, périodisation des régimes) et
`histoire4` (supérieur — les trois temps de Braudel, deux périodisations
concurrentes en regard).

### Corrigé

**Le manuel annonçait une durée d'essai périmée.** Il parlait encore de
quatre-vingt-dix jours quand le moteur en accorde trois cent soixante-cinq
depuis la 2.6.

**Le document zoomé se laissait mal atteindre.** L'aperçu changeait
d'échelle par une transformation, qui peint plus grand sans rien changer
à la boîte de mise en page : une compensation manuelle rattrapait la
hauteur, rien ne rattrapait la largeur, et le centrage rendait la partie
gauche d'une page trop large inatteignable. Le zoom passe par la
propriété du même nom, qui entre dans la mise en page, et le centrage
cède au bord dès que la page déborde.

**Les deux volets n'avaient pas de bord.** L'éditeur portait exactement
la couleur de la barre d'outils : il devient une surface encadrée, posée
sur son châssis, nommée « Source » dans son coin, et son cadre prend la
couleur du focus quand on y écrit. L'aperçu reçoit le même cadre et le
même cartouche, « Aperçu » — deux zones nommées de part et d'autre du
séparateur ; l'impression, elle, n'en garde rien. Le fond de l'aperçu
passe au sombre, comme celui de la saisie : la page blanche s'y détache
seule, et le hors-page cesse de tirer l'œil.

**La CI rejouait le banc d'essai sans le regarder.** L'étape de
non-régression appelait un binaire ambigu — trois binaires, pas de
`default-run` — et, une fois désambiguïsée, aurait appelé celui qui ne
vérifie aucun plafond. Elle exécute désormais `froid_chaud`, qui mesure en
médianes et sort en erreur au dépassement.

**Le calcul formel ne peut plus geler le rendu.** Une requête au moteur
SymPy est désormais bornée dans le temps — vingt secondes par défaut,
`DOCDG_DELAI_CAS` pour ajuster. Au-delà, l'ouvrier perdu dans son
exploration est tué et remplacé, et l'erreur s'affiche dans le document
comme toute autre.

**La version minimale de Rust est déclarée.** `rust-version = "1.79"`,
vérifiée : le transpileur compile tel quel avec la 1.79, et les dépendances
de l'application déclarent toutes un plancher inférieur.

## 2.6

### Ajouté

**L'écriture sur des lignes réglées.** `<Écris>sur des lignes{ … }` pose la
Seyès de l'école élémentaire et y écrit en cursive. C'est le geste qui
manquait au niveau que docdg servait le moins : le modèle d'écriture, la
ligne à recopier, la fiche de copie du cahier du jour.

```
page {
	seyès: Schola;
}

<Écris>sur des lignes{
Léa mange une pomme, Lili un abricot et Murielle des fraises des bois.
}
```

*Le principe qui a guidé la syntaxe :* **ne rien inventer.** Le retrait ne se
dit pas, il se dessine — une tabulation vaut un carreau, et l'enseignant qui
dit « saute deux carreaux » écrit deux tabulations. La source ressemble à la
fiche avant qu'elle ne s'imprime. Une ligne de source est une ligne écrite ;
une ligne vide est une ligne réglée qu'on laisse à remplir. Aucun attribut,
aucun mot nouveau : le bloc réutilise la tabulation, les styles en ligne et
l'interpolation qui existaient déjà.

**Rien ne se calcule.** Le carreau et l'interligne valent 8 mm, et la taille
de l'écriture s'en déduit : une police dessinée pour la Seyès pose ses hampes
sur trois interlignes et ses jambages sur deux, ce qui fixe le corps sans
qu'aucun calcul soit demandé à l'auteur. Un cahier plus large se dit
`hauteur: 12;`, et tout suit.

**L'interligne est une longueur, non un coefficient.** C'est ce qui permet à
`<ARIAL gras 14pt>{Paris}` de traverser une ligne sans la décrocher : le mot
écrit plus gros déborde de son interligne — ce que fait une main — sans que
la ligne de base bouge d'un dixième de millimètre. Un `line-height` sans
unité s'hérite comme nombre et se recalcule sur la taille de chaque mot ; la
réglure aurait enflé au premier mot plus grand.

**Un mot ne se coupe jamais.** Sur un cahier, la césure n'existe pas : elle
est désactivée dans le bloc même quand le document la demande partout
ailleurs.

**La Schola voyage avec le document.** La cursive nommée est cherchée sur le
système, comme toute police nommée — mais si c'est la Schola, docdg la joint
au document : une fiche s'imprime alors sur une machine où elle n'est pas
installée. Une fois, en tête, et seulement si le document écrit vraiment sur
des lignes.

### Changé

**Trois polices, trois mots.** Le bloc `page` distinguait `police` et `math`.
Il en distingue maintenant trois, une par écriture : `script:` pour le texte
imprimé, `seyès:` pour le manuscrit, `math:` pour les mathématiques. Les deux
premières ont leur défaut ; la cursive n'en a pas — personne ne peut deviner
celle qui est installée, et une cursive de remplacement ne serait pas une
cursive. Sans elle, `<Écris>sur des lignes` le dit plutôt que d'écrire en
Times sur des lignes.

*Ce que cela demande aux documents existants :* `police:` devient `script:`.

**Le carreau devient l'unité de retrait du document.** `tabulation` et
`hauteur` valent désormais 8 mm au lieu de 10 et 5 — la réglure de l'école
élémentaire, et la seule valeur qui n'ait jamais besoin d'être écrite. Une
tabulation décale donc de 8 mm partout, pas seulement sur des lignes.

### Corrigé

**Le PDF n'était pas à la hauteur de l'écran.** La réglure était peinte en
image de fond, et Chromium rastérise les fonds à l'export `--print-to-pdf` :
le motif partait dans le document en pixels, rééchantillonnés et compressés
avec perte. D'où ce que le zoom montrait — des carreaux verticaux
franchement disparus, puis, une fois leur épaisseur relevée, des traits
tantôt bleus tantôt magenta, tantôt doublés — quand le même document était
impeccable à l'écran. Une image de fond ne pouvait pas faire mieux : elle
n'est pas de la géométrie.

La réglure est donc désormais un `<svg>` dans le document, en millimètres,
que Chromium exporte en tracés vectoriels : le PDF ne contient plus une
seule image, et l'impression est aussi nette que l'écran à n'importe quel
grossissement. Une seule réglure couvre le bloc — le bloc la coupe à ses
dimensions réelles —, ce qui règle du même coup le repli des lignes : une
ligne de source qui s'étend sur trois lignes écrites est réglée sur les
trois. Et les traits gagnent l'épaisseur qui leur manquait pour tenir à
l'impression : 0,18 mm pour les interlignes, 0,25 mm pour les lignes et les
carreaux, 0,5 mm pour la marge.

**Le bleu des carreaux disputait la page à l'écriture.** Le bleu pur est
criard : à côté de lui, les lettres cessaient d'être ce qu'on regarde
d'abord, et les verticales semblaient passer devant elles alors qu'elles
passent dessous. La grille prend un bleu d'un cran plus clair et moins
saturé. Elle redevient ce qu'elle est — un support — et l'encre ressort.

**Les métriques de la cursive étaient un pari sur la fonte.** La géométrie
de la réglure suppose une ascendante de 0,903 em et une descendante de
0,602 em : c'est ce qui place la ligne de base aux cinq huitièmes du pas.
Mais elle se contentait de l'espérer, et la place réelle dépendait de ce que
le moteur lit dans la fonte — approche de ligne appliquée ou non, arrondis
au pixel entier —, chacun à sa façon. Ces trois valeurs sont désormais
imposées par `ascent-override`, `descent-override` et `line-gap-override` :
elles cessent d'être un pari pour devenir une consigne. La fonte est
déclarée pour toute cursive, y compris celle qui est installée sur la
machine — c'est le seul moyen d'en fixer les métriques.

**Rien ne disait que la réglure passe sous l'écriture.** Elle y passait par
le seul ordre du document, et les moteurs ne rangent pas tous de la même
façon ce qui est positionné. Un plan négatif le dit maintenant, et le bloc
s'isole pour que ce plan ne s'enfonce pas plus bas que lui.

**La justification manquait aux styles en ligne.** Le manuel annonçait
`justifie` parmi les alignements horizontaux, mais le style en ligne ne
connaissait que `centre`, `gauche` et `droite` : `<justifié>` ressortait tel
quel, chevrons compris. Il passe désormais par le même span en bloc que ses
trois voisins — et donc, sur un cahier, par la même hauteur de ligne. Les
deux graphies sont admises.

**docdg porte désormais ses cursives, et n'en réclame plus aucune.** La
Marelle — la cursive du ministère de l'Éducation nationale, publiée sous
licence SIL Open Font License 1.1, qui en permet expressément
l'incorporation — est embarquée au binaire et devient la cursive par défaut.
`<Écris>sur des lignes` fonctionne donc sans que le bloc `page` déclare quoi
que ce soit, et la fiche s'imprime sur une machine où rien n'est installé. La
Schola reste la seconde cursive embarquée, et se demande par son nom.

*Ce que cela retire :* l'erreur « aucune police manuscrite n'est déclarée »
n'a plus lieu d'être. Elle se justifiait tant que docdg dépendait de ce qui
était installé sur la machine ; il n'en dépend plus.

**Les métriques d'une cursive sont écrites dans la fonte, non demandées au
moteur.** Les surcharges CSS `ascent-override` et consorts ne suffisaient
pas : un moteur peut les ignorer sans le dire, et l'écriture s'enfonçait
alors sous son trait — mesuré à 0,9 mm dans l'aperçu, quand le PDF était
juste. Toute fonte que docdg embarque, la sienne comme celle que l'auteur
nomme par son fichier, voit donc ses métriques verticales réécrites sur la
Seyès avant l'embarquement : la hampe telle qu'elle la déclare, le jambage
aux deux tiers, l'approche de ligne nulle, et cela dans les trois tables que
les moteurs consultent, `USE_TYPO_METRICS` posé par-dessus. Aucun moteur n'a
plus à choisir.

**Fermer la fenêtre pose désormais une question.** Le panneau annonçait un
état — « Le document a été modifié depuis le dernier enregistrement » — et
laissait l'auteur en tirer les conséquences entre « Revenir au document » et
« Quitter sans enregistrer ». Il demande maintenant « Quitter sans
enregistrer ? », et l'on répond oui ou non.

**La marque de la barre n'avait pas de dimensions propres.** Un SVG en ligne
qui ne porte qu'une boîte de vue vaut cent pour cent de son conteneur, et non
son rapport naturel : la lettre pouvait s'étaler sur toute la barre selon le
moteur. Elle porte ses dimensions en attributs.

**Les barres de défilement se voient.** Dans un thème sombre, celles du
système s'effacent, et l'on ne distingue plus un contenu qui ne déborde pas
d'un défilement en panne.

**Le bas des pages était hors d'atteinte à fort grossissement.** L'aperçu
est mis à l'échelle par une transformation, qui peint plus grand au même
encombrement : la boîte de mise en page, elle, ne bouge pas. Au-delà de
100 %, le document dépassait donc la zone défilable sans qu'aucune barre
n'apparaisse ; en deçà, un blanc s'ouvrait en pied, proportionnel à la
réduction. Le conteneur reçoit désormais la hauteur que la transformation lui
prend ou lui donne. La mesure se prend sur la hauteur de mise en page, que la
transformation ignore : elle ne dépend pas de son propre résultat.

**L'interface prend les couleurs de la feuille.** docdg fabrique de la Seyès :
il lui manquait d'en avoir l'air. Les quinze valeurs écrites en dur dans la
feuille de style deviennent dix-neuf variables nommées, et la palette est
prélevée sur la réglure — bleu roi de la ligne d'écriture pour l'action
principale, bleu ciel des interlignes pour le focus, rouge de la marge pour
l'alerte et rien d'autre. Un thème devient possible ; il ne l'était pas.

**La barre d'outils se range en trois cartouches.** Quatorze boutons de même
poids visuel : rien ne ressortait. Fichier, Édition et Vue deviennent trois
encadrés à la manière des `fieldset` des réglages, nom posé sur le filet — et
**chacun porte sa couleur** : le bleu roi de la ligne d'écriture pour ce qui
touche au fichier, le bleu ciel des interlignes pour l'édition, le vert du
second crayon pour la vue. Un bouton la porte tamisée au repos et pleine au
survol ; l'export et les volets affichés la portent pleine. Le rouge reste
hors du jeu : il ne dit que l'alerte. La marque de docdg, le `d` cursif sur
son trait de marge, ouvre la barre.

**Une pastille dit que le document a changé.** Le panneau de fermeture le
demandait au moment de partir ; rien ne le disait avant. La pastille rouge
paraît dans la barre dès la première frappe et s'efface à l'enregistrement.

*Trois libellés suivent l'action qu'ils déclenchent :* « Charger » devient
« Ouvrir » et « Paramétrer » devient « Réglages » — le nom du panneau qu'il
ouvre.

**Un exposant ne décroche pas la réglure, même hors de l'application.** La
règle qui retire aux fragments de ligne le droit de fixer sa hauteur ne
visait que les `span` ; l'application neutralisait `sup` et `sub` de son
côté, si bien que le cahier en dépendait sans le dire. Il ne dépend plus de
rien : la règle les vise aussi.

**Appliquer les paramètres referme le panneau.** Il restait ouvert sur le
document qu'il venait de changer, et il fallait rouvrir les paramètres pour
le refermer et voir le résultat.

**Une autre cursive que la Schola sortait à moitié trop grande, ou pas du
tout.** Deux fautes se cumulaient. La première : docdg cherchait la fonte par
`local('Marelle')`, or `local()` ne compare pas au nom de famille mais au nom
complet et au nom PostScript — la fonte se présentant comme « Marelle
Regular » ou « Marelle-Regular », elle n'était pas trouvée, et l'écriture
retombait sur une police de secours. Les trois formes sont désormais
proposées.

La seconde tenait à une supposition. Le corps se déduit de la hampe — une
lettre montante doit couvrir trois interlignes — et docdg prenait pour
acquise la hampe de la Schola, 0,90 cadratin. Toutes les cursives ne sont pas
dessinées à la même échelle dans leur cadratin : celle que j'ai sous les yeux
monte à 1,44, et se trouvait donc composée à six millimètres et demi là où
quatre suffisaient — moitié trop grande, débordant sur les lignes voisines.

**Une cursive peut désormais se nommer par son fichier.** `seyès:
Marelle-Regular.ttf;` cherche la fonte à côté du document, la joint à lui
comme docdg joint la Schola, **et y lit ses proportions** : la hampe se lit
dans l'en-tête de la fonte, le jambage s'en déduit — deux interlignes contre
trois, c'est la Seyès qui le dit et non la fonte. Le corps et la ligne de
base suivent. Une cursive seulement nommée reste hors de portée : docdg ne
peut pas ouvrir ce qui est installé sur le système, et il continue d'y
supposer les proportions de la Schola. Nommez le fichier, et il mesure.

**L'aperçu et le PDF ne sont pas composés par le même moteur.** C'était la
clé, et elle a mis du temps à se voir : l'aperçu vit dans la vue web du
système — WebKit sous Linux —, le PDF sort de Chromium en headless. Or un pas
de 8 mm vaut 30,236 pixels, et WebKit arrondit la hauteur de ligne au pixel
entier là où Chromium la garde entière. La réglure, exacte, prenait donc un
quinzième de millimètre d'avance à chaque ligne dans l'aperçu — un millimètre
et demi en bas de page, mesuré — et rien du tout à l'impression, où le même
document était irréprochable.

Chacun reçoit désormais ce qu'il sait tenir. **À l'écran**, le pas est dit en
pixels entiers : il n'y a plus rien à arrondir, et la réglure est étirée
d'autant, si bien que les deux avancent du même pas. **À l'impression**, où le
moteur est exact, les huit millimètres sont rendus tels quels — le papier n'y
perd pas un centième. Le millimètre reste la mesure de la feuille ; le pixel
n'est que la monnaie de l'écran, où il n'a jamais représenté un millimètre
réel. Mesuré sur les deux chemins : la dérive sur vingt-sept lignes vaut
0,014 mm à l'impression et 0,006 mm à l'écran, contre 1,88 mm auparavant.

**Un alignement faisait flotter sa ligne.** `<centre>`, `<gauche>` et
`<droite>` produisent un span qui passe en bloc. La règle anti-gonflement,
faite pour les fragments de ligne, lui retirait toute hauteur : un titre
centré flottait un demi-pas au-dessus de la réglure, et tout ce qui suivait
glissait d'un pas entier — invisible, puisque la réglure se répète au pas,
mais faux. Un span en bloc **est** une ligne : il en reprend la hauteur. La
règle vaut pour tout alignement à venir ; une justification s'y rangera
d'elle-même.

**L'écriture décrochait de sa ligne par intermittence.** Le moteur cale
chaque ligne écrite sur le pixel entier. Or un pas de 8 mm en vaut 30,236 :
la fraction s'accumule d'une ligne à l'autre et se rattrape tous les quatre
pas, si bien que l'écriture décrit une dent de scie d'un pixel — un quart de
millimètre — autour de sa ligne. La réglure, elle, est vectorielle et
exacte : rien ne peut les accorder, et plus la page avançait, plus l'œil
attrapait les lignes en haut de la dent.

Les traits chevauchent donc leur position au lieu d'y pendre : la ligne
forte est centrée sur la ligne de base, les interlignes sur la leur. Une
demi-épaisseur de part et d'autre couvre justement la dent de scie — le
pied des lettres reste dans le trait, qu'elles montent ou qu'elles
descendent, et le jour au pire cas passe de 0,13 mm à 0,01 mm. Deux
conséquences en cascade : les garnitures du bloc deviennent égales, et
l'interligne qui tombait sur chaque ligne forte — jusqu'ici caché dessous —
n'est plus tracé, puisqu'il dépasserait. Il n'y en a jamais eu que trois
entre deux lignes fortes.

**Un cahier plus long qu'une page était perdu.** Le bloc `<Écris>sur des
lignes` était insécable : ne tenant pas sur la page en cours, il basculait
entier sur la suivante, y débordait, et tout ce qui dépassait le bas de
page était rogné — page blanche d'un côté, texte tronqué de l'autre. Le
cahier se scinde désormais comme la prose : les lignes entières qui tiennent
restent, la ligne frontière est scindée à une frontière de ligne écrite
lorsqu'elle s'y prête, et le reste part en page suivante. La réglure, elle,
ne se coupe pas — elle se redessine : chaque moitié reçoit la sienne, posée
depuis son propre bord haut, si bien que la page suivante recommence une
feuille entière, coiffe comprise.

**La fenêtre refusait parfois de se fermer.** Le premier clic sur la croix
n'ordonne pas la fermeture : il interroge l'éditeur, qui ouvre le panneau
« enregistrer ou quitter » si le document a changé. Restait à savoir quand
passer outre — c'était une temporisation de deux secondes, et elle était le
défaut : le second clic ne fermait que s'il tombait dans ces deux secondes,
sinon il ne faisait que rouvrir un panneau déjà ouvert, sans rien changer à
l'écran. Une demande de fermeture est désormais un état, non un instant :
tant qu'elle est en cours, le clic suivant ferme, quel qu'ait été le délai.
Répondre « rester » retire la demande, et la croix retrouve son
comportement ordinaire.

**La feuille commençait à gauche du trait rouge.** La réglure débordait de
deux carreaux dans la marge de la page, comme une feuille dépasse de sa zone
d'écriture. C'était joli et c'était faux : sur une fiche, la zone d'écriture
commence où commence la marge. Le trait rouge est maintenant calé contre le
bord gauche — marge de page plus espacement —, et rien n'est tracé à sa
gauche.

**Les réglages de page ne traversaient pas les fils de rendu.** Ils vivent
dans un `thread_local` : les fils de rayon ne voient pas ceux du fil
appelant. La police manuscrite s'y perdait, et le même document rendait une
réglure en séquentiel et une erreur en parallèle — c'est-à-dire dans
l'application. Les deux rendus sont désormais comparés par un test.

**Un mot écrit plus gros décrochait la réglure.** Même avec un interligne
absolu sur le bloc, un span à grande taille gonfle sa boîte de ligne par ses
propres métriques — mesuré : 8,14 mm au lieu de 8 — et toute la suite du
bloc glissait. La règle `span{line-height:0}`, jointe avec la feuille de
style de l'écriture, retire aux mots le droit de fixer la hauteur : seul
l'étai du bloc la donne. Les vingt-six lignes du document d'exemple mesurent
désormais 8,000 mm chacune, mot en 14 pt compris.

**Le mode mathématique s'ouvrait sur le cahier.** Le manuel promettait le
contraire : « 3 × 4 = 12 » est une ligne que l'enfant trace, non une formule
à composer — et la formule sortait en police mathématique au milieu de la
cursive. Le dollar redevient un caractère dans le bloc.

**La feuille devient véritable.** Trois retouches, toutes prélevées sur une
feuille Seyès réelle. La réglure porte désormais trois interlignes fins
au-dessus de la première ligne forte — la hauteur d'une majuscule — et deux
au-dessous de la dernière — la profondeur d'un jambage : deux coiffes d'un
huitième de pas complètent ce que la géométrie des lignes donne déjà. Les
carreaux verticaux prennent le bleu des lignes fortes — sur la feuille, la
verticale n'est pas un interligne — et la marge rouge se pose par-dessus
tout, en dernière couche, si bien que l'ordre dans lequel un moteur peint
cesse d'avoir la moindre importance.

**Une seule écriture pour une consigne.** Le manuel admettait deux
placements du complément — `<Affiche un cadre avec ...>{...}`, tout entre les
chevrons, et `<Affiche>un cadre avec ...{...}`, le chevron refermé sur le
verbe — en les déclarant équivalents. C'était deux mots pour une notion. Il
n'en reste qu'un : **le chevron se referme toujours sur le verbe**, la
consigne d'abord et l'objet ensuite, comme on parle. Les cent treize balises
du manuel sont réécrites, et la règle n°12 ne présente plus qu'une forme.

**Deux retouches de finition sur la feuille.** L'écriture ne pose plus sa
première lettre sur le trait rouge : elle le laisse respirer d'un
millimètre. Et le trait rouge cesse d'être saucissonné par les lignes qui le
croisent — il est tracé d'un seul tenant, en dernier, de la tête au pied du
bloc.

**Un nom déclaré avalait l'accolade du corps.** `<Soit>la matrice M{`
nommait la matrice « M{ » : tout calcul qui la rappelait ensuite ne la
trouvait plus, et la forme que le manuel donne pour canonique ne fonctionnait
donc pas. Le nom s'arrête désormais où commence le corps.

**Un complément ne pouvait pas passer à la ligne.** Le chevron refermé sur le
verbe, le complément devait tenir sur une seule ligne : toute énumération
d'attributs un peu longue restait condamnée à tout enfermer entre les
chevrons. Une consigne se poursuit maintenant à la ligne suivante dès que la
ponctuation l'appelle — une virgule d'énumération, des deux-points qui
annoncent des valeurs — ou tant que la ligne reste en retrait sous elle. La
poursuite ne s'amorce que sur cette ponctuation : une consigne ordinaire
suivie d'un alinéa n'absorbe pas le paragraphe.

**Les traits de la réglure étaient trop gras.** C'est l'écriture qui doit
ressortir de la page, non son support : les traits sont désormais au plus
fin que l'impression tienne. Seule la marge garde du corps — elle est le
repère où l'œil revient.

**Les carreaux suivaient l'écriture au lieu de la marge.** Le millimètre de
respiration ajouté après le trait rouge déplaçait la grille des carreaux
verticaux, qui quittait celle de la marge. Réglure et écriture ne partagent
plus la même origine : la première se pose sur la boîte de remplissage du
bloc, la seconde à un millimètre de là.

**Deux réglages morts encombraient la structure des pages.** `taille_pt` et
`interligne` avaient été ajoutés aux réglages transmis au rendu quand la
géométrie de la réglure dérivait encore de la typographie du document ; elle
ne dépend plus que de `hauteur`, et ces deux champs ne servaient plus qu'à
faire tousser le compilateur.

**Le signe moins typographique gonflait sa ligne.** U+2212 est absent des
cursives scolaires : le glyphe sortait en police de secours au milieu de la
cursive, et les métriques de cette police gonflaient la boîte de ligne
(8,084 mm au lieu de 8). Sur un cahier, le moins est le trait : U+2212
devient le trait d'union dans le bloc, et nulle part ailleurs.

Cette version s'ouvre par ailleurs sur les correctifs d'un audit de la 2.5 :
rien de visible dans les documents, tout dans la mécanique.

### Changé

**La bibliothèque des démonstrations se charge une fois, non à chaque
appel.** Chaque `<Montre>` résolu par la bibliothèque reparsait les 80 Ko de
`demonstrations.json` — les cent deux fiches entières, dont cent une ne
serviraient pas — puis normalisait chaque clé de chaque fiche pour la
comparaison. La base se parse désormais au premier appel et jamais ensuite,
clés et énoncés normalisés une fois pour toutes au chargement : la recherche
d'une fiche ne coûte plus que la comparaison de ses clés. Le fichier étant
embarqué au binaire, sa validité est une invariante de compilation ; un
test le verrouille avant chaque publication.

**Les temps de rendu se consignent.** `BENCH.md` fixe le protocole — les
quatre temps du crate `bench/` sur `vitrine4.txt` et `algo4.txt` — et tient
le relevé de chaque version : une dégradation doit se voir au commit fautif,
pas à la publication.

**Le code neuf écrit dans le tampon, pas à côté.** Le patron
`sortie.push_str(&format!(…))` — 162 occurrences héritées — alloue une
chaîne pour la jeter aussitôt copiée ; la règle pour tout code nouveau est
`write!(sortie, …)`, qui écrit directement. L'existant ne migre que là où
la mesure le justifie : sur un moteur aussi capillaire que la rédaction
française, une réécriture en masse coûterait plus en régressions qu'elle ne
rapporterait en microsecondes.

### Corrigé

**Une fiche mal formée n'interrompt plus la recherche.** Une fiche sans
champ « clés » faisait abandonner la recherche entière au lieu de passer à
la fiche suivante : toutes les fiches placées après elle dans la base
devenaient introuvables, sans le moindre message. Chaque fiche se lit
désormais pour elle-même.

**Un marqueur de conflit traînait dans ce journal.** Reliquat d'une fusion,
à la frontière des sections 2.5 et 2.4 — retiré.

**Les versions des crates suivent le journal.** `transpiler` et `app`
affichaient encore 2.4.0 alors que la 2.5 était publiée ; les trois crates
de l'atelier portent désormais le même numéro que la version en chantier.

### Outillage

**L'intégration continue rejoue tout à chaque poussée.** Un exemple du
manuel qui cesse de rendre faisait déjà échouer la compilation — encore
fallait-il lancer les tests. Un workflow GitHub Actions les lance désormais
sur Linux et macOS, compile l'application, et termine par le banc d'essai :
la suite de tests ne dépend plus de la discipline de qui pousse.

## 2.5 — la rédaction du supérieur

### Ajouté

**Les démonstrations : neuf raisonnements, une charpente chacun.** Le verbe est
`<Montre>` et lui seul — « Montrons que » compte 425 occurrences dans le corpus
dépouillé, « Démontrons que » 3. Le raisonnement direct est la forme nue ; les
neuf autres se nomment après le verbe, d'un seul nom chacun : `par
contraposée`, `par l'absurde`, `par récurrence`, `par disjonction de cas`, `par
analyse-synthèse`, `par double inclusion`, `pour tout`, `par le principe des
tiroirs`, et `l'existence et l'unicité` par le complément.

```
<Montre>par récurrence que pour tout entier $n$, … {
	initialisation{ … }
	hérédité{ … }
}
```

*La règle d'or s'applique ici comme partout* : le verbe seul dans la balise, le
raisonnement et l'énoncé en complément. Les dix noms sont uniformes — tous
introduits par « par », y compris la propriété universelle qui se dit `par
élément quelconque` : « pour tout » ouvrait aussi bien un énoncé qu'une demande
de raisonnement, et le moteur ne pouvait pas les distinguer.

Le raisonnement est **exigé quand l'auteur écrit le corps** — les étapes le
nomment déjà, `initialisation{}` n'a de sens que pour une récurrence. Il est
**facultatif quand le moteur fournit la démonstration**, puisque le
raisonnement est alors une propriété de la preuve, non de la demande ; mais
s'il est donné et qu'il contredit la fiche, il est refusé plutôt que passé sous
silence — sans quoi le corps d'une preuve directe se verrait réclamer les
étapes d'une récurrence.

Le moteur fournit la charpente — l'annonce, les étapes étiquetées, la
conclusion quand la logique l'exige — et l'auteur n'écrit que les
mathématiques. Le principe directeur vient d'Alain Troesch : **chaque
raisonnement s'annonce avant de se dérouler**. Pour l'analyse-synthèse,
l'annonce n'est pas une politesse — l'analyse commence par supposer la
conclusion vraie, et le correcteur pressé qui ne lit pas l'annonce y voit une
pétition de principe.

Les clôtures sont dosées d'après le corpus, où les formules d'achèvement sont
quasi absentes (« ce qui achève la démonstration » : 12 occurrences sur 2,1
millions de mots ; « cqfd » : 0). Ne sont refermés que les raisonnements dont
la conclusion fait partie de la logique : la récurrence invoque son principe,
la contraposée revient à l'énoncé direct, l'absurde conclut de la
contradiction, la disjonction constate que les cas couvrent tout, la double
inclusion assemble ses deux moitiés. Une étape oubliée est signalée par son
nom ; un raisonnement inconnu répond en listant les formes admises.

Le corps d'une étape est du docdg vivant : prose, mathématiques,
interpolations et commandes s'y composent. Deux documents d'exemple,
`demonstration3.txt` et `demonstration4.txt`, déroulent la panoplie.

**Le moteur démontre lui-même ce qui est à sa portée.** L'absence de corps
signifie dans les démonstrations ce qu'elle signifie partout dans le langage :
le moteur fait le travail. `<Montre>par récurrence que pour tout entier $n$,
$somme(k=0;n) k = (n(n+1))/2$` — sans accolades — vérifie l'initialisation et
l'hérédité par le calcul formel, puis rédige la démonstration complète, chaîne
d'égalités comprise. `<Montre pour tout>réel $x$, $x^2 + 1 >= 2x$` établit
l'inégalité par la forme canonique : la différence s'écrit carré plus
constante positive, et la rédaction le dit.

**Une bibliothèque de démonstrations classiques, du lycée à la L3.** Tout ne se
calcule pas : l'irrationalité de \(\sqrt2\), le théorème de Cantor, la
divergence de la série harmonique reposent sur une idée, et aucun système de
calcul formel ne les trouvera. **Cent deux** démonstrations sont écrites
une fois et rangées dans une base au format JSON, appelées par leur seul
énoncé :

```
<Montre>que $racine(2)$ est irrationnel
<Montre>qu'il existe une infinité de nombres premiers
<Montre>l'existence et l'unicité de la division euclidienne
```

La base stocke le **corps en docdg**, non du HTML : une fiche traverse la même
charpente que la démonstration écrite à la main, et une correction profite
aussitôt à toutes les formes de rendu. Les énoncés se comparent après
normalisation — minuscules, accents pliés, mathématiques et ponctuation
retirées — de sorte que « racine de 2 est irrationnel » et
« que $racine(2)$ est irrationnel. » désignent la même fiche. L'auteur garde le
dernier mot : un raisonnement précisé dans la balise l'emporte sur celui de la
fiche.

*Le principe qui délimite la base :* **on n'y range que ce qu'aucun outil ne
sait faire.** SymPy, SciPy et les crates couvrent la grande majorité des
démonstrations exigibles ; une fiche dont le contenu se réduit à une identité ou
à une inégalité vérifiable serait une redondance, et deux sources pour un même
résultat finissent toujours par diverger. Quatre fiches ont ainsi été retirées —
formule du binôme, inégalité de Bernoulli, parité d'un produit d'entiers
consécutifs, carré pair — parce qu'elles relèvent du calcul formel : elles
figurent désormais à la feuille de route du pont SymPy, pas dans la base.

La base est incorporée au binaire et suit les versions — docdg travaille en
classe, hors ligne, sans dépendance ni compte à créer.

Et le moteur **refuse de démontrer un mensonge** : une formule fausse est
rejetée avec la raison — « la formule est fausse au premier rang », « la
formule ne se transmet pas au rang suivant », « la positivité ne se lit pas
sur une forme canonique ». La vérification précède toujours la rédaction.

### Ajouté

**Chaque version a un terme.** docdg reste distribué gratuitement, mais une
installation est valable **90 jours**. La barre d'outils affiche discrètement
les jours restants, en gris tant que le terme est loin, en avertissement dans
les quinze derniers jours — personne ne doit découvrir l'échéance le matin où
il en a besoin. Passé le délai, l'application s'ouvre sur une page qui explique
où trouver la version suivante ; l'éditeur n'est pas chargé.

La date d'installation vit dans `installation.json`, au même endroit que les
réglages d'interface. La date de dernière ouverture y est conservée à côté, et
le calcul retient la plus tardive des deux : **reculer l'horloge du système ne
rend pas de jours**.

*Ce que le verrou ne fait pas :* il ne touche à aucun document. Les fichiers
`.docdg` sont ceux de l'utilisateur, sur son disque, et la version suivante les
rouvre tels quels. Le terme porte sur le logiciel, jamais sur le travail.

### Changé

**La bibliothèque de démonstrations n'a plus qu'une source.** Le fichier
externe, que l'enseignant pouvait placer à côté de l'exécutable ou dans son
dossier de réglages, est retiré : la base livrée avec le logiciel suit les
versions, et deux sources pour un même résultat finissent toujours par
diverger. Une fiche corrigée arrive désormais avec la mise à jour.

### Interface

**Annuler et rétablir, enfin.** Ctrl+Z annule, Ctrl+Y ou Ctrl+Maj+Z rétablit,
et deux flèches ↶ ↷ dans la barre d'outils font de même à la souris. Le
textarea perdait son historique natif à chaque réécriture programmée du
contenu — tabulation, réglages de page, chargement — : l'éditeur tient
désormais le sien, les frappes rapprochées groupées par gestes de 400 ms,
les insertions ponctuelles isolées dans leur propre pas, l'ouverture d'un
document repartant d'un historique vierge.

**La prose de calcul se scinde entre les pages.** Une résolution pas à pas,
une étude, une position relative — toute sortie rédigée du moteur — formait
un seul bloc qu'il était impossible de répartir sur deux pages : au premier
paramètre modifié, une demi-page se vidait. Ces blocs se scindent désormais
par nature : les paragraphes entiers qui tiennent restent, le paragraphe
frontière se coupe s'il s'y prête, le reste part en page suivante — et
seules les formules hors texte demeurent d'un seul tenant.

**La numérotation des pages se choisit.** Dans le bloc `page { … }`,
`numérotation: simple;` affiche `1`, `2`, `3` ; `numérotation: sans;` ne
numérote pas ; par défaut, la forme composée `1 / 3` demeure. L'option écrite
à la main survit à l'ouverture du panneau de réglages.

**Le grand blanc avant une figure reportée se comble.** Un objet insécable —
figure, tableau de signes ou de variations — qui ne tenait pas dans l'espace
restant basculait en page suivante en laissant une demi-page vide. Les blocs
qui le suivent remontent désormais combler ce blanc, avec trois interdits qui
préservent le sens de lecture : jamais au-delà d'un titre, jamais par-dessus
un saut de page, et l'on s'arrête au premier bloc qui ne tient pas — seul
l'objet reporté flotte, comme une figure de livre, le texte garde son ordre.

**En disposition empilée, le code garde le haut.** L'aperçu passait au-dessus
du code quand les panneaux s'empilaient ; l'ordre naturel — le code en haut,
le rendu en bas — est rétabli, et le séparateur se mesure depuis le haut en
conséquence.

### Corrigé

**Deux rendus étaient faux sans être en erreur — des tests de contenu les
verrouillent.** « <Décompose>4 782 » répondait par une décomposition en
éléments simples (un résultat absurde pour un entier) : la commande écrit
désormais la décomposition positionnelle de l'école — « 4 782 = (4 × 1 000) +
(7 × 100) + (8 × 10) + 2 », chaque produit parenthésé pour que les paquets se
voient, rangs à zéro omis — et la décomposition en éléments
simples se demande avec son complément. La rosace à 4 pétales en dessinait 8 :
\(r = \cos(k\,t)\) donne \(k\) pétales si \(k\) est impair, \(2k\) sinon —
« la rosace à N pétales » choisit maintenant le coefficient d'après la parité,
et l'exemple demande des pétales plutôt qu'un coefficient. Un nouveau fichier
de tests (`contenu.rs`) vérifie ce qui est écrit, pas seulement l'absence
d'erreur.

**Le recueil fait loi — et des tests le vérifient.** Le document de référence
sur le langage mathématique du supérieur est désormais adossé à une suite de
tests : aucun symbole logique (∀, ∃, ⇒, ⇔) dans la prose — le critère est
celui du recueil, le *mélange* de mots et de symboles dans une même phrase,
les formules isolées et les tableaux restant tolérés — ; « Soient » au
pluriel ; le nom puis la nature ; pas de « tel que » parasite dans les
déclarations. L'audit a corrigé six fiches de la bibliothèque qui écrivaient
« il existe N tel que \(n>N\Rightarrow…\) » : l'implication s'y dit
désormais en français — « tel que, pour n>N, … ».

**Le mot juste pour le point : on le place.** `<Place>un point B(3 ; 1)` le
déclare et le marque — c'est le geste du compas posé sur la feuille, et c'est
le mot des enseignants. Le point placé est connu de tout le document : la
collecte des déclarations le voit au même titre qu'un `<Soit>`.

**Chaque action du manuel a son exemple — et les niveaux sont exacts.** L'audit
de couverture croise désormais les commandes du manuel avec les 30 fichiers
d'exemple : arbres horizontaux et en prose, disque, demi-droites dans les deux
sens, cercle par son centre, fonction en la variable \(t\), champ de vecteurs,
pivot pas à pas — tout ce qui manquait a rejoint le fichier de son niveau. Le
PGCD, les pourcentages et les programmes de calcul sont au collège
(`algebre2`), pas à l'école élémentaire : `calcul1` s'en tient au programme du
cycle 2 et 3 — opérations, décomposition, fractions de même dénominateur,
division euclidienne, conversions. Un exemple migre au passage vers la forme
verbe-seul des listes (`<Dresse>une liste à puces`) ; l'insertion d'images
garde sa balise d'objet — `<Insère une image …>{fichier}` — qui est la forme
du manuel (règle n°12). Et
`vitrine4.txt` assume l'autre bout du spectre : la démonstration de force, où
chaque ligne source produit une rédaction complète — pour le futur site comme
pour le lecteur pressé.

**Deux nouveaux documents d'exemple, et un audit de couverture.** Les
commandes du manuel ont été confrontées aux fichiers d'exemple : l'arithmétique
n'y figurait nulle part. `calcul1.txt` la couvre — fractions, PGCD, division
euclidienne, notation scientifique, pourcentages, programmes de calcul — et
`geometrie1.txt` déroule les figures planes. Deux tests permanents verrouillent
l'ensemble : chaque bloc du manuel rend sans erreur, et chaque exemple aussi —
au nombre d'erreurs près pour les deux fichiers qui **démontrent** le canal
d'erreur, dont le compte exact est verrouillé.

**Chaque bloc du manuel rend sans erreur — et un test le garantit.** Les 155
exemples de code du README sont désormais extraits et rendus par la suite de
tests : un exemple du manuel qui échouerait ferait échouer la compilation de
la version. Pour y parvenir, chaque bloc est autoportant — les objets qu'il
utilise s'y déclarent —, les droites s'y tracent par leur équation réduite
(« la droite y = 2x + 1 ») ou entre deux points déclarés (« la droite (AB) »),
les régions du plan se grisent (« la région y > x + 1 »), les racines
n-ièmes d'un complexe quelconque se calculent (« les racines cubiques de
8i »), et un commentaire de fin de ligne n'atteint plus jamais le solveur.

**Les figures planes du manuel, toutes tenues par le moteur.** Le triangle
rectangle se place à son angle droit (« rectangle en A, de côté AB 3 cm et de
côté AC 4 cm »), l'isocèle a ses deux côtés égaux, les longueurs s'entendent
aussi en millimètres. Le segment et la demi-droite portent leur longueur
(« [AB] tel que AB = 4 cm »), le cercle se donne par rayon, par diamètre ou
par référence à deux points déclarés (« de rayon AB »). « <Trace>le point A »
reprend un point posé par `<Soit>`, `<Trace>{ … }` groupe des figures ou des
solides sans changer le vocabulaire, et un objet manquant produit son erreur
nommée — « le point A n'a pas été déclaré — posez-le par <Soit>un point
A(x;y) ».

**Trois commandes documentées rendues opérantes.** `<Représente>graphiquement la
fonction f` sans intervalle trace désormais dans la fenêtre standard
\([-5\,;\,5]\) — la commande la plus naturelle du logiciel échouait sur son cas
le plus simple. `<Soit>{ … }` en bloc déclare une ligne après l'autre, chaque
ligne passant par le même chemin que la forme en ligne. Et le champ de
vecteurs s'énonce : « Soit \(F\) le champ de vecteurs de \(\mathbb{R}^3\) dans
\(\mathbb{R}^3\) défini par… » — déclaration purement notationnelle, comme le
manuel l'annonçait sans que le moteur le fasse.

**Le manuel ne promet plus ce que le moteur ne tient pas.** La suite se déclare
par `<Soit>la suite u définie par u(0) = 1 et u(n+1) = 2u(n) + 1` — la forme
`<Définis>…{ }` documentée n'avait jamais existé. Les statistiques et la droite
graduée s'écrivent en prose (« avec les données {…} », « sur [-5 ; 5],
d'intervalle {…} ») — l'écriture `attribut:valeur` que le manuel montrait est
celle-là même qu'il déclare abandonnée. La section des opérations posées est
retirée : la fonctionnalité n'existe pas encore.

**Une démonstration ne bascule plus en bloc à la page suivante.** Elle sortait
enveloppée dans un conteneur, et la mise en page ne scinde que ce qu'elle sait
scinder : le conteneur en faisait un bloc unique, qu'un manque de place en fin
de page renvoyait tout entier plus loin — en laissant derrière lui un blanc
parfois considérable. La démonstration rend désormais une **suite de
paragraphes** sans enveloppe : chacun se pose où il tient, et la coupure se
fait entre deux étapes plutôt qu'avant la première.

### Changé

**La déclaration nomme avant de qualifier.** docdg écrivait « Soit une fonction
f(x) = x² − 2 », « Soit le point A(2 ; 3) », « Soit la matrice M = … ». L'usage
du supérieur pose l'inverse : le **nom** d'abord, sa **nature** ensuite.

```
Soit f la fonction définie par f(x) = x² − 2.
Soit A le point de coordonnées (2 ; 3).
Soient A et B les points de coordonnées (1 ; 2) et (−1 ; 2).
Soit u le vecteur de coordonnées (3 ; −2).
Soit M la matrice définie par : M = …
Soit (u_n) la suite définie par récurrence, avec …
```

*La mesure, plutôt que le jugement.* Six ouvrages de référence — Deschamps,
Nguyen, Gugger, Monier, Gorny, Preteseille, soit 2,1 millions de mots — ont été
dépouillés. L'ordre nom-nature l'emporte **777 contre 15**, et chez les six
auteurs sans exception : 291 contre 6 chez Deschamps, 149 contre 2 chez
Preteseille, 103 contre 0 chez Gorny. Ce n'est pas une préférence d'auteur,
c'est une convention.

Le système garde l'ordre inverse — « Soit (s) le système » — parce que son nom
est une étiquette parenthésée et non le nom propre de l'objet, et que c'est la
seule forme que le corpus atteste.

**La déclaration est une phrase, non une formule.** Elle sortait en bloc
mathématique, les mots français enveloppés de `\text{}`. Alain Troesch le dit
sans détour dans ses *Fondements* : une bonne rédaction passe par une mise en
langage des règles logiques, on rédige par phrases et non par un enchaînement
de formules. Seule l'expression reste désormais en mathématiques.

Au passage, la forme retenue satisfait une règle que l'ancienne frôlait de
trop près : `f(x)` désigne une **valeur**, jamais une application. « Soit une
fonction f(x) = … » nommait fonction ce qui n'en est pas une ; « Soit f la
fonction définie par f(x) = … » nomme `f`, et n'emploie `f(x)` qu'après
« définie par ».

**L'impératif s'accorde** : « Soient A et B les points… ». Les deux formes se
rencontrent dans les manuels — l'invariable y est même la plus fréquente, 363
contre 209 devant deux noms coordonnés. Mais docdg sert de modèle à qui rédige
une copie, et l'accord est la seule forme qu'un correcteur ne relèvera jamais.

### Corrigé

**La virgule décimale n'est plus prise pour un séparateur de coordonnées.**
`<Soit>un point A(2;-1,5)` posait un point de l'espace, de coordonnées
(2 ; −1 ; 5). Les composantes se coupaient sur le point-virgule **et** sur la
virgule, alors que la règle n°8 du manuel réserve celle-ci au rôle de virgule
décimale à l'intérieur d'un nombre. Encadrée de chiffres, elle appartient
désormais au nombre — et la faute passait jusqu'ici sans le moindre message.

---

## 2.4 — l'informatique au complet

La partie informatique — algorithmique et programmation — couvre désormais
tout le programme de NSI : les conteneurs et les fonctions qui les reçoivent
et les rendent, les piles et les files, les arbres et les graphes, et les
objets avec leurs quatre piliers. Rien n'y manque plus.

L'autre moitié du travail ne se voit pas dans une liste de fonctionnalités :
le vocabulaire et la syntaxe ont été repris pour qu'une notion ne s'écrive
que d'une seule façon. Plusieurs écritures s'étaient accumulées pour une même
action, chacune ajoutée de bonne foi, l'ensemble obligeant le lecteur à se
demander si elles diffèrent. Elles n'en font plus qu'une.

### Ajouté

**La programmation orientée objet.** Classes, attributs typés, méthodes,
constructeur : le dernier chapitre du programme de terminale NSI qui manquait.

```
soit une classe Point {
	abscisse: un réel
	ordonnée: un réel

	soit norme(): un réel = racine(abscisse * abscisse + ordonnée * ordonnée)
	soit translaté(dx: un réel ; dy: un réel): un Point = Point(abscisse + dx ; ordonnée + dy)
}

soit p: un Point = Point(3 ; 4)
```

La phrase de déclaration est celle des mathématiques — « soit une classe
Point ». Le nom d'une classe commence par une majuscule : c'est ce qui le
distingue d'un type du langage. Un attribut se lit `p.abscisse`, une méthode
s'appelle `p.norme()` — le point est ce que l'élève verra en Python, et il ne
sert qu'à l'intérieur d'un calcul, jamais dans la prose où il termine les
phrases.

Dans le corps d'une méthode, **les attributs sont visibles par leur nom**,
sans préfixe : `racine(abscisse * abscisse + …)` se lit comme la formule.

**Décision de conception : une classe est une fonction qui construit.** En
Python, `Point(3, 4)` *est* un appel de la classe. Le constructeur est donc
enregistré comme une entrée ordinaire de la table des fonctions, et les
méthodes sous la clé `Point.norme` — si bien que la vérification d'arité et de
types est celle qui existait déjà, et qu'aucune signature du moteur n'a changé.

**L'encapsulation.** Le défaut est la visibilité, comme en Scala ; un seul mot
la retire, et il se place en tête, comme `private` en C# :

```
soit une classe Compte {
	titulaire: une chaîne
	privé solde: un réel

	privé soit taux(): un réel = 0,02
	soit intérêts(): un réel = solde * taux()
}
```

Un mot, un défaut : il n'y a rien à retenir pour le cas courant, qui est celui
de l'élève. `privée` est accepté au féminin.

Ce qui est privé reste **entièrement visible depuis la classe** : `intérêts()`
lit l'attribut privé et appelle la méthode privée. Du dehors, l'un et l'autre
sont refusés avec un message qui dit l'issue — « il ne se lit que depuis la
classe ».

Une méthode en appelle une autre **par son nom nu**, sans préfixe : c'est tout
l'objet de l'encapsulation, une méthode publique s'appuyant sur des méthodes
privées. Le nom est complété à la lecture de la classe, et les attributs sont
passés sous les noms qu'ils portent déjà dans la portée de l'appelante.

Les fautes sont dites : arité du constructeur, type d'un attribut, attribut
absent, méthode absente, attribut ou méthode privé, objet d'une classe passé
pour une autre.

**L'héritage.** Un mot suffit, et c'est celui du français :

```
soit une classe Chien hérite de Animal {
	race: chaîne

	soit cri(): chaîne = "ouaf"
}
```

L'enfant reçoit les attributs du parent **en tête des siens**, ses méthodes, et
ses secrets — ce qui est privé chez le parent le reste chez l'enfant. Une
méthode redéfinie l'emporte, sans que celle du parent en soit affectée.

**Un enfant tient la place de son parent** : `décrit(x: Animal)` accepte un
chien, et `x.carte()` fonctionne. L'inverse est refusé et dit — « un Animal
n'est pas un Chien ».

*Décision de conception : la lignée voyage avec la valeur.* `Valeur::Objet`
porte le nom de sa classe **et la liste de ses ancêtres**, si bien que le
vérificateur de types n'a jamais à consulter la table des classes — il lit ce
qu'il a sous la main. Les méthodes héritées sont recopiées chez l'enfant à la
lecture de la classe, avec ses attributs à lui : l'appel reste un appel
ordinaire.

**Les arbres et les graphes** — sans un seul type nouveau. Le programme les
demande par leurs représentations, et le langage les portait déjà : un graphe
est une **liste d'adjacence**, c'est-à-dire un dictionnaire dont les valeurs
sont des listes ; un arbre parfait tient dans un tableau, les fils de `i`
étant en `2i+1` et `2i+2`.

```
soit g: dictionnaire de textes et de listes de textes = {A: {B ; C} ; B: {D} ; C: {D} ; D: {}}
```

Les parcours **en largeur** et **en profondeur** s'écrivent avec la file et la
pile — c'est l'exercice du programme, et c'est ce qui donne son sens à leur
distinction.

Deux verrous levés pour y parvenir :

- **Un attribut peut contenir un objet.** Les attributs voyagent sous forme
  écrite jusqu'à l'appel, et un objet ne se relisait pas. Une valeur a
  désormais une **forme relisible**, distincte de sa forme affichée : un point
  s'imprime `Point(abscisse: 3)` pour le lecteur, mais se réécrit `Point(3)` —
  l'appel qui le construit. Sans cela, ni arbre chaîné ni liste chaînée.
- **La source d'une boucle n'est plus forcément un nom** : `pour v dans g[s]`
  parcourt ce qu'une lecture indexée a rendu.

**Les piles et les files**, nommées, avec leurs six opérations et leur
discipline.

```
soit p: pile d'entiers = {}
soit p = empile(p ; 1)
soit p = empile(p ; 2)
#{sommet(p)}   → 2      #{dépile(p)}   → {1}

soit f: file d'entiers = {1 ; 2}
#{tête(f)}     → 1      #{défile(f)}   → {2}
```

`empile`, `dépile`, `sommet` d'un côté ; `enfile`, `défile`, `tête` de
l'autre ; `est vide` et `longueur` pour les deux. Chaque opération rend une
**nouvelle** structure : `soit p = empile(p ; 3)` montre que la pile d'après
n'est pas celle d'avant.

**La discipline est tenue** — c'est tout leur intérêt. Une file n'a pas de
sommet, une pile n'a pas de tête, aucune des deux ne s'indexe, et ni l'une ni
l'autre ne se confond avec une liste. Une structure vide n'a ni sommet ni tête,
et le dit.

Ces types ne remplacent pas l'exercice : `algo4` continue de montrer comment
les écrire à la main, ce que le programme demande avant de les employer. Une
**fonction écrite par l'utilisateur l'emporte d'ailleurs sur la primitive du
même nom** — sans quoi l'élève qui nomme la sienne `sommet` ou `dépile` se
heurterait au langage au lieu d'apprendre.

**L'abstraction.** Une classe peut dire ce que ses filles doivent savoir
faire, sans dire comment : une méthode déclarée sans membre droit n'a pas de
corps.

```
soit une classe abstraite Forme {
	nom: chaîne

	soit aire(): réel
	soit carte(): chaîne = nom
}

soit une classe Carré qui hérite de la classe Forme {
	côté: réel

	soit aire(): réel = côté * côté
}
```

Une classe abstraite ne s'instancie pas — « seules ses classes filles le
font ». Et une classe concrète ne peut rien laisser sans corps : « ou bien la
classe les définit, ou bien elle se déclare abstraite ». C'est la faute que
l'abstraction sert à dire, et elle se dit à la déclaration, non à l'appel.

**La modification d'un attribut** : `p.x = 5`, sans `soit` puisque rien n'est
déclaré. Le type déclaré de l'attribut est vérifié, un attribut inexistant est
dit, et ce qui est privé ne s'écrit pas plus du dehors qu'il ne s'y lit.

*Sur mutation ou copie.* docdg n'a **pas de références** : une boîte n'est
jamais partagée. Muter la boîte et lui réaffecter une copie modifiée sont donc
indiscernables — il n'y a rien qu'un marqueur de référence pourrait
distinguer, et le langage reste entièrement par valeur. Une méthode, elle,
reçoit ses attributs en copie : pour transformer un objet, elle en **rend un
nouveau**, comme `translaté`.

**Le polymorphisme.** La liaison est **dynamique par construction** : le nom
de classe est lu dans la *valeur*, non dans le type déclaré. Un paramètre
déclaré `Animal` qui reçoit un chien appelle `Chien.cri()`.

```
soit z: liste d'Animal = {Chien("Rex") ; Chat("Mia")}
pour a dans z {
	[#{a.cri()}]
}
```

Deux verrous levés pour y parvenir. Un objet peut désormais se **construire à
l'intérieur d'un littéral** de collection. Et un objet **ne survit pas à son
impression** : la boucle qui le parcourt reçoit la valeur, non sa forme
écrite. Au niveau du document, chaque tour reçoit sa propre boîte sous un nom
qui lui est propre — le contenu d'un tour est déroulé sur-le-champ mais lu
plus tard, et un nom réemployé ne porterait que la dernière valeur.

**L'héritage se dit de quatre façons**, de la plus brève à la plus parlée :
`hérite de`, `hérite de la classe`, `qui hérite de`, `qui hérite de la
classe`. C'est la phrase qui commande.

**Le type se passe du déterminant** dans une classe : `x: réel` plutôt que
`x: un réel`. À ce niveau la prose n'apporte rien, et l'article reste accepté.

**L'arbre binaire récursif s'écrit sans valeur nulle** : l'arbre vide est une
classe. `Arbre` abstraite, `Vide` et `Nœud` filles — la hiérarchie dit tout, un
arbre est vide ou c'est un nœud. Rien à ajouter au langage : les quatre piliers
suffisaient.

**Les fonctions reçoivent et rendent des conteneurs et des textes.** Jusqu'ici
`appelle` rendait un `f64` et liait ses paramètres dans une table de nombres :
la documentation le disait — « les paramètres et valeurs de retour sont des
nombres ». Aucun tri, aucune recherche, aucune manipulation de texte ne
s'écrivait donc en fonction. La contrainte est levée.

- Un paramètre déclaré `une collection de ...`, `un dictionnaire ...`, `une
  matrice ...` ou `un texte` est lié en **boîte locale**, non en nombre.
- La valeur retournée est une `Valeur`, vérifiée contre le type annoncé par le
  même `verifie` que les conteneurs.
- **Le passage se fait par copie** : une fonction reçoit une valeur, elle ne
  peut rien renvoyer dans la série de l'appelant. C'est ce qui rend saine
  l'acceptation d'une `collection d'entiers` là où une `collection de réels`
  est attendue — aucune écriture ne peut y injecter un réel.
- Dans un corps de fonction, `soit t = v` copie une boîte, et `t[i] = x` écrit
  par indice : le tri par insertion s'écrit tel qu'un manuel le montre.
- `a + b` joint deux collections ou deux textes, et `a + {x}` fait croître une
  collection, à l'intérieur d'une fonction comme au dehors.
- Un fragment nu attendu comme texte *est* ce texte, conformément à l'usage
  des littéraux (`{chat ; chien ; cheval}`).

**L'interpolation rend les valeurs composées.** `#{f(v)}` où `f` rend une
collection affichait « calcul absent » : l'évaluateur numérique ne sait pas
lire `{1 ; 2 ; 3}`. La forme littérale est désormais rendue telle quelle.

**Les messages de type nomment ce qu'ils attendent, avec l'article accordé** :
« 3 ne se lit pas comme une collection d'entiers » — genre et élision compris.

**Les primitives de conteneur.** Dix mots, résolus avant les fonctions de
l'utilisateur — elles ne sont pas redéfinissables :

| Rend une valeur | Rend vrai ou faux |
|---|---|
| `longueur`, `somme`, `min`, `max` | `contient(x ; v)` |
| `tri`, `inverse` | |
| `indice de(x ; v)` | |
| `insertion(v ; i ; x)`, `suppression(v ; i)` | |

Le nommage suit une règle : un **nom** pour ce qui produit une valeur, un
**verbe** pour ce qui répond par oui ou non. Le nom composé (`indice de`) est
réservé au langage ; un nom de fonction écrit par l'utilisateur tient en un
seul mot.

- Chacune rend une **nouvelle** collection : la série de départ ne bouge pas.
- `min(a ; b)` reste la fonction mathématique à deux arguments ; seule la forme
  à un argument conteneur est une primitive de collection.
- `tri` sur des textes suit la **collation française** : « école » se classe
  avant « Zoé », alors que le codepoint de « é » vient après celui de « z ».
- `indice de` sur un élément absent est une **faute dite**, non un −1 : la
  valeur sentinelle est un idiome de programmeur, pas de pédagogue.

**La tranche `v[i à j]`**, bornes incluses — comme « de 1 à 5 » fait cinq
tours. Le `à` évite la collision avec l'indexation matricielle `A[i ; j]`. Une
tranche dont la borne gauche dépasse la droite est **vide, et non fautive** :
c'est ce qui permet de conclure une fusion de deux listes triées en une ligne.

**Le `soit` d'un corps de fonction** accepte un type déclaré
(`soit r: une collection de réels = {}`) et reconnaît une valeur composée
(`soit t = tri(v)`), qu'il range en boîte locale au lieu de la forcer en
nombre.

**Le vocabulaire s'aligne sur celui des enseignants.** `liste` et `tableau`
nomment désormais le même type que `collection` ; `chaîne`, `chaîne de
caractères` et `texte` sont synonymes ; `renvoie` s'écrit aussi bien que
`retourne`. `inverse(v)` remplace `miroir(v)` : la syntaxe `inverse(...)` et la
prose « l'inverse de la matrice M » ne se confondent pas, la parenthèse les
sépare.

**`non`**, troisième connecteur logique enseigné avec `et` et `ou`, manquait.
Les conditions lisent aussi `vrai` et `faux` en toutes lettres, et un booléen
s'affiche ainsi : `contient(15 ; notes)` rend « vrai », non « 1 ».

**`insère`, `supprime`, `ajoute`** — à l'impératif, non `insertion` et
`suppression` : le nom aurait télescopé « tri par insertion », l'algorithme et
l'opération portant le même mot dans la même page. `ajoute(v ; x)` nomme enfin
l'opération la plus employée du programme, que la concaténation `v + {x}`
couvrait sans la dire.

**La préposition dit qui reçoit l'opération.** `dans notes insère(0 ; 20)` se
lit comme une phrase et nomme sans ambiguïté le conteneur, là où
`insère(notes ; 0 ; 20)` laissait deviner lequel des trois arguments le
désignait. Trois écritures coexistent, au choix de ce qui se lit le mieux : la
forme fonctionnelle, la forme prépositionnelle, et la forme directe
`notes contient(15)` pour les questions par oui ou non.

**La division euclidienne se dit comme au collège** : `quotient de 17 par 5`,
`reste de 17 par 5` — la phrase dit lequel des deux nombres est le dividende,
ce que `quotient(17 ; 5)` taisait. Les formes brèves restent admises, et la
tournure accepte un appel comme dividende : `quotient de longueur(notes) par 2`.

**`sortir`** arrête une boucle sans quitter ce qui l'entoure — c'est la
recherche qui cesse dès qu'elle a trouvé. Dans un corps de fonction comme dans
une boucle de document.

**La forme française `v contient(1)`.** Le conteneur peut précéder la
primitive et se glisse à sa place parmi les arguments : la phrase se lit à voix
haute. `contient(1 ; v)` reste admis.

**Le p-uplet.** Le programme de NSI le distingue explicitement de la liste :
longueur fixe, types pouvant différer. Il s'écrit `(entier ; entier)`, se
retourne, se délie et se lit par rang.

- `soit divise(a: entier ; b: entier): (entier ; entier) = (quotient de a par b ; reste de a par b)`
- `soit (q ; r) = divise(17 ; 5)` — la **déliaison** pose les deux noms d'un
  coup, au niveau document comme dans un corps de fonction ;
- `soit c: (entier ; texte) = (3 ; "trois")` — les membres peuvent différer, et
  `c[1]` rend le second ;
- l'arité est vérifiée : délier trois noms d'un couple est une faute dite, et
  `(1 ; 2)` n'est pas acceptable là où une liste est attendue.

Les extrema d'une série se rendent enfin en un seul parcours.

**`aléatoire(a ; b)`** tire un entier entre les deux bornes, comprises — un
xorshift sans dépendance, semé par l'horloge. Deux compilations donnent deux
tirages : c'est ce qu'une simulation attend. Sans lui, ni Monte-Carlo, ni
marche aléatoire, ni étude de fréquences.

**`continuer`** arrête le tour en cours ; la boucle poursuit. Le pendant de
`sortir`, dans un corps de fonction comme dans une boucle de document.

**Une déclaration typée accepte un appel**, y compris pour un type scalaire :
`soit x: entier = f(3)`.

**L'affectation d'une valeur composée devient cohérente.** Cinq écritures
naturelles échouaient, chacune pour une raison différente :

- `pour k dans d { … d[k] … }` — une **clé de dictionnaire peut être une
  variable**, non plus seulement un mot écrit à la main ;
- `soit f(): un dictionnaire … = {a: 1}` — l'accolade délimite ici un
  **littéral**, non un bloc d'instructions ; la distinction se fait sur la
  présence d'un mot du langage à l'intérieur ;
- `soit p: une liste d'entiers = empile({} ; 1)` — le membre droit d'une
  déclaration typée **n'est plus forcément un littéral** : un appel, une
  primitive, une concaténation conviennent ;
- `soit p = empile({} ; 1)` — une valeur composée **se pose sans que son type
  soit écrit** ; docdg reconnaît ce qu'elle est ;
- `soit v = tri(v)` — un conteneur **se réaffecte**. L'accumulateur
  `soit S = S + {k}` reste prioritaire et intact.

La pile et la file s'écrivent désormais entièrement dans le langage, en sept
lignes et sans une ligne de moteur.

**Le chapitre « texte » du programme s'écrit enfin.** La déclaration d'une
chaîne ne menait nulle part : sept manques la rendaient inerte. Ils sont
comblés ensemble.

- `m[0]` lit **une lettre**, qui est une chaîne d'un seul caractère — docdg
  n'introduit pas de type distinct, pas plus que Python. L'indice hors bornes
  compte en lettres.
- `pour c dans m` parcourt les lettres, dans un document comme dans un corps
  de fonction.
- `majuscule`, `minuscule`, `sans accents` — `majuscule("été")` rend `ÉTÉ`,
  non `ETE` : la typographie française accentue les capitales.
- `code("B")` et `caractère(97)` font le va-et-vient entre une lettre et son
  rang Unicode.
- `texte(42)` et `nombre("1,5")` convertissent, explicitement.
- Un littéral entre guillemets est lu comme chaîne partout, y compris en
  argument : `m contient("jour")`.
- Une **lecture indexée sert d'argument** : `code(m[0])`, `ajoute(r ; a[i])`.
  La concaténation `r + {a[i]}` le permettait déjà, l'appel nommé non.
- **Deux textes se comparent** dans une condition, avec la collation
  française : `si u vaut inverse(u)` reconnaît un palindrome.

César chiffre, le palindrome se reconnaît, le comptage de lettres tourne.

**La chaîne de caractères devient une valeur ordinaire** :
`soit m: chaîne de caractères = "bonjour"`. Un type scalaire n'a pas de
littéral entre accolades — sa valeur tient sur la fin de la ligne. Les
guillemets, droits ou français, délimitent sans appartenir à la valeur, ce qui
permet d'y garder espaces et signes du langage. La chaîne se mesure
(`longueur`), se découpe (`m[0 à 7]`), se joint (`+`) et se passe en argument.

**`quotient(a ; b)`** — la division euclidienne du collège. Le reste s'obtenait
déjà par `%` ; le quotient entier manquait, alors qu'il donne l'indice du
milieu dans une recherche dichotomique.

### Simplifié

**Une notion, un mot.** Plusieurs écritures s'étaient accumulées pour une même
action — chacune ajoutée de bonne foi, l'ensemble obligeant le lecteur à se
demander si elles diffèrent. Elles sont réduites :

| Retiré | Reste |
|---|---|
| `élaguer`, `compacter` | `élague`, `compacte` |
| `renvoie` | `retourne` |
| `hérite de la classe`, `qui hérite de` | `hérite de`, `qui hérite de la classe` |
| `tableau` comme type | `liste` |

`liste` et `chaîne de caractères` sont les mots qu'emploient les professeurs
d'informatique : ce sont **les seuls** du langage, y compris dans les types
composés — « une liste de chaînes de caractères ». `chaîne`, `texte`,
`collection` et `tableau` ne sont plus reconnus du tout : les garder « pour
compatibilité » aurait été garder les doublons sous un autre nom. Les
variantes sans accent (`chaine de caracteres`) restent admises parce qu'un
clavier peut manquer, non parce qu'elles nomment autre chose.

`<Saisis>une chaîne de caractères` remplace `<Saisis>un texte` : la commande
emploie le vocabulaire du langage.

**Les messages suivent le même vocabulaire** : « la liste compte 3 élément(s) »
et non « la collection », « une chaîne de caractères » et non « un texte ». Un langage qui dit `liste` et se plaint d'une
`collection` fait chercher deux notions là où il n'y en a qu'une.

**Les guillemets délimitent une chaîne de caractères partout** : au singulier,
dans une liste, et désormais **en clé de dictionnaire**, où ils restaient
collés à la valeur. C'est la distinction qui compte pour un professeur
d'informatique — le texte d'un document s'écrit nu, comme en LaTeX ; une
chaîne de caractères se cite, parce qu'elle relève de la programmation.

```
soit mots: liste de chaînes de caractères = {"chat" ; "chien"}
soit trajets: dictionnaire de chaînes de caractères et d'entiers = {"Marche": 5}
#{trajets["Marche"]}
```

### Corrigé

**Un `tant que` de document fait enfin croître un conteneur.**
`soit v = v + {k}` n'y accumulait que la valeur d'avant la boucle :
`subst_var` protège les accolades — et c'est ce qu'il faut, une accolade
délimite des données —, si bien que le tour ne pouvait rien y injecter. La
protection est levée pour les seules lignes qui font croître un conteneur ; la
prose garde ses accolades intactes.

**On ne parcourt plus une pile en silence.** `pour x dans p` ne faisait rien
sans rien dire, ce qui laissait croire la pile vide. C'est désormais une faute
dite : « on ne parcourt pas une pile : on la vide, en la dépilant ».

**Deux noms d'exemple corrigés.** `médiane_basse` devient `médiane` — le nom
composé n'avait aucune raison d'être, et la « médiane basse » n'a de sens que
pour un nombre pair de valeurs, ce qui n'était pas le cas. Sa formule emploie
désormais `quotient de longueur(t) par 2` au lieu d'un décalage écrit à la
main, qui datait d'avant l'existence de `quotient`. `remplace_tête` devient
`modifie`.

La règle est rappelée dans `algo3` : un nom de fonction tient en **un seul
mot**, le nom composé étant réservé aux mots du langage. Le trait bas ne sert
que lorsque deux mots sont vraiment nécessaires — parce que l'algorithme porte
ce nom, tel le tri par insertion.

**`élague` et `compacte`** — deux traitements distincts, que confondre serait
une erreur : le premier retire les espaces des deux bouts, celui dont on a
besoin après une saisie ; le second les retire tous, y compris à l'intérieur,
si bien que les mots se resserrent. L'impératif suit `insère`, `supprime`,
`ajoute` ; `élaguer` et `compacter` sont admis aussi, l'un et l'autre se disant
en classe.

**Un tour sauté ne laisse plus de ligne vide.** `continuer` émettait le
séparateur de fin de tour alors que le tour ne produit rien : une ligne vide
apparaissait là où il n'y a précisément rien. Le séparateur n'est plus émis que
si le tour a écrit quelque chose.

**Une primitive prépositionnelle accepte la parenthèse vide** :
`dans m compacte()`.

**`jonction` et `découpe`** — écrire une suite sur une ligne, et le chemin
inverse.

```
#{jonction(premiers(50) ; ", ")}   → 2, 3, 5, 7, 11, …
#{jonction("un code" ; " - ")}     → u - n -   - c - o - d - e
#{découpe("un code" ; " ")}        → {un ; code}
```

Une boucle qui affiche met chaque tour sur sa ligne ; l'accumulateur, lui,
laisse toujours un séparateur de trop à la fin. Il n'y a pas de raison de faire
compter l'élève.

Trois défauts ont été levés au passage, chacun visible seulement à l'usage :

- **Les guillemets gardent les espaces.** Leur contenu était rogné, si bien
  qu'un séparateur `" - "` devenait `"-"`. C'est pourtant ce à quoi ils
  servent.
- **Un point-virgule entre guillemets est une valeur, non une coupure** : le
  découpage des arguments l'avalait, et `jonction(v ; " ; ")` était illisible.
- **Un texte calculé n'est pas recalculé.** `jonction(v ; " / ")` rendait
  « 1 / 2 », que l'évaluateur reprenait pour une division : le résultat valait
  un demi. Un texte déjà calculé porte désormais une marque, ôtée à
  l'affichage — et ôtée aussi dans les conditions, où le texte est comparé et
  non montré.

**`algo4` met en œuvre les quatre piliers**, avec des exemples simples et
exécutés : un point pour les objets et leurs méthodes, un compte bancaire pour
l'encapsulation, un animal et son chien pour l'héritage, une liste d'animaux
pour le polymorphisme, une forme abstraite pour l'abstraction. Le fichier
annonçait encore la POO comme à venir. Il gagne aussi l'arbre binaire écrit
avec des objets, les graphes par liste d'adjacence, les deux parcours, et les
piles et files du langage à côté de celles écrites à la main.

Ces dernières sont renommées avec un suffixe : **une fonction qu'on écrit
masque la primitive du même nom** — c'est voulu, pour que l'élève puisse écrire
la sienne — mais cela empêcherait d'employer les deux dans un même document.
Le fichier le dit désormais.

**La source d'une boucle est vérifiée.** `pour c dans un code` faisait un
unique tour sur « un code », au lieu d'en lire les lettres : la dernière
branche traitait toute source inconnue comme une liste séparée par des
virgules. Une source qui n'est ni un nom posé, ni un littéral, ni une chaîne
entre guillemets est désormais **dite** — « une chaîne écrite sur place se met
entre guillemets ». Les formes `{…}`, `[…]`, le nom d'une boîte et le littéral
cité restent admis.

**L'affectation se passe de `soit`.** `soit` déclare ; le répéter à chaque
tour n'apprenait rien et ne se lisait pas :

```
soit somme = 0
pour k de 1 à 100 {
	somme = somme + k
}
La somme des entiers de 1 à 100 vaut #somme.
```

La condition qui rend la levée sûre tient en un mot : **le nom doit déjà
exister**. Une ligne de prose contenant un signe égal n'est donc jamais prise
pour une affectation, et un dièse à droite la signe comme prose — on écrit
`k=#k` pour afficher, jamais pour affecter. `soit` reste admis partout.

La réécriture se fait sur le texte, avant que quoi que ce soit ne mesure des
positions : rien en aval n'a eu à changer.

**Un attribut textuel s'affiche sans ses guillemets.** La forme *relisible* —
celle qui les porte — sert à faire voyager une valeur jusqu'à un appel ; à
l'affichage, c'est la forme lue qui convient. `#{c.titulaire}` rendait
« "Léa" » au lieu de « Léa ».

**Une primitive restait inerte dans la condition d'un `si` de document.**
`expand_conditions` n'avait accès ni aux conteneurs ni aux fonctions : la même
écriture fonctionnait dans un corps de fonction et pas dans le document. Les
conditions de tour d'une boucle sont désormais résolues au déroulement, ce qui
rend au passage visible un `sortir` niché dans un `si`.

**Un point-virgule d'appel coupait une cellule de tableau en deux.** `split_top`
comptait les accolades et les crochets, mais pas les parenthèses : la rangée
`[contient(2 ; v) ; #{contient(2 ; v)}]` produisait quatre cellules au lieu de
deux. Les parenthèses comptent maintenant au même titre.

**Les boucles ne tournaient pas dans un corps de fonction.** `expand_loops_avec`
et son homologue pour `tant que` balayaient toutes les lignes du document à la
recherche d'un bloc à dérouler, sans savoir qu'une ligne peut appartenir au
corps d'une fonction. La coupure tombait alors au milieu d'une déclaration,
dont l'accolade fermante restait de l'autre côté :

- avec `pour`, `parse_declaration` échouait sur le fragment tronqué et la
  fonction n'était jamais enregistrée — l'appel ressortait en « … » ;
- avec `tant que`, la boucle était extraite du corps et la fonction rendait la
  valeur qu'elle avait avant d'y entrer.

Les deux balayeurs sautent désormais par-dessus une déclaration de fonction
complète. C'est le même défaut, dans la même famille, que celui qui empêchait
autrefois une condition de fonctionner à l'intérieur d'une boucle.

Un corps de fonction accepte maintenant `pour … de … à …`, `pour … dans …`,
`tant que … faire`, `si … sinon` imbriqués, et l'écriture indexée `t[i] = x`.
Moyenne, comptage et recherche linéaire s'écrivent enfin.

**`sortir` et `continuer` agissent dans un `tant que` de document.** Les deux
mots fonctionnaient dans une boucle `pour` et dans un corps de fonction, mais
restaient sans le moindre effet dans un `tant que` ou un `faire … tant que`
écrit au niveau du document : la boucle allait jusqu'au bout, sans message.
Une recherche séquentielle — l'usage même du `tant que` — sortait donc
fausse.

Le corps d'un tour ouvre désormais ses blocs `si` **au fil des lignes**, avec
les variables telles qu'elles sont à ce point du tour, et non telles qu'elles
étaient en entrant : c'est ce qui rend visible à temps un `sortir` niché dans
un `si`. Un tour interrompu qui ne produit rien ne laisse plus de séparateur
derrière lui — la boucle coupée rend exactement le même document que la
boucle bornée.

**Le lanceur Python de Windows était ignoré.** Le moteur essayait `python3`
puis `python`, jamais `py` — que la documentation annonçait pourtant. Sur une
installation Windows ordinaire, SymPy restait donc silencieusement absent, et
les variations, dérivées et lois tombaient en mode dégradé sans que rien ne
le dise.

**Les messages d'erreur parlent d'une seule voix.** Le moteur avait deux
présentations pour la même chose : un filet rouge en marge pour les calculs,
du gras rouge pour les conteneurs. Le vocabulaire et la mise en forme sont
désormais définis en un seul endroit. Le canal des conteneurs garde la mise
en forme du langage — une instruction de conteneur rend du docdg, que le
moteur relit ensuite, et y glisser du HTML le ferait échapper.

### Interne

**Le rendu ne recopie plus l'environnement à chaque segment.** Les conteneurs
et les fonctions sont partagés derrière un compteur de références, et
l'instantané d'environnement est mutualisé entre les segments qui ne le
modifient pas. Le coût du rendu était le produit du nombre de segments par
celui des conteneurs ; il ne l'est plus. Sur mille paragraphes portant deux
cents conteneurs, le rendu passe de 383 ms à 110 ms.

**La découpe en segments ne dépend plus des lignes vides.** Une ligne vide
reste une coupure — c'est la plus lisible — mais elle n'est plus la seule :
deux lignes de prose se séparent d'elles-mêmes. Un document écrit au
kilomètre ne formait qu'un segment unique, et perdait du même coup le cache
incrémental (toute frappe recomposait tout) et le parallélisme (un segment
n'atteint jamais le seuil) : la mise en forme de l'auteur décidait de la
réactivité de l'aperçu, sans qu'il puisse le savoir. Huit cents lignes de
prose donnent maintenant huit cents segments, et une frappe en fin de
document se recompose en 0,7 ms.

*Décision de conception : la coupure fine se recolle à l'assemblage.* Un
segment était **aussi** l'unité de paragraphe — les lignes d'un même segment
se joignent par un saut de ligne. Couper ailleurs changeait donc le rendu.
Deux paragraphes que seule une coupure fine sépare sont recollés, et la
coupure n'est admise qu'entre deux lignes de prose nue, seul endroit où l'on
sache par avance ce que le rendu en fera : une image ou un cadre décide
lui-même s'il ferme le paragraphe. Le HTML des vingt-trois documents
d'exemple déterministes est identique, octet pour octet.

**Le contrôle de flux quitte `rendu.rs`.** Le fichier mêlait sur plus de
quatre mille lignes l'analyse de la source, l'exécution des boucles, les
saisies et la composition du HTML — c'est là que `sortir` avait fini par
fonctionner d'un côté et pas de l'autre, deux traitements du même mot à six
cents lignes l'un de l'autre. Les boucles, les conditions et les affectations
forment maintenant un module à part.

**Le parcours des délimiteurs est écrit une fois.** Cinq fonctions appariaient
des accolades ou coupaient au premier niveau, chacune avec ses règles :
l'une comptait les crochets, l'autre non ; l'une respectait les guillemets,
l'autre les traversait ; l'une tolérait une fermante orpheline. Ces
différences sont réelles et restent — mais elles se déclarent désormais par
un nom au lieu de se réécrire à chaque fois.

### Documentation

**Le chapitre du langage algorithmique rattrape le moteur.** Il datait de la
2.3 et ignorait tout ce qui a suivi : il gagne les piles et les files, les objets et
les quatre piliers, les arbres et les graphes, `jonction` et `découpe`,
`élague` et `compacte`, et la règle des guillemets.

**Les doubles affectations du README sont corrigées.** Il montrait encore
`soit n = n + 1` dans une boucle, là où `n = n + 1` suffit : `soit` déclare une
fois, l'affectation suit. Deux écritures pour un seul effet, c'est un doublon —
non un *shadowing*, puisque le nom est réaffecté et non ombré.

Les 154 blocs de code du README ont été exécutés : les seuls messages restants
viennent de catalogues de commandes montrées hors de tout document, où l'objet
n'est délibérément pas déclaré.

**Les liens du sommaire ne fonctionnaient pas** — ils renvoyaient tous en haut
du fichier. Les titres portent un émoji, que GitHub retire de l'ancre en
laissant un tiret : le vrai lien est `#-les-objets`, non `#les-objets`. Les
émojis à sélecteur de variante (`🛠️`) laissaient en outre un caractère
invisible qui décalait l'ancre une seconde fois. Les 34 liens internes sont
recalculés selon l'algorithme de GitHub, et aucun ne vise plus dans le vide.

**Les exemples d'algorithmique passent de quatre fichiers à trois**, alignés sur
la numérotation des autres séries — qui toutes commencent à 2. `algo1`
disparaît : son contenu relevait de l'école primaire, où docdg n'a pas d'objet.

| Fichier | Contenu |
|---|---|
| `algo2` | déclarer et initialiser, entrer et sortir, décider, répéter — y compris `sortir` et `continuer` |
| `algo3` | les conteneurs, les chaînes de caractères, et les fonctions |
| `algo4` | récursivité, algorithmes classiques, structures de données avancées |

`algo4` gagne les algorithmes que le programme demande et qu'aucun document ne
montrait : recherche linéaire, dichotomie, tri par insertion, tri par
sélection, fusion de deux listes triées.

Une observation en découle, qui vaut d'être écrite : **une saisie arrête le
document jusqu'à la réponse.** Placée au milieu d'`algo2`, elle masquait tout
ce qui suivait. Elle figure désormais en fin de document, et la règle est dite
dans le texte.

`exemples/algo4.txt` gagne une section « Recevoir et rendre un conteneur » et
perd son avertissement obsolète : la collision entre fonction mathématique et
fonction algorithmique était déjà refusée.

### Limite connue

Au **niveau du document**, la condition d'un `tant que` est évaluée avant que
les conteneurs n'existent : `tant que est vide(p) vaut faux` n'y voit pas `p`.
L'ordre du pipeline place ce déroulement avant la création des boîtes. Dans un
corps de fonction, où tout est déroulé au moment de l'appel, la même écriture
fonctionne — et c'est là qu'un algorithme s'écrit.

---

## 2.3 — la qualité du gris typographique

La composition du texte courant, au niveau de la ligne : les mots se coupent
en fin de ligne selon les motifs français, et aucune ligne ne reste seule au
bas ou au sommet d'une page.

**Le vocabulaire s'aligne sur celui des enseignants.** `liste` et `tableau`
nomment désormais le même type que `collection` ; `chaîne`, `chaîne de
caractères` et `texte` sont synonymes ; `renvoie` s'écrit aussi bien que
`retourne`. `inverse(v)` remplace `miroir(v)` : la syntaxe `inverse(...)` et la
prose « l'inverse de la matrice M » ne se confondent pas, la parenthèse les
sépare.

**`non`**, troisième connecteur logique enseigné avec `et` et `ou`, manquait.
Les conditions lisent aussi `vrai` et `faux` en toutes lettres, et un booléen
s'affiche ainsi : `contient(15 ; notes)` rend « vrai », non « 1 ».

**`insère`, `supprime`, `ajoute`** — à l'impératif, non `insertion` et
`suppression` : le nom aurait télescopé « tri par insertion », l'algorithme et
l'opération portant le même mot dans la même page. `ajoute(v ; x)` nomme enfin
l'opération la plus employée du programme, que la concaténation `v + {x}`
couvrait sans la dire.

**La préposition dit qui reçoit l'opération.** `dans notes insère(0 ; 20)` se
lit comme une phrase et nomme sans ambiguïté le conteneur, là où
`insère(notes ; 0 ; 20)` laissait deviner lequel des trois arguments le
désignait. Trois écritures coexistent, au choix de ce qui se lit le mieux : la
forme fonctionnelle, la forme prépositionnelle, et la forme directe
`notes contient(15)` pour les questions par oui ou non.

**La division euclidienne se dit comme au collège** : `quotient de 17 par 5`,
`reste de 17 par 5` — la phrase dit lequel des deux nombres est le dividende,
ce que `quotient(17 ; 5)` taisait. Les formes brèves restent admises, et la
tournure accepte un appel comme dividende : `quotient de longueur(notes) par 2`.

**`sortir`** arrête une boucle sans quitter ce qui l'entoure — c'est la
recherche qui cesse dès qu'elle a trouvé. Dans un corps de fonction comme dans
une boucle de document.

**La forme française `v contient(1)`.** Le conteneur peut précéder la
primitive et se glisse à sa place parmi les arguments : la phrase se lit à voix
haute. `contient(1 ; v)` reste admis.

**Le p-uplet.** Le programme de NSI le distingue explicitement de la liste :
longueur fixe, types pouvant différer. Il s'écrit `(entier ; entier)`, se
retourne, se délie et se lit par rang.

- `soit divise(a: entier ; b: entier): (entier ; entier) = (quotient de a par b ; reste de a par b)`
- `soit (q ; r) = divise(17 ; 5)` — la **déliaison** pose les deux noms d'un
  coup, au niveau document comme dans un corps de fonction ;
- `soit c: (entier ; texte) = (3 ; "trois")` — les membres peuvent différer, et
  `c[1]` rend le second ;
- l'arité est vérifiée : délier trois noms d'un couple est une faute dite, et
  `(1 ; 2)` n'est pas acceptable là où une liste est attendue.

Les extrema d'une série se rendent enfin en un seul parcours.

**`aléatoire(a ; b)`** tire un entier entre les deux bornes, comprises — un
xorshift sans dépendance, semé par l'horloge. Deux compilations donnent deux
tirages : c'est ce qu'une simulation attend. Sans lui, ni Monte-Carlo, ni
marche aléatoire, ni étude de fréquences.

**`continuer`** arrête le tour en cours ; la boucle poursuit. Le pendant de
`sortir`, dans un corps de fonction comme dans une boucle de document.

**Une déclaration typée accepte un appel**, y compris pour un type scalaire :
`soit x: entier = f(3)`.

**L'affectation d'une valeur composée devient cohérente.** Cinq écritures
naturelles échouaient, chacune pour une raison différente :

- `pour k dans d { … d[k] … }` — une **clé de dictionnaire peut être une
  variable**, non plus seulement un mot écrit à la main ;
- `soit f(): un dictionnaire … = {a: 1}` — l'accolade délimite ici un
  **littéral**, non un bloc d'instructions ; la distinction se fait sur la
  présence d'un mot du langage à l'intérieur ;
- `soit p: une liste d'entiers = empile({} ; 1)` — le membre droit d'une
  déclaration typée **n'est plus forcément un littéral** : un appel, une
  primitive, une concaténation conviennent ;
- `soit p = empile({} ; 1)` — une valeur composée **se pose sans que son type
  soit écrit** ; docdg reconnaît ce qu'elle est ;
- `soit v = tri(v)` — un conteneur **se réaffecte**. L'accumulateur
  `soit S = S + {k}` reste prioritaire et intact.

La pile et la file s'écrivent désormais entièrement dans le langage, en sept
lignes et sans une ligne de moteur.

**Le chapitre « texte » du programme s'écrit enfin.** La déclaration d'une
chaîne ne menait nulle part : sept manques la rendaient inerte. Ils sont
comblés ensemble.

- `m[0]` lit **une lettre**, qui est une chaîne d'un seul caractère — docdg
  n'introduit pas de type distinct, pas plus que Python. L'indice hors bornes
  compte en lettres.
- `pour c dans m` parcourt les lettres, dans un document comme dans un corps
  de fonction.
- `majuscule`, `minuscule`, `sans accents` — `majuscule("été")` rend `ÉTÉ`,
  non `ETE` : la typographie française accentue les capitales.
- `code("B")` et `caractère(97)` font le va-et-vient entre une lettre et son
  rang Unicode.
- `texte(42)` et `nombre("1,5")` convertissent, explicitement.
- Un littéral entre guillemets est lu comme chaîne partout, y compris en
  argument : `m contient("jour")`.
- Une **lecture indexée sert d'argument** : `code(m[0])`, `ajoute(r ; a[i])`.
  La concaténation `r + {a[i]}` le permettait déjà, l'appel nommé non.
- **Deux textes se comparent** dans une condition, avec la collation
  française : `si u vaut inverse(u)` reconnaît un palindrome.

César chiffre, le palindrome se reconnaît, le comptage de lettres tourne.

**La chaîne de caractères devient une valeur ordinaire** :
`soit m: chaîne de caractères = "bonjour"`. Un type scalaire n'a pas de
littéral entre accolades — sa valeur tient sur la fin de la ligne. Les
guillemets, droits ou français, délimitent sans appartenir à la valeur, ce qui
permet d'y garder espaces et signes du langage. La chaîne se mesure
(`longueur`), se découpe (`m[0 à 7]`), se joint (`+`) et se passe en argument.

**`quotient(a ; b)`** — la division euclidienne du collège. Le reste s'obtenait
déjà par `%` ; le quotient entier manquait, alors qu'il donne l'indice du
milieu dans une recherche dichotomique.

### Corrigé

**Une primitive restait inerte dans la condition d'un `si` de document.**
`expand_conditions` n'avait accès ni aux conteneurs ni aux fonctions : la même
écriture fonctionnait dans un corps de fonction et pas dans le document. Les
conditions de tour d'une boucle sont désormais résolues au déroulement, ce qui
rend au passage visible un `sortir` niché dans un `si`.

**Un point-virgule d'appel coupait une cellule de tableau en deux.** `split_top`
comptait les accolades et les crochets, mais pas les parenthèses : la rangée
`[contient(2 ; v) ; #{contient(2 ; v)}]` produisait quatre cellules au lieu de
deux. Les parenthèses comptent maintenant au même titre.

- **Une couleur de fond d'entête déteignait sur tout le tableau.** Dans
  `des entêtes en blanc sur fond bleu marine`, le mot `fond` était capté par
  la lecture du fond général : toutes les cellules prenaient la couleur
  réservée aux entêtes. Un `fond` introduit par `sur` ne qualifie plus que ce
  qui le précède — même correction pour le titre d'un cadre.
- La documentation du tableau annonçait une option `bordures` qui n'existe
  plus : un tableau porte toujours ses bordures. La ligne est retirée.
- **Un tableau ou un cadre sécable était entraîné à la page suivante par le
  bloc qui le suivait.** La règle qui garde un titre de section solidaire du
  texte qu'il annonce reconnaissait sa classe `sec` par sous-chaîne, et
  `secable` la contient : tout bloc sécable était traité en titre et emporté
  avec son voisin. La classe est désormais comparée en entier. Selon
  l'interligne et la position du bloc dans la page, le tableau se coupait
  correctement ou basculait en laissant un grand blanc — d'où une apparence
  d'imprévisibilité.

### Ajouté

**La césure française.**
- `césure: oui` (défaut) coupe les mots longs selon les motifs TeX du français
  — deux lettres au moins avant la coupure, trois après. Les blancs
  interlettres d'un texte justifié s'en trouvent régularisés.
- Les traits d'union conditionnels sont posés par le moteur, non par le
  navigateur : l'aperçu et le PDF coupent aux mêmes endroits, ce que les
  dictionnaires internes des navigateurs ne garantissaient pas.
- Sont épargnés : les mots de moins de six lettres, les mots composés, les
  sigles et les noms propres, les titres de section, les zones mathématiques,
  le code et les figures.
- `césure: non` rend le texte tel quel.

**Les veuves et les orphelines.**
- `orphelines: 2` et `veuves: 2` (défauts) fixent le nombre de lignes
  minimales laissées en bas de page et reportées en haut de la suivante.
- Un paragraphe qui déborde est désormais coupé à une frontière de ligne au
  lieu d'être reporté en bloc : moins de blancs en bas de page. La coupure ne
  tombe jamais au milieu d'un mot ni dans une formule, et la ligne qui la
  précède reste justifiée.
- Quand la contrainte est intenable, le paragraphe passe entier à la page
  suivante — le comportement d'avant.

**Les blocs sécables.**
- `avec coupure de page` était documenté depuis la 2.2 pour les cadres, mais
  n'était implémenté nulle part : la mention était sans effet. La locution
  cède la place à l'adjectif `sécable`, antonyme du `insécable` que la
  typographie française emploie depuis toujours, et la chose fonctionne
  désormais — pour les cadres comme pour les tableaux.
- Un cadre sécable se poursuit d'une page à l'autre ; son filet devient tireté
  au point de rupture, et son titre ne se répète pas.
- Un tableau sécable se poursuit de même, et sa **rangée d'entête se répète en
  tête de chaque fragment** — la règle académique du tableau long, sans rien à
  écrire pour l'obtenir. Il se coupe dès qu'une rangée tient dans la place
  restante : les règles de veuve et d'orpheline, faites pour la prose, ne
  s'appliquent pas aux rangées d'un tableau, sous peine de le voir refuser la
  coupure et laisser un grand blanc au bas de la page.
- Le défaut reste l'insécabilité : un théorème, un énoncé bref, un petit
  tableau restent d'un seul tenant. Les grilles ne se coupent pas encore.

**Un exemple pour le voir.** `exemples/publication3.txt` — « De la composition
du texte courant » — met les cinq réglages à l'épreuve : paragraphes assez
longs pour produire veuves et orphelines, mots interminables pour la césure,
formules qu'elle doit épargner, un cadre sécable de trois paragraphes, un
cadre bref insécable en contrepoint, et un tableau de vingt et une rangées.
Sa dernière section indique comment désactiver chaque réglage pour juger par
comparaison directe.

**Au formulaire.** Les trois réglages figurent au panneau **Paramétrer**, et
s'inscrivent dans le bloc `document { }` quand ils diffèrent du défaut.

**L'alinéa partout.** Une tabulation en tête de ligne creusait un alinéa au fil
du texte, mais était ignorée à l'intérieur d'un cadre, d'une grille ou d'un
bloc : la prose d'un cadre s'y composait au fer, sans retrait. L'indentation
est désormais rendue dans tous ces contextes, avec la même règle qu'ailleurs.
Dans un tableau, la tabulation reste le séparateur de colonnes.

**Trois défauts corrigés dans les conditionnelles et les boucles.**
- `si <condition> { ... }` ignorait toute variable posée par un `soit` plus
  haut dans le même segment : la condition était évaluée avant que la
  variable n'existe encore pour le moteur de rendu, silencieusement, sans
  erreur — le bloc conditionnel disparaissait purement et simplement, y
  compris sa branche `sinon`. Corrigé.
- Un `si` niché à l'intérieur d'un `pour` était évalué **avant** que la
  boucle ne soit déroulée, donc avant que sa variable n'existe : le bloc
  entier disparaissait, quelle que soit la condition. Les boucles se
  déplient désormais avant que leurs conditions internes ne soient jugées,
  et chaque tour n'ajoute son contenu au résultat que si sa condition le
  permet — un tour peut désormais ne rien produire, plutôt que de produire
  systématiquement, sans que rien n'existât auparavant pour l'exprimer.
- `appartient à [a;b]` et sa négation `n'appartient pas à [a;b]` n'étaient
  reconnus par aucune condition — seuls les comparateurs numériques
  existaient. Les deux s'ajoutent aux comparateurs existants, cohérents
  avec l'écriture déjà en usage pour les intervalles ailleurs dans le
  langage.

Ensemble, ces trois corrections rendent possible ce qui ne l'était pas :

```
<Dresse>un tableau [mc, mc, mc, mc]{
	Rang	Terme	Écart	Rapport(croissant)
	pour n de 0 à 20 {
		si (n appartient à [16;20]) {
			[#n ; #{2^(n+1) - 1} ; #{2^n} ; 2,000]
		}
	}
}
```

**Les conditions composées.** `et` et `ou` combinent désormais deux conditions
dans le même `si`, avec les parenthèses pour en préciser l'ordre — absents
jusqu'ici, seul un comparateur isolé était reconnu.

**L'accumulateur.** Une variable posée avant une boucle et réaffectée à
l'intérieur (`soit somme = somme + k`) conservait sa valeur initiale : les
affectations n'étaient lues qu'avant le déroulement des boucles, si bien que
les tours successifs restaient invisibles. Elles sont désormais relues une
fois les boucles déroulées, dans l'ordre, ce qui fait de la réaffectation un
véritable accumulateur — sommes, produits, compteurs sous condition. Un
compteur vaut sa valeur finale partout où il est relu, y compris à
l'intérieur de la boucle : afficher un total *en cours* de calcul demanderait
autre chose.

**Deux exemples pédagogiques.** `exemples/algo1.txt` introduit variables,
sortie, boucle simple, condition simple, boucle à pas, et `<Saisis>` — chaque
structure seule, sans en mêler deux. `exemples/algo2.txt` les combine : boucle
filtrante, recherche de diviseurs, filtrage par intervalle, opérateur
ternaire, conditions composées. `algo3`, à venir, portera sur les containers
— vecteurs, matrices, dictionnaires clé:valeur.

**Les conteneurs.** Trois structures de données entrent dans le langage, avec
un typage toujours écrit et le point-virgule pour unique séparateur d'éléments :

```
soit notes: une collection de décimaux = {12,5 ; 15 ; 9,5}
soit trajets: un dictionnaire de textes et d'entiers = {Marche: 5 ; Bus: 3}
soit A: une matrice 2×2 d'entiers = {{1 ; 2} ; {3 ; 4}}
```

La matrice s'écrit aussi en bloc — tabulations entre colonnes, retours à la
ligne entre rangées — et, déclarée avec `{}`, naît remplie de zéros. La
lecture s'indexe à partir de 0 (`#notes[1]`, `#A[0 ; 1]`, `#trajets[Marche]`),
l'écriture se fait sans `soit` (`notes[2] = 14`, `T[i ; j] = v`), la
croissance passe par la concaténation de singletons (`soit S = S + {k}`), la
boucle `dans` parcourt une collection, les rangées d'une matrice, ou les clés
d'un dictionnaire. Toute faute — indice hors bornes, clé absente, valeur d'un
autre type que celui déclaré, rangées inégales — est dite en rouge dans le
document, avec la ligne fautive, au lieu d'un silence. Un tableau à plusieurs
dimensions se compose : `une collection de collections d'entiers`, bâtie par
une rangée locale à la boucle et un conteneur extérieur qui accumule.
`exemples/algo3.txt` déroule l'ensemble.

**Une assertion périmée, corrigée.** Le test `l_exemple_basique3` réclamait une
phrase — « Émile est noté célibataire » — que `exemples/basique3.txt` ne produit
plus depuis que sa formulation a changé en 2.2 ; le document dit « Il est
célibataire ». L'assertion suivait le document, pas l'inverse. La suite est
désormais entièrement verte.

**Une course sur les réglages de page, corrigée.** `tabulation`, `hauteur` et
`précision` vivaient dans un verrou global au processus. Les tests s'exécutant
en parallèle dans un même processus, un document posant `précision: 3` pouvait
se faire écraser par un autre remettant les défauts — d'où des échecs qui
n'apparaissaient que sur les machines à nombreux cœurs. Les réglages sont
désormais propres à chaque fil, et explicitement transmis aux fils de rendu
parallèle. Reproduit puis vérifié à seize fils.

**Une balise porte un verbe, jamais un objet.** Les formes `<matrice>{…}` et
`<système>{…}`, documentées comme équivalentes à `<Soit>la matrice M {…}`,
contredisaient la règle : elles disparaissent du manuel. `<Soit>` déclare, ici
comme partout.

**Le point-virgule devient le séparateur universel des données.** Le
spécificateur de colonnes s'écrit désormais `[mc ; mg ; md]`, comme tout le
reste. La virgule y étant libérée, les **formes longues** deviennent
possibles — `[au centre ; en bas, à droite ; en haut, à gauche]` — là où seuls
les codes à deux lettres étaient lisibles. Les seize fichiers du corpus sont
migrés ; l'ancienne forme à virgules reste acceptée.

**Trois nettoyages avant diffusion.**
- `avec des bordures` disparaît du manuel et des exemples : l'option était
  inerte depuis longtemps, un tableau porte toujours ses bordures. Sept
  fichiers étaient concernés.
- Une boucle parcourt désormais une liste littérale entre **accolades**,
  séparée par des points-virgules — `pour f dans {chat.png ; chien.png}` —
  conformément à la règle qui réserve l'accolade aux données. Les crochets
  restent aux intervalles, `pour x dans [-2 ; 2]`, sans ambiguïté possible.
  L'ancienne forme à virgules continue de fonctionner.
- Un nom ne peut plus désigner à la fois une fonction mathématique et une
  fonction algorithmique : la collision est refusée avec un message explicite,
  au lieu de rendre l'appel indécidable.

**Les fonctions algorithmiques.** Un calcul se nomme, se type et se réutilise,
distinct de la fonction mathématique `<Soit>une fonction f(x)` que les chevrons
continuent de marquer :

```
soit addition(a: entier ; b: entier): entier = a + b
soit hypoténuse(a: réel ; b: réel): réel = {
	soit carrés = a*a + b*b
	retourne racine(carrés)
}
soit factorielle(n: entier): entier = si n vaut 0 { 1 } sinon { n * factorielle(n - 1) }
```

Le corps tient en une expression, ou entre accolades ; dès qu'il compte
plusieurs instructions, la valeur produite se désigne par `retourne` — rien
n'est implicite. La récursivité est admise, bornée à deux cents appels
imbriqués. L'appel s'écrit `#{addition(3 ; 7)}` et vaut partout où un calcul
est admis : dans le texte, dans une rangée de tableau, dans la condition d'une
boucle, dans un littéral de conteneur (`soit S = S + {carré(k)}`). Arité,
type d'argument, type de retour et corps sans `retourne` sont signalés
explicitement. `exemples/algo4.txt` déroule l'ensemble.

Les paramètres et valeurs de retour sont des nombres : ni collection ni texte
en argument pour l'instant. Rien n'interdit encore le même nom pour une
fonction mathématique et une fonction algorithmique.

**Quatre types de nombres, calqués sur les ensembles.** `entier` pour
\(\mathbb{Z}\), `décimal` pour \(\mathbb{D}\) — développement décimal fini —
`réel` pour \(\mathbb{R}\), et `complexe`, couple de deux réels noté
`(a ; b)` comme un point du plan. L'inclusion se vérifie dans les deux sens :
`{1/3}` est refusé à une collection de décimaux, accepté à une collection de
réels. Les matrices acceptent les trois premiers ; une matrice de complexes
est refusée explicitement plutôt que silencieusement mal calculée.

**Les boucles conditionnelles réparées par ricochet.** `tant que … faire {…}`
et `faire {…} tant que …`, documentés depuis longtemps, ne bouclaient pas :
faute d'accumulateur, la variable de contrôle ne pouvait pas avancer et le
texte ressortait tel quel. La correction de l'accumulateur les rétablit
toutes deux, avec leur sémantique propre — `tant que` peut ne faire aucun
tour, `faire` en fait toujours au moins un. `exemples/algo2.txt` les
illustre.

**Ce qui ne se coupe pas encore.** Les grilles restent des blocs entiers, et
les notes de bas de page appelées depuis un bloc scindé se regroupent au pied
de la page où le bloc a commencé.

## 2.2 — écrire un article, une thèse

Le chantier « publication » en entier : la page et les polices se règlent au
détail, le document long se structure (chapitres, page de titre), les renvois
et la bibliographie s'écrivent en français — et l'interface se réorganise
pour la rédaction au long cours.

### Ajouté

**Le bloc d'en-tête au complet.**
- `police:` accepte enfin un nom de police (Georgia, Latin Modern Roman…) et
  ne se confond plus avec `taille:`, qui règle seule le corps en points —
  l'ancienne clé `police: 11` réglait la taille, c'est corrigé.
- `math:` déclare la police mathématique ; la clé est stockée et exposée,
  mais non appliquée au rendu : KaTeX compose avec ses propres polices, dont
  les métriques sont indissociables de son algorithme.
- `tabulation:` règle la largeur d'une tabulation en millimètres (10 par
  défaut) ; les espaces valent le quart d'une tabulation, partout — en tête
  de ligne comme en milieu de paragraphe.
- `hauteur:` règle la hauteur d'un saut de ligne en millimètres (5 par
  défaut), y compris entre segments.
- `décalage:` règle en pourcentage l'amplitude des exposants et indices du
  texte (100 par défaut), via la variable CSS `--decalage`.
- `précision:` arrondit toutes les valeurs numériques affichées au nombre de
  décimales demandé ; `-1` (défaut) conserve le comportement historique. Les
  formes exactes (fractions, racines) ne sont pas concernées.

**La police locale, convention texecole.**
- Une suite de mots en MAJUSCULES dans une balise de style est un nom de
  police : `<TIMES NEW ROMAN gras>{très}` compose « très » en Times gras.
  La convention vaut en début de ligne, en milieu de paragraphe, et dans les
  styles nommés (`soit manuscrit = <SCHOLA italique>`).

**Le bloc `document{}` unifié.**
- Un seul bloc d'en-tête porte désormais tout : métadonnées (`titre`,
  `auteur`, `institution`, `date`) et réglages (`marges`, `police`,
  `taille`, `interligne`, `précision`…). L'ancien couple `page{}` +
  `document{}` reste accepté, dans les deux ordres — `document{}` est la
  forme recommandée.
- Le panneau Paramétrer expose toutes les options de la classe document :
  métadonnées vides par défaut, réglages avec leurs valeurs. À
  l'application, il réécrit un bloc `document{}` unique, n'inscrivant que
  les champs remplis et les réglages non par défaut.

**La structure du document long.**
- Le mot de style `chapitre` ouvre un niveau au-dessus de `section` :
  `soit h0 = <bleu nuit gras chapitre num>`. Les chapitres se numérotent
  seuls, remettent les sections à zéro et les préfixent (`2.1` = chapitre 2,
  section 1). Sans chapitre, la numérotation reste exactement celle d'avant.
- Le bloc `document{}` déclare les métadonnées — `titre`, `auteur`,
  `institution`, `date` — dans le bloc d'en-tête ; la balise `<page de titre>`
  les compose en première page, centrées, puis saute la page. Les clés
  absentes sont simplement omises ; changer une métadonnée invalide le cache.
- La table des matières connaît le niveau chapitre (entrées en gras).

**Les renvois croisés.**
- `<étiquette>{modele}` posée dans un titre ou un paragraphe mémorise la
  section courante ; `<renvoi>{modele}` s'y remplace par son numéro, lié à
  l'ancre (`1.1` sous chapitre, comme la table des matières). L'ordre est
  libre : un renvoi peut précéder son étiquette. Un renvoi sans étiquette
  s'affiche `??` en rouge, à la façon de LaTeX.
- La résolution se fait en fin de composition par marqueurs privés, comme la
  table des matières et les notes — elle traverse donc le cache incrémental
  et le rendu parallèle sans état partagé.

**La bibliographie.**
- `<Dresse>une bibliographie { [clé] Texte libre de l'entrée. }` : une entrée
  par ligne, numérotée dans l'ordre, ancrée par sa clé.
- `<cite>{clé}` s'y remplace par `[n]` lié ; `<cite>{a, b}` groupe plusieurs
  clés. La citation peut précéder la bibliographie — le cas normal, la
  bibliographie fermant le document. Clé inconnue : `??`.

**L'exemple `publication2.txt`** : page de titre, table des matières, deux chapitres
sectionnés — le squelette d'un mémoire.

**L'exemple `publication1.txt`** : un article court — titre, auteur, résumé,
mots-clés en petites capitales, notes de bas de page, tableau de mesures,
propagation d'incertitudes — qui exerce chaque réglage.

**L'interface en panneaux.**
- L'éditeur et l'aperçu sont désormais côte à côte — le code à gauche
  (40 %), la page à droite — parce qu'une page A4 portrait exige la hauteur
  de l'écran, pas sa largeur. Le bouton ⇄ bascule vers la disposition
  verticale d'origine (aperçu en haut), utile en orientation paysage.
- La poignée entre les deux volets se glisse librement (de 15 à 85 %) ;
  un double-clic rétablit le partage par défaut.
- Les boutons Code et Aperçu masquent l'un ou l'autre volet — jamais les
  deux : masquer le dernier réaffiche l'autre.
- La disposition, les proportions, les volets visibles et le zoom se
  retrouvent d'une session à l'autre (`~/.config/docdg/interface.json`,
  `%APPDATA%\docdg` sous Windows) ; un fichier corrompu retombe sans bruit
  sur les défauts.
- L'éditeur occupe enfin toute la largeur de son volet, structurellement :
  le dimensionnement est passé du `width: 100%` fragile au modèle flex.
- Les formules hors-texte ne portent plus d'ascenseur horizontal à l'écran :
  elles débordent visiblement, comme sur le papier — l'aperçu dit la vérité
  du PDF, et la hauteur ajoutée par l'ascenseur ne faussait plus la
  pagination. Aperçu masqué, le volet code occupe désormais tout l'espace,
  symétriquement à l'aperçu.

**Le vocabulaire s'aligne sur celui des enseignants.** `liste` et `tableau`
nomment désormais le même type que `collection` ; `chaîne`, `chaîne de
caractères` et `texte` sont synonymes ; `renvoie` s'écrit aussi bien que
`retourne`. `inverse(v)` remplace `miroir(v)` : la syntaxe `inverse(...)` et la
prose « l'inverse de la matrice M » ne se confondent pas, la parenthèse les
sépare.

**`non`**, troisième connecteur logique enseigné avec `et` et `ou`, manquait.
Les conditions lisent aussi `vrai` et `faux` en toutes lettres, et un booléen
s'affiche ainsi : `contient(15 ; notes)` rend « vrai », non « 1 ».

**`insère`, `supprime`, `ajoute`** — à l'impératif, non `insertion` et
`suppression` : le nom aurait télescopé « tri par insertion », l'algorithme et
l'opération portant le même mot dans la même page. `ajoute(v ; x)` nomme enfin
l'opération la plus employée du programme, que la concaténation `v + {x}`
couvrait sans la dire.

**La préposition dit qui reçoit l'opération.** `dans notes insère(0 ; 20)` se
lit comme une phrase et nomme sans ambiguïté le conteneur, là où
`insère(notes ; 0 ; 20)` laissait deviner lequel des trois arguments le
désignait. Trois écritures coexistent, au choix de ce qui se lit le mieux : la
forme fonctionnelle, la forme prépositionnelle, et la forme directe
`notes contient(15)` pour les questions par oui ou non.

**La division euclidienne se dit comme au collège** : `quotient de 17 par 5`,
`reste de 17 par 5` — la phrase dit lequel des deux nombres est le dividende,
ce que `quotient(17 ; 5)` taisait. Les formes brèves restent admises, et la
tournure accepte un appel comme dividende : `quotient de longueur(notes) par 2`.

**`sortir`** arrête une boucle sans quitter ce qui l'entoure — c'est la
recherche qui cesse dès qu'elle a trouvé. Dans un corps de fonction comme dans
une boucle de document.

**La forme française `v contient(1)`.** Le conteneur peut précéder la
primitive et se glisse à sa place parmi les arguments : la phrase se lit à voix
haute. `contient(1 ; v)` reste admis.

**Le p-uplet.** Le programme de NSI le distingue explicitement de la liste :
longueur fixe, types pouvant différer. Il s'écrit `(entier ; entier)`, se
retourne, se délie et se lit par rang.

- `soit divise(a: entier ; b: entier): (entier ; entier) = (quotient de a par b ; reste de a par b)`
- `soit (q ; r) = divise(17 ; 5)` — la **déliaison** pose les deux noms d'un
  coup, au niveau document comme dans un corps de fonction ;
- `soit c: (entier ; texte) = (3 ; "trois")` — les membres peuvent différer, et
  `c[1]` rend le second ;
- l'arité est vérifiée : délier trois noms d'un couple est une faute dite, et
  `(1 ; 2)` n'est pas acceptable là où une liste est attendue.

Les extrema d'une série se rendent enfin en un seul parcours.

**`aléatoire(a ; b)`** tire un entier entre les deux bornes, comprises — un
xorshift sans dépendance, semé par l'horloge. Deux compilations donnent deux
tirages : c'est ce qu'une simulation attend. Sans lui, ni Monte-Carlo, ni
marche aléatoire, ni étude de fréquences.

**`continuer`** arrête le tour en cours ; la boucle poursuit. Le pendant de
`sortir`, dans un corps de fonction comme dans une boucle de document.

**Une déclaration typée accepte un appel**, y compris pour un type scalaire :
`soit x: entier = f(3)`.

**L'affectation d'une valeur composée devient cohérente.** Cinq écritures
naturelles échouaient, chacune pour une raison différente :

- `pour k dans d { … d[k] … }` — une **clé de dictionnaire peut être une
  variable**, non plus seulement un mot écrit à la main ;
- `soit f(): un dictionnaire … = {a: 1}` — l'accolade délimite ici un
  **littéral**, non un bloc d'instructions ; la distinction se fait sur la
  présence d'un mot du langage à l'intérieur ;
- `soit p: une liste d'entiers = empile({} ; 1)` — le membre droit d'une
  déclaration typée **n'est plus forcément un littéral** : un appel, une
  primitive, une concaténation conviennent ;
- `soit p = empile({} ; 1)` — une valeur composée **se pose sans que son type
  soit écrit** ; docdg reconnaît ce qu'elle est ;
- `soit v = tri(v)` — un conteneur **se réaffecte**. L'accumulateur
  `soit S = S + {k}` reste prioritaire et intact.

La pile et la file s'écrivent désormais entièrement dans le langage, en sept
lignes et sans une ligne de moteur.

**Le chapitre « texte » du programme s'écrit enfin.** La déclaration d'une
chaîne ne menait nulle part : sept manques la rendaient inerte. Ils sont
comblés ensemble.

- `m[0]` lit **une lettre**, qui est une chaîne d'un seul caractère — docdg
  n'introduit pas de type distinct, pas plus que Python. L'indice hors bornes
  compte en lettres.
- `pour c dans m` parcourt les lettres, dans un document comme dans un corps
  de fonction.
- `majuscule`, `minuscule`, `sans accents` — `majuscule("été")` rend `ÉTÉ`,
  non `ETE` : la typographie française accentue les capitales.
- `code("B")` et `caractère(97)` font le va-et-vient entre une lettre et son
  rang Unicode.
- `texte(42)` et `nombre("1,5")` convertissent, explicitement.
- Un littéral entre guillemets est lu comme chaîne partout, y compris en
  argument : `m contient("jour")`.
- Une **lecture indexée sert d'argument** : `code(m[0])`, `ajoute(r ; a[i])`.
  La concaténation `r + {a[i]}` le permettait déjà, l'appel nommé non.
- **Deux textes se comparent** dans une condition, avec la collation
  française : `si u vaut inverse(u)` reconnaît un palindrome.

César chiffre, le palindrome se reconnaît, le comptage de lettres tourne.

**La chaîne de caractères devient une valeur ordinaire** :
`soit m: chaîne de caractères = "bonjour"`. Un type scalaire n'a pas de
littéral entre accolades — sa valeur tient sur la fin de la ligne. Les
guillemets, droits ou français, délimitent sans appartenir à la valeur, ce qui
permet d'y garder espaces et signes du langage. La chaîne se mesure
(`longueur`), se découpe (`m[0 à 7]`), se joint (`+`) et se passe en argument.

**`quotient(a ; b)`** — la division euclidienne du collège. Le reste s'obtenait
déjà par `%` ; le quotient entier manquait, alors qu'il donne l'indice du
milieu dans une recherche dichotomique.

### Corrigé

**Une primitive restait inerte dans la condition d'un `si` de document.**
`expand_conditions` n'avait accès ni aux conteneurs ni aux fonctions : la même
écriture fonctionnait dans un corps de fonction et pas dans le document. Les
conditions de tour d'une boucle sont désormais résolues au déroulement, ce qui
rend au passage visible un `sortir` niché dans un `si`.

**Un point-virgule d'appel coupait une cellule de tableau en deux.** `split_top`
comptait les accolades et les crochets, mais pas les parenthèses : la rangée
`[contient(2 ; v) ; #{contient(2 ; v)}]` produisait quatre cellules au lieu de
deux. Les parenthèses comptent maintenant au même titre.

- **La fenêtre ne se refermait plus après un clic dans la table des matières
  ou sur un renvoi.** La webview est créée par `with_html`, sans URL de base :
  suivre une ancre `#…` y déclenchait une navigation qui détruisait le
  contexte JavaScript, donc `window.demandeFermeture()` n'existait plus et
  `CloseRequested` restait sans effet — il fallait passer par le Dock. Les
  ancres sont désormais interceptées et remplacées par un défilement doux
  vers la cible. Par sécurité, la fenêtre se ferme aussi d'office si le
  script ne répond plus, ou à la seconde demande de fermeture en deux
  secondes : un document ne doit jamais devenir inquittable.
- Le panneau de réglages de l'application préserve désormais les clés qu'il
  n'édite pas au lieu de les effacer en réécrivant le bloc d'en-tête ; il
  expose maintenant toutes les options de la classe document.
- Les virgules décimales des masses molaires et conversions s'espacent
  correctement en KaTeX (`18{,}0` et non `18,0`).
- **Les limites, les zéros de dérivée et les branches sont mémoïsés.** Une
  fonction étudiée puis tracée faisait recalculer à SymPy les mêmes limites
  aux bords et les mêmes zéros de dérivée à chaque commande. Les treize
  sites d'appel à `limit` passent désormais par une aide mémoïsée, comme
  `zeros_dans` et `branche` — un appel répété coûte 0,01 ms au lieu de 89.
- `simplify` ne peut plus rendre une expression plus lourde qu'elle ne
  l'était : son résultat n'est retenu que s'il réduit le nombre
  d'opérations, et le ratio d'expansion est borné. Le pire cas devient
  prévisible.
- Diagnostic : `DOCDG_CAPTURE=/chemin/journal.jsonl` enregistre les requêtes
  envoyées à SymPy, en complément de `DOCDG_TRACE=1` qui les chronomètre.
- **Le ternaire ne confond plus une branche de texte avec une variable.**
  Dans `soit statut = si marié { marié } sinon { célibataire }`, le mot
  `marié` de la branche portait le nom de la saisie booléenne : il était
  évalué comme elle, et `statut` valait `1` au lieu de `marié`. Les deux
  branches s'accordent désormais en nature — si l'une est du texte, le
  résultat est du texte ; deux branches numériques restent numériques, et
  un ternaire sans `sinon` garde son comportement.

## 2.1 — la physique-chimie entre au programme

Du lycée à la licence, les commandes que réclamait une fiche de sciences
physiques : la chimie se calcule en Rust pur, sans détour par SymPy.

### Ajouté

**La chimie (module natif, aucun appel Python).**
- `<Équilibre>l'équation C3H8 + O2 -> CO2 + H2O` : équilibrage par pivot de
  Gauss exact sur entiers ; conservation des éléments et de la charge, donc
  les demi-équations redox ioniques passent (`Fe^2+ + MnO4^- + H^+ -> ...`).
  Flèche `->` ou `→`, signe `=` accepté.
- `<Calcule>la masse molaire de Fe2(SO4)3` : détail terme à terme puis total,
  table périodique embarquée (noms français, H à Pu), parenthèses imbriquées.
- `<Dresse>un tableau d'avancement pour ... avec n(CH4) = 0,5 et n(O2) = 0,8` :
  l'équation est équilibrée d'office ; états initial, en cours, final,
  réactif limitant et avancement maximal. Sans quantités, le tableau reste
  littéral en `n_0 - ax`.

**La physique.**
- `<Convertis>` étendu aux grandeurs physiques : temps, vitesses, pressions,
  énergies (dont l'électronvolt), puissances, fréquences, et températures
  affines `°C` ↔ `K`. Les conversions scolaires existantes sont inchangées.
- `<Donne>la valeur de la constante de Planck` : dix-sept constantes
  fondamentales (CODATA) en notation scientifique française.
- `<Propage>l'incertitude sur R = U/I avec u(U) = 0,05, u(I) = 0,002, ...` :
  formule des dérivées partielles puis application numérique si les valeurs
  sont fournies ; symbolique seule sinon.

**Les opérateurs de champs (licence).**
- Divergence et rotationnel d'un champ inline :
  `<Calcule>la divergence du champ F(x, y, z) = (x^2 ; xy ; z)`.
- Gradient et laplacien sur définition inline (`de f(x, y) = ...`) ; la forme
  existante sur fonction déclarée (`le gradient de psi`) reste la référence.

**Le vocabulaire s'aligne sur celui des enseignants.** `liste` et `tableau`
nomment désormais le même type que `collection` ; `chaîne`, `chaîne de
caractères` et `texte` sont synonymes ; `renvoie` s'écrit aussi bien que
`retourne`. `inverse(v)` remplace `miroir(v)` : la syntaxe `inverse(...)` et la
prose « l'inverse de la matrice M » ne se confondent pas, la parenthèse les
sépare.

**`non`**, troisième connecteur logique enseigné avec `et` et `ou`, manquait.
Les conditions lisent aussi `vrai` et `faux` en toutes lettres, et un booléen
s'affiche ainsi : `contient(15 ; notes)` rend « vrai », non « 1 ».

**`insère`, `supprime`, `ajoute`** — à l'impératif, non `insertion` et
`suppression` : le nom aurait télescopé « tri par insertion », l'algorithme et
l'opération portant le même mot dans la même page. `ajoute(v ; x)` nomme enfin
l'opération la plus employée du programme, que la concaténation `v + {x}`
couvrait sans la dire.

**La préposition dit qui reçoit l'opération.** `dans notes insère(0 ; 20)` se
lit comme une phrase et nomme sans ambiguïté le conteneur, là où
`insère(notes ; 0 ; 20)` laissait deviner lequel des trois arguments le
désignait. Trois écritures coexistent, au choix de ce qui se lit le mieux : la
forme fonctionnelle, la forme prépositionnelle, et la forme directe
`notes contient(15)` pour les questions par oui ou non.

**La division euclidienne se dit comme au collège** : `quotient de 17 par 5`,
`reste de 17 par 5` — la phrase dit lequel des deux nombres est le dividende,
ce que `quotient(17 ; 5)` taisait. Les formes brèves restent admises, et la
tournure accepte un appel comme dividende : `quotient de longueur(notes) par 2`.

**`sortir`** arrête une boucle sans quitter ce qui l'entoure — c'est la
recherche qui cesse dès qu'elle a trouvé. Dans un corps de fonction comme dans
une boucle de document.

**La forme française `v contient(1)`.** Le conteneur peut précéder la
primitive et se glisse à sa place parmi les arguments : la phrase se lit à voix
haute. `contient(1 ; v)` reste admis.

**Le p-uplet.** Le programme de NSI le distingue explicitement de la liste :
longueur fixe, types pouvant différer. Il s'écrit `(entier ; entier)`, se
retourne, se délie et se lit par rang.

- `soit divise(a: entier ; b: entier): (entier ; entier) = (quotient de a par b ; reste de a par b)`
- `soit (q ; r) = divise(17 ; 5)` — la **déliaison** pose les deux noms d'un
  coup, au niveau document comme dans un corps de fonction ;
- `soit c: (entier ; texte) = (3 ; "trois")` — les membres peuvent différer, et
  `c[1]` rend le second ;
- l'arité est vérifiée : délier trois noms d'un couple est une faute dite, et
  `(1 ; 2)` n'est pas acceptable là où une liste est attendue.

Les extrema d'une série se rendent enfin en un seul parcours.

**`aléatoire(a ; b)`** tire un entier entre les deux bornes, comprises — un
xorshift sans dépendance, semé par l'horloge. Deux compilations donnent deux
tirages : c'est ce qu'une simulation attend. Sans lui, ni Monte-Carlo, ni
marche aléatoire, ni étude de fréquences.

**`continuer`** arrête le tour en cours ; la boucle poursuit. Le pendant de
`sortir`, dans un corps de fonction comme dans une boucle de document.

**Une déclaration typée accepte un appel**, y compris pour un type scalaire :
`soit x: entier = f(3)`.

**L'affectation d'une valeur composée devient cohérente.** Cinq écritures
naturelles échouaient, chacune pour une raison différente :

- `pour k dans d { … d[k] … }` — une **clé de dictionnaire peut être une
  variable**, non plus seulement un mot écrit à la main ;
- `soit f(): un dictionnaire … = {a: 1}` — l'accolade délimite ici un
  **littéral**, non un bloc d'instructions ; la distinction se fait sur la
  présence d'un mot du langage à l'intérieur ;
- `soit p: une liste d'entiers = empile({} ; 1)` — le membre droit d'une
  déclaration typée **n'est plus forcément un littéral** : un appel, une
  primitive, une concaténation conviennent ;
- `soit p = empile({} ; 1)` — une valeur composée **se pose sans que son type
  soit écrit** ; docdg reconnaît ce qu'elle est ;
- `soit v = tri(v)` — un conteneur **se réaffecte**. L'accumulateur
  `soit S = S + {k}` reste prioritaire et intact.

La pile et la file s'écrivent désormais entièrement dans le langage, en sept
lignes et sans une ligne de moteur.

**Le chapitre « texte » du programme s'écrit enfin.** La déclaration d'une
chaîne ne menait nulle part : sept manques la rendaient inerte. Ils sont
comblés ensemble.

- `m[0]` lit **une lettre**, qui est une chaîne d'un seul caractère — docdg
  n'introduit pas de type distinct, pas plus que Python. L'indice hors bornes
  compte en lettres.
- `pour c dans m` parcourt les lettres, dans un document comme dans un corps
  de fonction.
- `majuscule`, `minuscule`, `sans accents` — `majuscule("été")` rend `ÉTÉ`,
  non `ETE` : la typographie française accentue les capitales.
- `code("B")` et `caractère(97)` font le va-et-vient entre une lettre et son
  rang Unicode.
- `texte(42)` et `nombre("1,5")` convertissent, explicitement.
- Un littéral entre guillemets est lu comme chaîne partout, y compris en
  argument : `m contient("jour")`.
- Une **lecture indexée sert d'argument** : `code(m[0])`, `ajoute(r ; a[i])`.
  La concaténation `r + {a[i]}` le permettait déjà, l'appel nommé non.
- **Deux textes se comparent** dans une condition, avec la collation
  française : `si u vaut inverse(u)` reconnaît un palindrome.

César chiffre, le palindrome se reconnaît, le comptage de lettres tourne.

**La chaîne de caractères devient une valeur ordinaire** :
`soit m: chaîne de caractères = "bonjour"`. Un type scalaire n'a pas de
littéral entre accolades — sa valeur tient sur la fin de la ligne. Les
guillemets, droits ou français, délimitent sans appartenir à la valeur, ce qui
permet d'y garder espaces et signes du langage. La chaîne se mesure
(`longueur`), se découpe (`m[0 à 7]`), se joint (`+`) et se passe en argument.

**`quotient(a ; b)`** — la division euclidienne du collège. Le reste s'obtenait
déjà par `%` ; le quotient entier manquait, alors qu'il donne l'indice du
milieu dans une recherche dichotomique.

### Corrigé

**Une primitive restait inerte dans la condition d'un `si` de document.**
`expand_conditions` n'avait accès ni aux conteneurs ni aux fonctions : la même
écriture fonctionnait dans un corps de fonction et pas dans le document. Les
conditions de tour d'une boucle sont désormais résolues au déroulement, ce qui
rend au passage visible un `sortir` niché dans un `si`.

**Un point-virgule d'appel coupait une cellule de tableau en deux.** `split_top`
comptait les accolades et les crochets, mais pas les parenthèses : la rangée
`[contient(2 ; v) ; #{contient(2 ; v)}]` produisait quatre cellules au lieu de
deux. Les parenthèses comptent maintenant au même titre.

- Le bassin SymPy se remplit entièrement dès `prechauffe()` au lieu d'un seul
  ouvrier ; un ouvrier abandonné est tué et moissonné (plus d'orphelin).
- Timeout de 20 s par requête dans l'ouvrier (`SIGALRM`, garde Windows) ;
  filtre des expressions étendu aux guillemets doubles et accents graves.
- Mémoïsation de `domaine_reel`, `ruptures_de` et `reduis` : les opérations
  qui étudient puis tracent la même fonction ne recalculent plus son domaine.
- Le rendu parallèle se déclenche dès 2 segments manquants au lieu de 4.
- Les chemins de la suite de tests suivent la nouvelle arborescence
  `exemples/` ; trace opt-in `DOCDG_TRACE=1` pour chronométrer les requêtes.

## 2.0 — le document devient vivant

Changement de nature plutôt que d'ampleur : docdg peut désormais interroger
son lecteur et se recomposer avec ses réponses, ce qu'aucun document figé
(LaTeX, Typst, PDF statique) ne permet.

### Ajouté

**Les saisies interactives.**
- La balise `<Saisis>` : cinq types (texte, entier, décimal à la virgule,
  booléen `vrai`/`faux`, caractère).
- Blocage typé : le document s'arrête à la première réponse manquante ou
  invalide ; rien de ce qui suit — texte, questions, calculs — ne s'affiche
  tant qu'elle n'est pas conforme. Message d'erreur temporaire sous le champ.
  Le champ de saisie prend automatiquement le focus.
- Cliquer sur une réponse déjà donnée (affichée en bleu) rouvre la question
  et redéroule le document à partir de là.
- Les réponses sont des valeurs comme les autres : elles s'affichent par
  `#nom`, entrent dans les calculs `#{...}`, et peuvent alimenter n'importe
  quelle description de balise (`<Trace>le solide cube, d'arête #a cm`).

**L'opérateur ternaire.**
- En bloc : `si <condition> { ... } sinon { ... }`.
- En ligne, comme valeur : `soit x = si <condition> { A } sinon { B }`,
  texte ou nombre.
- Condition nue (`si marié`, vraie si non nulle), `vaut vrai` / `vaut faux`,
  et tous les comparateurs existants (`vaut`, `moins de`, `plus de`,
  `au moins`, `au plus`, `différent de`).

**La géométrie dans l'espace**, portée par un nouveau moteur de projection
cavalière (fuyante 0,45/0,35) :
- sept solides en perspective cavalière, arêtes cachées en pointillés :
  cube, pavé droit, prisme, pyramide, cylindre, cône, sphère ;
- cinq patrons dépliés (cube, pavé, pyramide, cylindre, cône) ;
- le repère de l'espace (O ; x, y, z), gradué, avec points, segments,
  vecteurs et droites tracés, chemin de coordonnées en pointillés rouges ;
- les droites de l'espace comme objets (`<Soit>la droite d passant par ...
  et de vecteur directeur ...`), leur représentation paramétrique rédigée ;
- les positions relatives rédigées pas à pas (`<Étudie>la position
  relative ...`) : droites/droites, droite/plan, plans/plans — tous les cas
  (confondues, parallèles, sécantes, non coplanaires) conclus proprement.

**Les courbes et les coniques.**
- Courbes paramétrées et polaires (`<Trace>la courbe paramétrée ...`,
  `<Trace>la courbe polaire ...`), repère isotrope, couleurs du traceur.
- Coniques rédigées (`<Étudie>la conique d'équation ...`) : réduction
  complète (discriminant, rotation si terme croisé, centre, équation
  réduite), éléments caractéristiques (demi-axes, excentricité, foyers,
  asymptotes pour l'hyperbole, sommet/foyer/directrice pour la parabole),
  cas dégénérés reconnus, tracé avec centre et foyers marqués.

**Les fonctions de deux variables.**
- Surfaces `z = f(x, y)` en perspective cavalière, maillées et ombrées
  selon l'orientation des facettes.
- Lignes de niveau par *marching squares*, niveaux automatiques ou choisis.
- Extremums sous contrainte par multiplicateurs de Lagrange, candidats
  classés (minimum/maximum).
- Intégrales doubles et triples sur un pavé (Fubini) et sur un disque
  (passage en polaires rédigé, jacobien explicite).

**Le plan complexe et les résidus.**
- Évaluateur complexe intégré (`i`, `exp`, `ln`, `sin`, `cos`, `sqrt`,
  puissances).
- Image d'un domaine par une transformation holomorphe (`<Trace>l'image du
  carré ... par w = ...`, `<Trace>l'image du cercle ... par w = ...`).
- Résidus rédigés (`<Calcule>les résidus de ...`) : pôles, ordres,
  `Res(f, z₀)` pour chacun.

**Les groupes.**
- Tables de `ℤ/nℤ` pour l'addition et la multiplication
  (`<Dresse>la table de Z/5Z pour l'addition`).
- Générateurs avec l'indicatrice d'Euler (`<Détermine>les générateurs de
  Z/12Z`).
- Décomposition d'une permutation en cycles à supports disjoints, points
  fixes, signature, ordre (`<Décompose>la permutation (...) en cycles`).

**Les lois à densité.**
- Vérification qu'une fonction est une densité de probabilité (intégrale
  1, bornes finies ou infinies), espérance et variance rédigées
  (`<Étudie>la loi de densité ...`).
- Probabilités exactes (en erf) et approchées pour la loi normale ; tracé
  de sa densité.
- Illustration du théorème central limite : loi exacte de la somme de *n*
  dés (par convolution) superposée à sa limite normale.

**Le vocabulaire s'aligne sur celui des enseignants.** `liste` et `tableau`
nomment désormais le même type que `collection` ; `chaîne`, `chaîne de
caractères` et `texte` sont synonymes ; `renvoie` s'écrit aussi bien que
`retourne`. `inverse(v)` remplace `miroir(v)` : la syntaxe `inverse(...)` et la
prose « l'inverse de la matrice M » ne se confondent pas, la parenthèse les
sépare.

**`non`**, troisième connecteur logique enseigné avec `et` et `ou`, manquait.
Les conditions lisent aussi `vrai` et `faux` en toutes lettres, et un booléen
s'affiche ainsi : `contient(15 ; notes)` rend « vrai », non « 1 ».

**`insère`, `supprime`, `ajoute`** — à l'impératif, non `insertion` et
`suppression` : le nom aurait télescopé « tri par insertion », l'algorithme et
l'opération portant le même mot dans la même page. `ajoute(v ; x)` nomme enfin
l'opération la plus employée du programme, que la concaténation `v + {x}`
couvrait sans la dire.

**La préposition dit qui reçoit l'opération.** `dans notes insère(0 ; 20)` se
lit comme une phrase et nomme sans ambiguïté le conteneur, là où
`insère(notes ; 0 ; 20)` laissait deviner lequel des trois arguments le
désignait. Trois écritures coexistent, au choix de ce qui se lit le mieux : la
forme fonctionnelle, la forme prépositionnelle, et la forme directe
`notes contient(15)` pour les questions par oui ou non.

**La division euclidienne se dit comme au collège** : `quotient de 17 par 5`,
`reste de 17 par 5` — la phrase dit lequel des deux nombres est le dividende,
ce que `quotient(17 ; 5)` taisait. Les formes brèves restent admises, et la
tournure accepte un appel comme dividende : `quotient de longueur(notes) par 2`.

**`sortir`** arrête une boucle sans quitter ce qui l'entoure — c'est la
recherche qui cesse dès qu'elle a trouvé. Dans un corps de fonction comme dans
une boucle de document.

**La forme française `v contient(1)`.** Le conteneur peut précéder la
primitive et se glisse à sa place parmi les arguments : la phrase se lit à voix
haute. `contient(1 ; v)` reste admis.

**Le p-uplet.** Le programme de NSI le distingue explicitement de la liste :
longueur fixe, types pouvant différer. Il s'écrit `(entier ; entier)`, se
retourne, se délie et se lit par rang.

- `soit divise(a: entier ; b: entier): (entier ; entier) = (quotient de a par b ; reste de a par b)`
- `soit (q ; r) = divise(17 ; 5)` — la **déliaison** pose les deux noms d'un
  coup, au niveau document comme dans un corps de fonction ;
- `soit c: (entier ; texte) = (3 ; "trois")` — les membres peuvent différer, et
  `c[1]` rend le second ;
- l'arité est vérifiée : délier trois noms d'un couple est une faute dite, et
  `(1 ; 2)` n'est pas acceptable là où une liste est attendue.

Les extrema d'une série se rendent enfin en un seul parcours.

**`aléatoire(a ; b)`** tire un entier entre les deux bornes, comprises — un
xorshift sans dépendance, semé par l'horloge. Deux compilations donnent deux
tirages : c'est ce qu'une simulation attend. Sans lui, ni Monte-Carlo, ni
marche aléatoire, ni étude de fréquences.

**`continuer`** arrête le tour en cours ; la boucle poursuit. Le pendant de
`sortir`, dans un corps de fonction comme dans une boucle de document.

**Une déclaration typée accepte un appel**, y compris pour un type scalaire :
`soit x: entier = f(3)`.

**L'affectation d'une valeur composée devient cohérente.** Cinq écritures
naturelles échouaient, chacune pour une raison différente :

- `pour k dans d { … d[k] … }` — une **clé de dictionnaire peut être une
  variable**, non plus seulement un mot écrit à la main ;
- `soit f(): un dictionnaire … = {a: 1}` — l'accolade délimite ici un
  **littéral**, non un bloc d'instructions ; la distinction se fait sur la
  présence d'un mot du langage à l'intérieur ;
- `soit p: une liste d'entiers = empile({} ; 1)` — le membre droit d'une
  déclaration typée **n'est plus forcément un littéral** : un appel, une
  primitive, une concaténation conviennent ;
- `soit p = empile({} ; 1)` — une valeur composée **se pose sans que son type
  soit écrit** ; docdg reconnaît ce qu'elle est ;
- `soit v = tri(v)` — un conteneur **se réaffecte**. L'accumulateur
  `soit S = S + {k}` reste prioritaire et intact.

La pile et la file s'écrivent désormais entièrement dans le langage, en sept
lignes et sans une ligne de moteur.

**Le chapitre « texte » du programme s'écrit enfin.** La déclaration d'une
chaîne ne menait nulle part : sept manques la rendaient inerte. Ils sont
comblés ensemble.

- `m[0]` lit **une lettre**, qui est une chaîne d'un seul caractère — docdg
  n'introduit pas de type distinct, pas plus que Python. L'indice hors bornes
  compte en lettres.
- `pour c dans m` parcourt les lettres, dans un document comme dans un corps
  de fonction.
- `majuscule`, `minuscule`, `sans accents` — `majuscule("été")` rend `ÉTÉ`,
  non `ETE` : la typographie française accentue les capitales.
- `code("B")` et `caractère(97)` font le va-et-vient entre une lettre et son
  rang Unicode.
- `texte(42)` et `nombre("1,5")` convertissent, explicitement.
- Un littéral entre guillemets est lu comme chaîne partout, y compris en
  argument : `m contient("jour")`.
- Une **lecture indexée sert d'argument** : `code(m[0])`, `ajoute(r ; a[i])`.
  La concaténation `r + {a[i]}` le permettait déjà, l'appel nommé non.
- **Deux textes se comparent** dans une condition, avec la collation
  française : `si u vaut inverse(u)` reconnaît un palindrome.

César chiffre, le palindrome se reconnaît, le comptage de lettres tourne.

**La chaîne de caractères devient une valeur ordinaire** :
`soit m: chaîne de caractères = "bonjour"`. Un type scalaire n'a pas de
littéral entre accolades — sa valeur tient sur la fin de la ligne. Les
guillemets, droits ou français, délimitent sans appartenir à la valeur, ce qui
permet d'y garder espaces et signes du langage. La chaîne se mesure
(`longueur`), se découpe (`m[0 à 7]`), se joint (`+`) et se passe en argument.

**`quotient(a ; b)`** — la division euclidienne du collège. Le reste s'obtenait
déjà par `%` ; le quotient entier manquait, alors qu'il donne l'indice du
milieu dans une recherche dichotomique.

### Corrigé

**Une primitive restait inerte dans la condition d'un `si` de document.**
`expand_conditions` n'avait accès ni aux conteneurs ni aux fonctions : la même
écriture fonctionnait dans un corps de fonction et pas dans le document. Les
conditions de tour d'une boucle sont désormais résolues au déroulement, ce qui
rend au passage visible un `sortir` niché dans un `si`.

**Un point-virgule d'appel coupait une cellule de tableau en deux.** `split_top`
comptait les accolades et les crochets, mais pas les parenthèses : la rangée
`[contient(2 ; v) ; #{contient(2 ; v)}]` produisait quatre cellules au lieu de
deux. Les parenthèses comptent maintenant au même titre.

- `exp(x)` s'affiche désormais `e^x`, notation francophone, au lieu de
  `\exp(x)`.
- L'interpolation `#nom` replie sur le plus long préfixe de variable connu
  quand le nom lu gloutonnement n'existe pas (`Z/#nZ` avec `n` défini donne
  `Z/7Z` et non un `#nZ` littéral).
- Les cotes et mesures des solides et patrons (module espace) sont
  désormais placées à l'extérieur de la figure, parallèles au segment
  mesuré — reprise du principe déjà appliqué en géométrie plane.
- L'interpolation `#` fonctionne désormais dans la description de toute
  balise (`<Trace>`, `<Calcule>`, `<Étudie>`...), pas seulement `<Trace>`.

### Changé

- Réorganisation des exemples par domaine plutôt que par chantier : les
  nouveautés ci-dessus vivent dans les fichiers `geometrie2/3/4`,
  `analyse4`, `algebre4`, `statistiques-probabilites4`, `basique3`,
  suivant la logique déjà en place pour la 1.0.
- Suppression de `outils/`, `transpiler/examples/` et `transpiler/tests/`
  du dépôt public.

## 1.0 — première publication

Version initiale : docdg couvre le programme de mathématiques du CP aux
classes préparatoires (MPSI/MP), avec la syntaxe en prose française
imperative, la rédaction pas à pas par défaut, et l'export PDF modifiable.

### Points marquants

- Objets et actions de base : `<Soit>`, `<Trace>`, `<Affiche>`, `<Dresse>`,
  `<Calcule>`, `<Étudie>`, `<Résous>`, `<Détermine>`, `<Donne>`,
  `<Applique>`, `<Construis>`, `<Dénombre>`, `<Pose>`, `<Insère>`.
- Collège : trio Pythagore/Thalès/trigonométrie, statistiques d'une série,
  proportionnalité et pourcentages, mesures et conversions, programmes de
  calcul, transformations du plan.
- Lycée : convexité, asymptotes, équations trigonométriques, vecteurs et
  produit scalaire du plan et de l'espace, graphes et matrices
  d'adjacence, chaînes de Markov et état stable, équations diophantiennes,
  racines de l'unité, intervalle de fluctuation, inégalité de
  Bienaymé-Tchebychev.
- Supérieur (CPGE) : algèbre linéaire, réduction, polynômes, séries,
  analyse (limites, dérivées, intégrales, développements limités),
  physique-chimie (unités, conversions, Fourier, Laplace).
- Géométrie plane complète, figures avec repère orthonormé et cercle
  trigonométrique, solides en 3D de base, mode analytique.
- Statistiques : diagrammes, arbres de probabilités, droite graduée.
- Tables de Cayley pour `ℤ/nℤ` et `(ℤ/nℤ)*`.
- Mise en forme : styles en ligne, couleurs, tabulations, notes de bas de
  page, alias `soit`, structures de contrôle, calculs `#{...}`.
- Export PDF modifiable — la différence structurelle avec LaTeX et Typst.
- Publication sur CTAN aux côtés de texecole et scholatex.
