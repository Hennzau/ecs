use std::collections::HashSet;
use crate::core::{
    component,
    component::{
        ComponentID,
        Group,
    },
};

/// In ECS paradigms everything can be seen as a unique identifier
pub type SystemID = u64;

/// General trait that must be implemented for structs that must be understand as System
pub trait System {
    /// This function provides a way to know which components each system wants to use
    fn components(&self) -> HashSet<ComponentID>;

    /// Each system belongs to a certain group. Every system that use the same set of components
    /// are in the same group
    fn group(&self) -> Group {
        component::components_to_group(&self.components())
    }
}