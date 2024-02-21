use crate::ecs::{
    core::component::{
        ArchetypeID,
        ComponentID,
    },
    memory::storage::SparsePool,
};

pub struct Archetype {
    id: ArchetypeID,

    pool: SparsePool,
}