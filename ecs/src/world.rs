use std::collections::{HashMap, HashSet};

use crate::{
    entity::Entity,
    component::{
        ComponentTrait,
        AnyComponentPool,
        ComponentPool,
    }
};

pub struct World {
    entities: HashMap<Entity, Vec<u64>>,
    pools: HashMap<u64, Box<dyn AnyComponentPool>>,

    next: u64
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            pools: HashMap::new(),
            next: 0
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

    pub fn register_pool_or_retrieve<T: ComponentTrait + 'static>(&mut self) -> Option<&mut ComponentPool<T>> {
        if !self.pools.contains_key(&T::id()) {
            let pool: ComponentPool<T> = ComponentPool::new();

            self.pools.insert(T::id(), Box::new(pool));
        }

        return self.pools.get_mut(&T::id()).unwrap().as_any().downcast_mut::<ComponentPool<T>>();
    }
}