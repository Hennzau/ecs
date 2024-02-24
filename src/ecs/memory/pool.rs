use crate::ecs::core::component::{
    ComponentID,
    ComponentIndex,
    AnyComponent,
};

pub type PoolIndex = usize;

pub trait AnyPool {
    fn id(&self) -> ComponentID;
    fn type_id() -> ComponentID where Self: Sized;

    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;

    fn push_any(&mut self, value: Box<dyn AnyComponent>) -> Option<ComponentIndex>;

    fn swap(&mut self, a: ComponentIndex, b: ComponentIndex);

    fn pop(&mut self) -> Option<Box<dyn AnyComponent>>;
}

pub struct Pool<T> where T: AnyComponent + 'static {
    id: ComponentID,
    components: Vec<T>,
}

impl<T> Pool<T> where T: AnyComponent + 'static {
    pub fn new() -> Self {
        return Pool {
            id: T::type_id(),
            components: Vec::new(),
        };
    }

    pub fn push(&mut self, component: T) -> ComponentIndex {
        self.components.push(component);

        return self.components.len() - 1;
    }
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

    fn push_any(&mut self, value: Box<dyn AnyComponent>) -> Option<ComponentIndex> {
        let id = value.id();

        if let Ok(v) = value.into_any().downcast::<T>() {
            self.push(*v);

            return Some(self.components.len() - 1);
        }

        log::warn!("Failed to downcast component {} into pool {}", id, self.id());

        return None;
    }

    fn swap(&mut self, a: ComponentIndex, b: ComponentIndex) {
        self.components.swap(a, b);
    }

    fn pop(&mut self) -> Option<Box<dyn AnyComponent>> {
        return self.components.pop().and_then(|component| {
            return Some(component.into_box());
        })
    }
}