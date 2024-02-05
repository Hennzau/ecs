use std::any::Any;

pub type EventID = u64;

pub use ecs_macros::Event;

/// General trait that must be implemented for structs that must be understand as Event
/// The user doesn't have to manipulate this trait, everything is handled by the ECS crate and the
/// proc macro [derive(Event)]
pub trait AnyEvent {
    fn id(&self) -> EventID;

    fn event_id() -> EventID where Self: Sized;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any(&self) -> &dyn Any;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}