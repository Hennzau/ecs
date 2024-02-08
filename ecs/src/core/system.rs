use std::{
    cell::RefCell,
    rc::Rc,
};

use ahash::AHashSet;

use crate::core::{
    component,
    component::{
        ComponentID,
        Group,
    },
    entity::Entity,
    world::World,
    event::AnyEvent,
};
use crate::prelude::SystemType;

/// A SharedSystem is a system that is intended to be used for multiple application functions (on_join, on_tick etc...)
/// and that needs to communicate data from its functions.
pub type CustomSystem = Rc::<RefCell<dyn System>>;

/// This struct lets you build systems without the need to write `Rc::<RefCell<T>>`
pub struct SystemBuilder {}

impl SystemBuilder {
    /// This function provides a way to build a `Rc::<RefCell<T>>` directly from a T value. It can be used in constructors
    /// of your custom systems.
    ///
    /// # Arguments
    ///
    /// * `value` - The T value representing an instance of a System.
    ///
    /// # Returns
    ///
    /// Returns a shared pointer `Rc::<RefCell<T>>` from the T value
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// struct TestSystem {}
    /// impl System for TestSystem {
    ///     fn components(&self) -> AHashSet<ComponentID> {
    ///         return AHashSet::new();
    ///     }
    /// }
    ///
    /// impl TestSystem {
    ///     pub fn new () -> CustomSystem {
    ///         return SystemBuilder::create_system(Self {});
    ///     }
    /// }
    ///
    /// ```

    pub fn create_system<T: System>(value: T) -> Rc::<RefCell<T>> {
        return Rc::new(RefCell::new(value));
    }

    /// This function provides a way to easy implement `System::components` for you `CustomSystem`s.
    ///
    /// # Arguments
    ///
    /// * `components` - The list of components ID's
    ///
    /// # Returns
    ///
    /// Returns a set `ahash::AHashSet<ComponentID>` from the list
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct Position {}
    ///
    /// #[derive(Component)]
    /// pub struct Velocity {}
    ///
    /// struct TestSystem {}
    ///
    /// impl System for TestSystem {
    ///     fn components(&self) -> AHashSet<ComponentID> {
    ///         return SystemBuilder::track_components(&[
    ///             Position::component_id(),
    ///             Velocity::component_id()
    ///         ]);
    ///     }
    /// }
    ///
    /// impl TestSystem {
    ///     pub fn new () -> CustomSystem {
    ///         return SystemBuilder::create_system(Self {});
    ///     }
    /// }
    ///
    /// ```

    pub fn track_components(components: &[ComponentID]) -> AHashSet<ComponentID> {
        return components.into_iter().cloned().collect();
    }

    /// This function provides a way to easy mix types for your systems while adding them into `ApplicationBuilder`.
    ///
    /// # Arguments
    ///
    /// * `types` - The list of `SystemType`
    ///
    /// # Returns
    ///
    /// Returns a set `ahash::AHashSet<SystemType>` from the list
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct Position {}
    ///
    /// #[derive(Component)]
    /// pub struct Velocity {}
    ///
    /// struct TestSystem {}
    ///
    /// impl System for TestSystem {
    ///     fn components(&self) -> AHashSet<ComponentID> {
    ///         return SystemBuilder::track_components(&[
    ///             Position::component_id(),
    ///             Velocity::component_id()
    ///         ]);
    ///     }
    /// }
    ///
    /// impl TestSystem {
    ///     pub fn new () -> CustomSystem {
    ///         return SystemBuilder::create_system(Self {});
    ///     }
    /// }
    ///
    /// let builder = ApplicationBuilder::new();
    /// builder.add_system(TestSystem::new(), SystemBuilder::mix_types(&[SystemType::JOIN, SystemType::QUIT]));
    ///
    /// ```

    pub fn mix_types (types: &[SystemType]) -> AHashSet<SystemType> {
        return types.into_iter().cloned().collect();
    }
}

/// General trait that must be implemented for structs that must be understand as System
pub trait System {
    /// This function provides a way to know which components each system wants to use.
    ///
    /// # Returns
    ///
    /// Returns a hash set (`AHashSet`) of `ComponentID` instances representing the components that the system wants to use.
    ///
    /// # Example
    ///
    /// See [`crate::application::basic::systems::CloseApplication::components`]
    ///
    fn components(&self) -> AHashSet<ComponentID>;

    /// Each system belongs to a certain group. Every system that uses the same set of components
    /// are in the same group.
    ///
    /// # Returns
    ///
    /// Returns a `Group` instance representing the group to which the system belongs based on its set of components.
    fn group(&self) -> Group {
        component::group_id(&self.components())
    }

    /// Handles the system logic when an event is triggered.
    ///
    /// # Arguments
    ///
    /// * `_entities` - An array slice (`&[Entity]`) representing the entities affected by the event for this system.
    /// * `_world` - A mutable reference to the `World` instance, allowing modifications within the system logic.
    /// * `_event` - A boxed trait object (`Box<dyn AnyEvent>`) representing the triggered event.
    fn on_event(&mut self, _entities: &[Entity], _world: &mut World, _event: &Box<dyn AnyEvent>) {}

    /// Handles the system logic when entities are joined to the system.
    ///
    /// # Arguments
    ///
    /// * `_entities` - An array slice (`&[Entity]`) representing the entities that have been joined to the system.
    /// * `_world` - A mutable reference to the `World` instance, allowing modifications within the system logic.
    fn on_join(&mut self, _entities: &[Entity], _world: &mut World) {}

    /// Handles the system logic on each tick of the game loop.
    ///
    /// # Arguments
    ///
    /// * `_delta_time` - The time elapsed since the last tick, represented as a floating-point value.
    /// * `_entities` - An array slice (`&[Entity]`) representing the entities affected by the tick.
    /// * `_world` - A mutable reference to the `World` instance, allowing modifications within the system logic.
    ///
    /// # Example
    ///
    /// See [`crate::application::basic::systems::CloseApplication::on_tick`]
    fn on_tick(&mut self, _delta_time: f32, _entities: &[Entity], _world: &mut World) {}

    /// Handles the system logic when the application or game is about to quit.
    ///
    /// # Arguments
    ///
    /// * `_entities` - An array slice (`&[Entity]`) representing the entities affected by the quit event.
    /// * `_world` - A mutable reference to the `World` instance, allowing modifications within the system logic.
    fn on_quit(&mut self, _entities: &[Entity], _world: &mut World) {}
}