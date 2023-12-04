use std::any::Any;
use std::collections::HashMap;
use std::ops::DerefMut;

use crate::entity::Entity;

pub use ecs_macros::Component;

/*
    Trait needed for creating a Component (note: the user doesn't have to manipulate this trait
    everything is included in the macro "derive(Component)"
*/

pub trait ComponentTrait {
    fn id() -> u64 where Self: Sized;
}

/*
    Components pools are distinguished by the type of their components
    This is the global trait to recognized a pool and use it without the need to know the type
    of the pool
*/

pub trait AnyComponentPool {
    /*
        Gives the possibility to downcast a Component Pool
    */

    fn as_any(&mut self) -> &mut dyn Any;

    /*
        Check if the Component Pool contains a component for this entity
    */

    fn contains(&self, entity: &Entity) -> bool;

    /*
        Remove a component from this Component Pool and keep the Vec packed
    */

    fn swap(&mut self, a: &Entity, b: &Entity);
    fn remove_component(&mut self, entity: &Entity);
}

pub struct ComponentPool<T>
    where T: ComponentTrait {
    map: HashMap<Entity, usize>,
    components: Vec<Box<T>>,
}

/*
    Impl the AnyComponent Trait for all ComponentPool<T>
*/

impl<T: ComponentTrait + 'static> AnyComponentPool for ComponentPool<T> {
    fn as_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn contains(&self, entity: &Entity) -> bool {
        self.map.contains_key(entity)
    }

    /*

    To remove a component, in order to keep a packed Vec of components, you may need to swap the component
    with the last component stored and pop the vector

    */

    fn swap(&mut self, a: &Entity, b: &Entity) {
        if self.contains(a) && self.contains(b) {
            // Now we are sure that the Component Pool contains everything you need to swap

            let index_a = *self.map.get(a).unwrap();
            let index_b = *self.map.get(b).unwrap();

            self.components.swap(index_a, index_b);

            self.map.entry(*a).or_insert(index_b);
            self.map.entry(*b).or_insert(index_a);
        }
    }

    fn remove_component(&mut self, entity: &Entity) {
        if self.contains(entity) {
            let last_index = self.components.len() - 1;

            let last_entity = *self.map.iter().find_map(
                |(key, &val)|
                    if val == last_index { Some(key) } else { None }
            ).unwrap(); // We are sure that there is such an entity associated with this value by construction

            self.swap(&last_entity, entity);

            self.map.remove(entity);
            self.components.pop();
        }
    }
}

/*
    Every function that can not be in the trait definition because you need to know T
*/

impl<T: ComponentTrait + 'static> ComponentPool<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            components: Vec::new(),
        }
    }

    /*
        Try to get the component of a certain entity. None if this entity doesn't have T component
    */

    pub fn try_get_component(&mut self, entity: &Entity) -> Option<&mut T> {
        return match self.contains(entity) {
            true => {
                // Now we are sure that the Component Pool has a component associated with 'entity'

                let index = *self.map.get(entity).unwrap();

                Some(self.components.get_mut(index).unwrap().deref_mut())
            }
            false => {
                None
            }
        };
    }

    /*
        Try to generate a component from the Component Pool, associated with this entity :
        if this entity already has a component from this pool, return it
    */

    pub fn add_or_get_component(&mut self, entity: &Entity, value: T) -> &mut T {
        if !self.contains(entity) {
            let index = self.components.len();
            self.components.push(Box::new(value));

            self.map.insert(*entity, index);
        }

        // Now we are sure that 'entity' has T component
        self.try_get_component(entity).unwrap()
    }
}