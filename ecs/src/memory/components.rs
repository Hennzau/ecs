use std::collections::HashMap;

use crate::core::{
    entity::Entity,
    component::{
        ComponentID,
        AnyComponent
    }
};

pub struct Components {
    components: Vec<Vec<Box<dyn AnyComponent>>>,

    indices: Vec<HashMap<Entity, usize>>,
    map: HashMap<ComponentID, usize>
}

impl Components {
    pub fn new() -> Self {
        return Self {
            components: Vec::new(),
            indices: Vec::new(),
            map: HashMap::new()
        }
    }

    fn convert<T: AnyComponent + 'static>(component: &Box<dyn AnyComponent>) -> Option<&T> {
        return component.as_any().downcast_ref::<T>();
    }

    fn convert_mut<T: AnyComponent + 'static>(component: &mut Box<dyn AnyComponent>) -> Option<&mut T> {
        return component.as_any_mut().downcast_mut::<T>();
    }

    fn contains(&self, entity: &Entity, id: ComponentID) -> bool {
        return match self.map.get(&id) {
            Some(index) => match self.indices.get(index.clone()) {
                Some(indices) => indices.contains_key(entity),
                None => false
            },
            None => false
        }
    }

    pub fn try_add_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Option<&mut T> {
        let id = T::id();

        if self.contains(entity, id) {
            return None;
        }

        if let Some(index) = self.map.get(&id).cloned() {
            if let Some(components) = self.components.get_mut(index) {
                if let Some(indices) = self.indices.get_mut(index) {
                    let in_index = components.len();
                    indices.insert(entity.clone(), in_index);
                    components.push(Box::new(value));

                    if let Some(last) = components.last_mut() {
                        return Self::convert_mut(last);
                    }
                }
            }
        }

        return None;
    }
}