/*
    This module provides everything to correctly map the memory for entities and systems. It allows
    fast iterations over entities with selected components
*/

use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

use crate::memory::graph::BipartiteGroupsGraph;

pub type MemoryMappingDescriptor = Vec::<Vec<u64>>;

pub struct MemoryMapping {
    // the list of elements to map
    descriptor: MemoryMappingDescriptor,

    // contains the value associated for each element (usize here)
    containers: Vec<Vec<usize>>,

    // for each element, contains the index of the container that contains this element, and its index in this container
    mapping: HashMap<u128, (usize, usize)>,
}

impl MemoryMapping {
    pub fn new(mut descriptor: MemoryMappingDescriptor) -> Self {
        fn group_id(set: &Vec<u64>) -> u128 {
            let mut result = 0u128;

            for &id in set {
                result += id as u128;
            }

            return result;
        }

        fn b_contains_a(a: &Vec<u64>, b: &Vec<u64>) -> bool {
            a.iter().all(|x| b.contains(x))
        }

        /* construct the bipartite graph */

        descriptor.sort_unstable_by(|a, b| {
            match a.len() > b.len() {
                true => Ordering::Greater,
                false => Ordering::Less
            }
        });

        let mut graph = BipartiteGroupsGraph::new();

        for i in 0..descriptor.len() {
            let a = group_id(descriptor.get(i).unwrap()) as i128;

            for j in 0..i {
                let b = group_id(descriptor.get(j).unwrap()) as i128;

                if b_contains_a(descriptor.get(j).unwrap(), descriptor.get(i).unwrap()) {
                    graph.add_edge(b, -a);
                }
            }
        }

        graph.compute();

        /* Now read the output of the graph and create the correct mapping */

        let mut containers: Vec<Vec<usize>> = Vec::new();
        let mut mapping: HashMap<u128, (usize, usize)> = HashMap::new();

        for (u, v) in graph.layer_one.clone() {

            // First we create a "branch" that contains all elements, from u to None

            let mut branch: VecDeque<u128> = VecDeque::new();
            let mut current = u as u128;
            let mut next = v;

            branch.push_back(current);

            while next.is_some() {
                current = next.unwrap().abs() as u128;
                next = match graph.layer_one.get(&(current as i128)) {
                    Some(&paired) => paired,
                    None => None
                };

                branch.push_back(current);
            }

            // Now iterate from the back and insert elements in the right container

            let mut index: Option<usize> = None;
            while !branch.is_empty() {
                let current = branch.pop_back().unwrap();

                // Check if the last one is already in the mapping : if so get the index of the container
                // if not create the appropriate container and get its index

                if index.is_none() {
                    if mapping.contains_key(&current) {
                        let (vec_index, _) = mapping.get(&current).unwrap();
                        index = Some(*vec_index);
                    } else {
                        let i = containers.len();
                        containers.push(Vec::new());
                        containers.last_mut().unwrap().push(0);

                        mapping.insert(current, (i, 0));

                        index = Some(i);
                    }

                    continue;
                }

                let index = index.unwrap();
                if !mapping.contains_key(&current) {
                    let container = containers.get_mut(index).unwrap();
                    let in_index = container.len();
                    container.push(0);
                    mapping.insert(current, (index, in_index)); // do not forget to map them
                }
            }
        }

        Self {
            descriptor: descriptor,
            containers: containers,
            mapping: mapping,
        }
    }

    pub fn get_complete_groups_to_update_when_add(&self, groups: &Vec<u128>) -> HashMap<usize, Vec<usize>> {
        let mut map = HashMap::<usize, Vec<usize>>::new();

        for group in groups {
            let (index, in_index) = self.mapping.get(group).unwrap().clone();
            if !map.contains_key(&index) {
                map.insert(index, Vec::new());
            }

            map.get_mut(&index).unwrap().push(in_index);
        }

        for (key, value) in &mut map {
            value.sort_unstable();
        }

        return map;
    }

    pub fn value(&self, container: usize, index: usize) -> usize {
        return self.containers.get(container).unwrap().get(index).unwrap().clone();
    }

    pub fn update_value(&mut self, container: usize, index: usize, value: usize) {
        *self.containers.get_mut(container).unwrap().get_mut(index).unwrap() = value;
    }

    pub fn len(&self) -> usize {
        self.containers.len()
    }

    pub fn descriptor(&self) -> &MemoryMappingDescriptor {
        &self.descriptor
    }
}