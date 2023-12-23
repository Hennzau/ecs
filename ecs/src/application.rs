use std::collections::{HashMap, HashSet};

use crate::{
    core::{
        component::{
            AnyComponent,
            Component,
            Group,
        },
        entity::Entity,
        system::System,
    },
    memory::storage::Storage,
};
use crate::core::sub_app::SubApp;

pub mod prelude;

pub struct Application {
    storage: Storage,
    next: u64,

    systems: HashMap<Group, Vec<Box<dyn System>>>,
}

/// Implementation of debug functions
impl Application {
    pub fn entities(&self) -> &Vec<Vec<Entity>> {
        return self.storage.entities();
    }

    pub fn view(&self, group: Group) -> &[Entity] {
        return self.storage.view(group);
    }
}

/// Implementation of general functions
impl Application {
    pub fn new(systems: Vec<Box<dyn System>>) -> Self {
        let mut descriptor = Vec::<Vec<Component>>::new();

        for system in &systems {
            descriptor.push(system.components());
        }

        let mut mapped_systems = HashMap::<Group, Vec<Box<dyn System>>>::new();

        for system in systems {
            if !mapped_systems.contains_key(&system.id()) {
                mapped_systems.insert(system.id(), Vec::new());
            }

            mapped_systems.get_mut(&system.id()).unwrap().push(system);
        }

        return Self {
            storage: Storage::new(descriptor),
            next: 0,
            systems: mapped_systems,
        };
    }

    pub fn spawn(&mut self) -> Entity {
        self.storage.push_entity(self.next);
        self.next += 1;

        return self.next - 1;
    }

    pub fn add_get_or_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> &mut T {
        let (component, groups) = self.storage.add_get_or_get_component(entity, value);

        return component;
    }

    pub fn try_add_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> bool {
        let (result, groups) = self.storage.try_add_component(entity, value);

        return result;
    }

    pub fn try_remove_get_component_any(&mut self, entity: &Entity, id: Component) -> Option<Box<dyn AnyComponent>> {
        let (component, groups) = self.storage.try_remove_get_component_any(entity, id);

        return component;
    }

    pub fn try_remove_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<Box<T>> {
        let (component, groups) = self.storage.try_remove_get_component::<T>(entity);

        return component;
    }

    pub fn try_remove_component_any(&mut self, entity: &Entity, id: Component) -> bool {
        let (result, groups) = self.storage.try_remove_component_any(entity, id);

        return result;
    }

    pub fn try_remove_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> bool {
        let (result, groups) = self.storage.try_remove_component::<T>(entity);

        return result;
    }

    pub fn try_get_component_mut<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return self.storage.try_get_component_mut::<T>(entity);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return self.storage.try_get_component::<T>(entity);
    }
}

/// Implementation of systems functions
impl Application {
    fn on_startup(&mut self, groups: &HashSet<Group>, entities: &[Entity]) {
        let mut sub_app = SubApp::new(&mut self.storage);


    }
}
