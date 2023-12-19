use hnz::ecs::application::{
    Application,
    entity::Entity,
    system::System,
    component::{
        ComponentBuilder,
        AnyComponent,
    },
};

#[derive(ComponentBuilder)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(ComponentBuilder)]
struct Velocity {
    v_x: f32,
    v_y: f32,
}

struct Movement {}

impl System for Movement {
    fn components(&self) -> Vec<u64> {
        vec![
            Position::id(),
            Velocity::id(),
        ]
    }

    fn on_startup(&mut self, entities: &[Entity]) {
        println!("New movable entities: {:?}", entities);
    }

    fn on_update(&mut self, entities: &[Entity]) {
        for entity in entities {

        }
    }

    fn on_quit(&mut self, entities: &[Entity]) {}
}

fn main() {
    let mut example = Application::new();
}
