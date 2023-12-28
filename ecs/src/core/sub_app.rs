use crate::{
    memory::factory::Factory,
    core::{
        entity::Entity,
        component::AnyComponent,
    },
};

pub struct SubApp<'a> {
    factory: &'a mut Factory,
}

impl SubApp<'_> {
    pub fn new(factory: &mut Factory) -> SubApp<'_> {
        return SubApp {
            factory: factory
        };
    }

    pub fn try_get_component_mut<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return self.factory.try_get_component_mut::<T>(entity);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return self.factory.try_get_component::<T>(entity);
    }
}