pub use ecs_macros::Component;

use std::any::Any;

pub trait ToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> ToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub trait ComponentTrait: ToAny {
    fn id() -> u64;
}