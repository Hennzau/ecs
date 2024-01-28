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

pub mod events {
    use hnz::ecs::prelude::*;

    #[derive(Event)]
    pub struct PrintPosition {}
}

pub mod systems {
    use hnz::ecs::prelude::*;

    use crate::events;

    use crate::components::{
        Position2Df32,
        Velocity2Df32,
    };

    pub struct Movement {
        pub time: f32,
    }

    impl Movement {
        pub fn new() -> Self {
            return Self {
                time: 0.0
            };
        }
    }

    impl System for Movement {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![
                Position2Df32::component_id(),
                Velocity2Df32::component_id(),
            ].into_iter().collect();
        }

        fn on_tick(&mut self, delta_time: f32, entities: &[Entity], world: &mut World) {
            self.time += delta_time;

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

            if self.time > 1f32 {
                self.time = 0.0;
                world.send_event(Box::new(events::PrintPosition {}));
            }
        }

        fn on_event(&mut self, entities: &[Entity], world: &mut World, _event: &Box<dyn AnyEvent>) {
            for entity in entities {
                let position = world.try_get_component::<Position2Df32>(entity).unwrap();

                println!("Entity position: {:?}", (position.x, position.y));
            }
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let mut builder = ApplicationBuilder::new();
    builder.add_tick_system(basic::systems::CloseApplication::new());
    builder.add_tick_system(SystemBuilder::new(systems::Movement::new()));

    builder.add_event_system(events::PrintPosition::event_id(), SystemBuilder::new(systems::Movement::new()));

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
