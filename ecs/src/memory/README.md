# The ECS crate for HNZ

## Memory

The **memory** section of this crate offers a method to map a entity storage for the purpose of maintaining 
sorted elements.

The concept involves organizing entities into multiple vectors to monitor those that possess specific combinations of
components.

### Example

You may need fast access to all entities that have the set of components **A, B, C** and **A, B,** and **A**. One way to
achieve this is by sorting entities within a 'virtual nested storage'

````rust
[0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 10, 22, 6, 26, 14, 30, 2, 34, 18, 38]
   /\                                /\                                 /\
   ||                                ||                                 ||
  ABC                                AB                                 A
````

Here, entities are identified by their IDs, ranging from 0 to 40. These entities are stored contiguously in a vector and
sorted in a manner where entities possessing **A, B, C** precede the ABC cursor, those with **A, B** are positioned
before the AB cursor, and so forth...

The actual storage appears as follows:

````rust
[
    [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 10, 22, 6, 26, 14, 30, 2, 34, 18, 38],
       /\                                /\                                 /\
       ||                                ||                                 ||
       ABC                               AB                                 A
    [0, 6, 12, 18, 24, 30, 36, 21, 3, 27, 15, 33, 9, 39],
                   /\                                /\
                   ||                                ||
                   AC                                C
    [0, 12, 24, 36, 16, 20, 8, 28, 32, 4],
        /\                            /\
        ||                            ||
        BC                            B
]
````

The concept is to uphold a mapping system that directs you to the storage where your entities are located, along with
indicating the cursor. In our code, the ``mapping.rs`` module generate a map that appears as follows:

````rust
{
    "id of ABC":    (0, 0), // storage 0, cursor number 0,
    "id of AB":     (0, 1), // storage 0, cursor number 1...
    "id of A":      (0, 2),
    "id of AC":     (1, 0),
    "id of C":      (1, 1),
    "id of BC":     (2, 0),
    "id of B":      (2, 1),
}
````

As indicated in [mapping.rs](https://github.com/Hennzau/hnz/blob/main/ecs/src/memory/mapping.rs), it uses Hopcroft-Karp 
algorithm to generate a minimum mapping.

The value of the cursor is next used to manipulate entities within the storage. For example, if we want to add an entity
we may need to swap it with the cursor, and then increment the cursor etc... This is done in 
[entities.rs](https://github.com/Hennzau/hnz/blob/main/ecs/src/memory/entities.rs)

### Optimization

The aim of our mapping system is to create an efficient mapping, which finds a way to generate a minimum number of storages.
For this example, with 3 different components, when we want fast access to each combination, the minimum amount of storage to create is 3, and our mapping system achieves this.

### How to use this mapping method

Now that you've created a mapping, you need to utilize it to store entities in the correct order. To achieve this, when
you add a component to an entity, you must check the groups to which the entity now belongs. For these groups, utilize
the map to access the storage that may require updating, and perform updates using a test-and-swap approach for each
group, beginning from the largest to the smallest. (ABC is smaller than A because ABC encompasses A)

This has been implemented in : [entities.rs](https://github.com/Hennzau/hnz/blob/main/ecs/src/memory/entities.rs)