use ahash::{
    AHashMap,
    AHashSet
};

use crate::ecs::core::component::{
    ArchetypeID,
    ArchetypeIndex,
    ComponentID,
};

pub struct Archetype {
    pub id: ArchetypeID,

    pub components: AHashSet<ComponentID>,

    pub next: Vec<ArchetypeID>,
    pub prev: Vec<ArchetypeID>,
}

pub struct Column {
    pub size: usize,
    pub archetypes: Vec<Archetype>,
    pub map: AHashMap<ArchetypeID, ArchetypeIndex>,
}

impl Column {
    pub fn new() -> Self {
        Column {
            size: 0,
            archetypes: Vec::new(),
            map: AHashMap::new(),
        }
    }
}

pub struct ArchetypeGraph {
    columns: AHashMap<usize, Column>,
    map: AHashMap<ArchetypeID, usize>,
}

impl ArchetypeGraph {
    pub fn new() -> Self {
        ArchetypeGraph {
            columns: AHashMap::new(),
            map: AHashMap::new(),
        }
    }

    pub fn add_archetype(&mut self, archetype: Archetype) {
        if !self.map.contains_key(&archetype.id) {
            let size = archetype.components.len();
            let id = archetype.id;

            self.map.insert(id, size);

            if !self.columns.contains_key(&size) {
                self.columns.insert(size, Column::new());
            }

            let column = self.columns.get_mut(&size).unwrap();

            let index = column.archetypes.len();

            column.archetypes.push(archetype);
            column.map.insert(id, index);

            let mut current_previous = Vec::<ArchetypeID>::new();
            let mut current_next = Vec::<ArchetypeID>::new();

            let components = column.archetypes.get(index).unwrap().components.clone();

            if let Some(previous) = self.columns.get_mut(&(size - 1)) {
                for previous in &mut previous.archetypes {
                    if previous.components.is_subset(&components) {
                        previous.next.push(id);
                        current_previous.push(previous.id);
                    }
                }
            }

            if let Some(next) = self.columns.get_mut(&(size + 1)) {
                for next in &mut next.archetypes {
                    if components.is_subset(&next.components) {
                        next.prev.push(id);
                        current_next.push(next.id);
                    }
                }
            }

            self.columns.get_mut(&size).unwrap().archetypes.get_mut(index).unwrap().prev = current_previous;
            self.columns.get_mut(&size).unwrap().archetypes.get_mut(index).unwrap().next = current_next;
        }
    }

    pub fn get(&self, id: ArchetypeID) -> Option<&Archetype> {
        if let Some(size) = self.map.get(&id) {
            if let Some(column) = self.columns.get(size) {
                if let Some(index) = column.map.get(&id).cloned() {
                    return column.archetypes.get(index);
                }
            }
        }

        return None;
    }

    pub fn get_mut(&mut self, id: ArchetypeID) -> Option<&mut Archetype> {
        if let Some(size) = self.map.get(&id) {
            if let Some(column) = self.columns.get_mut(size) {
                if let Some(index) = column.map.get(&id).cloned() {
                    return column.archetypes.get_mut(index);
                }
            }
        }

        return None;
    }
}