use std::sync::Arc;

use ecs::prelude::*;
use futures::executor::block_on;

use winit::{
    event,
    window,
    event_loop::EventLoop,
    platform::pump_events::EventLoopExtPumpEvents,
};

pub use physics::maths::{
    Position2Di32 as Position,
    Scale2Du32 as Scale,
};

struct WinitWindow {
    raw_window: Arc<window::Window>,
}

struct WgpuGraphics {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    capabilities: wgpu::SurfaceCapabilities,
    format: wgpu::TextureFormat,
}

#[derive(Component)]
pub struct Window {
    winit_window: Option<WinitWindow>,
    wgpu_graphics: Option<WgpuGraphics>,
}

impl Window {
    pub fn new() -> Self {
        return Self {
            winit_window: None,
            wgpu_graphics: None,
        };
    }
}

pub struct WindowController {
    event_loop: EventLoop<()>,
    count: usize,
    instance: wgpu::Instance,
}

impl WindowController {
    pub fn new() -> CustomSystem {
        return SystemBuilder::create_system(Self {
            event_loop: EventLoop::new().unwrap(),
            count: 0,
            instance: wgpu::Instance::default(),
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
        return SystemBuilder::executed_on(&[
            SystemType::TICK,
            SystemType::JOIN,
            SystemType::QUIT
        ]);
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

            let mut builder = winit::window::WindowBuilder::new();
            builder = builder.with_title(title);
            builder = builder.with_inner_size(winit::dpi::LogicalSize::new(scale.width, scale.height));
            builder = builder.with_position(winit::dpi::LogicalPosition::new(position.x, position.y));

            // TODO: add a decoration component and use it here

            window.winit_window = Some(WinitWindow {
                raw_window: Arc::new(builder.build(&self.event_loop).unwrap())
            });

            if let Some(winit_window) = &window.winit_window {
                let surface = self.instance.create_surface(winit_window.raw_window.clone()).ok();

                let adapter = block_on(self.instance.request_adapter(&wgpu::RequestAdapterOptions {
                    compatible_surface: surface.as_ref(),
                    ..Default::default()
                })).unwrap();

                let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                }, None)).unwrap();

                if let Some(surface) = surface {
                    let swapchain_capabilities = surface.get_capabilities(&adapter);
                    let swapchain_format = swapchain_capabilities.formats[0];

                    let mut config = surface
                        .get_default_config(&adapter, window.winit_window.as_ref().unwrap().raw_window.inner_size().width, window.winit_window.as_ref().unwrap().raw_window.inner_size().height)
                        .unwrap();

                    surface.configure(&device, &config);

                    window.wgpu_graphics = Some(WgpuGraphics {
                        surface: surface,
                        device,
                        queue,
                        capabilities: swapchain_capabilities,
                        format: swapchain_format,
                    });
                }
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


    fn on_tick(&mut self, _delta_time: f32, _entities: &[Entity], _world: &mut World) {

    }
}