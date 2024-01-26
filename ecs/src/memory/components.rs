/// This module contains the `Components` struct, which is used to store all components in the game.
/// It aims to be a simple and efficient way to store components : user can add, remove and get components easily
/// and efficiently.

use ahash::AHashMap;

use crate::core::{
    entity::Entity,
    component::{
        ComponentID,
        AnyComponent,
    },
};

/// This struct is used to store all components in the game.
pub struct Components {
    /// Each element of the primary vector acts as a pool of components of the same type.
    components: Vec<Vec<Box<dyn AnyComponent>>>,

    /// Each element corresponds to indices from the pool of components of the same type.
    indices: Vec<AHashMap<Entity, usize>>,

    /// This map is used to find the right pool of components from the component ID.
    map: AHashMap<ComponentID, usize>,
}

impl Components {
    /// Creates a new instance of the `Components` struct.
    pub fn new() -> Self {
        return Self {
            components: Vec::new(),
            indices: Vec::new(),
            map: AHashMap::new(),
        };
    }

    /// Downcasts a `Box<dyn AnyComponent>` into a `&T` if possible.
    pub fn convert<T: AnyComponent + 'static>(component: &Box<dyn AnyComponent>) -> Option<&T> {
        return component.as_any().downcast_ref::<T>();
    }

    /// Downcasts a `Box<dyn AnyComponent>` into a `&mut T` if possible.
    pub fn convert_mut<T: AnyComponent + 'static>(component: &mut Box<dyn AnyComponent>) -> Option<&mut T> {
        return component.as_any_mut().downcast_mut::<T>();
    }

    /// Downcasts a `Option<&Box<dyn AnyComponent>>` into a `Option<&T>` if possible.
    pub fn convert_ok<T: AnyComponent + 'static>(component: Option<&Box<dyn AnyComponent>>) -> Option<&T> {
        return component.and_then(|component| component.as_any().downcast_ref::<T>());
    }

    /// Downcasts a `Option<&mut Box<dyn AnyComponent>>` into a `Option<&mut T>` if possible.
    pub fn convert_mut_ok<T: AnyComponent + 'static>(component: Option<&mut Box<dyn AnyComponent>>) -> Option<&mut T> {
        return component.and_then(|component| component.as_any_mut().downcast_mut::<T>());
    }

    /// Returns `true` if the given entity has the given component. It first checks if the pool exists and then checks
    /// if the pool contains the entity.
    pub fn contains(&self, entity: &Entity, id: ComponentID) -> bool {
        return match self.map.get(&id) {
            Some(index) => match self.indices.get(index.clone()) {
                Some(indices) => indices.contains_key(entity),
                None => false
            },
            None => false
        };
    }

    /// Adds a component to the given entity. If the entity already has the component, it returns an error.
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
            self.indices.push(AHashMap::from([(entity.clone(), 0)]));
            self.map.insert(id, index);

            return Ok(());
        }

        return Err(());
    }

    /// Adds a component to the given entity. If the entity already has the component, it returns an error.
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
                        indices.remove(entity);

                        return Ok(components.swap_remove(in_index));
                    }
                }
            }
        }

        return Err(());
    }

    /// Returns a reference to the component of the given entity if it exists.
    pub fn try_get_any_component(&self, entity: &Entity, id: ComponentID) -> Option<&Box<dyn AnyComponent>> {
        return self.map.get(&id).cloned().and_then(
            |index| self.components.get(index).and_then(
                |components| self.indices.get(index).and_then(
                    |indices| indices.get(entity).cloned().and_then(
                        |in_index| components.get(in_index)))));
    }

    /// Returns a mutable reference to the component of the given entity if it exists.
    pub fn try_get_any_mut_component(&mut self, entity: &Entity, id: ComponentID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.map.get(&id).cloned().and_then(
            |index| self.components.get_mut(index).and_then(
                |components| self.indices.get(index).and_then(
                    |indices| indices.get(entity).cloned().and_then(
                        |in_index| components.get_mut(in_index)))));
    }

    /// Returns a reference to the component of the given entity if it exists.
    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return Self::convert_ok(self.try_get_any_component(entity, T::component_id()));
    }

    /// Returns a mutable reference to the component of the given entity if it exists.
    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return Self::convert_mut_ok(self.try_get_any_mut_component(entity, T::component_id()));
    }
}