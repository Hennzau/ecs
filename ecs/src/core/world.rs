use std::collections::VecDeque;

use crate::{
    core::{
        component::{
            ComponentID,
            AnyComponent,
        },
        entity::Entity,
        event::AnyEvent,
    },
    memory::components::Components,
};

/// World represent an instance of an application that can be used by a system to access data
/// or to modify it.
pub struct World<'a> {
    pub components: &'a mut Components,
    pub events: VecDeque<Box<dyn AnyEvent>>,
}

impl World<'_> {
    /// Creates a new instance of the `World` struct with the specified components pool. A World instance should be used
    /// by an Application's instance and not directly by the user.
    ///
    /// # Arguments
    ///
    /// * `components` - A mutable reference to the `Components` instance for managing components.
    ///
    /// # Returns
    ///
    /// Returns a new `World` instance with the provided components pool.
    pub fn new(components: &mut Components) -> World<'_> {
        return World {
            components,
            events: VecDeque::new(),
        };
    }

    /// Returns a reference to the component of the given entity if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to retrieve the component.
    /// * `id` - The identifier of the component to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns `Some(&Box<dyn AnyComponent>)` with a reference to the component if it exists.
    /// Returns `None` if the entity does not have the specified component.
    ///
    /// # Examples
    /// ```
    /// use ecs::prelude::*;
    ///
    /// fn some_method_in_a_system (entities: &[Entity], world: &mut World) {
    ///     if let Some (entity) = entities.first() {
    ///         // Let's imagine that '1928402901938219' is the id of the component 'NameString'
    ///         if let Some (any_component) = world.try_get_any_component(entity.clone(), 1928402901938219) {
    ///             // Now any_component is a &Box<dyn AnyComponent>
    ///         }
    ///     }
    /// }
    /// ```
    pub fn try_get_any_component(&self, entity: Entity, id: ComponentID) -> Option<&Box<dyn AnyComponent>> {
        return self.components.try_get_any_component(entity, id);
    }

    /// Returns a mutable reference to the component of the given entity if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to retrieve the mutable reference to the component.
    /// * `id` - The identifier of the component to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut Box<dyn AnyComponent>)` with a mutable reference to the component if it exists.
    /// Returns `None` if the entity does not have the specified component.
    ///
    /// # Examples
    /// ```
    /// use ecs::prelude::*;
    ///
    /// fn some_method_in_a_system (entities: &[Entity], world: &mut World) {
    ///     if let Some (entity) = entities.first() {
    ///         // Let's imagine that '1928402901938219' is the id of the component 'NameString'
    ///         if let Some (any_component) = world.try_get_any_mut_component(entity.clone(), 1928402901938219) {
    ///             // Now any_component is a &mut Box<dyn AnyComponent>
    ///         }
    ///     }
    /// }
    /// ```
    pub fn try_get_any_mut_component(&mut self, entity: Entity, id: ComponentID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.components.try_get_any_mut_component(entity, id);
    }

    /// Returns a reference to the component of the given entity if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to retrieve the component.
    ///
    /// # Returns
    ///
    /// Returns `Some(&T)` with a reference to the component of type `T` if it exists.
    /// Returns `None` if the entity does not have the specified component.
    ///
    /// # Examples
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct NameString {
    ///     pub name: String,
    /// }
    ///
    /// fn some_method_in_a_system (entities: &[Entity], world: &mut World) {
    ///     if let Some (entity) = entities.first() {
    ///         if let Some (component) = world.try_get_component::<NameString>(entity.clone()) {
    ///             // Now component is a &NameString
    ///         }
    ///     }
    /// }
    /// ```
    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: Entity) -> Option<&T> {
        return self.components.try_get_component::<T>(entity);
    }

    /// Returns a mutable reference to the component of the given entity if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to retrieve the mutable reference to the component.
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut T)` with a mutable reference to the component of type `T` if it exists.
    /// Returns `None` if the entity does not have the specified component.
    ///
    /// # Examples
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct NameString {
    ///     pub name: String,
    /// }
    ///
    /// fn some_method_in_a_system (entities: &[Entity], world: &mut World) {
    ///     if let Some (entity) = entities.first() {
    ///         if let Some (component) = world.try_get_mut_component::<NameString>(entity.clone()) {
    ///             // Now component is a &mut NameString
    ///         }
    ///     }
    /// }
    /// ```
    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        return self.components.try_get_mut_component::<T>(entity);
    }

    /// Sends an event to the application for processing.
    ///
    /// # Arguments
    ///
    /// * `event` - A boxed trait object (`Box<dyn AnyEvent>`) representing the event to be sent to the application.
    ///
    /// # Examples
    /// ```
    /// use ecs::prelude::*;
    ///
    /// fn some_method_in_a_system (entities: &[Entity], world: &mut World) {
    ///     if let Some (entity) = entities.first() {
    ///         world.send_event(Box::new (basic::events::ModeratorCloseApplication::new(entity.clone())));
    ///     }
    /// }
    /// ```
    pub fn send_event(&mut self, event: Box<dyn AnyEvent>) {
        self.events.push_back(event);
    }
}