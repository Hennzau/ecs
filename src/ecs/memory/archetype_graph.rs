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
    id: ArchetypeID,

    components: AHashSet<ComponentID>,

    next: Vec<ArchetypeID>,
    prev: Vec<ArchetypeID>,
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
}