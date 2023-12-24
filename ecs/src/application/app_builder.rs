use crate::{
    application::Application,
    core::system::System,
};

pub struct AppBuilder {
    systems: Vec<Box<dyn System>>,
}

impl AppBuilder {
    pub fn new() -> Self {
        return Self {
            systems: Vec::new()
        };
    }

    pub fn add<T: System + 'static>(&mut self, value: T) {
        self.systems.push(Box::new(value));
    }

    pub fn build(self) -> Application {
        return Application::new(self.systems);
    }
}