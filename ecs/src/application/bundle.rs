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
            let res = self.application.try_add_any_component(self.entity, component);
            if res.is_err() {
                result = Err(());
            }
        }

        for component in self.components_to_remove {
            let res = self.application.try_remove_any_component(self.entity, component);
            if res.is_err() {
                result = Err(());
            }
        }

        return result;
    }
}

pub struct BatchBundle<'a> {
    batch: (Entity, usize),

    components_to_add: Vec<Vec<Box<dyn AnyComponent>>>,
    components_to_remove: Vec<ComponentID>,

    application: &'a mut Application,
}

impl BatchBundle<'_> {
    pub fn new(batch: (Entity, usize), application: &mut Application) -> BatchBundle {
        return BatchBundle {
            batch: batch,
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
            application: application,
        };
    }

    pub fn add_component<T: AnyComponent + 'static>(mut self, components: Vec<T>) -> Self {
        let mut box_components = Vec::<Box<dyn AnyComponent>>::new();

        for component in components {
            box_components.push(Box::new(component));
        }

        self.components_to_add.push(box_components);

        return self;
    }

    pub fn add_component_clone<T: Clone + AnyComponent + 'static>(mut self, component: T) -> Self {
        let mut box_components = Vec::<Box<dyn AnyComponent>>::new();

        for _ in 0..self.batch.1 {
            box_components.push(Box::new(component.clone()));
        }

        self.components_to_add.push(box_components);

        return self;
    }

    pub fn remove_component<T: AnyComponent + 'static>(mut self) -> Self {
        self.components_to_remove.push(T::component_id());

        return self;
    }

    pub fn try_build(self) -> Result<(), ()> {
        let mut result = Ok(());

        for components in self.components_to_add {
            let res = self.application.try_add_any_component_batch(self.batch, components);
            if res.is_err() {
                result = Err(());
            }
        }

        for component in self.components_to_remove {
            let res = self.application.try_remove_any_component_batch(self.batch, component);
            if res.is_err() {
                result = Err(());
            }
        }

        return result;
    }
}

pub struct MultipleBundle<'a> {
    entities: Vec<Entity>,

    components_to_add: Vec<Vec<Box<dyn AnyComponent>>>,
    components_to_remove: Vec<ComponentID>,

    application: &'a mut Application,
}

impl MultipleBundle<'_> {
    pub fn new(entities: Vec<Entity>, application: &mut Application) -> MultipleBundle {
        return MultipleBundle {
            entities: entities,
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
            application: application,
        };
    }

    pub fn add_component<T: AnyComponent + 'static>(mut self, components: Vec<T>) -> Self {
        let mut box_components = Vec::<Box<dyn AnyComponent>>::new();

        for component in components {
            box_components.push(Box::new(component));
        }

        self.components_to_add.push(box_components);

        return self;
    }

    pub fn add_component_clone<T: Clone + AnyComponent + 'static>(mut self, component: T) -> Self {
        let mut box_components = Vec::<Box<dyn AnyComponent>>::new();

        for _ in 0..self.entities.len() {
            box_components.push(Box::new(component.clone()));
        }

        self.components_to_add.push(box_components);

        return self;
    }

    pub fn remove_component<T: AnyComponent + 'static>(mut self) -> Self {
        self.components_to_remove.push(T::component_id());

        return self;
    }

    pub fn try_build(self) -> Result<(), ()> {
        let mut result = Ok(());

        for components in self.components_to_add {
            for (&entity, component) in self.entities.iter().zip(components) {
                let res = self.application.try_add_any_component(entity, component);
                if res.is_err() {
                    result = Err(());
                }
            }
        }

        for component in self.components_to_remove {
            for &entity in &self.entities {
                let res = self.application.try_remove_any_component(entity, component);
                if res.is_err() {
                    result = Err(());
                }
            }
        }

        return result;
    }
}