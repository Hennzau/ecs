use crate::entity::Entity;
use crate::memory::mapping::{MemoryMapping, MemoryMappingDescriptor};

pub struct MappedStorage {
    entities: Vec<Vec<Entity>>,
    mapping: MemoryMapping,
}

impl MappedStorage {
    pub fn new(descriptor: MemoryMappingDescriptor) -> Self {
        let mut entities: Vec<Vec<Entity>> = Vec::new();
        let mapping = MemoryMapping::new(descriptor);

        for _ in 0..mapping.len() {
            entities.push(Vec::new());
        }

        Self {
            entities: entities,
            mapping: mapping,
        }
    }

    pub fn systems(&self) -> &MemoryMappingDescriptor {
        self.mapping.descriptor()
    }
}