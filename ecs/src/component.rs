use std::any::Any;
use std::collections::HashMap;

use crate::entity::Entity;

pub use ecs_macros::Component;

pub trait ComponentTrait {
    fn id() -> u64 where Self: Sized;
}

pub trait AnyComponentPool {
    fn as_any(&self) -> &dyn Any;
}

pub struct ComponentPool<T>
    where T: ComponentTrait {
    pub components: Vec<Box<T>>

}

impl<T: ComponentTrait> ComponentPool<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::new()
        }
    }
}

impl<T: ComponentTrait + 'static> AnyComponentPool for ComponentPool<T> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}