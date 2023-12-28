use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    core::{
        component::{
            Component,
            AnyComponent,
            Group,
        },
        entity::Entity,
    },
    memory::{
        factory::Factory,
        MemoryMapping,
        MemoryMappingDescriptor,
    },
};

pub struct PackedEntities {
    mapping: MemoryMapping,
    entities: Vec<Vec<Entity>>,
    indices: Vec<HashMap<Entity, usize>>,
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

    pub fn entities(&self) -> &Vec<Vec<Entity>> {
        return &self.entities;
    }

    pub fn view(&self, group: Group) -> &[Entity] {
        let (index, in_index) = self.mapping.search_for(group);

        return &self.entities.get(index).unwrap()[0..in_index];
    }

    pub fn process_add(&mut self, entity: &Entity, previous_components: &HashSet<Component>, components_to_add: &HashSet<Component>) -> HashSet<Group> {
        let groups = self.mapping.get_next_membership(previous_components, components_to_add);
        let mapped_groups = self.mapping.map_and_sort(&groups);

        for (container, i) in mapped_groups {
            let index = match self.indices.get_mut(container) {
                Some(indices) => match indices.get(entity) {
                    Some(index) => Ok(index.clone()),
                    None => match self.entities.get_mut(container) {
                        Some(entities) => {
                            let last = entities.len();

                            entities.push(entity.clone());
                            indices.insert(entity.clone(), last);

                            Ok(last)
                        }
                        None => Err(format!("No entities for container {}!", container))
                    },
                },
                None => Err(format!("No indices for container {}!", container))
            };

            match index {
                Ok(mut index) => {
                    for j in i.iter().rev().copied() { // Iterate over the largest set of component to the smallest
                        let value = self.mapping.cursor(container, j);
                        self.mapping.advance_cursor(container, j);

                        index = self.swap_entities(container, index, value);
                    }
                }

                Err(e) => {
                    log::warn!("{}",e);
                }
            };
        }

        return groups;
    }

    pub fn process_remove(&mut self, entity: &Entity, components: &HashSet<Component>, components_removed: &HashSet<Component>) -> HashSet<Group> {
        let groups = self.mapping.get_next_membership(components, components_removed);
        let mapped_groups = self.mapping.map_and_sort(&groups);

        for (container, i) in mapped_groups {
            let index = match self.indices.get_mut(container) {
                Some(indices) => match indices.get(entity) {
                    Some(index) => Ok(index.clone()),
                    None => Err(format!("Entity {} was not mapped in indices for container {}", entity, container))
                },
                None => Err(format!("No indices for container {}!", container))
            };

            match index {
                Ok(mut index) => {
                    for j in i.iter().copied() { // Iterate over the largest set of component to the smallest
                        let value = self.mapping.cursor(container, j);
                        self.mapping.move_back_cursor(container, j);

                        index = self.swap_entities(container, index, value - 1);
                    }

                    // remove the entity if it's outside every groups of the container

                    let last_group = self.mapping.cursors(container).last().cloned();
                    if let Some(last_group) = last_group {
                        if index >= last_group {
                            if let Some(entities) = self.entities.get_mut(container) {
                                entities.swap_remove(index);
                            }
                            if let Some(indices) = self.indices.get_mut(container) {
                                indices.remove(entity);
                            }
                        }
                    }
                }

                Err(e) => {
                    log::warn!("{}", e);
                }
            };
        }

        return groups;
    }

    fn swap_entities(&mut self, container: usize, a: usize, b: usize) -> usize {
        let entity_a = self.entities.get(container).unwrap().get(a).unwrap();
        let entity_b = self.entities.get(container).unwrap().get(b).unwrap();

        if let Some(container) = self.indices.get_mut(container) {
            if let Some(cursor) = container.get_mut(entity_a) {
                *cursor = b;
            }

            if let Some(cursor) = container.get_mut(entity_b) {
                *cursor = a;
            }
        }

        if let Some(entities) = self.entities.get_mut(container) {
            entities.swap(a, b);
        }

        return b;
    }
}