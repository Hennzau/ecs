use crate::{
    core::{
        component::{
            ComponentID,
            AnyComponent,
        },
        entity::Entity,
    },

    application::Application,
};

pub struct Bundle<'a> {
    entity: Entity,

    components_to_add: Vec<Box<dyn AnyComponent>>,
    components_to_remove: Vec<ComponentID>,

    application: &'a mut Application,
}

impl Bundle<'_> {
    pub fn new(entity: Entity, application: &mut Application) -> Bundle {
        return Bundle {
            entity: entity,
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
            application: application,
        };
    }

    pub fn add_component<T: AnyComponent + 'static>(mut self, component: T) -> Self {
        self.components_to_add.push(Box::new(component));

        return self;
    }

    pub fn remove_component<T: AnyComponent + 'static>(mut self) -> Self {
        self.components_to_remove.push(T::component_id());

        return self;
    }

    pub fn try_build(self) -> Result<(), ()> {
        let mut result = Ok(());

        for component in self.components_to_add {
            let id = component.id();

            let res = self.application.try_add_any_component(&self.entity, id, component);
            if res.is_err() {
                result = Err(());
            }
        }

        for component in self.components_to_remove {
            let res = self.application.try_remove_any_component(&self.entity, component);
            if res.is_err() {
                result = Err(());
            }
        }

        return result;
    }
}