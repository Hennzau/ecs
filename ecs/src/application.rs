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
        system::SharedSystem,
        world::World,
    },
};

pub mod builder;
pub mod basic;

pub struct Application {
    mapping: MemoryMapping,
    entities: Entities,
    components: Components,

    next_entity: Entity,
    components_tracker: AHashMap<Entity, AHashSet<ComponentID>>,

    events: VecDeque<Box<dyn AnyEvent>>,

    event_systems: AHashMap<EventID, Vec<SharedSystem>>,

    join_systems: AHashMap<Group, Vec<SharedSystem>>,
    quit_systems: AHashMap<Group, Vec<SharedSystem>>,
    tick_systems: Vec<SharedSystem>,
}

impl Application {
    pub fn new(descriptor: MemoryMappingDescriptor,
               event_systems: AHashMap<EventID, Vec<SharedSystem>>,
               join_systems: AHashMap<Group, Vec<SharedSystem>>,
               quit_systems: AHashMap<Group, Vec<SharedSystem>>,
               tick_systems: Vec<SharedSystem>) -> Self {
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

    pub fn spawn(&mut self) -> Entity {
        let result = self.next_entity;

        self.components_tracker.insert(self.next_entity as Entity, AHashSet::new());
        self.next_entity += 1;

        return result;
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
                    let _ = self.try_remove_any_component(&event.entity, event.component_id);
                }

                self.launch_event_systems(event);
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

/// Components management functions

impl Application {
    pub fn try_add_any_component(&mut self, entity: &Entity, id: ComponentID, value: Box<dyn AnyComponent>) -> Result<(), ()> {
        return match self.components.try_add_any_component(entity, id, value) {
            Ok(()) => {
                if let Some(previous_components) = self.components_tracker.get_mut(entity) {
                    let groups = self.mapping.get_next_membership(&previous_components, &AHashSet::from([id]));

                    for group in groups {
                        let result = self.entities.try_add_group(group, &[entity.clone()]);

                        if let Err(e) = result {
                            log::warn!("Error while adding entity to group: {:?}", e);
                        }

                        if let Some(systems) = self.join_systems.get_mut(&group) {
                            for system in systems {
                                let mut world = World::new(&mut self.components);

                                system.borrow_mut().on_join(&[entity.clone()], &mut world);

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

    pub fn try_add_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Result<(), ()> {
        return self.try_add_any_component(entity, T::component_id(), Box::from(value));
    }

    pub fn try_add_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Option<&T> {
        return match self.try_add_component::<T>(entity, value) {
            Ok(()) => self.try_get_component::<T>(entity),
            Err(()) => None
        };
    }

    pub fn try_add_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Option<&mut T> {
        return match self.try_add_component::<T>(entity, value) {
            Ok(()) => self.try_get_mut_component::<T>(entity),
            Err(()) => None
        };
    }

    pub fn try_remove_any_component(&mut self, entity: &Entity, id: ComponentID) -> Result<Box<dyn AnyComponent>, ()> {
        return match self.components.try_remove_any_component(entity, id) {
            Ok(any_component) => {
                if let Some(previous_components) = self.components_tracker.get_mut(entity) {
                    previous_components.remove(&id);

                    let groups = self.mapping.get_next_membership(&previous_components, &AHashSet::from([id]));

                    for group in groups {
                        let result = self.entities.try_remove_group(group, &[entity.clone()]);

                        if let Err(e) = result {
                            log::warn!("Error while removing entity from group: {:?}", e);
                        }

                        if let Some(systems) = self.quit_systems.get_mut(&group) {
                            for system in systems {
                                let mut world = World::new(&mut self.components);

                                system.borrow_mut().on_quit(&[entity.clone()], &mut world);

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

    pub fn try_remove_get_any_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<Box<dyn AnyComponent>> {
        return self.try_remove_any_component(entity, T::component_id()).ok();
    }

    pub fn try_remove_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Result<(), ()> {
        return self.try_remove_any_component(entity, T::component_id()).map(|_| ());
    }

    pub fn try_get_any_component(&self, entity: &Entity, id: ComponentID) -> Option<&Box<dyn AnyComponent>> {
        return self.components.try_get_any_component(entity, id);
    }

    pub fn try_get_any_mut_component(&mut self, entity: &Entity, id: ComponentID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.components.try_get_any_mut_component(entity, id);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return self.components.try_get_component::<T>(entity);
    }

    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return self.components.try_get_mut_component::<T>(entity);
    }
}