# The ECS crate for HNZ

## Memory

The **memory** part of this crate provides a way to map a storage in order to keep elements
sorted.

The idea is to sort entities into many vectors to track which have a certain combination of components.

### Example
You may need a fast access to all entities that have the set of components **A,B,C** and **A,B** and **A**. One way to do this is to 
sort entities in a "virtual nested storage".

````rust
[0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 10, 22, 6, 26, 14, 30, 2, 34, 18, 38]
   /\                                /\                                 /\
   ||                                ||                                 ||
  ABC                                AB                                 A
````

Here, entities are represented by their ID (from 0 to 40). Entities are stored in a vector contiguously and sorted in a way that
every entities that have **A,B,C** are before the ABC cursor, every entities that have **A,B** are stored before the AB cursor etc...

The real storage looks like that:
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

The idea is to maintain a map that tells you the storage you have to look inside for your entities, and the cursor. In our code, 
the `graph.rs` module and `mapping.rs` module generate a map that looks like that :

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

The value of the cursor is accessible by the `mapping.value ()` function.

### Optimization

The aim of our mapping system is to create an efficient mapping, which finds a way to generate a minimum number of storages.
For this example, with 3 different components, when we want fast access to each combination, the minimum amount of storage to create is 3, and our mapping system achieves this.

### How to use this mapping method

Now you've create a mapping, you need to use it to store entities in the right order. To do that, when you add a component to an entity,
you need to check in which groups the entity belongs now. For thos groups, use the map to get the storage that may need to be updated,
and update it by test-and-swap for each group from the largest, to the smallest. (ABC is smallest than A because ABC contains A).

This has been implemented in : [storage.rs](..%2Fapplication%2Fstorage.rs)