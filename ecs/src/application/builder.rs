use ahash::{
    AHashMap,
    AHashSet
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
        system::System,
        component::Group
    },
};

pub struct ApplicationBuilder {
    event_systems: AHashMap<EventID, Vec<Box<dyn System>>>,

    join_systems: Vec<Box<dyn System>>,
    quit_systems: Vec<Box<dyn System>>,
    tick_systems: Vec<Box<dyn System>>,

    descriptor: MemoryMappingDescriptor,
    seen: AHashSet<Group>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        return Self {
            event_systems: AHashMap::new(),

            join_systems: Vec::new(),
            quit_systems: Vec::new(),
            tick_systems: Vec::new(),

            descriptor: MemoryMappingDescriptor::new(),
            seen: AHashSet::new()
        }
    }

    pub fn build(self) -> Application {
        return Application::new(
            self.descriptor,
            self.event_systems,
            self.join_systems,
            self.quit_systems,
            self.tick_systems,
        )
    }

    pub fn add_event_system(&mut self, event: EventID, system: Box<dyn System>) {
        if !self.event_systems.contains_key(&event) {
            self.event_systems.insert(event, Vec::new());
        }

        if !self.seen.contains(&system.group()) {
            self.descriptor.push(system.components());
            self.seen.insert(system.group());
        }

        self.event_systems.get_mut(&event).unwrap().push(system);
    }

    pub fn add_join_system(&mut self, system: Box<dyn System>) {
        if !self.seen.contains(&system.group()) {
            self.descriptor.push(system.components());
            self.seen.insert(system.group());
        }

        self.join_systems.push(system);
    }

    pub fn add_quit_system(&mut self, system: Box<dyn System>) {
        if !self.seen.contains(&system.group()) {
            self.descriptor.push(system.components());
            self.seen.insert(system.group());
        }

        self.quit_systems.push(system);
    }

    pub fn add_tick_system(&mut self, system: Box<dyn System>) {
        if !self.seen.contains(&system.group()) {
            self.descriptor.push(system.components());
            self.seen.insert(system.group());
        }

        self.tick_systems.push(system);
    }
}