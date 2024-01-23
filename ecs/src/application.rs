use std::collections::{HashMap, HashSet, VecDeque};

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
            AnyComponent
        },
        entity::Entity,
        event::{
            EventID,
            AnyEvent
        },
        system::System,
        world::World
    },
};

pub mod builder;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum ApplicationSignal {
    ApplicationStarted,
    ApplicationStopped,
}

pub struct Application {
    mapping: MemoryMapping,
    entities: Entities,
    components: Components,

    next_entity: Entity,
    components_tracker: HashMap<Entity, HashSet<ComponentID>>,

    signals: VecDeque<ApplicationSignal>,
    events: VecDeque<Box<dyn AnyEvent>>,

    signal_systems: HashMap<ApplicationSignal, Vec<Box<dyn System>>>,
    event_systems: HashMap<EventID, Vec<Box<dyn System>>>,

    join_systems: Vec<Box<dyn System>>,
    quit_systems: Vec<Box<dyn System>>,
    tick_systems: Vec<Box<dyn System>>,
}

impl Application {
    pub fn new(descriptor: MemoryMappingDescriptor,
               signal_systems: HashMap<ApplicationSignal, Vec<Box<dyn System>>>,
               event_systems: HashMap<EventID, Vec<Box<dyn System>>>,
               join_systems: Vec<Box<dyn System>>,
               quit_systems: Vec<Box<dyn System>>,
               tick_systems: Vec<Box<dyn System>>, ) -> Self {
        let mapping = MemoryMapping::new(descriptor);

        return Self {
            components: Components::new(),
            entities: mapping.create_storage(),
            mapping: mapping,

            next_entity: 0 as Entity,
            components_tracker: HashMap::new(),

            signals: VecDeque::new(),
            events: VecDeque::new(),

            signal_systems: signal_systems,
            event_systems: event_systems,

            join_systems: join_systems,
            tick_systems: tick_systems,
            quit_systems: quit_systems,
        };
    }

    pub fn spawn(&mut self) -> Entity {
        let result = self.next_entity;

        self.components_tracker.insert(self.next_entity as Entity, HashSet::new());
        self.next_entity += 1;

        return result;
    }

    pub fn run(&mut self) {
        self.signals.push_back(ApplicationSignal::ApplicationStarted);

        loop {
            match self.signals.pop_front() {
                Some(ApplicationSignal::ApplicationStarted) => {
                    self.launch_signal_systems(ApplicationSignal::ApplicationStarted);
                }
                Some(ApplicationSignal::ApplicationStopped) => {
                    self.launch_signal_systems(ApplicationSignal::ApplicationStopped);

                    break;
                }
                None => {}
            }

            while let Some(event) = self.events.pop_front() {
                self.launch_event_systems(event);
            }

            self.launch_tick_systems();
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
    pub fn launch_signal_systems(&mut self, signal: ApplicationSignal) {
        let mut world = World::new(&mut self.components);

        if let Some(systems) = self.signal_systems.get_mut(&signal) {
            for system in systems {
                if let Some(entities) = self.entities.try_view(system.group()) {
                    system.on_signal(entities, &mut world);
                }
            }
        }
    }

    pub fn launch_event_systems(&mut self, event: Box<dyn AnyEvent>) {
        let mut world = World::new(&mut self.components);

        if let Some(systems) = self.event_systems.get_mut(&event.id()) {
            for system in systems {
                if let Some(entities) = self.entities.try_view(system.group()) {
                    system.on_event(entities, &mut world, &event);
                }
            }
        }
    }

    pub fn launch_tick_systems(&mut self) {
        let mut world = World::new(&mut self.components);

        for system in &mut self.tick_systems {
            if let Some(entities) = self.entities.try_view(system.group()) {
                system.on_tick(entities, &mut world);
            }
        }
    }
}

/// Components management functions

impl Application {
    pub fn try_add_any_component(&mut self, entity: &Entity, id: ComponentID, value: Box<dyn AnyComponent>) -> Result<(), ()> {
        return match self.components.try_add_any_component(entity, id, value) {
            Ok(()) => {
                if let Some(previous_components) = self.components_tracker.get_mut(entity) {
                    let groups = self.mapping.get_next_membership(&previous_components, &HashSet::from([id]));

                    for group in groups {
                        let result = self.entities.try_add_group(group, &[entity.clone()]);

                        if let Err(e) = result {
                            log::warn!("Error while adding entity to group: {:?}", e);
                        }

                        for system in &mut self.join_systems {
                            system.on_join(&[entity.clone()], &mut World::new(&mut self.components));
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
        return self.try_add_any_component(entity, T::id(), Box::from(value));
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
                    println!("Entity: {} / Group : {} / {:?}", entity, id, previous_components);
                    previous_components.remove(&id);

                    let groups = self.mapping.get_next_membership(&previous_components, &HashSet::from([id]));

                    println!("Groups to remove: {:?}", groups);

                    for group in groups {
                        let result = self.entities.try_remove_group(group, &[entity.clone()]);

                        if let Err(e) = result {
                            log::warn!("Error while removing entity from group: {:?}", e);
                        }

                        for system in &mut self.quit_systems {
                            system.on_quit(&[entity.clone()], &mut World::new(&mut self.components));
                        }
                    }
                }

                Ok(any_component)
            }
            Err(()) => Err(())
        };
    }

    pub fn try_remove_get_any_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<Box<dyn AnyComponent>> {
        return self.try_remove_any_component(entity, T::id()).ok();
    }

    pub fn try_remove_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Result<(), ()> {
        return self.try_remove_any_component(entity, T::id()).map(|_| ());
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

