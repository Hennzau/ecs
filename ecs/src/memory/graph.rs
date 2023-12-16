use std::collections::{
    HashMap,
    VecDeque
};

pub struct BipartiteGroupsGraph {
    // First layer of the bipartite graph and its calculated vertex of the second layer
    pub layer_one: HashMap<i128, Option<i128>>,

    // Second layer of the bipartite graph and its calculated vertex of the first layer
    pub layer_two: HashMap<i128, Option<i128>>,

    // Neighbours from layer two, of the layer one
    pub layer_one_neighbours: HashMap<i128, Vec<i128>>,

    // Distances
    pub distances: HashMap<Option<i128>, u32>,
}

impl BipartiteGroupsGraph {
    pub fn new() -> Self {
        Self {
            layer_one: HashMap::new(),
            layer_two: HashMap::new(),
            layer_one_neighbours: HashMap::new(),
            distances: HashMap::new(),
        }
    }

    // Add an edge for the bipartite graph (from the layer one and the layer two)
    pub fn add_edge(&mut self, a: i128, b: i128) {
        if !self.layer_one_neighbours.contains_key(&a) {
            self.layer_one_neighbours.insert(a, Vec::new());
        }

        self.layer_one_neighbours.get_mut(&a).unwrap().push(b);

        if !self.layer_one.contains_key(&a) {
            self.layer_one.insert(a, None);
            self.distances.insert(Some(a), u32::MAX);
        }

        if !self.layer_two.contains_key(&b) {
            self.layer_two.insert(b, None);
            self.distances.insert(Some(b), u32::MAX);
        }
    }

    // BFS algorithm that computes distances of the graph

    fn compute_distances(&mut self) -> bool {
        let mut queue = VecDeque::<Option<i128>>::new();

        for (&vertex, pair) in &self.layer_one {
            if pair.is_none() {
                *self.distances.get_mut(&Some(vertex)).unwrap() = 0;

                queue.push_back(Some(vertex));
            } else {
                *self.distances.get_mut(&Some(vertex)).unwrap() = u32::MAX;
            }
        }

        *self.distances.get_mut(&None).unwrap() = u32::MAX;

        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();
            let dist_u = self.distances.get(&vertex).unwrap().clone();

            if dist_u < self.distances.get(&None).unwrap().clone() {
                // if vertex was None we would not be there

                for &v in self.layer_one_neighbours.get(&vertex.unwrap()).unwrap() {
                    let pair_v = self.layer_two.get(&v).unwrap();
                    let dist_pair_v = self.distances.get_mut(pair_v).unwrap();

                    if dist_pair_v.clone() == u32::MAX {
                        *dist_pair_v = dist_u + 1;
                        queue.push_back(*pair_v);
                    }
                }
            }
        }

        return self.distances.get(&None).unwrap().clone() < u32::MAX;
    }

    fn compute_matching(&mut self, vertex: Option<i128>) -> bool {
        if !vertex.is_none() {
            for v in self.layer_one_neighbours.get(&vertex.unwrap()).unwrap().clone() {
                let pair_v = self.layer_two.get_mut(&v).unwrap().clone();
                if self.distances.get(&vertex).unwrap().clone() == u32::MAX {
                    if self.distances.get(&pair_v).unwrap().clone() == u32::MAX {
                        if self.compute_matching(pair_v) {
                            *self.layer_two.get_mut(&v).unwrap() = vertex;
                            *self.layer_one.get_mut(&vertex.unwrap()).unwrap() = Some(v.clone());

                            return true;
                        }
                    }
                } else {
                    if self.distances.get(&pair_v).unwrap().clone() == self.distances.get(&vertex).unwrap().clone() + 1 {
                        if self.compute_matching(pair_v) {
                            *self.layer_two.get_mut(&v).unwrap() = vertex;
                            *self.layer_one.get_mut(&vertex.unwrap()).unwrap() = Some(v.clone());

                            return true;
                        }
                    }
                }
            }

            *self.distances.get_mut(&vertex).unwrap() = u32::MAX;
            return false;
        }

        return true;
    }

    pub fn compute(&mut self) {
        self.distances.insert(None, u32::MAX);

        loop {
            let dist_nil = self.compute_distances();
            if !dist_nil {
                break;
            }

            for (u, paired) in self.layer_one.clone() {
                if paired.is_none() {
                    self.compute_matching(Some(u));
                }
            }
        }
    }
}