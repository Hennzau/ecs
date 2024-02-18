use ahash::{
    AHashMap,
    AHashSet,
};

use crate::{
    application::{
        Application,
    },
    memory::{
        mapping::MemoryMappingDescriptor,
    },
    core::{
        event::EventID,
        system::{
            CustomSystem,
            SystemType
        },
        component::Group,
    },
};

/// Builder for constructing an application with specific configurations.
pub struct ApplicationBuilder {
    event_systems: AHashMap<EventID, Vec<CustomSystem>>,

    join_systems: AHashMap<Group, Vec<CustomSystem>>,
    quit_systems: AHashMap<Group, Vec<CustomSystem>>,
    tick_systems: Vec<CustomSystem>,

    descriptor: MemoryMappingDescriptor,
    seen: AHashSet<Group>,
}

impl ApplicationBuilder {
    /// Creates a new instance of the ApplicationBuilder.
    ///
    /// # Returns
    ///
    /// Returns a new ApplicationBuilder with default configurations.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    /// // Create a new instance of the ApplicationBuilder.
    /// let app_builder = ApplicationBuilder::new();
    ///
    /// // Use the created ApplicationBuilder for building an application with specific configurations.
    /// ```

    pub fn new() -> Self {
        return Self {
            event_systems: AHashMap::new(),

            join_systems: AHashMap::new(),
            quit_systems: AHashMap::new(),
            tick_systems: Vec::new(),

            descriptor: MemoryMappingDescriptor::new(),
            seen: AHashSet::new(),
        };
    }

    /// Builds the application using the specified configurations.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the Application based on the provided configurations.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let app_builder = ApplicationBuilder::new();
    ///
    /// // Build the application using the specified configurations.
    /// let application = app_builder.build();
    ///
    /// // Add entities and components to your application
    ///
    /// let entity = application.spawn ();
    ///
    /// // Start and run the created application.
    /// application.run(60f32);
    /// ```

    pub fn build(self) -> Application {
        return Application::new(
            self.descriptor,
            self.event_systems,
            self.join_systems,
            self.quit_systems,
            self.tick_systems,
        );
    }

    /// Adds a custom system to the application with specified system types.
    ///
    /// # Arguments
    ///
    /// * `system` - The custom system to be added to the application.
    /// * `types` - A hash set (`AHashSet`) of system types indicating when the system should be executed.
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
    ///
    ///     fn types(&self) -> AHashSet<ComponentID> {
    ///         return SystemBuilder::executed_on(&[SystemType::TICK]);
    ///     }
    /// }
    ///
    /// impl TestSystem {
    ///     pub fn new () -> CustomSystem {
    ///         return SystemBuilder::create_system(Self {});
    ///     }
    /// }
    ///
    /// let app_builder = ApplicationBuilder::new();
    ///
    /// // Add a custom system to the application with specified system types.
    /// app_builder.add_system(TestSystem::new());
    ///
    /// // Continue configuring the application or build it using other methods.
    /// ```

    pub fn add_system(&mut self, system: CustomSystem) {
        for system_type in system.borrow ().types().clone() {
            match system_type {
                SystemType::JOIN => self.add_join_system(system.clone()),
                SystemType::QUIT => self.add_quit_system(system.clone()),
                SystemType::TICK => self.add_tick_system(system.clone()),
                SystemType::EVENT(id) => self.add_event_system(id, system.clone())
            }
        }
    }

    /// Adds multiple custom systems to the application with specified system types.
    ///
    /// # Arguments
    ///
    /// * `systems` - A vector (`Vec`) of custom systems to be added to the application.
    /// * `types` - A hash set (`AHashSet`) of system types indicating when the systems should be executed.
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
    ///
    ///     fn types(&self) -> AHashSet<ComponentID> {
    ///         return SystemBuilder::executed_on(&[SystemType::TICK]);
    ///     }
    /// }
    ///
    /// let app_builder = ApplicationBuilder::new();
    ///
    /// let custom_system = TestSystem::new();
    ///
    /// // Add a custom system to the application with specified system types.
    /// app_builder.add_system(vec![custom_system]);
    ///
    /// // Continue configuring the application or build it using other methods.
    /// ```

    pub fn add_systems(&mut self, systems: Vec<CustomSystem>) {
        for system in systems {
            self.add_system(system);
        }
    }

    /// Adds an event system to the application with the specified event and system.
    ///
    /// # Arguments
    ///
    /// * `event` - The event ID associated with the event system.
    /// * `system` - The custom system to be added to the application for handling the specified event.

    fn add_event_system(&mut self, event: EventID, system: CustomSystem) {
        if !self.event_systems.contains_key(&event) {
            self.event_systems.insert(event, Vec::new());
        }

        if !self.seen.contains(&system.borrow().group()) {
            self.descriptor.push(system.borrow().components());
            self.seen.insert(system.borrow().group());
        }

        self.event_systems.get_mut(&event).unwrap().push(system);
    }

    /// Adds a join system to the application with the specified custom system.
    ///
    /// # Arguments
    ///
    /// * `system` - The custom system to be added to the application for handling entity join events.

    fn add_join_system(&mut self, system: CustomSystem) {
        let group = system.borrow().group();

        if !self.join_systems.contains_key(&group) {
            self.join_systems.insert(group, Vec::new());
        }

        self.join_systems.get_mut(&group).unwrap().push(system);
    }

    /// Adds a quit system to the application with the specified custom system.
    ///
    /// # Arguments
    ///
    /// * `system` - The custom system to be added to the application for handling entity quit events.

    fn add_quit_system(&mut self, system: CustomSystem) {
        let group = system.borrow().group();

        if !self.quit_systems.contains_key(&group) {
            self.quit_systems.insert(group, Vec::new());
        }

        self.quit_systems.get_mut(&group).unwrap().push(system);
    }

    /// Adds a tick system to the application with the specified custom system.
    ///
    /// # Arguments
    ///
    /// * `system` - The custom system to be added to the application for handling tick events.

    fn add_tick_system(&mut self, system: CustomSystem) {
        if !self.seen.contains(&system.borrow().group()) {
            self.descriptor.push(system.borrow().components());
            self.seen.insert(system.borrow().group());
        }

        self.tick_systems.push(system);
    }
}