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

pub struct Application {
    mapping: MemoryMapping,
    entities: Entities,
    components: Components,

    next_entity: Entity,
    components_tracker: AHashMap<Entity, AHashSet<ComponentID>>,

    events: VecDeque<Box<dyn AnyEvent>>,

    event_systems: AHashMap<EventID, Vec<CustomSystem>>,

    join_systems: AHashMap<Group, Vec<CustomSystem>>,
    quit_systems: AHashMap<Group, Vec<CustomSystem>>,
    tick_systems: Vec<CustomSystem>,
}

impl Application {
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

    /// Spawns a new entity and returns its id.
    pub fn spawn(&mut self) -> Entity {
        let result = self.next_entity;

        self.components_tracker.insert(self.next_entity as Entity, AHashSet::new());
        self.next_entity += 1;

        return result;
    }

    /// Spawns a batch of entities and returns their ids. Warning : a batch of entities
    /// should be used when you need to spawn a lot of similar entities. Which means that they must have
    /// the same components. If you need to spawn entities with different components, you should use
    /// the `spawn` method.
    pub fn spawn_batch(&mut self, amount: usize) -> (Entity, usize) {
        let leader = self.spawn();

        for _ in 1..amount {
            self.spawn();
        }

        return (leader, amount);
    }

    pub fn run(&mut self, max_rate: f32) {
        let starting_time = time::Instant::now();
        let mut previous_time = 0f32;

        'main: loop {
            let now_time = starting_time.elapsed().as_secs_f32();
            let delta_time = now_time - previous_time;

            previous_time = now_time;

            while let Some(event) = self.events.pop_front() {
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
            }

            self.launch_tick_systems(delta_time);

            let sleep_time = ((1f32 / max_rate) - delta_time).abs();
            std::thread::sleep(time::Duration::from_secs_f32(sleep_time));
        }
    }

    pub fn try_view(&self, group: Group) -> Option<&[Entity]> {
        return self.entities.try_view(group);
    }

    pub fn entities(&self) -> &[Vec<Entity>] {
        return self.entities.entities();
    }
}

/// Systems management functions

impl Application {
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
    pub fn bundle(&mut self, entity: Entity) -> bundle::Bundle {
        return bundle::Bundle::new(entity, self);
    }

    pub fn batch_bundle(&mut self, batch: (Entity, usize)) -> bundle::BatchBundle {
        return bundle::BatchBundle::new(batch, self);
    }

    pub fn multiple_bundle(&mut self, entities: Vec<Entity>) -> bundle::MultipleBundle {
        return bundle::MultipleBundle::new(entities, self);
    }
}

// Get components

impl Application {
    pub fn try_get_any_component(&self, entity: Entity, id: ComponentID) -> Option<&Box<dyn AnyComponent>> {
        return self.components.try_get_any_component(entity, id);
    }

    pub fn try_get_any_mut_component(&mut self, entity: Entity, id: ComponentID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.components.try_get_any_mut_component(entity, id);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: Entity) -> Option<&T> {
        return self.components.try_get_component::<T>(entity);
    }

    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        return self.components.try_get_mut_component::<T>(entity);
    }
}

// Add components

impl Application {
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

    pub fn try_add_component<T: AnyComponent + 'static>(&mut self, entity: Entity, value: T) -> Result<(), ()> {
        return self.try_add_any_component(entity, Box::from(value));
    }

    pub fn try_add_component_batch<T: AnyComponent + 'static>(&mut self, batch: (Entity, usize), values: Vec<T>) -> Result<(), ()> {
        let mut box_values = Vec::<Box<dyn AnyComponent>>::new();

        for value in values {
            box_values.push(Box::from(value));
        }

        return self.try_add_any_component_batch((batch.0, batch.1), box_values);
    }

    pub fn try_add_component_batch_clone<T: Clone + AnyComponent + 'static>(&mut self, batch: (Entity, usize), value: T) -> Result<(), ()> {
        let mut values = Vec::<Box<dyn AnyComponent>>::new();

        for _ in 0..batch.1 {
            values.push(Box::from(value.clone()));
        }

        return self.try_add_any_component_batch((batch.0, batch.1), values);
    }

    pub fn try_add_get_component<T: AnyComponent + 'static>(&mut self, entity: Entity, value: T) -> Option<&T> {
        return match self.try_add_component::<T>(entity, value) {
            Ok(()) => self.try_get_component::<T>(entity),
            Err(()) => None
        };
    }

    pub fn try_add_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: Entity, value: T) -> Option<&mut T> {
        return match self.try_add_component::<T>(entity, value) {
            Ok(()) => self.try_get_mut_component::<T>(entity),
            Err(()) => None
        };
    }
}

// Remove components

impl Application {
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

    pub fn try_remove_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Result<(), ()> {
        return self.try_remove_any_component(entity, T::component_id()).map(|_| ());
    }

    pub fn try_remove_component_batch<T: AnyComponent + 'static>(&mut self, batch: (Entity, usize)) -> Result<(), ()> {
        return self.try_remove_any_component_batch(batch, T::component_id()).map(|_| ());
    }

    pub fn try_remove_get_any_component(&mut self, entity: Entity, id: ComponentID) -> Option<Box<dyn AnyComponent>> {
        return self.try_remove_any_component(entity, id).ok();
    }

    pub fn try_remove_get_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Option<Box<T>> {
        return self.try_remove_any_component(entity, T::component_id()).ok().and_then(
            |component| component.into_any().downcast::<T>().ok());
    }
}