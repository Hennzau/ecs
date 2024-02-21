use std::any::Any;
use crate::ecs::core::{
    entity,
    entity::{
        Entity,
        EntityIndex,
        NULL_ENTITY,
    },
    component::{
        ComponentID,
        ArchetypeID,
        AnyComponent,
    },
};

pub type ColumnID = usize;

pub struct SparsePool {
    entities: Vec<Entity>,
    columns: Vec<Vec<Box<dyn AnyComponent>>>,
    sparse: Vec<EntityIndex>,
}

impl SparsePool {
    pub fn new(size: usize) -> Self {
        return Self {
            entities: Vec::with_capacity(100),
            columns: Vec::with_capacity(size),
            sparse: Vec::from(&[NULL_ENTITY; 100]),
        };
    }

    pub fn contains(&self, entity: Entity) -> bool {
        return match self.sparse.get(entity::as_key(entity)).cloned() {
            Some(entity_index) => return entity_index != NULL_ENTITY,
            None => false
        };
    }

    pub fn as_slice(&self) -> (&[Entity], Vec<&[Box<dyn AnyComponent>]>) {
        let mut components = Vec::new();

        for column in &self.columns {
            components.push(column.as_slice());
        }

        return (self.entities.as_slice(), components);
    }

    pub fn as_mut_slice(&mut self) -> (&[Entity], Vec<&mut [Box<dyn AnyComponent>]>) {
        let mut components = Vec::new();

        for column in &mut self.columns {
            components.push(column.as_mut_slice());
        }

        return (self.entities.as_slice(), components);
    }

    pub fn get(&self, entity: Entity, column: ColumnID) -> Option<&Box<dyn AnyComponent>> {
        return self.sparse.get(entity::as_key(entity)).cloned().and_then(|entity_index| {
            if entity_index == NULL_ENTITY {
                return None;
            }

            return self.columns.get(column).and_then(|components| {
                return components.get(entity_index);
            });
        });
    }

    pub fn get_mut(&mut self, entity: Entity, column: ColumnID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.sparse.get(entity::as_key(entity)).cloned().and_then(|entity_index| {
            if entity_index == NULL_ENTITY {
                return None;
            }

            return self.columns.get_mut(column).and_then(|components| {
                return components.get_mut(entity_index);
            });
        });
    }
}