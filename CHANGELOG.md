# Journal des versions

Ce projet suit un versionnage simple : le premier chiffre marque un changement
de nature (ce qu'on peut faire avec docdg), le second une extension dans le
même esprit.

## 2.3 — la qualité du gris typographique

La composition du texte courant, au niveau de la ligne : les mots se coupent
en fin de ligne selon les motifs français, et aucune ligne ne reste seule au
bas ou au sommet d'une page.

### Corrigé

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

### Corrigé

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

### Corrigé

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

### Corrigé

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
