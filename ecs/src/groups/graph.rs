use std::collections::{HashMap, HashSet};

pub struct Graph {
    // Key = Vertex, Value = Neighbours
    pub vertices: HashMap<i128, Vec<i128>>,

    // edge A -> B with capacity C (0 or 1)
    pub edges: HashMap<(i128, i128), u8>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new()
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
                        self.edges.insert((a, b), 1);
                    } else {
                        self.edges.insert((a, b), 0);
                    }
                }
            },
            None => {}
        }
    }

    pub fn update_capacity(&mut self, a: i128, b: i128, c: bool) {
        let edge = self.edges.get_mut(&(a, b));
        if edge.is_some() {
            if c {
                *edge.unwrap() = 1;
            } else {
                *edge.unwrap() = 0;
            }
        }
    }
}