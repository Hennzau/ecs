use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    core::component::{
        ComponentID,
        Group,
        components_to_group
    },
    memory::entities::Entities,
};

/// This module manages memory mapping to generate the appropriate Entities storage
/// based on the user's chosen set of components.

/// This mapping principle was conceived by Genouville Grégoire, Bianchi Bérénice, and Le Van Enzo.
/// It revolves around creating a specialized bipartite graph and employing the Hopcroft-Karp algorithm
/// to create an optimized mapping for PackedEntities.

/// The Hopcroft-Karp algorithm, initially recursive, aims to be transformed into an iterative approach.
/// Referencing: https://www.baeldung.com/cs/convert-recursion-to-iteration

pub type MemoryMappingDescriptor = Vec<Vec<ComponentID>>;

pub struct MemoryMapping {
    descriptor: MemoryMappingDescriptor,
}

impl MemoryMapping {
    pub fn new(descriptor: MemoryMappingDescriptor) -> MemoryMapping {
        return Self {
            descriptor: descriptor
        };
    }

    pub fn create_storage(&self) -> Entities {
        return Entities::new(Vec::new(), HashMap::new());
    }

    pub fn get_next_membership(&self, previous_components: &HashSet<ComponentID>, components_to_add: &HashSet<ComponentID>) -> HashSet<Group> {
        let mut previous_groups = HashSet::<Group>::new();
        let mut new_groups = HashSet::<Group>::new();

        for group in &self.descriptor {
            if group.iter().all(|x| previous_components.contains(x)) {
                previous_groups.insert(components_to_group(group));
            }

            if group.iter().all(|x| previous_components.contains(x) || components_to_add.contains(x)) {
                new_groups.insert(components_to_group(group));
            }
        }

        return new_groups.symmetric_difference(&previous_groups).cloned().collect();
    }
}