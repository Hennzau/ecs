use std::collections::HashMap;
use std::fmt::Error;

use crate::core::
{
    entity::Entity,
    component::Group,
};

pub struct Entities {
    entities: Vec<Vec<Entity>>,

    groups: Vec<Vec<usize>>,

    indices: Vec<HashMap<Entity, usize>>,

    map: HashMap<Group, (usize, usize)>,
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

    fn extract_entities(&self, group: Group, entities: &[Entity]) -> (Vec<Entity>, Vec<Entity>) {
        return self.map.get(&group).cloned().map_or_else(|| {
            log::warn!("Trying to extract entities from {:?} for group {}, but group wasn't mapped", entities, group);

            (Vec::new(), Vec::new())
        }, |(index, _)| self.indices.get(index).map_or_else(|| {
            log::warn!("Trying to extract entities from {:?} for group {}, range error for indices", entities, group);

            (Vec::new(), Vec::new())
        }, |indices| {
            let mut old = Vec::with_capacity(entities.len());
            let mut new = Vec::with_capacity(entities.len());

            for entity in entities {
                if !indices.contains_key(entity) {
                    new.push(entity.clone());
                } else {
                    old.push(entity.clone());
                }
            }

            (old, new)
        }));
    }

    pub fn try_add_group(&mut self, group: Group, entities: &[Entity]) -> Result<(), Error> {
        if let Some((index, in_index)) = self.map.get(&group).cloned() {
            if let Some(indices) = self.indices.get_mut(index) {
                if let Some(groups) = self.groups.get_mut(index) {
                    if let Some(array) = self.entities.get_mut(index) {
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
                                        array.swap(entity_index.clone(), nested.clone());

                                        *entity_index = nested.clone();
                                        *nested += 1;
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
                    }
                }
            }
        }

        return Result::Ok(());
    }

    pub fn add_group(&mut self, group: Group, entities: &[Entity]) {
        if let Some((index, in_index)) = self.map.get(&group).cloned() {
            let (old_entities, mut new_entities) = self.extract_entities(group, entities);

            log::info!("{:?}", old_entities);

            if let Some(groups) = self.groups.get_mut(index) {
                if let Some(array) = self.entities.get_mut(index) {
                    let new_entities_count = new_entities.len();

                    array.append(&mut new_entities);

                    let mut to_swap = Vec::<(usize, usize)>::new();
                    let (_, groups_to_cross) = groups.split_at_mut(in_index);

                    let mut last_index = array.len() - new_entities_count;
                    for nested in groups_to_cross.iter_mut().rev() {
                        for entity in &old_entities {
                            if let Some(Some(entity_index)) = self.indices.get_mut(index).map(|indices| indices.get_mut(entity)) {
                                if entity_index.clone() >= nested.clone() { // Entity is not in the group
                                    array.swap(entity_index.clone(), nested.clone());

                                    *entity_index = nested.clone();
                                    *nested += 1;
                                }
                            }
                        }

                        let space = last_index - nested.clone();
                        to_swap.push((nested.clone(), space));
                        last_index -= space;

                        *nested += new_entities_count;
                    }

                    for (index, space) in to_swap.iter().rev().cloned() {
                        let (before, after) = array.split_at_mut(index + space);

                        if space >= new_entities_count {
                            if let Some(entities) = after.get_mut(0..new_entities_count) {
                                if let Some(slice) = before.get_mut(index..(index + new_entities_count)) {
                                    entities.swap_with_slice(slice);
                                }
                            }
                        } else {
                            if let Some(entities) = after.get_mut((new_entities_count - space)..(new_entities_count - space + space)) {
                                if let Some(slice) = before.get_mut(index..(index + space)) {
                                    entities.swap_with_slice(slice);
                                }
                            }
                        }
                    }
                }
            } else {
                log::warn!("You tried to add entities {:?} to group {}, range error for groups : this group was not mapped correctly", entities, group);
            }
        } else {
            log::warn!("You tried to add entities {:?} to group {}, but this group wasn't mapped", entities, group);
        }
    }
}