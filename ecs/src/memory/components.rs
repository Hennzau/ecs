use std::collections::HashMap;

use crate::core::{
    entity::Entity,
    component::{
        ComponentID,
        AnyComponent,
    },
};

pub struct Components {
    components: Vec<Vec<Box<dyn AnyComponent>>>,

    indices: Vec<HashMap<Entity, usize>>,
    map: HashMap<ComponentID, usize>,
}

impl Components {
    pub fn new() -> Self {
        return Self {
            components: Vec::new(),
            indices: Vec::new(),
            map: HashMap::new(),
        };
    }

    pub fn convert<T: AnyComponent + 'static>(component: &Box<dyn AnyComponent>) -> Option<&T> {
        return component.as_any().downcast_ref::<T>();
    }

    pub fn convert_mut<T: AnyComponent + 'static>(component: &mut Box<dyn AnyComponent>) -> Option<&mut T> {
        return component.as_any_mut().downcast_mut::<T>();
    }

    fn convert_ok<T: AnyComponent + 'static>(component: Option<&Box<dyn AnyComponent>>) -> Option<&T> {
        return component.and_then(|component| component.as_any().downcast_ref::<T>());
    }

    fn convert_mut_ok<T: AnyComponent + 'static>(component: Option<&mut Box<dyn AnyComponent>>) -> Option<&mut T> {
        return component.and_then(|component| component.as_any_mut().downcast_mut::<T>());
    }

    fn contains(&self, entity: &Entity, id: ComponentID) -> bool {
        return match self.map.get(&id) {
            Some(index) => match self.indices.get(index.clone()) {
                Some(indices) => indices.contains_key(entity),
                None => false
            },
            None => false
        };
    }

    pub fn try_add_any_component(&mut self, entity: &Entity, id: ComponentID, value: Box<dyn AnyComponent>) -> Result<(), ()> {
        if self.contains(entity, id) {
            return Err(());
        }

        if let Some(index) = self.map.get(&id).cloned() {
            if let (Some(components), Some(indices)) = (self.components.get_mut(index), self.indices.get_mut(index)) {
                let in_index = components.len();
                indices.insert(entity.clone(), in_index);
                components.push(value);

                return Ok(());
            }
        } else {
            let index = self.components.len();
            self.components.push(vec![value]);
            self.indices.push(HashMap::from([(entity.clone(), 0)]));
            self.map.insert(id, index);

            return Ok(());
        }

        return Err(());
    }

    pub fn try_remove_any_component(&mut self, entity: &Entity, id: ComponentID) -> Result<Box<dyn AnyComponent>, ()> {
        if !self.contains(entity, id) {
            return Err(());
        }

        if let Some(index) = self.map.get(&id).cloned() {
            if let (Some(components), Some(indices)) = (self.components.get_mut(index), self.indices.get_mut(index)) {
                let last_in_index = components.len() - 1;
                let last = indices.iter().find_map(|(key, value)| if value.clone() == last_in_index { Some(key) } else { None });
                if let Some(last_entity) = last.cloned() {
                    if let Some(in_index) = indices.get(entity).cloned() {
                        indices.insert(last_entity, in_index);

                        return Ok(components.swap_remove(in_index));
                    }

                    indices.remove(entity);
                }
            }
        }

        return Err(());
    }

    pub fn try_get_any_component(&self, entity: &Entity, id: ComponentID) -> Option<&Box<dyn AnyComponent>> {
        return self.map.get(&id).cloned().and_then(
            |index| self.components.get(index).and_then(
                |components| self.indices.get(index).and_then(
                    |indices| indices.get(entity).cloned().and_then(
                        |in_index| components.get(in_index)))));
    }

    pub fn try_get_any_mut_component(&mut self, entity: &Entity, id: ComponentID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.map.get(&id).cloned().and_then(
            |index| self.components.get_mut(index).and_then(
                |components| self.indices.get(index).and_then(
                    |indices| indices.get(entity).cloned().and_then(
                        |in_index| components.get_mut(in_index)))));
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return Self::convert_ok(self.try_get_any_component(entity, T::id()));
    }

    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return Self::convert_mut_ok(self.try_get_any_mut_component(entity, T::id()));
    }
}