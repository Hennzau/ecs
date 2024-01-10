use std::{
    collections::hash_map::DefaultHasher,
    hash::{
        Hash,
        Hasher,
    },
};

/// Macro derive proc to implement AnyComponent trait and function 'id' for the current struct
pub use ecs_macros::Component;

/// In ECS paradigms everything can be seen as a unique identifier
pub type ComponentID = u64;
pub type Group = u64;

/// General trait that must be implemented for structs that must be understand as Component
/// The user doesn't have to manipulate this trait, everything is handled by the ECS crate and the
/// proc macro [derive(Component)]
pub trait AnyComponent {
    fn id() -> ComponentID where Self: Sized;
}

/// Converts a list of ComponentIDs into the Group format by hashing the list of IDs.
pub fn components_to_group(components: &[ComponentID]) -> Group {
    let mut hasher = DefaultHasher::new();

    components.hash(&mut hasher);

    return hasher.finish();
}