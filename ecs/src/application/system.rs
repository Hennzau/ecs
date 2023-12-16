use crate::application::entity::Entity;

pub trait SystemTrait {
    fn components() -> Vec<u64>;

    fn on_startup(&mut self, entities: &[Entity]);
    fn on_update(&mut self, entities: &[Entity]);
    fn on_quit(&mut self, entities: &[Entity]);
}