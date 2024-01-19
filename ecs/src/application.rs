use crate::{
    memory::{
        entities::Entities,
        mapping::{
            MemoryMapping,
            MemoryMappingDescriptor
        }
    },
    core::{
        component::Group,
        entity::Entity,
    },
};

pub struct Application {
    mapping: MemoryMapping,
    entities: Entities
}

impl Application {
    pub fn new(descriptor: MemoryMappingDescriptor) -> Self {
        let mapping = MemoryMapping::new(descriptor);

        return Self {
            entities: mapping.create_storage(),
            mapping: mapping,
        }
    }
}

