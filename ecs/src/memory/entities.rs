use ahash::{
    AHashMap, AHashSet,
};

use crate::core::{
    entity::Entity,
    component::Group,
};

pub struct Entities {
    /// This is the 'packed/dense' array containing all entities. It comprises multiple contiguous Entity storages,
    /// each associated with a distinct "main group" defined in the mapping. An example of such a storage could be:
    /// |---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    /// | 2 | 4 | 1 | 7 | 8 | 9 | 3 | 2 | 9 | 11| 4 | 5 | 6 | 9 |
    /// |---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    /// Here, each integer serves as a unique identifier referencing an entity.
    entities: Vec<Vec<Entity>>,

    /// These groups are generated based on a specific mapping. Each `Vec<usize>` represents a global group,
    /// which is a composition of ordered nested groups. Within these nested groups, the 'usize' value denotes
    /// the count of entities belonging to that particular nested group.
    ///
    /// Each `Vec<usize>` of this array can be seen as a vector of 'cursors' for various nested groups :
    ///
    /// |---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    /// | 2 | 4 | 1 | 7 | 8 | 9 | 3 | 2 | 9 | 11| 4 | 5 | 6 | 9 |
    /// |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    /// | - | - | - |ABC| - | - | - | - | AB| - | - | - | - | - | A |
    /// |---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
    groups: Vec<Vec<usize>>,

    /// This 'indices' array provides a way to track entities of a group inside de 'packed/dense' array
    indices: Vec<AHashMap<Entity, usize>>,

    /// This map correlates a Group with its index in the 'groups' (global group) array mentioned earlier,
    /// along with the 'in_index' representing the index of the corresponding nested group.
    map: AHashMap<Group, (usize, usize)>,
}

/// This submodule comprises various structures designed to manage errors encountered during
/// entity swapping and group addition processes.
pub mod entities_errors {
    use std::{
        error,
        fmt::{
            Display,
            Formatter,
        },
    };

    use crate::core::component::Group;

    pub type Result = std::result::Result<(), Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    pub struct GroupMappingError {
        pub group: Group,
    }

    impl Display for GroupMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : this group wasn't mapped correctly", self.group)
        }
    }

    impl error::Error for GroupMappingError {}

    #[derive(Debug, Clone)]
    pub struct EntitiesMappingError {
        pub group: Group,
    }

    impl Display for EntitiesMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : entities were not mapped correctly for this group", self.group)
        }
    }

    impl error::Error for EntitiesMappingError {}

    #[derive(Debug, Clone)]
    pub struct IndicesMappingError {
        pub group: Group,
    }

    impl Display for IndicesMappingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error with group {} : indices were not mapped correctly for this group", self.group)
        }
    }

    impl error::Error for IndicesMappingError {}
}

impl Entities {
    /// Creates a new instance, initializing internal data structures based on provided groups and mapping.
    ///
    /// # Arguments
    ///
    /// * `groups` - A vector of vectors representing the initial structure of entities grouped by group identifiers.
    /// * `map` -   A hash map (`AHashMap`) mapping group identifiers to tuple indices representing the index of corresponding
    ///             nested groups in 'groups' and the index of the desired nested group
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0]      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// ```

    pub fn new(groups: Vec<Vec<usize>>, map: AHashMap<Group, (usize, usize)>) -> Self {
        let mut entities = Vec::new();
        let mut indices = Vec::new();

        for _ in 0..groups.len() {
            entities.push(Vec::new());
            indices.push(AHashMap::new());
        }

        return Self {
            entities: entities,
            groups: groups,
            indices: indices,
            map: map,
        };
    }

    /// Returns a reference to the 'packed/dense' entities array.
    ///
    /// # Returns
    ///
    /// Returns a reference to the 'packed/dense' entities array, where each element of the array represents
    /// an array of entities belonging to a specific group.
    /// The returned reference has a lifetime tied to the lifetime of the entity manager.

    pub fn entities(&self) -> &[Vec<Entity>] {
        return &self.entities;
    }

    /// If the group is accurately mapped, this function returns a slice of the 'packed/dense' entities array
    /// containing all entities belonging to this particular group.
    ///
    /// # Arguments
    ///
    /// * `group` - The group for which the entities' slice is requested.
    ///
    /// # Returns
    ///
    /// Returns `Some(slice)` if the group is accurately mapped, providing a slice of entities in the group.
    /// Returns `None` if the group is not accurately mapped.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0]      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_group_to_entities(a, &[1, 2, 3, 4, 5]);
    /// let _ = entities.try_add_group_to_entity(ab, 1);
    ///
    /// let entity_slice = try_view(ab);
    /// assert!(entity_slice == Some (&[1]));
    ///
    /// let entity_slice = try_view(a);
    /// assert!(entity_slice == Some (&[1, 2, 3, 4, 5]));  // This is pseudo-code, there is no reason that entities would be sorted this way
    /// ```

    pub fn try_view(&self, group: Group) -> Option<&[Entity]> {
        return self.map.get(&group).cloned().map_or_else(|| {
            log::warn!("You tried to view entities from group {}, but this group wasn't mapped", group);

            None
        }, |(index, in_index)| self.groups.get(index).map_or_else(|| {
            log::warn!("You tried to view entities from group {}, range error for groups : this group was not mapped correctly", group);

            None
        }, |nested| nested.get(in_index).cloned().map_or_else(|| {
            log::warn!("You tried to view entities from group {}, range error for nested group : this nested group was not mapped correctly", group);

            None
        }, |count| self.entities.get(index).map_or_else(|| {
            log::warn!("You tried to view entities from group {}, range error for entities : this storage was not created correctly", group);

            None
        }, |entities| entities.get(0..count)))));
    }

    /// This function performs a smart relocation of entities within a group's array.
    /// It moves all 'entities' to the new position by swapping slices of the array.
    /// It also updates the indices of the entities in the 'indices' map.
    ///
    /// This function is useful for moving entities towards **the front of the array** using only a single swap_with_slice.
    ///
    /// # Arguments
    ///
    /// * `indices` - A mutable reference to an `AHashMap<Entity, usize>` containing the current indices of entities.
    /// * `array` - A mutable reference to a vector (`Vec<Entity>`) containing the entities to be relocated.
    /// * `old_first` - The starting index of the slice to be moved in the original array.
    ///                 In this specific method, 'old_first' points directly to the first entity on the left of the slice
    ///
    /// * `new_first` - The destination index where the slice should be moved in the array.
    /// * `count` - The number of elements to move from the starting index.
    ///
    /// # Panics
    ///
    /// The function panics if the provided indices exceed the bounds of the array.
    ///
    /// # Example
    ///
    /// ```
    /// use ahash::AHashMap;
    ///
    /// let mut indices_map = AHashMap::new();
    /// let mut entities_array = vec![1, 2, 3, 4, 5];
    ///
    /// indices_map.insert (1, 0); // Entity, index in 'entities_array'
    /// indices_map.insert (2, 1);
    /// indices_map.insert (3, 2);
    /// indices_map.insert (4, 3);
    /// indices_map.insert (5, 4);
    ///
    /// // Relocates entities from index 1 to index 3 to index 0.
    /// relocate_slice_ahead(&mut indices_map, &mut entities_array, 1, 0, 3);
    ///
    /// assert! (entities_array == vec![2, 3, 4, 1, 5]);
    /// ```
    ///
    /// # Note
    ///
    /// The function does not return anything but directly modifies the array and updates the indices map.

    fn relocate_slice_ahead(indices: &mut AHashMap<Entity, usize>, array: &mut Vec<Entity>, old_first: usize, new_first: usize, count: usize) {
        if new_first >= old_first {
            return;
        }

        // separate the array
        let (left, right) = array.split_at_mut(old_first);

        if new_first + count <= old_first {
            let previous_entities = left.get_mut(new_first..(new_first + count));
            let entities = right.get_mut(0..count);

            if let Some(previous_entities) = previous_entities {
                if let Some(entities) = entities {
                    // First we update new indices

                    for (i, entity) in previous_entities.iter().enumerate() {
                        indices.insert(entity.clone(), old_first + i);
                    }

                    for (i, entity) in entities.iter().enumerate() {
                        indices.insert(entity.clone(), new_first + i);
                    }

                    // We swap the slices of the array
                    previous_entities.swap_with_slice(entities);
                }
            }
        } else {
            let gap = count - (old_first - new_first);

            let previous_entities = left.get_mut(new_first..old_first);
            let entities = right.get_mut(gap..count); // Be aware, it's not gap..gap+count

            if let Some(previous_entities) = previous_entities {
                if let Some(entities) = entities {
                    // First we update new indices

                    for (i, entity) in previous_entities.iter().enumerate() {
                        indices.insert(entity.clone(), old_first + gap + i);
                    }

                    for (i, entity) in entities.iter().enumerate() {
                        indices.insert(entity.clone(), new_first + i);
                    }

                    // We swap the slices of the array
                    previous_entities.swap_with_slice(entities);
                }
            }
        }
    }

    /// This function performs a smart relocation of entities within a group's array.
    /// It moves all 'entities' to the new position by swapping slices of the array.
    /// It also updates the indices of the entities in the 'indices' map.
    ///
    /// This function is useful for moving entities towards **the back of the array** using only a single swap_with_slice.
    ///
    /// # Arguments
    ///
    /// * `indices` - A mutable reference to an `AHashMap<Entity, usize>` containing the current indices of entities.
    /// * `array` - A mutable reference to a vector (`Vec<Entity>`) containing the entities to be relocated.
    /// * `old_first` - The starting index of the slice to be moved in the original array.
    ///                 In this specific method, 'old_first' points directly to the next entity on the right of the slice
    ///
    /// * `new_first` - The destination index where the slice should be moved in the array.
    /// * `count` - The number of elements to move from the starting index.
    ///
    /// # Panics
    ///
    /// The function panics if the provided indices exceed the bounds of the array.
    ///
    /// # Example
    ///
    /// ```
    /// use ahash::AHashMap;
    ///
    /// let mut indices_map = AHashMap::new();
    /// let mut entities_array = vec![1, 2, 3, 4, 5];
    ///
    /// indices_map.insert (1, 0); // Entity, index in 'entities_array'
    /// indices_map.insert (2, 1);
    /// indices_map.insert (3, 2);
    /// indices_map.insert (4, 3);
    /// indices_map.insert (5, 4);
    ///
    /// // Relocates entities from index 3 to index 0 to index 4.
    /// relocate_slice_ahead(&mut indices_map, &mut entities_array, 3, 4, 3);
    ///
    /// assert! (entities_array == vec![4, 1, 2, 3, 5]);
    /// ```
    ///
    /// # Note
    ///
    /// The function does not return anything but directly modifies the array and updates the indices map.

    fn relocate_slice_behind(indices: &mut AHashMap<Entity, usize>, array: &mut Vec<Entity>, old_first: usize, new_first: usize, count: usize) {
        if new_first <= old_first {
            return;
        }

        // separate the array
        let (left, right) = array.split_at_mut(old_first);

        if new_first - count >= old_first {
            let entities = left.get_mut((old_first - count)..old_first);
            let previous_entities = right.get_mut((new_first - old_first - count)..(new_first - old_first));

            if let Some(previous_entities) = previous_entities {
                if let Some(entities) = entities {
                    // First we update new indices

                    for (i, entity) in previous_entities.iter().enumerate() {
                        indices.insert(entity.clone(), old_first - count + i);
                    }

                    for (i, entity) in entities.iter().enumerate() {
                        indices.insert(entity.clone(), new_first - count + i);
                    }

                    // We swap the slices of the array
                    previous_entities.swap_with_slice(entities);
                }
            }
        } else {
            let entities = left.get_mut((old_first - count)..(new_first - count));
            let previous_entities = right.get_mut(0..(new_first - old_first));

            if let Some(previous_entities) = previous_entities {
                if let Some(entities) = entities {
                    // First we update new indices

                    for (i, entity) in previous_entities.iter().enumerate() {
                        indices.insert(entity.clone(), old_first - count + i);
                    }

                    for (i, entity) in entities.iter().enumerate() {
                        indices.insert(entity.clone(), old_first + i);
                    }

                    // We swap the slices of the array
                    previous_entities.swap_with_slice(entities);
                }
            }
        }
    }

    /// Searches for entities in the 'waiting' vector located between `start_search` and `end_search` indices that
    /// need to be moved to the next group. It swaps them next to `end_search` to prepare for moving them to the 'entities_to_add' slice.
    ///
    /// # Arguments
    ///
    /// * `indices` - A mutable reference to an `AHashMap<Entity, usize>` containing the indices of entities in the 'array'.
    /// * `array` - A mutable reference to a vector (`Vec<Entity>`) representing the main entities array.
    /// * `waiting` - A mutable reference to a vector (`Vec<Entity>`) containing entities waiting to be moved.
    /// * `start_search` - The starting index for searching entities in the 'waiting' vector.
    /// * `end_search` - The ending index for searching entities in the 'waiting' vector.
    ///
    /// # Returns
    ///
    /// Returns a vector containing entities that have been swapped next to `end_search`.

    fn move_ahead_and_retrieve_waiting_entities(indices: &mut AHashMap<Entity, usize>, array: &mut Vec<Entity>, waiting: &mut Vec<Entity>, start_search: usize, end_search: usize) -> Vec<Entity> {
        let mut merged = Vec::<Entity>::new();

        for entity in waiting.iter().cloned() {
            if let Some(entity_index) = indices.get(&entity).cloned() {
                if entity_index >= start_search && entity_index < end_search {
                    let count = merged.len();

                    // We swap the entity to end_search - 1 - merge_count
                    if let Some(previous_entity) = array.get(end_search - 1 - count).cloned() {
                        merged.push(entity);

                        indices.insert(previous_entity, entity_index);
                        indices.insert(entity, end_search - 1 - count);

                        array.swap(entity_index, end_search - 1 - count);
                    }
                }
            }
        }

        for entity in &merged {
            waiting.retain(|e| e.clone() != entity.clone());
        }

        return merged;
    }

    /// Searches for entities in the 'waiting' vector located between `start_search` and `end_search` indices that
    /// need to be moved to the next group. It swaps them next to `start_search` to prepare for moving them to the 'entities_to_add' slice.
    ///
    /// # Arguments
    ///
    /// * `indices` - A mutable reference to an `AHashMap<Entity, usize>` containing the indices of entities in the 'array'.
    /// * `array` - A mutable reference to a vector (`Vec<Entity>`) representing the main entities array.
    /// * `waiting` - A mutable reference to a vector (`Vec<Entity>`) containing entities waiting to be moved.
    /// * `start_search` - The starting index for searching entities in the 'waiting' vector.
    /// * `end_search` - The ending index for searching entities in the 'waiting' vector.
    ///
    /// # Returns
    ///
    /// Returns a vector containing entities that have been swapped next to `start_search`.

    fn move_behind_and_retrieve_waiting_entities(indices: &mut AHashMap<Entity, usize>, array: &mut Vec<Entity>, waiting: &mut Vec<Entity>, start_search: usize, end_search: usize) -> Vec<Entity> {
        let mut merged = Vec::<Entity>::new();

        for entity in waiting.iter().cloned() {
            if let Some(entity_index) = indices.get(&entity).cloned() {
                if entity_index >= start_search && entity_index < end_search {
                    let count = merged.len();

                    // We swap the entity to end_search - 1 - merge_count
                    if let Some(previous_entity) = array.get(start_search + count).cloned() {
                        merged.push(entity);

                        indices.insert(previous_entity, entity_index);
                        indices.insert(entity, start_search + count);

                        array.swap(entity_index, start_search + count);
                    }
                }
            }
        }

        for entity in &merged {
            waiting.retain(|e| e.clone() != entity.clone());
        }

        return merged;
    }

    /// Attempts to add a set of entities to a specific group. For each entity provided, it performs
    /// the following action: if the entity already exists in the global group, it relocates the entity within
    /// the nested groups; otherwise, it adds the entity to the global group and then performs the relocation.
    /// **In fact this function performs smarts moving of all entities, it does not perform any operation on each entity**
    /// # Arguments
    ///
    /// * `group` - The target group to which the entities should be associated.
    /// * `entities` - A slice of entities to be added to the specified group.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all entities are successfully associated with the specified group.
    /// If any issues occur or inconsistencies are detected, it returns an `Err` indicating the problematic group.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0]      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_group_to_entities(a, &[1, 2, 3, 4, 5]);
    /// ```

    pub fn try_add_group_to_entities(&mut self, group: Group, entities: &[Entity]) -> entities_errors::Result {
        // This step involves retrieving all necessary storages to add entities and computing the new position of the entity.
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => match self.indices.get_mut(index) {
                Some(indices) => match self.entities.get_mut(index) {
                    Some(array) => match self.groups.get_mut(index) {
                        Some(groups) => {
                            // We gather all nested groups located to the right of the target group.
                            if let Some(groups_to_cross) = match in_index <= groups.len() {
                                true => {
                                    let (_, groups) = groups.split_at_mut(in_index);

                                    Some(groups)
                                }
                                false => None
                            } {
                                // We gather all entities that needs to be first added to the group and the ones that
                                // are already in one of the nested groups (maybe it's not located at the right place)

                                let mut entities_to_add = Vec::<Entity>::new();
                                let mut waiting_entities = Vec::<Entity>::new();

                                let mut current_index = array.len();

                                for entity in entities {
                                    if indices.contains_key(entity) {
                                        waiting_entities.push(entity.clone());
                                    } else {
                                        entities_to_add.push(entity.clone());

                                        indices.insert(entity.clone(), array.len());
                                        array.push(entity.clone());
                                    }
                                }

                                // The idea is to swap the whole 'entities_to_add' slice each time, and when this slice enters
                                // a group where entities from 'waiting_entities' are located, we swap them in order to move
                                // them in 'entities_to_add' slice.

                                // We traverse these groups from the right and we swap all entities that must be added to the group
                                // At the end of each nested groups

                                for nested in groups_to_cross.iter_mut().rev() {
                                    // We search for all entities that are between our slice 'entities_to_add' and the end of the
                                    // current nested group. We swap them next to 'entities_to_add' slice in order to move them in.
                                    // This way, 'entities_to_add' slice will be bigger and bigger at each iteration, gathering
                                    // all entities that must be moved in the right group.

                                    let mut merged = Self::move_ahead_and_retrieve_waiting_entities(indices, array, &mut waiting_entities, nested.clone(), current_index);

                                    current_index -= merged.len();
                                    entities_to_add.append(&mut merged);

                                    // This performs a smart relocation of all entities within the array of a group.

                                    Self::relocate_slice_ahead(indices, array, current_index, nested.clone(), entities_to_add.len());

                                    current_index = nested.clone();
                                    *nested += entities_to_add.len();
                                }
                            }
                            Ok(())
                        }
                        None => Err(entities_errors::GroupMappingError { group: group }.into())
                    }
                    None => Err(entities_errors::EntitiesMappingError { group: group }.into())
                }
                None => Err(entities_errors::IndicesMappingError { group: group }.into())
            },
            None => Err(entities_errors::GroupMappingError { group: group }.into())
        };
    }

    /// Attempts to add an entity to a specific group. If the entity already exists in the global group, it relocates
    /// the entity within the nested groups; otherwise, it adds the entity to the global group and then performs the relocation.
    ///
    /// # Arguments
    ///
    /// * `group` - The target group to which the entity should be associated.
    /// * `entity` - A reference to the entity to be added to the specified group.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the entity is successfully associated with the specified group.
    /// If any issues occur or inconsistencies are detected, it returns an `Err` indicating the problematic group.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0]      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_group_to_entity(a, 1 as Entity);
    /// ```

    pub fn try_add_group_to_entity(&mut self, group: Group, entity: Entity) -> entities_errors::Result {
        // This step involves retrieving all necessary storages to add entities and computing the new position of the entity.
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => match self.indices.get_mut(index) {
                Some(indices) => match self.entities.get_mut(index) {
                    Some(array) => match self.groups.get_mut(index) {
                        Some(groups) => {
                            // We gather all nested groups located to the right of the target group.
                            if let Some(groups_to_cross) = match in_index <= groups.len() {
                                true => {
                                    let (_, groups) = groups.split_at_mut(in_index);

                                    Some(groups)
                                }
                                false => None
                            } {
                                for nested in groups_to_cross.iter_mut().rev() {
                                    if let Some(entity_index) = indices.get(&entity).cloned() {
                                        if entity_index >= nested.clone() {
                                            if let Some(previous_entity) = array.get(nested.clone()).cloned() {
                                                indices.insert(previous_entity, entity_index);
                                                indices.insert(entity, nested.clone());

                                                array.swap(entity_index, nested.clone());
                                            }

                                            *nested += 1;
                                        }
                                    } else {
                                        // If the entity doesn't exist yet, we add it to the global group and it means
                                        // that the current nested group is the last one. So because they are no entities
                                        // that are in 'array' and in no nested group, we can safely add the entity at the end.
                                        // without the need to swap it.

                                        let entity_index = array.len();

                                        array.push(entity);
                                        indices.insert(entity, entity_index);

                                        *nested += 1;
                                    }
                                }
                            }
                            Ok(())
                        }
                        None => Err(entities_errors::GroupMappingError { group: group }.into())
                    }
                    None => Err(entities_errors::EntitiesMappingError { group: group }.into())
                }
                None => Err(entities_errors::IndicesMappingError { group: group }.into())
            },
            None => Err(entities_errors::GroupMappingError { group: group }.into())
        };
    }

    /// Attempts to add a set of entities to a set of groups. For each entity provided, it performs
    /// the following action: if the entity already exists in the global group, it relocates the entity within
    /// the nested groups; otherwise, it adds the entity to the global group and then performs the relocation.
    /// **In fact this function performs smarts moving of all entities, it does not perform any operation on each entity**
    /// # Arguments
    ///
    /// * `groups` - The groups to which the entities should be associated.
    /// * `entities` - A slice of entities to be added to the specified group.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all entities are successfully associated with all group in groups.
    /// If any issues occur or inconsistencies are detected, it returns an `Err` indicating the problematic group.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0],      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_groups_to_entities(vec![a, ab, bc].into_iter().collect(), &[1, 2, 3, 4, 5]);
    /// ```

    pub fn try_add_groups_to_entities(&mut self, groups: &AHashSet<Group>, entities: &[Entity]) -> entities_errors::Result {
        let mut result = Ok(());

        for group in groups {
            let res = self.try_add_group_to_entities(group.clone(), entities);
            if res.is_err() {
                result = res;
            }
        }

        return result;
    }

    /// Attempts to add an entity to a set of groups. If the entity already exists in the global group, it relocates
    /// the entity within the nested groups; otherwise, it adds the entity to the global group and then performs the relocation.
    ///
    /// # Arguments
    ///
    /// * `group` - The groups to which the entity should be associated.
    /// * `entity` - A reference to the entity to be added to the specified group.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the entity is successfully associated with all group in groups.
    /// If any issues occur or inconsistencies are detected, it returns an `Err` indicating the problematic group.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0],      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_groups_to_entity(vec![a, ab, bc].into_iter().collect(), 1 as Entity);
    /// ```

    pub fn try_add_groups_to_entity(&mut self, groups: &AHashSet<Group>, entity: Entity) -> entities_errors::Result {
        let mut result = Ok(());

        for group in groups {
            let res = self.try_add_group_to_entity(group.clone(), entity);
            if res.is_err() {
                result = res;
            }
        }

        return result;
    }

    /// Attempts to remove a set of entities from a specific group. For each entity provided, it performs
    /// the following action: if the entity exists in the nested groups, it relocates it to the end of each nested group
    /// and finally removes it from the packed array.
    ///
    /// # Arguments
    ///
    /// * `group` - The target group from which the entities should be removed.
    /// * `entities` - A slice of entities to be removed from the specified group.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all entities are successfully removed from the specified group.
    /// If any issues occur or inconsistencies are detected, it returns an `Err` indicating the problematic group.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0]      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_group_to_entities(a, &[1, 2, 3, 4, 5]);
    ///
    /// let _ = entities.try_remove_group_to_entities(a, &[4, 5]);
    ///```

    pub fn try_remove_group_to_entities(&mut self, group: Group, entities: &[Entity]) -> entities_errors::Result {
        // This step involves retrieving all necessary storages to add entities and computing the new position of the entity.
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => match self.indices.get_mut(index) {
                Some(indices) => match self.entities.get_mut(index) {
                    Some(array) => match self.groups.get_mut(index) {
                        Some(groups) => {
                            // We gather all nested groups located to the left of the target group (including the target group).
                            if let Some(groups_to_cross) = match in_index < groups.len() {
                                true => {
                                    let (groups, _) = groups.split_at_mut(in_index + 1);

                                    Some(groups)
                                }
                                false => None
                            } {
                                let mut current_index = 0usize;
                                let mut entities_to_remove = Vec::<Entity>::new();
                                let mut waiting_entities = Vec::<Entity>::from(entities);

                                for nested in groups_to_cross {
                                    let mut merged = Self::move_behind_and_retrieve_waiting_entities(indices, array, &mut waiting_entities, current_index, nested.clone());

                                    current_index += merged.len();
                                    entities_to_remove.append(&mut merged);

                                    Self::relocate_slice_behind(indices, array, current_index, nested.clone(), entities_to_remove.len());

                                    current_index = nested.clone();
                                    *nested -= entities_to_remove.len();
                                }

                                if in_index == groups.len() - 1 {
                                    for entity in entities_to_remove {
                                        array.pop();
                                        indices.remove(&entity);
                                    }
                                }
                            }

                            Ok(())
                        }
                        None => Err(entities_errors::GroupMappingError { group: group }.into())
                    }
                    None => Err(entities_errors::EntitiesMappingError { group: group }.into())
                }
                None => Err(entities_errors::IndicesMappingError { group: group }.into())
            },
            None => Err(entities_errors::GroupMappingError { group: group }.into())
        };
    }

    /// Attempts to remove an entity from a specific group. If the entity exists in the nested groups, it relocates it
    /// to the end of each nested group and finally removes it from the packed array.
    ///
    /// # Arguments
    ///
    /// * `group` - The target group from which the entity should be removed.
    /// * `entity` - The entity to be removed from the specified group.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the entity is successfully removed from the specified group.
    /// If any issues occur or inconsistencies are detected, it returns an `Err` indicating the problematic group.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0]      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_group_to_entities(a, &[1, 2, 3, 4, 5]);
    ///
    /// let _ = entities.try_remove_group_to_entity(a, 4);
    ///```

    pub fn try_remove_group_to_entity(&mut self, group: Group, entity: Entity) -> entities_errors::Result {
        // This step involves retrieving all necessary storages to add entities and computing the new position of the entity.
        return match self.map.get(&group).cloned() {
            Some((index, in_index)) => match self.indices.get_mut(index) {
                Some(indices) => match self.entities.get_mut(index) {
                    Some(array) => match self.groups.get_mut(index) {
                        Some(groups) => {
                            let last_in_index = groups.len() - 1;

                            // We gather all nested groups located to the left of the target group (including the target group).
                            if let Some(groups_to_cross) = match in_index < groups.len() {
                                true => {
                                    let (groups, _) = groups.split_at_mut(in_index + 1);

                                    Some(groups)
                                }
                                false => None
                            } {
                                // Lastly, we traverse these groups from the l, swapping all entities that currently
                                // are part of the group to the end.
                                for (i, nested) in groups_to_cross.iter_mut().enumerate() {
                                    if let Some(entity_index) = indices.get(&entity).cloned() {
                                        if entity_index < nested.clone() {
                                            if let Some(previous_entity) = array.get(nested.clone() - 1).cloned() {
                                                indices.insert(previous_entity, entity_index);
                                                indices.insert(entity.clone(), nested.clone() - 1);

                                                array.swap(entity_index, nested.clone() - 1);

                                                if i == last_in_index {
                                                    array.pop();
                                                    indices.remove(&entity);
                                                }
                                            }

                                            *nested -= 1;
                                        }
                                    }
                                }
                            }

                            Ok(())
                        }
                        None => Err(entities_errors::GroupMappingError { group: group }.into())
                    }
                    None => Err(entities_errors::EntitiesMappingError { group: group }.into())
                }
                None => Err(entities_errors::IndicesMappingError { group: group }.into())
            },
            None => Err(entities_errors::GroupMappingError { group: group }.into())
        };
    }

    /// Attempts to remove a set of entities from multiple groups. For each entity provided, it performs
    /// the following action: if the entity exists in any of the specified groups, it relocates it to the end
    /// of each nested group and finally removes it from the packed array.
    ///
    /// # Arguments
    ///
    /// * `groups` - A hash set (`AHashSet`) of target groups from which the entities should be removed.
    /// * `entities` - A slice of entities to be removed from the specified groups.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all entities are successfully removed from the specified groups.
    /// If any issues occur or inconsistencies are detected, it returns an `Err` indicating the problematic group.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0]      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_groups_to_entities(vec![a, bc].into_iter().collect(), &[1, 2, 3, 4, 5]);
    ///
    /// let _ = entities.try_remove_groups_to_entities(vec![b, c].into_iter().collect(), &[4, 5]);
    ///```

    pub fn try_remove_groups_to_entities(&mut self, groups: &AHashSet<Group>, entities: &[Entity]) -> entities_errors::Result {
        let mut result = Ok(());

        for group in groups {
            let res = self.try_remove_group_to_entities(group.clone(), entities);
            if res.is_err() {
                result = res;
            }
        }

        return result;
    }

    /// Attempts to remove an entity from multiple groups. If the entity exists in any of the specified groups,
    /// it relocates it to the end of each nested group and finally removes it from the packed array.
    ///
    /// # Arguments
    ///
    /// * `groups` - A hash set (`AHashSet`) of target groups from which the entity should be removed.
    /// * `entity` - The entity to be removed from the specified groups.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the entity is successfully removed from the specified groups.
    /// If any issues occur or inconsistencies are detected, it returns an `Err` indicating the problematic group.
    ///
    /// # Example
    ///
    /// ```
    /// // Create a new entities storage with initial groups and mapping.
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct A {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct B {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct C {}
    ///
    /// // Be aware, components ids and groups ids should be calculated by [`crate::core::component::groupd_id`]
    ///
    /// let a = A::component_id();
    /// let b = B::component_id();
    /// let c = C::component_id();
    /// let ab = A::component_id() + B::component_id();
    /// let ac = A::component_id() + C::component_id();
    /// let bc = B::component_id() + C::component_id();
    /// let abc = A::component_id() + B::component_id() + C::component_id();
    ///
    /// let groups = vec![
    ///     vec![0, 0, 0],  // for groups ABC - AB - A
    ///     vec![0, 0],     // For BC - B
    ///     vec![0, 0]      // For AC - C
    /// ];
    ///
    /// let mapping = AHashMap::new ();
    /// mapping.insert (abc, (0, 0));
    /// mapping.insert (ab,  (0, 1));
    /// mapping.insert (a,   (0, 2));
    /// mapping.insert (bc,  (1, 0));
    /// mapping.insert (b,   (1, 1));
    /// mapping.insert (ac,  (2, 0));
    /// mapping.insert (c,   (2, 1));
    ///
    /// let entities = ecs::memory::entities::Entities::new(groups, mapping);
    /// let _ = entities.try_add_groups_to_entities(vec![a, bc].into_iter().collect(), &[1, 2, 3, 4, 5]);
    ///
    /// let _ = entities.try_remove_groups_to_entity(vec![b, c].into_iter().collect(), 4);
    ///```

    pub fn try_remove_groups_to_entity(&mut self, groups: &AHashSet<Group>, entity: Entity) -> entities_errors::Result {
        let mut result = Ok(());

        for group in groups {
            let res = self.try_remove_group_to_entity(group.clone(), entity);
            if res.is_err() {
                result = res;
            }
        }

        return result;
    }
}