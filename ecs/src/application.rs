pub mod entity;
pub mod component;
pub mod system;

mod storage;

use std::collections::{HashMap, HashSet};
use storage::MappedStorage;

use crate::{
    application::{
        component::{
            AnyComponent,
            Component,
            Group,
            pool::{
                AnyComponentPool,
                ComponentPool
            }
        },
        entity::Entity,
        system::System
    }
};
use crate::application::component::set_to_group;

pub struct Application {
    entities: HashMap<Entity, HashSet<u64>>,
    next: u64,

    pools: HashMap<Component, Box<dyn AnyComponentPool>>,
    storage: MappedStorage,
    systems: HashMap<Group, Vec<Box<dyn System>>>
}

/*
    Entities
*/

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

        Self {
            entities: HashMap::new(),
            next: 0,
            pools: HashMap::new(),
            storage: MappedStorage::new(descriptor),
            systems: mapped_systems
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.insert(self.next as Entity, HashSet::new());

        self.next += 1;
        self.next - 1
    }

    fn alive(&self, entity: &Entity) -> bool { self.entities.contains_key(entity) }

    pub fn view(&self, group: u128) -> &[Entity] {
        self.storage.view(group)
    }
}

/*
    Components
*/

impl Application {
    fn try_group_id(&self, entity: &Entity) -> Option<u128> {
        match self.entities.get(entity) {
            Some(components) => Some(set_to_group(components)),
            None => None
        }
    }

    fn associated(&self, entity: &Entity, ids: &Vec<Component>) -> bool {
        if !self.alive(entity) {
            return false;
        }


        let components = self.entities.get(entity).unwrap();
        return ids.iter().all(|x| components.contains(x));
    }

    fn register_pool_or_retrieve<T: AnyComponent + 'static>(&mut self) -> &mut ComponentPool<T> {
        if !self.pools.contains_key(&T::id()) {
            let pool: ComponentPool<T> = ComponentPool::new();

            self.pools.insert(T::id(), Box::new(pool));
        }

        return self.pools.get_mut(&T::id()).unwrap().as_any().downcast_mut::<ComponentPool<T>>().unwrap();
    }

    fn try_retrieve_pool(&mut self, id: Component) -> Option<&mut Box<dyn AnyComponentPool>> {
        self.pools.get_mut(&id)
    }

    pub fn try_add_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Option<&mut T> {
        if !self.alive(entity) {
            return None;
        }

        let id = T::id();

        if !self.associated(entity, &vec![id]) {
            let components = self.entities.get_mut(entity).unwrap();
            self.storage.add_entity(entity, components, id);

            components.insert(id);
        }

        let pool = self.register_pool_or_retrieve::<T>();

        return Some(pool.add_or_get_component(entity, value));
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        if !self.pools.contains_key(&T::id()) {
            return None;
        }

        let pool = self.register_pool_or_retrieve::<T>();

        return pool.try_get_component(entity);
    }

    pub fn try_remove_component(&mut self, entity: &Entity, id: Component) -> Option<Box<dyn AnyComponent>> {
        if self.alive(entity) {
            if self.associated(entity, &vec![id]) {
                let components = self.entities.get_mut(entity).unwrap();
                components.remove(&id);

                let pool = self.try_retrieve_pool(id).unwrap();
                return pool.try_remove_component(entity);
            }
        }

        return None
    }
}

impl Application {
    pub fn debug_storage(&self) -> &Vec<Vec<Entity>> {
        return &self.storage.entities
    }

    pub fn debug_mapping(&self) -> &HashMap<u128, (usize, usize)> {
        return &self.storage.mapping.mapping
    }
}