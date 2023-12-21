use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    memory::{
        MemoryMapping,
        MemoryMappingDescriptor,
    },
    application::{
        entity::Entity,
        component::{
            Component,
            Group,
            components_to_group,
        },
    },
};

pub struct MappedStorage {
    entities: Vec<Vec<Entity>>,
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

    fn swap_entities(&mut self, container: usize, a: usize, b: usize) -> usize {
        let entity_a = self.entities.get(container).unwrap().get(a).unwrap().clone();
        let entity_b = self.entities.get(container).unwrap().get(b).unwrap().clone();

        *self.indices.get_mut(container).unwrap().get_mut(&entity_a).unwrap() = b;
        *self.indices.get_mut(container).unwrap().get_mut(&entity_b).unwrap() = a;

        let entities = self.entities.get_mut(container).unwrap();

        entities.swap(a, b);

        return b;
    }

    fn update_new(&self, components: &HashSet<Component>, component: Component) -> HashSet<Group> {
        let mut previous_groups = HashSet::<u128>::new();
        let mut new_groups = HashSet::<u128>::new();

        for group in self.mapping.descriptor() {
            if group.iter().all(|x| components.contains(x)) {
                previous_groups.insert(components_to_group(group));
            }

            if group.iter().all(|x| components.contains(x) || *x == component) {
                new_groups.insert(components_to_group(group));
            }
        }

        return new_groups.symmetric_difference(&previous_groups).cloned().collect();
    }

    pub fn process_add(&mut self, entity: &Entity, components: &HashSet<Component>, component: Component) -> HashSet<Group> {
        let groups = self.update_new(components, component);
        let mapped_groups = self.mapping.map_and_sort(&groups);

        for (container, i) in mapped_groups {
            let mut index = match self.indices.get(container).unwrap().get(entity) {
                Some(index) => index.clone(),
                None => {
                    let last = self.entities.get(container).unwrap().len();
                    self.entities.get_mut(container).unwrap().push(*entity);
                    self.indices.get_mut(container).unwrap().insert(*entity, last);

                    last
                }
            };

            for j in i.iter().rev().copied() { // Iterate over the largest set of component to the smallest
                let value = self.mapping.cursor(container, j);
                self.mapping.advance_cursor(container, j);

                index = self.swap_entities(container, index, value);
            }
        }

        return groups;
    }

    fn update_remove(&self, components: &HashSet<Component>, component: Component) -> HashSet<Group> {
        let mut previous_groups = HashSet::<u128>::new();
        let mut new_groups = HashSet::<u128>::new();

        for group in self.mapping.descriptor() {
            if group.iter().all(|x| components.contains(x)) {
                previous_groups.insert(components_to_group(group));
            }

            if group.iter().all(|x| components.contains(x) && *x != component) {
                new_groups.insert(components_to_group(group));
            }
        }

        return new_groups.symmetric_difference(&previous_groups).cloned().collect();
    }

    pub fn process_remove(&mut self, entity: &Entity, components: &HashSet<Component>, component: Component) -> HashSet<Group> {
        let groups = self.update_remove(components, component);
        let mapped_groups = self.mapping.map_and_sort(&groups);

        for (container, i) in mapped_groups {
            // By construction we are sure that there is such an entity in this container.
            let mut index = self.indices.get(container).unwrap().get(entity).unwrap().clone();

            for j in i.iter().copied() { // Iterate over the largest set of component to the smallest
                let value = self.mapping.cursor(container, j);
                self.mapping.move_back_cursor(container, j);

                index = self.swap_entities(container, index, value - 1);
            }

            // remove the entity if it's outside every groups of the container

            let last_group = self.mapping.cursors(container).last().unwrap().clone();
            if index >= last_group {
                self.entities.get_mut(container).unwrap().swap_remove(index);
                self.indices.get_mut(container).unwrap().remove(entity);
            }
        }

        return groups;
    }

    pub fn view(&self, group: Group) -> &[Entity] {
        let (index, in_index) = self.mapping.search_for(group);

        return &self.entities.get(index).unwrap()[0..in_index];
    }

    pub fn entities(&self) -> &Vec<Vec<Entity>> {
        &self.entities
    }
}