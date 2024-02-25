use std::sync::Arc;
use ahash::{
    AHashMap,
    AHashSet
};

use crate::ecs::core::{
    entity::Entity,
    component::{
        ComponentID,
        ArchetypeID,
        ArchetypeIndex,
        as_archetype
    }
};

pub struct Archetype {
    id: ArchetypeID,

    components: AHashSet<ComponentID>,

    next: Vec<ArchetypeID>,
    prev: Vec<ArchetypeID>,
}

impl Archetype {
    pub fn new(components: AHashSet<ComponentID>) -> Self {
        let id = as_archetype(&components);

        Archetype {
            id,
            components,
            next: vec![],
            prev: vec![],
        }
    }
}

struct MemoryColumn {
    size: usize,
    archetypes: Vec<Archetype>,
    map: AHashMap<ArchetypeID, ArchetypeIndex>,
}

impl MemoryColumn {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            let mut map = AHashMap::new();
            map.insert(0, 0);

            return MemoryColumn {
                size: 0,
                archetypes: vec![Archetype {
                    id: 0,
                    components: AHashSet::new(),
                    next: vec![],
                    prev: vec![]
                }],
                map,
            };
        }

        return MemoryColumn {
            size,
            archetypes: vec![],
            map: AHashMap::new(),
        };
    }

    pub fn add_archetype(&mut self, archetype: Archetype) {
        if !self.map.contains_key(&archetype.id) {
            self.map.insert(archetype.id, self.archetypes.len() - 1);
            self.archetypes.push(archetype);
        }
    }

    pub fn update_previous_and_next(&mut self, archetype: ArchetypeID, previous: &mut MemoryColumn, next: &mut MemoryColumn) {
        self.update_previous(archetype, previous);
        self.update_next(archetype, next);
    }

    pub fn update_previous(&mut self, archetype: ArchetypeID, previous: &mut MemoryColumn) {
        if let Some(archetype_index) = self.map.get(&archetype).cloned() {
            if let Some(archetype) = self.archetypes.get_mut(archetype_index) {
                for previous in &mut previous.archetypes {
                    if previous.components.is_subset(&archetype.components) {
                        archetype.prev.push(previous.id);
                        previous.next.push(archetype.id);
                    }
                }
            }
        }
    }

    pub fn update_next(&mut self, archetype: ArchetypeID, next: &mut MemoryColumn) {
        if let Some(archetype_index) = self.map.get(&archetype).cloned() {
            if let Some(archetype) = self.archetypes.get_mut(archetype_index) {
                for next in &mut next.archetypes {
                    if archetype.components.is_subset(&next.components) {
                        archetype.next.push(next.id);
                        next.prev.push(archetype.id);
                    }
                }
            }
        }
    }
}

pub struct MemoryGraph {
    entities: AHashMap<Entity, ArchetypeID>,

    columns: Vec<MemoryColumn>,

    map: AHashMap<ArchetypeID, usize>,
}

impl MemoryGraph {
    pub fn new() -> Self {
        MemoryGraph {
            entities: AHashMap::new(),
            columns: vec![MemoryColumn::new(0)],
            map: AHashMap::new(),
        }
    }

    unsafe fn update_archetype(&mut self, archetype: ArchetypeID){
        if let Some(size) = self.map.get(&archetype) {

        }
    }

    pub fn add_component(&mut self, entity: Entity, component: ComponentID) {}

    pub fn remove_component(&mut self, entity: Entity, component: ComponentID) {}
}