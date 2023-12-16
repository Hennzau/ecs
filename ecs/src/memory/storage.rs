use std::collections::{HashMap, HashSet};
use crate::application::entity::Entity;
use crate::memory::mapping::{MemoryMapping, MemoryMappingDescriptor};

pub struct MappedStorage {
    pub entities: Vec<Vec<Entity>>,
    indices: Vec<HashMap<Entity, usize>>,
    mapping: MemoryMapping,
}

impl MappedStorage {
    pub fn new(descriptor: MemoryMappingDescriptor) -> Self {
        let mut entities: Vec<Vec<Entity>> = Vec::new();
        let mut indices: Vec<HashMap<Entity, usize>> = Vec::new();
        let mapping = MemoryMapping::new(descriptor);

        for _ in 0..mapping.len() {
            entities.push(Vec::new());
            indices.push(HashMap::new());
        }

        Self {
            entities: entities,
            indices: indices,
            mapping: mapping,
        }
    }

    pub fn systems(&self) -> &MemoryMappingDescriptor {
        self.mapping.descriptor()
    }

    fn group_id(&self, set: &Vec<u64>) -> u128 {
        let mut result = 0u128;

        for &id in set {
            result += id as u128;
        }

        return result;
    }

    fn swap(&mut self, container: usize, a: usize, b: usize) {
        let entities = self.entities.get_mut(container).unwrap();

        entities.swap(a, b);
    }

    fn get_groups_to_update_when_add(&self, components: &HashSet<u64>, component: u64) -> Vec<u128> {
        let mut previous_groups = HashSet::<u128>::new();
        let mut new_groups = HashSet::<u128>::new();

        for group in self.mapping.descriptor() {
            if group.iter().all(|x| components.contains(x)) {
                previous_groups.insert(self.group_id(group));
            }

            if group.iter().all(|x| components.contains(x) || *x == component) {
                new_groups.insert(self.group_id(group));
            }
        }

        return new_groups.symmetric_difference(&previous_groups).cloned().collect();
    }

    pub fn add_entity(&mut self, entity: &Entity, components: &HashSet<u64>, component: u64) {
        let groups = self.get_groups_to_update_when_add(components, component);
        let groups = self.mapping.get_complete_groups_to_update_when_add(&groups);

        for (container, i) in groups {
            let mut index = match self.indices.get(container).unwrap().get(entity) {
                Some(index) => Some(*index),
                None => None
            };

            if index.is_none() {
                let last = self.entities.get(container).unwrap().len();
                self.entities.get_mut(container).unwrap().push(*entity);
                self.indices.get_mut(container).unwrap().insert(*entity, last);
                index = Some(last);
            }

            let index = index.unwrap();

            for j in i.iter().rev().copied() { // Iterate over the largest set of component to the smallest
                let value = self.mapping.value(container, j);
                self.swap(container, index, value);
                self.mapping.update_value(container, j, value + 1);
            }
        }
    }
}