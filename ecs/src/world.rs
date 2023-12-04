use std::collections::{HashMap, HashSet};

use crate::{
    entity::Entity,
    component::{
        ComponentTrait,
        AnyComponentPool,
        ComponentPool,
    },
};

pub struct World {
    entities: HashMap<Entity, Vec<u128>>,
    pools: HashMap<u64, Box<dyn AnyComponentPool>>,

    next: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            pools: HashMap::new(),
            next: 0,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.insert(self.next as Entity, Vec::new());

        self.next += 1;
        self.next - 1
    }

    pub fn alive(&self, entity: &Entity) -> bool {
        self.entities.contains_key(entity)
    }

    pub fn contains(&self, entity: &Entity, id: u128) -> bool {
        if !self.alive(entity) {
            return false;
        }

        let components = self.entities.get(entity).unwrap();

        return components.contains(&id);
    }

    pub fn group_id(&self, entity: &Entity) -> Option<u128> {
        match self.entities.get(entity) {
            Some(components) => {
                let mut result = 0;
                for component_id in components {
                    result += *component_id;
                }
                Some(result)
            }
            None => None
        }
    }

    fn register_pool_or_retrieve<T: ComponentTrait + 'static>(&mut self) -> Option<&mut ComponentPool<T>> {
        if !self.pools.contains_key(&T::id()) {
            let pool: ComponentPool<T> = ComponentPool::new();

            self.pools.insert(T::id(), Box::new(pool));
        }

        return self.pools.get_mut(&T::id()).unwrap().as_any().downcast_mut::<ComponentPool<T>>();
    }

    pub fn add_component<T: ComponentTrait + 'static>(&mut self, entity: &Entity, value: T) -> Option<&mut T> {
        if !self.alive(entity) {
            return None;
        }

        let id = T::id() as u128;

        if !self.contains(entity, id) {
            let components = self.entities.get_mut(entity).unwrap();
            components.push(id);
        }

        let pool = self.register_pool_or_retrieve::<T>().unwrap();

        return Some(pool.add_component_or_retrieve(entity, value));
    }

    pub fn get_component<T: ComponentTrait + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        None
    }
}