# Guide de démarrage de DocDG 4.1

Ce guide accompagne l'application distribuée. La documentation détaillée se trouve dans
`README.md` et dans les manuels **École**, **Collège**, **Lycée**, **Licence** et **Master et agrégation** fournis
avec le projet source ; l'interface est décrite dans `docs/INTERFACE.md`. La
licence de l'application figure dans `CLUF.md`.

## Ouvrir un exemple

1. Lancer l'application pour son système (exécutable Windows, application macOS ou paquet Linux).
2. Dans le ruban **Fichier**, choisir **Ouvrir** puis un fichier `.txt` du dossier `exemples/`.
3. Modifier le texte dans le volet **Source** : l'**Aperçu** se recompose à chaque frappe et suit la ligne en cours.
4. Dans le ruban **Fichier**, choisir **Exporter** : le PDF est identique, page pour page, à l'aperçu.
   Le même ruban liste, sous **Tables**, les tables du document : un clic sur l'une d'elles l'écrit en `.parquet` ou en `.arrow`, pour Polars, pandas ou un autre document.

Les exemples publics comptent **26 fichiers `.txt`**, dont
**15 recueils thématiques** (algèbre, algorithmique, analyse, analyse de
données, bases, chimie,
démonstrations, géométrie, histoire, physique-chimie, physique, sciences de
l'ingénieur, statistiques, probabilités et simulations). Les ressources des documents sont
rangées dans `exemples/IMAGES/`, `exemples/FIGURES/`, `exemples/DONNEES/` et
`exemples/histoire/`.

Gardez ces dossiers avec leurs fichiers `.txt` lorsque vous déplacez un exemple. Dans `histoire.txt`, le nom de la vignette est une chaîne entre guillemets et les mots qui suivent la balise sont du texte libre. Dans `donnees.txt`, le nuage des températures de l’Alpage compte un point par jour où la minimale et la maximale ont toutes deux été mesurées.

Les archives numérotées des exemples historiques ne sont **pas** livrées avec
la distribution utilisateur : elles sont conservées dans le dépôt de développement
sous `tests/fixtures/exemples-historiques/` pour les tests de non-régression.

## Réouvrir et corriger un document

Après sauvegarde et composition valide, docdg garde le Typst du document dans
son cache privé, dans votre dossier de données. À la réouverture, il le
réutilise si la source, le moteur et les ressources correspondent encore. Rien
n'est écrit à côté du `.txt`, qui reste votre document de travail ; un `.typ`
personnel placé là n'est ni lu ni modifié.

Fermer la fenêtre par la croix demande confirmation : « Quitter » ou Entrée
ferme, « Annuler » ou Échap revient au document. Si le document n'a jamais été
enregistré, la confirmation prévient que son texte serait perdu.

Une ligne qui ressemble à une instruction algorithmique mais n'en respecte
pas la syntaxe est composée comme du texte ordinaire, sans avertissement et
sans déclarer de variable. Un mot-clé en minuscule ne suffit pas à déclencher
l'exécution. Si une variable est ensuite utilisée sans déclaration valide,
le diagnostic apparaît à cet endroit dans l'aperçu. Les autres éléments restent composés. Deux écritures sont
signalées à leur place : une déclaration complète terminée par un point
(`soit n ∈ ℕ tel que n = 7.`), avec l'invitation à retirer le point, et un bloc
dont l'accolade fermante manque, à sa ligne d'ouverture ; le contenu de ce bloc
reste lisible comme source pour permettre sa correction. Aucun
bandeau rouge d'erreurs documentaires n'est ajouté au-dessus des pages. Les
aperçus contenant des diagnostics restent provisoires et ne remplacent pas le
cache valide. Un PDF exporté reproduit l'aperçu, diagnostics compris.

## Le typage est obligatoire

**Règle sans dérogation :** toute variable déclare son type à sa naissance, dans la notation du supérieur — l'ensemble pour un nombre, le nom de l'objet avec son article pour les autres types, la signature pour une fonction. Cette règle s'applique aux nombres, textes, listes, dictionnaires, vecteurs, matrices, styles, résultats de saisie, fonctions et indices de boucle (`pour k ∈ ⟦1 ; 10⟧`).

```docdg
soit n ∈ ℕ tel que n = 12
soit q ∈ ℚ tel que q = 1/3
soit z ∈ ℂ tel que z = 1 + 2i
soit titre un style défini par <bleu gras au centre>
soit notes une liste de décimaux telle que notes = {12,5 ; 15 ; 9,5}
soit affiche : ℕ → ℕ, n ↦ n
n ← 13
```

**Ces lignes ne déclarent rien :** `soit n = 12` (type absent), `soit n: un entier naturel = 12` (écriture à deux-points de la 3.9), `soit titre = <bleu gras>` (style non déclaré). Elles restent affichées comme du texte, sans alerte. L'utilisation ultérieure de `#n` sans déclaration valide déclenche le diagnostic ; écrire `$n$` dans une formule reste une simple notation mathématique. **Une réaffectation** s'écrit avec la flèche des manuels d'algorithmique, `n ← 13` : elle ne recrée pas la variable et ne répète pas son type. La graphie `<-` n'existe pas, car `x <- 5` se confondrait avec la comparaison `x < -5`.

Les constructions `<Soit>A(1 ; 2) un point` (énoncé mathématique) et `soit n ∈ ℕ tel que n = 12` (déclaration algorithmique) sont différentes. Une écriture historique qu'un parcours du moteur accepterait encore n'en devient pas correcte : elle se corrige avant publication.

**Prose et algorithme.** `soit $x = 2$ ou $x = -2$.`, `pour cette raison, le résultat est vrai.` et `selon la valeur de $x$, le signe change.` sont des phrases affichées sans avertissement. `Soit` avec une majuscule reste également du texte. Une instruction complète et conforme, comme `soit n ∈ ℕ tel que n = 12`, est exécutée. Les erreurs de calcul et de valeur d'une instruction reconnue restent signalées à leur emplacement.

Référence normative : [`docs/TYPAGE_OBLIGATOIRE.md`](docs/TYPAGE_OBLIGATOIRE.md).

## Les parenthèses, les crochets et les accolades

L'éditeur referme ce qu'il ouvre : taper `(`, `[` ou `{` pose le signe fermant
et laisse le curseur entre les deux. Taper le fermant déjà présent passe
par-dessus ; le retour arrière entre une paire vide efface les deux signes ;
Entrée entre `{` et `}` ouvre le bloc sur une ligne indentée ; un passage
sélectionné est entouré par la paire. Les intervalles ouverts se saisissent
comme d'habitude : `]a ; b[`, `[a ; b[`.

## Insérer depuis le bandeau

Dans **Actions**, **Mathématiques** ou **Programmation**, choisissez un modèle pour l’insérer au curseur. Si un passage est sélectionné, le modèle le remplace. Un modèle d’action qui demande par exemple deux points ajoute avant lui les déclarations qui manquent. **Annuler** restaure le passage avec sa sélection. **Rétablir** retrouve le texte inséré et sa position d’édition.

Dans une formule, le bouton `^{}` de **Puissance** ou `_{}` d’**Indice** insère une paire d’accolades et place le curseur entre elles. On peut ainsi compléter `$u_{n+1}$`. Les boutons **x²** (exposant) et **x₂** (indice) de l’onglet **Styles** entourent le texte sélectionné. Dans **Alphabet**, choisissez un groupe puis une lettre, par exemple `𝔸`.

Les guillemets délimitent une valeur textuelle. Dans l’exemple suivant, la première ligne déclare la valeur et la seconde affiche la phrase **Le nom de cet élève est Inès.**

```docdg
soit nom une chaîne de caractères telle que nom = "Inès"
Le nom de cet élève est #nom.
```

## Une seule écriture par notion

Chaque notion s'écrit d'une seule façon, celle du tableau, à côté de son
symbole. Une ancienne écriture n'est jamais lue de travers : l'aperçu donne,
à sa ligne, la forme attendue.

| Notion | Écriture |
|---|---|
| valeur absolue | `|x|` ou `valeur absolue(x)` |
| cardinal | `cardinal(A)` |
| loi normale | `normale(0 ; 1)` |
| réciproques hyperboliques | `argsh(x)`, `argch(x)`, `argth(x)` |
| négation | `non P` |
| appartenance | `x appartient à E`, `pour tout x appartenant à E`, `x ∈ E` |
| non parallèle, différent | `(AB) non parallèle (CD)`, `a différent de b` (ou `a != b`) |
| exposant composé | `10^(-3)`, `u_(n+1)` |
| colonnes | `<Composer>sur 2 colonnes { … }` |
| constante, test, primitive | `<Donner>la constante de Planck`, `<Effectuer>le test de Student …`, `<Calculer>une primitive de f` |
| tolérances | `avec une tolérance relative de 10^(-8) et une tolérance absolue de 10^(-10)` |
| homogénéité | `<Vérifier>l'homogénéité de E = m c^2 avec E énergie, m masse, c célérité` |

Dans une classe, les membres se déclarent comme partout : `abscisse ∈ ℝ`,
`titulaire une chaîne de caractères`, et une méthode comme une fonction
(`soit norme une fonction à valeurs dans ℝ définie par norme() = …`).

## Saisir un tableau

Le bandeau insère un tableau complet, déjà aligné : sa ligne de pistes, une ligne d'entêtes avec une fusion horizontale (`entête1  -  -`) et deux entêtes de ligne à fusion verticale (`entête7:`). Il suffit d'en remplacer les mots. En frappe directe, la **tabulation** passe à la cellule suivante et le **retour à la ligne** commence une nouvelle ligne : à chacun de ces gestes, l'éditeur réaligne toutes les colonnes du tableau, et le curseur attend sous la colonne suivante. Le retrait se reprend tout seul d'une ligne à l'autre, augmente après une accolade ouvrante et diminue quand on tape l'accolade fermante.

## Créer un premier document

Créer un fichier texte contenant :

```docdg
document {
    orientation: portrait;
    marges: 15;
    taille: 12;
}

<gras au centre>Mon premier document

Calculer $2+3$ et écrire le résultat.
```

Les balises règlent la présentation et les consignes mathématiques. Le fichier
source demeure un texte ordinaire ; les calculs avancés peuvent requérir le
moteur Python embarqué et son initialisation. Les fichiers du dossier
`exemples/` constituent les références pratiques les plus complètes.

## Limites de cette livraison

Les conventions de notation et de typage sont fixées par
`docs/NOTATIONS_SUPERIEUR.md` et `docs/TYPAGE_OBLIGATOIRE.md`. Les tournures
garanties sont celles que composent les exemples de `exemples/`, les blocs de
code des manuels et les choix du bandeau : chacun est exécuté par la suite de
tests. Les réserves typographiques connues sont listées dans
`docs/RESERVES_POUR_4_1.md`.

Pour signaler un défaut, joindre la version de docdg, le système d'exploitation,
le fichier `.txt` minimal qui reproduit l'erreur et, si nécessaire, le PDF produit.

## Statistiques et probabilités, deux parcours

Les fichiers `exemples/statistiques.txt` et `exemples/probabilites.txt` peuvent être ouverts séparément. Le premier réunit les indicateurs, les graphiques et l’inférence ; le second présente les lois conjointes, le conditionnement et les simulations à graine explicite. `statistiques-probabilites.txt` reste disponible comme archive pédagogique et référence comparative de la version 4.1.

## Estimation et tests statistiques (S2)

La rubrique Probabilités et statistique du bandeau propose désormais six modèles d’estimation et d’inférence pour le supérieur. Ouvrez `exemples/statistiques.txt` pour comparer les résultats ; le texte rédigé est libre, les valeurs littérales dans le code restent entre guillemets. Référence : `docs/STATISTIQUES_INFERENCE_S2.md`.

### Statistiques multivariées — DocDG 4.1, S3

DocDG propose la régression linéaire multiple avec diagnostics des résidus et des points influents, l'ACP centrée-réduite par SVD et l'analyse de variance (ANOVA) à un facteur. Les commandes en français, les hypothèses d'application, les limites numériques et les exemples sont décrits dans [`docs/STATISTIQUES_MULTIVARIEES_S3.md`](docs/STATISTIQUES_MULTIVARIEES_S3.md). Les démonstrations correspondantes figurent dans `exemples/statistiques.txt`.

### Probabilités et statistiques avancées · S4

Ouvrir `exemples/probabilites.txt` pour les classes et les lois stationnaires d’une chaîne de Markov. Dans `exemples/statistiques.txt`, deux chapitres traitent de l’estimation par maximum de vraisemblance et d’une activité trimestrielle. Les modèles se trouvent sous **Étudier** et **Calculer**. Voir le [guide S4](docs/STATISTIQUES_PROBABILITES_S4.md).

### Lois et description des séries · S5

Dans le bandeau **Mathématiques** du niveau **Supérieur**, ouvrir le domaine **Probabilités et statistique**. Les verbes **Calculer**, **Déterminer**, **Tracer**, **Étudier** et **Représenter** donnent accès aux modèles S5. Les pages « Lois discrètes et plans de tirage » et « Rangs, dispersion relative et forme » dans les deux recueils en montrent l’usage. [Guide de syntaxe et de calcul S5](docs/LOIS_ET_DESCRIPTION_S5.md).

### Inférence et prévision · S6

Dans Mathématiques au niveau Supérieur, les verbes Effectuer, Calculer, Déterminer et Étudier donnent accès aux modèles S6. Le chapitre « Étude économique sur douze magasins » reprend le cycle complet, depuis la lecture du CSV jusqu’à la prévision des ventes. [Commandes, hypothèses et exemples S6](docs/TESTS_ET_MODELES_S6.md).

### Couples, Markov et estimation · S7

Les cas « Deux produits dans un panier » et « De la réflexion à l’achat » se trouvent dans le recueil de probabilités. Les calculs d’estimation et de risque se trouvent dans le recueil de statistiques. [Commandes, conventions et exemples](docs/PROBABILITES_ESTIMATION_S7.md).

Dans le bandeau des actions, choisir la matière et le niveau avant le verbe et le domaine. Si le choix comporte plus de 17 objets, les boutons Précédente et Suivante donnent accès aux autres pages. Changer de domaine ou de filtre ramène à la première page.

## Insérer un résultat de statistiques ou de probabilités

Écrivez par exemple `<Énoncer>Courbe de Lorenz et indice de Gini` pour insérer la fiche dans le document. Pour obtenir une preuve, écrivez `<Démontrer>Variance corrigée d’un échantillon, au niveau licence 2`. Les recueils `statistiques.txt` et `probabilites.txt` se terminent chacun par un chapitre consacré à ces appels. [Titres disponibles et niveaux des démonstrations](Redaction.md).

Pour passer d’un résultat numérique à son interprétation, essayez `<Énoncer>Intervalle de prévision en régression linéaire` dans un exercice de prévision, ou `<Énoncer>Intervalle de Wilson` après une estimation de proportion.
