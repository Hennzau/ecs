use crate::ecs::core::component::{
    ComponentID,
    AnyComponent
};

pub struct Pool<T> where T: AnyComponent {
    id: ComponentID,
    components: Vec<T>
}

impl<T> Pool<T> where T: AnyComponent {
    pub fn new() -> Pool<T> {
        return Pool {
            id: T::type_id(),
            components: Vec::new()
        }
    }
}

pub trait AnyPool {
    fn id(&self) -> ComponentID;
    fn type_id() -> ComponentID where Self: Sized;

    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
}

impl<T> AnyPool for Pool<T> where T: AnyComponent + 'static {
    fn id(&self) -> ComponentID {
        return self.id;
    }

    fn type_id() -> ComponentID {
        return T::type_id();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        return self;
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        return self;
    }

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        return self;
    }
}