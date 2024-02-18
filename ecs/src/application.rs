use std::{
    collections::VecDeque,
    time,
};

use ahash::{
    AHashSet,
    AHashMap,
};

use crate::{
    memory::{
        entities::Entities,
        mapping::{
            MemoryMapping,
            MemoryMappingDescriptor,
        },
        components::Components,
    },
    core::{
        component::{
            Group,
            ComponentID,
            AnyComponent,
        },
        entity::Entity,
        event::{
            EventID,
            AnyEvent,
        },
        system::CustomSystem,
        world::World,
    },
};

pub mod builder;
pub mod basic;
pub mod bundle;

/// Represents the core application structure managing entities, components, and systems.
pub struct Application {
    /// Memory mapping for entities storage optimization.
    mapping: MemoryMapping,

    /// Entities storage managing entity-related data.
    entities: Entities,

    /// Components pool for storing and managing components.
    components: Components,

    /// Next available entity ID for entity creation.
    next_entity: Entity,

    /// Tracks components associated with each entity.
    components_tracker: AHashMap<Entity, AHashSet<ComponentID>>,

    /// Queue for storing events to be processed.
    events: VecDeque<Box<dyn AnyEvent>>,

    /// Event systems organized by EventID for event handling.
    event_systems: AHashMap<EventID, Vec<CustomSystem>>,

    /// Join systems organized by Group for entity join event handling.
    join_systems: AHashMap<Group, Vec<CustomSystem>>,

    /// Quit systems organized by Group for entity quit event handling.
    quit_systems: AHashMap<Group, Vec<CustomSystem>>,

    /// Tick systems for handling periodic events.
    tick_systems: Vec<CustomSystem>,
}

impl Application {
    /// Creates a new instance of the Application with the specified configurations.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor for memory mapping optimization.
    /// * `event_systems` - Event systems organized by EventID for event handling.
    /// * `join_systems` - Join systems organized by Group for entity join event handling.
    /// * `quit_systems` - Quit systems organized by Group for entity quit event handling.
    /// * `tick_systems` - Tick systems for handling periodic events.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the Application with the provided configurations.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::memory::mapping::MemoryMappingDescriptor;
    ///
    /// use ecs::prelude::*;
    ///
    /// let descriptor = MemoryMappingDescriptor::new();
    /// let event_systems = AHashMap::new();
    /// let join_systems = AHashMap::new();
    /// let quit_systems = AHashMap::new();
    /// let tick_systems = Vec::new();
    ///
    /// // Create a new instance of the Application with the specified configurations.
    /// let application = Application::new(descriptor, event_systems, join_systems, quit_systems, tick_systems);
    /// ```
    ///
    /// # Note
    ///
    /// User should use [`crate::application::builder::ApplicationBuilder`] to create new instance of an Application

    pub fn new(descriptor: MemoryMappingDescriptor,
               event_systems: AHashMap<EventID, Vec<CustomSystem>>,
               join_systems: AHashMap<Group, Vec<CustomSystem>>,
               quit_systems: AHashMap<Group, Vec<CustomSystem>>,
               tick_systems: Vec<CustomSystem>) -> Self {
        let mapping = MemoryMapping::new(descriptor);

        return Self {
            components: Components::new(),
            entities: mapping.create_storage(),
            mapping: mapping,

            next_entity: 0 as Entity,
            components_tracker: AHashMap::new(),

            events: VecDeque::new(),

            event_systems: event_systems,

            join_systems: join_systems,
            tick_systems: tick_systems,
            quit_systems: quit_systems,
        };
    }

    /// Spawns a new entity and returns its ID.
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly spawned entity.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build(); // Empty application with no systems
    ///
    /// let new_entity_id = application.spawn();
    ///
    /// // Use the newly spawned entity ID for further operations.
    /// ```

    pub fn spawn(&mut self) -> Entity {
        let result = self.next_entity;

        self.components_tracker.insert(self.next_entity as Entity, AHashSet::new());
        self.next_entity += 1;

        return result;
    }

    /// Spawns a batch of entities and returns their IDs.
    ///
    /// # Arguments
    ///
    /// * `amount` - The number of entities to spawn in the batch.
    ///
    /// # Returns
    ///
    /// Returns a tuple containing the ID of the first entity in the batch and the total number of entities spawned.
    ///
    /// # Note
    ///
    /// Use this method when you need to spawn a lot of similar entities with the same components. If entities have different components,
    /// consider using the `spawn` method for individual entity spawning or 'spawn_set' for multiple entity spawning.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build(); // Empty application
    ///
    /// // Spawn a batch of entities and get the ID of the first entity and the total number of entities spawned.
    /// let (first_entity_id, total_spawned) = application.spawn_batch(100);
    ///
    /// assert!(total_spawned == 100);
    ///
    /// // Use the IDs of the spawned entities for further operations.
    /// ```

    pub fn spawn_batch(&mut self, amount: usize) -> (Entity, usize) {
        let leader = self.spawn();

        for _ in 1..amount {
            self.spawn();
        }

        return (leader, amount);
    }

    /// Spawns a set of entities and returns their IDs.
    ///
    /// # Arguments
    ///
    /// * `amount` - The number of entities to spawn in the set.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<Entity>` containing the IDs.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// let entities = application.spawn_set(50);
    ///
    /// assert!(entities.len() == 50);
    /// ```

    pub fn spawn_set(&mut self, amount: usize) -> Vec<Entity> {
        let mut entities = Vec::new();

        for _ in 0..amount {
            entities.push(self.spawn());
        }

        return entities;
    }

    /// Destroy a single entity
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to destroy
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// let entity = application.spawn();
    ///
    /// application.destroy(entity);
    /// ```

    pub fn destroy(&mut self, entity: Entity) {
        if let Some(components) = self.components_tracker.remove(&entity) {
            for component in components.iter().cloned() {
                let _ = self.try_remove_any_component(entity, component);
            }
        }
    }

    /// Destroy a batch
    ///
    /// # Arguments
    ///
    /// * `batch` - The batch to destroy.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// let batch = application.spawn_batch(50);
    ///
    /// application.destroy_batch(batch);
    /// ```

    pub fn destroy_batch(&mut self, batch: (Entity, usize)) {
        let (leader, amount) = batch;
        if let Some(components) = self.components_tracker.remove(&leader) {
            for component in components {
                let _ = self.try_remove_any_component_batch((leader, amount), component);
            }
        }
    }

    /// Destroy a set of entities
    ///
    /// # Arguments
    ///
    /// * `entities` - The set of entities to destroy.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// let entities = application.spawn_set(50);
    ///
    /// application.destroy_set(&entities);
    /// ```

    pub fn destroy_set(&mut self, entities: &[Entity]) {
        for entity in entities {
            self.destroy(entity.clone());
        }
    }

    /// Runs the application loop with a specified maximum rate for tick systems.
    ///
    /// # Arguments
    ///
    /// * `max_rate` - The maximum rate at which tick systems should be executed in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// // Run the application loop with the specified maximum tick rate.
    /// application.run(60f32);
    /// ```

    pub fn run(&mut self, max_rate: f32) {
        let starting_time = time::Instant::now();
        let mut previous_time = 0f32;

        'main: loop {
            let now_time = starting_time.elapsed().as_secs_f32();
            let delta_time = now_time - previous_time;

            previous_time = now_time;

            while let Some(event) = self.events.pop_front() {
                let (close, event) = self.process_event(event);

                if close {
                    break 'main;
                }

                if let Some(event) = event {
                    self.launch_event_systems(event);
                }
            }

            self.launch_tick_systems(delta_time);

            let sleep_time = ((1f32 / max_rate) - delta_time).abs();
            std::thread::sleep(time::Duration::from_secs_f32(sleep_time));
        }
    }

    /// Tries to view a slice of entities belonging to a specific group.
    ///
    /// # Arguments
    ///
    /// * `group` - The group for which to retrieve the entity slice.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a slice of entities belonging to the specified group, or `None` if the group is not found.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// let target_group = 1283719281320; // Let's imagine that this is the unique ID of a certain group.
    ///
    /// // Try to view a slice of entities belonging to the specified group.
    /// if let Some(entity_slice) = application.try_view(target_group) {
    ///     // Process the entity slice as needed.
    /// } else {
    ///     // Handle the case where the specified group is not found.
    /// }
    /// ```

    pub fn try_view(&self, group: Group) -> Option<&[Entity]> {
        return self.entities.try_view(group);
    }

    /// Retrieves a reference to the internal storage of entities grouped by their components.
    ///
    /// # Returns
    ///
    /// Returns a reference to the internal storage of entities organized by their components.

    pub fn entities(&self) -> &[Vec<Entity>] {
        return self.entities.entities();
    }
}

/// Systems management functions

impl Application {
    /// Launches event systems to handle the specified event.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to be processed by the event systems.

    fn launch_event_systems(&mut self, event: Box<dyn AnyEvent>) {
        let mut world = World::new(&mut self.components);

        if let Some(systems) = self.event_systems.get_mut(&event.id()) {
            for system in systems {
                let group = system.borrow().group().clone();

                if let Some(entities) = self.entities.try_view(group) {
                    system.borrow_mut().on_event(entities, &mut world, &event);
                }
            }
        }

        self.events.append(&mut world.events);
    }

    /// Launches tick systems with the specified delta time.
    ///
    /// # Arguments
    ///
    /// * `delta_time` - The time elapsed since the last tick in seconds.

    fn launch_tick_systems(&mut self, delta_time: f32) {
        let mut world = World::new(&mut self.components);

        for system in &mut self.tick_systems {
            let group = system.borrow().group().clone();

            if let Some(entities) = self.entities.try_view(group) {
                system.borrow_mut().on_tick(delta_time, entities, &mut world);
            }
        }

        self.events.append(&mut world.events);
    }

    /// Process the event if it's an application event.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to be processed.
    ///
    /// # Returns
    ///
    /// Returns the event if the application did not process it.

    fn process_event(&mut self, event: Box<dyn AnyEvent>) -> (bool, Option<Box<dyn AnyEvent>>) {
        if event.id() == basic::events::CloseApplication::event_id() {
            return (true, None);
        }

        if event.id() == basic::events::ModeratorCloseApplication::event_id() {
            return (true, None);
        }

        if event.id() == basic::events::TryAddComponent::event_id() {
            let event = event.into_any().downcast::<basic::events::TryAddComponent>().unwrap();

            let entity = event.entity;
            let component_id = event.component.id();

            if let Err(_) = self.try_add_any_component(entity, event.component) {
                log::warn!("Failed to add component {} to entity {}", component_id, entity);
            }

            return (false, None);
        }

        if event.id() == basic::events::TryAddComponentBatch::event_id() {
            let event = event.into_any().downcast::<basic::events::TryAddComponentBatch>().unwrap();

            let batch = event.batch;
            let components = event.components;

            if let Err(_) = self.try_add_any_component_batch(batch, components) {
                log::warn!("Failed to add components to batch {:?}", batch);
            }

            return (false, None);
        }

        if event.id() == basic::events::TryAddComponentSet::event_id() {
            let event = event.into_any().downcast::<basic::events::TryAddComponentSet>().unwrap();

            let set = event.entities;
            let components = event.components;

            if let Err(_) = self.try_add_any_component_set(&set, components) {
                log::warn!("Failed to add components to set {:?}", set);
            }

            return (false, None);
        }

        if event.id() == basic::events::TryRemoveComponent::event_id() {
            if let Some(event) = event.as_any().downcast_ref::<basic::events::TryRemoveComponent>() {
                let entity = event.entity;
                let component_id = event.component_id;

                if let Err(_) = self.try_remove_any_component(entity, component_id) {
                    log::warn!("Failed to remove component {} from entity {}", component_id, entity);
                }

                return (false, None);
            }
        }

        if event.id() == basic::events::TryRemoveComponentBatch::event_id() {
            if let Some(event) = event.as_any().downcast_ref::<basic::events::TryRemoveComponentBatch>() {
                let batch = event.batch;
                let component_id = event.component_id;

                if let Err(_) = self.try_remove_any_component_batch(batch, component_id) {
                    log::warn!("Failed to remove component {} from batch {:?}", component_id, batch);
                }

                return (false, None);
            }
        }

        if event.id() == basic::events::TryRemoveComponentSet::event_id() {
            if let Some(event) = event.as_any().downcast_ref::<basic::events::TryRemoveComponentSet>() {
                let set = &event.entities;
                let component_id = event.component_id;

                if let Err(_) = self.try_remove_any_component_set(set, component_id) {
                    log::warn!("Failed to remove component {} from set {:?}", component_id, set);
                }

                return (false, None);
            }
        }

        return (false, Some(event));
    }
}

// Bundle

impl Application {
    /// Creates a bundle for modifying and interacting with a specific entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to create the bundle.
    ///
    /// # Returns
    ///
    /// Returns a bundle instance for the specified entity.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// let entity = application.spawn();
    ///
    /// // Create a bundle for the specified entity.
    /// let bundle = application.bundle(entity);
    ///
    /// // Use the entity bundle for modifying or interacting with the entity.
    /// ```
    pub fn bundle(&mut self, entity: Entity) -> bundle::Bundle {
        return bundle::Bundle::new(entity, self);
    }

    /// Creates a batch bundle for modifying and interacting with entities spawned in a batch.
    ///
    /// # Arguments
    ///
    /// * `batch` - A tuple containing the ID of the first entity in the batch and the total number of entities spawned.
    ///
    /// # Returns
    ///
    /// Returns a batch bundle instance for the specified entity batch.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// let batch = application.spawn_batch(100);
    ///
    /// // Create a bundle for the specified batch.
    /// let bundle = application.batch_bundle(batch);
    ///
    /// // Use the batch bundle for modifying or interacting with the batch.
    /// ```
    pub fn batch_bundle(&mut self, batch: (Entity, usize)) -> bundle::BatchBundle {
        return bundle::BatchBundle::new(batch, self);
    }

    /// Creates a set bundle for modifying and interacting with a set of entities.
    ///
    /// # Arguments
    ///
    /// * `entities` - A vector containing the entities for which to create the set bundle.
    ///
    /// # Returns
    ///
    /// Returns a set bundle instance for the specified set of entities.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let mut application = ApplicationBuilder::new().build();
    ///
    /// let entities = application.spawn_set(50);
    ///
    /// // Create a bundle for the specified set.
    /// let bundle = application.set_bundle(entities);
    ///
    /// // Use the set bundle for modifying or interacting with entities.
    /// ```
    pub fn set_bundle(&mut self, entities: Vec<Entity>) -> bundle::SetBundle {
        return bundle::SetBundle::new(entities, self);
    }
}

// Get components

impl Application {
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
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_component(entity, TestComponent {});
    ///
    /// // Try to get the component of the specified entity.
    /// if let Some(any_component) = application.try_get_any_component(entity, TestComponent::component_id()) {
    ///     // Now any_component is a `&Box<dyn AnyComponent>`
    /// }
    ///  ```

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
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_component(entity, TestComponent {});
    ///
    /// // Try to get the component of the specified entity.
    /// if let Some(any_component) = application.try_get_any_mut_component(entity, TestComponent::component_id()) {
    ///     // Now any_component is a `&mut Box<dyn AnyComponent>`
    /// }
    ///  ```

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
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_component(entity, TestComponent {});
    ///
    /// // Try to get the component of the specified entity.
    /// if let Some(any_component) = application.try_get_component::<TestComponent>(entity) {
    ///     // Now any_component is a `&TestComponent`
    /// }
    ///  ```

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
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_component(entity, TestComponent {});
    ///
    /// // Try to get the component of the specified entity.
    /// if let Some(any_component) = application.try_get_mut_component::<TestComponent>(entity) {
    ///     // Now any_component is a `&mut TestComponent`
    /// }
    ///  ```

    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        return self.components.try_get_mut_component::<T>(entity);
    }
}

// Add components

impl Application {
    /// Adds a component to the given entity. If the entity already has the component, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which the component should be added.
    /// * `value` - A boxed trait object implementing `AnyComponent` representing the component to be added.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the component is successfully added to the entity.
    /// Returns `Err(())` if the entity already has the component.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_any_component(entity, Box::new (TestComponent {}));
    ///  ```

    pub fn try_add_any_component(&mut self, entity: Entity, value: Box<dyn AnyComponent>) -> Result<(), ()> {
        let id = value.id();

        return match self.components.try_add_any_component(entity, value) {
            Ok(()) => {
                if let Some(previous_components) = self.components_tracker.get_mut(&entity) {
                    let groups = self.mapping.get_next_membership(&previous_components, &AHashSet::from([id]));

                    let result = self.entities.try_add_groups_to_entity(&groups, entity);

                    if let Err(e) = result {
                        log::warn!("Error while adding entity to groups {:?} : {:?}", groups, e);
                    }

                    for group in groups {
                        if let Some(systems) = self.join_systems.get_mut(&group) {
                            for system in systems {
                                let mut world = World::new(&mut self.components);

                                system.borrow_mut().on_join(&[entity], &mut world);

                                self.events.append(&mut world.events);
                            }
                        }
                    }

                    previous_components.insert(id);
                }

                Ok(())
            }
            Err(()) => Err(())
        };
    }

    /// Attempts to add multiple components to entities in a batch.
    ///
    /// # Arguments
    ///
    /// * `batch` - A tuple containing the ID of the first entity in the batch and the total number of entities spawned.
    /// * `values` - A vector containing boxed instances of components to be added to the entities in the batch.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all components are successfully added to the entities, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let batch = application.spawn_batch(100);
    ///
    /// let _ = application.try_add_any_component_batch(batch, vec![Box::new (TestComponent {}); 100]);
    ///  ```

    pub fn try_add_any_component_batch(&mut self, batch: (Entity, usize), values: Vec<Box<dyn AnyComponent>>) -> Result<(), ()> {
        let (leader, amount) = batch;
        if let Some(previous_components) = self.components_tracker.get(&leader) {
            if let Some(first) = values.first() {
                let groups = self.mapping.get_next_membership(&previous_components, &AHashSet::from([first.id()]));

                let entities = (leader..(leader + amount as u64)).collect::<Vec<Entity>>();
                let result = self.entities.try_add_groups_to_entities(&groups, &entities);

                for &entity in &entities {
                    if let Some(previous_components) = self.components_tracker.get_mut(&entity) {
                        previous_components.insert(first.id());
                    }
                }

                if let Err(e) = result {
                    log::warn!("Error while adding entity to groups {:?} : {:?}", groups, e);
                }

                let mut result = Ok(());

                for (&entity, value) in entities.iter().zip(values) {
                    if let Err(()) = self.components.try_add_any_component(entity, value) {
                        result = Err(());
                    }
                }

                for group in groups {
                    if let Some(systems) = self.join_systems.get_mut(&group) {
                        for system in systems {
                            let mut world = World::new(&mut self.components);

                            system.borrow_mut().on_join(&entities, &mut world);

                            self.events.append(&mut world.events);
                        }
                    }
                }

                return result;
            }
        }

        Err(())
    }

    /// Attempts to add multiple components to entities in a set.
    ///
    /// # Arguments
    ///
    /// * `set` - A slice containing the entities to which components should be added.
    /// * `values` - A vector containing boxed instances of components to be added to the entities in the set.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all components are successfully added to the entities, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let set = application.spawn_set(50);
    ///
    /// let _ = application.try_add_any_component_set(set, vec![Box::new (TestComponent {}); 50]);
    ///  ```

    pub fn try_add_any_component_set(&mut self, set: &[Entity], values: Vec<Box<dyn AnyComponent>>) -> Result<(), ()> {
        let mut result = Ok(());
        for (entity, component) in set.iter().zip(values) {
            if let Err(()) = self.try_add_any_component(*entity, component) {
                result = Err(());
            }
        }

        return result;
    }

    /// Attempts to add a specific type of component to a specified entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which the component should be added.
    /// * `value` - The value of the component to be added to the entity.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the component is successfully added to the entity, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_component(entity, TestComponent {});
    ///  ```

    pub fn try_add_component<T: AnyComponent + 'static>(&mut self, entity: Entity, value: T) -> Result<(), ()> {
        return self.try_add_any_component(entity, Box::from(value));
    }

    /// Attempts to add multiple components of a specific type to entities in a batch.
    ///
    /// # Arguments
    ///
    /// * `batch` - A tuple containing the ID of the first entity in the batch and the total number of entities spawned.
    /// * `values` - A vector containing values of the component type to be added to the entities in the batch.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all components are successfully added to the entities, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let batch = application.spawn_batch(100);
    ///
    /// let _ = application.try_add_component_batch(batch, vec![TestComponent {};100]);
    ///  ```

    pub fn try_add_component_batch<T: AnyComponent + 'static>(&mut self, batch: (Entity, usize), values: Vec<T>) -> Result<(), ()> {
        let mut box_values = Vec::<Box<dyn AnyComponent>>::new();

        for value in values {
            box_values.push(Box::from(value));
        }

        return self.try_add_any_component_batch((batch.0, batch.1), box_values);
    }

    /// Attempts to add multiple cloned instances of a specific type of component to entities in a batch.
    ///
    /// # Arguments
    ///
    /// * `batch` - A tuple containing the ID of the first entity in the batch and the total number of entities spawned.
    /// * `value` - The cloned instance of the component type to be added to the entities in the batch.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all cloned instances are successfully added to the entities, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let batch = application.spawn_batch(100);
    ///
    /// let _ = application.try_add_component_batch(batch, TestComponent {});
    ///  ```

    pub fn try_add_component_batch_clone<T: Clone + AnyComponent + 'static>(&mut self, batch: (Entity, usize), value: T) -> Result<(), ()> {
        let mut values = Vec::<Box<dyn AnyComponent>>::new();

        for _ in 0..batch.1 {
            values.push(Box::from(value.clone()));
        }

        return self.try_add_any_component_batch((batch.0, batch.1), values);
    }

    /// Attempts to add multiple components of a specific type to entities in a set.
    ///
    /// # Arguments
    ///
    /// * `entities` - A slice containing the entities to which components should be added.
    /// * `values` - A vector containing values of the component type to be added to the entities in the set.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all components are successfully added to the entities, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let set = application.spawn_set(50);
    ///
    /// let _ = application.try_add_component_set(set, vec![TestComponent {}; 50]);
    ///  ```

    pub fn try_add_component_set<T: AnyComponent + 'static>(&mut self, entities: &[Entity], values: Vec<T>) -> Result<(), ()> {
        let mut box_values = Vec::<Box<dyn AnyComponent>>::new();

        for value in values {
            box_values.push(Box::from(value));
        }

        return self.try_add_any_component_set(entities, box_values);
    }

    /// Attempts to add multiple cloned instances of a specific type of component to entities in a set.
    ///
    /// # Arguments
    ///
    /// * `entities` - A slice containing the entities to which cloned instances of components should be added.
    /// * `value` - The cloned instance of the component type to be added to the entities in the set.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all cloned instances are successfully added to the entities, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let set = application.spawn_set(50);
    ///
    /// let _ = application.try_add_component_set_clone(set, TestComponent {});
    ///  ```

    pub fn try_add_component_set_clone<T: Clone + AnyComponent + 'static>(&mut self, entities: &[Entity], value: T) -> Result<(), ()> {
        let mut box_values = Vec::<Box<dyn AnyComponent>>::new();

        for _ in 0..entities.len() {
            box_values.push(Box::from(value.clone()));
        }

        return self.try_add_any_component_set(entities, box_values);
    }

    /// Attempts to add a specific type of component to a specified entity and returns a reference to the added component.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which the component should be added.
    /// * `value` - The value of the component to be added to the entity.
    ///
    /// # Returns
    ///
    /// Returns `Some(&T)` with a reference to the added component if successful, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// if let Some (test) = application.try_add_get_component(entity, TestComponent {}) {
    ///     // Now test is a `&TestComponent`
    /// }
    ///  ```

    pub fn try_add_get_component<T: AnyComponent + 'static>(&mut self, entity: Entity, value: T) -> Option<&T> {
        return match self.try_add_component::<T>(entity, value) {
            Ok(()) => self.try_get_component::<T>(entity),
            Err(()) => None
        };
    }

    /// Attempts to add a specific type of component to a specified entity and returns a mutable reference to the added component.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which the component should be added.
    /// * `value` - The value of the component to be added to the entity.
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut T)` with a mutable reference to the added component if successful, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// if let Some (test) = application.try_add_get_mut_component(entity, TestComponent {}) {
    ///     // Now test is a `&mut TestComponent`
    /// }
    ///  ```

    pub fn try_add_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: Entity, value: T) -> Option<&mut T> {
        return match self.try_add_component::<T>(entity, value) {
            Ok(()) => self.try_get_mut_component::<T>(entity),
            Err(()) => None
        };
    }
}

// Remove components

impl Application {
    /// Attempts to remove a component of a specific type from a specified entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity from which the component should be removed.
    /// * `id` - The identifier of the component type to be removed from the entity.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Box<dyn AnyComponent>)` with a boxed instance of the removed component if successful, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_any_component(entity, Box::new (TestComponent {}));
    ///
    /// let _ = application.try_remove_any_component(entity, TestComponent::component_id());
    ///  ```

    pub fn try_remove_any_component(&mut self, entity: Entity, id: ComponentID) -> Result<Box<dyn AnyComponent>, ()> {
        return match self.components.try_remove_any_component(entity, id) {
            Ok(any_component) => {
                if let Some(previous_components) = self.components_tracker.get_mut(&entity) {
                    previous_components.remove(&id);

                    let groups = self.mapping.get_next_membership(&previous_components, &AHashSet::from([id]));

                    let result = self.entities.try_remove_groups_to_entity(&groups, entity);

                    if let Err(e) = result {
                        log::warn!("Error while removing entity from groups {:?} : {:?}", groups, e);
                    }

                    for group in groups {
                        if let Some(systems) = self.quit_systems.get_mut(&group) {
                            for system in systems {
                                let mut world = World::new(&mut self.components);

                                system.borrow_mut().on_quit(&[entity], &mut world);

                                self.events.append(&mut world.events);
                            }
                        }
                    }
                }

                Ok(any_component)
            }
            Err(()) => Err(())
        };
    }

    /// Attempts to remove components of a specific type from entities in a batch.
    ///
    /// # Arguments
    ///
    /// * `batch` - A tuple containing the ID of the first entity in the batch and the total number of entities spawned.
    /// * `id` - The identifier of the component type to be removed from the entities in the batch.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Box<dyn AnyComponent>>)` with a vector of boxed instances of the removed components if successful,
    /// otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let batch = application.spawn_batch(100);
    ///
    /// let _ = application.try_add_any_component_batch(batch, vec![Box::new (TestComponent {}); 100]);
    /// let _ = application.try_remove_any_component_batch(batch, TestComponent::component_id());
    ///  ```

    pub fn try_remove_any_component_batch(&mut self, batch: (Entity, usize), id: ComponentID) -> Result<Vec<Box<dyn AnyComponent>>, ()> {
        let (leader, amount) = batch;
        let entities = (leader..(leader + amount as u64)).collect::<Vec<Entity>>();

        for &entity in &entities {
            if let Some(previous_components) = self.components_tracker.get_mut(&entity) {
                previous_components.remove(&id);
            }
        }

        if let Some(previous_components) = self.components_tracker.get_mut(&leader) {
            let groups = self.mapping.get_next_membership(&previous_components, &AHashSet::from([id]));

            let result = self.entities.try_remove_groups_to_entities(&groups, &entities);

            if let Err(e) = result {
                log::warn!("Error while adding entity to groups {:?} : {:?}", groups, e);
            }

            let mut result = Ok(Vec::new());

            let mut components = Vec::<Box<dyn AnyComponent>>::new();
            for &entity in &entities {
                let res = self.components.try_remove_any_component(entity, id);

                if let Ok(component) = res {
                    components.push(component);
                } else {
                    result = Err(());
                }
            }

            for group in groups {
                if let Some(systems) = self.quit_systems.get_mut(&group) {
                    for system in systems {
                        let mut world = World::new(&mut self.components);

                        system.borrow_mut().on_join(&entities, &mut world);

                        self.events.append(&mut world.events);
                    }
                }
            }

            return result;
        }

        Err(())
    }

    /// Attempts to remove components of a specific type from entities in a set.
    ///
    /// # Arguments
    ///
    /// * `entities` - A slice containing the entities from which components should be removed.
    /// * `id` - The identifier of the component type to be removed from the entities in the set.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Box<dyn AnyComponent>>)` with a vector of boxed instances of the removed components if successful,
    /// otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let set = application.spawn_set(50);
    ///
    /// let _ = application.try_add_any_component_set(set, vec![Box::new (TestComponent {}); 50]);
    ///
    /// let _ = application.try_remove_any_component_set(set, TestComponent::component_id());
    ///  ```

    pub fn try_remove_any_component_set(&mut self, entities: &[Entity], id: ComponentID) -> Result<Vec<Box<dyn AnyComponent>>, ()> {
        let mut result = Ok(Vec::new());
        for &entity in entities {
            if let Err(()) = self.try_remove_any_component(entity, id) {
                result = Err(());
            }
        }

        return result;
    }

    /// Attempts to remove a component of a specific type from a specified entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity from which the component should be removed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the component is successfully removed from the entity, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_component(entity, TestComponent {});
    ///
    /// let _ = application.try_remove_component::<TestComponent>(entity);
    ///  ```

    pub fn try_remove_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Result<(), ()> {
        return self.try_remove_any_component(entity, T::component_id()).map(|_| ());
    }

    /// Attempts to remove components of a specific type from entities in a batch.
    ///
    /// # Arguments
    ///
    /// * `batch` - A tuple containing the ID of the first entity in the batch and the total number of entities spawned.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if components are successfully removed from entities in the batch, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let batch = application.spawn_batch(100);
    ///
    /// let _ = application.try_add_component_batch(batch, vec![TestComponent {};100]);
    ///
    /// let _ = application.try_remove_component_batch::<TestComponent>(batch);
    ///  ```

    pub fn try_remove_component_batch<T: AnyComponent + 'static>(&mut self, batch: (Entity, usize)) -> Result<(), ()> {
        return self.try_remove_any_component_batch(batch, T::component_id()).map(|_| ());
    }

    /// Attempts to remove components of a specific type from entities in a set.
    ///
    /// # Arguments
    ///
    /// * `entities` - A slice containing the entities from which components should be removed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if components are successfully removed from entities in the set, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let set = application.spawn_set(50);
    ///
    /// let _ = application.try_add_component_set(set, vec![TestComponent {}; 50]);
    ///
    /// let _ = application.try_remove_component_set::<TestComponent>(set);
    ///  ```

    pub fn try_remove_component_set<T: AnyComponent + 'static>(&mut self, entities: &[Entity]) -> Result<(), ()> {
        return self.try_remove_any_component_set(entities, T::component_id()).map(|_| ());
    }

    /// Attempts to remove a component of a specific type from a specified entity and returns a boxed instance of the removed component.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity from which the component should be removed.
    /// * `id` - The identifier of the component type to be removed from the entity.
    ///
    /// # Returns
    ///
    /// Returns `Some(Box<dyn AnyComponent>)` with a boxed instance of the removed component if successful, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_get_component(entity, TestComponent {});
    ///
    /// if let Some (test) = application.try_remove_get_any_component(entity, TestComponent::component_id()) {
    ///   // Now test is a `Box<dyn AnyComponent>`
    /// }
    ///  ```

    pub fn try_remove_get_any_component(&mut self, entity: Entity, id: ComponentID) -> Option<Box<dyn AnyComponent>> {
        return self.try_remove_any_component(entity, id).ok();
    }

    /// Attempts to remove a component of a specific type from a specified entity and returns a boxed instance of the removed component.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity from which the component should be removed.
    ///
    /// # Returns
    ///
    /// Returns `Some(Box<T>)` with a boxed instance of the removed component if successful, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent {}
    ///
    /// let mut application = ApplicationBuilder::new().build();
    /// let entity = application.spawn();
    ///
    /// let _ = application.try_add_get_component(entity, TestComponent {});
    ///
    /// if let Some (test) = application.try_remove_get_component::<TestComponent>(entity) {
    ///   // Now test is a `Box<TestComponent>`
    /// }
    ///  ```

    pub fn try_remove_get_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Option<Box<T>> {
        return self.try_remove_any_component(entity, T::component_id()).ok().and_then(
            |component| component.into_any().downcast::<T>().ok());
    }
}