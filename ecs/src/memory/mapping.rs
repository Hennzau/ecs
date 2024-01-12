use std::cmp::Ordering;
/// This module manages memory mapping to generate the appropriate Entities storage
/// based on the user's chosen set of components.

/// This mapping principle was conceived by Genouville Grégoire, Bianchi Bérénice, and Le Van Enzo.
/// It revolves around creating a specialized bipartite graph and employing the Hopcroft-Karp algorithm
/// to create an optimized mapping for PackedEntities.

/// The Hopcroft-Karp algorithm, initially recursive, aims to be transformed into an iterative approach.
/// Referencing: https://www.baeldung.com/cs/convert-recursion-to-iteration

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    core::component::{
        ComponentID,
        Group,
        components_to_group,
    },
    memory::entities::Entities,
};

/// This type allows you to specify to the memory mapper the set of components you intend to use for your systems.
pub type MemoryMappingDescriptor = Vec<HashSet<ComponentID>>;

type iGroup = i64;

const INFTY: usize = usize::MAX;

/// This struct represents the memory mapper
pub struct MemoryMapping {
    /// Represents the set of components you intend to use for your systems.
    pub descriptor: MemoryMappingDescriptor,

    /// Represents the first layer of the bipartite graph along with its corresponding calculated vertices in the second layer.
    pub layer_one: HashMap<Group, Option<iGroup>>,

    /// Represents the first second of the bipartite graph along with its corresponding calculated vertices in the first layer.
    pub layer_two: HashMap<iGroup, Option<Group>>,

    /// Describes neighbors in layer two corresponding to vertices in layer one.
    pub layer_one_neighbors: HashMap<Group, Vec<iGroup>>,

    /// Distances
    pub distances: HashMap<Option<iGroup>, usize>,
}

impl MemoryMapping {
    pub fn new(mut descriptor: MemoryMappingDescriptor) -> MemoryMapping {
        fn b_strictly_contains_a(a: &HashSet<ComponentID>, b: &HashSet<ComponentID>) -> bool {
            return a != b && a.is_subset(b);
        }

        let mut layer_one = HashMap::new();
        let mut layer_two = HashMap::new();
        let mut layer_one_neighbors = HashMap::new();
        let mut distances = HashMap::new();

        descriptor.sort_unstable_by(|a, b| match a.len() > b.len() {
            true => Ordering::Greater,
            false => Ordering::Less,
        });

        for components_1 in &descriptor {
            let group_a = components_to_group(components_1);
            let igroup_a = -(group_a as iGroup);

            if !layer_one.contains_key(&group_a) {
                layer_one.insert(group_a, None);
                distances.insert(Some(group_a as iGroup), INFTY);
            }

            if !layer_two.contains_key(&igroup_a) {
                layer_two.insert(igroup_a, None);
                distances.insert(Some(igroup_a), INFTY);
            }

            if !layer_one_neighbors.contains_key(&group_a) {
                layer_one_neighbors.insert(group_a, vec![igroup_a]);
            }

            for components_2 in &descriptor {
                let group_b = components_to_group(components_2) as Group;

                if b_strictly_contains_a(components_1, components_2) {
                    if !layer_one.contains_key(&group_b) {
                        layer_one.insert(group_b, None);
                        distances.insert(Some(group_b as iGroup), INFTY);
                    }

                    if !layer_two.contains_key(&igroup_a) {
                        layer_two.insert(igroup_a, None);
                        distances.insert(Some(igroup_a), INFTY);
                    }

                    if let Some(neighbors) = layer_one_neighbors.get_mut(&group_b) {
                        neighbors.push(igroup_a);
                    } else {
                        layer_one_neighbors.insert(group_b, vec![igroup_a]);
                    }
                }
            }
        }

        // TODO: Compute graph

        return Self {
            descriptor: descriptor,
            layer_one: layer_one,
            layer_two: layer_two,
            layer_one_neighbors: layer_one_neighbors,
            distances: distances,
        };
    }

    /// Generates the corresponding Entities storage based on the description and the mapping obtained from the Hopcroft-Karp algorithm.
    pub fn create_storage(&self) -> Entities {
        let mut groups = Vec::new();
        let mut mapping = HashMap::new();

        let mut temp = Vec::<VecDeque<Group>>::new();
        let mut indices = HashMap::<Group, (usize, (Group, Group))>::new();

        // TODO : build temp and indices

        for (u, v) in &self.layer_one {

        }

        for queue in temp {
            groups.push(Vec::new());

            if let Some(last) = groups.last_mut() {
                for group in queue {
                    last.push(group);
                }
            }
        }

        for (group, (index, (_, _))) in indices {
            indices.insert(group, index);
        }

        return Entities::new(groups, mapping);
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