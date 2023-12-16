use crate::application::entity::Entity;

pub trait System {
    fn components(&self) -> Vec<u64>;

    fn id(&self) -> u128 {
        let mut result = 0u128;

        for id in self.components() {
            result += id as u128;
        }

        return result;
    }

    fn on_startup(&mut self, entities: &[Entity]);
    fn on_update(&mut self, entities: &[Entity]);
    fn on_quit(&mut self, entities: &[Entity]);
}