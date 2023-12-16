use std::collections::HashSet;
use crate::application::entity::Entity;
use crate::memory::mapping::{MemoryMapping, MemoryMappingDescriptor};

pub struct MappedStorage {
    entities: Vec<Vec<Entity>>,
    mapping: MemoryMapping,
}

impl MappedStorage {
    pub fn new(descriptor: MemoryMappingDescriptor) -> Self {
        let mut entities: Vec<Vec<Entity>> = Vec::new();
        let mapping = MemoryMapping::new(descriptor);

        for _ in 0..mapping.len() {
            entities.push(Vec::new());
        }

        Self {
            entities: entities,
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

    pub fn register_entity(&mut self, entity: &Entity, components: &HashSet<u64>, component: u64) {
        let groups = self.get_groups_to_update_when_add(components, component);
    }
}