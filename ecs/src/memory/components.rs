use ahash::AHashMap;

use crate::core::{
    entity::Entity,
    component::{
        ComponentID,
        AnyComponent,
    },
};

pub struct Components {
    /// Each element of the primary vector acts as a pool of components of the same type.
    components: Vec<Vec<Box<dyn AnyComponent>>>,

    /// Each element corresponds to indices from the pool of components of the same type.
    indices: Vec<AHashMap<Entity, usize>>,

    /// This map is used to find the right pool of components from the component ID.
    map: AHashMap<ComponentID, usize>,
}

impl Components {
    /// Creates a new instance of the `Components` struct.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the `Components` struct with initialized internal data structures.
    ///
    /// # Example
    ///
    /// ```
    /// let components = ecs::memory::components::Components::new();
    /// // Use the newly created instance of the `Components` struct.
    /// ```
    ///
    /// The method initializes the internal state of the `Components` struct.
    /// It returns a new instance ready to be used for managing components in hnz ECS.
    pub fn new() -> Self {
        return Self {
            components: Vec::new(),
            indices: Vec::new(),
            map: AHashMap::new(),
        };
    }

    /// Downcasts a `Box<dyn AnyComponent>` into a `&T` if possible.
    ///
    /// # Arguments
    ///
    /// * `component` - A reference to a boxed trait object implementing `AnyComponent`.
    ///
    /// # Returns
    ///
    /// Returns `Some(&T)` if the downcast is successful, providing a reference to the downcasted type `T`.
    /// Returns `None` if the downcast is not possible.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let any_component: Box<dyn AnyComponent> = Box::new(SpecificComponent {});
    ///
    /// // Attempt to downcast the boxed trait object to a specific type.
    /// if let Some(component_ref) = convert::<SpecificComponent>(&any_component) {
    ///     // Successfully downcasted to the desired type. Use the reference.
    /// } else {
    ///     // Unable to downcast to the desired type.
    /// }
    /// ```
    ///
    /// The method attempts to downcast a boxed trait object into a reference of the specified type `T`.
    /// It returns `Some(&T)` if the downcast is successful and `None` otherwise.
    pub fn convert<T: AnyComponent + 'static>(component: &Box<dyn AnyComponent>) -> Option<&T> {
        return component.as_any().downcast_ref::<T>();
    }

    /// Downcasts a `Box<dyn AnyComponent>` into a `&mut T` if possible.
    ///
    /// # Arguments
    ///
    /// * `component` - A mutable reference to a boxed trait object implementing `AnyComponent`.
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut T)` if the downcast is successful, providing a mutable reference to the downcasted type `T`.
    /// Returns `None` if the downcast is not possible.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let mut any_component: Box<dyn AnyComponent> = Box::new(SpecificComponent {});
    ///
    /// // Attempt to downcast the boxed trait object to a specific type.
    /// if let Some(component_ref) = convert_mut::<SpecificComponent>(&mut any_component) {
    ///     // Successfully downcasted to the desired type. Use the mutable reference.
    /// } else {
    ///     // Unable to downcast to the desired type.
    /// }
    /// ```
    ///
    /// The method attempts to downcast a boxed trait object into a mutable reference of the specified type `T`.
    /// It returns `Some(&mut T)` if the downcast is successful and `None` otherwise.
    pub fn convert_mut<T: AnyComponent + 'static>(component: &mut Box<dyn AnyComponent>) -> Option<&mut T> {
        return component.as_any_mut().downcast_mut::<T>();
    }

    /// Downcasts an `Option<&Box<dyn AnyComponent>>` into an `Option<&T>` if possible.
    ///
    /// # Arguments
    ///
    /// * `component` - An option containing a reference to a boxed trait object implementing `AnyComponent`.
    ///
    /// # Returns
    ///
    /// Returns `Some(&T)` if the downcast is successful, providing a reference to the downcasted type `T`.
    /// Returns `None` if the downcast is not possible or if the input option is `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let any_component: Box<dyn AnyComponent> = Box::new(SpecificComponent {});
    /// let option_component: Option<&Box<dyn AnyComponent>> = Some (&any_component);
    ///
    /// // Attempt to downcast the option into a specific type.
    /// if let Some(component_ref) = convert_ok::<SpecificComponent>(option_component) {
    ///     // Successfully downcasted to the desired type. Use the reference.
    /// } else {
    ///     // Unable to downcast to the desired type or the option is None.
    /// }
    /// ```
    ///
    /// The method attempts to downcast an option containing a boxed trait object into a reference of the specified type `T`.
    /// It returns `Some(&T)` if the downcast is successful and `None` otherwise.
    pub fn convert_ok<T: AnyComponent + 'static>(component: Option<&Box<dyn AnyComponent>>) -> Option<&T> {
        return component.and_then(|component| component.as_any().downcast_ref::<T>());
    }

    /// Downcasts an `Option<&mut Box<dyn AnyComponent>>` into an `Option<&mut T>` if possible.
    ///
    /// # Arguments
    ///
    /// * `component` - An option containing a mutable reference to a boxed trait object implementing `AnyComponent`.
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut T)` if the downcast is successful, providing a mutable reference to the downcasted type `T`.
    /// Returns `None` if the downcast is not possible or if the input option is `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let any_component: Box<dyn AnyComponent> = Box::new(SpecificComponent {});
    /// let option_component: Option<&Box<dyn AnyComponent>> = Some (&any_component);
    ///
    /// // Attempt to downcast the option into a specific type.
    /// if let Some(component_ref) = convert_mut_ok::<SpecificComponent>(option_component) {
    ///     // Successfully downcasted to the desired type. Use the mutable reference.
    /// } else {
    ///     // Unable to downcast to the desired type or the option is None.
    /// }
    /// ```
    ///
    /// The method attempts to downcast an option containing a mutable boxed trait object into a mutable reference of the specified type `T`.
    /// It returns `Some(&mut T)` if the downcast is successful and `None` otherwise.
    pub fn convert_mut_ok<T: AnyComponent + 'static>(component: Option<&mut Box<dyn AnyComponent>>) -> Option<&mut T> {
        return component.and_then(|component| component.as_any_mut().downcast_mut::<T>());
    }

    /// Returns `true` if the given entity has the given component. It first checks if the pool exists and then checks
    /// if the pool contains the entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which the presence of the component is checked.
    /// * `id` - The identifier of the component to check for.
    ///
    /// # Returns
    ///
    /// Returns `true` if the entity has the specified component, and the pool exists. Otherwise, returns `false`.
    ///
    /// # Example
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let entity = 0 as Entity; // Let's imagine that '0' is the id of an existing entity spawned in the application
    ///
    /// let components = ecs::memory::components::Components::new();
    /// let _ = components.try_add_any_component(entity, Box::new(SpecificComponent {}));
    ///
    /// assert!(components.contains(entity, SpecificComponent::component_id()));
    /// ```
    pub fn contains(&self, entity: Entity, id: ComponentID) -> bool {
        return match self.map.get(&id) {
            Some(index) => match self.indices.get(index.clone()) {
                Some(indices) => indices.contains_key(&entity),
                None => false
            },
            None => false
        };
    }

    /// Adds a component to the given entity. If the entity already has the component, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which the component should be added.
    /// * `value` - A boxed trait object implementing `AnyComponent` representing the component to be added.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the component is successfully added to the entity.
    /// Returns `Err(())` if the entity already has the component.
    ///
    /// # Example
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let entity = 0 as Entity; // Let's imagine that '0' is the id of an existing entity spawned in the application
    ///
    /// let mut components = ecs::memory::components::Components::new();
    ///
    /// assert!(components.try_add_any_component(entity, Box::new(SpecificComponent {})).is_ok());
    /// ```
    pub fn try_add_any_component(&mut self, entity: Entity, value: Box<dyn AnyComponent>) -> Result<(), ()> {
        let id = value.id();

        if self.contains(entity, id) {
            return Err(());
        }

        if let Some(index) = self.map.get(&id).cloned() {
            if let (Some(components), Some(indices)) = (self.components.get_mut(index), self.indices.get_mut(index)) {
                let in_index = components.len();
                indices.insert(entity, in_index);
                components.push(value);

                return Ok(());
            }
        } else {
            let index = self.components.len();
            self.components.push(vec![value]);
            self.indices.push(AHashMap::from([(entity, 0)]));
            self.map.insert(id, index);

            return Ok(());
        }

        return Err(());
    }

    /// Attempts to remove a component from the given entity. If the entity does not have the specified component, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity from which the component should be removed.
    /// * `id` - The identifier of the component to be removed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Box<dyn AnyComponent>)` with the removed component if successful.
    /// Returns `Err(())` if the entity does not have the specified component.
    ///
    /// # Example
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let entity = 0 as Entity; // Let's imagine that '0' is the id of an existing entity spawned in the application
    ///
    /// let mut components = ecs::memory::components::Components::new();
    ///
    /// assert!(components.try_add_any_component(entity, Box::new(SpecificComponent {})).is_ok());
    /// assert!(components.try_remove_any_component(entity, SpecificComponent::component_id()).is_ok());
    ///
    /// ```
    pub fn try_remove_any_component(&mut self, entity: Entity, id: ComponentID) -> Result<Box<dyn AnyComponent>, ()> {
        if !self.contains(entity, id) {
            return Err(());
        }

        if let Some(index) = self.map.get(&id).cloned() {
            if let (Some(components), Some(indices)) = (self.components.get_mut(index), self.indices.get_mut(index)) {
                let last_in_index = components.len() - 1;

                let last = indices.iter().find_map(|(key, value)| if value.clone() == last_in_index { Some(key) } else { None });

                if let Some(last_entity) = last.cloned() {
                    if let Some(in_index) = indices.get(&entity).cloned() {
                        indices.insert(last_entity, in_index);
                        indices.remove(&entity);

                        return Ok(components.swap_remove(in_index));
                    }
                }
            }
        }

        return Err(());
    }

    /// Returns a reference to the component of the given entity if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to retrieve the component.
    /// * `id` - The identifier of the component to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns `Some(&Box<dyn AnyComponent>)` with a reference to the component if it exists.
    /// Returns `None` if the entity does not have the specified component.
    ///
    /// # Example
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let entity = 0 as Entity; // Let's imagine that '0' is the id of an existing entity spawned in the application
    ///
    /// let mut components = ecs::memory::components::Components::new();
    ///
    /// assert!(components.try_add_any_component(entity, Box::new(SpecificComponent {})).is_ok());
    ///
    /// if let Some (any_component) = components.try_get_any_component (entity, SpecificComponent::component_id()) {
    ///     // Now 'any_component' is a '&Box<dyn AnyComponent>>'
    /// }
    /// ```
    pub fn try_get_any_component(&self, entity: Entity, id: ComponentID) -> Option<&Box<dyn AnyComponent>> {
        return self.map.get(&id).cloned().and_then(
            |index| self.components.get(index).and_then(
                |components| self.indices.get(index).and_then(
                    |indices| indices.get(&entity).cloned().and_then(
                        |in_index| components.get(in_index)))));
    }

    /// Returns a mutable reference to the component of the given entity if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to retrieve the mutable reference to the component.
    /// * `id` - The identifier of the component to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut Box<dyn AnyComponent>)` with a mutable reference to the component if it exists.
    /// Returns `None` if the entity does not have the specified component.
    ///
    /// # Example
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let entity = 0 as Entity; // Let's imagine that '0' is the id of an existing entity spawned in the application
    ///
    /// let mut components = ecs::memory::components::Components::new();
    ///
    /// assert!(components.try_add_any_component(entity, Box::new(SpecificComponent {})).is_ok());
    ///
    /// if let Some (any_component) = components.try_get_any_mut_component (entity, SpecificComponent::component_id()) {
    ///     // Now 'any_component' is a '&mut Box<dyn AnyComponent>>'
    /// }
    /// ```
    pub fn try_get_any_mut_component(&mut self, entity: Entity, id: ComponentID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.map.get(&id).cloned().and_then(
            |index| self.components.get_mut(index).and_then(
                |components| self.indices.get(index).and_then(
                    |indices| indices.get(&entity).cloned().and_then(
                        |in_index| components.get_mut(in_index)))));
    }

    /// Returns a reference to the component of the given entity if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to retrieve the component.
    ///
    /// # Returns
    ///
    /// Returns `Some(&T)` with a reference to the component of type `T` if it exists.
    /// Returns `None` if the entity does not have the specified component.
    ///
    /// # Example
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let entity = 0 as Entity; // Let's imagine that '0' is the id of an existing entity spawned in the application
    ///
    /// let mut components = ecs::memory::components::Components::new();
    ///
    /// assert!(components.try_add_any_component(entity, Box::new(SpecificComponent {})).is_ok());
    ///
    /// if let Some (any_component) = components.try_get_component::<SpecificComponent> (entity) {
    ///     // Now 'any_component' is a '&SpecificComponent'
    /// }
    /// ```
    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: Entity) -> Option<&T> {
        return Self::convert_ok(self.try_get_any_component(entity, T::component_id()));
    }

    /// Returns a mutable reference to the component of the given entity if it exists.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to retrieve the mutable reference to the component.
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut T)` with a mutable reference to the component of type `T` if it exists.
    /// Returns `None` if the entity does not have the specified component.
    ///
    /// # Example
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// struct SpecificComponent {}
    ///
    /// let entity = 0 as Entity; // Let's imagine that '0' is the id of an existing entity spawned in the application
    ///
    /// let mut components = ecs::memory::components::Components::new();
    ///
    /// assert!(components.try_add_any_component(entity, Box::new(SpecificComponent {})).is_ok());
    ///
    /// if let Some (any_component) = components.try_get_mut_component::<SpecificComponent> (entity) {
    ///     // Now 'any_component' is a '&mut SpecificComponent'
    /// }
    /// ```
    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        return Self::convert_mut_ok(self.try_get_any_mut_component(entity, T::component_id()));
    }
}