use std::collections::{HashMap, HashSet};

use crate::{
    entity::Entity,
    component::{
        ComponentTrait,
        AnyComponentPool,
        ComponentPool,
    },
};
use crate::memory::storage::FastStorage;

pub type SystemsComponentsDescriptor = Vec::<Vec<u64>>;

pub struct Application {
    entities: HashSet<Entity>,
    groups: HashSet<u128>,

    next: u64,

    pools: HashMap<u64, Box<dyn AnyComponentPool>>,
    storage: FastStorage,
}

impl Application {
    pub fn new(mut descriptor: SystemsComponentsDescriptor) -> Self {
        Self {
            entities: HashSet::new(),
            groups: HashSet::new(),
            next: 0,
            pools: HashMap::new(),
            storage: FastStorage::new(descriptor)
        }
    }
}