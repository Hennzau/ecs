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

    pub fn entities(&self) -> &Vec<Vec<Entity>> {
        return &self.entities;
    }

    pub fn view(&self, group: Group) -> &[Entity] {
        let (index, in_index) = self.mapping.search_for(group);

        return &self.entities.get(index).unwrap()[0..in_index];
    }

    pub fn process_add(&mut self, entity: &Entity, previous_components: &HashSet<Component>, components_to_add: &HashSet<Component>) -> HashSet<Group> {
        let groups = self.mapping.get_next_membership(previous_components, components_to_add);
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