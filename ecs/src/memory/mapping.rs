/*
    This module provides everything to correctly map the memory for entities and systems. It allows
    fast iterations over entities with selected components
*/

use std::{
    cmp::Ordering,
    collections::{
        HashMap, VecDeque
    }
};

use crate::core::component::{
    Component,
    Group,
    components_to_group
};

use crate::memory::graph::BipartiteGroupsGraph;

pub type MemoryMappingDescriptor = Vec::<Vec<Component>>;

pub struct MemoryMapping {
    // the list of elements to map
    descriptor: MemoryMappingDescriptor,

    // contains the value associated for each cursor (usize here)
    cursor: Vec<Vec<usize>>,

    // for each key (u128/Group), contains the index of the container that contains the cursor, and its index in this container
    mapping: HashMap<Group, (usize, usize)>,
}

impl MemoryMapping {
    pub fn new(mut descriptor: MemoryMappingDescriptor) -> Self {
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
            let a = components_to_group(descriptor.get(i).unwrap()) as i128;

            for j in 0..i {
                let b = components_to_group(descriptor.get(j).unwrap()) as i128;

                if b_contains_a(descriptor.get(j).unwrap(), descriptor.get(i).unwrap()) {
                    graph.add_edge(b, -a);
                }
            }
        }

        graph.compute();

        /* Now read the output of the graph and create the correct mapping */

        let mut containers: Vec<Vec<usize>> = Vec::new();
        let mut mapping: HashMap<Group, (usize, usize)> = HashMap::new();

        for (u, v) in graph.layer_one.clone() {

            // First we create a "branch" that contains all elements, from u to None

            let mut branch: VecDeque<Group> = VecDeque::new();
            let mut current = u as Group;
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
            cursor: containers,
            mapping: mapping,
        }
    }

    pub fn map_and_sort(&self, groups: &Vec<Group>) -> HashMap<usize, Vec<usize>> {
        let mut result = HashMap::new();

        for group in groups {
            let (index, in_index) = self.mapping.get(group).unwrap().clone();
            if !result.contains_key(&index) {
                result.insert(index, Vec::new());
            }

            result.get_mut(&index).unwrap().push(in_index);
        }

        for value in result.values_mut() {
            value.sort_unstable();
        }

        return result;
    }

    pub fn cursors(&self, container: usize) -> &Vec<usize> {
        &self.cursor.get(container).unwrap()
    }

    pub fn cursor(&self, container: usize, index: usize) -> usize {
        self.cursor.get(container).unwrap().get(index).unwrap().clone()
    }

    pub fn search_for(&self, group: Group) -> (usize, usize) {
        let (container, index) = self.mapping.get(&group).unwrap().clone();

        return (container, self.cursor(container, index));
    }

    pub fn advance_cursor(&mut self, container: usize, index: usize) {
        (*self.cursor.get_mut(container).unwrap().get_mut(index).unwrap()) += 1;
    }

    pub fn move_back_cursor(&mut self, container: usize, index: usize) {
        (*self.cursor.get_mut(container).unwrap().get_mut(index).unwrap()) -= 1;
    }

    pub fn len(&self) -> usize {
        self.cursor.len()
    }

    pub fn descriptor(&self) -> &MemoryMappingDescriptor {
        &self.descriptor
    }
}