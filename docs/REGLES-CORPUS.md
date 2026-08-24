# Les règles de validation du corpus

Ces règles sont appliquées par `corpus/src/chargement.rs` au moment de la
compilation, et font échouer `cargo build` si l'une d'elles est violée. Elles
sont énumérées ici pour qu'on puisse les connaître sans lire le code, et pour
qu'un outil externe puisse les reproduire.

Pour valider sans reconstruire l'application :

```
cargo xtask corpus
```

## Trois degrés de gravité

| degré | effet |
|---|---|
| **erreur** | fait échouer la compilation |
| **anomalie** | fait échouer la compilation, mais signale une incohérence de déclaration plutôt qu'une donnée fausse |
| **lacune** | simple décompte : un énoncé annoncé démontré dont la démonstration reste à écrire, déclaré par `demonstration_prevue` |

## Les règles

### Identité et vocabulaire

1. **Unicité des identifiants** sur l'ensemble des fichiers, énoncés,
   démonstrations, grandeurs et cadres confondus.
2. **Unicité des titres et des alias repliés.** `<Énonce>` résout par titre
   replié — minuscules, accents ôtés, espaces normalisés : deux entrées
   homonymes rendraient le résultat arbitraire.
3. **Vocabulaire fermé** pour `type`, `statut`, `niveau`, `voie` et `matiere` :
   toute valeur hors des listes de `corpus.toml` et `niveaux.toml` est refusée.
4. **Un alias ne peut coïncider** avec un identifiant existant ni avec un autre
   alias.

### Références

5. **Résolution de toute référence** : `depend_de`, `mentions`, `enonce_id`,
   `hypotheses`, et chaque valeur de `symboles`.
6. **Acyclicité** du graphe `depend_de`.

### Cohérence des ancrages

7. **Tout énoncé porte au moins un ancrage** et au moins un domaine.
8. **Une définition ne se démontre pas.** Un énoncé de type `définition` ou
   `axiome` ne peut être ancré `démontré` : ce qui se démontre est une
   propriété, un théorème ou une relation.
9. **Un ancrage `démontré` exige une démonstration** au même niveau, ou le
   drapeau `demonstration_prevue` qui déclare la lacune.
10. **Le drapeau `demonstration_prevue` est refusé** si la démonstration
    existe, ou si l'ancrage est `admis`.

### Démonstrations

11. **Monotonie des niveaux.** Une démonstration rédigée à un niveau ne peut
    citer, par `depend_de` ou par `mentions`, un énoncé disponible seulement à
    un niveau supérieur.
12. **Pas de dépendance tue.** Si le corps d'une démonstration nomme le titre
    d'un énoncé du corpus, celui-ci doit figurer dans `depend_de` s'il est une
    étape, dans `mentions` s'il n'est que cité. Les titres de moins de dix-huit
    caractères sont ignorés, trop susceptibles d'apparaître par hasard.

### Dimensions

13. **Toute `dimension` s'analyse** dans la syntaxe des sept dimensions de
    base, exposants en douzièmes.
14. **Toute `relation` est homogène** dans le contexte construit à partir de
    ses `symboles` : les deux membres d'une égalité s'accordent, les termes
    d'une somme aussi, l'argument d'une fonction transcendante est sans
    dimension.
15. **Un exposant fractionnaire s'écrit entre parenthèses** — `x^(1/2)`. Sans
    cette exigence, la barre de `p^2 / Z` serait happée comme celle d'un
    exposant rationnel et la division disparaîtrait silencieusement.
16. **`formule` et `relation` vont de pair** : l'une sans l'autre est signalée
    comme anomalie, l'affichage sans vérification ou l'inverse.

### Rangement

17. **Une démonstration vit dans le fichier homonyme de celui de son énoncé.**
    Cette règle est tenue par convention, non par le chargeur : le domicile
    d'une démonstration se lit sans chercher, et un domaine se relit d'un
    seul tenant.

## Ce que la validation ne contrôle pas

- **Les arêtes `depend_de` entre énoncés ne sont pas soumises à la monotonie.**
  Un énoncé de licence 1 peut dépendre d'un énoncé de licence 3 sans que rien
  ne le signale. C'est délibéré tant que le champ ne distingue pas la
  dépendance pédagogique — ce qu'il faut avoir vu avant — de la dépendance
  épistémique — ce dont l'énoncé est un cas particulier. Un audit automatisé
  en recense 69 sur 3226 à la publication de la 3.0 : 57 sautent d'un niveau,
  9 de deux, 2 de trois, 1 de quatre — le foyer principal est la série des
  orbitales atomiques, posée en licence 1 sur des résultats de licence 3.
- **Les grandeurs sans dimension ne se distinguent pas entre elles.** Un pH et
  un albédo ont la même dimension : aucune vérification ne les sépare.
- **Le contenu scientifique n'est pas vérifié.** L'homogénéité attrape un
  facteur manquant ou un opérateur du mauvais ordre, jamais une erreur de
  nature vectorielle : `div(E)` et `rot(E)` ont la même dimension.
