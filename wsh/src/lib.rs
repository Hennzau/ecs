use std::thread::sleep;
use std::time::Duration;
use ecs::core::{
    entity::Entity,
    sub_app::SubApp,
    component::{
        AnyComponent,
        Component,
        ComponentBuilder,
    },
    system::System,
};

use maths::{
    Position2Di32,
    Scale2Di32,
};

use winit::{
    event_loop::EventLoop,
    window::{
        Window,
        WindowBuilder,
    },
    dpi::{
        LogicalPosition,
        LogicalSize,
    },
};
use winit::event::{Event, WindowEvent};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

#[derive(ComponentBuilder)]
pub struct HNZWindow {
    pub title: String,
    pub event_loop: Option<EventLoop<()>>,
    pub winit_window: Option<Window>,
}

impl HNZWindow {
    pub fn new(title: String) -> Self {
        return Self {
            title: title,
            event_loop: None,
            winit_window: None,
        };
    }
}

pub struct HNZWindowBuilder {}

impl System for HNZWindowBuilder {
    fn components(&self) -> Vec<Component> {
        return vec![
            HNZWindow::id(),
            Position2Di32::id(),
            Scale2Di32::id(),
        ];
    }

    fn on_startup(&mut self, entities: &[Entity], app: &mut SubApp) {
        for entity in entities {
            let position = app.try_get_component::<Position2Di32>(entity).unwrap().clone();
            let scale = app.try_get_component::<Scale2Di32>(entity).unwrap().clone();

            let window = app.try_get_component_mut::<HNZWindow>(entity).unwrap();
            let title = window.title.clone();

            if window.event_loop.is_none() {
                window.event_loop = match EventLoop::new() {
                    Ok(event_loop) => Some(event_loop),
                    Err(e) => {
                        log::warn!("entity {} cannot build its event loop for component HNZWindow. {}", entity, e);

                        None
                    }
                }
            }

            if window.event_loop.is_some() && window.winit_window.is_none() {
                window.winit_window = match WindowBuilder::new()
                    .with_title(title)
                    .with_inner_size(LogicalSize::new(scale.width, scale.height))
                    .with_position(LogicalPosition::new(position.x, position.y))
                    .build(window.event_loop.as_ref().unwrap()) {
                    Ok(window) => Some(window),
                    Err(e) => {
                        log::warn!("Entity {} cannot build its own winit_window. {}", entity, e);

                        None
                    }
                }
            }
        }
    }
}

pub struct HNZWindowUpdater {}

impl System for HNZWindowUpdater {
    fn components(&self) -> Vec<Component> {
        return vec![
            HNZWindow::id()
        ];
    }

    fn on_update(&mut self, _delta_time: f32, entities: &[Entity], app: &mut SubApp) {
        for entity in entities {
            let window = app.try_get_component_mut::<HNZWindow>(entity).unwrap();

            let timeout = Some(Duration::ZERO);

            if let Some(event_loop) = &mut window.event_loop {
                if let Some(window) = &mut window.winit_window {
                    let status = event_loop.pump_events(timeout, |event, event_loop| {
                        if let Event::WindowEvent { window_id, event } = &event {
                            log::info!("{window_id:?} : {event:?}");
                        }

                        match event {
                            Event::WindowEvent {
                                event: WindowEvent::CloseRequested,
                                window_id,
                            } if window_id == window.id() => event_loop.exit(),
                            Event::AboutToWait => {
                                window.request_redraw();
                            }
                            _ => (),
                        }
                    });

                    if let PumpStatus::Exit(exit_code) = status {
                        println!("EXIT");
                    }
                }
            }

            sleep(Duration::from_millis(16));
        }
    }
}