use std::collections::HashSet;

use crate::{
    application::Application,
    core::system::System,
};

pub struct AppBuilder {
    systems: Vec<Box<dyn System>>,

    seen: HashSet<String>,
}

impl AppBuilder {
    pub fn new() -> Self {
        return Self {
            systems: Vec::new(),
            seen: HashSet::new(),
        };
    }

    pub fn add<T: System + 'static>(&mut self, value: T) {
        let name = std::any::type_name::<T>();

        if !self.seen.contains(name) {
            self.systems.push(Box::new(value));
            self.seen.insert(name.to_string());
        } else {
            log::warn!("System {} has already been inserted into the AppBuilder.", name);
        }
    }

    pub fn build(self) -> Application {
        return Application::new(self.systems);
    }
}