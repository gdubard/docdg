# Guide de démarrage de docdg 3.9

Ce guide accompagne l'application distribuée. La documentation détaillée se trouve dans
`README.md` et dans les manuels **École**, **Collège**, **Lycée** et **Licence** fournis
avec le projet source. La licence de l'application figure dans `CLUF.md`.

## Ouvrir un exemple

1. Lancer l'application pour son système (exécutable Windows, application macOS ou paquet Linux).
2. Dans **Fichier > Ouvrir**, choisir un fichier `.txt` dans le dossier `exemples/`.
3. Modifier le texte dans la zone source et consulter l'aperçu.
4. Utiliser la fonction d'export PDF de l'application pour produire la version à imprimer.

Les exemples publics de la version 3.9 comptent **28 fichiers `.txt`**, dont
**12 recueils thématiques** (algèbre, algorithmique, analyse, bases, chimie,
démonstrations, géométrie, histoire, physique-chimie, physique, sciences de
l'ingénieur, statistiques et probabilités). Les ressources des documents sont
rangées dans `exemples/IMAGES/`, `exemples/FIGURES/` et `exemples/histoire/`.

Les archives numérotées des exemples historiques ne sont **pas** livrées avec
la distribution utilisateur : elles sont conservées dans le dépôt de développement
sous `tests/fixtures/exemples-historiques/` pour les tests de non-régression.

## Le typage est obligatoire en version 3.9

**Règle sans dérogation pour les variables :** toute déclaration de valeur ou de style comporte le nom, le deux-points, le type **avec son article**, puis `=` et la valeur. Cette règle s'applique aux nombres, textes, listes, dictionnaires, vecteurs, matrices, styles, résultats de saisie, paramètres et types de retour des fonctions ainsi qu'aux indices déclarés dans une boucle.

```docdg
soit n: un entier naturel = 12
soit q: un rationnel = 1/3
soit z: un complexe = 1 + 2i
soit titre: un style = <bleu gras au centre>
soit notes: une liste de décimaux = {12,5 ; 15 ; 9,5}
soit affiche(n: un entier naturel): un entier naturel = n
n = 13
```

**Interdit :** `soit n = 12` (type absent), `soit n: entier naturel = 12` (article absent), `soit titre = <bleu gras>` (style non déclaré). **Une réaffectation** s'écrit `n = 13` : elle ne recrée pas une variable et ne répète pas son type.

Les constructions `<Soit>un point A` (énoncé mathématique) et `soit n: un entier naturel = 12` (déclaration algorithmique) sont différentes. Les gabarits textuels historiques avec paramètres ne constituent pas une autorisation de déclarer des valeurs sans type. Si une ancienne écriture non typée demeure acceptée par un parcours du moteur, elle est non conforme à la règle et doit être corrigée avant publication.

Référence normative : [`docs/TYPAGE_OBLIGATOIRE.md`](docs/TYPAGE_OBLIGATOIRE.md).

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

Les conventions décrites dans `docs/VECTEURS.md` et `docs/NOMBRES.md` du dépôt
source comprennent aussi des **propositions de grammaire à compléter** : elles
ne doivent pas être prises comme une garantie que chaque tournure est disponible.
Les références exécutables sont les commandes effectivement reconnues par la
version installée, accompagnées des tests de non-régression du moteur.

Pour signaler un défaut, joindre la version de docdg, le système d'exploitation,
le fichier `.txt` minimal qui reproduit l'erreur et, si nécessaire, le PDF produit.
