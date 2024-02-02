/// This module manages all entities within an application based on a specific mapping.
/// A mapping refers to an efficient memory distribution that dictates how entities should be sorted
/// to facilitate access to entities possessing a known set of components (A, B, C, etc.)
/// without necessitating iteration or conditional statements.
///
/// This approach is founded on the concept of 'nested storages' introduced by other ECS systems,
/// notably Skypjack in his blog: https://skypjack.github.io/ for EnTT.
/// It involves smart swapping strategies to avoid fragmenting the main array.

use ahash::{
    AHashMap, AHashSet,
};

use crate::core::{
    entity::Entity,
    component::Group,
};

pub struct Entities {
    /// This is the 'packed/dense' array containing all entities. It comprises multiple contiguous Entity storages,
    /// each associated with a distinct "main group" defined in the mapping. An example of such a storage could be:
    /// |---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    /// | 2 | 4 | 1 | 7 | 8 | 9 | 3 | 2 | 9 | 11| 4 | 5 | 6 | 9 |
    /// |---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    /// Here, each integer serves as a unique identifier referencing an entity.
    entities: Vec<Vec<Entity>>,

    /// These groups are generated based on a specific mapping. Each Vec<usize> represents a global group,
    /// which is a composition of ordered nested groups. Within these nested groups, the 'usize' value denotes
    /// the count of entities belonging to that particular nested group.
    groups: Vec<Vec<usize>>,

    /// This 'indices' array provides a way to track entities of a group inside de 'packed/dense' array
    indices: Vec<AHashMap<Entity, usize>>,

    /// This map correlates a Group with its index in the 'groups' (global group) array mentioned earlier,
    /// along with the 'in_index' representing the index of the corresponding nested group.
    map: AHashMap<Group, (usize, usize)>,
}

/// This submodule comprises various structures designed to manage errors encountered during
/// entity swapping and group addition processes.
pub mod entities_errors {
    use std::{
        error,
        fmt::{
            Display,
            Formatter,
        },
    };

    use crate::core::component::Group;

    pub type Result = std::result::Result<(), Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    pub struct GroupMappingError {
        pub group: Group,
    }

    impl Display for GroupMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : this group wasn't mapped correctly", self.group)
        }
    }

    impl error::Error for GroupMappingError {}

    #[derive(Debug, Clone)]
    pub struct EntitiesMappingError {
        pub group: Group,
    }

    impl Display for EntitiesMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : entities were not mapped correctly for this group", self.group)
        }
    }

    impl error::Error for EntitiesMappingError {}

    #[derive(Debug, Clone)]
    pub struct IndicesMappingError {
        pub group: Group,
    }

    impl Display for IndicesMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : indices were not mapped correctly for this group", self.group)
        }
    }

    impl error::Error for IndicesMappingError {}
}

impl Entities {
    /// Creates a new instance initializing internal data structures based on provided groups and mapping.
    pub fn new(groups: Vec<Vec<usize>>, map: AHashMap<Group, (usize, usize)>) -> Self {
        let mut entities = Vec::new();
        let mut indices = Vec::new();

        for _ in 0..groups.len() {
            entities.push(Vec::new());
            indices.push(AHashMap::new());
        }

        return Self {
            entities: entities,
            groups: groups,
            indices: indices,
            map: map,
        };
    }

    pub fn entities(&self) -> &[Vec<Entity>] {
        return &self.entities;
    }

    /// When the group is accurately mapped, this function will return a slice of the 'packed/dense' entities array
    /// containing all entities belonging to this particular group.
    pub fn try_view(&self, group: Group) -> Option<&[Entity]> {
        return self.map.get(&group).cloned().map_or_else(|| {
            log::warn!("You tried to view entities from group {}, but this group wasn't mapped", group);

            None
        }, |(index, in_index)| self.groups.get(index).map_or_else(|| {
            log::warn!("You tried to view entities from group {}, range error for groups : this group was not mapped correctly", group);

            None
        }, |nested| nested.get(in_index).cloned().map_or_else(|| {
            log::warn!("You tried to view entities from group {}, range error for nested group : this nested group was not mapped correctly", group);

            None
        }, |count| self.entities.get(index).map_or_else(|| {
            log::warn!("You tried to view entities from group {}, range error for entities : this storage was not created correctly", group);

            None
        }, |entities| entities.get(0..count)))));
    }

    // This function performs a smart relocation of entities within the array of a group. It moves all 'entities' in the new
    // by swaping the slices of the array. It also updates the indices of the entities in the 'indices' map.

    fn relocate_slice(indices: &mut AHashMap<Entity, usize>, array: &mut Vec<Entity>, old_first: usize, new_first: usize, count: usize) {
        if new_first >= old_first {
            return;
        }

        // separate the array
        let (left, right) = array.split_at_mut(old_first);

        if new_first + count <= old_first {
            let previous_entities = left.get_mut(new_first..(new_first + count));
            let entities = right.get_mut(0..count);

            if let Some(previous_entities) = previous_entities {
                if let Some(entities) = entities {
                    // First we update new indices

                    for (i, entity) in previous_entities.iter().enumerate() {
                        indices.insert(entity.clone(), old_first + i);
                    }

                    for (i, entity) in entities.iter().enumerate() {
                        indices.insert(entity.clone(), new_first + i);
                    }

                    // We swap the slices of the array
                    previous_entities.swap_with_slice(entities);
                }
            }
        } else if new_first < old_first {
            let gap = count - (old_first - new_first);

            let previous_entities = left.get_mut(new_first..old_first);

            let entities = right.get_mut(gap..count); // Be aware, it's not gap..gap+count

            if let Some(previous_entities) = previous_entities {
                if let Some(entities) = entities {
                    // First we update new indices

                    for (i, entity) in previous_entities.iter().enumerate() {
                        indices.insert(entity.clone(), old_first + gap + i);
                    }

                    for (i, entity) in entities.iter().enumerate() {
                        indices.insert(entity.clone(), new_first + i);
                    }

                    // We swap the slices of the array
                    previous_entities.swap_with_slice(entities);
                }
            }
        }
    }

    // This functions search for all entities in 'waiting' that are located between start_search and end_search.
    // It swaps them next to end_search - 1 - merge_count in order to move them in 'entities_to_add' slice next.

    fn swap_and_retrieve_waiting_entities(indices: &mut AHashMap<Entity, usize>, array: &mut Vec<Entity>, waiting: &mut Vec<Entity>, start_search: usize, end_search: usize) -> Vec<Entity> {
        let mut merged = Vec::<Entity>::new();

        for entity in waiting.iter().cloned() {
            if let Some(entity_index) = indices.get(&entity).cloned() {
                if entity_index >= start_search && entity_index < end_search {
                    let count = merged.len();

                    // We swap the entity to end_search - 1 - merge_count
                    if let Some(previous_entity) = array.get(end_search - 1 - count).cloned() {
                        merged.push(entity);

                        indices.insert(previous_entity, entity_index);
                        indices.insert(entity, end_search - 1 - count);

                        array.swap(entity_index, end_search - 1 - count);
                    }
                }
            }
        }

        for entity in &merged {
            waiting.retain(|e| e.clone() != entity.clone());
        }

        return merged;
    }

    /// This method accepts a set of entities to be added to a specific group. For each entity provided, it performs
    /// the following action: if the entity already exists in the global group, it will relocate the entity within
    /// the nested groups; otherwise, it will add the entity to the global group and then perform the relocation.
    ///
    /// This function returns Ok(()) if all 'entities' are successfully associated with the specified 'group'.
    /// If any issues occur or inconsistencies are detected, it will return an Error indicating the problematic group.
    pub fn try_add_group_to_entities(&mut self, group: Group, entities: &[Entity]) -> entities_errors::Result {
        // This step involves retrieving all necessary storages to add entities and computing the new position of the entity.
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => match self.indices.get_mut(index) {
                Some(indices) => match self.entities.get_mut(index) {
                    Some(array) => match self.groups.get_mut(index) {
                        Some(groups) => {
                            // We gather all nested groups located to the right of the target group.
                            if let Some(groups_to_cross) = match in_index <= groups.len() {
                                true => {
                                    let (_, groups) = groups.split_at_mut(in_index);

                                    Some(groups)
                                }
                                false => None
                            } {
                                // We gather all entities that needs to be first added to the group and the ones that
                                // are already in one of the nested groups (maybe it's not located at the right place)

                                let mut entities_to_add = Vec::<Entity>::new();
                                let mut waiting_entities = Vec::<Entity>::new();

                                let mut current_index = array.len();

                                for entity in entities {
                                    if indices.contains_key(entity) {
                                        waiting_entities.push(entity.clone());
                                    } else {
                                        entities_to_add.push(entity.clone());

                                        indices.insert(entity.clone(), array.len());
                                        array.push(entity.clone());
                                    }
                                }

                                // The idea is to swap the whole 'entities_to_add' slice each time, and when this slice enters
                                // a group where entities from 'waiting_entities' are located, we swap them in order to move
                                // them in 'entities_to_add' slice.

                                // We traverse these groups from the right and we swap all entities that must be added to the group
                                // At the end of each nested groups

                                for nested in groups_to_cross.iter_mut().rev() {
                                    // We search for all entities that are between our slice 'entities_to_add' and the end of the
                                    // current nested group. We swap them next to 'entities_to_add' slice in order to move them in.
                                    // This way, 'entities_to_add' slice will be bigger and bigger at each iteration, gathering
                                    // all entities that must be moved in the right group.

                                    let mut merged = Self::swap_and_retrieve_waiting_entities(indices, array, &mut waiting_entities, nested.clone(), current_index);

                                    current_index -= merged.len();
                                    entities_to_add.append(&mut merged);

                                    // This performs a smart relocation of all entities within the array of a group.

                                    Self::relocate_slice(indices, array, current_index, nested.clone(), entities_to_add.len());

                                    current_index = nested.clone();
                                    *nested += entities_to_add.len();
                                }
                            }
                            Ok(())
                        }
                        None => Err(entities_errors::GroupMappingError { group: group }.into())
                    }
                    None => Err(entities_errors::EntitiesMappingError { group: group }.into())
                }
                None => Err(entities_errors::IndicesMappingError { group: group }.into())
            },
            None => Err(entities_errors::GroupMappingError { group: group }.into())
        };
    }

    pub fn try_add_group_to_entity(&mut self, group: Group, entity: &Entity) -> entities_errors::Result {
        // This step involves retrieving all necessary storages to add entities and computing the new position of the entity.
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => match self.indices.get_mut(index) {
                Some(indices) => match self.entities.get_mut(index) {
                    Some(array) => match self.groups.get_mut(index) {
                        Some(groups) => {
                            // We gather all nested groups located to the right of the target group.
                            if let Some(groups_to_cross) = match in_index <= groups.len() {
                                true => {
                                    let (_, groups) = groups.split_at_mut(in_index);

                                    Some(groups)
                                }
                                false => None
                            } {
                                for nested in groups_to_cross.iter_mut().rev() {
                                    if let Some(entity_index) = indices.get(entity).cloned() {
                                        if entity_index >= nested.clone() {
                                            if let Some(previous_entity) = array.get(nested.clone()).cloned() {
                                                indices.insert(previous_entity, entity_index);
                                                indices.insert(entity.clone(), nested.clone());

                                                array.swap(entity_index, nested.clone());
                                            }

                                            *nested += 1;
                                        }
                                    } else {
                                        // If the entity doesn't exist yet, we add it to the global group and it means
                                        // that the current nested group is the last one. So because they are no entities
                                        // that are in 'array' and in no nested group, we can safely add the entity at the end.
                                        // without the need to swap it.

                                        let entity_index = array.len();

                                        array.push(entity.clone());
                                        indices.insert(entity.clone(), entity_index);

                                        *nested += 1;
                                    }
                                }
                            }
                            Ok(())
                        }
                        None => Err(entities_errors::GroupMappingError { group: group }.into())
                    }
                    None => Err(entities_errors::EntitiesMappingError { group: group }.into())
                }
                None => Err(entities_errors::IndicesMappingError { group: group }.into())
            },
            None => Err(entities_errors::GroupMappingError { group: group }.into())
        };
    }

    pub fn try_add_groups_to_entities(&mut self, groups: &AHashSet<Group>, entities: &[Entity]) -> entities_errors::Result {
        let mut result = Ok(());

        for group in groups {
            let res = self.try_add_group_to_entities(group.clone(), entities);
            if res.is_err() {
                result = res;
            }
        }

        return result;
    }

    pub fn try_add_groups_to_entity(&mut self, groups: &AHashSet<Group>, entity: Entity) -> entities_errors::Result {
        let mut result = Ok(());

        for group in groups {
            let res = self.try_add_group_to_entities(group.clone(), entities);
            if res.is_err() {
                result = res;
            }
        }

        return result;
    }

    /// This method accepts a set of entities to be removed to a specific group. For each entity provided, it performs
    /// the following action: if the entity exists in the nested groups, it will relocate it at the end of each nested group
    /// to finally remove it from the packed array
    pub fn try_remove_group_to_entities(&mut self, group: Group, entities: &[Entity]) -> entities_errors::Result {
        // This step involves retrieving all necessary storages to add entities and computing the new position of the entity.
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => match self.indices.get_mut(index) {
                Some(indices) => match self.entities.get_mut(index) {
                    Some(array) => match self.groups.get_mut(index) {
                        Some(groups) => {
                            let last_in_index = groups.len() - 1;

                            // We gather all nested groups located to the left of the target group (including the target group).
                            if let Some(groups_to_cross) = match in_index < groups.len() {
                                true => {
                                    let (groups, _) = groups.split_at_mut(in_index + 1);

                                    Some(groups)
                                }
                                false => None
                            } {
                                // Lastly, we traverse these groups from the l, swapping all entities that currently
                                // are part of the group to the end.
                                for (i, nested) in groups_to_cross.iter_mut().enumerate() {
                                    for entity in entities {
                                        if let Some(entity_index) = indices.get(entity).cloned() {
                                            if entity_index < nested.clone() {
                                                if let Some(previous_entity) = array.get(nested.clone() - 1).cloned() {
                                                    indices.insert(previous_entity, entity_index);
                                                    indices.insert(entity.clone(), nested.clone() - 1);

                                                    array.swap(entity_index, nested.clone() - 1);

                                                    if i == last_in_index {
                                                        array.pop();
                                                        indices.remove(entity);
                                                    }
                                                }

                                                *nested -= 1;
                                            }
                                        }
                                    }
                                }
                            }

                            Ok(())
                        }
                        None => Err(entities_errors::GroupMappingError { group: group }.into())
                    }
                    None => Err(entities_errors::EntitiesMappingError { group: group }.into())
                }
                None => Err(entities_errors::IndicesMappingError { group: group }.into())
            },
            None => Err(entities_errors::GroupMappingError { group: group }.into())
        };
    }

    pub fn try_remove_group_to_entity(&mut self, group: Group, entity: Entity) -> entities_errors::Result {
        return self.try_remove_group_to_entities(group, &[entity]);
    }

    pub fn try_remove_groups_to_entities(&mut self, groups: &AHashSet<Group>, entities: &[Entity]) -> entities_errors::Result {
        let mut result = Ok(());

        for group in groups {
            let res = self.try_remove_group_to_entities(group.clone(), entities);
            if res.is_err() {
                result = res;
            }
        }

        return result;
    }

    pub fn try_remove_groups_to_entity(&mut self, groups: &AHashSet<Group>, entity: Entity) -> entities_errors::Result {
        let mut result = Ok(());

        for group in groups {
            let res = self.try_remove_group_to_entity(group.clone(), entities);
            if res.is_err() {
                result = res;
            }
        }

        return result;
    }
}