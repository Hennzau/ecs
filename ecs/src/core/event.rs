use std::any::Any;

pub type EventID = u64;

pub use ecs_macros::Event;

pub trait AnyEvent {
    fn id(&self) -> EventID;

    fn event_id() -> EventID where Self: Sized;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any(&self) -> &dyn Any;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}