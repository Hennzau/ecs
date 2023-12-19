use crate::core::{
    entity::Entity,
    component::{
        components_to_group,
        Group
    }
};

pub trait System {
    fn components(&self) -> Vec<u64>;

    fn id(&self) -> Group {
        components_to_group(&self.components())
    }

    fn on_startup(&mut self, entities: &[Entity]);
    fn on_update(&mut self, entities: &[Entity]);
    fn on_quit(&mut self, entities: &[Entity]);
}