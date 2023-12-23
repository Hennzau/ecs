use crate::{
    memory::storage::Storage,
    core::{
        entity::Entity,
        component::{
            Component,
            AnyComponent,
            Group,
        },
    },
};

pub struct SubApp<'a> {
    storage: &'a mut Storage,
}

impl SubApp<'_> {
    pub fn new(storage: &mut Storage) -> SubApp<'_> {
        return SubApp {
            storage: storage
        };
    }

    pub fn view(&self, group: Group) -> &[Entity] {
        return self.storage.view(group);
    }

    pub fn try_get_component_mut<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return self.storage.try_get_component_mut::<T>(entity);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return self.storage.try_get_component::<T>(entity);
    }
}