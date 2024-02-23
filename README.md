# Description des structures de données majoritairement utilisées dans les ECS

L'objectif de ces structures de données est **toujours** de permettre un accès rapide à un ensemble d'entitées qui ont
une certaine combinaison de composants. Il est également
demandé à ce que l'accès aux composants de ces entités se fasse également de manière rapide.

On recherche donc toujours à compacter un maximum les données de façon à ce que les accès en mémoire soient les plus
rapides possibles en itérant simplement sur des espaces mémoire contiguës (cela permet entre autre de profiter au
maximum des caches).

Récapitulatif:

- On ajoute des composants à des entitées. Ces dernières sont reconnues par un identifiant unique qui est un entier.
- On veut pouvoir accéder rapidement à toutes les entitées qui ont une certaine combinaison de composants.
- On veut pouvoir accéder rapidement aux composants de ces entitées.

Le cas typique d'utilisation est le suivant:

  ```rust
    for (entity, position, velocity) in Query<Position, Velocity> () {
      println ! ("Entity {} has position {:?} and velocity {:?}", entity, position, velocity); 
      position.x += velocity.x * dt; 
      position.y += velocity.y * dt;
    }
  ```

## Archétypes

### Définition

Un archétype est une structure qui maintient un ensemble de données.

- La liste des composants auquel l'archétype est associé. Cette liste est fixée dès la création de l'archétype.
- Une liste des entitées qui ont **uniquement** ces composants. Cette liste est ordonnée.
- Une matrice où chaque colone est associée à un composant et à chaque ligne correspond l'instance du composant de l'
  entité de la même ligne de la liste ordonnée précédente.

Par exemple l'archétype [A, B] peut ressemblé à ceci:

| Entitées | A  | B  |
|----------|----|----|
| 1        | A1 | B1 |
| 0        | A0 | B0 |
| 3        | A3 | B3 |
| 5        | A5 | B5 |
| 4        | A4 | B4 |

Où A0, A1, A3, A4, A5 sont des instances du composant A et B0, B1, B3, B4, B5 sont des instances du composant B pour les
entitées 0, 1, 3, 4, 5 respectivement.

### Utilisation

Un ECS basé sur les Archétypes va maintenir une liste d'archétypes. Lorsque l'utilisateur ajoute un composant à une
entité, le système va chercher l'archétype qui correspond à la liste de composants de l'entité. Si l'archétype n'existe
pas, il est créé. L'entité est alors ajoutée à la liste des entitées de l'archétype et les instances des composants sont
ajoutées à la matrice.

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

| []       | [A]      |   | [A,B]    |   |   |
|----------|----------|---|----------|---|---|
| Entitées | Entitées | A | Entitées | A | B |
| 0        |          |   |          |   |   |

A l'étape (2), à l'entié 0 est ajouté le composant A. On déplace donc l'entité 0 de l'archétype [] à l'archétype [A] Et
on place l'instance A0 dans la la colonne A de l'archétype [A].

| []       | [A]      |    | [A,B]    |   |   |
|----------|----------|----|----------|---|---|
| Entitées | Entitées | A  | Entitées | A | B |
|          | 0        | A0 |          |   |   |

A l'étape (3), à l'entié 0 est ajouté le composant B. On déplace donc l'entité 0 de l'archétype [A] à
l'archétype [A, B], on déplace également l'instance A0 dans la colone A de l'archétype [A, B] Et on place l'instance B0
dans la la colonne B de l'archétype [A, B].

| []       | [A]      |   | [A,B]    |    |    |
|----------|----------|---|----------|----|----|
| Entitées | Entitées | A | Entitées | A  | B  |
|          |          |   | 0        | A0 | B0 |

A l'étape (4), l'entité 1 est créée sans composant.

| []       | [A]      |   | [A,B]    |    |    |
|----------|----------|---|----------|----|----|
| Entitées | Entitées | A | Entitées | A  | B  |
| 1        |          |   | 0        | A0 | B0 |

A l'étape (5), à l'entié 1 est ajouté le composant A et B. On déplace donc l'entité 1 de l'archétype [] à
l'archétype [A, B], et on place l'instance A1 dans la colone A de l'archétype [A, B] et l'instance B1 dans la la colonne
B de l'archétype [A, B].

| []       | [A]      |   | [A,B]    |    |    |
|----------|----------|---|----------|----|----|
| Entitées | Entitées | A | Entitées | A  | B  |
|          |          |   | 0        | A0 | B0 |
|          |          |   | 1        | A1 | B1 |

On stocke les différents archétypes dans un tableau ou dans un arbre pour pouvoir les retrouver rapidement.

### Avantages

Le gros avantage de cette structure est que les entitées et les composants se trouvent toujours localement au même
endroit et sont ordonnées de façon à ce que si l'on itère sur les entitées et sur les colones de la matrice de l'
archétype, on récupère
des données qui sont ordonnées de la même manière (à l'indice i de la liste des entitées correspond l'instance i de
chaque composant).

De plus les données sont bien ordonnées en mémoire, ce qui permet de profiter au maximum des caches.

Egalement, il est facile de savoir où une entité se trouvera après l'insertion d'un composant

### Inconvénients

Un inconvénient de cette structure est que l'on doit maintenir une map qui associe à chaque composant la liste des
archétypes qui le contiennent mais il est vrai que ce n'est pas forcément un problème.

Le gros inconvénient de cette méthode est que la mémoire est complètement fragmentée. Si l'on souhaite toutes les
entités qui ont le composant A, il faut regrouper toutes les entitées
des archétypes qui contiennent A, c'est à dire par exemple [A], [A, B], [A, C], [A, B, C], etc. Les données sont donc
éparpillées dans la mémoire, ce qui peut ralentir les accès.

## Non Owning Groups

Une autre structure de données est celle implémentée dans projet EnTT de Skypjack. Et c'est celle que j'ai déjà proposé
dans le projet.

### Définition

Un groupe est une structure qui maintient un ensemble de données.

- Une liste de *sous-groupes imbriquées* strictement. Par exemple [A] < [A, B] < [A, B, C].
- Une liste des entitées qui ont **entre autre** ces composants. Cette liste est ordonnée.
- Une liste de curseurs qui pointent, pour chaque *soous-groupe imbriqué*, vers la position la première entité du groupe
  qui ne possède plus les composants nécessaire à l'appartenance au *sous-groupe imbriqué*.~~

Les données en mémoire pour le groupe ([A], [A, B], [A, B, C]) pourrait ressembleur à ceci:

| Entitées | 2 | 4 | 1 | 7   | 8 | 9 | 3 | 10 | 12 | 11 | 13 | 5 | 6 | 14 |   |
|----------|---|---|---|-----|---|---|---|----|----|----|----|---|---|----|---|
| Curseurs |   |   |   | ABC |   |   |   |    | AB |    |    |   |   |    | A |

Ici donc:

- Les entitées, 2, 4 et 1 possèdent les composants A, B et C.
- Les entitées, 2, 4, 1, 7, 8, 9, 3, 10 possèdent les composants A et B.
- Les entitées, 2, 4, 1, 7, 8, 9, 3, 10, 12, 11, 13, 5, 6 et possèdent le composant A.

### Utilisation

Un ECS basé sur les groupes va maintenir une liste de groupes. Lorsque l'utilisateur ajoute un composant à une entité,
le système va chercher la liste des *sous-groupes imbriquées* auxquels l'entité appartient désormais grâce à l'ajout de
ce composant.
De ces sous-groupes imbriquées on va chercher les groupes correspondant et ajouter l'entité à la liste des entitées de
ces groupes. On va ensuite faire des déplacements intelligents dans ces listes afin de positionner l'entité à la bonne
place.

Si l'on considère les séquences suivantes :

```
(1) let e0 = spawn();

(2) e0.add(A);
(3) e0.add(B);
(4) e0.add(C);

(5) let e1 = spawn();

(6) e1.add(A);
(7) e1.add(C);
```

Les sous-groupes imbriquées concernés sont [A], [B], [C], [A, B], [A, C], [B, C] et [A, B, C]. Il est possible donc de
les répartir en trois groupes :

- Le groupe 1 : ([A], [A, B], [A, B, C])
- Le groupe 2 : ([B], [B, C])
- Le groupe 3 : ([C], [A, C])

On peut représenter les données de ces groupes pour chaque étape :

A l'étape (1), l'entité 0 est créée sans composant.

| Groupe 1 | Entitées |     |  |  |
|----------|----------|-----|--|--|
|          | Curseurs | A   |  |  |
|          | Curseurs | AB  |  |  |
|          | Curseurs | ABC |  |  |
| Groupe 2 | Entitées |     |  |  |
|          | Curseurs | B   |  |  |
|          | Curseurs | BC  |  |  |
| Groupe 3 | Entitées |     |  |  |
|          | Curseurs | C   |  |  |
|          | Curseurs | AC  |  |  |

A l'étape (2), à l'entié 0 est ajouté le composant A. Le seul *sous-groupe imbriqué* auquel l'entité appartient est
donc [A] du groupe 1. On ajoute donc l'entité 0 à la liste des entitées du groupe 1 et on déplace les curseurs de la
liste des entitées du groupe 1 pour qu'ils pointent sur la bonne entité.

| Groupe 1 | Entitées | 0   |   |  |
|----------|----------|-----|---|--|
|          | Curseurs |     | A |  |
|          | Curseurs | AB  |   |  |
|          | Curseurs | ABC |   |  |
| Groupe 2 | Entitées |     |   |  |
|          | Curseurs | B   |   |  |
|          | Curseurs | BC  |   |  |
| Groupe 3 | Entitées |     |   |  |
|          | Curseurs | C   |   |  |
|          | Curseurs | AC  |   |  |

A l'étape (3), à l'entié 0 est ajouté le composant B. Maintenant l'entité 0 est dans les *sous-groupes imbriquées* [A]
et [A, B] du groupe 1. Et également [B] du groupe 2. On ajoute donc l'entité 0 à la liste des entitées du groupe 2 et on
déplace les curseurs de la liste des entitées du groupe 1 et du groupe 2 pour qu'ils pointent sur la bonne entité.

| Groupe 1 | Entitées | 0   |    |  |
|----------|----------|-----|----|--|
|          | Curseurs |     | A  |  |
|          | Curseurs |     | AB |  |
|          | Curseurs | ABC |    |  |
| Groupe 2 | Entitées | 0   |    |  |
|          | Curseurs |     | B  |  |
|          | Curseurs | BC  |    |  |
| Groupe 3 | Entitées |     |    |  |
|          | Curseurs | C   |    |  |
|          | Curseurs | AC  |    |  |

A l'étape (4), à l'entié 0 est ajouté le composant C. Maintenant l'entité 0 est dans les *sous-groupes imbriquées* [A]
et [A, B] et [A, B, C] du groupe 1. Et également [B] et [B, C] du groupe 2. Et également [C] et [A, C] du groupe 3. On
ajoute donc l'entité 0 à la liste des entitées du groupe 3 et on déplace les curseurs de la liste des entitées du groupe
1, du groupe 2 et du groupe 3 pour qu'ils pointent sur la bonne entité.

| Groupe 1 | Entitées | 0 |     |  |
|----------|----------|---|-----|--|
|          | Curseurs |   | A   |  |
|          | Curseurs |   | AB  |  |
|          | Curseurs |   | ABC |  |
| Groupe 2 | Entitées | 0 |     |  |
|          | Curseurs |   | B   |  |
|          | Curseurs |   | BC  |  |
| Groupe 3 | Entitées | 0 |     |  |
|          | Curseurs |   | C   |  |
|          | Curseurs |   | AC  |  |

A l'étape (5), l'entité 1 est créée sans composant. Donc la structure des groupes ne change pas

A l'étape (6), à l'entié 1 est ajouté le composant A. Le seul *sous-groupe imbriqué* auquel l'entité appartient est
donc [A] du groupe 1. On ajoute donc l'entité 1 à la liste des entitées du groupe 1 et on déplace les curseurs de la
liste des entitées du groupe 1 pour qu'ils pointent sur la bonne entité.

| Groupe 1 | Entitées | 0 | 1   |   |
|----------|----------|---|-----|---|
|          | Curseurs |   |     | A |
|          | Curseurs |   | AB  |   |
|          | Curseurs |   | ABC |   |
| Groupe 2 | Entitées | 0 |     |   |
|          | Curseurs |   | B   |   |
|          | Curseurs |   | BC  |   |
| Groupe 3 | Entitées | 0 |     |   |
|          | Curseurs |   | C   |   |
|          | Curseurs |   | AC  |   |

A l'étape (7), à l'entié 1 est ajouté le composant C. Maintenant l'entité 1 est dans les *sous-groupes imbriquées* [A]
du groupe 1. Et également [C] et [A, C] du groupe 3. On ajoute donc l'entité 1 à la liste des entitées du groupe 3 et on
déplace les curseurs de la liste des entitées du groupe 1 et du groupe 3 pour qu'ils pointent sur la bonne entité.

| Groupe 1 | Entitées | 0 | 1   |    |
|----------|----------|---|-----|----|
|          | Curseurs |   |     | A  |
|          | Curseurs |   | AB  |    |
|          | Curseurs |   | ABC |    |
| Groupe 2 | Entitées | 0 |     |    |
|          | Curseurs |   | B   |    |
|          | Curseurs |   | BC  |    |
| Groupe 3 | Entitées | 0 | 1   |    |
|          | Curseurs |   |     | C  |
|          | Curseurs |   |     | AC |

### Avantages

L'avantage majeur de cette structure est que **une seule** itération suffit pour récupérer toutes les entitées qui ont
une certaine combinaison de composants. Il n'y a plus du tout de fragmentation de la mémoire et nos données sont
parfaitement ordonnées.

Du moins seulement pour les entitées

### Inconvénients

Plusieurs gros inconvénients sont à noter. Certes il n'y a plus de fragmentation de la mémoire mais les données peuvent
être dupliquées de nombreuse fois. Par exemple dans la séquence précédente, le système va stocker plusieurs fois les
entitées 0 et 1 car elles possèdent en même temps les caractéristiques de *sous-groupes imbriquées* présents dans deux
groupes différents.

Le fait que les données ne soient plus placée de manière unique dans un conteneur ne permet donc pas de placer les
instances des composants intelligemments et nous devons les placer ailleurs et récupérer ces composants au travers d'une
map qui, à chaque entité, associe l'endroit dans lequel son instance de composant est placée.

Cela réduit donc drastiquement le temps d'accès aux composants des entitées.

C'est le fait que cette structure ne **possède** pas les instances des composants qu'elle est nommée **Non Owning Groups**.

## Full/Partial/Non Owning Groups

### Complément de la structure de groupe

Il est possible d'utiliser **en même temps** que les structures de groupes, des **pools** de composants qui gèrent les instances des composants pour chaque entité, pour chaque type de composant.
A priori ces **pools** sont complètement indépendantes des structures internes des données des groupes.

Ainsi, **en plus** des groupes précédents, on ajoute :

- Une liste de *pools* de composants. Chaque pool est associée à un type de composant.
- Une map qui associe à chaque entité, pour chaque composant, son emplacement dans la pool de ce composant.

Ainsi un exemple de données type pour une application avec un groupe ([A], [A, B], [A, B, C]) (nommé *groupe 1*) pourrait ressembler à ceci:

| **Groupe 1** |    |    |    |     |    |    |    |    |    |     |     |     |     |     |   |
|--------------|----|----|----|-----|----|----|----|----|----|-----|-----|-----|-----|-----|---|
| Entitées     | 2  | 4  | 1  | 7   | 8  | 9  | 3  | 10 | 12 | 11  | 13  | 5   | 6   | 14  |   |
| Curseurs     |    |    |    | ABC |    |    |    |    | AB |     |     |     |     |     | A |

| **Pools**    |    |    |    |    |    |    |    |     |    |     |     |     |     |     |
|--------------|----|----|----|----|----|----|----|-----|----|-----|-----|-----|-----|-----|
| A            | A1 | A2 | A3 | A4 | A5 | A6 | A7 | A8  | A9 | A10 | A11 | A12 | A13 | A14 |
| B            | B1 | B2 | B3 | B4 | B7 | B8 | B9 | B10 |    |     |     |     |     |     |
| C            | C1 | C2 | C4 |    |    |    |    |     |    |     |     |     |     |     |

| **Map**       |   |   |   |   |   |   |   |   |   |    |    |    |    |    |
|---------------|---|---|---|---|---|---|---|---|---|----|----|----|----|----|
| Entitiées     | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
| A             | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9  | 10 | 11 | 12 | 13 |
| B             | 0 | 1 | 2 | 3 |   |   | 4 | 5 | 6 | 7  |    |    |    |    |
| C             | 0 | 1 |   | 2 |   |   |   |   |   |    |    |    |    |    |

### Définition

La nouvelle idée qui peut venir améliorer les points négatifs des Non Owning Groups, est de coupler chaque pool à un groupe en particulier.
On entend par couplage, que l'ordre des entitées dans le groupe est le même que l'ordre des instances des composants dans la pool.

Ainsi si l'on attache la pool de composant A au groupe 1, on peut être sûr que l'instance A2 de l'entité 2 se trouve à la même place que l'entité 2 dans le groupe 1, un exemple serait de disposer les données comme ceci:

| **Groupe 1** |    |    |    |     |    |    |    |     |     |     |     |    |    |     |   |
|--------------|----|----|----|-----|----|----|----|-----|-----|-----|-----|----|----|-----|---|
| Entitées     | 2  | 4  | 1  | 7   | 8  | 9  | 3  | 10  | 12  | 11  | 13  | 5  | 6  | 14  |   |
| Curseurs     |    |    |    | ABC |    |    |    |     | AB  |     |     |    |    |     | A |
| **Pool A**   | A2 | A4 | A1 | A7  | A8 | A9 | A3 | A10 | A12 | A11 | A13 | A5 | A6 | A14 |

| **Autres Pools** |    |    |    |    |    |    |    |     |    |     |     |     |     |     |
|------------------|----|----|----|----|----|----|----|-----|----|-----|-----|-----|-----|-----|
| B                | B1 | B2 | B3 | B4 | B7 | B8 | B9 | B10 |    |     |     |     |     |     |
| C                | C1 | C2 | C4 |    |    |    |    |     |    |     |     |     |     |     |

| **Map**       |   |   |   |   |    |    |   |   |   |    |    |    |    |    |
|---------------|---|---|---|---|----|----|---|---|---|----|----|----|----|----|
| Entitiées     | 1 | 2 | 3 | 4 | 5  | 6  | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
| A             | 2 | 0 | 6 | 1 | 11 | 12 | 3 | 4 | 5 | 7  | 9  | 8  | 10 | 13 |
| B             | 0 | 1 | 2 | 3 |    |    | 4 | 5 | 6 | 7  |    |    |    |    |
| C             | 0 | 1 |   | 2 |    |    |   |   |   |    |    |    |    |    |

### Utilisation

Désormais, lorsque l'utilisateur veut accéder à toutes les entitées qui ont le composant A, il peut simplement itérer sur sur les deux espaces contigues **Entitées** et **Pool A** du groupe 1.
Tout sera bien ordonné.

On peut égamement associé les pools B et C au groupe 1, et les coupler :

| **Groupe 1** |    |    |    |     |    |    |    |     |     |     |     |     |      |      |   |
|--------------|----|----|----|-----|----|----|----|-----|-----|-----|-----|-----|------|------|---|
| Entitées     | 2  | 4  | 1  | 7   | 8  | 9  | 3  | 10  | 12  | 11  | 13  | 5   | 6    | 14   |   |
| Curseurs     |    |    |    | ABC |    |    |    |     | AB  |     |     |     |      |      | A |
| **Pool A**   | A2 | A4 | A1 | A7  | A8 | A9 | A3 | A10 | A12 | A11 | A13 | A5  | A6   | A14  |   |
| **Pool B**   | B2 | B4 | B1 | B7  | B8 | B9 | B3 | B10 |     |     |     |     |      |      |   |
| **Pool C**   | C2 | C4 | C1 |     |    |    |    |     |     |     |     |     |      |      |   |

| **Map**       |   |   |   |   |    |    |   |   |   |    |    |    |    |    |
|---------------|---|---|---|---|----|----|---|---|---|----|----|----|----|----|
| Entitiées     | 1 | 2 | 3 | 4 | 5  | 6  | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
| A             | 2 | 0 | 6 | 1 | 11 | 12 | 3 | 4 | 5 | 7  | 9  | 8  | 10 | 13 |
| B             | 2 | 0 | 6 | 1 |    |    | 3 | 4 | 5 | 7  |    |    |    |    |
| C             | 2 | 0 |   | 1 |    |    |   |   |   |    |    |    |    |    |

Ainsi si l'utilisateur souhaite accéder aux entitées qui ont les composants [A, B], il peut simplement itérer jusqu'au curseur correspondant les espaces contiguës **Entitées**, **Pool A** et **Pool B** du groupe 1.

### Limites

Ceci semble parfait, on peut désormais ordonner notre mémoire en accord avec les groupes afin que ces derniers possèdent les ressources. 

Néanmoins l'exemple précédent est trop simple. Si l'on reprend un exemple où il y'a 3 groupes :

- Le groupe 1 : ([A], [A, B], [A, B, C])
- Le groupe 2 : ([B], [B, C])
- Le groupe 3 : ([C], [A, C])

Il n'est pas possible de coupler les trois pools avec les trois groupes en même temps. Il faut donc **choisir** les groupes avec lesquels on veut coupler les pools.

#### Exemple 1

Si l'on couple les trois pools avec le groupe 1 alors on aura les distinctions suivantes :

- L'accès aux entitées du **sous-groupe imbriqué** [A] sera en **Full Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [A, B] sera en **Full Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [A, B, C] sera en **Full Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [B] sera en **Non Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [B, C] sera en **Non Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [C] sera en **Non Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [A, C] sera en **Non Owning**.

Le terme **Non Owning** est utilisé ici pour signifier que l'accès aux entitées et aux **composants** se fait par accès à la MAP qui associe à chaque entité, pour chaque composant, son emplacement dans la pool de ce composant.
C'est à dire qu'en itérant sur les entitées, on doit aller chercher dans la MAP le composant associé à chaque entité.

Le terme **Full Owning** est utilisé ici pour signifier que l'accès aux entitées et aux **composants** se fait par itération sur les espaces contigues **Entitées** et **Pool** du groupe.
C'est à dire qu'en itérant sur les entitées, on peut simplement itérer en parallèle et *zip* les deux itérations pour obtenir le composant correspondant à chaque entité

#### Exemple 2

Si l'on couple les pools A et B au groupe 1 et la pool C au groupe 3 alors on aura les distinctions suivantes :

- L'accès aux entitées du **sous-groupe imbriqué** [A] sera en **Full Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [A, B] sera en **Full Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [A, B, C] sera en **Partial Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [B] sera en **Non Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [B, C] sera en **Non Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [C] sera en **Full Owning**.
- L'accès aux entitées du **sous-groupe imbriqué** [A, C] sera en **Partial Owning**.

Le terme **Partial Owning** est utilisé ici pour signifier que l'accès aux entitées et aux **composants** se fait en partie en itérant sur les espaces congitues **Entitées** et **Pool** du groupe et en partie par accès à la MAP qui associe à chaque entité, pour chaque composant, son emplacement dans la pool de ce composant.
C'est à dire qu'en itérant sur les entitées, on peut itérer simplement en parallèle pour un composant et aller chercher dans la MAP le composant associé à chaque entité pour l'autre type de composant.

### Avantages

Il est clair qu'associé les pools aux groupes permet de résoudre à la fois les problèmes de fragmentation de la mémoire de la structure par Archétypes et permet un accès rapide à des combinaisons de composants.

### Inconvénients

Le gros défaut de ce système est qu'il n'est pas possible que chaque **sous-groupe imbriqué** soit en **Full Owning**. Il faut donc choisir intelligemment selon les besoins de l'application.