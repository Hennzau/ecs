pub mod core;
pub mod memory;

pub mod prelude {
    pub use crate::ecs::core::{
        component::{
            Component,
            ComponentID,
            ArchetypeID,
            AnyComponent,
        }
    };

    pub use ahash::{
        AHashSet,
        AHashMap,
        RandomState,
    };
}