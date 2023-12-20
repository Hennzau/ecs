use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    core::component::{
        Component,
        AnyComponent,
    },
    memory::pool::{
        ComponentPool,
        AnyComponentPool,
    },
};
use crate::core::entity::Entity;

struct Factory {
    pools: HashMap<Component, Box<dyn AnyComponentPool>>,
}

impl Factory {
    fn try_get_pool_any_mut(&mut self, id: Component) -> Option<&mut Box<dyn AnyComponentPool>> {
        return self.pools.get_mut(&id);
    }

    fn try_get_pool_any(&self, id: Component) -> Option<&Box<dyn AnyComponentPool>> {
        return self.pools.get(&id);
    }

    fn try_get_pool_mut<T: AnyComponent + 'static>(&mut self) -> Option<&mut ComponentPool<T>> {
        let id = T::id();

        return match self.try_get_pool_any_mut(id) {
            Some(any_pool) => any_pool.as_any_mut().downcast_mut::<ComponentPool<T>>(),
            None => None
        };
    }

    fn try_get_pool<T: AnyComponent + 'static>(&self) -> Option<&ComponentPool<T>> {
        let id = T::id();

        return match self.try_get_pool_any(id) {
            Some(any_pool) => any_pool.as_any().downcast_ref::<ComponentPool<T>>(),
            None => None
        };
    }

    fn add_get_or_get_pool<T: AnyComponent + 'static>(&mut self) -> &mut ComponentPool<T> {
        let id = T::id();

        if !self.pools.contains_key(&id) {
            let pool = ComponentPool::<T>::new();

            self.pools.insert(id, Box::new(pool));
        }

        return self.try_get_pool_mut::<T>().unwrap();
    }

    pub fn add_get_or_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> &mut T {
        let pool = self.add_get_or_get_pool::<T>();

        return pool.add_get_or_get(entity, value);
    }

    /// Try to add T to entity, return true if this entity had not T before
    pub fn try_add_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> bool {
        let pool = self.add_get_or_get_pool::<T>();

        return pool.try_add(entity, value);
    }

    pub fn try_get_component_mut<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return match self.try_get_pool_mut::<T>() {
            Some(pool) => pool.try_get_mut(entity),
            None => None
        };
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return match self.try_get_pool::<T>() {
            Some(pool) => pool.try_get(entity),
            None => None
        };
    }

    pub fn try_remove_get_component_any(&mut self, entity: &Entity, id: Component) -> Option<Box<dyn AnyComponent>> {
        return match self.try_get_pool_any_mut(id) {
            Some(pool) => pool.try_remove_get_any(entity),
            None => None
        };
    }

    pub fn try_remove_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<Box<T>> {
        return match self.try_get_pool_mut::<T>() {
            Some(pool) => pool.try_remove_get(entity),
            None => None
        };
    }

    /// Try to remove component, and return False if entity had not this component
    pub fn try_remove_component_any(&mut self, entity: &Entity, id: Component) -> bool {
        return match self.try_get_pool_any_mut(id) {
            Some(pool) => pool.try_remove(entity),
            None => false
        };
    }

    pub fn try_remove_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> bool {
        let id = T::id();

        return match self.try_get_pool_any_mut(id) {
            Some(pool) => pool.try_remove(entity),
            None => false
        };
    }
}