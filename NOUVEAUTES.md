# Nouveautés de DocDG 4.1

DocDG 4.1 ajoute les tables de données, un SQL rédigé en français et les
statistiques du supérieur, et rend chaque résultat composé plus sûr. Les
documents écrits avec la 4.0 s'ouvrent tels quels ; vos fiches personnelles du
corpus sont reprises automatiquement au premier lancement.

## Les tables de données

- **Lire une table** depuis un fichier CSV, Parquet ou Arrow, ou depuis une
  base DuckDB ou SQLite : `<Lire>le fichier "notes.csv"`,
  `<Lire>la table "ventes" de la base "magasin.duckdb"`. Les dates, les
  booléens et les valeurs manquantes sont conservés.
- **Une table sans entêtes** reçoit les siens (`avec les entêtes "a", "b" et
  "c"`), et une liste, un dictionnaire ou une matrice devient une table avec
  `table(…)`.
- **Interroger** une table en SQL ou en français avec `<Interroger>` : filtrer,
  regrouper, joindre, trier, calculer par fenêtre, modifier une table
  (`<Insérer>`, `<Mettre à jour>`, `<Supprimer>`). Une requête lit aussi
  directement les fichiers CSV, Parquet et JSON du dossier du document. Voir
  [Le SQL en français](docs/SQL_FRANCAIS.md) et
  [Interroger des données](docs/DONNEES_DUCKDB_D2.md), qui explique aussi
  comment apporter une base tenue par PostgreSQL, MySQL, SQL Server, Oracle ou
  Access.
- **Feuilleter** une table page par page depuis le ruban Fichier, et
  l'**exporter** en Parquet ou en Arrow :
  [Feuilleter et exporter une table](docs/DONNEES_ARROW_D3.md).
- Les statistiques et les graphiques prennent directement les colonnes d'une
  table : `<Calculer>la moyenne de la série releves["Maximale"]`.

## Les statistiques et probabilités du supérieur

Six guides accompagnent les nouvelles commandes, toutes proposées par le
bandeau (622 modèles) :

- [lois conjointes et simulations](docs/STATISTIQUES_PROBABILITES_S1.md) ;
- [inférence statistique](docs/STATISTIQUES_INFERENCE_S2.md) : estimateurs,
  intervalles de Wilson et de Clopper-Pearson, test binomial exact,
  Mann-Whitney, Wilcoxon ;
- [régression, ACP, ANOVA](docs/STATISTIQUES_MULTIVARIEES_S3.md) ;
- [chaînes de Markov, vraisemblance, séries chronologiques](docs/STATISTIQUES_PROBABILITES_S4.md) ;
- [lois de probabilité et statistique descriptive](docs/LOIS_ET_DESCRIPTION_S5.md) :
  dix lois nouvelles, centiles, asymétrie, aplatissement, courbe de Lorenz,
  indice de Gini ;
- [tests et modèles statistiques](docs/TESTS_ET_MODELES_S6.md) : corrélation,
  Fisher, Welch, comparaisons multiples, puissance et effectif, prévision ;
- [couples, chaînes de Markov et estimation](docs/PROBABILITES_ESTIMATION_S7.md) :
  méthode des moments, risque quadratique, intervalles bootstrap.

Le corpus compte 1 674 énoncés et 608 démonstrations, dont les fiches de cours
de ces chapitres, appelées par `<Énoncer>` et `<Démontrer>`.

## Des résultats plus sûrs

- Un nombre écrit avec ses milliers garde sa valeur : « 4 000 » vaut 4000
  partout. Deux nombres accolés sans opération (« 4000 000 ») sont refusés avec
  un message, au lieu d'être multipliés.
- Un décimal saisi est un nombre exact : `<Résoudre>l'équation 2x + 1 = 7,5`
  donne 3,25, et une conversion est une égalité exacte (125 m = 0,125 km).
- Les inéquations à une inconnue se résolvent dans ℝ et rendent leurs
  solutions en intervalles.
- Une limite demandée sans côté n'est donnée que si les deux côtés
  s'accordent ; une loi normale lit ses paramètres décimaux ou négatifs ; une
  probabilité n'est jamais négative.
- Un patron qui tient dans la page s'imprime en vraie grandeur et le dit ; un
  patron trop grand porte son échelle.
- Les écritures du supérieur sont comprises : `M^3`, `M³`, `M^(-1)`, une suite
  déclarée par ses indices (`u_0 = 1 et u_(n+1) = 2u_n + 1`).

## L'application

- **L'enregistrement ne peut plus abîmer un document** : en cas d'échec, le
  fichier garde sa version précédente, le texte reste à l'écran et « Enregistrer
  sous » permet de l'écrire ailleurs.
- La frappe reste fluide dans les très longs documents, et l'aperçu se met à
  jour en quelques millisecondes.
- Le bandeau se filtre par matière et par niveau ; une couleur choisie entoure
  la sélection au lieu de la remplacer.
- [L'interface](docs/INTERFACE.md) : rubans, volets, saisies et export.

Les retours d'usage sont les bienvenus : docdg69@gmail.com.
