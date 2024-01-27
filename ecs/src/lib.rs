/// The ECS library consists in 3 main parts :
/// -   The 'core' sub-crate contains everything that make an ECS an ECS. Every other part will
///     manipulate the data provided in this sub-crate.
///
/// -   The 'memory' sub-crate is the most important part : it provides algorithms and functions
///     to smartly sort entities, components and systems in order to provide fast-access to them,
///     without the need of massive iterate-testing.
///
/// -   The 'application' sub-crate is the user-interface to the ECS : it allows the user to not use
///     the 'memory' sub-crate while creating its applications

pub mod core;
pub mod memory;
pub mod application;

pub mod prelude {
    pub use crate::{
        core::{
            entity::Entity,
            component::{
                AnyComponent,
                ComponentID,
                Component,
                group_id,
            },
            system::{
                SharedSystem,
                CustomSharedSystem,
                System,
            },
            world::World,
            event::{
                AnyEvent,
                EventID,
                Event,
            },
        },
        application::{
            Application,
            builder::ApplicationBuilder,
            basic,
        },
    };

    pub use ahash::{
        AHashSet,
        AHashMap,
        RandomState,
    };
}