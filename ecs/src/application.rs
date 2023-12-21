pub mod entity;
pub mod component;
pub mod system;

mod storage;

use std::collections::{
    HashMap,
    HashSet
};

use crate::{
    application::{
        storage::MappedStorage,
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

    pub fn entities(&self) -> &Vec<Vec<Entity>> {
        self.storage.entities()
    }
}

/*
    Components
*/

impl Application {
    fn associated(&self, entity: &Entity, ids: &Vec<Component>) -> bool {
        if !self.alive(entity) {
            return false;
        }

        let components = self.entities.get(entity).unwrap();
        return ids.iter().all(|x| components.contains(x));
    }

    fn add_get_or_get_pool<T: AnyComponent + 'static>(&mut self) -> &mut ComponentPool<T> {
        if !self.pools.contains_key(&T::id()) {
            let pool: ComponentPool<T> = ComponentPool::new();

            self.pools.insert(T::id(), Box::new(pool));
        }

        return self.pools.get_mut(&T::id()).unwrap().as_any().downcast_mut::<ComponentPool<T>>().unwrap();
    }

    fn try_get_pool(&mut self, id: Component) -> Option<&mut Box<dyn AnyComponentPool>> {
        self.pools.get_mut(&id)
    }

    pub fn try_add_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Option<&mut T> {
        if !self.alive(entity) {
            return None;
        }

        let id = T::id();

        let mut groups = HashSet::<Group>::new();
        if !self.associated(entity, &vec![id]) {
            let components = self.entities.get_mut(entity).unwrap();
            groups = self.storage.process_add(entity, components, id);

            components.insert(id);
        }

        // You must call 'on_startup' methods, after creating components because those systems may need to get the components

        let pool = self.add_get_or_get_pool::<T>();
        pool.add_get_or_get(entity, value);

        self.on_startup(&groups, entity);

        let pool = self.add_get_or_get_pool::<T>();

        return pool.try_get(entity);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        if !self.pools.contains_key(&T::id()) {
            return None;
        }

        let pool = self.add_get_or_get_pool::<T>();

        return pool.try_get(entity);
    }

    pub fn try_remove_get_component(&mut self, entity: &Entity, id: Component) -> Option<Box<dyn AnyComponent>> {
        if self.alive(entity) {
            if self.associated(entity, &vec![id]) {
                let components = self.entities.get(entity).unwrap();
                let groups = self.storage.process_remove(entity, components, id);

                // You need to call 'on_quit' methods before removing components because systems may need those

                self.on_quit(&groups, entity);

                let components = self.entities.get_mut(entity).unwrap();
                components.remove(&id);

                let pool = self.try_get_pool(id).unwrap();
                return pool.try_remove_get(entity);
            }
        }

        return None
    }

    pub fn try_remove_component(&mut self, entity: &Entity, id: Component) {
        if self.alive(entity) {
            if self.associated(entity, &vec![id]) {
                let components = self.entities.get(entity).unwrap();
                let groups = self.storage.process_remove(entity, components, id);

                // You need to call 'on_quit' methods before removing components because systems may need those

                self.on_quit(&groups, entity);

                let components = self.entities.get_mut(entity).unwrap();
                components.remove(&id);

                let pool = self.try_get_pool(id).unwrap();
                pool.try_remove(entity);
            }
        }
    }
}

/*
    Systems
*/

impl Application {
    pub fn on_startup(&mut self, groups: &HashSet<Group>, entity: &Entity) {
        for group in groups {
            for system in self.systems.get_mut(group).unwrap() {
                system.on_startup(&[entity.clone()]);
            }
        }
    }

    pub fn on_quit(&mut self, groups: &HashSet<Group>, entity: &Entity) {
        for group in groups {
            for system in self.systems.get_mut(group).unwrap() {
                system.on_quit(&[entity.clone()]);
            }
        }
    }
}