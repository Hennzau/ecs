use std::collections::HashMap;

use crate::core::{
    entity::Entity,
    component::{
        Component,
        AnyComponent,
        Group,
        components_to_group,
    },
};
use crate::core::sub_app::SubApp;
use crate::memory::storage::Storage;

pub trait System {
    fn components(&self) -> Vec<u64>;

    fn id(&self) -> Group {
        components_to_group(&self.components())
    }

    fn on_startup(&mut self, entities: &[Entity], app: &mut SubApp) {}
    fn on_update(&mut self, entities: &[Entity], app: &mut SubApp) {}
    fn on_quit(&mut self, entities: &[Entity], app: &mut SubApp) {}
}