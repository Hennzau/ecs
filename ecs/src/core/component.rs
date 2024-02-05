use std::any::Any;

use ahash::{
    AHashSet,
    RandomState,
};

pub use ecs_macros::Component;

/// In ECS paradigms everything can be seen as a unique identifier
pub type ComponentID = u64;
pub type Group = u64;

/// General trait that must be implemented for structs that must be understand as Component
/// The user doesn't have to manipulate this trait, everything is handled by the ECS crate and the
/// proc macro [derive(Component)]
pub trait AnyComponent {
    fn id(&self) -> ComponentID;

    fn component_id() -> ComponentID where Self: Sized;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any(&self) -> &dyn Any;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

/// Converts a list of ComponentIDs into the Group format by hashing the sum of IDs.
///
/// # Arguments
///
/// * `components` - A hash set (`AHashSet`) of `ComponentID` instances to be converted into the `Group` format.
///
/// # Returns
///
/// Returns a `Group` id representing the hashed result of the provided list of `ComponentID` instances.
///
/// # Example
///
/// ```
/// use ecs::prelude::*;
///
/// let A = 13855858878564166539 as ComponentID;
/// let B = 6981191862617893938 as ComponentID;
///
/// let mut set = AHashSet::new();
/// set.insert(A);
/// set.insert(B);
///
/// let group = group_id(&set);
///
/// let hasher = RandomState::with_seed(0);
///
/// assert!(group == hasher.hash_one(&(A + B)));
/// ```
pub fn group_id(components: &AHashSet<ComponentID>) -> Group {
    let mut result = 0 as u128;

    for component in components {
        result += component.clone() as u128;
    }

    let hasher = RandomState::with_seed(0);

    return hasher.hash_one(&result);
}