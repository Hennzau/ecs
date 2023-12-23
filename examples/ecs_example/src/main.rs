use hnz::ecs::application::prelude::*;

#[derive(ComponentBuilder)]
struct Position2D {
    x: f32,
    y: f32,
}

#[derive(ComponentBuilder)]
struct Velocity2D {
    dx: f32,
    dy: f32,
}

struct ApplyMovement {}

impl System for ApplyMovement {
    fn components(&self) -> Vec<Component> {
        return vec![
            Position2D::id(),
            Velocity2D::id(),
        ];
    }

    fn on_startup(&mut self, entities: &[Entity], app: &mut SubApp) {
        for entity in entities {
            let position = app.try_get_component::<Position2D>(entity).unwrap();
            let velocity = app.try_get_component::<Velocity2D>(entity).unwrap();

            println!("{} is now a Movable entity",
                     entity.clone()
            );

            println!("{} has an initial position ({},{}) and an initial velocity ({},{})",
                     entity.clone(), position.x, position.y, velocity.dx, velocity.dy
            );
        }
    }

    fn on_update(&mut self, delta_time: f32, entities: &[Entity], app: &mut SubApp) {
        for entity in entities {
            let velocity = app.try_get_component::<Velocity2D>(entity).unwrap();
            let (dx, dy) = (velocity.dx * delta_time, velocity.dy * delta_time);

            let position = app.try_get_component_mut::<Position2D>(entity).unwrap();

            position.x += dx;
            position.y += dy;
        }
    }
}

struct ShowPosition {
    global_time: f32,
}

impl System for ShowPosition {
    fn components(&self) -> Vec<Component> {
        return vec![
            Position2D::id()
        ];
    }

    fn on_update(&mut self, delta_time: f32, entities: &[Entity], app: &mut SubApp) {
        for entity in entities {
            let position = app.try_get_component::<Position2D>(entity).unwrap();

            println!("[{}] : Entity {} located at position ({},{})", self.global_time, entity.clone(), position.x, position.y);
        }

        self.global_time += delta_time;
    }
}

fn main() {
    let mut app = Application::new(vec![
        Box::new(ApplyMovement {}),
        Box::new(ShowPosition {
            global_time: 0f32
        }),
    ]);

    let window = app.spawn();
    app.try_add_component(&window, Position2D { x: 0f32, y: 0f32 });
    app.try_add_component(&window, Velocity2D { dx: 20f32, dy: 20f32 });

    app.run();
}