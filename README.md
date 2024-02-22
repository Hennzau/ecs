# Description des structures de données

L'objectif de ces structures de données est **toujours** de permettre un accès rapide à un ensemble d'entitées qui ont une certaine combinaison de composants. Il est également
demandé à ce que l'accès aux composants de ces entités se fasse également de manière rapide.

On recherche donc toujours à compacter un maximum les données de façon à ce que les accès en mémoire soient les plus rapides possibles en itérant simplement sur des espaces mémoire contiguës (cela permet entre autre de profiter au maximum des caches).

Récapitulatif:

- On ajoute des composants à des entitées. Ces dernières sont reconnues par un identifiant unique qui est un entier.
- On veut pouvoir accéder rapidement à toutes les entitées qui ont une certaine combinaison de composants.
- On veut pouvoir accéder rapidement aux composants de ces entitées.

Le cas typique d'utilisation est le suivant:

```rust

for (entity, position, velocity) in Query<Position, Velocity> () {
    println!("Entity {} has position {:?} and velocity {:?}", entity, position, velocity);
    position.x += velocity.x * dt;
    position.y += velocity.y * dt;
}
```

## Archétypes

### Définition

Un archétype est une structure qui maintient un ensemble de données.

- La liste des composants auquel l'archétype est associé. Cette liste est fixée dès la création de l'archétype.
- Une liste des entitées qui ont **uniquement** ces composants. Cette liste est ordonnée.
- Une matrice où chaque colone est associée à un composant et à chaque ligne correspond l'instance du composant de l'entité de la même ligne de la liste ordonnée précédente.

Par exemple l'archétype [A, B] peut ressemblé à ceci:

| Entitées | A | B |
|----------|---|---|
| 1        | A1| B1|
| 0        | A0| B0|
| 3        | A3| B3|
| 5        | A5| B5|
| 4        | A4| B4|

Où A0, A1, A3, A4, A5 sont des instances du composant A et B0, B1, B3, B4, B5 sont des instances du composant B pour les entitées 0, 1, 3, 4, 5 respectivement.

### Utilisation

Un ECS basé sur les Archétypes va maintenir une liste d'archétypes. Lorsqu'une entité est créée, le système va chercher l'archétype qui correspond à la liste de composants de l'entité. Si l'archétype n'existe pas, il est créé. L'entité est alors ajoutée à la liste des entitées de l'archétype et les instances des composants sont ajoutées à la matrice.

Si l'on considère les séquences suivantes :

```
(1) let e0 = spawn();

(2) e0.add(A);
(3) e0.add(B);

(4) let e1 = spawn();

(5) e1.add(A,B);
```

Les archétypes concernés sont [], [A] et [A, B]. On peut représenter les données de ces archétypes pour chaque étape :

A l'étape (1), l'entité 0 est créée sans composant.

| []       | [A]      |   | [A,B]     |   |   |
|----------|----------|---|-----------|---|---|
| Entitées | Entitées | A | Entitées  | A | B |
| 0        |          |   |           |   |   |

A l'étape (2), à l'entié 0 est ajouté le composant A. On déplace donc l'entité 0 de l'archétype [] à l'archétype [A] Et on place l'instance A0 dans la la colonne A de l'archétype [A].

| []       | [A]      |    | [A,B]    |   |   |
|----------|----------|----|----------|---|---|
| Entitées | Entitées | A  | Entitées | A | B |
|          | 0        | A0 |          |   |   |

A l'étape (3), à l'entié 0 est ajouté le composant B. On déplace donc l'entité 0 de l'archétype [A] à l'archétype [A, B], on déplace également l'instance A0 dans la colone A de l'archétype [A, B] Et on place l'instance B0 dans la la colonne B de l'archétype [A, B].

| []       | [A]      |   | [A,B]    |    |    |
|----------|----------|---|----------|----|----|
| Entitées | Entitées | A | Entitées | A  | B  |
|          |          |   | 0        | A0 | B0 |

A l'étape (4), l'entité 1 est créée sans composant.

| []       | [A]      |   | [A,B]    |    |    |
|----------|----------|---|----------|----|----|
| Entitées | Entitées | A | Entitées | A  | B  |
| 1        |          |   | 0        | A0 | B0 |

A l'étape (5), à l'entié 1 est ajouté le composant A et B. On déplace donc l'entité 1 de l'archétype [] à l'archétype [A, B], et on place l'instance A1 dans la colone A de l'archétype [A, B] et l'instance B1 dans la la colonne B de l'archétype [A, B].


| []       | [A]      |   | [A,B]    |    |    |
|----------|----------|---|----------|----|----|
| Entitées | Entitées | A | Entitées | A  | B  |
|          |          |   | 0        | A0 | B0 |
|          |          |   | 1        | A1 | B1 |

On stocke les différents archétypes dans un tableau ou dans un arbre pour pouvoir les retrouver rapidement.

### Avantages

Le gros avantage de cette structure est que les entitées et les composants se trouvent toujours localement au même endroit et sont ordonnées de façon à ce que si l'on itère sur les entitées et sur les colones de la matrice de l'archétype, on récupère
des données qui sont ordonnées de la même manière (à l'indice i de la liste des entitées correspond l'instance i de chaque composant). 

De plus les données sont bien ordonnées en mémoire, ce qui permet de profiter au maximum des caches.

Egalement, il est facile de savoir où une entité se trouvera après l'insertion d'un composant

### Inconvénients

Un inconvénient de cette structure est que l'on doit maintenir une map qui associe à chaque composant la liste des archétypes qui le contiennent mais il est vrai que ce n'est pas forcément un problème.

Le gros inconvénient de cette méthode est que la mémoire est complètement fragmentée. Si l'on souhaite toutes les entités qui ont le composant A, il faut regrouper toutes les entitées
des archétypes qui contiennent A, c'est à dire par exemple [A], [A, B], [A, C], [A, B, C], etc. Les données sont donc éparpillées dans la mémoire, ce qui peut ralentir les accès.

## Non Owning Groups

Une autre structure de données est celle implémentée dans projet EnTT de Skypjack. Et c'est celle que j'ai déjà proposé dans le projet.

### Définition

Un groupe est une structure qui maintient un ensemble de données.

- Une liste de *sous-groupes imbriquées* strictement. Par exemple [A] < [A, B] < [A, B, C].
- Une liste des entitées qui ont **entre autre** ces composants. Cette liste est ordonnée.
- Une liste de curseurs qui pointent, pour chaque *soous-groupe imbriqué*, vers la position la dernière entité du groupe qui possède les composants du *sous-groupe imbriqué*.


