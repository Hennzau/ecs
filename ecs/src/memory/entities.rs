/// This module manages all entities within an application based on a specific mapping.
/// A mapping refers to an efficient memory distribution that dictates how entities should be sorted
/// to facilitate access to entities possessing a known set of components (A, B, C, etc.)
/// without necessitating iteration or conditional statements.
///
/// This approach is founded on the concept of 'nested storages' introduced by other ECS systems,
/// notably Skypjack in his blog: https://skypjack.github.io/ for EnTT.
/// It involves smart swapping strategies to avoid fragmenting the main array.

use std::collections::HashMap;

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
    indices: Vec<HashMap<Entity, usize>>,

    /// This map correlates a Group with its index in the 'groups' (global group) array mentioned earlier,
    /// along with the 'in_index' representing the index of the corresponding nested group.
    map: HashMap<Group, (usize, usize)>,
}

/// This submodule comprises various structures designed to manage errors encountered during
/// entity swapping and group addition processes.
pub mod errors {
    use std::{
        error,
        fmt::{
            Display,
            Formatter
        }
    };

    use crate::core::component::Group;

    pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    pub struct GroupMappingError {
        pub group: Group
    }

    impl Display for GroupMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : this group wasn't mapped correctly", self.group)
        }
    }

    impl error::Error for GroupMappingError {}

    #[derive(Debug, Clone)]
    pub struct EntitiesMappingError {
        pub group: Group
    }

    impl Display for EntitiesMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : entities were not mapped correctly for this group", self.group)
        }
    }

    impl error::Error for EntitiesMappingError {}

    #[derive(Debug, Clone)]
    pub struct IndicesMappingError {
        pub group: Group
    }

    impl Display for IndicesMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : indices were not mapped correctly for this group", self.group)
        }
    }

    impl error::Error for IndicesMappingError {}
}

impl Entities {
    pub fn new() -> Self {
        let mut entities = Vec::new();
        let mut groups = Vec::new();
        let mut indices = Vec::new();
        let mut map = HashMap::new();

        entities.push(Vec::new());
        indices.push(HashMap::new());
        groups.push(vec![0, 0, 0]);

        map.insert(0, (0, 0));
        map.insert(1, (0, 1));
        map.insert(2, (0, 2));

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
    pub fn view(&self, group: Group) -> Option<&[Entity]> {
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
    pub fn try_add_group(&mut self, group: Group, entities: &[Entity]) -> errors::Result<()> {
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => match self.indices.get_mut(index) {
                Some(indices) => match self.entities.get_mut(index) {
                    Some(array) => match self.groups.get_mut(index) {
                        Some(groups) => {
                            if let Some(groups_to_cross) = match in_index <= groups.len() {
                                true => {
                                    let (_, groups) = groups.split_at_mut(in_index);

                                    Some(groups)
                                }
                                false => None
                            } {
                                for nested in groups_to_cross.iter_mut().rev() {
                                    for entity in entities {
                                        if let Some(entity_index) = indices.get_mut(entity) {
                                            if entity_index.clone() >= nested.clone() {
                                                array.swap(entity_index.clone(), nested.clone());

                                                *entity_index = nested.clone();
                                                *nested += 1;
                                            }
                                        } else {
                                            let entity_index = array.len();
                                            array.push(entity.clone());
                                            array.swap(entity_index, nested.clone());

                                            indices.insert(entity.clone(), nested.clone());
                                            *nested += 1;
                                        }
                                    }
                                }
                            }

                            Ok(())
                        },
                        None => Err(errors::GroupMappingError { group: group }.into())
                    }
                    None => Err(errors::EntitiesMappingError { group: group }.into())
                }
                None => Err(errors::IndicesMappingError { group: group }.into())
            },
            None => Err(errors::GroupMappingError { group: group }.into())
        };
    }
}