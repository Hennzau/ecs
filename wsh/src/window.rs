use ecs::prelude::*;

use physics::maths::{
    Position2Di32,
    Scale2Du32
};

type WinitWindow = winit::window::Window;
type WinitEventLoop = winit::event_loop::EventLoop<()>;

#[derive(Component)]
pub struct Window {
    pub winit_window: Option<WinitWindow>
}

impl Window {
    pub fn new() -> Self {
        return Self {
            winit_window: None
        }
    }
}

pub struct WindowController {
    event_loop: WinitEventLoop,
}

impl WindowController {
    pub fn new() -> Self {
        return Self {
            event_loop: WinitEventLoop::new().unwrap(),
        }
    }
}

impl System for WindowController {
    fn components(&self) -> AHashSet<ComponentID> {
        return vec![
            Window::component_id(),
            Position2Di32::component_id(),
            Scale2Du32::component_id(),
        ].into_iter().collect();
    }

    /// This function is called when an entity joins this system.

    fn on_join(&mut self, entities: &[Entity], world: &mut World) {
        for entity in entities {
            let position = world.try_get_component::<Position2Di32>(entity).unwrap().clone();
            let scale = world.try_get_component::<Scale2Du32>(entity).unwrap().clone();
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
        }
    }

    fn on_tick(&mut self, delta_time: f32, entities: &[Entity], world: &mut World) {}
}