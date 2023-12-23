use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    core::{
        component::{
            Component,
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
use crate::core::component::AnyComponent;

struct PackedEntities {
    mapping: MemoryMapping,
    entities: Vec<Vec<Entity>>,
    indices: Vec<HashMap<Entity, usize>>,
}

pub struct Storage {
    packed: PackedEntities,
    entities: HashMap<Entity, HashSet<Component>>,

    factory: Factory,
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
                    Some(index) => Some(index.clone()),
                    None => match self.entities.get_mut(container) {
                        Some(entities) => {
                            let last = entities.len();

                            entities.push(entity.clone());
                            indices.insert(entity.clone(), last);

                            Some(last)
                        }
                        None => None
                    },
                },
                None => None
            };

            if let Some(mut index) = index {
                for j in i.iter().rev().copied() { // Iterate over the largest set of component to the smallest
                    let value = self.mapping.cursor(container, j);
                    self.mapping.advance_cursor(container, j);

                    index = self.swap_entities(container, index, value);
                }
            }
        }

        return groups;
    }

    pub fn process_remove(&mut self, entity: &Entity, components: &HashSet<Component>, components_removed: &HashSet<Component>) -> HashSet<Group> {
        let groups = self.mapping.get_next_membership(components, components_removed);
        let mapped_groups = self.mapping.map_and_sort(&groups);

        for (container, i) in mapped_groups {
            let index = match self.indices.get_mut(container) {
                Some(indices) => match indices.get(entity) {
                    Some(index) => Some(index.clone()),
                    None => None
                },
                None => None
            };

            if let Some(mut index) = index {
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

impl Storage {
    pub fn new(descriptor: MemoryMappingDescriptor) -> Self {
        return Self {
            packed: PackedEntities::new(descriptor),
            entities: HashMap::new(),
            factory: Factory::new(),
        };
    }

    pub fn push_entity(&mut self, id: u64) {
        self.entities.insert(id as Entity, HashSet::new());
    }

    pub fn entities(&self) -> &Vec<Vec<Entity>> {
        return self.packed.entities();
    }

    pub fn view(&self, group: Group) -> &[Entity] {
        return self.packed.view(group);
    }

    pub fn add_get_or_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> (&mut T, HashSet<Group>) {
        let mut groups = HashSet::<Group>::new();
        let components = self.entities.get(entity).cloned();

        if self.try_add(entity, T::id()) {
            if let Some(components) = components {
                groups = self.packed.process_add(entity, &components, &HashSet::from([T::id()]));
            }
        }

        return (self.factory.add_get_or_get_component(entity, value), groups);
    }

    pub fn try_add_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> (bool, HashSet<Group>) {
        let mut groups = HashSet::<Group>::new();
        let components = self.entities.get(entity).cloned();

        if self.try_add(entity, T::id()) {
            if let Some(components) = components {
                groups = self.packed.process_add(entity, &components, &HashSet::from([T::id()]));
            }
        }

        return (self.factory.try_add_component(entity, value), groups);
    }

    pub fn try_remove_get_component_any(&mut self, entity: &Entity, id: Component) -> (Option<Box<dyn AnyComponent>>, HashSet<Group>) {
        let mut groups = HashSet::<Group>::new();

        if self.try_remove(entity, id) {
            if let Some(components) = self.entities.get(entity) {
                groups = self.packed.process_remove(entity, components, &HashSet::from([id]));
            }
        }

        return (self.factory.try_remove_get_component_any(entity, id), groups);
    }

    pub fn try_remove_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> (Option<Box<T>>, HashSet<Group>) {
        let mut groups = HashSet::<Group>::new();

        if self.try_remove(entity, T::id()) {
            if let Some(components) = self.entities.get(entity) {
                groups = self.packed.process_remove(entity, components, &HashSet::from([T::id()]));
            }
        }

        return (self.factory.try_remove_get_component::<T>(entity), groups);
    }

    pub fn try_remove_component_any(&mut self, entity: &Entity, id: Component) -> (bool, HashSet<Group>) {
        let mut groups = HashSet::<Group>::new();

        if self.try_remove(entity, id) {
            if let Some(components) = self.entities.get(entity) {
                groups = self.packed.process_remove(entity, components, &HashSet::from([id]));
            }
        }

        return (self.factory.try_remove_component_any(entity, id), groups);
    }

    pub fn try_remove_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> (bool, HashSet<Group>) {
        let mut groups = HashSet::<Group>::new();

        if self.try_remove(entity, T::id()) {
            if let Some(components) = self.entities.get(entity) {
                groups = self.packed.process_remove(entity, components, &HashSet::from([T::id()]));
            }
        }

        return (self.factory.try_remove_component::<T>(entity), groups);
    }

    pub fn try_get_component_mut<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return self.factory.try_get_component_mut::<T>(entity);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return self.factory.try_get_component::<T>(entity);
    }

    fn try_add(&mut self, entity: &Entity, id: Component) -> bool {
        if let Some(components) = self.entities.get_mut(entity) {
            if components.contains(&id) {
                return false;
            }

            components.insert(id);
        } else {
            let mut set = HashSet::new();
            set.insert(id);

            self.entities.insert(entity.clone(), set);
        }

        return true;
    }

    fn try_remove(&mut self, entity: &Entity, id: Component) -> bool {
        if let Some(components) = self.entities.get_mut(entity) {
            if !components.contains(&id) {
                return false;
            }

            components.remove(&id);

            return true;
        }

        return false;
    }
}