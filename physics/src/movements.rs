use std::collections::HashSet;

use ecs::prelude::*;

use crate::maths;

pub struct Movementf32 {}

impl System for Movementf32 {
    fn components(&self) -> HashSet<ComponentID> {
        return vec![maths::Position2Df32::id(), maths::Velocity2Df32::id()].into_iter().collect();
    }

    fn on_tick(&mut self, delta_time: f32, entities: &[Entity], world: &mut World) {
        for entity in entities {
            let position = world.try_get_component::<maths::Position2Df32>(entity).unwrap();
            let velocity = world.try_get_component::<maths::Velocity2Df32>(entity).unwrap();

            let new_position = maths::Position2Df32 {
                x: position.x + velocity.x * delta_time,
                y: position.y + velocity.y * delta_time,
            };

            let position = world.try_get_mut_component::<maths::Position2Df32>(entity).unwrap();
            *position = new_position;
        }
    }
}

pub struct Movementi32 {}

impl System for Movementi32 {
    fn components(&self) -> HashSet<ComponentID> {
        return vec![maths::Position2Di32::id(), maths::Velocity2Di32::id()].into_iter().collect();
    }

    fn on_tick(&mut self, delta_time: f32, entities: &[Entity], world: &mut World) {
        for entity in entities {
            let position = world.try_get_component::<maths::Position2Di32>(entity).unwrap();
            let velocity = world.try_get_component::<maths::Velocity2Di32>(entity).unwrap();

            let new_position = maths::Position2Di32 {
                x: position.x + (velocity.x as f32 * delta_time) as i32,
                y: position.y + (velocity.y as f32 * delta_time) as i32,
            };

            let position = world.try_get_mut_component::<maths::Position2Di32>(entity).unwrap();
            *position = new_position;
        }
    }
}