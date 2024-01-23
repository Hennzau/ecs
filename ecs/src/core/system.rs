use std::collections::HashSet;

use crate::core::{
    component,
    component::{
        ComponentID,
        Group,
    },
    entity::Entity,
    world::World,
    event::AnyEvent
};

/// General trait that must be implemented for structs that must be understand as System
pub trait System {
    /// This function provides a way to know which components each system wants to use

    fn components(&self) -> HashSet<ComponentID>;

    /// Each system belongs to a certain group. Every system that use the same set of components
    /// are in the same group

    fn group(&self) -> Group {
        component::components_to_group(&self.components())
    }

    fn on_signal(&mut self, _entities: &[Entity], _world: &mut World) {}

    fn on_event(&mut self, _entities: &[Entity], _world: &mut World, _event: &Box<dyn AnyEvent>) {}

    fn on_join(&mut self, _entities: &[Entity], _world: &mut World) {}

    fn on_tick(&mut self, _entities: &[Entity], _world: &mut World) {}

    fn on_quit(&mut self, _entities: &[Entity], _world: &mut World) {}
}