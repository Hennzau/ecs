use crate::{
    core::{
        component::{
            ComponentID,
            AnyComponent,
        },
        entity::Entity,
    },
    application::Application,
};

pub struct Bundle<'a> {
    entity: Entity,

    components_to_add: Vec<Box<dyn AnyComponent>>,
    components_to_remove: Vec<ComponentID>,

    application: &'a mut Application,
}

impl Bundle<'_> {
    /// Creates a new instance of the Bundle for the specified entity and application.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity associated with the bundle.
    /// * `application` - A mutable reference to the application for applying the bundle operations.
    ///
    /// # Returns
    ///
    /// Returns a new Bundle instance with the specified entity and application.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let entity = application.spawn();
    ///
    /// let bundle = Bundle::new(entity, &mut application);
    /// ```

    pub fn new(entity: Entity, application: &mut Application) -> Bundle {
        return Bundle {
            entity: entity,
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
            application: application,
        };
    }

    /// Adds a component to the bundle for the specified entity.
    ///
    /// # Arguments
    ///
    /// * `component` - The component to be added to the entity.
    ///
    /// # Returns
    ///
    /// Returns the updated Bundle instance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let entity = application.spawn();
    ///
    /// let bundle = Bundle::new(entity, &mut application);
    /// bundle.add_component(TestComponent1 {});
    /// bundle.add_component(TestComponent2 {});
    /// ```

    pub fn add_component<T: AnyComponent + 'static>(mut self, component: T) -> Self {
        self.components_to_add.push(Box::new(component));

        return self;
    }

    /// Removes a component from the bundle for the specified entity.
    ///
    /// # Returns
    ///
    /// Returns the updated Bundle instance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let entity = application.spawn();
    ///
    /// let bundle = Bundle::new(entity, &mut application);
    /// bundle.add_component(TestComponent1 {});
    /// bundle.remove_component::<TestComponent1>();
    /// ```

    pub fn remove_component<T: AnyComponent + 'static>(mut self) -> Self {
        self.components_to_remove.push(T::component_id());

        return self;
    }

    /// Attempts to build and apply the bundle operations to the associated entity.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all operations are successfully applied, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let entity = application.spawn();
    ///
    /// let bundle = Bundle::new(entity, &mut application);
    /// bundle.add_component(TestComponent1 {});
    /// bundle.add_component(TestComponent2 {});
    ///
    /// let _ = bundle.try_build();
    ///
    /// // Now entity should have 2 components : TestComponent1 and TestComponent2
    /// ```

    pub fn try_build(self) -> Result<(), ()> {
        let mut result = Ok(());

        for component in self.components_to_add {
            let res = self.application.try_add_any_component(self.entity, component);
            if res.is_err() {
                result = Err(());
            }
        }

        for component in self.components_to_remove {
            let res = self.application.try_remove_any_component(self.entity, component);
            if res.is_err() {
                result = Err(());
            }
        }

        return result;
    }
}

pub struct BatchBundle<'a> {
    batch: (Entity, usize),

    components_to_add: Vec<Vec<Box<dyn AnyComponent>>>,
    components_to_remove: Vec<ComponentID>,

    application: &'a mut Application,
}

impl BatchBundle<'_> {
    /// Creates a new instance of the Bundle for the specified batch and application.
    ///
    /// # Arguments
    ///
    /// * `batch` - The batch associated with the bundle.
    /// * `application` - A mutable reference to the application for applying the bundle operations.
    ///
    /// # Returns
    ///
    /// Returns a new Bundle instance with the specified batch and application.
    /// Creates a new instance of the Bundle for the specified entity and application.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let batch = application.spawn_batch(100);
    ///
    /// let bundle = BatchBundle::new(batch, &mut application);
    /// ```

    pub fn new(batch: (Entity, usize), application: &mut Application) -> BatchBundle {
        return BatchBundle {
            batch: batch,
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
            application: application,
        };
    }

    /// Adds a component to the bundle for the specified batch.
    ///
    /// # Arguments
    ///
    /// * `components` - The components to be added to each entity of the batch, len must be equal to batch's amount.
    ///
    /// # Returns
    ///
    /// Returns the updated Bundle instance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let batch = application.spawn_batch(100);
    ///
    /// let bundle = BatchBundle::new(batch, &mut application);
    ///
    /// bundle.add_component(vec![TestComponent1 {}; 100]);
    /// bundle.add_component(vec![TestComponent2 {}; 100]);
    /// ```

    pub fn add_component<T: AnyComponent + 'static>(mut self, components: Vec<T>) -> Self {
        if components.len() != self.batch.1 {
            log::warn!("You tried to add components for this batch : {:?} but you did not pass enough components for all\
            entities in this batch", self.batch);

            return self;
        }

        let mut box_components = Vec::<Box<dyn AnyComponent>>::new();

        for component in components {
            box_components.push(Box::new(component));
        }

        self.components_to_add.push(box_components);

        return self;
    }

    /// Adds a component to the bundle for the specified batch.
    ///
    /// # Arguments
    ///
    /// * `component` - The component to be added to the batch. Must derive Clone because this value will be clone
    ///                 for all entities in the batch.
    ///
    /// # Returns
    ///
    /// Returns the updated Bundle instance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let batch = application.spawn_batch(100);
    ///
    /// let bundle = BatchBundle::new(batch, &mut application);
    ///
    /// bundle.add_component_clone(TestComponent1 {});
    /// bundle.add_component_clone(TestComponent2 {});
    /// ```

    pub fn add_component_clone<T: Clone + AnyComponent + 'static>(mut self, component: T) -> Self {
        let mut box_components = Vec::<Box<dyn AnyComponent>>::new();

        for _ in 0..self.batch.1 {
            box_components.push(Box::new(component.clone()));
        }

        self.components_to_add.push(box_components);

        return self;
    }

    /// Removes a component from the bundle for the specified batch.
    ///
    /// # Returns
    ///
    /// Returns the updated Bundle instance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let batch = application.spawn_batch(100);
    ///
    /// let bundle = BatchBundle::new(batch, &mut application);
    ///
    /// bundle.add_component(vec![TestComponent1 {}; 100]);
    /// bundle.remove_component::<TestComponent1> ();
    /// ```

    pub fn remove_component<T: AnyComponent + 'static>(mut self) -> Self {
        self.components_to_remove.push(T::component_id());

        return self;
    }

    /// Attempts to build and apply the bundle operations to the associated batch.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all operations are successfully applied, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let batch = application.spawn_batch(100);
    ///
    /// let bundle = BatchBundle::new(batch, &mut application);
    ///
    /// bundle.add_component(vec![TestComponent1 {}; 100]);
    ///
    /// bundle.try_build();
    ///
    /// // Now every entity in the batch should have TestComponent1
    /// ```

    pub fn try_build(self) -> Result<(), ()> {
        let mut result = Ok(());

        for components in self.components_to_add {
            let res = self.application.try_add_any_component_batch(self.batch, components);
            if res.is_err() {
                result = Err(());
            }
        }

        for component in self.components_to_remove {
            let res = self.application.try_remove_any_component_batch(self.batch, component);
            if res.is_err() {
                result = Err(());
            }
        }

        return result;
    }
}

pub struct SetBundle<'a> {
    entities: Vec<Entity>,

    components_to_add: Vec<Vec<Box<dyn AnyComponent>>>,
    components_to_remove: Vec<ComponentID>,

    application: &'a mut Application,
}

impl SetBundle<'_> {
    /// Creates a new instance of the Bundle for the specified set and application.
    ///
    /// # Arguments
    ///
    /// * `set` - The set associated with the bundle.
    /// * `application` - A mutable reference to the application for applying the bundle operations.
    ///
    /// # Returns
    ///
    /// Returns a new Bundle instance with the specified entity and application.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let set = application.spawn_set(100);
    ///
    /// let bundle = SetBundle::new(set, &mut application);
    /// ```

    pub fn new(entities: Vec<Entity>, application: &mut Application) -> SetBundle {
        return SetBundle {
            entities: entities,
            components_to_add: Vec::new(),
            components_to_remove: Vec::new(),
            application: application,
        };
    }

    /// Adds a component to the bundle for the specified set.
    ///
    /// # Arguments
    ///
    /// * `components` - The components to be added to each entity of the set, len must be equal to set's size.
    ///
    /// # Returns
    ///
    /// Returns the updated Bundle instance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let set = application.spawn_set(100);
    ///
    /// let bundle = SetBundle::new(set, &mut application);
    ///
    /// bundle.add_component(vec![TestComponent1 {}; 100]);
    /// ```

    pub fn add_component<T: AnyComponent + 'static>(mut self, components: Vec<T>) -> Self {
        if components.len() != self.entities.len() {
            log::warn!("You tried to add components for this set : {:?} but you did not pass enough components for all\
            entities in this batch", self.entities);

            return self;
        }

        let mut box_components = Vec::<Box<dyn AnyComponent>>::new();

        for component in components {
            box_components.push(Box::new(component));
        }

        self.components_to_add.push(box_components);

        return self;
    }

    /// Adds a component to the bundle for the specified set.
    ///
    /// # Arguments
    ///
    /// * `component` - The component to be added to the entity. Must derive Clone because this value will be clone
    ///                 for all entities in the set.
    ///
    /// # Returns
    ///
    /// Returns the updated Bundle instance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Clone, Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Clone, Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let set = application.spawn_set(100);
    ///
    /// let bundle = SetBundle::new(set, &mut application);
    ///
    /// bundle.add_component(TestComponent1 {});
    /// ```

    pub fn add_component_clone<T: Clone + AnyComponent + 'static>(mut self, component: T) -> Self {
        let mut box_components = Vec::<Box<dyn AnyComponent>>::new();

        for _ in 0..self.entities.len() {
            box_components.push(Box::new(component.clone()));
        }

        self.components_to_add.push(box_components);

        return self;
    }

    /// Removes a component from the bundle for the specified set.
    ///
    /// # Returns
    ///
    /// Returns the updated Bundle instance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let set = application.spawn_set(100);
    ///
    /// let bundle = SetBundle::new(set, &mut application);
    ///
    /// bundle.add_component(vec![TestComponent1 {}; 100]);
    /// bundle.remove_component::<TestComponent1> ();
    /// ```

    pub fn remove_component<T: AnyComponent + 'static>(mut self) -> Self {
        self.components_to_remove.push(T::component_id());

        return self;
    }

    /// Attempts to build and apply the bundle operations to the associated entity.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all operations are successfully applied, otherwise returns `Err(())`.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs::prelude::*;
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent1 {}
    ///
    /// #[derive(Component)]
    /// pub struct TestComponent2 {}
    ///
    /// let app_builder = ApplicationBuilder::new();
    /// let application = app_builder.build();
    ///
    /// let set = application.spawn_set(100);
    ///
    /// let bundle = SetBundle::new(set, &mut application);
    ///
    /// bundle.add_component(vec![TestComponent1 {}; 100]);
    /// bundle.try_build();
    ///
    /// // Now every entities in 'set' should have TestComponent1
    /// ```

    pub fn try_build(self) -> Result<(), ()> {
        let mut result = Ok(());

        for components in self.components_to_add {
            let res = self.application.try_add_any_component_set(&self.entities, components);

            if res.is_err() {
                result = Err(());
            }
        }

        for component in self.components_to_remove {
            let res = self.application.try_remove_any_component_set(&self.entities, component);

            if res.is_err() {
                result = Err(());
            }
        }

        return result;
    }
}