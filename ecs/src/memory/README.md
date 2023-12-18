# hnz

## The ECS crate for HNZ

### Memory

The **memory** part of this crate provides a way to map a storage in order to keep elements
sorted.

The idea is to sort entities into many vectors to track which have a certain combination of components.

#### Example
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

