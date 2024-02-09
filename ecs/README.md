# HNZ - ECS

The main purpose of this crate is to provide a simple and efficient way to implement an Entity Component System (ECS) in Rust.
Our ECS is designed to minimize the amout of storages required to maintain a set of entities, while providing a fast way to
access entities that possess a specific set of components.

You can see some examples of how to use the ECS in the examples in the directory [examples](https://github.com/Hennzau/hnz/blob/main/examples/). This directory contains the
following examples:

  - ``ecs_memory``: shows how internal memory is managed by the ECS.
  - ``ecs_systems``: shows how to use custom systems for your application.
  - ``ecs_events``: show how to combine custom systems and custom events in your application.

Each example can be run by executing the following command:

````cargo run --example 'ecs_memory'```` where ``'ecs_memory'`` can be replaced by any other example.

There is also an ``ecs_benchmark`` example.

## Memory

The [memory](https://github.com/Hennzau/hnz/blob/main/ecs/src/memory) section of this crate offers a method to map a entity storage for the purpose of maintaining
sorted elements.

The concept involves organizing entities into multiple vectors to monitor those that possess specific combinations of
components. To avoid the need to store each entity entity of a group in a separate vector, we use a virtual nested storage.
This means that we use a single vector to store entities of multiple groups, and we use a mapping system to indicate where
to search for the entities of a specific group.

One of these storage may look like this:

    |---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    | 2 | 4 | 1 | 7 | 8 | 9 | 3 | 2 | 9 | 11| 4 | 5 | 6 | 9 |       Entity IDs
    |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    | - | - | - |ABC| - | - | - | - | AB| - | - | - | - | - | A |   Groups cursors
    |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|

You've to understand this storage as : 
- The first vector is the vector of entities, it contains the IDs of the entities.
- The second vector is the vector of groups cursors. The cursors are placed in a way that the entities that possess the components of the group are
  located before the cursor, and the entities that don't possess the components are located after the cursor. This is a nested storage because every entity that has *ABC* also has *AB* and *A*.

The complete storage consists of multiple vectors, each one for different nested groups. An example with two sets of groups is shown below:

    |---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    | 2 | 4 | 1 | 7 | 8 | 9 | 3 | 2 | 9 | 11| 4 | 5 | 6 | 9 |
    |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    | - | - | - |ABC| - | - | - | - | AB| - | - | - | - | - | A |
    |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    | 2 | 3 | 9 | 4 | 12| 18| 20| 1 |
    |---|---|---|---|---|---|---|---|---|
    | - | - | - | BC| - | - | - | - | B |
    |---|---|---|---|---|---|---|---|---|

In this example, there is 5 groups that are sorted in 2 *nested* storages:
- The first storage contains the groups *ABC* that is included in *AB* that is included in *A*.
- The second storage contains the groups *BC* that is included in *B*.

As you can see, entities '1', '2', '3', '4' and '9' are duplicated (they are in both storages). The main purpose of our
memory system is to minimize the amount of *nested* storages required to maintain a set of entities in order to avoid
duplication of entities. This minimization is achieved by solving a maximum matching problem using the Hopcroft-Karp algorithm
in [mapping.rs](https://github.com/Hennzau/hnz/blob/main/ecs/src/memory/mapping.rs).

Our memory system is then designed to provide a fast way to access entities that possess a specific set of components. It contains
a mapping system that allows to access in which storage the groups are located and where the cursors are located. It then gives
a slice of the entities that are in the wanted group.

## Application

The [application](https://github.com/Hennzau/hnz/blob/main/ecs/src/application.rs) 