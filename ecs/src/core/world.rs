/// World represent an instance of an application that can be used by a system to access data
/// or to modify it.

use crate::{
    core::{
        component::{
            ComponentID,
            AnyComponent
        },
        entity::Entity,
    },

    memory::components::Components
};

pub struct World<'a> {
    components: &'a mut Components,
}

impl World<'_> {
    pub fn new(components: &mut Components) -> World<'_> {
        return World {
            components,
        };
    }

    pub fn try_get_any_component(&self, entity: &Entity, id: ComponentID) -> Option<&Box<dyn AnyComponent>> {
        return self.components.try_get_any_component(entity, id);
    }

    pub fn try_get_any_mut_component(&mut self, entity: &Entity, id: ComponentID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.components.try_get_any_mut_component(entity, id);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return self.components.try_get_component::<T>(entity);
    }

    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return self.components.try_get_mut_component::<T>(entity);
    }
}