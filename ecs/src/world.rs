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
    entities: HashSet<Entity>,
    pools: HashMap<u64, Box<dyn AnyComponentPool>>,

    next: u64
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
            pools: HashMap::new(),
            next: 0
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.insert(self.next as Entity);

        self.next += 1;
        self.next - 1
    }

    pub fn alive(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }

    pub fn register_pool<T: ComponentTrait + 'static>(&mut self) -> Option<&mut ComponentPool<T>> {
        if !self.pools.contains_key(&T::id()) {
            let pool: ComponentPool<T> = ComponentPool::new();

            self.pools.insert(T::id(), Box::new(pool));
        }

        return self.pools.get_mut(&T::id()).unwrap().as_any().downcast_mut::<ComponentPool<T>>();
    }
}