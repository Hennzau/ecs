use ecs::prelude::*;

use winit::{
    event::{
        Event,
        WindowEvent,
    },
    platform::pump_events::EventLoopExtPumpEvents,
};

pub use physics::maths::{
    Position2Di32 as Position,
    Scale2Du32 as Scale,
};

type WinitWindow = winit::window::Window;
type WinitEventLoop = winit::event_loop::EventLoop<()>;

#[derive(Component)]
pub struct Window {
    pub winit_window: Option<WinitWindow>,
}

impl Window {
    pub fn new() -> Self {
        return Self {
            winit_window: None
        };
    }
}

pub struct WindowController {
    event_loop: WinitEventLoop,
    count: usize,
}

impl WindowController {
    pub fn new() -> CustomSystem {
        return SystemBuilder::create_system(Self {
            event_loop: WinitEventLoop::new().unwrap(),
            count: 0,
        });
    }
}

impl System for WindowController {
    fn components(&self) -> AHashSet<ComponentID> {
        return SystemBuilder::focus_on(&[
            Window::component_id(),
            Position::component_id(),
            Scale::component_id(),
        ]);
    }


    fn types(&self) -> AHashSet<SystemType> {
        return SystemBuilder::executed_on(&[SystemType::JOIN, SystemType::QUIT, SystemType::TICK]);
    }

    fn on_join(&mut self, entities: &[Entity], world: &mut World) {
        for &entity in entities {
            let position = world.try_get_component::<Position>(entity).unwrap().clone();
            let scale = world.try_get_component::<Scale>(entity).unwrap().clone();
            let title = match world.try_get_component::<basic::components::Label>(entity) {
                Some(label) => label.text.clone(),
                None => entity.to_string(),
            };

            let window = world.try_get_mut_component::<Window>(entity).unwrap();

            if window.winit_window.is_none() {
                let mut builder = winit::window::WindowBuilder::new();
                builder = builder.with_title(title);
                builder = builder.with_inner_size(winit::dpi::LogicalSize::new(scale.width, scale.height));
                builder = builder.with_position(winit::dpi::LogicalPosition::new(position.x, position.y));

                // TODO: add a decoration component and use it here

                window.winit_window = builder.build(&self.event_loop).ok();
            }

            self.count += 1;
        }
    }

    fn on_quit(&mut self, entities: &[Entity], world: &mut World) {
        for _ in 0..entities.len() {
            self.count -= 1;

            if self.count == 0 {
                world.send_event(Box::new(basic::events::CloseApplication {}));
            }
        }
    }

    fn on_tick(&mut self, _delta_time: f32, entities: &[Entity], world: &mut World) {
        let status = self.event_loop.pump_events(Some(std::time::Duration::ZERO), |event, _| {
            // Retrieve window event
            if let Event::WindowEvent { event, window_id } = event {
                // Retrieve all entities and get the window component

                for &entity in entities {
                    let window = world.try_get_mut_component::<Window>(entity).unwrap();
                    if let Some(winit_window) = &mut window.winit_window {

                        // Match event and check if the window event is for the current window
                        match event {
                            WindowEvent::CloseRequested => {
                                if winit_window.id() == window_id {
                                    world.send_event(Box::new(basic::events::TryRemoveComponent {
                                        entity: entity,
                                        component_id: Window::component_id(),
                                    }));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        if let winit::platform::pump_events::PumpStatus::Exit(exit_code) = status {
            log::warn!("Exit code: {}", exit_code);

            world.send_event(Box::new(basic::events::CloseApplication {}));
        }
    }
}