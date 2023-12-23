use crate::core::{
    entity::Entity,
    component::{
        Component,
        Group,
        components_to_group,
    },
    sub_app::SubApp,
};

pub trait System {
    fn components(&self) -> Vec<Component>;

    fn id(&self) -> Group {
        components_to_group(&self.components())
    }

    fn on_startup(&mut self, _entities: &[Entity], _app: &mut SubApp) {}
    fn on_update(&mut self, _delta_time: f32, _entities: &[Entity], _app: &mut SubApp) {}
    fn on_quit(&mut self, _entities: &[Entity]) {}
}