use ahash::AHashMap;

use crate::ecs::{
    core::component::{
        ComponentID,
        Component,
        AnyComponent,
        ComponentIndex,
        ArchetypeIndex,
    },
    memory::pool::{
        Pool,
        PoolIndex,
        AnyPool
    }
};

pub struct Components {
    pools: Vec<Box<dyn AnyPool>>,
    map: AHashMap<ComponentID, PoolIndex>
}

impl Components {
    pub fn new() -> Self {
        return Components {
            pools: Vec::new(),
            map: AHashMap::new()
        };
    }
}

// Manage the pools

impl Components {
    fn generate_or_retrieve<T: AnyComponent + 'static>(&mut self) -> &mut Pool<T> {
        if let Some(index) = self.map.get(&T::type_id()) {
            return self.pools.get_mut(*index).unwrap().as_any_mut().downcast_mut::<Pool<T>>().unwrap();
        }

        let pool = Box::new(Pool::<T>::new()) as Box<dyn AnyPool>;

        let index = self.pools.len();

        self.pools.push(pool);
        self.map.insert(T::type_id(), index);

        return self.pools.get_mut(index).unwrap().as_any_mut().downcast_mut::<Pool<T>>().unwrap();
    }

    fn retrieve<T: AnyComponent + 'static>(&self) -> Option<&Pool<T>> {
        if let Some(index) = self.map.get(&T::type_id()) {
            return self.pools.get(*index).unwrap().as_any().downcast_ref::<Pool<T>>();
        }

        return None;
    }

    fn retrieve_mut<T: AnyComponent + 'static>(&mut self) -> Option<&mut Pool<T>> {
        if let Some(index) = self.map.get(&T::type_id()) {
            return self.pools.get_mut(*index).unwrap().as_any_mut().downcast_mut::<Pool<T>>();
        }

        return None;
    }

    fn retrieve_any(&self, id: ComponentID) -> Option<&Box<dyn AnyPool>> {
        if let Some(index) = self.map.get(&id) {
            return Some(self.pools.get(*index).unwrap());
        }

        return None;
    }

    fn retrieve_any_mut(&mut self, id: ComponentID) -> Option<&mut Box<dyn AnyPool>> {
        if let Some(index) = self.map.get(&id) {
            return Some(self.pools.get_mut(*index).unwrap());
        }

        return None;
    }
}