use crate::ecs::{
    core::component::{
        ArchetypeID,
        ComponentID,
    },
    memory::storage::{
        SparsePool,
        ColumnID,
    },
};

pub struct Archetype {
    id: ArchetypeID,

    pool: SparsePool,
}