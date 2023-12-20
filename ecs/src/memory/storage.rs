use std::collections::{HashMap, HashSet};

use crate::{
    core::{
        entity::Entity,
        component::{
            Component,
            Group,
            components_to_group,
        },
    },
    memory::{
        MemoryMapping,
        MemoryMappingDescriptor,
    },
};

struct PackedEntities {
    mapping: MemoryMapping,
    entities: Vec<Vec<Entity>>,
    indices: Vec<HashMap<Entity, usize>>,
}

pub struct Storage {
    packed: PackedEntities,
    entities: HashMap<Entity, HashSet<Component>>,
}

impl PackedEntities {
    pub fn new(descriptor: MemoryMappingDescriptor) -> Self {
        let mut entities = Vec::<Vec<Entity>>::new();
        let mut indices = Vec::<HashMap<Entity, usize>>::new();
        let mapping = MemoryMapping::new(descriptor);

        for _ in 0..mapping.len() {
            entities.push(Vec::new());
            indices.push(HashMap::new());
        }

        return Self {
            entities: entities,
            indices: indices,
            mapping: mapping,
        };
    }

    fn swap_entitids_test(&mut self, container: usize, a: usize, b: usize) -> Option<usize> {
        if let Some((entity_a, entity_b)) = match self.entities.get(container) {
            Some(container) => match container.get(a).cloned() {
                Some(entity_a) => match container.get(b).cloned() {
                    Some(entity_b) => Some((entity_a, entity_b)),
                    None => None
                }
                None => None
            }
            None => None
        } {
            return match self.entities.get_mut(container) {
                Some(entities) => {
                    entities.swap(a, b);

                    return match self.indices.get_mut(container) {
                        Some(indices) => match indices.get_mut(&entity_a) {
                            Some(cursor_a) => {
                                *cursor_a = b;

                                return match indices.get_mut(&entity_b) {
                                    Some(cursor_b) => {
                                        *cursor_b = a;

                                        return Some (b);
                                    }
                                    None => None
                                };
                            }
                            None => None
                        },
                        None => None
                    };
                }
                None => None
            };
        }

        return None;
    }

    fn swap_entities(&mut self, container: usize, a: usize, b: usize) -> usize {
        let entity_a = self.entities.get(container).unwrap().get(a).unwrap().clone();
        let entity_b = self.entities.get(container).unwrap().get(b).unwrap().clone();

        *self.indices.get_mut(container).unwrap().get_mut(&entity_a).unwrap() = b;
        *self.indices.get_mut(container).unwrap().get_mut(&entity_b).unwrap() = a;

        let entities = self.entities.get_mut(container).unwrap();

        entities.swap(a, b);

        return b;
    }
}