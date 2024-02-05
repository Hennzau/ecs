pub mod events {
    use crate::core::{
        event::{
            Event,
            EventID,
            AnyEvent,
        },
        component::{
            ComponentID,
            AnyComponent,
        },
        entity::Entity,
    };

    use ahash::RandomState;

    /// Event indicating the request to close the application.
    #[derive(Event)]
    pub struct CloseApplication {}

    /// Event indicating the request to close the application by a moderator.
    #[derive(Event)]
    pub struct ModeratorCloseApplication {
        moderator: Entity,
    }

    impl ModeratorCloseApplication {
        pub fn new(moderator: Entity) -> Self {
            return Self {
                moderator
            };
        }
    }

    /// Event indicating the request to remove a component from an entity
    #[derive(Event)]
    pub struct TryRemoveComponent {
        pub entity: Entity,
        pub component_id: ComponentID,
    }

    /// Event indicating the request to remove a component from a batch
    #[derive(Event)]
    pub struct TryRemoveComponentBatch {
        pub batch: (Entity, usize),
        pub component_id: ComponentID,
    }

    /// Event indicating the request to remove a component from a set
    #[derive(Event)]
    pub struct TryRemoveComponentSet {
        pub entities: Vec<Entity>,
        pub component_id: ComponentID,
    }

    /// Event indicating the request to add a component to an entity
    #[derive(Event)]
    pub struct TryAddComponent {
        pub entity: Entity,
        pub component: Box<dyn AnyComponent>,
    }

    /// Event indicating the request to add a component to a batch
    #[derive(Event)]
    pub struct TryAddComponentBatch {
        pub batch: (Entity, usize),
        pub component: Vec<Box<dyn AnyComponent>>,
    }

    /// Event indicating the request to add a component to a set
    #[derive(Event)]
    pub struct TryAddComponentSet {
        pub entities: Vec<Entity>,
        pub component: Vec<Box<dyn AnyComponent>>,
    }
}

pub mod components {
    use crate::core::component::{
        Component,
        ComponentID,
        AnyComponent,
    };

    use ahash::RandomState;

    /// Component representing a Moderator : it is a special component for managing application
    #[derive(Component)]
    pub struct Modperator {}

    /// Component representing a time duration.
    #[derive(Component)]
    pub struct Duration {
        pub time: f32,
    }

    impl Duration {
        pub fn new(time: f32) -> Self {
            return Self {
                time
            };
        }
    }

    /// Component representing a label with associated text.
    #[derive(Component)]
    pub struct Label {
        pub text: String,
    }

    impl Label {
        pub fn new(text: String) -> Self {
            return Self {
                text
            };
        }
    }
}

pub mod systems {
    use ahash::AHashSet;

    use crate::{
        application::basic::{
            events,
            components::{
                Duration
            },
        },
        core::{
            component::{
                ComponentID,
                AnyComponent,
            },
            entity::Entity,
            world::World,
            system::{
                System,
                SystemBuilder,
                CustomSystem,
            },
        },
    };
    use crate::prelude::basic::components::Modperator;

    /// This system identifies if a moderator with a time duration should close the application
    pub struct CloseApplication {
        pub time: f32,
    }

    impl CloseApplication {
        /// Creates a new instance of the CloseApplication system with default values.
        ///
        /// # Returns
        ///
        /// Returns a new instance of the CloseApplication system. This instance is wrapped
        /// with a Rc::<RefCell<dyn System>> == CustomSystem, using SystemBuilder::new function
        pub fn new() -> CustomSystem {
            return SystemBuilder::new(Self {
                time: 0.0
            });
        }
    }

    impl System for CloseApplication {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![
                Duration::component_id(),
                Modperator::component_id(),
            ].into_iter().collect();
        }
        /// Handles the system logic on each tick of the game loop. It identifies if there is only one moderator
        /// for the running application and if it owns a Duration component. In those cases, the application
        /// will be closed by sending an event (through world) if current time exceeds duration time of the moderator
        ///
        /// # Arguments
        ///
        /// * `delta_time` - The time elapsed since the last tick, represented as a floating-point value.
        /// * `entities` - An array slice (`&[Entity]`) representing the entities affected by the tick. **Must be a singleton**
        /// * `world` - A mutable reference to the `World` instance, allowing modifications within the system logic.
        fn on_tick(&mut self, delta_time: f32, entities: &[Entity], world: &mut World) {
            if entities.len() != 1 {
                log::warn!("Different entities are moderator : {:?}. You must create only one moderator at a time", entities);
                return;
            }

            self.time += delta_time;

            if let Some(moderator) = entities.first().cloned() {
                if let Some(component) = world.try_get_component::<Duration>(moderator) {
                    if self.time >= component.time {
                        world.send_event(Box::new(events::ModeratorCloseApplication::new(moderator)));
                    }
                }
            }
        }
    }
}


