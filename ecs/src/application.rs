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
    /// let descriptor = // ... (create or obtain a MemoryMappingDescriptor instance)
    /// let event_systems = // ... (create or obtain an AHashMap of EventID and Vec<CustomSystem>)
    /// let join_systems = // ... (create or obtain an AHashMap of Group and Vec<CustomSystem>)
    /// let quit_systems = // ... (create or obtain an AHashMap of Group and Vec<CustomSystem>)
    /// let tick_systems = // ... (create or obtain a Vec<CustomSystem> for tick systems)
    ///
    /// // Create a new instance of the Application with the specified configurations.
    /// let application = Application::new(descriptor, event_systems, join_systems, quit_systems, tick_systems);
    ///
    /// // Start and run the created application.
    /// application.run();
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    ///
    /// // Spawn a new entity and get its ID.
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
    /// consider using the `spawn` method for individual entity spawning.
    ///
    /// # Example
    ///
    /// ```
    /// let mut application = // ... (create or obtain an Application instance)
    /// let batch_size = // ... (specify the number of entities to spawn in the batch)
    ///
    /// // Spawn a batch of entities and get the ID of the first entity and the total number of entities spawned.
    /// let (first_entity_id, total_spawned) = application.spawn_batch(batch_size);
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

    /// Runs the application loop with a specified maximum rate for tick systems.
    ///
    /// # Arguments
    ///
    /// * `max_rate` - The maximum rate at which tick systems should be executed in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// let mut application = // ... (create or obtain an Application instance)
    /// let max_tick_rate = // ... (specify the maximum rate for tick systems)
    ///
    /// // Run the application loop with the specified maximum tick rate.
    /// application.run(max_tick_rate);
    /// ```
    pub fn run(&mut self, max_rate: f32) {
        let starting_time = time::Instant::now();
        let mut previous_time = 0f32;

        'main: loop {
            let now_time = starting_time.elapsed().as_secs_f32();
            let delta_time = now_time - previous_time;

            previous_time = now_time;

            while let Some(event) = self.events.pop_front() {
                if let Some(event) = event.as_any().downcast_ref::<basic::events::ModeratorCloseApplication>() {
                    log::info!("Application closed by moderator {}", event.moderator);

                    break 'main;
                }

                if let Some(_) = event.as_any().downcast_ref::<basic::events::CloseApplication>() {
                    break 'main;
                }

                if let Some(event) = event.as_any().downcast_ref::<basic::events::TryRemoveComponent>() {
                    let _ = self.try_remove_any_component(event.entity, event.component_id);
                } else if let Some(_) = event.as_any().downcast_ref::<basic::events::TryAddComponent>() {
                    let event = event.into_any().downcast::<basic::events::TryAddComponent>().unwrap();

                    let entity = event.entity.clone();

                    let _ = self.try_add_any_component(entity, event.component);
                } else {
                    self.launch_event_systems(event);
                }

                // TODO: manage other events 'TryBatch, TrySet'
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
    /// let application = // ... (create or obtain an Application instance)
    /// let target_group = // ... (specify the group for which to view entities)
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
    ///
    /// # Example
    ///
    /// ```
    /// let application = // ... (create or obtain an Application instance)
    ///
    /// // Get a reference to the internal storage of entities.
    /// let entity_storage = application.entities();
    ///
    /// // Use the entity storage for further operations.
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// let mut application = // ... (create or obtain an Application instance)
    /// let custom_event = // ... (create or obtain a Box<dyn AnyEvent> instance)
    ///
    /// // Launch event systems to handle the specified event.
    /// application.launch_event_systems(custom_event);
    /// ```
    pub fn launch_event_systems(&mut self, event: Box<dyn AnyEvent>) {
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
    ///
    /// # Example
    ///
    /// ```
    /// let mut application = // ... (create or obtain an Application instance)
    /// let elapsed_time = // ... (calculate or obtain the time elapsed since the last tick)
    ///
    /// // Launch tick systems with the specified delta time.
    /// application.launch_tick_systems(elapsed_time);
    /// ```
    pub fn launch_tick_systems(&mut self, delta_time: f32) {
        let mut world = World::new(&mut self.components);

        for system in &mut self.tick_systems {
            let group = system.borrow().group().clone();

            if let Some(entities) = self.entities.try_view(group) {
                system.borrow_mut().on_tick(delta_time, entities, &mut world);
            }
        }

        self.events.append(&mut world.events);
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entity = // ... (specify the entity for which to create a bundle)
    ///
    /// // Create a bundle for the specified entity.
    /// let entity_bundle = application.bundle(target_entity);
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let entity_batch = // ... (specify the entity batch using spawn_batch or other methods)
    ///
    /// // Create a batch bundle for the specified entity batch.
    /// let batch_bundle = application.batch_bundle(entity_batch);
    ///
    /// // Use the batch bundle for modifying or interacting with the entities in the batch.
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entities = // ... (specify the set of entities for which to create a set bundle)
    ///
    /// // Create a set bundle for the specified set of entities.
    /// let set_bundle = application.set_bundle(target_entities);
    ///
    /// // Use the set bundle for modifying or interacting with the entities in the set.
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let entity_batch = // ... (specify the entity batch using spawn_batch or other methods)
    /// let components_to_add = // ... (create or obtain a vector of components to add)
    ///
    /// // Try to add multiple components to entities in the specified batch.
    /// let result = application.try_add_any_component_batch(entity_batch, components_to_add);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entities = // ... (specify the set of entities to which components should be added)
    /// let components_to_add = // ... (create or obtain a vector of components to add)
    ///
    /// // Try to add multiple components to entities in the specified set.
    /// let result = application.try_add_any_component_set(target_entities, components_to_add);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entity = // ... (specify the entity to which the component should be added)
    /// let component_value = // ... (create or obtain the value of the component to be added)
    ///
    /// // Try to add a specific type of component to the specified entity.
    /// let result = application.try_add_component(target_entity, component_value);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let entity_batch = // ... (specify the entity batch using spawn_batch or other methods)
    /// let components_to_add = // ... (create or obtain a vector of components to add)
    ///
    /// // Try to add multiple components of a specific type to entities in the specified batch.
    /// let result = application.try_add_component_batch(entity_batch, components_to_add);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// use std::clone::Clone;
    /// let mut application = // ... (create or obtain an Application instance)
    /// let entity_batch = // ... (specify the entity batch using spawn_batch or other methods)
    /// let component_value = // ... (create or obtain the cloned instance of the component to add)
    ///
    /// // Try to add multiple cloned instances of a specific type of component to entities in the specified batch.
    /// let result = application.try_add_component_batch_clone(entity_batch, component_value);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entities = // ... (specify the set of entities to which components should be added)
    /// let components_to_add = // ... (create or obtain a vector of components to add)
    ///
    /// // Try to add multiple components of a specific type to entities in the specified set.
    /// let result = application.try_add_component_set(target_entities, components_to_add);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// use std::clone::Clone;
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entities = // ... (specify the set of entities to which cloned instances should be added)
    /// let component_value = // ... (create or obtain the cloned instance of the component to add)
    ///
    /// // Try to add multiple cloned instances of a specific type of component to entities in the specified set.
    /// let result = application.try_add_component_set_clone(target_entities, component_value);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entity = // ... (specify the entity to which the component should be added)
    /// let component_value = // ... (create or obtain the value of the component to be added)
    ///
    /// // Try to add a specific type of component to the specified entity and get a reference to the added component.
    /// let result = application.try_add_get_component(target_entity, component_value);
    ///
    /// // Check the result and use the reference to the added component if successful.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entity = // ... (specify the entity to which the component should be added)
    /// let component_value = // ... (create or obtain the value of the component to be added)
    ///
    /// // Try to add a specific type of component to the specified entity and get a mutable reference to the added component.
    /// let result = application.try_add_get_mut_component(target_entity, component_value);
    ///
    /// // Check the result and use the mutable reference to the added component if successful.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entity = // ... (specify the entity from which the component should be removed)
    /// let component_id = // ... (specify the identifier of the component type to be removed)
    ///
    /// // Try to remove a component of a specific type from the specified entity.
    /// let result = application.try_remove_any_component(target_entity, component_id);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let entity_batch = // ... (specify the entity batch using spawn_batch or other methods)
    /// let component_id = // ... (specify the identifier of the component type to be removed)
    ///
    /// // Try to remove components of a specific type from entities in the specified batch.
    /// let result = application.try_remove_any_component_batch(entity_batch, component_id);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entities = // ... (specify the set of entities from which components should be removed)
    /// let component_id = // ... (specify the identifier of the component type to be removed)
    ///
    /// // Try to remove components of a specific type from entities in the specified set.
    /// let result = application.try_remove_any_component_set(target_entities, component_id);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entity = // ... (specify the entity from which the component should be removed)
    ///
    /// // Try to remove a component of a specific type from the specified entity.
    /// let result = application.try_remove_component::<YourComponentType>(target_entity);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let entity_batch = // ... (specify the entity batch using spawn_batch or other methods)
    ///
    /// // Try to remove components of a specific type from entities in the specified batch.
    /// let result = application.try_remove_component_batch::<YourComponentType>(entity_batch);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entities = // ... (specify the set of entities from which components should be removed)
    ///
    /// // Try to remove components of a specific type from entities in the specified set.
    /// let result = application.try_remove_component_set::<YourComponentType>(target_entities);
    ///
    /// // Check the result and handle any errors if necessary.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entity = // ... (specify the entity from which the component should be removed)
    /// let component_id = // ... (specify the identifier of the component type to be removed)
    ///
    /// // Try to remove a component of a specific type from the specified entity and get a boxed instance of the removed component.
    /// let result = application.try_remove_get_any_component(target_entity, component_id);
    ///
    /// // Check the result and use the boxed instance of the removed component if successful.
    /// ```
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
    /// let mut application = // ... (create or obtain an Application instance)
    /// let target_entity = // ... (specify the entity from which the component should be removed)
    ///
    /// // Try to remove a component of a specific type from the specified entity and get a boxed instance of the removed component.
    /// let result = application.try_remove_get_component::<YourComponentType>(target_entity);
    ///
    /// // Check the result and use the boxed instance of the removed component if successful.
    /// ```
    pub fn try_remove_get_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Option<Box<T>> {
        return self.try_remove_any_component(entity, T::component_id()).ok().and_then(
            |component| component.into_any().downcast::<T>().ok());
    }
}