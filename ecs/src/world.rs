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
    entities: HashMap<Entity, HashSet<u64>>,
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
        self.entities.insert(self.next as Entity, HashSet::new());

        self.next += 1;
        self.next - 1
    }

    pub fn alive(&self, entity: &Entity) -> bool {
        self.entities.contains_key(entity)
    }

    pub fn associated(&self, entity: &Entity, id: u64) -> bool {
        if !self.alive(entity) {
            return false;
        }

        let components = self.entities.get(entity).unwrap();

        return components.contains(&id);
    }

    pub fn try_group_id(&self, entity: &Entity) -> Option<u128> {
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

    fn try_retrieve_pool_by_id(&mut self, id: u64) -> Option<&mut Box<dyn AnyComponentPool>> {
        self.pools.get_mut(&id)
    }

    fn register_pool_or_retrieve<T: ComponentTrait + 'static>(&mut self) -> &mut ComponentPool<T> {
        if !self.pools.contains_key(&T::id()) {
            let pool: ComponentPool<T> = ComponentPool::new();

            self.pools.insert(T::id(), Box::new(pool));
        }

        return self.pools.get_mut(&T::id()).unwrap().as_any().downcast_mut::<ComponentPool<T>>().unwrap();
    }

    pub fn try_add_component<T: ComponentTrait + 'static>(&mut self, entity: &Entity, value: T) -> Option<&mut T> {
        if !self.alive(entity) {
            return None;
        }

        let id = T::id();

        if !self.associated(entity, id) {
            let components = self.entities.get_mut(entity).unwrap();
            components.insert(id);
        }

        let pool = self.register_pool_or_retrieve::<T>();

        return Some(pool.add_or_get_component(entity, value));
    }

    pub fn remove_component<T: ComponentTrait + 'static>(&mut self, entity: &Entity) {
        if self.alive(entity) {
            let id = T::id();

            if self.associated(entity, id) {
                let pool = self.register_pool_or_retrieve::<T>();
                pool.remove_component(entity);

                let components = self.entities.get_mut(entity).unwrap();
                components.remove(&id);
            }
        }
    }

    pub fn remove_component_by_id(&mut self, entity: &Entity, id: u64) {
        if self.alive(entity) {
            if self.associated(entity, id) {
                let pool = self.try_retrieve_pool_by_id(id).unwrap();
                pool.remove_component(entity);

                let components = self.entities.get_mut(entity).unwrap();
                components.remove(&id);
            }
        }
    }

    pub fn try_get_component<T: ComponentTrait + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        if !self.pools.contains_key(&T::id()) {
            return None;
        }

        let pool = self.register_pool_or_retrieve::<T>();

        return pool.try_get_component(entity);
    }
}