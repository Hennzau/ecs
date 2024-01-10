use std::cmp::Ordering;
/// This module manages memory mapping to generate the appropriate Entities storage
/// based on the user's chosen set of components.

/// This mapping principle was conceived by Genouville Grégoire, Bianchi Bérénice, and Le Van Enzo.
/// It revolves around creating a specialized bipartite graph and employing the Hopcroft-Karp algorithm
/// to create an optimized mapping for PackedEntities.

/// The Hopcroft-Karp algorithm, initially recursive, aims to be transformed into an iterative approach.
/// Referencing: https://www.baeldung.com/cs/convert-recursion-to-iteration

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

/// This type allows you to specify to the memory mapper the set of components you intend to use for your systems.
pub type MemoryMappingDescriptor = Vec<Vec<ComponentID>>;

type iGroup = i64;

const INFTY: usize = usize::MAX;

/// This struct represents the memory mapper
pub struct MemoryMapping {
    /// Represents the set of components you intend to use for your systems.
    pub descriptor: MemoryMappingDescriptor,

    /// Represents the first layer of the bipartite graph along with its corresponding calculated vertices in the second layer.
    pub layer_one: HashMap<iGroup, Option<iGroup>>,

    /// Represents the first second of the bipartite graph along with its corresponding calculated vertices in the first layer.
    pub layer_two: HashMap<iGroup, Option<iGroup>>,

    /// Describes neighbors in layer two corresponding to vertices in layer one.
    pub layer_one_neighbors: HashMap<iGroup, Vec<iGroup>>,

    /// Distances
    pub distances: HashMap<Option<iGroup>, usize>,
}

impl MemoryMapping {
    pub fn new(mut descriptor: MemoryMappingDescriptor) -> MemoryMapping {
        descriptor.sort_unstable_by(|a, b| match a.len() > b.len() {
            true => Ordering::Greater,
            false => Ordering::Less,
        });

        for i in 0..descriptor.len() {

            for j in 0..i {

            }
        }

        return Self {
            descriptor: descriptor,
            layer_one: HashMap::new(),
            layer_two: HashMap::new(),
            layer_one_neighbors: HashMap::new(),
            distances: HashMap::new(),
        };
    }

    /// Generates the corresponding Entities storage based on the description and the mapping obtained from the Hopcroft-Karp algorithm.
    pub fn create_storage(&self) -> Entities {
        return Entities::new(Vec::new(), HashMap::new());
    }

    /// Calculates the group to which an entity belongs when adding additional components to it, given its previous set of components.
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