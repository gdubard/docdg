# *DocDG 4.0*

> Interface DocDG 4.0 : [rubans, volets, saisies et export](docs/INTERFACE.md).

**docdg** est une application de composition documentaire écrite en Rust,
autour d'un langage de balises en français. L'auteur écrit un fichier texte
ordinaire, que le moteur transforme en document ; l'interface est native
(iced), et Typst compose l'aperçu comme le PDF exporté, identiques page pour
page. Elle couvre notamment cadres,
grilles, tableaux, images, notations mathématiques, tracés de fonctions,
géométrie plane et spatiale, statistiques, physique-chimie et rédaction longue
(chapitres, renvois croisés, bibliographie). La justesse scientifique et la
fidélité typographique doivent être appréciées à l'aide des tests et exemples,
non déduites de la seule présence d'une commande dans la documentation.

**Auteur et mainteneur :** Gérard Dubard — docdg69@gmail.com
**Licence :** gratuiciel propriétaire — voir [CLUF.md](CLUF.md)
**Dépôt :** https://github.com/gdubard/docdg

---

**Des fiches d'exercices prêtes à imprimer, sans écrire de code.**

> *Un petit langage de balises, cohérent, pour les documents qu'un enseignant fabrique réellement.*

[![Version](https://img.shields.io/badge/version-4.0-2980b9?style=flat-square)](CHANGELOG.md)
[![Engine](https://img.shields.io/badge/moteur-Rust-E95420?style=flat-square)](https://www.rust-lang.org/)
[![Standalone](https://img.shields.io/badge/distribution-autonome-27ae60?style=flat-square)](https://github.com/gdubard/docdg)
[![Licence](https://img.shields.io/badge/licence-propriétaire_freeware-8e44ad?style=flat-square)](CLUF.md)

**[Journal des versions](CHANGELOG.md)**

> **Règle normative depuis la version 3.9 : typage explicite obligatoire.** Toute nouvelle variable, tout style, conteneur, paramètre de fonction, type de retour, résultat de saisie et indice de boucle doit porter un **type déclaré explicitement, avec son article français**. Une inférence possible ne dispense jamais de la déclaration. Par exemple `soit n ∈ ℕ tel que n = 5` et `soit h1 un style défini par <bleu gras>` ; `soit n = 5` et `soit n: entier naturel = 5` ne déclarent aucune variable et restent du texte sans avertissement. Une utilisation ultérieure d'un nom sans déclaration valide est signalée à son emplacement. Une réaffectation n'est **pas** une déclaration : `n ← 6`. Le contrat détaillé figure dans [Typage obligatoire](docs/TYPAGE_OBLIGATOIRE.md). La persistance éventuelle d'une syntaxe historique dans le moteur ou un exemple est un **défaut à corriger**, non une variante autorisée.

---

## Avant de commencer : installation

**docdg est une application autonome** : l'exécutable est la seule chose à installer. Il embarque son propre moteur de calcul — Python 3.13 avec SymPy et mpmath pour le calcul formel, NumPy et SciPy pour le calcul numérique, dans des versions figées et propres à chaque système — qui prend en charge les calculs avancés (tableau de variations, dérivées, primitives, zéros de fonctions, lois de probabilité, équations différentielles). Aucune distribution TeX, aucun éditeur spécialisé, aucun Python à installer : un Python éventuellement présent sur la machine n'est ni utilisé ni modifié.

**Télécharger docdg.** Récupérer le binaire correspondant à votre système sur la page des versions du dépôt :

```
https://github.com/gdubard/docdg/releases
```

Sous Windows, télécharger et lancer l'exécutable `.exe` ; il n'est pas nécessaire de modifier le `PATH` pour une application graphique. Sous macOS, utiliser l'application `.app`. Sous Linux, télécharger l'**AppImage**, la rendre exécutable (`chmod +x`) et la lancer : elle ne dépend ni de GTK ni de WebKitGTK depuis la 4.0 (les dialogues de fichiers passent par le portail XDG du bureau). Consulter [docs/COMPILER.md](docs/COMPILER.md) pour l'AppImage et la validation sur chaque système.

**Premier lancement.** Au tout premier démarrage, docdg installe son moteur de calcul dans le dossier de données de l'utilisateur (`%LOCALAPPDATA%\docdg\runtime` sous Windows, `~/Library/Application Support/docdg/runtime` sous macOS, `~/.local/share/docdg/runtime` sous Linux), une fois pour toutes : l'opération prend quelques secondes et se fait en arrière-plan. Une nouvelle version de docdg remplace ce dossier et efface l'ancien.

**Les témoins du moteur.** En haut à droite de la fenêtre, une pastille
**Document** puis une pastille par bibliothèque — SymPy, mpmath, NumPy et
SciPy — portent chacune leur voyant. Celui du document est rouge pendant le
chargement ou une recomposition, vert dès que l'aperçu est à jour, et le temps
global de chargement s'affiche à côté. Ceux des bibliothèques sont rouges
pendant leur chargement, qui se fait en arrière-plan dès l'ouverture, verts
dès qu'elles sont disponibles — une à deux secondes d'ordinaire, davantage au
tout premier lancement. On peut écrire pendant le chargement : les calculs qui
en dépendent s'affichent dès que le moteur est prêt.

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
| [Redaction.md](Redaction.md) | les 1606 énoncés et les 602 démonstrations du corpus, par matière et par niveau, avec la phrase qui les appelle — plus les 101 démonstrations classiques servies par `<Montrer>` |

### La référence

Les commandes **présentes et testées** sont celles que composent les exemples de `exemples/` et les blocs de code des manuels : chacun est exécuté par la suite de tests, et le contrat du typage est dans [docs/TYPAGE_OBLIGATOIRE.md](docs/TYPAGE_OBLIGATOIRE.md).

**Conventions du supérieur :** [contrats de notation, de typage et d'hypothèses mathématiques](docs/NOTATIONS_SUPERIEUR.md). Cette référence distingue la syntaxe écrite par l'utilisateur de la notation mathématique et documente les limites actuelles du moteur.

1. [Qu'est-ce que docdg ?](#-quest-ce-que-docdg-)
2. [Fonctionnalités clés](#-fonctionnalités-clés)
3. [Pourquoi docdg ?](#-pourquoi-docdg-)
4. [Ce dont vous avez besoin](#-ce-dont-vous-avez-besoin)
5. [Premiers documents](#-premiers-documents)
6. [Le classeur de saisie](#-le-classeur-de-saisie) — les six intercalaires, l'insertion directe, l'enregistrement automatique
7. [La syntaxe](#-la-syntaxe)
8. [Les objets](#-les-objets) — cadres, tableaux, images, listes, grilles, sections
9. [La mise en colonnes](#-la-mise-en-colonnes) — un passage sur une à cinq colonnes, le reste en pleine largeur
10. [Les actions](#-les-actions) — `Soit`, `Tracer` : figures, repère, solides, mode analytique
11. [Les démonstrations](#-les-démonstrations) — les dix raisonnements, la preuve automatique, la bibliothèque
12. [Les saisies interactives](#-les-saisies-interactives) — `<Saisir>`, blocage typé, alternative `si … sinon`, le document vivant
13. [Les styles et la mise en forme](#-les-styles-et-la-mise-en-forme)
14. [Le langage algorithmique](#-le-langage-algorithmique) — types, conteneurs, primitives, chaînes, fonctions, p-uplets
15. [Les graphiques de fonctions](#-les-graphiques-de-fonctions)
16. [Les mathématiques](#-les-mathématiques)
17. [La géométrie](#-la-géométrie)
18. [La géométrie dans l'espace](#-la-géométrie-dans-lespace) — solides, patrons, repère de l'espace, droites et plans, positions relatives
19. [Les statistiques](#-les-statistiques) — diagrammes, arbres de probabilités, droite graduée
20. [L'écriture en réglure](#-lécriture-en-réglure) — la réglure Seyès, la cursive, les styles et les mathématiques sur les lignes
21. [La physique-chimie](#-la-physique-chimie) — équations, masses molaires, avancement, unités, constantes, incertitudes
22. [La frise chronologique](#-la-frise-chronologique) — l'histoire entre en scène : dates, titres, détails
23. [Le corpus : le personnaliser, l'emprunter](#-le-corpus--le-personnaliser-lemprunter) — l'onglet Édition, le journal, la base d'un établissement
24. [Les exemples, par niveau](#-les-exemples-par-niveau)
25. [Les documents complexes](#-les-documents-complexes)
26. [Comprendre les erreurs et les cas particuliers](#-comprendre-les-erreurs-et-les-cas-particuliers)
27. [Référence complète](#-référence-complète) — couleurs et options de classe
28. [Bonnes pratiques](#-bonnes-pratiques)
29. [Le calcul scientifique étendu](#-le-calcul-scientifique-étendu) — SymPy : ce que débloque le second moteur
30. [Les tournures et le placement](#-les-tournures-et-le-placement) — synonymes, bornes naturelles, placement en langage naturel

*Annexes, index alphabétique et index thématique : pas encore rédigés dans cette version, à venir.*

---

## **📖 Qu'est-ce que docdg ?**

**docdg est un langage de composition documentaire.**  
Il permet de **créer simplement des documents de grande qualité** grâce à un **moteur de composition Rust** qui produit directement une composition Typst et son PDF.

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
2. **L’aperçu et le PDF générés** (ce que docdg **produit pour vous**) ;
3. **Une explication** de ce que fait ce code.

> 💡 **Vous pouvez lire ce manuel dans l'ordre**, ou **sauter directement au chapitre qui vous intéresse** et revenir aux fondations en cas de besoin.

### **📜 Une convention de lecture**

Dans tout ce manuel, un bloc marqué **`docdg`** est ce que **vous écrivez**. C'est la seule syntaxe à connaître : docdg génère l’aperçu Typst et le PDF pour vous, en coulisse.

---

## **✨ Fonctionnalités clés**


| **Fonctionnalité**                  | **Description**                                                                                                           | **Exemples**                                       |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| **🏷️ Une seule syntaxe de balise** | `<attributs>{contenu}` couvre texte, tableaux, images, maths, cadres, listes et mises en page pleine page                 | `<Afficher>un cadre avec une bordure bleue{Texte}`, `<Dresser>un tableau {<mc>...}` |
| **🎨 148 couleurs nommées**         | Une palette en **français**, une couleur par nom, accordée selon l'orthographe | `rouge`, `bleu nuit`, `vert forêt` |
| **📐 Un mini-langage mathématique** | Fractions, racines, sommes, produits, intégrales, dérivées, trigonométrie, matrices et systèmes, **le tout dans `$...$`** | `$somme(k=1;n) k^2$`, `$intégrale(x=a;b) f(x)$`    |
| **📦 Cadres et grilles**            | Panneaux `<Afficher>un cadre` colorés et mises en page Typst à zones nommées et fusionnées                                            | `<Afficher>une grille`, `<Afficher>un cadre avec des coins arrondis de 5 mm`       |
| **📊 Des tableaux sans douleur**    | Placement de cellule en **deux lettres**, fusions, en-têtes, bordures, **aucun `&` à compter**                            | `<Dresser>un tableau {<mc>...}`                        |
| **♻️ Alias et macros**              | Factorisez un style **une fois**, nommez-le partout ; **une seule retouche restyle tout le document**                     | `soit titre un style défini par <gras 18 pt>`, `soit exercice{n} = ...` |
| **🔁 Structures de contrôle**       | Boucles, conditionnelles et interpolation **`#{...}`** dans le corps du document                                          | `pour n ∈ ⟦1 ; 5⟧`, `si note >= 10`                 |


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

docdg est un exécutable autonome : rien d'autre à installer, ni Python, ni SymPy, ni distribution TeX — le moteur de calcul formel est embarqué.

**Téléchargement :** récupérez le binaire pour votre système sur la page des versions :

```
https://github.com/gdubard/docdg/releases
```

Sous macOS, glissez `docdg.app` dans `/Applications`. Sous Windows, lancez
`docdg.exe`. Sous Linux, après extraction de l'archive :

```bash
chmod +x docdg
sudo mv docdg /usr/local/bin/
```

Vérifiez l'installation — la commande affiche `DocDG 4.0` :

```bash
docdg --version
```

Lancé avec le chemin d'un fichier (`docdg exemples/vitrine.txt`), DocDG
l'ouvre directement.

## **🚀 Premiers documents**

### **📄 Le document minimal**

**docdg**

```docdg
Bonjour le monde !
```

> C'est tout : saisissez ce texte dans docdg, enregistrez votre document, puis utilisez l'export PDF.
### **⚙️ Le bloc document**

Tout ce qui décrit le **corps physique** du document — son format, ses polices, sa conduite — s'écrit dans un bloc `document { }` en tête du fichier. Il est entièrement facultatif : sans lui, des valeurs par défaut raisonnables s'appliquent.

> **📌 L'identité n'est pas un réglage.** Le titre, l'auteur, l'institution, l'emblème, la nature du document : tout cela **se compose** sur la première page, avec les mêmes outils que le reste — des styles (`au centre`, une taille), `<Insérer l'image …>` pour l'emblème, et `<page suivante>` pour refermer la couverture. Le bloc `document { }` ne décrit que le papier. Une clé d'identité qui s'y glisse est refusée, et le refus nomme le chemin.

**docdg**

```docdg
document {
  marges: 20;

  espacements: 2;

  script: Georgia;
  seyès: Marelle;

  taille: 11;
  interligne: 1,3;
  tabulation: 8;
  hauteur: 8;
  décalage: 100;

  césure: oui;
  orphelines: 2;
  veuves: 2;

  précision: 3;
}
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| Rubrique | **Marges externes** |
| `marges: 20;` | 20 mm sur les quatre côtés — le défaut est 4;5;5;5 |
| Rubrique | **Espacements internes** |
| `espacements: 2;` | 2 mm entre le contenu et le bord des cadres — le défaut est 0 |
| Rubrique | **Polices** |
| `script: Georgia;` | Police du texte imprimé — les lettres scriptes |
| `seyès: Marelle;` | Cursive des lignes réglées — seule la Marelle est embarquée ; un fichier .ttf voisin est lu et joint au document |
| Rubrique | **Taille et espacement** |
| `taille: 11;` | Corps de base (pt) |
| `interligne: 1,3;` | Coefficient d'interligne |
| `tabulation: 8;` | Largeur tabulation (mm) — le carreau de la Seyès |
| `hauteur: 8;` | Hauteur saut de ligne (mm) — l'interligne de la Seyès |
| `décalage: 100;` | Décalage exposants/indices (%) |
| Rubrique | **Typographie** |
| `césure: oui;` | Coupure des mots en fin de ligne |
| `orphelines: 2;` | Lignes minimales en bas de page |
| `veuves: 2;` | Lignes minimales en haut de page |
| Rubrique | **Calculs** |
| `précision: 3;` | Décimales pour arrondi (-1 = pas d'arrondi) |


> **💡 Virgule décimale.** Dans le bloc, la virgule est la virgule décimale (`interligne: 1,2`), **et elle seule** : `interligne: 1.2` est refusé, et le refus nomme la forme attendue. Tolérer les deux écritures, ce serait les enseigner toutes les deux. Les valeurs à quatre composantes utilisent le point-virgule (`marges: 25;20;25;20`). Partout dans le corps du document, la virgule reste la virgule décimale ; docdg la distingue d'une énumération par l'absence d'espace après elle.

> **⚠️ Une valeur mal écrite se dit.** Un réglage numérique illisible — un point décimal, un mot à la place d'un nombre, deux mesures là où il en faut une ou quatre — ne tombe plus en silence : la clé serait restée à son défaut sans que rien ne le signale, et l'auteur aurait cherché longtemps. La valeur est refusée et nommée, séparément des clés inconnues : ce ne sont pas les mêmes reproches.

> **📌 Un bloc, un nom.** Le bloc s'appelle `document { }`, et c'est le seul. L'ancien `page { }` est refusé, et le refus nomme le remplacement — comme toute clé retirée : une clé sans accent (`cesure`), un ancien alias anglais (`margin`, `padding`) ou une clé d'identité reçoivent la phrase qui remet le document au propre. Rien ne casse en silence.

**Tableau récapitulatif :**

| **Option**   | **Type**           | **Défaut**  | **Description**                |
| ------------ | ------------------ | ----------- | ------------------------------ |
| `orientation`| `portrait`/`paysage` | `portrait` | Orientation de la page       |
| `marges`     | nombre ou 4 valeurs | `4;5;5;5`  | Marges externes (mm) — `20` ou `25;20;25;20` (haut;droite;bas;gauche) |
| `espacements`| nombre ou 4 valeurs | `0`        | Espacements internes des cadres, tableaux et zones (mm) |
| `script`     | texte              | `Georgia`   | Police du texte imprimé — les lettres scriptes ; Georgia, Times New Roman et Arial voyagent avec le document, tout autre nom est cherché sur la machine |
| `seyès`      | texte              | `Marelle`   | Cursive des lignes réglées — seule la `Marelle` est embarquée ; un fichier `.ttf` voisin est lu et joint au document |
| `taille`     | nombre             | `11`        | Taille de base (pt)            |
| `interligne` | nombre             | `1,3`       | Coefficient d'interligne       |
| `tabulation` | nombre             | `8`         | Largeur tabulation (mm) — le carreau de la Seyès |
| `hauteur`    | nombre             | `8`         | Hauteur saut de ligne (mm) — l'interligne de la Seyès |
| `décalage`   | nombre             | `100`       | Décalage exposants/indices (%) |
| `césure`     | `oui`/`non`/nombre | `oui`       | Coupure des mots en fin de ligne, selon les motifs français — un nombre relève le seuil : `césure: 8;` épargne tout mot de moins de huit lettres |
| `orphelines` | nombre             | `2`         | Lignes minimales laissées en bas de page |
| `numérotation` | `composée`, `simple` ou `sans` | `composée` | Numéros de page : `1 / 3`, `1` seul, ou aucun |
| `veuves`     | nombre             | `2`         | Lignes minimales reportées en haut de page |
| `précision`  | nombre             | `-1`        | Décimales pour arrondi (`-1` = pas d'arrondi) |
| `graine`     | entier positif     | *absente*   | Fixe les tirages : le document devient reproductible, l'énoncé et son corrigé coïncident. Absente, chaque compilation tire à neuf |
| `récursion`  | entier de 10 à 100 000 | `500`   | Profondeur maximale des appels imbriqués d'une fonction. Au-delà, le calcul s'arrête avec un message, sans jamais faire planter l'application ; relevez la limite pour une récursion profonde mais légitime |
| `niveau`     | texte              | *absent*    | Le niveau scolaire du document, en toutes lettres — de `CP` à `master 2`. Il décide de ce que le corpus admet et de ce qu'il démontre |
| `corpus`     | chemin             | *embarqué*  | Désigne une base personnelle (`.docdgc`) à la place de la base embarquée — celle d'un enseignant ou d'un établissement |

> **🖱️ Sans écrire le bloc.** L'intercalaire **Document** du classeur affiche toutes ces options. Chaque changement met à jour le bloc `document { }` en tête du fichier, sans bouton à presser, en n'inscrivant que les réglages qui diffèrent du défaut.

> **↦ L'alinéa partout.** Une tabulation en tête de ligne creuse un alinéa de la largeur réglée par `tabulation`, **au fil du texte comme à l'intérieur d'un cadre, d'une grille ou d'un bloc**. La règle est la même partout : une tabulation, un alinéa ; deux tabulations, deux alinéas. Dans un tableau en revanche, la tabulation reste le séparateur de colonnes — elle y garde son rôle de structure.

> **✂️ La césure et les lignes isolées.** Trois réglages, actifs par défaut, tiennent la qualité du gris typographique. La **césure** coupe les mots longs en fin de ligne selon les motifs français de TeX (deux lettres au moins avant la coupure, trois après) : les blancs interlettres d'un texte justifié s'en trouvent régularisés. Elle épargne déjà, par défaut, tout mot de moins de six lettres, les mots composés, les sigles, les noms propres, les titres et tout ce qui relève des mathématiques. Si six lettres restent trop court à votre goût, un nombre à la place de `oui`/`non` relève ce seuil : `césure: 10;` n'autorise plus la coupure qu'à partir de dix lettres, sans rien changer aux autres exceptions. Les **orphelines** sont les lignes esseulées en bas de page, les **veuves** celles qui échouent seules en haut de la suivante : docdg coupe désormais un paragraphe à une frontière de ligne plutôt que de le reporter en bloc, en garantissant le nombre de lignes demandé de chaque côté. Le réglage `césure: non` rend le texte tel quel, sans coupure.

> **␣ Les blancs de la ponctuation française.** docdg pose lui-même, au rendu, les espaces que la typographie française exige et que le clavier ne donne pas : une **espace fine insécable** devant `;`, `!` et `?`, une **espace insécable** devant `:` et `»` et après `«`. Rien à écrire ni à régler — on tape une espace ordinaire, docdg la remplace par la bonne, et la ligne ne se coupe plus devant un signe double, à l'écran comme dans le PDF. Les mathématiques, le code et les éléments de classe `donnee` sont épargnés : là, un point-virgule sépare des données et n'est pas une ponctuation.

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
soit titre un style défini par <rouge gras 18 pt au centre>

<titre>Ma feuille d'exercices
```
---

## **🗂️ Le classeur de saisie**

L'onglet **Saisie** de l'application porte sept rubans, décrits en détail dans
[docs/INTERFACE.md](docs/INTERFACE.md).

| ruban | ce qu'on y trouve |
|---|---|
| **Fichier** | Nouveau, Ouvrir, Enregistrer sous et Exporter ; le chemin de la source et celui du dernier PDF exporté |
| **Document** | tous les réglages du bloc `document { }` — page, texte, marges, espacements, conduite, numérotation, niveau, corpus |
| **Styles** | tout ce qui se met entre chevrons sans verbe : caractère, alignement, couleur et fond, police, taille, niveau de titre et forme de numérotation |
| **Actions** | les trente-sept verbes, rangés par catégorie, et leurs objets, filtrés par matière et par niveau |
| **Corpus** | les énoncés et les démonstrations de la base, filtrés par matière, niveau, domaine et type |
| **Mathématiques** | les formules prêtes à l'emploi, en cinq domaines puis en catégories |
| **Programmation** | déclarations, saisies, calcul matriciel, structures et instructions du langage algorithmique |

Actions, Mathématiques et Programmation se déplient en cascade : chaque étage
s'ouvre dans un cadre titré, chaque choix est une pastille, et les rangées
passent à la ligne au lieu de défiler. Les volets Source et Aperçu gardent leur
place.

L'enregistrement est automatique dès que le document a un fichier : il est
réécrit une seconde après la dernière frappe. Il n'y a donc ni bouton
« Enregistrer » ni raccourci d'enregistrement ; un nouveau document reçoit son
fichier par **Enregistrer sous**. **Ouvrir** remplace le texte affiché au lieu
de l'ajouter à la suite.

**Fermer la fenêtre demande confirmation.** La croix ouvre « Quitter DocDG ? » :
**Quitter** (ou Entrée) ferme, **Annuler** (ou Échap, ou un clic à côté) revient
au document. Un document qui n'a jamais reçu de fichier y est signalé : son
texte serait perdu.

**La réouverture réutilise le Typst sauvegardé.** Après enregistrement du texte
et composition valide, docdg garde le Typst du document dans son cache privé,
dans le dossier de données de l'utilisateur — et là seulement : il n'écrit rien
à côté de `cours.txt`. À la prochaine ouverture, il vérifie le texte, le moteur
et les ressources avant de reprendre ce Typst sans réexécuter les instructions
DocDG. Un document courant est repris d'un seul tenant ; pour un document
volumineux, la première page est montrée d'abord, puis le document entier. La
composition des pages par Typst reste nécessaire. Le cache se régénère seul et
ne garde que les 256 documents les plus récemment composés ; le `.txt` reste le
document à modifier.

**La prose reste de la prose.** Une ligne dont la syntaxe algorithmique n'est
pas reconnue reste affichée sans avertissement et n'est pas exécutée. Une
utilisation de variable sans déclaration valide, une erreur de calcul ou une
erreur documentaire est signalée à son emplacement ; l'aperçu continue à
composer les éléments valides. Les
messages ne sont plus regroupés en rouge au-dessus du document. Deux écritures
sont signalées à leur place, parce que l'intention y est sans équivoque : une
déclaration complète terminée par un point (`soit n ∈ ℕ tel que n = 7.`), que
le message invite à retirer, et un bloc dont l'accolade fermante manque,
signalé à sa ligne d'ouverture. Le contenu d'un bloc non fermé reste lisible
comme source ; docdg n'invente pas une fermeture d'accolade. Si le moteur de composition ne peut plus composer
le document, un aperçu de secours conserve la source lisible et les diagnostics
dont l'origine est connue. Ces aperçus provisoires ne remplacent pas le cache
d'une composition valide. Les contrôles de qualification restent stricts.

**Tout s'insère en direct.** Il n'y a ni bouton « Insérer » ni bouton
« Appliquer ». Choisir « gras » écrit `<gras>{}` dans la source, le curseur
entre les accolades ; tapez votre texte, puis choisissez « bleu marine » et la
balise devient `<bleu marine gras>{votre texte}` sans que rien de ce que vous
avez tapé ne bouge, et le curseur reste où il était. Les réglages du document
se réappliquent de même, en réécrivant le bloc `document { }` en place.

**L'éditeur tient le retrait pour vous.** Un retour à la ligne reprend le
retrait de la ligne précédente, et l'augmente d'une tabulation après une
accolade ouvrante ; une accolade fermante tapée sur une ligne encore vide
retire cette tabulation. Dans un tableau, la tabulation et le retour à la
ligne **réalignent les colonnes** de tout le tableau (détails au chapitre des
tableaux).

**Les accolades suivent l'usage.** En début de ligne, la balise encadre un
bloc : l'accolade fermante passe à la ligne suivante et le curseur se place sur
la ligne intercalaire, de sorte que ce qu'on voit à l'écran ait la forme de ce
qui sera composé. Au milieu d'une phrase, il s'agit d'une chaîne dont on change
le style, et les accolades restent sur la ligne. Elles disparaissent d'
elles-mêmes quand la balise porte un niveau de titre, qui prend le texte qui le
suit.

**Rendez la main à la source pour continuer.** Une liste déroulante garde le
clavier tant qu'on n'en est pas sorti : après une insertion, **cliquez dans la
source** avant de taper. C'est aussi ce clic qui clôt l'élément inséré — tant
qu'il reste ouvert, un nouveau choix dans le classeur le réécrit au lieu d'en
ajouter un second, ce qui est justement ce qu'on veut pour composer une balise
en plusieurs touches. Un témoin lumineux montre où le curseur reprendra, et le
bandeau en rappelle la ligne et la colonne.

**Chaque objet s'insère avec un exemple qui marche.** Plutôt qu'une amorce vide
où la syntaxe reste à deviner, le classeur écrit un modèle complet et renseigné
que vous modifiez : un tableau vient avec sa ligne de pistes, ses entêtes
fusionnées et ses cellules de données ; une grille avec son plan de largeurs,
ses zones nommées et leurs blocs de placement ; matrice avec ses
coefficients, un système avec ses équations, un graphe avec ses arcs, une frise
avec ses bornes et ses événements. Les tableaux se règlent au passage —
colonnes, groupes, rangées — et les grilles par leur couleur de bordure et leur
écart.

**Les actions écrivent ce qu'elles supposent.** Un tableau de signes se dresse
pour une fonction, encore faut-il que cette fonction existe. Le classeur écrit
donc les deux lignes d'un coup, et seulement si la déclaration manque :

```docdg
<Soit>f une fonction définie par f(x) = x² − 5x + 6
<Dresser>le tableau de signes de f
```

Il en va de même pour les variations, les statistiques, les matrices : la
déclaration précède toujours ce qui l'emploie.

**Enregistrez ou ouvrez une fois, docdg s'occupe du reste.** Vous pouvez taper
directement dans la source sans rien ouvrir — mais tant qu'aucun fichier n'est
désigné, docdg n'a nulle part où écrire, et le ruban Fichier vous le signale.
Dès le premier **Ouvrir** ou le premier **Enregistrer sous**, l'enregistrement
automatique s'active : le document est réécrit tout seul une seconde après
votre dernière frappe. C'est le seul geste d'administration à retenir, et il ne
se fait qu'une fois par document.

## **📝 La syntaxe**

> **La règle d'or, une fois pour toutes.** Une action s'écrit **`<Action>`complément** : le verbe seul dans la balise, le complément à l'extérieur, en français complet — articles compris et nom de l'objet rappelé.
>
> ```docdg
> <Résoudre>l'équation 3x + 5 = 17
> <Tracer>la fonction f
> <Calculer>l'aire du disque de rayon 3
> ```
>
> Quand l'action porte sur un contenu de plusieurs lignes, celui-ci suit entre accolades : `<Action>complément {` … `}`. Les attributs se disent avec leurs articles (« avec **des** bordures », « **un** écart de 3 mm »), et un objet déclaré se rappelle par sa catégorie (« le système S », « la fonction f » — jamais le nom seul). Les options éventuelles d'une action restent dans la balise, après le verbe. Et parce que l'objectif est d'écrire moins : **la rédaction pas à pas est le comportement, pas une option**. `<Résoudre>le système s` déroule le pivot, `<Calculer>1/2 + 1/3` détaille la mise au même dénominateur — il n'y a rien à demander. L'écriture au chevron englobant, `<Action complément>`, se compile de façon identique, mais toute la documentation et tous les exemples suivent la règle ci-dessus : une seule forme à apprendre, une seule forme à relire.


La syntaxe de docdg repose sur **quatre familles de commandes** :


| **Famille**    | **Rôle**             | **Exemples**                      |
| -------------- | -------------------- | --------------------------------- |
| **Actions**    | Opérations           | `<Tracer>`, `<Soit>`, `<Afficher>`, `<Dresser>`  |
| **Objets**     | Éléments du document | `un cadre`, `un tableau`, `une image`      |
| **Propriétés** | Personnalisation     | `rouge`, `gras`, `14 pt`, `au centre` |
| **Valeurs**    | Paramètres           | `oui`, `non`, `30`, `bleu foncé`  |

**Une seule façon de parler à docdg : la phrase.** Une balise se lit dans l'ordre où elle s'écrit — l'**action** d'abord (le verbe, entre chevrons), puis l'**objet** qu'elle concerne, avec son article et ses attributs en prose, puis enfin le **contenu**, entre accolades : `<Action>objet avec propriétés{contenu}`. Le chevron se referme le plus souvent tout de suite après l'objet et ses attributs — `<Afficher>un cadre avec ...{...}` — mais pour les objets qui se nomment en vue d'un calcul ultérieur (matrice, un système), le chevron peut aussi se refermer tout de suite après le verbe seul, l'objet et son nom suivant en toutes lettres : `<Soit>M une matrice définie par {...}` (voir la section **Matrices et systèmes**) ; les deux s'y compilent de façon identique. Toute balise d'action commence par un **verbe à l'infinitif** — l'usage du secondaire comme du supérieur — jamais par l'objet nu (règle n°12) ; seules font exception la déclaration `<Soit>` et les balises de style (`<gras>`, un style nommé), qui ne commandent aucune action. Les attributs se disent **en prose**, introduits par `avec`, avec le nom et son article (`avec une bordure rouge, un fond jaune clair et des coins arrondis de 5 mm`) — l'ancienne écriture en couples `attribut:valeur` n'existe plus, hors des rares réglages techniques signalés comme tels (`x:{...}` des tableaux de signes, par exemple). Convention du manuel : quand un chapitre étudie un objet en particulier, la tête de balise s'écrit soit avec un verbe (`<Dresser>un tableau ...`), soit avec trois points qui tiennent la place de n'importe quel verbe d'action : `<... un tableau ...>`.

### **📌 Les règles de base**

1. **Les balises** sont entre **chevrons** et commencent par un **verbe** : `<Afficher>un cadre`, `<Dresser>un tableau`, `<Insérer l'image IMAGES/photo.png avec une largeur de 30 mm>`  (règle n°12)
2. **Le contenu** est entre **accolades** : `<Afficher>un cadre{Du texte ici}`. Quand il s'étend sur plusieurs lignes, il **commence à la ligne qui suit l'accolade**, jamais à sa droite — comme le corps d'un bloc dans tout langage, et pour que la source garde la forme du rendu. Ce qui est écrit à droite d'une accolade ouvrante est signalé ; seuls des espaces ne comptent pas pour du contenu, et une construction qui tient sur une seule ligne n'est pas concernée. docdg n'a pas de syntaxe de commentaire : `%` désigne l'opérateur modulo dans les calculs.
3. **Les noms composés** utilisent des **espaces** : `<table des matières>`
4. **Les propriétés** s'écrivent **en français** : `<bleu foncé au centre 14 pt>{Titre}`
5. **Les couleurs** : `rouge`, `bleu foncé` (français)
6. **Les polices** : en **MAJUSCULES** : `<ARIAL>`
7. **Échappement** : doublez les caractères spéciaux pour les afficher littéralement : `<<` → `<`, `>>` → `>`, `{{` → `{`, `}}` → `}`, `##` → `#`, `$$` → `$`.
8. **Les séparateurs** : le point-virgule `;` sépare **les éléments d'une donnée** — coordonnées (`A(2;3)`), bornes d'intervalles (`[-5 ; 5]`), séries de nombres (`{12 ; 15 ; 9}`) ou de textes (`{"lundi" ; "mardi"}`), arguments d'une fonction (`arrondi(5/3; 2)`). La virgule `,` sépare les fragments de prose (règle n°10) et reste la virgule décimale à l'intérieur d'un nombre (`3,5`). La barre `|` sépare les paires `clé: valeur` des données nommées (`{Marche: 5 | Bus: 3}`) et les colonnes des rangées saisies. Deux constructions suivent l'usage mathématique plutôt que cette répartition : les paramètres d'une signature se séparent par la virgule (`(a, b) ↦ a − b`), et un dictionnaire lie chaque clé à sa valeur par deux-points, ses entrées restant séparées par le point-virgule (`{"Marche": 5 ; "Bus": 3}`).

9. **Majuscule de phrase** : **Seules** les actions dont l'écriture se lit comme une phrase française (`<Soit>`, `<Tracer>`, etc.) commencent par une majuscule. Les **mots-clés techniques** (`soit`, `pour`, `si`, `tant que`) restent en minuscules.
10. **Article + prose** : toute action-phrase (`<Soit>`, `<Tracer>`) désigne ses objets avec un article et décrit leurs attributs en prose. En détail :

   - **L'article porte l'intention.** `<Soit>` introduit du nouveau : le type vient **après** le nom, avec l'article **indéfini** (`A(2;7) un point`). Au pluriel, la balise `<Soit>` reste invariable dans la source et la phrase imprimée s'accorde d'elle-même (« Soient A et B deux points… ») ; l'article **cède la place au nombre**, en chiffres : `<Soit>A(-1,5;3), B(2;5) et C(0;-4,3) 3 points` — une seule phrase pour trois points, et docdg vérifie que le compte annoncé correspond au nombre de noms. Le nombre ne se met jamais avant les noms : `<Soit>3 points A, B et C …` est refusé, avec la forme corrigée. Enfin, la **distribution respective** — l'idiome des énoncés du secondaire et du supérieur (« de coordonnées respectives », « d'unités respectives ») — sépare les noms des valeurs et les apparie dans l'ordre : `<Soit>A, B et C 3 points de coordonnées respectives (1;2), (-1;2) et (-1;-2)`. `<Tracer>` prend l'article **défini** pour un objet déjà posé (`le point A`) et l'**indéfini** quand il crée la figure au moment du tracé (`un triangle ABC équilatéral, de côté 5 cm`). Les deux formes produisent le même tracé. L'article se met devant chaque nom — objets compris : `<Afficher>un cadre avec une bordure rouge…`, `<Afficher>une grille…`.
   - **Les qualificatifs** restent accolés au nom sans « de » : `équilatéral`, `rectangle en A`.
   - **Les propriétés à antonyme court** se disent en prose : `avec les marques` / `sans marques`, `avec les valeurs` / `sans valeurs`. `avec` prend l'article, `sans` ne le prend pas — comme en français (`avec du sucre` / `sans sucre`). `avec` sert aussi à introduire un groupe d'attributs : `avec un fond jaune clair, des coins arrondis de 5 mm et un titre {Attention}`.
   - **La couleur** se dit avec `en` : `en bleu`, `en rouge foncé`.
   - **L'épaisseur du trait** se dit `de trait N mm` (`de trait 0,5 mm`) — c'est une mesure : son unité est obligatoire, comme toutes les unités de longueur.

11. **Les attributs d'un objet s'ouvrent par « avec »**, se séparent par une virgule, et les deux derniers se relient par « et ». C'est la seule forme admise ; écrire les attributs sans `avec` est refusé avec un message qui rappelle la règle. Chaque attribut porte de préférence son **nom précédé d'un article** — plus proche du langage naturel que l'adjectif seul :

    ```docdg
    <Afficher un cadre avec un fond vert menthe, une bordure bleu canard,
        des coins arrondis de 3 mm et un titre {Théorème}>{ ... }
    ```

    L'adjectif `bordé de` n'existe plus : la couleur de bordure se dit `une bordure bleu canard`, comme le fond se dit `un fond vert menthe` — le nom précédé de son article (`une bordure`, `un fond`) se lit mieux que l'adjectif, et docdg ne garde qu'une forme canonique par concept. Les qualificatifs qui font corps avec l'objet restent des adjectifs (`un triangle rectangle`, `un triangle équilatéral`). Un objet **sans** attribut s'écrit nu (`<Afficher>un cadre{...}`, `<Dresser>un tableau{...}`).
   - **Le connecteur est obligatoire** entre deux attributs de prose : une virgule entre fragments, ou `et` au sein d'un fragment — `en rouge et de trait 0,5 mm`, jamais la juxtaposition nue. Seule la charnière vers un réglage `clé:valeur` s'accroche sans connecteur.
   - **Les unités de longueur** (`cm`, `mm`) sont **obligatoires** : `de côté 5,3 cm`, jamais `de côté 5,3`. Une longueur sans unité est une erreur, dite en français. *Seule exception :* dans un repère, les longueurs se comptent en **graduations** et s'écrivent sans unité.
   - **Le clé:valeur** ne garde que les réglages purement techniques sans tournure française courante. Chaque fois qu'un nom français existe, la prose gagne : `avec 200 échantillons` plutôt qu'un réglage abstrait, `un pas de $pi/6$` plutôt qu'un `step:`.

12. **Une balise d'action commence par un verbe à l'infinitif** — `<Dresser>un tableau ...`, `<Afficher>un cadre ...`, `<Insérer l'image ...>`, `<Construire>un arbre ...`, `<Tracer>...` — **jamais par l'objet nu**. L'objet porte toujours son article, et l'article suit un verbe : `<Afficher>un cadre` et non `<cadre>`. Écrire l'objet nu est refusé avec un message qui rappelle la règle. Quatre familles seulement échappent à l'infinitif : la **déclaration** `<Soit>`, formule consacrée du discours mathématique, et trois familles qui ne posent aucun objet dans le document : les **styles** (`<gras rouge 14 pt>`), les **commandes structurelles** (`<section>`, `<sous-section>`, `<table des matières>`) et les **définitions d'alias** (`soit exo{n} = <Afficher>un cadre avec ...`, où le membre droit décrit un objet sans encore l'afficher — le verbe y est le bienvenu mais la définition seule ne produit rien). Les verbes d'objet interchangeables sont `Afficher`, `Dresser`, `Construire` et `Insérer` ; le manuel écrit `<... un tableau ...>` quand le verbe importe peu.


### **🔹 Structure d'une balise**

```
<Action>objet avec propriétés {
  CONTENU
}
```

**Exemple :**

```docdg
<Afficher>un cadre avec une bordure rouge, un fond jaune clair, des coins arrondis de 5 mm et un titre {Attention} {
  <gras 14 pt>{Exercice important !}
}
```

---

## **📦 Les objets**

### **1️⃣ `<Afficher>un cadre` — Mettre en valeur du contenu**

**Syntaxe :** `<Afficher>un cadre PROSE{CONTENU}`

**Propriétés complètes :**


| **Propriété**            | **S'écrit**                          | **Défaut**       | **Description**                          |
| ------------------------ | ------------------------------------- | ------------------ | ----------------------------------------- |
| Couleur de la bordure     | `une bordure <couleur>`              | `gris`              | Couleur de la bordure                     |
| Couleur du texte / fond   | `texte <couleur>`, `fond <couleur>`  | `texte noir, fond blanc` | Couleur du texte et du fond du cadre |
| Coins arrondis            | `coins arrondis de <N>mm`            | pas d'arrondi       | Rayon des coins                           |
| Largeur                   | `large de <N>mm` / `large de N%`     | auto                | Largeur du cadre                          |
| Épaisseur de la bordure   | `bordure épaisse de <N>mm`           | `0,4 mm`             | Épaisseur du trait                        |
| Marge intérieure          | `marge de <N>mm`                     | `3 mm`               | Marge intérieure                          |
| Titre                     | `avec un titre {...}`                        | pas de titre        | Titre affiché en haut du cadre            |
| Couleur du titre          | `titre en <couleur texte> sur fond <couleur fond>` | -      | Couleur du texte et du fond du titre      |
| Coupure entre pages       | `sécable` / `insécable`              | `insécable`         | Autorise le cadre à se poursuivre d'une page à l'autre |


`texte <couleur>` et `fond <couleur>` sont deux fragments indépendants de la même prose, séparés par une virgule (règle n°8) : on peut n'écrire que l'un des deux, l'autre garde sa valeur par défaut. Même principe pour `titre en ... sur fond ...`.

**Un cadre recto/verso.** Une ligne ne contenant que `---` coupe le cadre en deux parties, l'énoncé au-dessus, la solution en dessous — pratique pour une fiche de révision ou une carte question/réponse :

```docdg
<Afficher>un cadre avec une bordure bleu marine et un titre {Question}{
  Quelle est la dérivée de $x \mapsto x^2$ ?
  ---
  $2x$
}
```

**Exemples :**

```docdg
<Afficher>un cadre avec une bordure bleue, un fond bleu Alice et des coins arrondis de 4 mm{
  <gras 14 pt au centre>{Exercice 1}
  Résoudre $x^2 - 5x + 6 = 0$.
}

<Afficher un cadre avec une bordure rouge foncé, un fond blanc, des coins arrondis de 6 mm,
  un titre {Théorème de Pythagore} et un titre en blanc sur fond rouge foncé>{
  Dans un triangle rectangle...
}
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| Précision | Cadre simple |
| Précision | Cadre avec titre |


### **2️⃣ `<Dresser>un tableau` — Tableaux simplifiés**

**Le tableau commence toujours par sa ligne de pistes** : une piste par colonne, entre chevrons, séparées par des tabulations. Une piste porte un code de placement à deux lettres (obligatoire), et peut y ajouter une largeur avec `en` :

```docdg
<Dresser>un tableau {
	<35 mm en mg>	<mg>
	Champ	Saisie
	Nom :	[Nom :]
	[Prénom :]
	Adresse :	[Rue et numéro :]
	[Ville et code postal :]
}
```

- `<35 mm en mg>` : largeur fixée à 35 mm, aligné au milieu à gauche.
- `<mg>` : pas de largeur déclarée — cette colonne, comme toute colonne sans largeur, reçoit une part égale de ce qu'il reste, jusqu'à occuper toute la largeur disponible (celle de la page si le tableau n'a pas de largeur propre).
- Il n'y a pas de hauteur de cellule : une cellule prend la hauteur qu'exige son contenu.

**Trois règles suffisent, après la ligne de pistes :**

1. **Hors des crochets, tout est entête** — la rangée du haut comme la colonne de gauche.
2. **Chaque donnée est entre crochets**, une paire par cellule, séparées par une ou plusieurs tabulations : `[Nom]	[15]`, jamais de point-virgule à l'intérieur.
3. **Les fusions se lisent dans l'entête** : un tiret en fin de mot fusionne horizontalement, un deux-points fusionne verticalement — voir plus bas.

> **⇥ Les colonnes s'alignent d'elles-mêmes.** Dans le volet Source, la tabulation et le retour à la ligne réalignent tout le tableau : chaque colonne démarre au même endroit sur toutes les rangées, les données d'une rangée de continuation se rangent sous leur colonne, et une tabulation en fin de rangée place le curseur sous la colonne suivante. Un tableau collé ou inséré depuis le bandeau arrive déjà aligné. Seules les tabulations bougent — la grammaire en admet une ou plusieurs —, si bien que le document lu ne change pas d'un iota ; une rangée en cours de frappe (crochet encore ouvert) est laissée telle quelle, et un seul Cmd-Z (Ctrl-Z) annule la frappe et l'alignement ensemble.

```docdg
<Dresser>un tableau {
	<mc>	<mg>	<md>
	Jour	Matière	Note
	[Lundi]	[Maths]	[15]
	[Mardi]	[EPS]	[16]
}
```

La première rangée, hors crochets, est l'entête, en gras, sans que le mot `entête` figure nulle part.

**Fusion verticale : le deux-points.** Une entête suivie d'un deux-points porte les rangées de données qui suivent immédiatement sur la même ligne — un seul deux-points suffit, quel que soit le nombre de rangées couvertes ; sa portée se lit à ce qui la suit, pas à un décompte :

```docdg
<Dresser>un tableau {
	<mc>	<mc>	<mg>
	Semaine	Jour	Matière
	S1 :	Lundi :	[Maths]
	[Français]
	Mardi :	[EPS]
	S2 :	Lundi :	[Maths]
}
```

« S1 » couvre trois rangées, « Lundi » deux : chacun couvre ce qu'il porte. Le deux-points est consommé, jamais imprimé — mais un deux-points **à l'intérieur d'une donnée** (`[Nom :]`) n'est que du texte : la grammaire ne regarde le deux-points qu'en dehors des crochets.

**Fusion horizontale : le tiret après l'entête.** Un ou plusieurs tirets après une entête la font couvrir une colonne de plus par tiret. L'espacement ne compte pas — `Identité-`, `Identité -` et un tiret seul dans sa colonne disent la même chose, les tabulations ne servant qu'à aligner la source à l'œil — alignement que l'éditeur fait lui-même. Sans risque pour un mot composé : le tiret de fusion n'est jamais suivi d'une lettre, si bien que « Sous-total » reste un seul mot :

```docdg
<Dresser>un tableau {
	<mc>	<mc>	<mc>	<mc>
	Identité -	Coordonnées
	Nom	Prénom	Ville	Téléphone
	[Dupont]	[Iris]	[Lyon]	[06 12 34 56 78]
}
```

« Identité - » couvre 2 colonnes ; « Coordonnées », dernier mot de la rangée, s'étend jusqu'au bord par la règle qui suit — inutile d'y ajouter un tiret. Le tiret peut aussi s'écrire seul dans la colonne qu'il couvre, pour que la source ait la forme du tableau :

```docdg
<Dresser>un tableau {
	<mc>	<mc>	<mc>	<mc>
	Sous-total	-	-	Reste
	Janvier	Février	Mars	Avril
	[12]	[15]	[9]	[14]
}
```

**Fusion horizontale implicite : la rangée courte.** Une rangée d'entête plus courte que le tableau, et qui ne se termine ni par un tiret ni par un deux-points, étend sa dernière cellule jusqu'au bord droit — un titre seul couvre toute la largeur :

```docdg
<Dresser>un tableau {
	<mc>	<mg>	<md>
	Bulletin du premier trimestre
	Jour	Matière	Note
	Lundi :	[Maths]	[15]
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

Employé seul, sans piste vertical/horizontal explicite, `au centre` vaut `mc`, `à gauche` vaut `mg`, `à droite` vaut `md` — la forme en toutes lettres n'est jamais moins correcte que le code à deux lettres, c'est un raccourci, pas une syntaxe distincte : `<au milieu à gauche>` et `<mg>` sont la même piste.

**Options, dans la description de la balise :**

| **Option**              | **S'écrit**                          | **Défaut**    | **Description**                              |
| ----------------------- | --------------------------------------- | ------------- | ---------------------------------------------- |
| Couleur de la bordure    | `une bordure <couleur>`               | `noir`        | Couleur des filets, tracés d'office             |
| Couleur des cellules     | `texte <couleur>`, `fond <couleur>`   | -             | Couleur du texte et du fond des cellules        |
| Couleur des entêtes      | `des entêtes en <couleur texte> sur fond <couleur fond>` | -   | Couleur de toutes les cellules d'entête         |
| Écart entre cellules     | `écart de <N> mm`                     | `0 mm`        | Espacement entre cellules                       |
| Coupure entre pages      | `sécable` / `insécable`               | `insécable`   | Autorise le tableau à se poursuivre d'une page à l'autre ; la rangée d'entête se répète en tête de chaque fragment |

Mêmes conventions que pour `<Afficher>un cadre` : `texte <couleur>` / `fond <couleur>`, chaque partie facultative.

**Exemple complet :**

```docdg
<Dresser un tableau avec un écart de 3 mm,
  une bordure bleu marine, un fond bleu Alice et un entête en blanc, sur fond bleu marine>{
	<mc>	<mg>	<mc>
	Jour	Matière	Note
	Lundi :	[Mathématiques]	[15]
	Mardi :	[Français]	[12]
}
```

**Boucles et variables dans une cellule.** Une cellule accepte tout ce qu'accepte le corps du document, `#variable` et boucle `pour` comprises — un tableau peut donc se construire depuis un conteneur :

```docdg
<Dresser>un tableau {
	<mg>	<md>
	Jour	Note
	pour j ∈ {"Lundi" ; "Mardi" ; "Mercredi"} {
		[#j]	[#{aléatoire(0 ; 20)}]
	}
}
```

Pour écrire des crochets littéraux dans une cellule, les protéger par des dollars : `$[AB]$`.


### **3️⃣ Dénombrement, arithmétique, complexes**

```docdg
<Dénombrer>les combinaisons de 3 parmi 10
<Dénombrer>les arrangements de 3 parmi 10
<Dénombrer>les permutations de 5
<Dresser>le triangle de Pascal jusqu'à la ligne 5
```

Le calcul est posé avant d'être fait : la formule s'affiche, puis le résultat.

```docdg
<Dresser>la décomposition en facteurs premiers de 360
<Appliquer>l'algorithme d'Euclide à 84 et 60
```

La décomposition donne aussi le nombre de diviseurs ; Euclide déroule ses divisions, conclut sur le PGCD et le PPCM, et fournit une relation de Bézout.

### **3️⃣ Le programme de calcul**

Le collège pose un programme de calcul, le fait tourner sur un nombre, puis le met en formule. Les deux verbes lisent le **même corps**, une étape par ligne :

```docdg
<Appliquer>le programme de calcul à 5 {
	choisir un nombre
	ajouter 3
	multiplier par 2
	soustraire 4
}

<Exprimer>le programme de calcul en fonction de x {
	choisir un nombre
	ajouter 3
	multiplier par 2
	soustraire 4
}
```

`<Appliquer>` déroule les étapes une à une, en montrant chaque passage ; `<Exprimer>` compose l'expression littérale et la réduit.

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
<Calculer>le périmètre du rectangle de longueur 7 cm et de largeur 4 cm
<Calculer>le périmètre du triangle de côtés 3, 4 et 5
<Calculer>le périmètre du cercle de rayon 5
<Calculer>l'aire du carré de côté 6
<Calculer>l'aire du triangle de base 6 et de hauteur 4
<Calculer>l'aire du disque de rayon 3
<Calculer>le volume du pavé droit de longueur 5, de largeur 3 et de hauteur 2
<Calculer>le volume du cylindre de rayon 2 et de hauteur 5
<Calculer>le volume du cône de rayon 3 et de hauteur 4
```

La formule est posée, puis appliquée. L'unité est facultative ; lorsqu'elle est
donnée, elle suit la dimension — `cm` pour un périmètre, `cm²` pour une aire,
`cm³` pour un volume.

### **3️⃣ Les anneaux de factorisation**

`<Factoriser>` travaille par défaut dans $\mathbb{R}$. Trois compléments l'en font sortir :

```docdg
<Factoriser>x^4 - 1 dans les complexes
<Factoriser>x^4 - 1 dans C[X]
<Factoriser>x^2 + 1 dans Q[X]
```

`dans les complexes` et `dans C[X]` sont la même demande. Dans $\mathbb{R}$ et dans $\mathbb{C}$, la factorisation va jusqu'aux facteurs irréductibles lorsque les coefficients sont des nombres et que les racines gardent une écriture courte (`x^2 - 2`, `x^2 - pi`, `x^4 + 1`) ; sinon docdg rend la factorisation rationnelle et dit que le facteur restant n'a pas été décomposé — c'est une limite du moteur, non un résultat sur le polynôme. `<Résoudre>` reçoit les mêmes compléments, plus `dans les entiers` (ou `dans ZZ`) pour une équation diophantienne.

```docdg
<Écrire>le complexe z = 1 + i sous ses trois formes
```

Formes algébrique, trigonométrique et exponentielle, avec module exact (`√2` reste `√2`) et argument en fraction de π quand il tombe juste.

### **3️⃣ `<Dresser>la table de Cayley du groupe ...`**

```docdg
<Dresser>la table de Cayley du groupe Z/4Z
<Dresser>la table de Cayley du groupe (Z/8Z)*
```

Addition modulo *n* pour `Z/nZ`, multiplication des inversibles pour `(Z/nZ)*` : la table est calculée, pas saisie.

### **3️⃣ Lycée : vecteurs, lois, suites**

```docdg
<Écrire>le vecteur u de coordonnées (3 ; -2)
```

Notation colonne, deux ou trois coordonnées, fractions acceptées. Le nom est facultatif.

```docdg
<Dresser>la loi de probabilité de X{
	valeurs : [1 ; 2 ; 3]
	probabilités : [1/6 ; 1/3 ; 1/2]
}
```

Le tableau se dresse tout seul. La somme des probabilités est vérifiée : une loi qui ne totalise pas 1 est refusée, avec le total fautif dans le message.

```docdg
<Soit>u une suite définie par u(0) = 1 et u(n+1) = 2u(n) + 1

<Calculer>les 6 premiers termes de u
```

La définition s'affiche en notation indicielle, puis les termes se calculent à la demande.

### **3️⃣ `<Étudier>le second degré` — Trinômes**

Le trinôme se donne tel qu'il s'écrit, un par ligne. L'étude sort complète : discriminant, racines, forme factorisée, forme canonique.

```docdg
<Étudier>le second degré{
	2x^2 - 3x + 1
	x^2 - 4x + 4
	x^2 + x + 1
}
```

Les fractions restent exactes tant que le discriminant est un carré parfait ; sinon le radical est conservé et une valeur approchée l'accompagne. La variable se précise si besoin : `<Étudier>le second degré en t`.

### **3️⃣ `<Insérer>une image` — Insertion d'images**

**Syntaxe :** `<Insérer l'image CHEMIN avec ATTRIBUTS>` — le chemin s'écrit **en entier dans la balise**, relatif au fichier du document ; rien ne reste à accoler après. Un nom comportant des espaces s'accolade : `l'image {mon chat.png}`. Les dimensions se disent **en prose**, comme tout attribut (règle n°11), l'unité (mm) obligatoire. docdg ne cherche jamais dans un dossier par défaut : omettre le chemin est refusé avec un message qui rappelle la forme.

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
<Insérer l'image IMAGES/diagramme.png avec une largeur de 50 mm>
<Insérer l'image IMAGES/photo.png avec une largeur de 80 mm, une hauteur de 60 mm et la légende {Figure 1}>
<Insérer l'image IMAGES/GEOMETRIE/cercle.png avec un côté de 30 mm>
```

### **4️⃣ `<Dresser>une liste ADJECTIF` — Listes variées**

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
<Dresser>une liste à puces{
pommes
poires
<gras>{cerises}
}

<Dresser>une liste numérotée Rouge 14 pt{
Premier point
Deuxième point
}
```

### **5️⃣ `<Afficher>une grille` — Mise en page avancée**

**La grille s'ouvre par son plan** : une ligne de largeurs au-dessus des colonnes, puis une ligne par rangée, qui commence par sa hauteur et nomme ses zones. Le plan se lit à l'œil, comme un croquis tracé à la main, et une ligne vide le sépare des zones :

```docdg
<Afficher>une grille avec un écart de 3 mm {
	<24 mm>	<max>	<20 mm>
	<min>	titre	titre	date
	<10 cm>	consigne	corps	corps
	<min>	logo	corps	corps

	titre : en haut à gauche, avec un fond bleu Alice et une bordure bleu marine {
		<bleu marine gras 14 pt>Contrôle de mathématiques
	}
	date : en haut à droite {
		Le 12 mai
	}
	consigne : au centre, avec une bordure bleue et un fond gris perle {
		Calculatrice autorisée.
	}
	logo : au centre {
		<Insérer l'image IMAGES/oiseau.png avec une largeur de 20 mm>
	}
	corps : au centre {
		Résoudre $x^2 - 5x + 6 = 0$ et vérifier les deux solutions.
	}
}
```

**Les pistes**, entre chevrons, disent la place :

| S'écrit   | Signifie                                                    |
| --------- | ----------------------------------------------------------- |
| `<min>`   | au plus juste : la zone prend ce qu'exige son contenu        |
| `<max>`   | le reste de la place, partagé à parts égales entre les `max` |
| `<24 mm>` | une largeur ou une hauteur fixée (`mm`, `cm`)                |

La hauteur d'une rangée est facultative : sans elle, la rangée vaut `<min>`.

**Un nom de zone par case, un point pour une case vide.** Une zone qui occupe plusieurs cases répète son nom — « titre » sur deux colonnes, « corps » sur un carré de deux colonnes et deux rangées. Le point laisse une case vide :

```docdg
<Afficher>une grille avec un écart de 4 mm {
	<max>	<10 mm>	<max>
	<min>	nom	.	date
	<15 cm>	corps	corps	corps

	nom : à gauche { Nom : }
	date : à droite { Date : }
	corps : au centre { ... }
}
```

**Le plan se vérifie.** Une zone doit dessiner un rectangle plein ; une zone décrite mais absente du plan, ou annoncée dans le plan mais jamais décrite, est signalée à l'endroit même de la grille.

**Chaque zone se décrit ensuite** : son nom, un deux-points, son **placement** (`au centre`, `en haut à gauche`, ou le code à deux lettres `mc`, `hg`…), puis ses **attributs** après `avec`, séparés par des virgules, `et` avant le dernier — la même convention que `<Afficher>un cadre` et `<Dresser>un tableau` (règle n°11). Employé seul, `au centre` vaut `mc`, `à gauche` vaut `mg`, `à droite` vaut `md`. Une zone peut ainsi surcharger `une bordure <couleur>` et ajouter `un texte <couleur>, un fond <couleur>`.

**Options de la balise :**

| **Propriété**            | **S'écrit**                          | **Défaut** | **Description**                                                  |
| ------------------------- | --------------------------------------| ---------- | ------------------------------------------------------------------|
| Bordures                   | `des bordures`, une couleur peut suivre | absentes   | Affiche une bordure autour de chaque zone (`des bordures orange`)  |
| Couleur de la bordure      | `une bordure <couleur>`              | `gris`     | Couleur des bordures (par défaut, surchargeable par zone)          |
| Couleur du contenu         | `texte <couleur>`, `fond <couleur>`  | `texte noir, fond blanc` | Couleur du contenu (par défaut, surchargeable par zone) |
| Écart                      | `écart de <N> mm`                    | `3 mm`     | Espacement entre zones                                              |


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

Sept niveaux, deux natures. Les trois premiers sont des **divisions majeures** :
chacune prend une page à elle seule. Les quatre autres sont des divisions
courantes insérées dans le texte.

**Toute division est numérotée automatiquement**, comme dans LaTeX. Les cinq options explicites
`num`, `roman`, `Roman`, `alpha`, `Alpha` servent à réaffirmer la numérotation décimale
(`num`) ou à en modifier la forme. Pour supprimer le numéro d'une division, écrire
**`sans numéro`**, par exemple `<section sans numéro>` (équivalent de `\section*{...}`).
Le titre non numéroté conserve son ancre de navigation, mais n'incrémente pas les compteurs.

| Division | Équivalent LaTeX | Par défaut (ou `num`) | Avec `sans numéro` |
|---|---|---|---|
| `tome` | grande division dérivée de `\part` | Tome I | Titre seul |
| `livre` | grande division dérivée de `\chapter` | Livre 1 | Titre seul |
| `partie` | `\part` | Partie I | Titre seul |
| `chapitre` | `\chapter` / `\chapter*` | 1 — Titre | Titre |
| `section` | `\section` / `\section*` | 1.1 — Titre | Titre |
| `sous-section` | `\subsection` / `\subsection*` | 1.1.1 — Titre | Titre |
| `sous-sous-section` | `\subsubsection` / `\subsubsection*` | 1.1.1.1 — Titre | Titre |

```docdg
<chapitre>Analyse
<section>Fonctions
<sous-section>Continuité
<section sans numéro>Exercices complémentaires
```

Les trois premiers titres sont numérotés `1`, `1.1` et `1.1.1` ; le dernier
ne porte aucun numéro. Les formes `sans numéro` et `roman` / `Roman` / `alpha` / `Alpha`
ne peuvent pas se combiner. Le séparateur visible est un tiret, jamais `°)` pour un titre.

Les cinq formes se distinguent ainsi : `num` produit la numérotation hiérarchique
(`1`, `1.2`, `1.2.3`) ; `roman` et `Roman` produisent le rang courant en
chiffres romains (`i`, `ii` et `I`, `II`) ; `alpha` et `Alpha` produisent le
rang courant en lettres (`a`, `b` et `A`, `B`). Le séparateur reste un tiret.
Seules les variantes `Num` et `NUM` sont refusées, car elles ne définissent
aucune nouvelle forme. Les cinq options sont sensibles à la casse (`Alpha`, mais jamais `ALPHA`).

**Aucune division majeure ne remet à zéro celle qui la suit** — c'est la conduite de la classe `book`, où les chapitres se suivent d'une partie à l'autre. Chaque échelle court sur l'ouvrage entier, aucun numéro ne paraît deux fois, et un renvoi n'a jamais à dire dans quelle partie chercher.

Une étiquette posée sur une division se renvoie **par son seul numéro** : `la partie <renvoi>{support}` donne « la partie I », jamais « la partie Partie I ». Le nom appartient à la division, la phrase appartient à l'auteur.

**Exemple :**

```docdg
soit h0 un style défini par <bleu marine gras chapitre Roman>
soit h1 un style défini par <bleu gras section Alpha>

<tome>Ce que la matière imposait

<partie>La longueur qu'un support autorise

<h0>Le volumen

<h1>Une longueur qui ne se discutait pas
```

Une division se déclare aussi bien par un style nommé — `soit p un style défini par <bleu nuit partie>` — que par son mot nu, comme ci-dessus.

---

## **🧱 La mise en colonnes**

Toutes les pages n'ont pas besoin d'être en colonnes, et c'est pourquoi la colonne n'est pas un réglage du document : **un passage se compose en colonnes, entre deux accolades**, et le reste garde la pleine largeur.

```docdg
<Composer>sur 2 colonnes {
	La colonne étroite est la respiration de la page savante : l'œil y saisit
	la ligne entière sans balayer, et la césure y trouve son plein emploi.

	Les colonnes s'équilibrent d'elles-mêmes : on ne dit pas où couper, on dit
	combien de colonnes, et le moteur répartit.
}
```

**D'une à cinq colonnes**, le nombre s'écrit en chiffres, comme partout dans la source — `sur 2 colonnes` ; l'écriture en toutes lettres est refusée avec la forme attendue. Deux pour un article, trois pour un lexique ou une liste de mots courts.

Le corps est du docdg ordinaire : des paragraphes, des listes, des cadres, des formules. **Ce qui suit l'accolade fermante retrouve la pleine largeur** — un tableau large, une frise, une figure se posent donc entre deux passages en colonnes, sans syntaxe d'échappement.

**L'espacement se règle dans le bloc `document {…}`**, et nulle part ailleurs : `espacements: {haut;droite;bas;gauche}` donne le blanc laissé au bord de la page, en millimètres. Ce blanc vaut pour le bord d'une colonne comme pour le bord de la page, si bien que **la gouttière entre deux colonnes est la somme de l'espacement droit et de l'espacement gauche**. Il n'y a pas de mot pour la gouttière — elle se déduit. Sans espacement, les colonnes se touchent ; avec deux valeurs différentes, on obtient la marge d'un cahier, large à gauche et presque nulle à droite :

```docdg
document {
	orientation: paysage;
	marges: {15;5;15;0};
	espacements: {0;0;0;25};
}
```

**Un filet se demande** et se décrit comme tous les traits de docdg — une couleur, un dessin, une épaisseur :

```docdg
<Composer>sur 3 colonnes avec un filet gris clair {
	Trois colonnes conviennent à un lexique : la ligne est brève, l'œil en
	saisit plusieurs d'un seul coup.

	Le filet n'est jamais obligatoire — l'écart suffit le plus souvent —, mais
	il rend service quand la gouttière est étroite.
}
```

**Un passage plus haut qu'une page se poursuit sur la suivante**, en colonnes de même, sans qu'on ait rien à déclarer : à la différence du cadre, qui est insécable tant qu'on ne le déclare pas sécable, le passage en colonnes se coupe toujours — un texte courant n'a aucune raison de rester d'un seul tenant.

`basique.txt` montre la forme (« Le passage en colonnes »), `publication3.txt` la démonstration typographique : un passage de quatre paragraphes qui traverse une page, filet compris. Une colonne accueille tout ce qu'accueille la page, [réglure Seyès comprise](#-lécriture-en-réglure).

---

## **🧾 Les démonstrations**

Une démonstration s'écrit avec le verbe **`<Montrer>`** et lui seul — la règle d'or s'applique ici comme partout : le verbe dans la balise, le raisonnement et l'énoncé en complément, `<Montrer>par récurrence que …` — dans les manuels du supérieur, « Montrons que » écrase toutes les autres annonces. Le raisonnement direct est la forme nue ; les neuf autres se nomment après le verbe, d'un seul nom chacun, parce que **chaque raisonnement s'annonce avant de se dérouler** — une analyse-synthèse non annoncée, par exemple, ressemble à une pétition de principe pour le correcteur pressé.

Le moteur fournit la charpente — l'annonce, les étapes nommées, la conclusion quand la logique l'exige — et vous n'écrivez que les mathématiques. Les clôtures suivent l'usage réel : seules celles qui **font partie du raisonnement** sont produites (la récurrence invoque son principe, l'absurde conclut de la contradiction, la disjonction constate que les cas couvrent tout), et jamais de « ce qui achève la démonstration ».

| Raisonnement | Écriture | Étapes attendues |
|---|---|---|
| directe | `<Montrer>que …` | — |
| contraposée | `<Montrer>par contraposée que …` | `contraposée{…}` |
| absurde | `<Montrer>par l'absurde que …` | `absurde{…}`, et `contradiction{…}` en option |
| récurrence | `<Montrer>par récurrence que …` | `initialisation{…}`, `hérédité{…}` |
| disjonction de cas | `<Montrer>par disjonction de cas que …` | `cas ⟨description⟩ {…}`, répété |
| analyse-synthèse | `<Montrer>par analyse-synthèse que …` | `analyse{…}`, `synthèse{…}` |
| double implication | `<Montrer>par double implication que …` | `directe{…}`, `réciproque{…}` |
| double inclusion | `<Montrer>par double inclusion que …` | `directe{…}`, `réciproque{…}` |
| propriété universelle | `<Montrer>par élément quelconque …` | `soit{…}` |
| tiroirs | `<Montrer>par le principe des tiroirs que …` | `objets{…}`, `tiroirs{…}` |
| existence et unicité | `<Montrer>l'existence et l'unicité de …` | `existence{…}`, `unicité{…}` |

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
<Montrer>par récurrence que pour tout entier $n$, $somme(k=0;n) k = (n(n+1))/2$ {
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
<Énoncer>le théorème <étiquette>{carre-impair} {
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

### **Trois parties, trois usages**

Les trois parties de `exemples/demonstration.txt` :

| Partie | Ce qu'elle montre |
|---|---|
| « Démonstrations rédigées par l'auteur » | **l'auteur écrit, le moteur charpente** — les raisonnements écrits à la main, plus la preuve logée dans l'énoncé |
| « Démonstrations automatiques : lycée » | **le moteur démontre seul** au lycée — aucun corps entre accolades |
| « Démonstrations automatiques : supérieur » | **le moteur démontre seul** dans le supérieur — suites, séries, structures algébriques, algèbre linéaire, arithmétique, analyse et topologie |

### **Trois sources, une seule rédaction**

Une démonstration peut venir de trois endroits, et sort toujours dans la même langue :

| Source | Quand | Ce qu'elle apporte |
|---|---|---|
| **Vous** | vous écrivez le corps entre accolades | les mathématiques ; le moteur fournit la charpente |
| **Le calcul formel** (SymPy) | pas de corps, et l'énoncé est calculable | la vérification **et** la rédaction, chaîne d'égalités comprise |
| **La bibliothèque** (`demonstrations.json`) | pas de corps, et l'énoncé y figure | les démonstrations d'idée, que rien ne calcule |

**Sans corps, le moteur démontre lui-même** ce qui est à la portée du calcul formel — comme partout dans le langage, l'absence d'accolades signifie que le moteur fait le travail :

```docdg
<Montrer>par récurrence que pour tout entier $n$, $somme(k=0;n) k = (n(n+1))/2$
<Montrer>par élément quelconque réel $x$, $x^2 + 1 >= 2x$
```

La première vérifie l'initialisation et l'hérédité par le calcul, puis rédige la démonstration complète ; la seconde établit l'inégalité par la forme canonique. Une formule fausse est **refusée avec la raison** — le moteur ne démontre pas un mensonge.

**Ce que le calcul n'atteint pas, la bibliothèque le fournit.** Une idée ne se calcule pas : cent une démonstrations classiques — treize au lycée, vingt-neuf en L1, vingt-cinq en L2, trente-cinq en classes préparatoires — sont écrites une fois et appelées par leur seul énoncé. La base ne contient **que** cela — tout ce qu'une identité ou une inégalité vérifiable démontre relève de SymPy, jamais d'une fiche.

```docdg
<Montrer>que $racine(2)$ est irrationnel
<Montrer>qu'il existe une infinité de nombres premiers
<Montrer>l'existence et l'unicité de la division euclidienne
```

La base ne contient **que** ce qu'aucun outil ne sait faire : une idée, une construction, un argument de structure. Tout ce qu'une identité ou une inégalité vérifiable démontre relève de SymPy — deux sources pour un même résultat finiraient par diverger. On y trouve donc l'irrationalité de √2, le théorème de Cantor, la divergence de la série harmonique, la caractérisation d'un sous-groupe ; on n'y trouve ni la formule du binôme ni l'inégalité de Bernoulli, qui se vérifient par le calcul.

L'énoncé est reconnu après normalisation — accents, mathématiques et ponctuation n'y font rien. Un raisonnement précisé dans la balise l'emporte sur celui de la fiche, et un énoncé absent fait proposer les plus proches. La base vit dans `transpiler/src/maths/demonstrations.json` et suit les versions du logiciel : chaque fiche porte son identifiant, ses clés, son niveau, son raisonnement et son corps **en docdg**.

<a id="les-actions"></a>

## **✏️ Les actions**

### **`<Soit>` — Poser des hypothèses de départ**

**Rôle :** `<Soit>` déclare un ou plusieurs éléments ou hypothèses d'un coup (points, valeurs, fonctions, champs, systèmes...) *et les affiche* dans le document sous la forme d'un énoncé « Soit ... ». Une déclaration complète avec `soit` en minuscule est exécutée silencieusement, sans rien afficher (voir « Alias et macros »). Une ligne qui n'en respecte pas la syntaxe reste de la prose, sans alerte. C'est le même verbe sous deux formes — la casse suit la règle n°9.

**Syntaxe (plusieurs éléments) :**

```docdg
<Soit>{
	A(2;-1,5) un point
	B(-4,1;3,4) un point
}
```

**Syntaxe (un seul élément) :** comme pour `<Tracer>`, le contenu suit directement le chevron fermant sans accolades — celles-ci ne servent qu'à regrouper plusieurs lignes.

```docdg
<Soit>A(2;-1,5) un point
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

**L'ordre suit l'usage du supérieur : le nom, puis la nature.** « Soit A le point de coordonnées (2 ; 3) », et non « Soit le point A(2 ; 3) ». Au pluriel, la balise ne change pas et la phrase rendue s'accorde — `<Soit>A et B 2 points de coordonnées respectives …` donne « Soient A et B deux points de coordonnées respectives … » : le nombre s'écrit en chiffres dans la source, où il ne dépend d'aucune orthographe, et se rend en toutes lettres. Chaque déclaration est une **phrase**, non une formule affichée — sauf la fonction, qui se compose hors texte comme au tableau, sur deux lignes alignées :

| Déclaration | Rendu |
|---|---|
| `<Soit>f une fonction définie par f(x) = x² − 2` | Soit *f* la fonction définie par *f*(*x*) = *x*² − 2. |
| `<Soit>f une fonction de ℝ dans ℝ définie par f(x) = x² − 2` | Soit *f* : ℝ ⟶ ℝ, puis à la ligne *x* ⟼ *x*² − 2 |
| `<Soit>f et g 2 fonctions définies par f(x) = x² et g(x) = −x²` | Soit *f* et *g* deux fonctions définies par *f*(*x*) = *x*² et *g*(*x*) = −*x*². |
| `<Soit>u(3;-2) un vecteur` | Soit *u⃗* le vecteur de coordonnées (3 ; −2). |
| `<Soit>M une matrice définie par {…}` | Soit *M* la matrice définie par : puis la matrice hors texte |
| `<Soit>s un système défini par {…}` | Soit (*s*) le système : puis le système hors texte |

Le système garde l'ordre inverse : son nom est une étiquette parenthésée, non le nom propre de l'objet.

**Une fonction se déclare comme on l'écrit au tableau, dans l'un de trois registres.** Le nom, puis son type introduit par l'article, sans virgule — « f une fonction » —, puis sa définition ; le registre dit seulement comment les ensembles sont donnés :

| Registre | Écriture | Ensembles | Rendu |
|---|---|---|---|
| Simplifié | `<Soit>t une fonction définie par t(x) = tan(x)` | implicites | la phrase « Soit t la fonction définie par t(x) = tan(x). » |
| Rédigé | `<Soit>f une fonction de ℝ dans ℝ définie par f(x) = x²` | en mots | « Soit f : ℝ ⟶ ℝ », puis « x ⟼ x² » |
| Formel | `<Soit>f : ℝ → ℝ, x ↦ x²` | en symboles | « Soit f : ℝ ⟶ ℝ », puis « x ⟼ x² » |

Sans ensembles écrits, la convention est celle de la fonction de la variable réelle : définie sur son ensemble de définition, à valeurs dans ℝ (ℝ² au départ pour deux variables) ; `<Déterminer>le domaine de t` le calcule. Avec ensembles, la déclaration se compose hors texte, à gauche, comme un tableau à trois colonnes : la variable calée sous l'ensemble de départ, ⟼ sous ⟶, l'image sous l'ensemble d'arrivée. Dans cette déclaration, le lien est toujours « définie par » — DocDG refuse « telle que » et rappelle la forme admise — et le participe s'accorde. Plusieurs fonctions se déclarent ensemble avec leur nombre en chiffres, que docdg contrôle : `<Soit>f et g 2 fonctions définies par f(x) = x² et g(x) = 2x`. Dans DocDG, le mot-clé `<Soit>` reste invariable (« Soit deux droites ») : `<Soient>`, que la langue admet, y est refusé avec la forme corrigée.

Un nom de plusieurs lettres se compose droit, comme `\operatorname`, jusque dans ses appels, si bien qu'une définition récursive reste lisible ; une définition par morceaux prend l'accolade des cas, une ligne « valeur si condition », la dernière « valeur sinon » :

```docdg
<Soit>factorielle une fonction de ℕ dans ℕ définie par
factorielle(n) = {
    1 si n = 0
    n × factorielle(n − 1) sinon
}
```

Déclarée par `soit` en minuscule, une fonction devient algorithmique : elle se calcule sans s'afficher (`#{factorielle(5)}` vaut 120), et, typage oblige, elle donne toujours ses ensembles — `soit g une fonction de ℤ dans ℤ définie par g(n) = n + 1` ou `soit g : ℤ → ℤ, n ↦ n + 1` ; le registre simplifié lui est refusé. Les ensembles s'écrivent comme on les lit : `ℝ²*+` donne ℝ avec 2 en exposant et ∗+ en indice, en prose comme entre dollars.

Les points ainsi déclarés peuvent ensuite être repris par `<Tracer>` (voir plus bas) sans qu'il soit nécessaire de redonner leurs coordonnées.

**Les points et les vecteurs, sous deux formes.** Comme toute déclaration, celle d'un point suit l'ordre de l'usage : le nom, puis le type introduit par l'article — ou par le nombre en chiffres quand on en déclare plusieurs —, sans virgule entre eux. Les coordonnées se donnent soit **accolées aux noms**, comme on écrit un vecteur u(1 ; 0) — la forme courte des énoncés, « Soit A(1 ; 2), B(−1 ; 2) et C(−1 ; −2) trois points » —, soit en fin de phrase, par la **distribution respective** :

```docdg
<Soit>A(1;2), B(-1;2) et C(-1;-2) 3 points
<Soit>A, B et C 3 points de coordonnées respectives (1;2), (-1;2) et (-1;-2)
```

Au singulier : `<Soit>A(1 ; 2) un point` ou `<Soit>A un point de coordonnées (1 ; 2)`. Les vecteurs suivent les mêmes deux formes (`<Soit>u(1 ; 0 ; 1) et v(0 ; 1 ; 1) 2 vecteurs`). Les autres objets prennent leur lien : `<Soit>u une suite définie par u(0) = 1 et u(n+1) = 2u(n) + 1`, `<Soit>S un système défini par {` … `}`, `<Soit>M une matrice définie par {` … `}` ou, d'une ligne, `<Soit>M une matrice telle que M = {{1 ; 2} ; {3 ; 4}}`. Les anciennes tournures (`<Soit>les points A(1;2)…`, `<Soit>un point A(…)`, `<Soit>la matrice M {`) sont refusées avec la forme attendue.

Les deux phrases posent exactement les trois mêmes points. Deux garde-fous, dits en français : le **compte** annoncé par le nombre doit correspondre au nombre de points décrits, et la distribution respective exige **autant de couples de coordonnées que de noms**, appariés dans l'ordre. La factorisation vaut aussi pour les **fonctions** — une seule phrase les pose toutes :

```docdg
<Soit>f, g et h 3 fonctions définies par f(x) = exp(−x²), g(x) = −x⁴ + 2x² + 1 et h(x) = (x+1)/(x−2)
```

et chacune est enregistrée comme si elle avait été posée seule (tableaux de variations, dérivées, zéros, tracé : tout suit). Les coefficients décimaux s'écrivent à la française (`-4,9t^2 + 20t`) : la virgule décimale, encadrée de chiffres, n'est jamais confondue avec la virgule qui sépare les fonctions. Elle vaut de même pour les **phrases calculantes** — `<Calculer>la dérivée de f, g et h` vaut trois calculs, un par fonction, et de même pour la dérivée seconde, la primitive, les zéros... — et pour le **tracé** : `<Représenter>graphiquement les fonctions f, g et h pour x dans [-2 ; 2] et y dans [-3 ; 3]` dessine les trois courbes dans le même repère, chacune avec sa couleur et sa légende. La distribution respective sert aussi au repère (`avec des unités respectives de 2 cm et 0,5 cm`, voir `<Tracer>`) ; son extension aux autres objets (sphères, événements) suivra au fil des versions.

> **Règle de l'article :** indéfini à la déclaration (`<Soit>A(1 ; 2) un point`), défini à toute reprise ultérieure (`<Tracer>` ou une autre action : `le point A`).

**Le champ de vecteurs.** Pour les classes préparatoires, `<Soit>` sait déclarer un champ :

```docdg
<Soit>F un champ de vecteurs de ℝ³ dans ℝ³ défini par F(x, y, z) = (x², yz, sin(z))
```

qui s'affiche « Soit F le champ de vecteurs de ℝ³ dans ℝ³ défini par F(x, y, z) = (x², yz, sin(z)) ». Comme toute déclaration, le nom vient d'abord, puis le type ; les ensembles s'écrivent comme partout, ℝ³, ℝ²*+. La déclaration est **purement notationnelle** : un champ vectoriel n'a ni courbe ni tableau de variations associables, il n'est donc pas enregistré comme les fonctions scalaires le sont.


### **`<Tracer>` — Dessiner tout type de figure**

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
<Tracer>le triangle ABC équilatéral, de côté 5 cm, avec les marques
<Tracer>le carré ABCD, de côté 4 cm, avec les marques
<Tracer>le triangle ABC rectangle en A, de côté AB 3 mm et de côté AC 4 mm, avec les marques
```

**Le cercle en détail :**

```docdg
<Tracer>le cercle O, de rayon 3 cm
<Soit>A(0;0) et B(3;0) 2 points
<Tracer>le cercle O, de rayon AB
<Tracer>le cercle O, de diamètre 6 cm
<Tracer>le cercle, de centre O et de rayon 4 cm
<Tracer>le cercle, de centre (2;3) et de rayon 4 cm
<Tracer>le disque O, de rayon 3 cm, rempli
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `<Tracer>le cercle O, de rayon 3 cm` | rayon en cm |
| `<Tracer>le cercle O, de rayon AB` | rayon = longueur du segment AB |
| `<Tracer>le cercle O, de diamètre 6 cm` | via le diamètre plutôt que le rayon |
| `<Tracer>le cercle, de centre O et de rayon 4 cm` | centre nommé, sans coordonnées |
| `<Tracer>le cercle, de centre (2;3) et de rayon 4 cm` | centre positionné explicitement |


Mêmes options pour `disque` (`de centre A`, `de rayon N` ou `de diamètre N`, `rempli`) et pour `polygone` (`de centre A`, `de rayon N` pour un polygone régulier inscrit).

#### **🧭 Déclarer le repère**

Un repère se déclare comme tout objet, nom puis type : son origine et sa base, sa nature, son espace.

```docdg
<Soit>(O ; i, j) un repère orthonormé du plan
<Soit>(O ; i, j, k) un repère orthonormé de l'espace
<Soit>(O ; i, j) un repère orthogonal du plan
<Soit>(O ; i, j) un repère du plan
```

La nature compte, parce que les formules en dépendent. Dans un repère **orthonormé**, ‖u‖² = x² + y². Dans un repère **orthogonal**, les vecteurs de base ont chacun leur longueur, et ‖u‖² = x²‖i‖² + y²‖j‖² : la norme, le produit scalaire, l'angle et le projeté orthogonal se calculent dès que ces longueurs sont déclarées.

```docdg
<Soit>(O ; i, j) un repère orthogonal du plan tel que ‖i‖ = 2 et ‖j‖ = 1
<Soit>u(3 ; 4) un vecteur
<Calculer>la norme de u
```

Le calcul affiché porte les longueurs : ‖u‖ = √(3² × 2² + 4²) = 2√13. Si elles manquent, `<Calculer>la norme de u` est refusé, et le message dit lesquelles déclarer. Dans un repère **quelconque**, les coordonnées ne suffisent pas : il faudrait aussi les angles des vecteurs de base, et le calcul est refusé pour cette raison. Sans repère déclaré, rien ne change : les calculs supposent, comme toujours, un repère orthonormé.

Le repère est un objet du document. Sa déclaration pose aussi son origine comme point et ses vecteurs de base comme vecteurs : après `<Soit>(O ; i, j) un repère orthogonal du plan tel que ‖i‖ = 2 et ‖j‖ = 1`, `<Calculer>la norme de i` donne 2 et `<Calculer>le produit scalaire de i et j` donne 0. La distance d'un point à un plan suit les mêmes longueurs. Les figures en héritent : `<Tracer>dans un repère où…` prend les proportions du repère déclaré, et une figure qui demanderait un repère orthonormé alors qu'il est déclaré orthogonal est refusée.

**Vecteur par combinaison.** Un vecteur se pose aussi comme combinaison de vecteurs déjà posés, ceux du repère compris : `<Soit>u un vecteur tel que u = 2i + 3j`, `<Soit>w un vecteur tel que w = 2u − 3v`, ou avec un coefficient nommé, `w = k u`. docdg en calcule les coordonnées, et le vecteur sert ensuite comme tout autre. Un nom qui n'est ni un vecteur ni un nombre est signalé.

**Opérateur.** `soit rg un opérateur` déclare une notation : dans les formules, `rg` est alors composé en droit avec l'espacement d'un opérateur, `$rg(A)$` comme `$rg A$`. Les opérateurs usuels n'ont pas à être déclarés — `rang`, `noyau`, `image`, `trace`, `déterminant`, `dim`, `pgcd`, `card` et leurs voisins sont connus.

**Autres objets nommés.** Une suite explicite et un polynôme se posent comme une fonction et se calculent de même : `<Soit>v une suite définie par v(n) = (n + 1) / (2n + 3)`, puis `#{v(3)}` ; `<Soit>P un polynôme défini par P(X) = X³ − 2X + 1`, puis `#{P(2)}`. Un intervalle et un cercle sont des objets contrôlés — les bornes d'un intervalle se suivent, le centre d'un cercle est un point déjà posé, son rayon est strictement positif — et reçoivent un nom que le moteur résout partout où une commande le cite, quelle que soit la tournure — `sur I`, `dans I`, `l'abscisse appartient à I` — : `<Soit>I un intervalle tel que I = [0 ; 1]`, puis `<Soit>f une fonction de I dans ℝ définie par f(x) = x²` ; `<Soit>C un cercle de centre A et de rayon 3`, puis `<Tracer>le cercle C`.

#### **📊 Repère orthonormé**

**Syntaxe :** `un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5]`. Le repère est **orthonormé** par défaut : une graduation vaut la même longueur sur les deux axes (1 cm si rien n'est précisé), et l'adjectif `orthonormé` peut se dire explicitement.

**L'unité se règle en prose**, et son unité de longueur (cm ou mm) est obligatoire, comme pour toute mesure (règle n°10) :

- `avec une unité de 1,5 cm` — la même unité sur les deux axes, le repère reste orthonormé ;
- `avec des unités respectives de 2 cm et 0,5 cm` — la première pour l'axe des abscisses, la seconde pour celui des ordonnées (**distribution respective**, règle n°10) : le repère est alors **orthogonal**, et se dit tel.

**Exemples :**

```docdg
<Tracer>dans un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5] {
	le point A(1;2)
}
<Tracer>dans un repère orthonormé où l'abscisse appartient à [-4 ; 4] et l'ordonnée à [-3 ; 3], avec une unité de 1,5 cm {
	le point A(1;2)
}
<Tracer>dans un repère orthogonal où l'abscisse appartient à [-3 ; 3] et l'ordonnée à [-3 ; 3], avec des unités graphiques de 2 cm pour l'axe des abscisses et de 0,5 cm pour l'axe des ordonnées {
	le point A(1;2)
}
```

Déclarer un repère `orthonormé` avec deux unités distinctes est une contradiction : docdg la refuse et rappelle la forme `orthogonal`. *(L'ancien réglage `unité:N` en clé:valeur, comme les réglages d'affichage `avec le quadrillage`, `avec les axes` et `avec les noms`, n'existent plus : l'unité est une grandeur qui se dit en prose, et les axes gradués sont constitutifs du repère.)*

#### **🎡 Cercle trigonométrique**

**Syntaxe :** `le cercle trigonométrique, de rayon 4 cm`, suivi du réglage d'affichage `avec les valeurs` / `sans valeurs`, et des réglages techniques `pas:N`, `plage:A;B` en clé:valeur.

**Exemple :**

```docdg
<Tracer>le cercle trigonométrique, de rayon 4 cm, avec les valeurs
<Tracer>le cercle trigonométrique, de rayon 3,2 cm, avec les valeurs et un pas de $pi/6$
```

#### **🎲 Solides en 3D**

**Formes :** `le cube`, `le pavé`, `la pyramide`, `le cylindre`, `le cône`, `la sphère`

**Attributs constitutifs (en prose « de... ») :** `d'arête N`, `de côté N;N;N`, `de base N`, `de hauteur N`, `de rayon N`, `de diamètre N`, `d'angle N`, `de coefficient N`

Même convention que pour les figures planes : `arête`, `rayon`, `diamètre`, `hauteur` acceptent une valeur en cm ou le nom d'un segment déjà défini.

> ✅ **Arêtes cachées calculées automatiquement !**

**Exemple :**

```docdg
<Tracer>{
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
<Soit>A(2;-1,5) un point
<Placer>le point A
<Placer>un point B(3 ; 1)
<Placer>les points A, B et C tels que AB = 5 cm, BC = 4 cm et AC = 3 cm
<Tracer>la droite (AB)
<Tracer>la demi-droite [AB)
<Tracer>la demi-droite (AB]
<Tracer>le segment de droite [AB]
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `<Placer>le point A` | A doit avoir été défini au préalable (ex. via <Soit>) |
| `<Placer>un point B(3 ; 1)` | place B : le déclare et le marque |


**Placer par les distances.** En géométrie, un point se donne rarement par ses
coordonnées : il se construit au compas, à partir de ses distances aux autres.
`<Placer>les points A, B et C tels que AB = 5 cm, BC = 4 cm et AC = 3 cm` pose
les trois points à ces longueurs exactes — le premier à l'origine, le deuxième
sur l'horizontale, les suivants par intersection de deux cercles. Les points
sont déclarés comme par `<Soit>`, donc les figures suivantes peuvent s'y
référer. Trois longueurs incompatibles — un côté plus long que la somme des
deux autres — laissent le triangle introuvable, et docdg le dit.

**Convention pour les droites :** `[` / `]` = point inclus, `(` / `)` = point exclu — la même convention que pour la [droite graduée](#-droite-graduée).

La même prose (`la droite (AB)`, `la demi-droite [AB)`, `le segment de droite [AB]`) fonctionne aussi bien **dans un repère**, dès que les points portent des coordonnées connues (posés par `<Soit>`, ou placés dans le même bloc) : la droite et la demi-droite s'arrêtent alors au bord du repère plutôt qu'à une longueur fixe, et le segment relie exactement les deux points. C'est la même phrase, seul le contexte — avec ou sans repère ouvert — décide de l'échelle.

**Construction avec longueur imposée.** Le geste de base au collège — tracer un segment ou une demi-droite d'une longueur donnée — s'écrit avec la clause `tel que` (ou `telle que`), suivie de la longueur avec son unité :

```docdg
<Tracer>le segment de droite [AB] tel que AB = 4 cm
<Tracer>le segment [CD] tel que CD = 28 mm
<Tracer>la demi-droite (AB] telle que AB = 3,5 cm
```

Le segment est dessiné à l'échelle (1 cm = 1 cm sur le papier), ses extrémités nommées et pointées, la longueur inscrite au-dessus. L'unité (`cm` ou `mm`) est **obligatoire** : `tel que AB = 5` seul est refusé, avec un message qui le rappelle. Sans clause `tel que`, le segment garde une longueur d'affichage par défaut.

#### **📐 Mode analytique**

Pour construire une figure complète dans une partie (ou un cadre) — plusieurs points, droites, courbes, vecteurs liés entre eux — **avec ou sans repère**, on ouvre un bloc `<Tracer>{...}` : tout ce qui se trouve entre les accolades sera dessiné dans le même canevas. Si la première phrase du bloc est **le repère lui-même** — `un repère où l'abscisse appartient à [a ; b] et l'ordonnée à [c ; d]` — ses bornes fixent le canevas, et tout ce qui suit s'y inscrit ; sans cette phrase, la figure se construit librement, hors de tout repère. Le repère se dit `orthonormé` explicitement si l'on veut le préciser (c'est le défaut) ; le repère `orthogonal` prend des unités distinctes sur les deux axes : `un repère orthogonal où ..., avec des unités respectives de 2 cm et 0,5 cm` (abscisses puis ordonnées).

**Syntaxe :**

```docdg
<Tracer>dans un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5] {
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
<Tracer>dans un repère où l'abscisse appartient à [-5 ; 5] et l'ordonnée à [-5 ; 5]{
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

> Les points posés par un `<Soit>` antérieur — au singulier (`<Soit>A(2;7) un point`) comme au pluriel factorisé (`<Soit>A(-3;1), B(3;-1) et C(2;4) 3 points`) — **persistent** : tout bloc `<Tracer>` qui les nomme les reçoit automatiquement. La médiatrice et la bissectrice sur figure métrique (hors repère, `le triangle ABC...`) restent prévues pour une version ultérieure.

**L'angle sous toutes ses formes.** Au-delà de `l'angle BAC` (trois points, sommet central), le repère accepte :

```docdg
<Tracer>dans un repère où l'abscisse appartient à [-2 ; 2] et l'ordonnée à [-2 ; 2]{
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

> **Périmètre du moteur de figures — et feuille de route.** Le moteur de figures est un moteur **plan**. L'angle dièdre entre deux plans, le plan défini par trois points de l'espace, l'angle entre une droite et un plan — toutes les constructions qui exigent une vraie troisième dimension — sont notés pour une version ultérieure dotée d'un moteur 3D (les solides actuels sont des projections câblées, pas un espace calculé). De même, `la droite d, passant par A et B` comme objet *nommé* réutilisable attend le mécanisme d'alias de figures (`soit d une droite telle que d = ...`), prévu avec la persistance des points entre blocs. Cette forme est prospective, non disponible en 3.9.

> **Un seul verbe, une seule prose.** Le mode analytique n'introduit aucune syntaxe nouvelle : c'est un `<Tracer>le repère, ...` ordinaire (règle n°10, article défini, bornes en prose `où l'abscisse appartient à [a ; b] et l'ordonnée à [c ; d]`) dont le contenu entre accolades énumère les objets à inscrire dans ce repère, chacun à son tour avec sa propre prose. Une ébauche antérieure du langage ouvrait ce mode par un réglage `axes:{...}` en clé:valeur ; mais les bornes décrivent *ce que montre* la figure, au même titre qu'un rayon ou un côté — elles sont donc en prose, et le mot `axes:` n'existe pas. On n'écrit jamais `<Tracer, où l'abscisse ...>` (virgule après un verbe sans objet, incorrecte en français) : le repère est toujours nommé (`le repère`), exactement comme on nomme `le cercle` ou `la fonction`. Le même repère se dit aussi bien `<Représenter>graphiquement le repère, ...` dès que son contenu relève de l'analyse (courbes de fonctions) plutôt que de la géométrie plane — voir l'encadré ci-dessous.

**Le repère est un objet : il accueille des courbes de fonctions.** Une fonction posée par `<Soit>` se trace *dans* un repère qui contient aussi des points, des droites ou des vecteurs — la courbe est un habitant du repère parmi les autres :

```docdg
<Soit>f et g 2 fonctions définies par f(x) = x² − 2 et g(x) = −x² + 3

<Représenter>graphiquement un repère où l'abscisse appartient à [-3 ; 3] et l'ordonnée à [-4 ; 4]{
    les courbes des fonctions f et g
    la droite d'équation y = x
    le point A(-2 ; 3,5)
}
```

Chaque courbe reçoit sa couleur et son nom en étiquette. Trois proses équivalentes pour la courbe : `la fonction f`, `la courbe de la fonction f`, et au pluriel factorisé `les courbes des fonctions f et g` (ou `les fonctions f et g`). La droite se dit par son équation, `la droite d'équation y = ...`, avec n'importe quelle expression au second membre. La fenêtre et les unités viennent du repère englobant — la courbe n'en redit rien.

**La forme académique du repère.** Les énoncés du secondaire et du supérieur nomment le repère et détaillent chaque axe ; docdg les lit tels quels :

```docdg
<Représenter>graphiquement dans un repère orthogonal (O ; i, j) pour x ∈ [-3 ; 3] et y ∈ [-9 ; 9] avec des unités graphiques de 1,5 cm pour l'axe des abscisses et de 0,5 cm pour l'axe des ordonnées{
    les courbes des fonctions f et g
    la droite d'équation y = x
    le point A(-2 ; 3,5)
}
```

- **`<Représenter>graphiquement ...`** est le verbe de l'**analyse** : fonction, courbe, diagramme en escalier d'une suite, statistique (voir plus loin) — la représentation d'un concept abstrait, avec ou sans repère explicite. `<Tracer>` reste le verbe de la **géométrie**, plane ou dans l'espace : triangle, cercle, droite, polygone, médiatrice, bissectrice, solide — le tracé concret d'une figure, **même posée dans un repère** (un point, une droite ou une médiatrice entre coordonnées se trace, elle ne se représente pas graphiquement). Les deux verbes peuvent ouvrir le même mode analytique et les mêmes blocs ; seul le contenu — fonction ou figure — décide lequel s'emploie. Une figure mixte (une droite tracée à côté d'une courbe, par exemple) se range du côté de son objet principal.

| Domaine | Verbe | Exemples |
| --- | --- | --- |
| Analyse | `<Représenter>graphiquement` | fonctions, courbes, diagrammes en escalier, statistiques |
| Géométrie plane | `<Tracer>` | triangles, cercles, droites, polygones, médiatrices |
| Géométrie dans l'espace | `<Tracer>` | cubes, pyramides, sphères |
| Probabilités | `<Construire>` | arbres de probabilités |
| Algèbre linéaire | `<Afficher>` | matrices, tableaux |

- **`dans un repère ...`** : le `dans` de la tournure académique se lit naturellement après le verbe.
- **`(O ; i, j)`** nomme le repère, comme au tableau (les écritures `(O, i, j)` et `(O ; i ; j)` sont lues de même) — la notation est acceptée et consommée ; elle n'ajoute rien au tracé (l'origine s'appelle toujours O).
- **`pour x ∈ [a ; b] et y ∈ [c ; d]`** — qui se dit aussi `pour x appartient à [a ; b] et y à [c ; d]` — est l'équivalent de `où l'abscisse appartient à ... et l'ordonnée à ...` — les deux tournures se valent.
- **`avec des unités graphiques de 1,5 cm pour l'axe des abscisses et de 0,5 cm pour l'axe des ordonnées`** est la forme longue, chaque axe nommé en toutes lettres, de `avec des unités respectives de 1,5 cm et 0,5 cm` — l'ordre des axes peut être inversé, les valeurs se remettent d'elles-mêmes dans le bon ordre.

---

<a id="les-saisies-interactives"></a>

## **🖊️ Les saisies interactives**

**La grande nouveauté de docdg 2.0** : un document docdg peut poser des questions au lecteur, et se construire avec ses réponses. C'est ce qu'aucun document figé — LaTeX, Typst, PDF statique — ne peut faire : puisque docdg compose lui-même son rendu, le document devient **vivant**.

### La balise `<Saisir>`

Une saisie se déclare comme une définition ordinaire : le type à sa place habituelle, à gauche du signe égal, et la balise ne porte que la question entre accolades.

```docdg
soit prénom une chaîne de caractères telle que prénom = <Saisir>{Quel est le prénom de l'élève ?}
soit âge ∈ ℤ tel que âge = <Saisir>{Quel est son âge ?}
soit taille ∈ 𝔻 tel que taille = <Saisir>{Quelle est sa taille, en mètres ?}
soit fraction ∈ ℚ tel que fraction = <Saisir>{Quelle fraction (ex. 1/3) ?}
soit mesure ∈ ℝ tel que mesure = <Saisir>{Quel réel (ex. racine(2)) ?}
soit affixe ∈ ℂ tel que affixe = <Saisir>{Quel complexe (ex. 1 + 2i) ?}
soit marié ∈ 𝔹 tel que marié = <Saisir>{Est-il marié ?}
soit initiale un caractère tel que initiale = <Saisir>{Quelle est sa lettre préférée ?}
```

Le type s'écrit là où tout type s'écrit — la saisie ne fait aucune exception à la règle. Neuf types simples se saisissent : **un entier naturel** (positif ou nul), **un entier relatif** (ou, par convention, **un entier**), **un décimal** (écrit avec **la virgule**, jamais le point — `1,65`), **un rationnel** (par exemple `1/3`), **un réel** (par exemple `racine(2)`), **un complexe** (par exemple `1 + 2i`), **un booléen** (`vrai`, `oui`, `faux` ou `non`), **un caractère** (un seul), **une chaîne de caractères** (non vide). Les nombres sont évalués et vérifiés par le même moteur de typage exact que les déclarations ordinaires : une valeur irrationnelle n'est pas admise comme rationnelle et un complexe non réel n'est pas admis comme réel. Le type se déclare avec la variable, et non dans la balise. Une déclaration algorithmique de saisie sans type, ou dont la syntaxe n'est pas reconnue, reste du texte sans avertissement et sans poser la question. Une déclaration reconnue dont le type n'est pas directement saisissable — une liste ou une matrice — conserve un diagnostic sur ce type.

### Le blocage typé

**Le document s'arrête à la première question sans réponse valide.** Rien de ce qui suit ne s'affiche — ni le texte, ni les questions suivantes, ni les calculs qui dépendent de la réponse. Tant que la saisie ne correspond pas au type, un message d'erreur temporaire s'affiche sous le champ (« Un nombre décimal est attendu, écrit avec une virgule — par exemple 1,65. ») puis s'estompe. Les questions se posent donc l'une après l'autre, dans l'ordre du document.

Une valeur validée s'affiche en bleu ; **cliquer dessus rouvre la question** — le document se replie jusqu'à elle et se déroule de nouveau avec la nouvelle réponse.

### Les réponses sont des valeurs comme les autres

Une saisie s'affiche avec `#nom` et entre dans les calculs comme n'importe quelle variable :

```docdg
soit prénom une chaîne de caractères telle que prénom = "Iris"
soit âge ∈ ℕ tel que âge = 9
soit taille ∈ 𝔻 tel que taille = 1,32
soit a ∈ ℕ tel que a = 3
#prénom a #âge ans et mesure #taille m, soit #{taille * 100} cm.
<Tracer>le solide cube, d'arête #a cm
```

Un booléen s'affiche `vrai`/`faux` mais vaut 1/0 dans les conditions ; les descriptions de balises interpolent aussi les `#`, si bien qu'un solide, un calcul ou une figure peuvent dépendre d'une réponse du lecteur.

### L'alternative : le bloc et la ligne

L'alternative existe sous deux formes. Les exemples suivants fixent les réponses pour pouvoir être exécutés séparément. **Le bloc**, pour dérouler du contenu :

```docdg
soit marié ∈ 𝔹 tel que marié = vrai
si marié {
	Il est marié.
} sinon {
	Il n'est pas marié.
}
```

**La ligne**, pour produire une valeur — du texte ou un nombre :

```docdg
soit marié ∈ 𝔹 tel que marié = vrai
soit âge ∈ ℕ tel que âge = 17
soit statut une chaîne de caractères telle que statut = si marié { marié } sinon { célibataire }
soit tarif ∈ ℕ tel que tarif = si âge moins de 18 { 5 } sinon { 9 }
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

**Couleurs :** `rouge`, `bleu foncé`, `vert forêt` — les 148 noms français listés dans la [référence des couleurs](#-référence-des-couleurs-148-disponibles), avec leurs accords. Dans une balise de style, **la couleur nue est celle du texte**, et `fond <couleur>` celle du fond : `<gris clair>` écrit en gris clair, `<fond gris clair>` pose un fond gris clair. Un style nommé les combine — `soit correction un style défini par <fond gris clair>`.

**Filet de sécurité — teinte absente de la palette française :** les 148 noms français couvrent l'intégralité de la palette CSS usuelle ; il ne devrait donc jamais être nécessaire d'en sortir. Aucun exemple de ce guide n'utilise de nom anglais — docdg se lit et s'écrit intégralement en français.

**Tailles :** `12 pt`, `14 pt`

**Polices :** `<ARIAL>`, `<TIMES NEW ROMAN 12 pt>` — Georgia, Times New Roman et Arial voyagent avec le document et s'affichent à l'identique sur toutes les machines ; tout autre nom est cherché sur le système qui compose.

### **📏 Alignements**

**Horizontal :** `à gauche`, `au centre`, `à droite`, `justifié`

**Vertical (dans cellules) :** `hg`, `hc`, `hd`, `mg`, `mc`, `md`, `bg`, `bc`, `bd` (alias en toutes lettres : voir [Les objets - le tableau](#2-dresseun-tableau--tableaux-simplifiés))

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
x<exposant>{2}
y<indice>{1}
H<indice>{2}O
x<exposant 4 mm>{2}
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `x<exposant>{2}` | x² (décalage par défaut de la classe) |
| `y<indice>{1}` | y₁ |
| `H<indice>{2}O` | H₂O |
| `x<exposant 4 mm>{2}` | x² avec exposant relevé de 4 mm (mesure explicite) |


> **Nombre collé ou séparé ?** Deux familles, deux conventions, une seule logique. Quand le nombre **compte des objets** (une multiplicité), il est **collé** au mot : `<4 colonnes>` (voir « Fusion de cellules »). Quand le nombre est une **mesure** avec une unité (mm), il est **séparé** et porte son unité : `<exposant 4 mm>`. Sans nombre, `<exposant>` et `<indice>` prennent le décalage par défaut fixé par l'option de classe `décalage` — c'est la forme courante, le nombre n'étant utile que pour un ajustement ponctuel.

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

> **Reconnaissance des instructions :**
>
> - `soit` (minuscule) ouvre une déclaration **si la ligne complète en respecte la syntaxe** ; la déclaration est alors exécutée sans s'afficher.
>   Exemple conforme : `soit x ∈ ℤ tel que x = 5`. `soit x = 5` reste du texte sans alerte et ne crée pas `x`.
> - `<Soit>` (majuscule) : **Action-phrase** qui affiche un énoncé dans le PDF.
>   Exemple : `<Soit>A(1 ; 2) un point`
>
> Cette distinction suit la **règle n°9** : seules les actions-phrases (qui s’écrivent comme des phrases françaises) commencent par une majuscule.

**Mauvaise et bonne pratiques**

À éviter : « Soit » en majuscule pour une déclaration technique. Cette ligne n'est pas une instruction ; elle s'imprime telle quelle dans le PDF :
```text
Soit titre = <gras 18 pt au centre>
```

Pour déclarer un style, écrire une déclaration complète en minuscule ; pour afficher un énoncé, utiliser la balise d'action `<Soit>` :
```docdg
soit titre un style défini par <gras 18 pt au centre>
<Soit>A(2;3) un point
```

**La minuscule ne suffit pas à reconnaître une instruction.** En tête de ligne, `soit`, `pour`, `si`, `sinon`, `tant que`, `faire` et `selon la valeur de` ne sont exécutés que si leur construction complète respecte la grammaire. Sinon, la ligne reste affichée comme du texte ordinaire, sans avertissement et sans effet sur les variables. Ainsi `soit $x = 2$ ou $x = -2$.`, `pour cette raison, le résultat est vrai.` et `selon la valeur de $x$, le signe change.` sont des phrases. « Soit $x$ un réel positif. » avec une majuscule reste aussi du texte. Au milieu d'une phrase, « soit » garde son sens de « ou bien ».

**Le diagnostic se place à l'utilisation d'une variable.** Après une ligne mal formée telle que `soit n = 12`, `#n` demande une valeur qui n'a pas été déclarée : le message explique que la déclaration est absente ou ne respecte pas la syntaxe de DocDG. Il apparaît à l'endroit de cette utilisation ; le reste du document est composé. Les symboles d'une formule, comme `$n$` ou `$x_{n}$`, sont des notations mathématiques libres. Une instruction reconnue reste soumise aux contrôles de valeur, de type et de calcul.

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `Soit titre = <gras 18 pt au centre>` | Texte affiché ; aucune déclaration de style |
| `<Soit>A(2;3) un point` | → Correct : action-phrase |


**Définir un style :**

```docdg
soit titre un style défini par <bleu marine gras 18 pt au centre>
soit important un style défini par <gras souligné rouge>
```

**Variables numériques :**

```docdg
soit pi ∈ 𝔻 tel que pi = 3,14159
soit rayon ∈ ℕ tel que rayon = 5
soit aire ∈ ℝ tel que aire = pi * rayon^2
L'aire vaut #aire cm².
```

**Macros avec paramètres :**

```docdg
soit bonjour{nom} = Bonjour #nom !
soit carre{x} = #{x^2}

<bonjour{Jean}>
<carre{5}>
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `<bonjour{Jean}>` | → Bonjour Jean ! |
| `<carre{5}>` | → 25 |


**Exemple complet** — les trois premières lignes déclarent sans rien afficher (`soit`, mot-clé technique) ; les deux dernières sont des actions-phrases, qui s'affichent dans le PDF. docdg n'a pas de commentaire : toute ligne qui n'est pas une instruction s'imprime.
```docdg
soit titre un style défini par <gras 18 pt au centre>
soit pi ∈ 𝔻 tel que pi = 3,14159
soit bonjour{nom} = Bonjour #nom !

<Soit>A(2;3) un point
<Tracer>le cercle, de centre A et de rayon 5 cm
```

### **🔁 Structures de contrôle**

> **💡 Les deux branches s'accordent en nature.** Dans `soit statut une chaîne de caractères telle que statut = si marié { marié } sinon { célibataire }`, si l'une des deux branches est du texte, le résultat est du texte — même lorsqu'un mot de branche porte le nom d'une valeur déjà définie. Deux branches numériques donnent un nombre, calculable comme les autres.


**Boucles `pour` :**

```docdg
pour n ∈ ⟦1 ; 5⟧ {
  <gras>Exercice #n
}
```

**La variable de parcours dit son domaine**, comme au tableau : `pour k ∈ ⟦1 ; 10⟧` parcourt l'intervalle d'entiers, bornes incluses.

```docdg
pour k ∈ ⟦1 ; 10⟧ {
  #k
}
```

Ce programme affiche les dix valeurs de 1 à 10. Le domaine porte le type : `⟦1 ; 10⟧` donne un entier naturel, `⟦-2 ; 2⟧` un entier relatif, `[0 ; 2] par pas de 0,25` un décimal, et `pour x ∈ v` prend le type des éléments de `v`. La forme non typée `pour k de ... à ...` et l'écriture à deux-points `pour k: un entier naturel de 1 à 3` sont refusées. Un **ensemble écrit sur place** se parcourt aussi : `pour k ∈ {1 ; 3 ; 5}` donne à `k` le plus petit type de la hiérarchie numérique ℕ ⊂ ℤ ⊂ 𝔻 ⊂ ℚ ⊂ ℝ capable de représenter exactement toutes les valeurs — ℕ ici, qui contient 0, ℤ pour `{-1 ; 2}`, 𝔻 pour `{0,5 ; 1}`, ℚ pour `{1/3 ; 2}` —, et `pour jour ∈ {"lundi" ; "mardi"}` une chaîne de caractères. Une chaîne s'écrit **entre guillemets** : un mot nu serait pris pour une variable, il est refusé avec la forme corrigée. Un ensemble **n'a pas d'élément répété** : `{1 ; 3 ; 3 ; 5}` est refusé, et l'on parcourt des répétitions en déclarant une liste, puis en la parcourant. L'ordre d'écriture est une convention de parcours, non une propriété de l'ensemble. Les éléments ont tous la même nature, des nombres ou des chaînes ; un ensemble vide, des éléments mêlés ou composés sont refusés, avec la forme attendue. Un paramètre de fonction se parcourt de même, selon son type déclaré.

**Les deux bornes sont incluses** : `pour n ∈ ⟦1 ; 5⟧` parcourt 1, 2, 3, 4 **et** 5 (cinq passages), exactement comme on le lirait à voix haute. Contrairement à la plupart des langages de programmation (où une borne « à N » exclut souvent N), docdg ne demande pas à un professeur de compter « il faut mettre 6 pour aller jusqu'à 5 » — la phrase française fait foi.

```docdg
soit noms une liste de chaînes de caractères telle que noms = {"chat.png" ; "chien.png"}
pour f ∈ noms {
  <Insérer l'image IMAGES/#f avec une largeur de 20 mm>
}
```

**Le pas** se règle en prose, à la suite du domaine : `par pas de p`. Les bornes et le pas acceptent les **décimaux**, virgule française comprise :

```docdg
pour i ∈ 𝔻, allant de 0,5 à 2,5 par pas de 0,5 {
  Valeur : #i
}
```

**Le booléen s'écrit en quatre mots.** `vrai` et `oui` d'un côté, `faux` et `non` de l'autre, la casse étant indifférente : `Oui`, `NON` et `Faux` sont lus comme leurs minuscules. Une question fermée posée à l'élève se répond donc comme on la lit.

```docdg
soit inscrit ∈ 𝔹 tel que inscrit = <Saisir>{L'élève est-il inscrit ?}
```

**Boucles `répéter … fois` :** quand le rang n'a pas à paraître, la répétition se dit comme en Logo — un nombre de tours, rien d'autre. Aucune variable de boucle n'est introduite, donc aucune à nommer ni à ignorer.

```docdg
répéter 4 fois {
  <Tracer>le segment [AB] tel que AB = 3 cm
}
```

Le nombre de tours accepte une expression : `répéter n fois`, `répéter 2*k fois`. La forme `répète` est acceptée de même, et les boucles s'imbriquent. Écrire « il faut répéter l'exercice trois fois » dans une phrase ne déclenche rien : la structure ne se reconnaît qu'à l'accolade ouvrante qui la suit sur la même ligne.

**Boucles `tant que` :** la condition est vérifiée **avant** chaque passage ; le mot `faire` devant l'accolade est facultatif mais recommandé — il se lit mieux.

```docdg
soit n ∈ ℕ tel que n = 0
tant que n < 3 faire {
  Passage numéro #n
  n ← n + 1
}
```

**Boucles `faire ... tant que` :** le corps s'exécute **au moins une fois**, la condition est vérifiée **après** chaque passage.

```docdg
soit n ∈ ℕ tel que n = 0
faire {
  Passage numéro #n
  n ← n + 1
} tant que n < 3
```

> **Compteurs et portées.** `soit` **déclare**, une fois ; `n ← n + 1` **réaffecte** un nom déjà posé, et c'est ce qui fait avancer la boucle. Le répéter à chaque tour n'apprendrait rien. La flèche rend cette écriture sûre deux fois : `←` ne se confond avec rien d'autre dans une phrase, et le nom doit déjà exister. Un nom posé pour la première fois *à l'intérieur* d'un bloc, lui, reste local au bloc (règle des portées, plus bas) : ce qu'on pose en tête sert partout, ce qu'on pose dans un tiroir reste dans le tiroir.

> **Minuscule ou majuscule.** La majuscule initiale marque la classe **là où une classe est attendue** : après un deux-points et devant la parenthèse d'instanciation — `soit c un Carré tel que c = Carré("c" ; 3)`. Partout ailleurs elle ne dit rien de particulier, et `A` nomme une matrice comme au tableau noir. Un nom de variable identique à une primitive — `somme`, `min`, `max`, `trier`, `longueur` — est en revanche refusé : une fonction que vous écrivez remplace la primitive du même nom, ce qui est voulu ; une variable ne ferait que la masquer, en silence.

**Conditions `si` :**

```docdg
soit note ∈ ℤ tel que note = 15

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
soit note ∈ ℤ tel que note = 15
si note au moins 10 {
  <vert gras>Admis !
}
```

**Choisir parmi plusieurs cas : `selon la valeur de`.** Quand une même valeur se compare à plusieurs cas, `selon la valeur de` remplace la cascade de `si`. Un cas est une valeur isolée, plusieurs valeurs séparées par `;`, un intervalle — fermé `[4 ; 10]` ou demi-ouvert `]10 ; 20]` — ou une combinaison par `ou` ; le premier cas compatible est retenu, et `sinon` reste facultatif.

```docdg
soit note ∈ ℤ tel que note = 15
selon la valeur de note {
	0 { <rouge gras>Nulle. }
	1 ; 2 ; 3 { Faible. }
	[4 ; 10] { Moyenne. }
	]10 ; 20] { <vert gras>Bonne. }
	sinon { La note est hors barème. }
}
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `si note au moins 10 {` | strictement équivalent à : si note >= 10 |


Les conditions se **combinent** avec les connecteurs du français, `et` et `ou`, et s'enchaînent avec `sinon si` — seule la première branche vraie est retenue :

```docdg
soit note ∈ ℤ tel que note = 12
soit rattrapage ∈ ℤ tel que rattrapage = 8

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
- Circulaires réciproques : `arcsin`, `arccos`, `arctan`
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

Le type se dit comme au tableau noir. Pour les nombres, c'est l'appartenance à un ensemble qui le porte : `soit n ∈ ℕ tel que n = 7`, `soit x ∈ ℝ tel que x = racine(2)` — sept ensembles s'écrivent ainsi, ℕ, ℤ, 𝔻, ℚ, ℝ, ℂ et 𝔹. Pour tout autre type, le nom du type se lit en toutes lettres, à la suite du nom et sans virgule, et **porte toujours son article** : `soit v un vecteur de 4 réels tel que v = …`, `soit s une chaîne de caractères telle que s = "été"` — `tel`, `telle`, `tels` ou `telles` s'accordent avec lui. Un style se déclare par `défini par` : `soit h0 un style défini par <bleu marine gras chapitre num>`. L'article n'est pas un ornement — c'est la règle du langage tout entier, celle qui vaut déjà pour l'objet d'une balise (`<Afficher>un cadre`, jamais `<cadre>`) et pour la prose de `<Soit>`. Un nom sans article sonne faux en français, et docdg se veut le plus proche possible de la langue. Plusieurs nombres se posent d'un trait : `soit a, b ∈ ℤ tels que (a ; b) = (3 ; -5)`, et plusieurs conteneurs de même type de même façon : `soit u, v des vecteurs de 2 rationnels tels que u = {1 ; 2} et v = {3 ; 4}`. Le mot-clé algorithmique `soit` est invariable dans DocDG. Une phrase commençant par `soient` reste de la prose. L'ancienne écriture à deux-points, `soit prix: un entier = 150`, ne constitue pas une déclaration reconnue : elle est affichée sans alerte et ne crée pas `prix`.

Le type n'est pas décoratif : il est vérifié à la déclaration, à chaque réaffectation et à la sortie d'une boucle, et l'erreur nomme la variable fautive.

**Le type est obligatoire, et l'article avec lui.** Toute variable déclare son type à sa naissance — valeurs, conteneurs, styles, résultats de saisie, indices de boucle, paramètres et types de retour. `soit prix = 150` ne déclare rien — aucun type n'est dit ; la forme correcte est `soit prix ∈ ℤ tel que prix = 150`. `soit s, chaîne de caractères, telle que …` ne constitue pas non plus une déclaration : l'article fait partie du type. L'écriture antérieure `soit prix: un entier = 150` reste également du texte. Ces lignes n'émettent aucun avertissement ; l'erreur n'apparaît que si l'on utilise ensuite un nom sans déclaration valide.

**Aucune variable nommée n'échappe au typage**, y compris une table de données sur plusieurs lignes : celle-ci se déclare avec un type explicite. Les composants paramétrés comme `soit exo{n} = ...` sont des **gabarits textuels** historiques, et non des variables de valeur ; leur statut lexical spécifique ne justifie aucune déclaration de valeur non typée. Toute nouvelle forme de macro produisant une valeur devra elle aussi être typée. Voir le [contrat de typage](docs/TYPAGE_OBLIGATOIRE.md) pour les points du moteur restant à aligner.

**Une réaffectation ne redit ni `soit` ni le type** : ils ont été posés à la déclaration. Elle s'écrit avec la flèche d'affectation des manuels d'algorithmique : après `soit prix ∈ ℤ tel que prix = 150`, on écrit `prix ← 180`. Écrire une seconde déclaration `soit prix ∈ ℤ tel que prix = 180` serait incorrect. La flèche `←` s'insère d'un clic depuis le ruban Symboles ; la graphie ASCII `<-` n'existe pas, car `x <- 5` ne se distinguerait pas de la comparaison `x < -5`. Le signe `=` est réservé à la déclaration : hors d'elle, `prix = #prix` est une égalité du texte qui s'affiche. Dans le corps d'une fonction, écrire `prix = 180` à la place de `prix ← 180` rend le bloc non reconnu : il reste du texte, sans alerte et sans créer de fonction.

| Type | Ce qu'il contient |
|---|---|
| `entier naturel` | un élément de ℕ — entier positif ou nul |
| `entier` (ou `entier relatif`) | un élément de ℤ |
| `décimal` | un élément de 𝔻 — développement décimal fini |
| `rationnel` | un élément de ℚ — affiché en fraction irréductible : `1/3 + 1/6` s'écrit `1/2` |
| `réel` | un élément de ℝ |
| `complexe` | un nombre de la forme `a + ib`, écrit en notation algébrique : `3 - 4i`, `2i`, `-i` |
| `booléen` | vrai ou faux |
| `chaîne de caractères` | du texte ; `chaîne` et `texte` sont synonymes |

L'inclusion ℕ ⊂ ℤ ⊂ 𝔻 ⊂ ℚ ⊂ ℝ ⊂ ℂ ne se remonte jamais, et **l'appartenance se décide sur la représentation exacte, jamais sur un flottant** : `-3` appartient à ℤ et non à ℕ, `1/3` à ℚ et non à 𝔻, `racine(2)` et `pi` à ℝ et non à ℚ. En sens inverse elle s'emploie librement : `soit z ∈ ℂ tel que z = 3` est accepté, puisque ℝ ⊂ ℂ. Le refus nomme les deux ensembles, le sens de l'inclusion et la sortie :

> ⚠ **pi est un réel : il n'appartient pas à ℚ. L'inclusion ℚ ⊂ ℝ ne se remonte pas — déclarez « un réel », ou demandez une valeur approchée avec arrondi(… ; n).**

Deux notions se distinguent, et c'est le cœur du typage. Le **domicile** d'une valeur — le plus petit ensemble de la chaîne auquel elle appartient — décide de la **validité**. Le **type déclaré** — l'ensemble où l'on a choisi de travailler — décide de l'**écriture**, car une même valeur a plusieurs notations également justes : `soit q ∈ ℚ tel que q = 1/3 + 1/6` donne `1/2`, `soit d ∈ 𝔻 tel que d = 1/3 + 1/6` donne `0,5`. `<Afficher>le type de q` dit les deux d'un trait : **un rationnel mais sa valeur appartient à 𝔻**.

Les entiers sont des grands entiers : `2^64` s'écrit `18446744073709551616`, sans saturation. Une variable typée s'emploie partout comme une autre : dans `pour`, `tant que`, `si` et `#nom`.

### **📦 Les conteneurs**

Quatre conteneurs, une seule grammaire — le nom, puis le type avec son article, puis `tel que` et la valeur, sans virgule, comme on écrit « Soit n un entier tel que… ». **Le type se dit avec son article**, comme tout nom français : `une liste de décimaux`, jamais `liste` nue. La grammaire est la même partout où un nom naît côté langage — déclaration, paramètre et valeur d'une fonction, déstructuration, indice de boucle, saisie — et elle ne souffre aucune exception, y compris quand le type semble évident. Les indices commencent à **0**, comme en NSI ; le point-virgule sépare, l'accolade rassemble.

```docdg
soit notes une liste de décimaux telle que notes = {12,5 ; 15 ; 9,5}
soit trajets un dictionnaire de chaînes de caractères et d'entiers tel que trajets = {"Marche": 5 ; "Bus": 3}
soit A une matrice 2×2 d'entiers telle que A = {{1 ; 2} ; {3 ; 4}}
soit devise une chaîne de caractères telle que devise = "liberté égalité fraternité"
```

`liste`, `tableau` et `collection` nomment le même type : ce sont les mots du programme, tous acceptés. `vecteur` se déclare et s'écrit de la même façon, mais c'est une matrice à une colonne : il porte l'algèbre linéaire (transposée, produit, norme), et une `liste de réels` ne se substitue pas à lui sans conversion déclarée.

**Lire, écrire, découper.** `notes[1]` lit, `notes[1] ← 14` écrit, `notes[0 à 2]` découpe une tranche **bornes incluses** — comme « de 1 à 5 » fait cinq tours. Une tranche dont la borne de gauche dépasse celle de droite est simplement **vide**, ce n'est pas une faute : c'est ce qui permet de conclure une fusion de deux listes triées en une ligne. Une matrice se lit par `A[ligne ; colonne]`, un dictionnaire par sa clé.

**Grandir.** Le `+` concatène deux conteneurs ; `ajouter(v ; x)` en ajoute un élément.

### **🧰 Les primitives**

Un **nom** pour ce qui produit une valeur, un **verbe** pour ce qui répond par oui ou non. Chacune rend une **nouvelle** valeur : la série de départ n'est jamais modifiée.

| Primitive | Rend |
|---|---|
| `longueur(v)` | le nombre d'éléments — collection, dictionnaire, matrice ou chaîne |
| `somme(v)`, `min(v)`, `max(v)` | un scalaire |
| `trier(v)`, `inverser(v)` | une nouvelle collection |
| `contient(x ; v)` | vrai ou faux |
| `indice de(x ; v)` | le rang — l'absence est une **faute dite**, non un −1 |
| `insérer(v ; i ; x)`, `supprimer(v ; i)`, `ajouter(v ; x)` | une nouvelle liste |
| `joindre(v ; séparateur)` | une ligne de texte |
| `découper(t ; séparateur)` | une liste de chaînes de caractères |

`min(a ; b)` à deux arguments reste la fonction mathématique. Une fonction que vous écrivez **l'emporte sur la primitive du même nom** : l'exercice qui demande d'écrire soi-même `sommet` ou `dépiler` reste possible.

**Écrire une suite sur une ligne.** Une boucle qui affiche met chaque tour sur sa ligne ; `joindre` aligne une suite avec le séparateur voulu, sans l'écueil de l'accumulateur qui en laisse un de trop à la fin.

```docdg
#{joindre({2 ; 3 ; 5 ; 7} ; ", ")}
#{découper("un code" ; " ")}
```

**Trois écritures, au choix de ce qui se lit.** La forme fonctionnelle `insérer(notes ; 0 ; 20)`, la forme prépositionnelle `dans notes insérer(0 ; 20)`, et pour les questions par oui ou non la forme directe `notes contient(15)`. C'est la phrase qui commande.

**La division euclidienne se dit comme au collège** : `quotient de 17 par 5`, `reste de 17 par 5` — la phrase dit lequel des deux nombres est le dividende. Les formes brèves `quotient(17 ; 5)` et `%` restent admises.

### **🔤 Les chaînes de caractères**

Les guillemets, droits ou français, délimitent sans appartenir à la valeur.

```docdg
soit m une chaîne de caractères telle que m = "Bonjour"
```

`m[0]` rend une lettre — qui est une chaîne d'un seul caractère, docdg n'introduit pas de type distinct. `pour c ∈ m` la parcourt lettre à lettre. S'y ajoutent `majuscule`, `minuscule`, `sans accents`, `élaguer` (les espaces des deux bouts) et `compacter` (tous les espaces), `code("B")` et `caractère(97)` pour le va-et-vient avec le rang Unicode, et les conversions `texte(42)` et `nombre("1,5")`.

**Les guillemets délimitent, ils n'appartiennent pas à la valeur** — droits ou français. C'est ce qui distingue le texte d'un document, qui s'écrit nu comme en LaTeX, de la chaîne de caractères qui relève de la programmation : elle se cite, y compris dans une liste et en clé de dictionnaire.

```docdg
soit mots une liste de chaînes de caractères telle que mots = {"chat" ; "chien"}
soit trajets un dictionnaire de chaînes de caractères et d'entiers tel que trajets = {"Marche": 5}
```

`majuscule("été")` rend **ÉTÉ**, non ETE : la typographie française accentue les capitales. Le tri suit la **collation française** — « école » se classe avant « Zoé », alors que le codepoint de « é » vient après celui de « z ».

### **🧩 Les fonctions algorithmiques**

Les chevrons marquent la fonction **mathématique** — objet d'étude qu'on dérive et qu'on trace ; le `soit` nu marque la fonction **algorithmique** — calcul nommé qu'on réutilise. Un même nom ne peut désigner les deux.

```docdg
soit addition : ℤ² → ℤ, (a, b) ↦ a + b
```

La signature se lit comme au tableau : le domaine, la flèche `→`, le codomaine, puis la variable et son image par `↦`. Les ensembles s'y disent par leurs symboles, un produit par l'exposant (`ℝ²`) ou par `×` (`une liste d'entiers × ℤ → ℤ`), et les autres types en toutes lettres. Dès que le corps compte plusieurs lignes, il suit entre accolades et la valeur produite se désigne par `retourner`, et rien n'est implicite :

```docdg
soit hypoténuse : ℝ² → ℝ, (a, b) ↦ {
	soit carrés ∈ ℝ tel que carrés = a×a + b×b
	retourner racine(carrés)
}
```

Une fonction dont le bloc d'instructions ne définit aucune valeur de retour reste du texte, sans avertissement et sans créer de fonction. Si une fonction reconnue atteint à l'exécution une branche qui ne retourne aucune valeur, cette erreur reste signalée à son appel. `retourner` placé dans une boucle en sort sur-le-champ. À l'intérieur d'une classe, une méthode se déclare comme toute fonction : `soit translaté : ℝ² → un Point, (dx, dy) ↦ …`, ou, sans argument, `soit norme une fonction à valeurs dans ℝ définie par norme() = …`.

**Un paramètre peut être un conteneur ou une chaîne**, et la valeur rendue aussi. Le passage se fait **par copie** : une fonction reçoit une valeur, elle ne peut rien renvoyer dans la série de l'appelant. C'est ce qui rend saine l'acceptation d'une `liste d'entiers` là où une `liste de réels` est attendue.

La récursivité est admise, bornée à deux cents appels imbriqués — au-delà, docdg s'arrête et le dit plutôt que d'épuiser la pile.

### **🎁 Le p-uplet**

Longueur fixe, types pouvant différer — le programme le distingue explicitement de la liste. Il s'écrit entre parenthèses, comme un point du plan.

```docdg
soit divise : ℤ² → ℤ × ℤ, (a, b) ↦ (quotient de a par b ; reste de a par b)
soit résultats (un entier ; un entier) tel que résultats = divise(17 ; 5)
soit q ∈ ℤ tel que q = résultats[0]
soit r ∈ ℤ tel que r = résultats[1]
```

Les **deux valeurs sont introduites avec un type déclaré**. On peut extraire les membres du p-uplet par leur rang (`résultats[0]`, `résultats[1]`). L'ancienne déliaison `soit (q ; r) = ...` crée deux noms sans annotation : **elle ne constitue pas une syntaxe conforme au contrat de typage obligatoire**. La forme prévue est `soit q, r ∈ ℤ tels que (q ; r) = divise(17 ; 5)` ; l'écriture de la 3.9, `soit (q: un entier ; r: un entier) = …`, reste du texte et ne crée aucun des deux noms.

### **🎲 Le hasard, et le cours d'une boucle**

`aléatoire(a ; b)` tire un entier entre les deux bornes, comprises. Deux compilations donnent deux tirages : c'est ce qu'une simulation attend — Monte-Carlo, marche aléatoire, étude de fréquences.

`sortir` arrête une boucle entière ; `continuer` n'arrête que le tour en cours. Les deux valent dans un document comme dans un corps de fonction.

### **🥞 Les piles et les files**

Les structures nommées du programme de terminale, avec leur discipline — c'est tout leur intérêt.

```docdg
soit p une pile d'entiers telle que p = {}
p ← empiler(p ; 1)
p ← empiler(p ; 2)
#{sommet(p)}    #{dépiler(p)}

soit f une file de chaînes de caractères telle que f = {"a" ; "b"}
#{tête(f)}      #{défiler(f)}
```

`empiler`, `dépiler`, `sommet` d'un côté ; `enfiler`, `défiler`, `tête` de l'autre ; `est vide` et `longueur` pour les deux. Chaque opération rend une **nouvelle** structure. Une file n'a pas de sommet, une pile n'a pas de tête, et aucune des deux ne s'indexe ni ne se parcourt : on la vide.

### **🧱 Les objets**

Une classe rassemble des données et ce qu'on sait en faire. Son nom commence par une majuscule. Ses membres se déclarent comme partout ailleurs : un attribut numérique par son ensemble (`abscisse ∈ ℝ`), les autres par leur type et son article (`titulaire une chaîne de caractères`, `gauche un Arbre`), une méthode comme une fonction.

```docdg
soit une classe Point {
	abscisse ∈ ℝ
	ordonnée ∈ ℝ

	soit norme une fonction à valeurs dans ℝ définie par norme() = racine(abscisse * abscisse + ordonnée * ordonnée)
}

soit p un Point tel que p = Point(3 ; 4)
#{p.abscisse}    #{p.norme()}
p.abscisse ← 6
```

Dans le corps d'une méthode, les attributs sont visibles par leur nom : la formule s'écrit comme au tableau. Un attribut se modifie sans `soit`, puisque rien n'est déclaré.

**Les quatre piliers** sont couverts, chacun d'un seul mot :

| Pilier | Écriture |
|---|---|
| **Encapsulation** | `privé solde ∈ ℝ`, `privé soit taux une fonction à valeurs dans ℝ définie par taux() = 0,02` |
| **Héritage** | `soit une classe Chien qui hérite de la classe Animal { … }` |
| **Polymorphisme** | la valeur décide de la méthode appelée, non le type écrit |
| **Abstraction** | `soit une classe abstraite Forme { … }`, où `soit aire une fonction à valeurs dans ℝ`, sans « définie par », n'a pas de corps |

Ce qui est privé reste visible **depuis la classe** ; du dehors, la lecture comme l'écriture sont refusées. Un enfant tient la place de son parent, l'inverse est dit. Une classe abstraite ne s'instancie pas, et une classe concrète ne peut rien laisser sans corps.

**Arbres et graphes** s'écrivent avec ce qui précède, sans type nouveau : un graphe est une liste d'adjacence, un arbre binaire se fait d'objets qui se contiennent — l'arbre vide étant une classe, non une valeur nulle.

### **⚠️ Les fautes sont dites**

Un indice hors bornes, une clé absente, un type qui ne convient pas, une récursion sans arrêt, une boucle qui ne finit pas : chaque cas produit un message français nommant ce qui était attendu, avec la ligne fautive — jamais un silence ni un document faux.

---

## **📊 Les graphiques de fonctions**

### **Déclaration d'une fonction**

Déclarer une fonction, c'est **poser un objet nouveau** — exactement ce que fait `<Soit>` pour un point. On emploie donc la même action, avec l'article indéfini (règle n°10) :

```docdg
<Soit>f une fonction définie par f(x) = x² − 2
<Soit>g une fonction définie par g(t) = sin(t) + cos(t)
```

> **Pourquoi `<Soit>` et non un objet `<fonction>` ?** Une fonction est une **hypothèse de départ** au même titre qu'un point : elle se *pose*, elle ne se *dessine* pas (c'est `<Représenter>graphiquement` qui la dessinera ensuite). La formule scolaire canonique est d'ailleurs « **Soit** f la fonction définie par… ». Unifier toute déclaration sous le verbe `<Soit>` (points, valeurs, fonctions) évite d'avoir à retenir un objet spécial `<fonction>` : un seul verbe pour tout ce qu'on pose, un seul verbe de graphage (`<Représenter>graphiquement`) pour tout ce qu'on représente dans un repère.

Plusieurs déclarations, de natures différentes si l'on veut, se regroupent entre accolades ; chaque ligne doit être une déclaration complète. Un groupe dont la syntaxe n'est pas reconnue reste affiché comme du texte, sans déclaration partielle :

```docdg
<Soit>{
    f une fonction définie par f(x) = x² − 2
    g une fonction définie par g(t) = sin(t) + cos(t)
}
```

### **Tracé de la courbe**

```docdg
<Soit>f une fonction définie par f(x) = x² − 2
<Représenter>graphiquement la fonction f
<Représenter>graphiquement la fonction f pour x appartient à [-5 ; 5] et y à [-10 ; 10]
<Représenter>graphiquement la fonction f, en rouge et de trait 0,5 mm
```

**Bornes d'affichage :** `pour x appartient à [A ; B] et y à [C ; D]`, en prose — même convention que le repère (voir règle n°10). Les variantes `pour x dans [A ; B]` et `sur [A ; B]` (abscisse seule) sont acceptées.

**En prose :** couleur (`en rouge`), épaisseur (`de trait 0,5 mm`), bornes d'affichage (`pour x appartient à [A ; B] et y à [C ; D]`), finesse (`avec 200 échantillons`).

**Plusieurs courbes dans le même repère** (factorisation plurielle, règle n°10) : `<Représenter>graphiquement les fonctions f, g et h pour x dans [-2 ; 2] et y dans [-3 ; 3]` — article pluriel (`les`, `des`) ou nombre (`3 fonctions f, g et h`, le compte est vérifié). Chaque courbe reçoit sa couleur et son entrée de légende ; les fonctions doivent avoir été posées par `<Soit>` auparavant.

**En prose également :** finesse (`avec 200 échantillons`), aire sous la courbe (`avec l'aire entre A et B`, ou `area:{a, b}` quand les bornes sont des nombres à donner tels quels), aire entre deux courbes (`avec l'aire jusqu'à g`), et le tracé en escalier d'une suite récurrente : `avec le diagramme en escalier depuis U0 sur N termes` (N vaut 8 si omis).

### **Analyse et algèbre du supérieur**

Chaque construction se pose en une phrase, sur une fonction ou une matrice déjà introduite par `<Soit>` ; le calcul est formel (SymPy), le rendu suit les notations du supérieur.

```docdg
<Soit>f une fonction définie par f(x) = sin(x)/x

<Calculer>la limite de f en 0
<Calculer>la limite de f en +infini
<Calculer>la limite de f en 0 à droite

<Calculer>le développement limité de f en 0 à l'ordre 4

<Calculer>les racines cinquièmes de l'unité
<Calculer>les racines cubiques de 8i

<Factoriser>x^2 - 5x + 6
<Simplifier>(x^2 - 1)/(x - 1)
<Résoudre>l'équation x^2 = 2x + 1

<Soit>M une matrice définie par {
    2 ; 1
    1 ; 1
}
<Calculer>le déterminant de M
<Calculer>l'inverse de M
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `<Calculer>la limite de f en 0` | lim f(x) = 1, x -> 0 |
| `<Calculer>la limite de f en +infini` | ... et -infini |
| `<Calculer>la limite de f en 0 à droite` | ... ou à gauche |
| `<Calculer>le développement limité de f en 0 à l'ordre 4` | avec le o(x^4) français |
| `<Calculer>les racines cinquièmes de l'unité` | S = (1, e^(2ipi/5), ...) |
| `<Calculer>les racines cubiques de 8i` | racines n-ièmes complexes |
| `<Factoriser>x^2 - 5x + 6` | (x - 2)(x - 3) |
| `<Simplifier>(x^2 - 1)/(x - 1)` | x + 1 |
| `<Résoudre>l'équation x^2 = 2x + 1` | S = (1 - rac(2), 1 + rac(2)) |
| `<Calculer>le déterminant de M` | det(M) = 1 (exact) |
| `<Calculer>l'inverse de M` | M^(-1), fractions exactes |


Le développement limité s'écrit avec le petit o de l'usage français (`+ o(x^n)`), les solutions d'équations sortent en radicaux exacts, le déterminant et l'inverse en fractions. Une matrice non inversible ou une fonction non posée sont dites en français.

### **Poser et résoudre un système**

Le système se pose par `<Soit>`, qui l'affiche et le mémorise, puis se résout par `<Résoudre>` — en **calcul exact** : coefficients entiers, décimaux ou fractions `p/q`, solutions rendues en fractions irréductibles. Le solveur dit aussi, en français, quand le système n'a aucune solution ou en admet une infinité.

```docdg
<Soit>s un système défini par {
    a + b + c = 2
    4a + 2b + c = 3
    9a + 3b + c = 6
}

<Résoudre>le système s
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `<Résoudre>le système s` | Le pivot de Gauss déroulé étape par étape, jusqu'à (1, -2, 3). |


> **Les deux moteurs de calcul.** docdg calcule à deux niveaux, sans option à régler : le **moteur interne** — arithmétique exacte et Gauss-Jordan embarqués en Rust pur, rien à installer, `<Résoudre>` et `#{...}` fonctionnent partout — et **SymPy**, sollicité automatiquement par les constructions de calcul formel (dérivées, zéros exacts, variations automatiques, [sympy.org](https://www.sympy.org)). SymPy et SciPy sont embarqués dans docdg avec leur propre Python : rien à installer, aucune option à activer, aucune ligne de commande à modifier. Chaque requête passe par un canal privé entre docdg et son moteur (jamais par la ligne de commande : pas d'injection), est mise en cache (l'expression répétée est gratuite), et remonte ses erreurs en français — rien n'est avalé.

### **Déterminer le domaine de définition d'une fonction**

Après avoir défini une fonction réelle d'une variable, demandez son domaine de définition dans le langage usuel de l'analyse :

```docdg
<Soit>q une fonction définie par q(x) = |x|racine((1−x)/(1+3x))
<Déterminer>le domaine de définition de q
```

Le moteur affiche le domaine exact : \(D_q=\left]-\frac13\,;\,1\right]\). Il distingue les bornes incluses et exclues, les intervalles disjoints, l'ensemble vide et l'ensemble des réels. La formulation `<Déterminer>le domaine de définition de la fonction q` est également acceptée. Cette opération relève du calcul formel (SymPy, embarqué). Si le domaine ne peut pas être déterminé exactement, docdg affiche un diagnostic plutôt qu'un ensemble approximatif ou incorrect.

### **Tableau de variations**

Le tableau de variations **se calcule tout seul** — dérivée, zéros réels, signes et valeurs exactes. La fonction doit avoir été posée par `<Soit>` auparavant : le tableau retrouve sa définition et l'affiche en tête.

```docdg
<Soit>q une fonction définie par q(x) = −x⁴ + 2x² + 1

<Dresser>le tableau de variations de q
```

C'est la seule forme : on nomme la fonction, docdg fait le reste. `<Étudier>les variations de q` en est le synonyme conjugué.

Les compagnons du calcul formel s'écrivent tous par une **action** — un verbe à l'infinitif, jamais un nom seul : `<Calculer>la dérivée de q` (et `<Calculer>la dérivée seconde de q`) affiche la dérivée simplifiée ; `<Déterminer>les zéros de q` l'ensemble exact des solutions de q(x) = 0, radicaux compris ; `<Calculer>une primitive de q` les primitives, à une constante près (« la primitive » est refusée : une fonction en admet une infinité). Les verbes interchangeables sont `Calculer`, `Déterminer` et `Donner`. Ces phrases se **factorisent** au pluriel (règle n°10) : `<Calculer>la dérivée de f, g et h` affiche les trois dérivées, une par ligne — l'argument doit être une pure liste de noms, virgules entre eux, « et » avant le dernier. Le tracé de la même fonction se fait par une phrase en langage naturel :

```docdg
<Soit>q une fonction définie par q(x) = x³ − x
<Représenter>graphiquement la fonction q pour x appartient à [-2 ; 2] et y à [-3 ; 3], avec 200 échantillons
```

### **Tableau de signes**

```docdg
<Soit>f une fonction définie par f(x) = (x+2)(x−3)
<Dresser>le tableau de signes de f
```

---

## **🔢 Les mathématiques**

> **Le cœur de docdg** : Un mini-langage mathématique puissant.

**Syntaxe :** `$EXPRESSION MATHÉMATIQUE$`

### **Fractions et racines**

```docdg
$1/2 + 1/3$
$racine(2)$
$racine[3](8)$
$|x - 2|$
$E(x)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$1/2 + 1/3$` | Fractions enchaînées |
| `$racine(2)$` | Racine carrée |
| `$racine[3](8)$` | Racine cubique |
| `$\|x - 2\|$` | Valeur absolue, entre barres |
| `$E(x)$` | Partie entière (ent(x) et floor(x) tolérés) |


La valeur absolue s'écrit entre barres partout — dans une formule, dans une fonction posée (`<Soit>k une fonction définie par k(x) = |x − 2|`) comme dans une équation à résoudre (`<Résoudre>l'équation |x - 2| = 3`). La partie entière s'écrit `E(x)` — la notation la plus simple — et se rend $E(x)$ (`ent` et `floor` restent compris) ; toutes deux s'évaluent, se tracent et se prêtent aux limites à gauche et à droite (`<Calculer>la limite de pe en 2 à gauche`).

**Le tracé d'une courbe est complet d'office** — même doctrine que la rédaction : dans un repère, `la courbe de la fonction f` embarque ce que dit le tableau de variations. Les tangentes horizontales se tracent aux extremums (avec le point marqué), les asymptotes verticales, horizontales et obliques viennent en tirets, et les fonctions en escalier dessinent leurs marches sans faux traits verticaux, avec le point plein au bord atteint et le cercle ouvert au bord exclu, comme au tableau. Il n'y a rien à demander.

### **Puissances et indices**

```docdg
$x^2$
$x^(a+b)$
$u_n$
$u_(n-1)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$x^2$` | x² |
| `$x^(a+b)$` | x^(a+b) - parenthèses importantes ! |
| `$u_n$` | uₙ |
| `$u_(n-1)$` | uₙ₋₁ |


### **Ensembles de nombres**

Chaque ensemble s'écrit soit avec l'abréviation à double lettre (usage mathématique courant), soit en toutes lettres — les deux formes sont équivalentes :

```docdg
$NN$ ou $naturels$
$ZZ$ ou $entiers$
$DD$ ou $décimaux$
$QQ$ ou $rationnels$
$RR$ ou $réels$
$CC$ ou $complexes$
$PP$ ou $probabilité$
$KK$ ou $corps$
$HH$ ou $quaternions$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$NN$ ou $naturels$` | ℕ |
| `$ZZ$ ou $entiers$` | ℤ |
| `$DD$ ou $décimaux$` | 𝔻 |
| `$QQ$ ou $rationnels$` | ℚ |
| `$RR$ ou $réels$` | ℝ |
| `$CC$ ou $complexes$` | ℂ |
| `$PP$ ou $probabilité$` | ℙ |
| `$KK$ ou $corps$` | 𝕂 |
| `$HH$ ou $quaternions$` | ℍ |


### **Quantificateurs et logique**

```docdg
$pour tout x appartenant à RR$
$pour tout x appartenant à RR$
$pour tout x n'appartenant pas à RR$
$il existe x$
$il n'existe pas de x$
$x appartient à NN$
$x n'appartient pas à NN$
$tel que x appartient à RR$
$(P et Q) ou non P$
$(P et Q) ou non P$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$pour tout x appartenant à RR$` | ∀x ∈ ℝ |
| `$pour tout x appartenant à RR$` | ∀x ∈ ℝ |
| `$pour tout x n'appartenant pas à RR$` | ∀x ∉ ℝ |
| `$il existe x$` | ∃x |
| `$il n'existe pas de x$` | ∄x |
| `$x appartient à NN$` | x ∈ ℕ |
| `$x n'appartient pas à NN$` | x ∉ ℕ |
| `$tel que x appartient à RR$` | \| x ∈ ℝ |
| `$(P et Q) ou non P$` | (P ∧ Q) ∨ ¬P |
| `$(P et Q) ou non P$` | (P ∧ Q) ∨ ¬P |


L'appartenance se dit de trois façons, au choix de la rédaction : `dans` — la
plus brève —, `appartient à` pour le verbe conjugué, et `appartenant à` pour
le participe présent, tournure la plus fréquente dans le supérieur (« pour
tout x appartenant à ℝ »). Chacune se nie en insérant `n'` et `pas` :
`n'appartient pas à`, `n'appartenant pas à`. Le mot `tel que` pose la barre de
condition (∣), la même qu'en probabilités conditionnelles.

Dans une zone mathématique, `et` et `ou` sont les connecteurs logiques (∧, ∨) ; `non` donne ¬ (`négation de` est refusé, avec la forme attendue). Suivi d'une parenthèse, en revanche, `non` désigne le **complémentaire** d'un ensemble ou d'un événement : `non(A)` s'affiche $\overline{A}$.

### **Opérations sur les ensembles**

Chaque opération se dit en toutes lettres, en français :

```docdg
$A inclus dans B$
$A inclus dans ou égal à B$
$A non inclus dans B$
$A non inclus dans ou égal à B$
$A union B$
$A inter B$
$A privé de B$
$vide$
$non(A union B) = non(A) inter non(B)$
$parties de(A)$
$cardinal(A)$
$a congru à b modulo n$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$A inclus dans B$` | A ⊂ B |
| `$A inclus dans ou égal à B$` | A ⊆ B |
| `$A non inclus dans B$` | A ⊄ B |
| `$A non inclus dans ou égal à B$` | A ⊈ B |
| `$A union B$` | A ∪ B |
| `$A inter B$` | A ∩ B |
| `$A privé de B$` | A \ B |
| `$vide$` | ∅ |
| `$non(A union B) = non(A) inter non(B)$` | loi de De Morgan |
| `$parties de(A)$` | 𝒫(A) |
| `$cardinal(A)$` | Card(A) |
| `$a congru à b modulo n$` | a ≡ b [n] |


Les grands opérateurs indexés se disent `réunion de` et `intersection de` :

```docdg
$réunion de(i=1;n) A_i$
$intersection de(i=1;n) A_i$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$réunion de(i=1;n) A_i$` | ⋃ de i=1 à n |
| `$intersection de(i=1;n) A_i$` | ⋂ de i=1 à n |


### **Probabilités : conditionnement et complémentaire**

```docdg
$PP(A sachant B)$
$PP(non(B))$
$répartition de(X; x)$
$densité de(X; t)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$PP(A sachant B)$` | P(A \| B) |
| `$PP(non(B))$` | P(B̄) |
| `$répartition de(X; x)$` | F_X(x) |
| `$densité de(X; t)$` | f_X(t) |


### **Algèbre linéaire**

Les opérateurs suivent les usages du supérieur ; les majuscules sont celles de la tradition (Vect, Tr, Ker...) :

```docdg
$engendré par(u, v)$
$trace(A)$
$déterminant(A)$
$^t{A}$
$A^(-1)$
$comatrice(A)$
$spectre(A)$
$A^*$
$noyau(A)$
$image(A)$
$rang(A)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$engendré par(u, v)$` | Vect(u, v) |
| `$trace(A)$` | Tr(A) |
| `$déterminant(A)$` | det(A) |
| `$^t{A}$` | transposée |
| `$A^(-1)$` | inverse |
| `$comatrice(A)$` | Com(A) |
| `$spectre(A)$` | Sp(A) |
| `$A^*$` | adjoint |
| `$noyau(A)$` | Ker(A) |
| `$image(A)$` | Im(A) |
| `$rang(A)$` | rg(A) |


### **Fonctions mathématiques**

```docdg
$sin(x)^2 + cos(x)^2 = 1$
$arcsin(1/2) = pi/6$
$sinus en degrés(30) = 0,5$
$exp(1) = e$
$ln(e) = 1$
$|-5| = 5$
$norme(vecteur(AB))$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$sinus en degrés(30) = 0,5$` | Sinus en degrés |


### **Sommes et produits**

```docdg
$somme(k=1;n) k^2$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$somme(k=1;n) k^2$` | ∑ₖ=₁ⁿ k² |


**Les opérateurs différentiels, en toutes lettres.** Le gradient, la divergence, le rotationnel et le laplacien s'écrivent comme ils se disent — et sortent dans les notations françaises (`grad`, `div`, `rot`, `Δ`) :

```docdg
$gradient(f)$
$divergence(F)$
$rotationnel(F)$
$laplacien f$
$partielle f / partielle x$
$nabla f$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$gradient(f)$` | grad f |
| `$divergence(F)$` | div F |
| `$rotationnel(F)$` | rot F |
| `$laplacien f$` | Δf   (préfixe, sans parenthèses obligatoires) |
| `$partielle f / partielle x$` | ∂f/∂x |
| `$nabla f$` | ∇f, pour qui préfère la notation nabla |


Le laplacien suit la convention du symbole préfixe : `laplacien f` se lit comme `nabla f`, les parenthèses éventuelles appartiennent à l'opérande. La dérivée partielle s'obtient par la fraction : le mot `partielle` (ou `partiel`) devant chaque membre — le moteur compose alors la fraction aux ∂ droits. Les notations alternatives que vous citeriez dans un énoncé (« noté ∇f ou grad f ») s'écrivent naturellement : `$nabla f$ ou $gradient(f)$`.

```docdg
$produit(k=1;n) k$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$produit(k=1;n) k$` | ∏ₖ=₁ⁿ k |


### **Limites et intégrales**

```docdg
$lim(x->0) sin(x)/x = 1$
$intégrale(x=a;b) f(x)$
$intégrale(x) f(x)$
$intégrale contour(C) f(z)$
$intégrale valeur principale(x=a;b) f(x)$
$intégrale moyenne(x=a;b) f(x)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$lim(x->0) sin(x)/x = 1$` | limₓ→₀ |
| `$intégrale(x=a;b) f(x)$` | ∫ₐᵇ f(x) dx |
| `$intégrale(x) f(x)$` | ∫ f(x) dx (primitive) |
| `$intégrale contour(C) f(z)$` | ∮_C |
| `$intégrale valeur principale(x=a;b) f(x)$` | Valeur principale |
| `$intégrale moyenne(x=a;b) f(x)$` | ⨍ₐᵇ (moyenne) |


**Intégrales multiples :**

```docdg
$intégrale(x=a;b ; y=c;d) f(x;y)$
$intégrale(x=a;b ; y=c;d ; z=e;g) f$
$intégrale surface(S)$
$intégrale volume(V)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$intégrale(x=a;b ; y=c;d) f(x;y)$` | ∬ₐᵇᶜᵈ f(x,y) dy dx |
| `$intégrale(x=a;b ; y=c;d ; z=e;g) f$` | ∭ₐᵇᶜᵈᵉᵍ f dz dy dx |
| `$intégrale surface(S)$` | ∯_S (surface) |
| `$intégrale volume(V)$` | ∭_V (volume) |


### **Dérivées**

```docdg
$dy/dx$
$(d^2 y)/(dx^2)$
$partielle f / partielle x$
$(partielle u)/(partielle t) = (partielle^2 u)/(partielle x^2)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$dy/dx$` | dy/dx |
| `$(d^2 y)/(dx^2)$` | d²y/dx² |
| `$partielle f / partielle x$` | ∂f/∂x |
| `$(partielle u)/(partielle t) = (partielle^2 u)/(partielle x^2)$` | Équation chaleur |


### **Matrices et systèmes**

**Formule compacte, en ligne :**

```docdg
$matrice(1 2 ; 3 4)$
$matrice crochets(1 2 ; 3 4)$
$det(1 2 ; 3 4)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$matrice(1 2 ; 3 4)$` | Matrice avec parenthèses |
| `$matrice crochets(1 2 ; 3 4)$` | Matrice avec crochets |
| `$det(1 2 ; 3 4)$` | Déterminant |


**Bloc, sur plusieurs lignes — parenthèses ou crochets comme délimiteur :**

```docdg
<Afficher>la matrice(
	1	2	3
	-x	0	x^2
)

<Afficher>la matrice[
	1	0
	0	1
]
```

Parenthèses ou crochets s'écrivent une fois, à l'ouverture, et referment le bloc eux-mêmes — ce que l'on tape est ce que l'on voit au rendu. Le verbe seul, suivi de l'objet en toutes lettres (`<Afficher>la matrice(`, refermé par `)` seul — la même convention qui referme `<Tracer>le cercle...`), sépare nettement la consigne de l'objet qu'elle concerne.

La même forme scindée s'applique aux objets **nommés**, gardés pour un calcul ultérieur — la matrice ou le système reçoit alors son nom avant l'ouverture, et les accolades neutres restent le délimiteur (les rangées contiennent déjà des parenthèses ou des crochets, dans une équation ou un coefficient) :

```docdg
<Soit>M une matrice définie par {
	2	1
	1	1
}

<Soit>s un système défini par {
	2x + 3y = 7
	x - y = 1
}
```

équivalent à `<Soit>M une matrice définie par {...}` et `<Soit>s un système défini par {...}`.

Dans ces blocs, la **tabulation** sépare les colonnes et le **saut de ligne** sépare les lignes ; avec parenthèses ou crochets, c'est le **point-virgule** qui sépare les colonnes (voir plus haut). Pour un système, découper la ligne en plusieurs colonnes permet d'aligner sur plusieurs points à la fois (le signe `+` et le signe `=`), pas seulement sur le `=`.

Une balise porte toujours un **verbe d'action** : il n'existe pas de balise `<matrice>` ni `<système>` qui nommerait l'objet sans le poser. C'est `<Soit>` qui déclare, ici comme partout ailleurs.

### **Opérateurs avancés**

```docdg
$grad(f)$
$div(F)$
$rot(F)$
$laplacien(f)$
$dérivée directionnelle(f ; u)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$grad(f)$` | ∇f (gradient) |
| `$div(F)$` | ∇·F (divergence) |
| `$rot(F)$` | ∇×F (rotationnel) |
| `$laplacien(f)$` | Δf (laplacien) |
| `$dérivée directionnelle(f ; u)$` | ∇ᵤf (dérivée directionnelle) |


### **Probabilités et statistiques**

```docdg
$C(n ; k)$
$A(n ; k)$
$factorielle(n)$
$n!$
$PP(A)$
$PP(A sachant B)$
$EE(X)$ ou $espérance(X)$
$variance(X)$
$écart type(X)$
$covariance(X ; Y)$
$normale(mu ; sigma)$
$poisson(lambda)$
$binomiale(n ; p)$
$fonction de répartition(X ; x)$
$densité(X ; x)$
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `$C(n ; k)$` | Coefficient binomial ⁽ⁿₖ⁾ |
| `$A(n ; k)$` | Arrangement Aₙᵏ |
| `$factorielle(n)$` | Factorielle n! |
| `$n!$` | Factorielle (alternative) |
| `$PP(A)$` | ℙ(A) Probabilité |
| `$PP(A sachant B)$` | ℙ(A∣B) Probabilité conditionnelle |
| `$EE(X)$ ou $espérance(X)$` | 𝔼(X) Espérance |
| `$variance(X)$` | Var(X) Variance |
| `$écart type(X)$` | σ(X) Écart-type |
| `$covariance(X ; Y)$` | Cov(X,Y) Covariance |
| `$normale(mu ; sigma)$` | 𝒩(μ,σ²) Loi normale |
| `$poisson(lambda)$` | 𝒫(λ) Loi de Poisson |
| `$binomiale(n ; p)$` | ℬ(n,p) Loi binomiale |
| `$fonction de répartition(X ; x)$` | F_X(x) Fonction de répartition |
| `$densité(X ; x)$` | f_X(x) Densité |


---

## **📐 La géométrie**

Toute la géométrie — plane, dans l'espace, avec ou sans repère — se trace avec l'action [**`<Tracer>`**](#-les-actions) : figures planes, cercle trigonométrique, solides en 3D, points/droites/segments/médiatrices, y compris posés dans un repère. L'analyse se représente graphiquement avec [**`<Représenter>graphiquement`**](#-les-actions) : courbes de fonctions, diagrammes en escalier, statistiques, et le mode analytique dès que son contenu relève de l'analyse. Voir le chapitre **Les actions** pour le détail de chaque cas.

---

## **🧊 La géométrie dans l'espace**

Nouveauté de docdg 2.0, portée par un **moteur de projection cavalière** (fuyante à 0,45/0,35, celle des manuels) : les solides, leurs patrons, le repère de l'espace et la géométrie analytique rédigée.

### Les solides et leurs patrons

Sept solides en perspective cavalière, arêtes cachées en pointillés et cotes en centimètres :

```docdg
<Tracer>le solide cube, d'arête 3 cm
<Tracer>le solide pavé droit, de longueur 4 cm, de largeur 2 cm et de hauteur 3 cm
<Tracer>le solide prisme, de base 3 cm et de longueur 5 cm
<Tracer>le solide pyramide, de base 4 cm et de hauteur 3 cm
<Tracer>le solide cylindre, de rayon 2 cm et de hauteur 5 cm
<Tracer>le solide cône, de rayon 2 cm et de hauteur 4 cm
<Tracer>le solide sphère, de rayon 3 cm
```

Les génératrices du cône sont les tangentes exactes à l'ellipse de base depuis le sommet. Chaque solide dépliable a son patron, prêt à découper — `<Tracer>le patron du cône de rayon 2 cm et de hauteur 4 cm` — pour le cube, le pavé, la pyramide (apothème coté), le cylindre (rectangle de largeur 2πr) et le cône (secteur d'angle 2πr/g).

### Le repère de l'espace

Les points déclarés se placent dans un repère (O ; x, y, z) gradué, avec le chemin de coordonnées en pointillés rouges depuis l'origine — le dessin canonique de Terminale :

```docdg
<Soit>A(1;2;3) et B(3;1;2) 2 points
<Tracer>dans un repère de l'espace les points A et B, le segment [AB] et le vecteur AB
```

Les segments se citent entre crochets, les vecteurs se dessinent en flèches rouges, et les droites déclarées se tracent en vert, découpées aux bords du repère.

### Les droites, les plans, les positions relatives

Une droite se déclare par un point et un vecteur directeur — ou, dans le plan comme dans l'espace, par deux points : `<Soit>d une droite passant par A(1 ; 0) et B(3 ; 2)` — en coordonnées ou par référence à des objets déclarés. Droite et plan s'affichent comme des énoncés, sous leur vrai nom : « Soit la droite *d* passant par le point de coordonnées (1 ; 0 ; 2) et de vecteur directeur de coordonnées (1 ; 1 ; −1). », « Soit le plan 𝒬 d'équation 2*x* + *y* − *z* = 3. » :

```docdg
<Soit>d une droite passant par A(1;0;2) et de vecteur directeur u(1;1;-1)
<Soit>P un plan d'équation 2x + y - z = 3
<Donner>une représentation paramétrique de la droite d
<Donner>le vecteur normal du plan P
<Calculer>la distance du point (1 ; 2 ; 0) au plan P
```

Et les positions relatives se **rédigent pas à pas**, comme une copie modèle — colinéarité des vecteurs, résolution du système, vérification de la troisième équation, conclusion avec le point ou la droite d'intersection :

```docdg
<Soit>d une droite passant par A(1;0;2) et de vecteur directeur u(1;1;-1)
<Soit>d' une droite passant par B(0;1;3) et de vecteur directeur v(2;2;-2)
<Soit>P un plan d'équation 2x + y - z = 3
<Soit>Q un plan d'équation 2x + y - z = 5
<Étudier>la position relative des droites d et d'
<Étudier>la position relative de la droite d et du plan P
<Étudier>la position relative des plans P et Q
```

Chaque cas conclut proprement : droites confondues, strictement parallèles, sécantes en \(I\), ou non coplanaires ; droite incluse, strictement parallèle ou sécante au plan ; plans confondus, strictement parallèles ou sécants selon une droite \(Δ\) donnée en représentation paramétrique.

---

## **📊 Les statistiques**

### **📈 Diagrammes statistiques**

**Syntaxe :** `<Représenter>graphiquement une statistique TYPE données:DONNÉES` — le type se met directement, sans préfixe `type:` (comme `<Dresser>une liste à puces` ou les figures de `<Tracer>`). `données:` reste en clé:valeur : ce sont de vraies données structurées, pas un réglage de style.


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
<Représenter>graphiquement une statistique en barres avec les données {1: 4 | 2: 7 | 3: 2}
<Représenter>graphiquement une statistique en camembert avec les données {Pommes: 5 | Poires: 3 | Bananes: 2}
<Représenter>graphiquement une statistique en histogramme avec les bornes {0 ; 5 ; 10 ; 20} et les effectifs {3 ; 7 ; 2}
```

**Avec variables :**

```docdg
soit notes un dictionnaire de chaînes de caractères et de décimaux tel que notes = {
	"Mathématiques": 15,5 ;
	"Français": 12 ;
	"Histoire": 14
}
<Représenter>graphiquement une statistique en barres avec les données notes
<Représenter>graphiquement une statistique en camembert avec les données notes
```

### **🌳 Arbres de probabilités**

**Syntaxe :** chaque branche s'écrit `Étiquette probabilité`, ses
sous-branches entre accolades :

```docdg
<Construire>un arbre{
    Étiquette 0,5 {
        Sous-étiquette 0,4
    }
}
```

L'arbre se dessine horizontalement, la racine à gauche, et le produit du
chemin — P(A ∩ B) — s'imprime au bout de chaque feuille.

**Exemples :**

```docdg
<Construire>un arbre{
    A 0,3 {
        B 0,6
        !B 0,4
    }
    !A 0,7 {
        B 0,1
        !B 0,9
    }
}

<Construire>un arbre{
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
<Afficher>un filet bleu nuit de 0,35 mm, de largeur 110 mm au centre

<Afficher>une citation bleu nuit{
	Le gris d'un texte est ce qui reste quand on cesse de lire.
}

<Afficher>une citation avec un fond bleu clair et une bordure bleu canard{
	Un fond appelle une marge intérieure : la citation la prend d'elle-même.
}

<Afficher>un cadre de largeur 120 mm au centre avec une bordure bleu marine de 0,6 mm, un fond bleu Alice et des coins arrondis de 3 mm{
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
<Représenter>graphiquement la droite graduée sur [-5 ; 5] {
	|x - 1/2| >= 5/2
}
```

**Syntaxe déclarée (ensemble donné) :**

```docdg
<Représenter>graphiquement la droite graduée sur [-5 ; 5], d'intervalle {[-2 ; 3)} et de points {-4}
```

**Symboles :** `[`/`]` = inclus, `(`/`)` = exclu, `union`, `inf`, `-inf`

---

<a id="lecriture-en-reglure"></a>

## **✒️ L'écriture en réglure**

`<Écrire>en réglure Seyès { … }` pose une **vraie réglure Seyès** — traits forts au pas de l'interligne, trois interlignes fins entre eux, carreaux verticaux, marge rouge — et y écrit **en cursive**. C'est la page de cahier de l'école élémentaire, au millimètre, à l'écran comme au PDF.

```docdg
<Écrire>en réglure Seyès{
	Léa mange une pomme, Lili un abricot et Murielle des fraises des bois.
	L'été est chaud, l'hiver est froid — mais l'automne est doux.

}
```

**La réglure se nomme.** `en réglure Seyès` donne les grands carreaux de l'école élémentaire, avec leurs interlignes fins et leur marge rouge. C'est la désignation du papier réglé, et c'est la seule annonce que la balise accepte.

```docdg
<Écrire>en réglure Seyès{
	Pose et effectue :
			47 × 6

}
```

**Une ligne de source est une ligne réglée** : ce qui est écrit sur une ligne s'écrit sur une ligne, et **une ligne vide est une ligne à remplir** — c'est ainsi qu'on prépare un modèle à recopier. **Une tabulation vaut un carreau** : trois tabulations rentrent de trois carreaux, ce qui pose les opérations en colonne sans compter les espaces.

Trois clés du bloc `document {…}` règlent le cahier : `seyès` nomme la cursive (seule la **Marelle** voyage avec le document ; une autre est cherchée sur le système ou lue dans un `.ttf` voisin), `hauteur` donne l'interligne en millimètres, `tabulation` la largeur du carreau. La taille de l'écriture s'en déduit : elle n'est pas à régler, elle est **imposée par la réglure**, montantes et jambages compris.

**Un objet ne se pose pas sur le cahier.** Un tableau, une figure, une image, un calcul : ces balises sont refusées dans le corps du bloc, et le message le dit. Une page réglée est faite de lignes à écrire ; ce qui n'est pas de l'écriture se compose hors du bloc, où la pleine mesure lui est acquise.

**Ce qui vit sur le cahier, ce sont les lignes, les styles et les mathématiques en ligne.** Un alignement (`<au centre>`), un style nommé (`<consigne>`), une mise en forme au fil de la ligne, et le mode mathématique entre `$` et `$` : une formule courte s'écrit sur la ligne réglée comme le reste.

Une fraction écrite entre `$` et `$` se compose **en style de texte** : à la taille de la phrase, comme le ferait LaTeX. Le style d'affichage — numérateur et dénominateur pleine taille — est réservé aux formules détachées. Sur la réglure, elle **pose sa barre sur le trait**, numérateur au-dessus et dénominateur dans le haut du carreau d'en dessous, **sans décaler la ligne suivante** : aucune ligne n'est réservée sous elle, et tout ce qui suit reste sur les traits. Dans une opération, les signes `+` et `=` se posent sur la barre, et un entier ou une lettre voisins se tiennent sur la ligne, comme au cahier : `$2/5 + 4 = 22/5$`. Il en va de même d'un exposant, d'un indice, d'un radical ou d'une somme en ligne.

```docdg
soit consigne un style défini par <rouge gras>

<Écrire>en réglure Seyès{
	<consigne>Le carré et le cube.
	On écrit $x^2$ pour le carré de $x$, et $x^3$ pour son cube.
	Pour $x = 3$ : $3^2 = 9$ et $3^3 = 27$.

}
```

**La police, la graisse, l'italique et la couleur se disent dans l'annonce**, avec les mots de style ordinaires — la cursive du document est le défaut, et c'est ainsi qu'on écrit en script sur la réglure :

```docdg
<Écrire>en réglure Seyès en ARIAL bleu nuit{
	Pose et effectue :
			47 × 6

}
```

**La taille et l'interligne, en revanche, ne se règlent pas** : ils se déduisent des carreaux, sans quoi l'écriture quitterait les lignes. `14 pt` ou `grand` dans l'annonce sont refusés, et le message le dit. Pour écrire plus grand, c'est `hauteur` qu'on change dans le bloc `document {…}` — la réglure grandit, et l'écriture avec elle.

**Un objet posé sur le cahier n'est pas écrit en cursive** : il reprend la police du document, comme partout ailleurs. Un tableau de variations reste un tableau de variations, sur la réglure comme en pleine page.

**Une réglure ne se pose pas dans une autre.** Une page de cahier n'a qu'une réglure, et le moteur refuse l'imbrication en le disant. C'est **la composition qui porte la réglure**, jamais l'inverse.

**Le cahier d'écolier, en paysage.** Une réglure posée dans une composition **remplit la composition entière** : la nappe réglée court sous toutes les colonnes, chacune avec sa marge rouge et ses carreaux, et le texte passe d'une colonne à la suivante sans quitter les lignes. C'est la double page du petit cahier — marge intérieure large, marge extérieure quasi nulle.

```docdg
document {
	orientation: paysage;
	marges: {15;5;15;0};
	espacements: {0;0;0;25};
}

<Composer>sur 2 colonnes {
<Écrire>en réglure Seyès{
	Lundi 8 septembre

	Mardi 9 septembre

}
}
```

La réglure doit être **le seul contenu de la composition**, sans rien avant ni après : une nappe réglée ne se partage pas avec des colonnes de texte imprimé. Un mélange est refusé, et le message le dit.

Le nombre de carreaux et de lignes n'est jamais à déclarer : **il se déduit** de la disposition, des quatre marges, des quatre espacements et du nombre de colonnes. Le moteur pose le plus grand nombre entier de carreaux et de lignes qui tienne dans la place et laisse le reliquat en blanc : aucun carreau, aucune ligne n'est jamais tronqué, quel que soit le réglage. `docs/CAHIERS.md` donne la règle et les réglages exacts du cahier A4, du petit cahier 17 × 22 et du cahier d'écolier en paysage.

La première partie de `exemples/seyes.txt` montre les usages de classe — phrases modèles, opérations posées, un dessin à légender, une étude de fonction posée sur les lignes — et la seconde la tenue du texte long sur la réglure et les compositions en colonnes.

## **⚗️ La physique-chimie**

Le corpus couvre la physique et la chimie du CE2 à l'agrégation :
751 énoncés et 215 démonstrations de physique-chimie, 234 grandeurs et 122 cadres
d'hypothèses, répartis en 102 domaines — le catalogue complet, par matière et par
niveau, est dans [Redaction.md](Redaction.md). Les relations y sont vérifiées en homogénéité
au moment de la compilation. Depuis la 3.9, les sciences de l'ingénieur s'y
ajoutent, de la première au master 2.

**`<Vérifier>`** soumet au même contrôle une relation écrite dans le document.
Les symboles ne se devinent pas : ils se nomment après `avec`, par le nom que la
grandeur porte au corpus.

```docdg
<Vérifier>l'homogénéité de σ = E ε avec σ contrainte normale, E module d'Young, ε déformation
```

La relation s'écrit comme au tableau : le produit par juxtaposition (`m c^2`,
`mc²`), par `×` ou par `*` ; les lettres grecques telles quelles ou par leur
nom (`σ` ou `sigma`, `ℏ` ou `hbar`) ; `π` est un nombre. Le moteur répond par
la dimension commune aux deux membres, ou nomme l'endroit où l'égalité se rompt.

Deux verbes lui sont propres. **`<Équilibrer>`** ajuste les coefficients
stœchiométriques d'une équation de réaction — nombres entiers minimaux, ions et
électrons compris :

```docdg
<Équilibrer>CH4 + O2 -> CO2 + H2O
<Équilibrer>MnO4^- + Fe^2+ + H^+ -> Mn^2+ + Fe^3+ + H2O
```

**`<Propager>`** rédige la propagation des incertitudes sur un modèle : les
dérivées partielles, la formule de composition quadratique et l'application
numérique sortent complètes.

```docdg
<Propager>l'incertitude sur g = 4pi^2 L/T^2 avec u(L) = 0,001, u(T) = 0,01, L = 0,995, T = 2,004
```

Le détail est donné dans les guides par niveau : [Collège](College.md),
[Lycée](Lycee.md), [Licence](Licence.md), [Master et agrégation](MasterAgregation.md).

## **🕰️ La frise chronologique**

L'histoire entre en scène — et c'est une vraie frise, non une ligne du temps : un **bandeau** gradué, les **périodes dedans**, les **événements dehors** en cartouches reliés à leur date. Une ligne par événement : la date, deux-points, le titre, et, s'il éclaire, le détail **entre parenthèses** en fin de ligne.

```docdg
<Construire>la frise chronologique {
09/11/1989 : Chute du mur de Berlin
20/07/1969 : Premier pas sur la Lune (Neil Armstrong marche sur la Lune.)
12/07/1998 : Victoire en Coupe du monde (La France remporte sa première étoile.)
}
```

**Le bandeau se referme sur une grande pointe** : le temps ne s'arrête pas au bord de la feuille. La frise occupe **toute la largeur utile** de la page, portrait ou paysage.

**L'échelle du temps est graduée** en années rondes, le pas s'ajustant à l'étendue — de l'année au millénaire. Ses verticales descendent d'un bandeau à l'autre : ce qui tombe dans la même colonne est simultané.

Les cartouches **se rangent d'eux-mêmes dans l'ordre du temps**, quelle que soit la saisie, et **ne se chevauchent jamais** : ils se répartissent **de part et d'autre du bandeau**, en alternance le long du temps, et s'étagent sur autant de rangées qu'il faut de chaque côté, un trait de rappel reliant chacun à sa date. **Rien n'est jamais barré** : les traits se tracent tous avant les cartouches, qui sont opaques, et les verticales de la graduation s'interrompent devant chaque cartouche comme devant chaque bandeau — elles ne subsistent que dans les espaces libres, ce qui suffit à faire lire la colonne sans traverser ce qu'on regarde.

La date se lit en `AAAA`, `MM/AAAA` ou `JJ/MM/AAAA` ; **l'année négative est admise** — l'Antiquité s'écrit `-52 : Alésia` — et `vers` dit l'incertitude des sources sans gêner le placement : la date s'imprime toujours telle qu'elle s'est écrite.

**La frise s'annonce « la frise chronologique » et rien d'autre** : la description ne porte aucun complément, et le moteur refuse celui qu'on lui donnerait. La frise ne porte donc pas de titre — celui qui en veut un l'écrit lui-même, au-dessus ou en dessous, comme tout autre titre du document, et le place où sa page le demande :

```docdg
soit h2 un style défini par <gras au centre>

<h2>La Révolution française

<Construire>la frise chronologique {
14/07/1789 : Prise de la Bastille
21/09/1792 : Proclamation de la République
}
```

Un long détail se replie de lui-même sur deux ou trois lignes courtes. La parenthèse s'apparie depuis la fin de la ligne : une parenthèse *dans* la description ne trompe pas la lecture, et une ligne entièrement parenthésée reste un titre.

**Une période est un événement qui dure.** Ses deux dates s'écrivent `de 1914 à 1918`, `1789 - 1799`, `1914 -- 1918` ou `1914 — 1918`, au choix — le tiret simple demande seulement de respirer, pour ne pas se confondre avec le signe d'une année négative —, et elle occupe **l'intérieur du bandeau**, avec son nom, ses bornes et sa description si la place le permet. Les périodes qui se suivent se touchent sans se gêner ; celles qui se chevauchent se partagent la hauteur du bandeau ; celles qui sont trop étroites pour se nommer dedans se nomment dans un cartouche, dehors :

```docdg
<Construire>la frise chronologique {
de 1914 à 1918 : Grande Guerre (Le premier conflit mondial.)
1936 -- 1938 : Front populaire
11/11/1918 : Armistice
06/02/1934 : Crise du 6 février
}
```

**La frise multilinéaire** range ses événements par bandes nommées, qui partagent la même échelle du temps — ce qui met la simultanéité sous les yeux, et permet de confronter deux découpages concurrents :

```docdg
<Construire>la frise chronologique {
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

Les chapitres de `exemples/histoire.txt` suivent les niveaux du programme. **Collège** : la frise linéaire simple et la frise par périodes — ordonner des événements, se repérer dans le temps. **Lycée** : la frise thématique et multilinéaire, et la périodisation des régimes français — articuler cause et effet, problématiser un découpage. **Enseignement supérieur** : les trois temps de Braudel en trois bandes d'échelles différentes, et deux périodisations concurrentes posées l'une sous l'autre — mettre en évidence la simultanéité, questionner la pertinence des découpages.

## **🗄️ Le corpus : le personnaliser, l'emprunter**

Le corpus est la bibliothèque d'énoncés et de démonstrations dans laquelle
`<Démontrer>` et `<Rappeler>` puisent. Depuis la 3.6, il ne se contente plus
d'être consulté : il s'amende.

### Ce que fait l'onglet Édition

**Consulter.** Deux listes déroulantes, la matière et le niveau, mènent à
l'entrée voulue. Un énoncé montre son texte, son genre, ses ancrages — le
niveau où il est exigible, et s'il y est admis ou démontré — ses prérequis et
ses symboles.

**Créer.** Un énoncé neuf se déclare comme les autres : un genre, un titre, un
texte, une matière, au moins un ancrage de niveau. Un prérequis se choisit
**par son titre**, jamais par son identifiant — vous écrivez « Nombre dérivé »,
docdg inscrit ce qu'il faut.

**Modifier.** Toute entrée, publiée ou personnelle, se reprend. La vôtre
remplace celle de la base pour vos documents ; celle de la base reste intacte
pour tout le monde.

**Masquer.** Rien ne se supprime de la base publiée — on la masque. L'entrée
cesse d'exister pour vos documents, et le masque se lève d'un retour en
arrière.

Chaque enregistrement est **contrôlé avant d'être accepté** : un prérequis
qui n'existe pas, une démonstration rattachée à un énoncé absent, une
définition qu'on prétendrait démontrer — docdg refuse et nomme la règle,
plutôt que d'accepter une base incohérente qu'un document révélerait plus
tard.

### Revenir en arrière

Le **journal** retient chaque opération avec sa date, son heure et son
libellé. Les deux flèches de l'onglet Édition s'y déplacent : **◀** défait,
**▶** refait, et le libellé affiché dit où vous êtes. On remonte aussi loin
qu'on veut, jusqu'à la personnalisation vide.

Rien n'est effacé en reculant. La seule opération irréversible est
d'**enregistrer** après avoir reculé : les opérations que vous aviez devant
vous tombent, et docdg vous demande confirmation en vous disant combien.

### Emprunter la base d'un établissement

La clé `corpus` du bloc `document` fait composer un document sur une **autre**
base que la vôtre — celle qu'un établissement ou un collègue a partagée :

```
document {
    niveau: première;
    corpus: college.docdgc;
}
```

Le chemin se lit depuis le dossier du document, comme celui des images. Cette
base **remplace** entièrement celle de docdg : elle n'est pas fusionnée à
votre travail personnel, afin que le document rende exactement ce que son
auteur voyait. Si le fichier manque ou n'est pas lisible, docdg le dit et
compose sur sa base à lui plutôt que de s'arrêter.

### Où vivent vos fichiers

```
Documents/
    docdg/
        personnel.docdgp    votre travail
        journal.docdgp      vos opérations, datées
```

Un dossier visible, deux fichiers ordinaires : pour les mettre à l'abri ou les
emporter sur un autre poste, copiez-les comme n'importe quel fichier. Ils sont
scellés — docdg seul les relit, ce qui garantit qu'aucune manipulation
extérieure ne les abîme à votre insu — et contrôlés au démarrage suivant.

---

## **🗂️ Les exemples, par niveau**

Les recueils thématiques sont désormais **un fichier par domaine**, sans suffixe numérique. Chaque recueil commence par une **table des matières native et cliquable** (`<table des matières>`) : on accède aux chapitres depuis l’aperçu.

| Recueils consolidés | Parcours |
|---|---|
| `exemples/algebre.txt` | Collège, Lycée, Enseignement supérieur, Approfondissements |
| `exemples/algo.txt` | Collège, Lycée, Enseignement supérieur |
| `exemples/analyse.txt` | Collège, Lycée, Enseignement supérieur |
| `exemples/basique.txt` | Bases : écriture et mise en forme, langage complet, **nuancier de 148 couleurs**, atelier interactif |
| `exemples/calcul.txt` | Calcul élémentaire et nombres au collège |
| `exemples/chimie.txt` | Collège, Lycée, Enseignement supérieur, Approfondissements |
| `exemples/demonstration.txt` | Démonstrations rédigées par l'auteur, Démonstrations automatiques : lycée, Démonstrations automatiques : supérieur |
| `exemples/geometrie.txt` | École élémentaire, Collège, Lycée, Enseignement supérieur |
| `exemples/histoire.txt` | École élémentaire, Collège, Lycée, Enseignement supérieur |
| `exemples/math.txt` | Pythagore (quatrième), exponentielle (terminale), complétude (licence 1), groupes finis (master 2) |
| `exemples/mathlang.txt` | Mini-langage mathématique : **271 notations classées en 37 catégories** |
| `exemples/physique-chimie.txt` | Lycée, Enseignement supérieur |
| `exemples/physique.txt` | Collège, Lycée, Enseignement supérieur, Approfondissements |
| `exemples/seyes.txt` | Écriture en réglure Seyès et compositions en colonnes |
| `exemples/si.txt` | Lycée, Classes préparatoires et supérieur, Approfondissements en sciences de l’ingénieur |
| `exemples/statistiques-probabilites.txt` | Collège, Lycée, Enseignement supérieur |

`exemples/vitrine.txt` commence par la réglure Seyès, puis présente sur sa deuxième page une sélection de notations de `mathlang.txt` avant les démonstrations scientifiques et typographiques.

**Organisation des exemples :** les 23 fichiers publics regroupent les démonstrations thématiques du langage. Les réglages `document { … }` s'appliquent à chaque fichier entier ; les démonstrations nécessitant un niveau particulier utilisent une surcharge explicite.

Les exemples numérotés des versions antérieures (`algebre2`, `algebre3`, `demonstration4`…) sont réunis dans ces fichiers, un par thème. Seule la série `publication1` à `publication5` reste séparée, pour garder ses mises en page ; leurs saisies interactives sont préremplies là où une question bloquerait la lecture des chapitres suivants.

---

## **📄 Les documents complexes**

### **Exemple complet : Fiche d'exercices**

```docdg
document {
  marges: 20;
  taille: 12;
  précision: 2;
}

soit titre un style défini par <bleu marine gras 18 pt au centre>
soit sous-titre un style défini par <bleu marine gras 14 pt>
soit exercice un style défini par <bleu marine gras sous-section>
soit cadre-ex un style défini par <cadre avec une bordure bleu marine, un fond bleu Alice et des coins arrondis de 4 mm>
soit important un style défini par <gras rouge souligné>
soit indication un style défini par <italique bleu>

<Afficher>une grille avec un écart de 5 mm{
	<max>	<max>	<50 mm>
	<min>	titre	titre	logo
	<min>	info	info	logo

	titre : au centre {
		<titre>Fiche d'Exercices - Équations et Fonctions
	}
	logo : au centre {
		<Insérer l'image IMAGES/logo.png avec une largeur de 40 mm>
	}
	info : à gauche {
		Nom: _______________ Prénom: _______________

		Classe: _______ Date: ___________
	}
}

<table des matières>

<exercice>Exercice 1 : Équations du second degré

<Afficher>un cadre-ex{
  Résoudre :
  <Dresser>une liste numérotée{
    $x^2 - 5x + 6 = 0$
    $2x^2 - 8x = 0$
    $x^2 - 7 = 0$
  }
  <indication>Indice :</indication> Utilisez $x = (-b +- racine(b^2 - 4ac))/(2a)$.
}

<exercice>Exercice 2 : Géométrie

<Afficher>un cadre-ex{
  <Tracer>le triangle ABC rectangle en A, de côté AB 3 cm et de côté AC 4 cm, avec les marques
  
  Calculer :
  <Dresser>une liste numérotée{
    La longueur de l'hypoténuse BC = #{racine(3^2 + 4^2)} cm
    Le périmètre = #{3 + 4 + racine(3^2 + 4^2)} cm
    L'aire = #{3*4/2} cm²
  }
}

<exercice>Exercice 3 : Étude de fonction

<Afficher>un cadre-ex{
  <Soit>f une fonction définie par f(x) = x² − 4x + 3
  
  <Dresser le tableau de variations de f, x:{-infini | 2 | +infini},
      dérivée:{- | +}, variations:{+infini \ -1 / +infini}>
  <Représenter>graphiquement la fonction f pour x appartient à [-1 ; 5] et y à [-2 ; 4], en bleu, avec 200 échantillons
}

<exercice>Exercice 4 : Statistiques

<Afficher>un cadre-ex{
  Notes : 12 (3x), 14 (12x), 16 (5x)
  
  <Représenter>graphiquement une statistique barres données:{12: 3 | 14: 12 | 16: 5}
  
  Moyenne = #{ (12*3 + 14*12 + 16*5) / (3+12+5) }
}

```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| Rubrique | **Styles réutilisables** |
| Rubrique | **En-tête** |
| Rubrique | **Table des matières** |
| Rubrique | **Exercices** |
| `<Représenter>graphiquement la fonction f pour x appartient à [-1 ; 5] et y à [-2 ; 4], en bleu, avec 200 échantillons` | Courbe |


---

## **🧭 Comprendre les erreurs et les cas particuliers**

Ce chapitre rassemble les points de détail qui, une fois compris, évitent la plupart des blocages. Il répond aux questions que se pose tout utilisateur dès son premier document un peu long.

### **🚦 Les messages d'erreur**

Une tentative d'instruction algorithmique mal formée reste de la prose, sans avertissement. Pour les erreurs d'utilisation, de calcul ou de balise, docdg donne un diagnostic **en français**, à l'emplacement concerné. Les erreurs les plus fréquentes :

| **Ce que vous écrivez**              | **Message docdg**                                              |
| ------------------------------------ | ----------------------------------------------------------------- |
| `<Placer>le point A` sans avoir défini A | *Le point A n'a pas été défini. Déclarez-le d'abord avec `<Soit>A(...) un point`.* |
| `<Représenter>graphiquement la fonction f` sans `<Soit>f : …` | *La fonction f n'a pas été déclarée.*                     |
| `#n` sans déclaration valide de `n` | La variable n'a pas de déclaration valide avant cette utilisation : elle est absente ou sa déclaration ne respecte pas la syntaxe de DocDG. |
| `<Afficher>un cadre{...` (accolade non refermée) | *Accolade ouverte à la ligne N jamais refermée.*                  |
| `<Tracer>un carré ABCD, de côté 5` (unité manquante) | *Longueur sans unité : écrivez « de côté 5 cm » ou « de côté 50 mm ». L'unité est obligatoire hors d'un repère.* |
| `données:{12, 15}` (virgule dans une liste numérique) | *Liste numérique : utilisez le point-virgule (`12 ; 15`).*  |

> **Pourquoi c'est important.** Des messages en français, qui nomment la cause et proposent la correction, sont ce qui distingue un outil pédagogique d'un compilateur. Tout message d'erreur qui ne serait pas en français est un cas non intercepté : signalez-le, il a vocation à être traduit.

### **🔭 La portée des `soit`**

Une déclaration `soit x ∈ ℤ tel que x = ...` est **visible depuis sa déclaration jusqu'à la fin du document**, y compris à l'intérieur des blocs (`<Afficher>un cadre`, boucles `pour`, conditions `si`) ouverts *après* elle. À l'inverse, une variable déclarée **à l'intérieur** d'un bloc ne vit que dans ce bloc :

```docdg
soit tva ∈ 𝔻 tel que tva = 0,2

<Afficher>un cadre{
  soit prix ∈ ℕ tel que prix = 100
  Total : #{prix * (1 + tva)} €
}

```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `soit tva ∈ 𝔻 tel que tva = 0,2` | visible partout ensuite |
| `soit prix ∈ ℕ tel que prix = 100` | visible seulement dans ce cadre |
| Précision | Ici, tva existe encore ; prix, non. |


La variable d'une boucle (`pour n ∈ ⟦1 ; 5⟧`) ne vit que le temps de la boucle. Cette règle est celle qu'on attend naturellement : ce qu'on pose « en tête » sert partout, ce qu'on pose « dans un tiroir » reste dans le tiroir.

### **🔢 Calculs `#{...}` et virgule décimale**

Un calcul `#{...}` rend son résultat avec une **virgule décimale** (règle d'affichage française), arrondi selon l'option `précision` de la classe :

```docdg
#{5/2}
#{1/3}
#{3*4/2}
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `#{5/2}` | → 2,5 |
| `#{1/3}` | → 0,333  (avec précision=3) |
| `#{3*4/2}` | → 6      (résultat entier : pas de virgule) |


**Attention à la collision avec les séparateurs.** Le résultat `2,5` contient une virgule *sans espace après* : le lexer la reconnaît comme décimale, jamais comme énumération (règle n°8). Vous pouvez donc écrire sans crainte `#{5/2}` dans une phrase. En revanche, si vous insérez un calcul **dans une liste numérique** (séparée par `;`), le résultat garde sa virgule décimale et le point-virgule reste le séparateur de liste — aucune ambiguïté :

```docdg
données:{#{5/2} ; #{7/2} ; 4}
```

**Repères sur les instructions de cet exemple** (ces indications sont hors du code docdg) :

| Écriture ou rubrique | Explication |
|---|---|
| `données:{#{5/2} ; #{7/2} ; 4}` | → 2,5 ; 3,5 ; 4 |


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
| `script`     | texte              | `Georgia`   | Police du texte imprimé        |
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

docdg confie automatiquement les calculs symboliques et numériques avancés
à SymPy, embarqué avec son propre Python — aucune option à activer, les phrases de calcul formel appellent le
moteur formel d'elles-mêmes. Chaque résultat s'affiche en notation
française, virgule décimale comprise.

### Le calcul formel (SymPy)

| Phrase | Effet |
|---|---|
| `<Factoriser>EXPR` | Factorisation |
| `<Développer>EXPR` ou `<Développer>et réduire EXPR` | Développement |
| `<Simplifier>EXPR` | Simplification |
| `<Décomposer>en éléments simples EXPR` | Éléments simples |
| `<Calculer>la forme canonique de EXPR` | Trinôme canonique |
| `<Résoudre>l'équation EXPR = EXPR` | Résolution dans ℝ |
| `<Résoudre>dans CC l'équation ...` | Domaine : `RR`, `CC`, `ZZ`, `NN`, `QQ` ou `les complexes`, `les entiers`... |
| `<Calculer>la somme de EXPR pour k de a à b` | Forme close de la somme |
| `<Calculer>le produit de EXPR pour k de a à b` | Forme close du produit |
| `<Calculer>la dérivée de f` / `<Calculer>la dérivée seconde de f` | Dérivées (fonction posée par `<Soit>`) |
| `<Calculer>une primitive de f` | Primitive |
| `<Déterminer>les zéros de f` | Ensemble des zéros |
| `<Calculer>la limite de f en a` (+ `à droite` / `à gauche`) | Limite |
| `<Calculer>le développement limité de f en a à l'ordre n` | DL avec reste de Landau |
| `<Calculer>les racines n-ièmes de z` | Racines complexes, ordinal en toutes lettres (`carrées`... `douzièmes`), `l'unité` admis pour 1 |
| `<Calculer>l'intégrale numérique de f entre a et b` | Valeur approchée |

### Les équations différentielles

La notation prime se lit telle quelle ; l'inconnue et sa variable se
déclarent au besoin :

```
<Résoudre>l'équation différentielle y'' + 4y = 0
<Résoudre>l'équation différentielle N' = -0,12N, d'inconnue N(t)
```

Les équations aux dérivées partielles du premier ordre linéaire
s'écrivent en notation indicée (`u_x`, `u_y`, `u_xx`...) :

```
<Résoudre>l'équation aux dérivées partielles u_x + u_y = u
```

Au-delà du premier ordre, SymPy ne sait pas résoudre symboliquement et
docdg le dit en français.

### L'algèbre linéaire calculée

Une matrice posée par `<Soit>M une matrice définie par {...}` accepte, outre
`<Calculer>le déterminant de M` et `<Calculer>l'inverse de M` :

```
<Calculer>les valeurs propres de M
<Diagonaliser>M
```

Les valeurs propres s'affichent avec leurs multiplicités ; la
diagonalisation donne P et D avec M = P D P⁻¹, ou explique pourquoi la
matrice n'est pas diagonalisable.

### Les équations différentielles, numériquement

Quand une équation différentielle n'a pas de solution exprimable — le
pendule pesant sans approximation des petits angles, le système de Lorenz,
une cinétique à plusieurs étapes —, `numériquement` la confie à
`scipy.integrate.solve_ivp`, embarqué : les valeurs affichées sont exactement
celles d'un TP Python. La méthode par défaut est celle de `solve_ivp`,
**RK45** (Dormand-Prince, Runge-Kutta 5(4) à pas adaptatif), avec ses
tolérances : relative 10⁻³, absolue 10⁻⁶.

```docdg
<Résoudre>numériquement l'équation différentielle θ'' + (g/L)sin(θ) = 0, avec g = 9,81, L = 0,5, θ(0) = 1 et θ'(0) = 0, sur [0 ; 10]
```

L'équation s'écrit sous forme explicite (`y'' = …`) ou implicite, pourvu
qu'elle soit affine en la dérivée d'ordre le plus élevé ; la multiplication
peut rester implicite (`2y`, `xy`, `k sin(x)`). Les paramètres se posent dans
la phrase (`avec k = 0,5`) ou reprennent les variables du document. Un
système du premier ordre s'écrit dans un bloc, une équation par ligne :

```docdg
<Résoudre>numériquement le système différentiel, avec k1 = 1, k2 = 0,5, a(0) = 1, b(0) = 0 et c(0) = 0, sur [0 ; 10] {
	a' = -k1 a
	b' = k1 a - k2 b
	c' = k2 b
}
```

Chaque composante est tracée en fonction du temps, avec sa légende.
`en traçant z en fonction de x` trace une trajectoire dans le plan des
inconnues, `le portrait de phase` celui de y' en fonction de y :

```docdg
<Résoudre>numériquement le système différentiel, avec x(0) = 1, y(0) = 1 et z(0) = 1, sur [0 ; 40], en traçant z en fonction de x, avec une tolérance relative de 10^(-8) et une tolérance absolue de 10^(-10) {
	x' = 10(y - x)
	y' = x(28 - z) - y
	z' = xy - 8/3 z
}
```

| Précision | Effet |
|---|---|
| `par la méthode de Radau` | Méthode de `solve_ivp` : `RK45` (défaut), `RK23`, `DOP853` (haute précision), `Radau`, `BDF` (problèmes raides), `LSODA` (choix automatique) |
| `d'inconnue N(x)` | Nomme l'inconnue et la variable (par défaut `t`) |
| `avec une tolérance relative de 10^(-8)`, `une tolérance absolue de 10^(-10)` | Resserre les tolérances (`rtol` et `atol` de `solve_ivp`) |
| `en traçant y en fonction de t` | Une seule composante en fonction du temps |
| `en rouge`, `en vert`… | Couleur de la première courbe |

Une solution qui explose (`y' = y^2`), une équation non affine en sa dérivée
la plus haute, une condition initiale manquante ou un paramètre non déclaré
donnent un message en français, jamais un tracé faux.

Un **problème raide** — une cinétique où coexistent des constantes de vitesse
d'ordres de grandeur très différents, comme celle de Robertson — épuise les
méthodes explicites : on choisit alors une méthode implicite.

```docdg
<Résoudre>numériquement le système différentiel, avec a(0) = 1, b(0) = 0 et c(0) = 0, sur [0 ; 40], par la méthode de Radau, avec une tolérance relative de 10^(-6) et une tolérance absolue de 10^(-10) {
	a' = -0,04a + 10^4 b c
	b' = 0,04a - 10^4 b c - 3 × 10^7 b^2
	c' = 3 × 10^7 b^2
}
```

Quand des conditions portent sur **les deux bouts** de l'intervalle, docdg
reconnaît un **problème aux limites** et le confie à
`scipy.integrate.solve_bvp` — profil de température, déformée d'une poutre,
chaînette ; il donne alors l'état aux deux extrémités, dont la pente
initiale que l'on cherche souvent :

```docdg
<Résoudre>numériquement l'équation différentielle θ'' = m^2 θ, d'inconnue θ(x), avec m = 3, θ(0) = 1 et θ'(1) = 0, sur [0 ; 1]
```

`jusqu'à ce que y = 0` arrête l'intégration à l'instant où la condition est
atteinte — temps de vol, apogée (`jusqu'à ce que y' = 0`), seuil d'une
concentration — et donne l'état du système à cet instant. Une condition déjà
satisfaite au départ n'est retenue qu'au retour :

```docdg
<Résoudre>numériquement l'équation différentielle y'' = -g, avec g = 9,81, y(0) = 0 et y'(0) = 20, sur [0 ; 10], jusqu'à ce que y = 0
```

Un paramètre collé à l'inconnue se détache de lui-même : `2λx'`, `kx`,
`ωθ'` se lisent 2·λ·x', k·x, ω·θ' dès que λ, k et ω sont déclarés. Chaque
exemple de discipline se clôt sur un chapitre de calcul numérique rédigé :
`physique.txt` (radioactivité, circuit RC, chute avec frottement, spectre
d'une note), `chimie.txt` (cinétique d'ordre 2), `physique.txt` (pendule,
oscillateur amorti, spectres, Bode, filtrage), `chimie.txt` (réactions
successives, cinétique raide) et `analyse.txt` (Van der Pol, Lotka-Volterra,
Lorenz).

### Les réponses temporelles et les lieux de transfert

À partir d'une fonction de transfert \(H(p)\), `scipy.signal` donne la
réponse à un échelon ou à une impulsion, et les lieux de Nyquist et de
Black-Nichols. La réponse indicielle relève ses caractéristiques normalisées :
valeur finale, premier dépassement, temps de montée de 10 % à 90 %, temps de
réponse à 5 % — calculés à la précision des formules du cours
(\(D_1 = e^{-\pi\xi/\sqrt{1-\xi^2}}\), \(t_{5\%} = \tau \ln 20\)).

```docdg
<Tracer>la réponse indicielle de H(p) = 1/(1 + 2ξp/ω0 + p^2/ω0^2), avec ξ = 0,2 et ω0 = 10
```

| Phrase | `scipy.signal` |
|---|---|
| `la réponse indicielle de H(p) = …` | `step`, avec la bande des 5 % et les caractéristiques |
| `la réponse impulsionnelle de H(p) = …` | `impulse` ; l'impulsion de Dirac d'un transfert non strictement propre est signalée |
| `le diagramme de Nyquist de T(p) = …` | lieu complet et symétrique, point critique \(-1\), marges ; recadré sur le point critique quand le module part à l'infini |
| `le diagramme de Black de T(p) = …` | gain en dB selon la phase, point critique \((-180° ; 0\ \text{dB})\), marges |
| `sur [0 ; 5]`, `entre 0,1 et 100 rad/s` | durée d'observation, plage de pulsations |

Les exemples `si.txt` (terminale), `si.txt` (PTSI, PT, licence) et
`si.txt` (master : structures, électronique) les mettent en situation.

### Le spectre d'un signal

`<Tracer>le spectre du signal …` échantillonne un signal, en calcule la
transformée de Fourier rapide par `scipy.fft.rfft` et trace son spectre
d'amplitude unilatéral ; les raies principales sont relevées, fréquence et
amplitude, avec la résolution et la fréquence de Nyquist.

```docdg
<Tracer>le spectre du signal s(t) = sin(2π·50t) + 0,5sin(2π·120t), échantillonné à 1000 Hz sur [0 ; 1], avec une fenêtre de Hann
```

| Précision | Effet |
|---|---|
| `échantillonné à 1000 Hz` (ou `à 2 kHz`) | Fréquence d'échantillonnage |
| `sur [0 ; 1]` | Durée d'observation : la résolution vaut 1/durée |
| `avec une fenêtre de Hann` | Pondération : rectangulaire (défaut), Hann, Hamming, Blackman, flattop ; l'amplitude est corrigée du gain de la fenêtre |
| `avec f = 50` | Paramètres du signal |
| `jusqu'à 500 Hz` | Borne du tracé (au plus la fréquence de Nyquist) |

Un signal carré montre ses harmoniques impaires en 4/(nπ) ; un signal
modulé en amplitude, sa porteuse et ses deux bandes latérales :

```docdg
<Tracer>le spectre du signal u(t) = (1 + m cos(2π fm t)) cos(2π fp t), échantillonné à 10 kHz sur [0 ; 0,5], avec m = 0,5, fm = 100, fp = 1000, jusqu'à 1500 Hz
```

### La géométrie algorithmique et la classification

Sur des points écrits entre accolades, `scipy.spatial` et `scipy.cluster`
calculent et tracent :

| Phrase | Méthode |
|---|---|
| `<Tracer>l'enveloppe convexe des points {(0;0) (4;0) …}` | `ConvexHull` : sommets, aire, périmètre |
| `<Tracer>la triangulation de Delaunay des points {…}` | `Delaunay` : \(2n - h - 2\) triangles |
| `<Tracer>le diagramme de Voronoï des points {…}` | `Voronoi`, cellules du bord tronquées au cadre |
| `<Calculer>les deux points les plus proches parmi les points {…}` | `pdist` sur les \(n(n-1)/2\) paires |
| `<Tracer>la classe du point (3 ; 3) par les 3 plus proches voisins parmi {(1;1) A, (6;6) B, …}` | `cKDTree` et vote ; `par le plus proche voisin` pour \(k = 1\) |
| `<Tracer>la classification en 3 groupes des points {…}` | k-moyennes (`kmeans2`, k-means++, graine fixe), centres et inertie |
| `<Tracer>le dendrogramme des points {…} en 3 groupes` | classification ascendante hiérarchique de Ward, coupe en groupes |

Les points sont numérotés \(P_1, P_2, …\) dans le texte comme sur la figure ;
des points alignés ou confondus sont signalés en français.

### SciPy dans docdg, en un coup d'œil

Les fichiers cités sont les **recueils publics consolidés** : leurs chapitres
regroupent plusieurs niveaux.

| Domaine | Ce que docdg en tire | Exemples |
|---|---|---|
| `integrate` | équations différentielles (6 méthodes, raides comprises), problèmes aux limites, événements, intégrales doubles et triples | `physique.txt`, `chimie.txt`, `analyse.txt`, `si.txt` |
| `fft`, `signal` | spectres, filtrage, Bode, réponses indicielle et impulsionnelle, Nyquist, Black | `physique.txt`, `si.txt` |
| `optimize`, `stats` | ajustement avec incertitudes, programmation linéaire, intervalles de confiance, tests | `physique-chimie.txt`, `algebre.txt`, `statistiques-probabilites.txt` |
| `interpolate` | splines, Lagrange, PCHIP, lissage, dérivée (titrage) | `chimie.txt`, `physique.txt`, `analyse.txt` |
| `linalg` | LU, QR, Cholesky, valeurs singulières, conditionnement, exponentielle, valeurs propres | `algebre.txt`, `si.txt` |
| `special`, `constants` | Bessel, polynômes orthogonaux, Γ, ζ, Airy, intégrales elliptiques, constantes CODATA | `analyse.txt`, `physique.txt`, `si.txt` |
| `spatial`, `cluster` | enveloppe convexe, Delaunay, Voronoï, plus proches voisins, k-moyennes, dendrogramme | `algo.txt`, `statistiques-probabilites.txt` |

### Les constantes physiques et les fonctions spéciales

`<Donner>la constante de Planck` donne la valeur CODATA 2022 de
`scipy.constants`, avec son unité et son statut : **exacte** pour les sept
constantes qui définissent le SI depuis 2019 (et celles qui s'en déduisent),
**mesurée** pour les autres, avec leur incertitude-type relative. Les chiffres
sont groupés par trois, à la française : \(c = 299\,792\,458\ \mathrm{m\,s^{-1}}\).

Vitesse de la lumière, constantes de Planck (et réduite), de Boltzmann,
d'Avogadro, charge élémentaire, constante de gravitation, masses de
l'électron, du proton et du neutron, permittivité et perméabilité du vide,
constantes des gaz parfaits, de Faraday, de Stefan-Boltzmann, de Rydberg,
de structure fine, de Wien, rayon et magnéton de Bohr, unité de masse
atomique, électron-volt, accélération normale de la pesanteur.

Les fonctions spéciales viennent de `scipy.special` :

| Phrase | Fonction |
|---|---|
| `<Tracer>les fonctions de Bessel J_0, J_1 et J_2 sur [0 ; 20]` | Bessel \(J_n\), \(Y_n\), modifiées \(I_n\), \(K_n\) |
| `<Calculer>les 5 premiers zéros de la fonction de Bessel J_0` | zéros \(j_{n,k}\) et \(y_{n,k}\) |
| `<Tracer>les polynômes de Legendre de degré 0 à 4 sur [-1 ; 1]` | Legendre, Hermite, Laguerre, Tchebychev |
| `<Calculer>la fonction Gamma en 0,5`, `la fonction zêta de Riemann en 2` | \(\Gamma\), \(\zeta\) ; `<Tracer>la fonction d'Airy Ai sur …` |
| `<Calculer>les intégrales elliptiques complètes de module 0,5` | \(K(k)\) et \(E(k)\) |
| `<Calculer>la période du pendule simple d'amplitude 2,5 rad` | \(T = 4\sqrt{L/g}\,K(\sin\theta_0/2)\), comparée à \(T_0\) |

### L'algèbre linéaire numérique

Sur une matrice déclarée par `<Soit>A une matrice définie par { … }`, `scipy.linalg`
décompose, conditionne et exponentie ; les coefficients s'affichent en
fractions quand ils en sont, en décimales sinon. Le calcul exact (déterminant,
inverse, valeurs propres exactes, diagonalisation) reste à SymPy.

```docdg
<Soit>A une matrice définie par {
	0 ; 2 ; 1
	1 ; 1 ; 1
	2 ; 1 ; 3
}
<Calculer>la décomposition LU de A
```

| Phrase | `scipy.linalg` |
|---|---|
| `la décomposition LU de A` | `lu` : \(A = PLU\), pivot partiel |
| `la décomposition QR de A` | `qr` : \(Q\) orthogonale, \(R\) triangulaire |
| `la décomposition de Cholesky de A` | `cholesky` : \(A = LL^{\mathsf{T}}\), symétrie et définie-positivité vérifiées |
| `les valeurs singulières de A` | `svdvals`, rang numérique |
| `le conditionnement de A` | \(\kappa_2 = \sigma_{\max}/\sigma_{\min}\) et chiffres perdus |
| `l'exponentielle de A` | `expm` : \(e^{A}\), solution de \(X' = AX\) |
| `numériquement les valeurs propres de A` | `eigvals`, au-delà du degré 4 |

### Les intégrales multiples, numériquement

`<Calculer>l'intégrale double de …` calcule exactement, par SymPy ;
`<Calculer>numériquement l'intégrale double de …` (ou `triple`) confie le
calcul à `scipy.integrate.dblquad` ou `tplquad`, pour les intégrandes sans
primitive exprimable. La borne d'une variable peut dépendre des variables
précédentes : `[0 ; 1] × [0 ; x]` décrit le triangle \(0 \leqslant y \leqslant x \leqslant 1\).
Le disque (`sur le disque de rayon 2`) est reconnu, les variables se nomment
(`de f(u ; v) = …`) et les paramètres se posent (`avec k = 4`).

```docdg
<Calculer>numériquement l'intégrale double de exp(-(x^2 + y^2)) sur [0 ; 1] × [0 ; x]
```

### Les intervalles de confiance et les tests statistiques

`<Calculer>l'intervalle de confiance …` et `<Effectuer>le test … ` confient
l'inférence à `scipy.stats`. Chaque test énonce \(H_0\) et \(H_1\), donne la
statistique, les degrés de liberté et la p-valeur, puis la décision au seuil
choisi (5 % par défaut) ; pour Student et le \(\chi^2\), la densité de la loi
montre la zone de rejet et la valeur observée.

```docdg
<Effectuer>le test de Student de l'hypothèse μ = 12 sur la série 12,3 ; 11,9 ; 12,6 ; 12,1 ; 12,8 ; 12,4 ; 12,2 ; 12,7, au seuil de 5 %
```

| Phrase | Méthode de `scipy.stats` |
|---|---|
| `l'intervalle de confiance à 95 % de la moyenne de la série …` | Loi de Student |
| `l'intervalle de confiance à 95 % de la proportion pour 42 succès sur 100` | Approximation normale et intervalle exact de Clopper-Pearson |
| `le test de Student de l'hypothèse μ = 12 sur la série …` | `ttest_1samp` |
| `le test de Student d'égalité des moyennes des séries … et …` | `ttest_ind` (Welch, variances inégales) |
| `… des séries appariées … et …` | `ttest_rel` |
| `le test du khi-deux d'adéquation des effectifs … à la loi uniforme` | `chisquare` ; ou `aux effectifs théoriques …`, `aux probabilités …` |
| `le test du khi-deux d'indépendance du tableau {20 ; 30 \| 15 ; 25}` | `chi2_contingency` (Yates pour un tableau 2 × 2) |
| `le test de normalité de Shapiro-Wilk de la série …` | `shapiro` |
| `au seuil de 1 %`, `unilatéral à droite`, `à gauche` | Seuil et hypothèse alternative |

### L'interpolation et le lissage

`<Tracer>l'interpolation des données {…}` fait passer une courbe par des
mesures, par `scipy.interpolate`, et la trace avec les points mesurés.
`avec sa dérivée` ajoute la courbe dérivée et repère son extremum, c'est-à-dire
le point d'inflexion : l'équivalence d'un titrage pH-métrique.

```docdg
<Tracer>l'interpolation de pH en fonction de V des données {(0;3) (2;3,4) (4;3,8) (6;4,2) (8;4,7) (9;5,1) (9,5;5,5) (10;7) (10,5;10,4) (11;10,9) (12;11,3) (14;11,7) (16;11,9)} par une spline cubique, avec sa dérivée
```

| Précision | Méthode de `scipy.interpolate` |
|---|---|
| `par une spline cubique` (défaut) | `CubicSpline`, conditions « not-a-knot » |
| `par une spline naturelle` | `CubicSpline`, dérivées secondes nulles aux bords |
| `par une interpolation linéaire` | affine par morceaux |
| `par le polynôme de Lagrange` | `BarycentricInterpolator` — le phénomène de Runge |
| `par une interpolation monotone` (ou `PCHIP`) | `PchipInterpolator`, sans dépassement entre deux mesures |
| `par une interpolation d'Akima` | `Akima1DInterpolator` |
| `par une spline de lissage` | `make_smoothing_spline`, lissage choisi par validation croisée |
| `de pH en fonction de V` | Nomme les grandeurs des axes |
| `en V = 10` (ou `en x = 1,5 ; 2,5`) | Valeurs de la courbe en ces points |

docdg n'extrapole pas : une valeur demandée hors de l'intervalle des mesures,
comme deux mesures de même abscisse, donne un message en français.

### La programmation linéaire

`<Résoudre>le programme linéaire : …` optimise une fonction linéaire sous des
contraintes linéaires par `scipy.optimize.linprog` (méthode HiGHS). docdg
donne l'optimum et le point où il est atteint — en valeur exacte quand elle
est rationnelle —, et, à deux variables, trace le polygone des contraintes,
la droite de niveau optimale et le sommet optimal.

```docdg
<Résoudre>le programme linéaire : maximiser z = 3x + 2y sous les contraintes x + y ≤ 4, x + 3y ≤ 6, x ≥ 0 et y ≥ 0
```

L'objectif se nomme (`z = …`) ou non ; les contraintes, séparées par des
virgules, `et` ou `;`, emploient `≤`, `≥`, `=` (ou `<=`, `>=`) ; les variables
indicées (`x1`, `x2`…) s'impriment en indice. Un programme sans solution ou
non borné est diagnostiqué en toutes lettres.

### Le filtrage et le diagramme de Bode

`<Tracer>le diagramme de Bode de H(p) = …` trace le gain en décibels et la
phase d'une fonction de transfert, sur une échelle logarithmique en pulsation,
par `scipy.signal.freqs`. docdg relève le gain statique, la ou les pulsations
de coupure à −3 dB (|H| = H(0)/√2), la résonance et, quand le gain franchit
0 dB, la pulsation de coupure à 0 dB et la marge de phase — celle d'une
boucle ouverte d'asservissement.

```docdg
<Tracer>le diagramme de Bode de H(p) = K/(1 + 2ξp/ω0 + p^2/ω0^2), avec K = 2, ξ = 0,2 et ω0 = 100
```

La plage de pulsations se choisit seule, deux décades de part et d'autre des
pôles et des zéros ; `entre 0,01 et 1000 rad/s` l'impose. La variable peut
s'appeler `p` ou `s`.

`<Tracer>le filtrage du signal …` conçoit un filtre numérique par
`scipy.signal` (sections du second ordre, la forme numériquement stable) et
l'applique au signal échantillonné ; le signal d'origine et le signal filtré
sont tracés ensemble.

```docdg
<Tracer>le filtrage du signal s(t) = sin(2π·5t) + 0,3sin(2π·120t), échantillonné à 1000 Hz sur [0 ; 1], par un passe-bas de Butterworth d'ordre 4 à 20 Hz, avec un bruit d'écart type 0,2
```

| Précision | Effet |
|---|---|
| `par un passe-bas … à 20 Hz`, `passe-haut … à 50 Hz` | Filtre et fréquence de coupure |
| `par un passe-bande … entre 40 et 60 Hz`, `coupe-bande …` | Filtre à deux fréquences |
| `de Butterworth` (défaut), `de Tchebychev … à 1 dB d'ondulation` | Famille du filtre |
| `d'ordre 4` | Ordre (obligatoire) |
| par défaut | Filtrage aller-retour `sosfiltfilt`, sans déphasage — celui des TP |
| `en temps réel` | Filtrage causal `sosfilt`, retardé comme un filtre physique |
| `avec un bruit d'écart type 0,2` | Bruit gaussien ajouté, tiré d'une graine fixe : le document est reproductible |

### Le calcul numérique (SymPy)

| Phrase | Effet |
|---|---|
| `<Résoudre>numériquement l'équation EXPR = EXPR sur [a ; b]` | Racine numérique, cherchée à partir du milieu de l'intervalle — l'intervalle guide le point de départ |
| `<Calculer>la probabilité que X <= a pour la loi normale(m ; s)` | Probabilité ; opérateurs `<=`, `>=`, `=`, `<`, `>` |
| `<Calculer>le quantile d'ordre p de la loi ...` | Quantile |
| `<Calculer>l'espérance de la loi ...` / `<Calculer>l'écart type de la loi ...` | Moments |
| `<Ajuster>f(x) = a exp(b x) aux données {(0;1) (1;2,7) ...}` | Moindres carrés non linéaires par `scipy.optimize.curve_fit` : chaque paramètre avec son incertitude-type u, tirée de la matrice de covariance, et le coefficient de détermination R² — le tout dans la balise, les données comprises |

Les lois reconnues : `normale(m ; s)`, `binomiale(n ; p)`,
`poisson(l)`, `uniforme(a ; b)`, `exponentielle(l)` (paramètre λ, à la
française), `student(k)`, `khi-deux(k)`.

<a id="les-tournures-et-le-placement"></a>

## **🖋️ Les tournures et le placement**

### Les verbes d'objets

Chaque phrase peut commencer par un verbe : `<Afficher>...`,
`<Dresser>...`, `<Construire>...` et `<Insérer>...` s'acceptent devant
tout objet, article compris.

```
<Dresser>un tableau bordures entête{ ... }
<Afficher>une liste à puces{ ... }
<Insérer l'image IMAGES/photo.png avec une largeur de 30 mm>
<Dresser>le tableau de signes de f
```

### Les verbes non encore illustrés

Six verbes du moteur ne paraissaient nulle part dans ce manuel. Ils sont
pourtant éprouvés par les tests et employés dans les exemples livrés :

| verbe | ce qu'il fait | forme |
| --- | --- | --- |
| `<Convertir>` | change d'unité, en respectant la dimension et les chiffres significatifs de la donnée | `<Convertir>90 km/h en m/s` |
| `<Effectuer>` | pose une opération et la déroule | `<Effectuer>la division euclidienne de 152 par 6` |
| `<Compléter>` | remplit ce qui manque, marqué `?` | `<Compléter>le tableau de proportionnalité {…}` |
| `<Exprimer>` | met en formule un énoncé en prose | `<Exprimer>le programme de calcul en fonction de x {…}` |
| `<Orthonormaliser>` | applique le procédé de Gram-Schmidt | `<Orthonormaliser>la famille u et v` |
| `<Trigonaliser>` | réduit une matrice à sa forme triangulaire | `<Trigonaliser>D` |

### Les tournures du supérieur

`<On pose>`, `<On considère>` et `<On note>` sont synonymes de `<Soit>`
et s'affichent avec leur verbe. `<Étudier>les variations de f` vaut le
tableau de variations.

### La factorisation des points

Les points se regroupent sous un article pluriel :
`<Soit>A(2;3) et B(-1;4) 2 points` — la forme « un point... et un
point... » est refusée avec un rappel de la règle.

### Les bornes naturelles

```
<Représenter>graphiquement la fonction f sur [-2 ; 5], en ordonnée [0 ; 3], en bleu
<Représenter>graphiquement la droite graduée sur [-3 ; 4], d'intervalle {[-2 ; 3[} et de points {1}
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

Le plan se dessine dans le corps de la grille : une ligne de largeurs
au-dessus des colonnes, puis une ligne par rangée qui commence par sa
hauteur et nomme ses zones. Chaque zone se décrit ensuite par
`nom : placement, avec propriétés {contenu}`, les propriétés en toutes
lettres :

```
<max>	<max>	<40 mm>
<min>	titre	titre	logo
<15 cm>	corps	corps	logo

titre : en haut à gauche, avec une bordure bleu marine et un fond bleu Alice { ... }
corps : au centre { ... }
```

Au niveau de la grille, `bordures`, `une bordure <couleur>`,
`fond <couleur>` et `texte <couleur>` posent les défauts que chaque
zone peut surcharger. Une rangée de cadres côte à côte s'obtient par
`<Afficher>une rangée, écart de 5 mm{ ... }`.

### Les données nommées

Une table de données se déclare et se réutilise par son nom, ses
entrées en `clé: valeur` comme les données en ligne :

```
soit sondage un dictionnaire de chaînes de caractères et d'entiers tel que sondage = {
	"Jeudi": 8 ;
	"Lundi": 5 ;
	"Mardi": 3
}

<Représenter>graphiquement une statistique en barres horizontal données:sondage
```

---

Les retours d'usage (bugs, tournures manquantes, besoins non couverts) sont les bienvenus sur le dépôt du projet.
