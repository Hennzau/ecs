pub mod events {
    use crate::core::event::{
        Event,
        AnyEvent
    };

    #[derive(Event)]
    pub struct CloseApplication {}
}

pub mod components {
    use crate::core::component::{
        Component,
        AnyComponent
    };

    #[derive(Component)]
    pub struct SendCloseEventAfterTime {
        pub time: f32,
    }
}

pub mod systems {
    use std::collections::HashSet;

    use crate::{
        application::basic::{
            events,
            components::{
                SendCloseEventAfterTime
            }
        },
        core::{
            component::{
                ComponentID,
                AnyComponent
            },
            entity::Entity,
            world::World,
            system::System
        }
    };

    pub struct CloseApplication {
        pub time: f32,
    }

    impl CloseApplication {
        pub fn new () -> Self {
            return Self {
                time: 0.0
            }
        }
    }

    impl System for CloseApplication {
        fn components(&self) -> HashSet<ComponentID> {
            return vec![
                SendCloseEventAfterTime::id()
            ].into_iter().collect();
        }

        fn on_tick(&mut self, delta_time: f32, entities: &[Entity], world: &mut World) {
            self.time += delta_time;

            for entity in entities {
                if let Some(component) = world.try_get_component::<SendCloseEventAfterTime>(entity) {
                    if self.time >= component.time {
                        world.send_event(Box::new(events::CloseApplication {}));
                    }
                }
            }
        }
    }
}


