use crate::entity::Entity;
use crate::memory::mapping::{MemoryMapping, MemoryMappingDescriptor};

pub struct FastStorage {
    entities: Vec<Vec<Entity>>,
    mapping: MemoryMapping,
}

impl FastStorage {
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

    pub fn systems_id(&self) -> &MemoryMappingDescriptor {
        self.mapping.descriptor()
    }
}