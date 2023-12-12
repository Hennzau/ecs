/*
    This module provides everything to correctly map the memory for entities and systems. It allows
    fast iterations over entities with selected components
*/

use std::collections::{HashMap, VecDeque};
use crate::groups::graph::BipartiteGroupsGraph;

pub type MemoryMappingDescriptor = Vec::<Vec<u64>>;

pub struct MemoryMapping {
    descriptor: MemoryMappingDescriptor,
    containers: Vec<Vec<usize>>,
    mapping: HashMap<u128, (usize, usize)>
}

impl MemoryMapping {
    pub fn new(descriptor: MemoryMappingDescriptor) -> Self {
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

        for (u, v) in graph.layer_one.clone() {
            let mut branch = VecDeque::new();
            let mut current = u;
            let mut next = v;
            branch.push_back(current);

            while next.is_some() {
                current = next.unwrap().abs();
                next = match graph.layer_one.get(&current) {
                    Some(&paired) => paired,
                    None => None
                };

                branch.push_back(current);
            }
        }

        Self {
            descriptor: descriptor,
            containers: Vec::new(),
            mapping: HashMap::new()
        }
    }
}