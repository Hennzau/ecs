# hnz
HNZ is the toolkit i'm currently developing

Problème de tri : 

Contexte : dans le paradigme ECS (Entity Component System), on définit des structures de données (Component), qu'on attribue à différentes entitiés. La manière dont une entité ayant une certaine combinaison de composants est traité dans un système.

Ainsi, chaque entité contient une liste de composants, qu'on représente comme une liste d'identifiant (hash, donc d'entiers) de la forme. On s'intéresse ici à un algorithme qui permet de stocker les combinaisons de composants présents de manière à pouvoir itérer rapidement sur toutes les entités possédant telle combinaison de composants.

Le problème : 

On doit trier un ensemble de parties d'un ensemble d'entier. Par exemple si nos composants ont les identifiants **[0, 1, 2, 3]**, on peut s'amuser à vouloir trier l'ensemble des combinaisons :  

```[[0], [1], [0, 1], [2], [0, 2], [1, 2], [0, 1, 2], [3], [0, 3], [1, 3], [0, 1, 3], [2, 3], [0, 2, 3], [1, 2, 3], [0, 1, 2, 3]]``` (c'est en fait l'ensemble des parties de [0, 1, 2, 3] sans l'ensemble vide ici. Il s'agit du **cas maximal**, ou il y'a un maximum de combinaisons différentes à stocker)

Le but est d'obtenir à partir de cet ensemble de partie, une nouvelle liste qui contient des listes de combinaisons. Par exemple le cas le plus optimisé pour stocker les données de la manière que je veux est :

```
[
[[3], [2, 3], [1, 2, 3], [0, 1, 2, 3]],

[[2], [1, 2], [0, 1, 2]],

[[1], [1, 3], [0, 1, 3]],

[[0], [0, 3], [0, 2, 3]],

[[0, 2]],

[[0, 1]],

]
```

La règle est très simple, chaque élément de la liste de retour (donc chaque ligne que j'ai représenté ici) contient une liste de combinaison qui est telle que, de gauche vers la droite, chaque liste est incluse dans la suivante. par exemple le deuxième élément de la liste de retour (la deuxième ligne) est bien une liste de combinaisons qui vérifie [2] inclus dans [1, 2] inclus dans [0, 1, 2] (l'ordre des éléments dans les combinaisons n'importe pas).

**L'objectif** est de déterminer un algorithme qui accompli ce tri avec le moins d'éléments dans la liste de retour (donc le moins de ligne possible). Par exemple ce retour respecte la règle, mais contient 1 ligne de trop par rapport à une solution optimale : 

```
[
[[3], [0, 3], [0, 2, 3], [0, 1, 2, 3]],

[[1], [0, 1], [0, 1, 3]],

[[2], [0, 2], [0, 1, 2]],

[[0]],

[[1, 2], [1, 2, 3]],

[[1, 3]],

[[2, 3]]
]
```

Voilà, je rappelle qu'ici j'ai donné les exemples avec l'ensemble des parties de [0, 1, 2, 3] mais ça peut tout à fait (et dans la majorité des cas) n'être qu'une liste de parties de [0, 1, 2, 3]
