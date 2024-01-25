use simple_logger::SimpleLogger;

use hnz::ecs::prelude::*;

pub mod components {
    use hnz::ecs::prelude::*;

    #[derive(Component)]
    pub struct Position2Df32 {
        pub x: f32,
        pub y: f32,
    }

    #[derive(Component)]
    pub struct Velocity2Df32 {
        pub vx: f32,
        pub vy: f32,
    }
}

pub mod systems {
    use std::collections::HashSet;

    use hnz::ecs::prelude::*;

    use crate::components::{
        Position2Df32,
        Velocity2Df32
    };

    pub struct Movement {}

    impl System for Movement {
        fn components(&self) -> HashSet<ComponentID> {
            return vec![
                Position2Df32::id(),
                Velocity2Df32::id(),
            ].into_iter().collect();
        }

        fn on_tick(&mut self, delta_time: f32, entities: &[Entity], world: &mut World) {
            for entity in entities {
                let position = world.try_get_component::<Position2Df32>(entity).unwrap();
                let velocity = world.try_get_component::<Velocity2Df32>(entity).unwrap();

                let new_position = Position2Df32 {
                    x: position.x + velocity.vx * delta_time,
                    y: position.y + velocity.vy * delta_time,
                };

                let position_mut = world.try_get_mut_component::<Position2Df32>(entity).unwrap();
                *position_mut = new_position;
            }
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let mut builder = ApplicationBuilder::new();
    builder.add_tick_system(Box::new(basic::systems::CloseApplication::new()));
    builder.add_tick_system(Box::new(systems::Movement {}));

    let mut app = builder.build();

    let moderator = app.spawn();
    let _ = app.try_add_component(&moderator, basic::components::SendCloseEventAfterTime {
        time: 4.0,
    });

    let entity = app.spawn();
    let _ = app.try_add_component(&entity, components::Position2Df32 {
        x: 0.0,
        y: 0.0,
    });

    let _ = app.try_add_component(&entity, components::Velocity2Df32 {
        vx: 0.5,
        vy: 1.0,
    });

    app.run(60f32);

    let position = app.try_get_component::<components::Position2Df32>(&entity).unwrap();

    println!("Entity position: {:?}", (position.x, position.y));
}