use std::{
    cell::RefCell,
    rc::Rc
};

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
    event_systems: AHashMap<EventID, Vec<Rc<RefCell<dyn System>>>>,

    join_systems: AHashMap<Group, Vec<Rc<RefCell<dyn System>>>>,
    quit_systems: AHashMap<Group, Vec<Rc<RefCell<dyn System>>>>,
    tick_systems: Vec<Rc<RefCell<dyn System>>>,

    descriptor: MemoryMappingDescriptor,
    seen: AHashSet<Group>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        return Self {
            event_systems: AHashMap::new(),

            join_systems: AHashMap::new(),
            quit_systems: AHashMap::new(),
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

    pub fn add_event_system(&mut self, event: EventID, system: Rc<RefCell<dyn System>>) {
        if !self.event_systems.contains_key(&event) {
            self.event_systems.insert(event, Vec::new());
        }

        if !self.seen.contains(&system.borrow().group()) {
            self.descriptor.push(system.borrow().components());
            self.seen.insert(system.borrow().group());
        }

        self.event_systems.get_mut(&event).unwrap().push(system);
    }

    pub fn add_join_system(&mut self, system: Rc<RefCell<dyn System>>) {
        if !self.seen.contains(&system.borrow().group()) {
            self.descriptor.push(system.borrow().components());
            self.seen.insert(system.borrow().group());
        }

        let group = system.borrow().group();

        if !self.join_systems.contains_key(&group) {
            self.join_systems.insert(group, Vec::new());
        }

        self.join_systems.get_mut(&group).unwrap().push(system);
    }

    pub fn add_quit_system(&mut self, system: Rc<RefCell<dyn System>>) {
        if !self.seen.contains(&system.borrow().group()) {
            self.descriptor.push(system.borrow().components());
            self.seen.insert(system.borrow().group());
        }

        let group = system.borrow().group();

        if !self.quit_systems.contains_key(&group) {
            self.quit_systems.insert(group, Vec::new());
        }

        self.quit_systems.get_mut(&group).unwrap().push(system);
    }

    pub fn add_tick_system(&mut self, system: Rc<RefCell<dyn System>>) {
        if !self.seen.contains(&system.borrow().group()) {
            self.descriptor.push(system.borrow().components());
            self.seen.insert(system.borrow().group());
        }

        self.tick_systems.push(system);
    }
}