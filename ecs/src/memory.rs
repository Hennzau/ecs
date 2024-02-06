/// This module manages memory mapping to generate the appropriate Entities storage
/// based on the user's chosen set of components.
///
/// This mapping principle was conceived by Genouville Grégoire, Bianchi Bérénice, and Le Van Enzo.
/// It revolves around creating a specialized bipartite graph and employing the Hopcroft-Karp algorithm
/// to create an optimized mapping for PackedEntities.
///
/// The idea is to construct a bipartite graph where each group appears both in the left and right groups.
/// Then, we connect each group on the left to every group on the right that contains it.
/// Finally, we use the Hopcroft-Karp algorithm to determine the minimal bipartite matching.
///
/// The Hopcroft-Karp algorithm, initially recursive, aims to be transformed into an iterative approach.
/// Referencing: <https://www.baeldung.com/cs/convert-recursion-to-iteration>

pub mod mapping;

/// This module manages all entities within an application based on a specific mapping.
/// A mapping refers to an efficient memory distribution that dictates how entities should be sorted
/// to facilitate access to entities possessing a known set of components (A, B, C, etc.)
/// without necessitating iteration or conditional statements.
///
/// This approach is founded on the concept of 'nested storages' introduced by other ECS systems,
/// notably Skypjack in his blog: <https://skypjack.github.io/> for EnTT.
/// It involves smart swapping strategies to avoid fragmenting the main array.

pub mod entities;

/// This module contains the `Components` struct, which is used to store all components in the game.
/// It aims to be a simple and efficient way to store components : user can add, remove and get components easily
/// and efficiently.

pub mod components;