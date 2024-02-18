/// The 'core' sub-crate contains everything that make an ECS an ECS. Every other part will
/// manipulate the data provided in this sub-crate.
pub mod core;

/// The 'memory' sub-crate is the most important part : it provides algorithms and functions
/// to smartly sort entities and components in order to provide fast-access to them,
/// without the need of massive iterate-testing.
pub mod memory;

/// The 'application' sub-crate is the user-interface to the ECS : it allows the user to not use
/// the 'memory' sub-crate while creating its applications
pub mod application;

/// The 'prelude' module is a shortcut to the most important parts of the ECS crate. It is intended
/// to be used in the user's code to avoid long imports.
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
                CustomSystem,
                System,
                SystemBuilder,
                SystemType
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
            bundle::{
                Bundle,
                BatchBundle,
                SetBundle
            }
        },
    };

    pub use ahash::{
        AHashSet,
        AHashMap,
        RandomState,
    };
}