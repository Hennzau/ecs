
pub mod mapping;

/// This module manages all entities within an application based on a specific mapping.
/// A mapping refers to an efficient memory distribution that dictates how entities should be sorted
/// to facilitate access to entities possessing a known set of components (A, B, C, etc.)
/// without necessitating iteration or conditional statements.
///
/// This approach is founded on the concept of 'nested storages' introduced by other ECS systems,
/// notably Skypjack in his blog: https://skypjack.github.io/ for EnTT.
/// It involves smart swapping strategies to avoid fragmenting the main array.
pub mod entities;

/// This module contains the `Components` struct, which is used to store all components in the game.
/// It aims to be a simple and efficient way to store components : user can add, remove and get components easily
/// and efficiently.
pub mod components;