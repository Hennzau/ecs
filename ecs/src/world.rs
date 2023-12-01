use std::collections::{HashMap, HashSet};

use crate::entity::Entity;

pub struct World {
    entities: HashSet<Entity>,

    next: u64
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
            next: 0
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.insert(self.next as Entity);

        self.next += 1;
        self.next - 1
    }

    pub fn alive(&self, entity: &Entity) -> bool {
        self.entities.contains(entity)
    }
}