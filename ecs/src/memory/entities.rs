/// This module manages all entities within an application based on a specific mapping.
/// A mapping refers to an efficient memory distribution that dictates how entities should be sorted
/// to facilitate access to entities possessing a known set of components (A, B, C, etc.)
/// without necessitating iteration or conditional statements.
///
/// This approach is founded on the concept of 'nested storages' introduced by other ECS systems,
/// notably Skypjack in his blog: https://skypjack.github.io/ for EnTT.
/// It involves smart swapping strategies to avoid fragmenting the main array.

use ahash::AHashMap;

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

    /// This method accepts a set of entities to be added to a specific group. For each entity provided, it performs
    /// the following action: if the entity already exists in the global group, it will relocate the entity within
    /// the nested groups; otherwise, it will add the entity to the global group and then perform the relocation.
    ///
    /// This function returns Ok(()) if all 'entities' are successfully associated with the specified 'group'.
    /// If any issues occur or inconsistencies are detected, it will return an Error indicating the problematic group.
    pub fn try_add_group(&mut self, group: Group, entities: &[Entity]) -> entities_errors::Result {
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
                                // Lastly, we traverse these groups from the right, swapping all entities that currently
                                // aren't part of the group to the end. If entity doesn't exist yet, it will add it before moving it
                                for nested in groups_to_cross.iter_mut().rev() {
                                    for entity in entities {
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

    /// This method accepts a set of entities to be removed to a specific group. For each entity provided, it performs
    /// the following action: if the entity exists in the nested groups, it will relocate it at the end of each nested group
    /// to finally remove it from the packed array
    pub fn try_remove_group(&mut self, group: Group, entities: &[Entity]) -> entities_errors::Result {
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
}