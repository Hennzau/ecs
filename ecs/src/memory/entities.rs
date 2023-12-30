/// This module manages an Entity container


use std::collections::HashMap;

use crate::core::
{
    entity::Entity,
    component::Group,
};


/// This is the struct that contains Entities for the application
pub struct PackedEntities {
    /// This is a 'packed/dense' array of all entities for the current application. This array will
    /// be kept sorted using different algorithms and principles
    entities: Vec<Entity>,

    /// This 'groups' array represents the distribution and separation of entities inside the packed
    /// array from above. Each global group is a composition of ordered nested groups
    groups: Vec<                //  Global Groups
        (usize, Vec<usize>),    //  (starting index, Nested Group)
    >,

    /// This 'indices' array provides a way to track entities of a group inside de 'packed/dense' array
    indices: Vec<HashMap<Entity, usize>>,

    /// This map associates a Group with its index in the 'groups' (=Global Group) array from above
    /// and the in_index of the corresponding Nested Group
    map: HashMap<Group, (usize, usize)>,
}

impl PackedEntities {
    /// Create an empty PackedEntities # TODO : mapping
    pub fn new() -> Self {
        let mut groups = Vec::<(usize, Vec<usize>)>::new();
        let mut indices = Vec::<HashMap<Entity, usize>>::new();
        let mut map = HashMap::<Group, (usize, usize)>::new();

        groups.push((0, vec![0, 0]));
        groups.push((0, vec![0]));

        indices.push(HashMap::new());
        indices.push(HashMap::new());

        map.insert(0, (0, 0));
        map.insert(1, (0, 1));
        map.insert(2, (1, 0));

        return Self {
            entities: Vec::new(),
            groups: groups,
            indices: indices,
            map: map,
        };
    }

    pub fn array(&self) -> &[Entity] {
        return &self.entities;
    }

    /// If the group is correctly mapped, it will return the slice of the 'packed/dense' entities array
    /// that contains all entities of this group
    pub fn view(&self, group: Group) -> Option<&[Entity]> {
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => {
                match self.groups.get(index) {
                    Some((start, nested_groups)) => {
                        match nested_groups.get(in_index).cloned() {
                            Some(count) => Some(&self.entities[(*start)..((*start) + count)]),
                            None => {
                                log::warn!("Trying to view group {}, was mapped but nested group wasn't in the storage", group);
                                None
                            }
                        }
                    }
                    None => {
                        log::warn!("Trying to view group {}, was mapped but wasn't in the storage", group);
                        None
                    }
                }
            }
            None => {
                log::warn!("Trying to view group {}, but wasn't mapped", group);
                None
            }
        };
    }

    /// This method will check if the desired 'entity' is present in the global-group corresponding to
    /// the 'group' desired.
    fn is_in_global_group(&self, group: Group, entity: &Entity) -> bool {
        return match self.map.get(&group).cloned() {
            Some((index, _)) => {
                match self.indices.get(index) {
                    Some(indices) => indices.contains_key(entity),
                    None => false
                }
            }
            None => false
        };
    }

    /// This method will check if the desired 'entity' is present only in the global-group corresponding
    /// to the 'group', and not in nested-groups
    fn is_only_in_global_group(&self, group: Group, entity: &Entity) -> bool {
        return match self.map.get(&group).cloned() {
            Some((index, _)) => match self.groups.get(index) {
                Some((start, nested_groups)) => match self.indices.get(index) {
                    Some(indices) => match indices.get(entity).copied() {
                        Some(entity_index) => match nested_groups.last().copied() {
                            Some(last_count) => start.clone() + last_count <= entity_index,
                            None => false
                        }
                        None => false
                    }
                    None => false
                }
                None => false
            }
            None => false
        };
    }

    /// This method will add desired entities to the global group if they are not in this global-group
    fn add_to_global_group(&mut self, group: Group, entities: &[Entity]) {
        if let Some((index, _)) = self.map.get(&group).cloned() {
            for &entity in entities {
                // If the entity is not in the global-group of 'group', we will add it to it
                if !self.is_in_global_group(group, &entity) {
                    // In order to add the entity at the end of the global-group, we need to cross all
                    // groups from the end
                    let groups_to_cross = (index + 1)..self.groups.len();

                    // Lets push the entity at the end of the 'packed/dense' array
                    let mut entity_index = self.entities.len();
                    self.entities.push(entity);

                    // Cross all global-groups groups till global-group of 'group'
                    for i in groups_to_cross.rev() {
                        if let Some((start, nested_groups)) = self.groups.get_mut(i) {
                            for nested_group_count in nested_groups.iter().rev() {
                                let next_index = start.clone() + nested_group_count;

                                self.entities.swap(entity_index, next_index);
                                entity_index = next_index;
                            }

                            // Now add a padding of 1 for this global-group
                            *start += 1;
                        } else {
                            log::error!("Crossing groups is not possible due to a range error. The array can no longer be considered as sorted");
                        }
                    }

                    // Add the entity to the global-group : but it will not belong to any nested-groups
                    if let Some(indices) = self.indices.get_mut(index) {
                        indices.insert(entity, entity_index);
                    } else {
                        log::error!("Adding entity {} to the global group of {} failed due to a range error for indices. The array can no longer be considered as sorted", entity, group);
                    }
                }
            }
        } else {
            log::warn!("Trying to add entities to global-group of group {}, but group wasn't mapped", group);
        }
    }

    /// This method takes a set of entities to add to a certain group. For each entity, it will move
    /// the entity inside the nested groups if this entity already was in the global group, or it will
    /// add it to the global group before moving it
    pub fn push(&mut self, group: Group, entities: &[Entity]) {
        // First, add all entities to global_group if they are not in it
        self.add_to_global_group(group, entities);

        if let Some((index, in_index)) = self.map.get(&group).cloned() {
            if let Some((start, nested_groups)) = self.groups.get_mut(index) {
                if let Some(indices) = self.indices.get_mut(index) {
                    let end = nested_groups.len();

                    for nested_group_count in nested_groups[in_index..end].iter_mut().rev() {
                        for entity in entities {
                            let next_index = start.clone() + nested_group_count.clone();

                            if let Some(entity_index) = indices.get_mut(entity) {

                                // Check if the entity is not in the group yet
                                if entity_index.clone() >= next_index {

                                    // Swap the entity at the end of the nested-group, and extend the
                                    // nested group (do not forget to update entity_index)
                                    self.entities.swap(entity_index.clone(), next_index);
                                    *entity_index = next_index;
                                    *nested_group_count += 1;
                                }
                            } else {
                                log::error!("Adding entity {} failed because entity was not mapped in indices for the group {}. The array can no longer be considered as sorted", entity, group);
                            }
                        }
                    }
                } else {
                    log::error!("Adding entities to group {} failed due to a range error for indices. The array can no longer be considered as sorted", group);
                }
            } else {
                log::error!("Crossing groups is not possible due to a range error. The array can no longer be considered as sorted");
            }
        } else {
            log::warn!("Trying to add entities to group {}, but group wasn't mapped", group);
        }
    }

    /// This method takes a set of entities that belongs to a global-group, and check if those entities
    /// are at the end of the global-group (outside every nested groups) and then remove them by
    /// crossing every groups
    fn remove_from_global_group(&mut self, group: Group, entities: &[Entity]) {
        if let Some((index, _)) = self.map.get(&group).cloned() {
            for entity in entities {
                if self.is_only_in_global_group(group, &entity) {
                    if let Some(indices) = self.indices.get(index) {
                        if let Some(mut entity_index) = indices.get(entity).cloned() {
                            let groups_to_cross = (index + 1)..self.groups.len();

                            for i in groups_to_cross {
                                if let Some((start, nested_groups)) = self.groups.get_mut(i) {
                                    for nested_group_count in nested_groups {
                                        let next_index = start.clone() + nested_group_count.clone();

                                        self.entities.swap(entity_index, next_index - 1);
                                        entity_index = next_index - 1;
                                    }

                                    *start -= 1;
                                } else {
                                    log::error!("Crossing groups is not possible due to a range error. The array can no longer be considered as sorted");
                                }
                            }

                            self.entities.swap_remove(entity_index);

                            // Remove the entity
                            if let Some(indices) = self.indices.get_mut(index) {
                                indices.remove(entity);
                            } else {
                                log::error!("Removing entity {} to the global group of {} failed due to a range error for indices. The array can no longer be considered as sorted", entity, group);
                            }
                        } else {
                            log::error!("Removing entity {} to the global group of {} failed due to a range error for indices. The array can no longer be considered as sorted", entity, group);
                        }
                    } else {
                        log::error!("Removing entities to global group {} failed due to a range error for indices. The array can no longer be considered as sorted", group);
                    }
                }
            }
        } else {
            log::warn!("Trying to remove entities to group {}, but group wasn't mapped", group);
        }
    }

    /// This method takes a set of entities to remove from a certain group. For each entity, it will
    /// move desired entities outside the group and if it becomes outside all nested groups, it will
    /// delete it by moving it at the end of the 'packed/dense' array
    pub fn remove(&mut self, group: Group, entities: &[Entity]) {
        if let Some((index, in_index)) = self.map.get(&group).cloned() {
            if let Some((start, nested_groups)) = self.groups.get_mut(index) {
                if let Some(indices) = self.indices.get_mut(index) {
                    for nested_group_count in &mut nested_groups[0..(in_index + 1)] {
                        for entity in entities {
                            let next_index = start.clone() + nested_group_count.clone();

                            if let Some(entity_index) = indices.get_mut(entity) {
                                if entity_index.clone() < next_index {
                                    self.entities.swap(entity_index.clone(), next_index - 1);
                                    *entity_index = next_index - 1;
                                    *nested_group_count -= 1;
                                }
                            } else {
                                log::warn!("Trying to remove entity {} from group {} failed because entity was not mapped for this group", entity, group);
                            }
                        }
                    }
                } else {
                    log::error!("Removing entities to group {} failed due to a range error for indices. The array can no longer be considered as sorted", group);
                }
            } else {
                log::error!("Crossing groups is not possible due to a range error. The array can no longer be considered as sorted");
            }
        } else {
            log::warn!("Trying to remove entities to group {}, but group wasn't mapped", group);
        }

        // Then remove all entities that are outside nested groups of the global group
        self.remove_from_global_group(group, entities);
    }
}