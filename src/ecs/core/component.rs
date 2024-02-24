use std::any::Any;

pub use macros::Component;
pub use ahash::RandomState;

pub type ComponentID = u64;
pub type ComponentIndex = usize;

pub type ArchetypeID = u64;
pub type ArchetypeIndex = usize;

pub trait AnyComponent {
    fn type_id() -> ComponentID where Self: Sized;

    fn id(&self) -> ComponentID;

    fn into_box(self) -> Box<dyn AnyComponent>;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;

}