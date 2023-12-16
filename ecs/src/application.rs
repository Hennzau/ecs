use std::collections::{HashMap, HashSet};

pub mod entity;
pub mod component;
pub mod system;

use entity::Entity;
use component::{
    AnyComponentPool,
    ComponentPool,
    ComponentTrait,
};

use system::System;

use crate::memory::storage::MappedStorage;

pub struct Application {
    entities: HashMap<Entity, HashSet<u64>>,
    next: u64,

    pools: HashMap<u64, Box<dyn AnyComponentPool>>,
    pub storage: MappedStorage,

    systems: HashMap<u128, Vec<Box<dyn System>>>
}

impl Application {
    pub fn new(systems: Vec<Box<dyn System>>) -> Self {
        let mut descriptor = Vec::<Vec<u64>>::new();

        for system in &systems {
            descriptor.push(system.components());
        }

        let mut mapped_systems = HashMap::<u128, Vec<Box<dyn System>>>::new();

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

    fn associated(&self, entity: &Entity, ids: &Vec<u64>) -> bool {
        if !self.alive(entity) {
            return false;
        }


        let components = self.entities.get(entity).unwrap();
        return ids.iter().all(|x| components.contains(x));
    }

    fn register_pool_or_retrieve<T: ComponentTrait + 'static>(&mut self) -> &mut ComponentPool<T> {
        if !self.pools.contains_key(&T::id()) {
            let pool: ComponentPool<T> = ComponentPool::new();

            self.pools.insert(T::id(), Box::new(pool));
        }

        return self.pools.get_mut(&T::id()).unwrap().as_any().downcast_mut::<ComponentPool<T>>().unwrap();
    }

    fn try_group_id(&self, entity: &Entity) -> Option<u128> {
        match self.entities.get(entity) {
            Some(components) => {
                let mut result: u128 = 0;
                for component_id in components {
                    result += (*component_id) as u128;
                }
                if result > 0 {
                    Some(result)
                } else {
                    None
                }
            }
            None => None
        }
    }

    fn try_retrieve_pool(&mut self, id: u64) -> Option<&mut Box<dyn AnyComponentPool>> {
        self.pools.get_mut(&id)
    }

    pub fn try_add_component<T: ComponentTrait + 'static>(&mut self, entity: &Entity, value: T) -> Option<&mut T> {
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

    pub fn try_get_component<T: ComponentTrait + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        if !self.pools.contains_key(&T::id()) {
            return None;
        }

        let pool = self.register_pool_or_retrieve::<T>();

        return pool.try_get_component(entity);
    }

    pub fn try_remove_component(&mut self, entity: &Entity, id: u64) -> Option<Box<dyn ComponentTrait>> {
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
