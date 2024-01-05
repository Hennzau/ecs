use std::collections::HashMap;

/// Represents the distribution of the memory inside the flat storage
pub struct LayerOneDistribution {
    /// List of (start, count) for each group
    pub groups: Vec<(usize, usize)>,
    pub map: HashMap<u128, usize>,
}

pub struct LayerTwoDistribution {
    /// List of (start, List (count)) for each global groups
    pub groups: Vec<
        (usize, Vec<usize>)
    >,

    pub map: HashMap<u128, (usize, usize)>,
}

pub enum Distribution {
    LayerOne(LayerOneDistribution),
    LayerTwo(LayerTwoDistribution),
}

impl Distribution {
    pub fn index(&self, group: u128) -> Option<usize> {
        return match self {
            Distribution::LayerOne(distribution) => distribution.map.get(&group).cloned(),
            Distribution::LayerTwo(distribution) => return match distribution.map.get(&group).cloned() {
                Some((index, i_)) => Some(index),
                None => None
            }
        };
    }
}

pub struct FlatStorage<T> {
    distribution: Distribution,
    array: Vec<T>,
    indices: Vec<HashMap<u64, usize>>,
}

impl<T> FlatStorage<T> {
    pub fn new(distribution: Distribution) -> Self {
        return Self {
            distribution: distribution,
            array: Vec::new(),
            indices: Vec::new(),
        };
    }

    pub fn array(&self) -> &[T] {
        return &self.array;
    }

    pub fn view(&self, group: u128) -> Option<&[T]> {
        return match &self.distribution {
            Distribution::LayerOne(distribution) => match distribution.map.get(&group).cloned() {
                Some(index) => match distribution.groups.get(index) {
                    Some((start, count)) => Some(&self.array[(*start)..((*start) + count)]),
                    None => {
                        log::warn!("Trying to view group {}, range error for groups", group);
                        None
                    }
                }
                None => {
                    log::warn!("Trying to view group {}, but wasn't mapped", group);
                    None
                }
            }
            Distribution::LayerTwo(distribution) => match distribution.map.get(&group).cloned() {
                Some((index, in_index)) => match distribution.groups.get(index) {
                    Some((start, nested_groups)) => match nested_groups.get(in_index).cloned() {
                        Some(count) => Some(&self.array[(*start)..((*start) + count)]),
                        None => {
                            log::warn!("Trying to view group {}, was mapped but nested group wasn't in the storage", group);
                            None
                        }
                    }
                    None => {
                        log::warn!("Trying to view group {}, was mapped but wasn't in the storage", group);
                        None
                    }
                }
                None => {
                    log::warn!("Trying to view group {}, but wasn't mapped", group);
                    None
                }
            }
        };
    }

    fn is_in_layer_one(&self, group: u128, id: u64) -> bool {
        return match self.distribution.index(group) {
            Some(index) => match self.indices.get(index) {
                Some(indices) => indices.contains_key(&id),
                None => false
            }
            None => false
        };
    }

    fn is_in_layer_one_and_not_in_layer_two(&self, group: u128, id: u64) -> bool {
        return match self.distribution.index(group) {
            Some(index) => match &self.distribution {
                Distribution::LayerOne(distribution) => match self.indices.get(index) {
                    Some(indices) => indices.contains_key(&id),
                    None => false
                },
                Distribution::LayerTwo(distribution) => match distribution.groups.get(index) {
                    Some((start, nested_groups)) => match self.indices.get(index) {
                        Some(indices) => match indices.get(&id).copied() {
                            Some(id_index) => match nested_groups.last().copied() {
                                Some(last_count) => start.clone() + last_count <= id_index,
                                None => false
                            }
                            None => false
                        }
                        None => false
                    }
                    None => false
                }
            }
            None => false
        };
    }
}