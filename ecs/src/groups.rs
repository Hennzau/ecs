/*
This is the main module for stocking entities in different groups, in order to iterate over them efficiently
 */

use std::collections::HashMap;
use crate::entity::Entity;

pub struct Group {
    pub components: Vec<Vec<u64>>,

    sparse: Vec<usize>,
    packed: Vec<Entity>,
    subsets: HashMap<u128, usize>
}

impl Group {
    pub fn new(base: u64) -> Self {
        Self {
            components: vec![vec![base]],

            sparse: Vec::new(),
            packed: Vec::new(),
            subsets: HashMap::from([(base, 0)])
        }
    }

    pub fn contains(&self, set: Vec<u64>) -> bool {
        self.components.iter().any(|components| {
            components.len() == set.len() && components.iter().all(|&x| set.contains(&x))
        })
    }

    pub fn can_be_added(&self, set: Vec<u64>) -> (bool, Option<usize>) {
        if self.components.is_empty() {
            return (false, None);
        }

        if set.len() == self.components.last().unwrap().len() {
            return (false, None);
        }

        if set.len() > self.components.last().unwrap().len() {
            let contained = self.components.last().unwrap().iter().all(|&x| set.contains(&x));

            return (contained, None);
        }

        // Now we are sure that 'self.components' has strictly more than 1 component

        let mut empty_spaces: Vec<usize> = Vec::new();


        for i in 1..self.components.len() {
            let previous_component = self.components.get(i - 1).unwrap();
            let current_component = self.components.get(i).unwrap();

            if previous_component.len() < current_component.len() {
                empty_spaces.push(i);
            }
        }

        for empty in empty_spaces {
            if set.len() > self.components.get((empty) - 1).unwrap().len() && set.len() < self.components.get(empty).unwrap().len() {
                return (true, Some(empty));
            }
        }

        return (false, None);
    }
}