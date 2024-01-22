use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    memory::{
        entities::Entities,
        mapping::{
            MemoryMapping,
            MemoryMappingDescriptor,
        },
        components::Components,
    },
    core::{
        component::{
            ComponentID,
            AnyComponent
        },
        entity::Entity,
    },
};

pub struct Application {
    mapping: MemoryMapping,
    entities: Entities,
    components: Components,

    next_entity: Entity,
    components_tracker: HashMap<Entity, HashSet<ComponentID>>,
}

impl Application {
    pub fn new(descriptor: MemoryMappingDescriptor) -> Self {
        let mapping = MemoryMapping::new(descriptor);

        return Self {
            components: Components::new(),
            entities: mapping.create_storage(),
            mapping: mapping,

            next_entity: 0 as Entity,
            components_tracker: HashMap::new(),
        };
    }

    pub fn spawn(&mut self) -> Entity {
        let result = self.next_entity;

        self.components_tracker.insert(self.next_entity as Entity, HashSet::new());
        self.next_entity += 1;

        return result;
    }

    pub fn try_add_any_component(&mut self, entity: &Entity, id: ComponentID, value: Box<dyn AnyComponent>) -> Result<(), ()> {
        return match self.components.try_add_any_component(entity, id, value) {
            Ok(()) => {
                if let Some(previous_components) = self.components_tracker.get_mut(entity) {
                    let groups = self.mapping.get_next_membership(&previous_components, &HashSet::from([id]));

                    for group in groups {
                        let result = self.entities.try_add_group(group, &[entity.clone()]);
                        if let Err(e) = result {
                            log::warn!("Error while adding entity to group: {:?}", e);
                        }
                    }

                    previous_components.insert(id);
                }

                Ok(())
            }
            Err(()) => Err(())
        };
    }

    pub fn try_add_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Result<(), ()> {
        return self.try_add_any_component(entity, T::id(), Box::from(value));
    }

    pub fn try_add_get_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Option<&T> {
        return match self.try_add_component::<T>(entity, value) {
            Ok(()) => self.try_get_component::<T>(entity),
            Err(()) => None
        };
    }

    pub fn try_add_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: &Entity, value: T) -> Option<&mut T> {
        return match self.try_add_component::<T>(entity, value) {
            Ok(()) => self.try_get_mut_component::<T>(entity),
            Err(()) => None
        };
    }

    pub fn try_remove_any_component(&mut self, entity: &Entity, id: ComponentID) -> Result<Box<dyn AnyComponent>, ()> {
        return match self.components.try_remove_any_component(entity, id) {
            Ok(any_component) => {
                if let Some(previous_components) = self.components_tracker.get_mut(entity) {
                    previous_components.remove(&id);

                    let groups = self.mapping.get_next_membership(&previous_components, &HashSet::from([id]));

                    for group in groups {
                        let result = self.entities.try_remove_group(group, &[entity.clone()]);
                        if let Err(e) = result {
                            log::warn!("Error while removing entity from group: {:?}", e);
                        }
                    }
                }

                Ok(any_component)
            }
            Err(()) => Err(())
        };
    }

    pub fn try_remove_get_any_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<Box<dyn AnyComponent>> {
        return self.try_remove_any_component(entity, T::id()).ok();
    }

    pub fn try_remove_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Result<(), ()> {
        return self.try_remove_any_component(entity, T::id()).map(|_| ());
    }

    pub fn try_get_any_component(&self, entity: &Entity, id: ComponentID) -> Option<&Box<dyn AnyComponent>> {
        return self.components.try_get_any_component(entity, id);
    }

    pub fn try_get_any_mut_component(&mut self, entity: &Entity, id: ComponentID) -> Option<&mut Box<dyn AnyComponent>> {
        return self.components.try_get_any_mut_component(entity, id);
    }

    pub fn try_get_component<T: AnyComponent + 'static>(&self, entity: &Entity) -> Option<&T> {
        return self.components.try_get_component::<T>(entity);
    }

    pub fn try_get_mut_component<T: AnyComponent + 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        return self.components.try_get_mut_component::<T>(entity);
    }
}

