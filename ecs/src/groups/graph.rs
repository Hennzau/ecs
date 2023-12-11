use std::collections::{HashMap, HashSet, VecDeque};

pub struct BipartiteGroupsGraph {
    // First layer of the bipartite graph and its calculated vertex of the second layer
    pub layer_one: HashMap<i128, Option<i128>>,
    // Second layer of the bipartite graph and its calculated vertex of the first layer
    pub layer_two: HashMap<i128, Option<i128>>,

    // Neighbours from layer two, of the layer one
    pub layer_one_neighbours: HashMap<i128, Vec<i128>>,

    // Distances
    pub distances: HashMap<i128, Option<u32>>
}

impl BipartiteGroupsGraph {
    pub fn new() -> Self {
        Self {
            layer_one: HashMap::new(),
            layer_two: HashMap::new(),
            layer_one_neighbours: HashMap::new(),
            distances: HashMap::new()
        }
    }

    // Add an edge for the bipartite graph (from the layer one and the layer two)
    pub fn add_edge(&mut self, a: i128, b: i128) {
        if !self.layer_one_neighbours.contains_key(&a) {
            self.layer_one_neighbours.insert(a, Vec::new());
        }

        self.layer_one_neighbours.get_mut(&a).unwrap().push(b);
    }

    // BFS algorithm that computes distances of the graph

    fn compute_distances(&mut self) -> u32 {
        let mut queue = VecDeque::<i128>::new();

        for (&vertex, pair) in &self.layer_one {
            if pair.is_none() {
                *self.distances.get_mut(&vertex).unwrap() = Some(0);

                queue.push_back(vertex);
            } else {
                *self.distances.get_mut(&vertex).unwrap() = None;
            }
        }

        let mut nil = u32::MAX;
        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();

            if self.distances.get(&vertex).unwrap().is_some_and(|x| x < nil) {
                for paired in self.layer_one_neighbours.get(&vertex).unwrap() {
                    match self.layer_two.get(&paired).unwrap() {
                        Some(v) => {
                            if self.distances.get(v).unwrap().is_none() {
                                *self.distances.get_mut(v).unwrap() = Some(self.distances.get(&vertex).unwrap().unwrap() + 1);

                                queue.push_back(*v);
                            }
                        },
                        None => {
                            if nil == u32::MAX {
                                nil = self.distances.get(&vertex).unwrap().unwrap() + 1;
                            }
                        }
                    }
                }
            }
        }

        return nil;
    }

    // DFS algorithm that computes the matching

    fn compute_matching(&mut self, vertex: Option<i128>, dist_nil: u32) -> bool {
        if vertex.is_some() {
            for &paired in self.layer_one_neighbours.get(&vertex.unwrap()).unwrap() {
                match self.layer_two.get(&paired).unwrap() {
                    Some(v) => {},
                    None => {
                        match self.distances.get(&vertex.unwrap()).unwrap() {
                            Some(u) => {},
                            None => {
                                if dist_nil == u32::MAX {
                                    *self.layer_one.get_mut(&vertex.unwrap()).unwrap() = Some(paired);
                                }
                            }
                        }
                    }
                }
            }
        }

        return true;
    }

    pub fn compute(&mut self) {
        for (&vertex, neighbours) in &self.layer_one_neighbours {
            self.layer_one.insert(vertex, None);
            self.layer_two.insert(-vertex, None);
            self.distances.insert(vertex, None);
            self.distances.insert(-vertex, None);
        }

        loop {
            let dist_nil = self.compute_distances();
            if dist_nil == u32::MAX {
                break;
            }

            for (vertex, paired) in self.layer_one.clone() {
                if paired.is_none() {
                    self.compute_matching(Some(vertex), dist_nil);
                }
            }
        }
    }
}