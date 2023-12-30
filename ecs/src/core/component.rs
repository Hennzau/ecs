/// Macro derive proc to implement AnyComponent trait and function 'id' for the current struct
pub use ecs_macros::Component;

/// In ECS paradigms everything can be seen as a unique identifier
pub type ComponentID = u64;
pub type Group = u128;

/// General trait that must be implemented for structs that must be understand as Component
/// The user doesn't have to manipulate this trait, everything is handled by the ECS crate and the
/// proc macro [derive(Component)]
pub trait AnyComponent {
    fn id() -> ComponentID where Self: Sized;
}

/// This is a utility function that converts a list of ComponentIDs into the Group format
/// # This function assumes that the sum of unique ComponentIDs is unique
pub fn components_to_group(components: &[ComponentID]) -> Group {
    let mut result = 0 as Group;

    for &component in components {
        result += component as Group;
    }

    return result;
}