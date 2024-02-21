use std::any::Any;

use ahash::{
    AHashSet,
    RandomState,
};

pub type ComponentID = u64;
pub type ArchetypeID = u64;

pub use macros::Component;

pub trait AnyComponent {
    fn type_id() -> ComponentID where Self: Sized;
    fn id(&self) -> ComponentID;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any(&self) -> &dyn Any;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

/// Converts a list of ComponentIDs into the Archetype format by hashing the sum of IDs.
///
/// # Arguments
///
/// * `components` - A set of `ComponentID`
///
/// # Returns
///
/// Returns a `ArchetypeID` id representing the hashed result of the provided list of `ComponentID` instances.
///
/// # Example
///
/// ```
/// use hnz::ecs::core::component::*;
/// use ahash::{
///     AHashSet,
///     RandomState
/// };
///
/// let A = 13855858878564166539 as ComponentID;
/// let B = 6981191862617893938 as ComponentID;
///
/// let mut set = AHashSet::new();
/// set.insert(A);
/// set.insert(B);
///
/// let archetype_id = as_archetype_id(&set);
///
/// let hasher = RandomState::with_seed(0);
///
/// assert!(archetype_id == hasher.hash_one(&(A + B)));
/// ```

pub fn as_archetype_id(components: &Vec<ComponentID>) -> ArchetypeID {
    let mut result = 0 as u128;

    for component in components {
        result += component.clone() as u128;
    }

    let hasher = RandomState::with_seed(0);

    return hasher.hash_one(&result);
}