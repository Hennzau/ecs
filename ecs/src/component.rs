use std::any::Any;
use std::collections::HashMap;
use std::ops::DerefMut;

use crate::entity::Entity;

pub use ecs_macros::Component;

pub trait ComponentTrait: Default {
    fn id() -> u64 where Self: Sized;
}

pub trait AnyComponentPool {
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct ComponentPool<T>
    where T: ComponentTrait {
    components: Vec<Box<T>>,
    map: HashMap<Entity, u32>
}

impl<T: ComponentTrait> ComponentPool<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            map: HashMap::new()
        }
    }

    pub fn generate(&mut self) -> (u32, &mut T) {
        let result = self.components.len() as u32;

        self.components.push(Box::new(T::default()));

        (result, self.components.last_mut().unwrap().deref_mut())
    }

    pub fn assign(&mut self, entity: &Entity, index: u32) {
        if !self.map.contains_key(entity) {
            self.map.insert(*entity, index);
        } else {
            self.map.entry(*entity).or_insert(index);
        }
    }
}

impl<T: ComponentTrait + 'static> AnyComponentPool for ComponentPool<T> {
    fn as_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}