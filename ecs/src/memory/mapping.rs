/// This module manages memory mapping to generate the appropriate Entities storage
/// based on the user's chosen set of components.
///
/// This mapping principle was conceived by Genouville Grégoire, Bianchi Bérénice, and Le Van Enzo.
/// It revolves around creating a specialized bipartite graph and employing the Hopcroft-Karp algorithm
/// to create an optimized mapping for PackedEntities.
///
/// The idea is to construct a bipartite graph where each group appears both in the left and right groups.
/// Then, we connect each group on the left to every group on the right that contains it.
/// Finally, we use the Hopcroft-Karp algorithm to determine the minimal bipartite matching.
///
/// The Hopcroft-Karp algorithm, initially recursive, aims to be transformed into an iterative approach.
/// Referencing: https://www.baeldung.com/cs/convert-recursion-to-iteration

use std::collections::VecDeque;
use ahash::{
    AHashMap,
    AHashSet
};

use crate::{
    core::component::{
        ComponentID,
        Group,
        group_id,
    },
    memory::entities::Entities,
};

/// This type allows you to specify to the memory mapper the set of components you intend to use for your systems.
pub type MemoryMappingDescriptor = Vec<AHashSet<ComponentID>>;

type IGroup = i128; // We use i128 because we need to be able to represent -u64

const INFTY: u64 = u64::MAX;

/// This struct represents the memory mapper
pub struct MemoryMapping {
    /// Represents the set of components you intend to use for your systems.
    pub descriptor: MemoryMappingDescriptor,

    /// Represents the first layer of the bipartite graph along with its corresponding calculated vertices in the second layer.
    pub layer_one: AHashMap<Group, Option<IGroup>>,

    /// Represents the first second of the bipartite graph along with its corresponding calculated vertices in the first layer.
    pub layer_two: AHashMap<IGroup, Option<Group>>,

    /// Describes neighbors in layer two corresponding to vertices in layer one.
    pub layer_one_neighbors: AHashMap<Group, Vec<IGroup>>,

    /// Distances of each vertex from the source vertex.
    pub distances: AHashMap<Option<IGroup>, u64>,
}

impl MemoryMapping {
    pub fn new(descriptor: MemoryMappingDescriptor) -> MemoryMapping {
        fn second_strictly_contains_first(first: &AHashSet<ComponentID>, second: &AHashSet<ComponentID>) -> bool {
            return first != second && first.is_subset(second);
        }

        let mut layer_one = AHashMap::new();
        let mut layer_two = AHashMap::new();
        let mut layer_one_neighbors = AHashMap::new();
        let mut distances = AHashMap::new();

        for components_a in &descriptor {
            let group_a = group_id(components_a);
            let igroup_a = -(group_a as IGroup);

            if !layer_one.contains_key(&group_a) {
                layer_one.insert(group_a, None);
                distances.insert(Some(group_a as IGroup), INFTY);
            }

            if !layer_two.contains_key(&igroup_a) {
                layer_two.insert(igroup_a, None);
                distances.insert(Some(igroup_a), INFTY);
            }

            if !layer_one_neighbors.contains_key(&group_a) {
                layer_one_neighbors.insert(group_a, Vec::new());
            }

            for components_b in &descriptor {
                let group_b = group_id(components_b) as Group;

                if second_strictly_contains_first(components_b, components_a) {
                    if !layer_one.contains_key(&group_b) {
                        layer_one.insert(group_b, None);
                        distances.insert(Some(group_b as IGroup), INFTY);
                    }

                    if let Some(neighbors) = layer_one_neighbors.get_mut(&group_b) {
                        neighbors.push(igroup_a);
                    } else {
                        layer_one_neighbors.insert(group_b, vec![igroup_a]);
                    }
                }
            }
        }

        distances.insert(None, INFTY);

        // Now apply Hopcroft-Karp to determine the optimal matching : the result of the matching will be accessible
        // from both layer_one and layer_two maps

        loop {
            if !Self::compute_distances(&layer_one, &layer_two, &layer_one_neighbors, &mut distances) {
                break;
            }

            for (u, paired) in layer_one.clone() {
                if paired.is_none() {
                    Self::compute_matching(Some(u), &mut layer_one, &mut layer_two, &layer_one_neighbors, &mut distances);
                }
            }
        }

        return Self {
            descriptor: descriptor,
            layer_one: layer_one,
            layer_two: layer_two,
            layer_one_neighbors: layer_one_neighbors,
            distances: distances,
        };
    }

    /// Generates the corresponding Entities storage based on the previously generated graph.
    ///
    /// This function first constructs the mapping from the graph and then passes it to the Entities constructor.
    pub fn create_storage(&self) -> Entities {
        let mut groups = Vec::new();
        let mut mapping = AHashMap::new();

        for (u, v) in &self.layer_one {
            if mapping.contains_key(u) { continue; } // If u has already been mapped, juste ignored it

            // Create the list of groups u belongs to : first from u to None, then from u to the first group.
            let mut list = VecDeque::<Group>::from(vec![u.clone()]);

            // Get next group (at the right of layer_one)
            let mut next = v.clone();
            while let Some(icurrent) = next {
                let current = icurrent.abs() as Group;

                list.push_back(current);
                next = match self.layer_one.get(&current) {
                    Some(next_) => next_.clone(),
                    None => None
                };
            }

            // Get previous group (at the left of layer_two)
            let iu = -(u.clone() as IGroup);
            if let Some(t) = self.layer_two.get(&iu) {
                let mut previous = t.clone();
                while let Some(current) = previous {
                    let icurrent = -(current as IGroup);

                    list.push_front(current);
                    previous = match self.layer_two.get(&icurrent) {
                        Some(previous_) => previous_.clone(),
                        None => None
                    }
                };
            }

            // Now get those groups from the right of the list and insert them in the right group
            let index = groups.len();
            groups.push(Vec::new());

            if let Some(last) = groups.last_mut() {
                let mut in_index = 0usize;
                while let Some(group) = list.pop_back() {
                    last.push(0);
                    mapping.insert(group, (index, in_index));

                    in_index += 1;
                }
            }
        }

        return Entities::new(groups, mapping);
    }

    /// Calculates the group to which an entity belongs when adding additional components to it, given its previous set of components.
    pub fn get_next_membership(&self, previous_components: &AHashSet<ComponentID>, components_to_add: &AHashSet<ComponentID>) -> AHashSet<Group> {
        let mut previous_groups = AHashSet::<Group>::new();
        let mut new_groups = AHashSet::<Group>::new();

        for group in &self.descriptor {
            if group.iter().all(|x| previous_components.contains(x)) {
                previous_groups.insert(group_id(group));
            }

            if group.iter().all(|x| previous_components.contains(x) || components_to_add.contains(x)) {
                new_groups.insert(group_id(group));
            }
        }

        return new_groups.symmetric_difference(&previous_groups).cloned().collect();
    }

    /// This section of the code implements the Hopcroft-Karp algorithm. It should be used after creating the MemoryMapping.
    ///
    /// This function calculates new distances in the graph and updates them.

    fn compute_distances(layer_one: &AHashMap<Group, Option<IGroup>>, layer_two: &AHashMap<IGroup, Option<Group>>, layer_one_neighbors: &AHashMap<Group, Vec<IGroup>>, distances: &mut AHashMap<Option<IGroup>, u64>) -> bool {
        let mut queue = VecDeque::<Option<Group>>::new();

        for (vertex, pair) in layer_one {
            let ivertex_pos = vertex.clone() as IGroup;

            if pair.is_none() {
                if let Some(distance) = distances.get_mut(&Some(ivertex_pos)) {
                    *distance = 0;

                    queue.push_back(Some(vertex.clone()));
                }
            } else {
                if let Some(distance) = distances.get_mut(&Some(ivertex_pos)) {
                    *distance = INFTY;
                }
            }
        }

        if let Some(distance) = distances.get_mut(&None) {
            *distance = INFTY;
        }

        while let Some(vertex) = queue.pop_front() {
            let ivertex = vertex.map(|vert| vert as IGroup);

            if let Some(dist_u) = distances.get(&ivertex).cloned() {
                if let Some(nil) = distances.get(&None).cloned() {
                    if dist_u < nil {
                        if let Some(vertex) = vertex {
                            if let Some(neighbors) = layer_one_neighbors.get(&vertex) {
                                for v in neighbors {
                                    if let Some(pair_v) = layer_two.get(v) {
                                        let ipair_v = pair_v.map(|vert| vert as IGroup);
                                        if let Some(dist_pair_v) = distances.get_mut(&ipair_v) {
                                            if dist_pair_v.clone() == INFTY {
                                                *dist_pair_v = dist_u + 1;
                                                queue.push_back(pair_v.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        return match distances.get(&None) {
            Some(distance) => distance.clone() < INFTY,
            None => false
        };
    }

    /// This function calculates the right pair according to the current calculated distances

    fn compute_matching(vertex: Option<Group>, layer_one: &mut AHashMap<Group, Option<IGroup>>, layer_two: &mut AHashMap<IGroup, Option<Group>>, layer_one_neighbors: &AHashMap<Group, Vec<IGroup>>, distances: &mut AHashMap<Option<IGroup>, u64>) -> bool {
        if let Some(vertex) = vertex {
            if let Some(neighbors) = layer_one_neighbors.get(&vertex).cloned() {
                for v in neighbors {
                    if let Some(pair_v) = layer_two.get(&v).cloned() {
                        let ipair_v = pair_v.map(|vert| vert as IGroup);
                        if let Some(vertex_dist) = distances.get(&Some(vertex as IGroup)).cloned() {
                            if let Some(pair_v_dist) = distances.get(&ipair_v).cloned() {
                                if (vertex_dist == INFTY && pair_v_dist == INFTY) || (vertex_dist != INFTY && pair_v_dist == vertex_dist + 1) {
                                    if Self::compute_matching(pair_v.clone(), layer_one, layer_two, layer_one_neighbors, distances) {
                                        if let Some(pair_v) = layer_two.get_mut(&v) {
                                            *pair_v = Some(vertex);
                                        }

                                        if let Some(pair_u) = layer_one.get_mut(&vertex) {
                                            *pair_u = Some(v.clone());
                                        }

                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(distance) = distances.get_mut(&Some(vertex as IGroup)) {
                *distance = INFTY;
                return false;
            }
        }

        return true;
    }
}