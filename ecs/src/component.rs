use std::any::Any;
use std::collections::HashMap;
use std::ops::DerefMut;

use crate::entity::Entity;

pub use ecs_macros::Component;
use crate::entity;

// Trait needed for creating a Component (note: the user doesn't have to manipulate this trait
// everything is included in the macro "derive(Component)"

pub trait ComponentTrait {
    fn id() -> u64 where Self: Sized;
}

// Components pools are distinguished by the type of their components
// This is the global trait to recognized a pool

pub trait AnyComponentPool {
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct ComponentPool<T>
    where T: ComponentTrait {
    /*
    sparse      : [x |x |x |0 | ...] <-- sparse index that refers to packed and components
    packed      : [e3|e7|e8|e6] <-- entities
    components  : [A0|A1|A2|A3] <-- components
     */

    pub sparse: Vec<usize>,
    pub packed: Vec<Entity>,
    pub components: Vec<Box<T>>,
}

// Impl the AnyComponent Trait for all ComponentPool<T>

impl<T: ComponentTrait + 'static> AnyComponentPool for ComponentPool<T> {
    fn as_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

impl<T: ComponentTrait> ComponentPool<T> {
    const ENTITY_THOMB: usize = usize::MAX;
    // Tombstone

    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            packed: Vec::new(),
            components: Vec::new(),
        }
    }

    fn as_sparse(&self, entity: &Entity) -> usize {
        (*entity) as usize
    }

    fn as_index(&self, entity: &Entity) -> usize {
        match self.sparse.get(self.as_sparse(entity)) {
            Some(i) => *i,
            None => Self::ENTITY_THOMB
        }
    }

    pub fn contains(&self, entity: &Entity) -> bool {
        self.as_sparse(entity) < self.sparse.len() && self.as_index(entity) != Self::ENTITY_THOMB
    }

    pub fn component(&mut self, entity: &Entity) -> Option<&mut T> {
        return match self.contains(entity) {
            true => {
                let index = self.as_index(entity);

                Some(self.components.get_mut(index).unwrap().deref_mut())
            },
            false => {
                None
            }
        };
    }

    pub fn create_or_retrieve(&mut self, entity: &Entity, value: T) -> &mut T {
        if !self.contains(entity) {
            let index = self.components.len();
            self.components.push(Box::new(value));
            self.packed.push(*entity);

            let sparse = self.as_sparse(entity);

            if sparse >= self.sparse.len() {
                self.sparse.resize(sparse + 1, Self::ENTITY_THOMB);
            }

            self.sparse[sparse] = index;
        }

        self.component(entity).unwrap()
    }

    pub fn swap(&mut self, a: &Entity, b: &Entity) {
        let sparse_a = self.as_sparse(a);
        let sparse_b = self.as_sparse(b);
        let index_a = self.as_index(a);
        let index_b = self.as_index(b);

        if index_a != Self::ENTITY_THOMB as usize && index_b != Self::ENTITY_THOMB as usize {
            self.components.swap(index_a, index_b);
            self.packed.swap(index_a, index_b);

            self.sparse[sparse_a] = index_b;
            self.sparse[sparse_b] = index_a;
        }
    }

    pub fn remove(&mut self, entity: &Entity) {
        let sparse = self.as_sparse(entity);
        let index = self.as_index(entity);

        if index != Self::ENTITY_THOMB {
            let last_entity = *self.packed.last().unwrap();
            let last_sparse = self.as_sparse(&last_entity);

            self.swap(&last_entity, entity);

            self.sparse[sparse] = Self::ENTITY_THOMB;

            self.packed.pop();
            self.components.pop();
        }
    }
}