use std::collections::{HashMap, HashSet};

pub struct Graph {
    // Key = Vertex, Value = Neighbours
    pub vertices: HashMap<i128, Vec<i128>>, // len p

    // edge A -> B with capacity (C (0 or 1), and flow D)
    pub edges: HashMap<(i128, i128), (u8, u8)>, // len q
}

pub struct Residual {
    pub vertices: HashMap<i128, Vec<i128>>,
    pub edges: HashMap<(i128, i128), u8>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, value: i128) {
        self.vertices.insert(value, Vec::new());
    }

    pub fn add_edge(&mut self, a: i128, b: i128, c: bool) {
        match self.vertices.get_mut(&a) {
            Some(neighbours) => {
                neighbours.push(b);

                if !self.edges.contains_key(&(a, b)) {
                    if c {
                        self.edges.insert((a, b), (1, 0));
                    } else {
                        self.edges.insert((a, b), (0, 0));
                    }
                }
            }
            None => {}
        }
    }

    pub fn update_capacity(&mut self, a: i128, b: i128, c: bool) {
        let edge = self.edges.get_mut(&(a, b));
        if edge.is_some() {
            if c {
                *edge.unwrap() = (1, 0);
            } else {
                *edge.unwrap() = (0, 0);
            }
        }
    }

    pub fn construct_residual(&self) -> Residual {
        let mut residual = Residual::new();

        for ((a, b), (c, f)) in self.edges {
            if !residual.vertices.contains_key(a) {
                residual.vertices.insert(a, Vec::new());
            }

            if !residual.vertices.contains_key(b) {
                residual.vertices.insert(b, Vec::new());
            }

            if c == 1 && f == 0 {
                residual.vertices.get_mut(a).unwrap().push(b);
                residual.edges.insert((a, b), c);
                v
            } else if c == 1 && f == 1 {
                residual.vertices.get_mut(b).unwrap().push(a);
                residual.edges.insert((b, a), c);
            }
        }

        return residual;
    }   // O(q)
}

impl Residual {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }


}