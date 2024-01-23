use std::collections::{
    HashMap,
    HashSet
};

use crate::{
    application::{
        Application,
        ApplicationSignal
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
    signal_systems: HashMap<ApplicationSignal, Vec<Box<dyn System>>>,
    event_systems: HashMap<EventID, Vec<Box<dyn System>>>,

    join_systems: Vec<Box<dyn System>>,
    quit_systems: Vec<Box<dyn System>>,
    tick_systems: Vec<Box<dyn System>>,

    descriptor: MemoryMappingDescriptor,
    seen: HashSet<Group>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        return Self {
            signal_systems: HashMap::new(),
            event_systems: HashMap::new(),

            join_systems: Vec::new(),
            quit_systems: Vec::new(),
            tick_systems: Vec::new(),

            descriptor: MemoryMappingDescriptor::new(),
            seen: HashSet::new()
        }
    }

    pub fn build(self) -> Application {
        return Application::new(
            self.descriptor,
            self.signal_systems,
            self.event_systems,
            self.join_systems,
            self.quit_systems,
            self.tick_systems,
        )
    }

    pub fn add_signal_system(&mut self, signal: ApplicationSignal, system: Box<dyn System>) {
        if !self.signal_systems.contains_key(&signal) {
            self.signal_systems.insert(signal.clone(), Vec::new());
        }

        if !self.seen.contains(&system.group()) {
            self.descriptor.push(system.components());
            self.seen.insert(system.group());
        }

        self.signal_systems.get_mut(&signal).unwrap().push(system);
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