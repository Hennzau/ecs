use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    core::{
        component::{
            Component,
            AnyComponent,
            Group,
        },
        entity::Entity,
    },
    memory::{
        factory::Factory,
        storage::PackedEntities,
    },
};

pub fn add_get_or_get_component<'a, T: AnyComponent + 'static>(components: &mut HashMap<Entity, HashSet<Component>>, packed: &mut PackedEntities, factory: &'a mut Factory, entity: &Entity, value: T) -> (&'a mut T, HashSet<Group>) {
    let mut groups = HashSet::<Group>::new();
    let previous_components = components.get(entity).cloned();

    if try_add(components, entity, T::id()) {
        if let Some(components) = previous_components {
            groups = packed.process_add(entity, &components, &HashSet::from([T::id()]));
        }
    }

    return (factory.add_get_or_get_component(entity, value), groups);
}

pub fn try_add_component<T: AnyComponent + 'static>(components: &mut HashMap<Entity, HashSet<Component>>, packed: &mut PackedEntities, factory: &mut Factory, entity: &Entity, value: T) -> (bool, HashSet<Group>) {
    let mut groups = HashSet::<Group>::new();
    let previous_components = components.get(entity).cloned();

    if try_add(components, entity, T::id()) {
        if let Some(components) = previous_components {
            groups = packed.process_add(entity, &components, &HashSet::from([T::id()]));
        }
    }

    return (factory.try_add_component(entity, value), groups);
}

pub fn try_remove_get_component_any(components: &mut HashMap<Entity, HashSet<Component>>, packed: &mut PackedEntities, factory: &mut Factory, entity: &Entity, id: Component) -> (Option<Box<dyn AnyComponent>>, HashSet<Group>) {
    let mut groups = HashSet::<Group>::new();

    if try_remove(components, entity, id) {
        if let Some(components) = components.get(entity) {
            groups = packed.process_remove(entity, components, &HashSet::from([id]));
        }
    }

    return (factory.try_remove_get_component_any(entity, id), groups);
}

pub fn try_remove_get_component<T: AnyComponent + 'static>(components: &mut HashMap<Entity, HashSet<Component>>, packed: &mut PackedEntities, factory: &mut Factory, entity: &Entity) -> (Option<Box<T>>, HashSet<Group>) {
    let mut groups = HashSet::<Group>::new();

    if try_remove(components, entity, T::id()) {
        if let Some(components) = components.get(entity) {
            groups = packed.process_remove(entity, components, &HashSet::from([T::id()]));
        }
    }

    return (factory.try_remove_get_component::<T>(entity), groups);
}

pub fn try_remove_component_any(components: &mut HashMap<Entity, HashSet<Component>>, packed: &mut PackedEntities, factory: &mut Factory, entity: &Entity, id: Component) -> (bool, HashSet<Group>) {
    let mut groups = HashSet::<Group>::new();

    if try_remove(components, entity, id) {
        if let Some(components) = components.get(entity) {
            groups = packed.process_remove(entity, components, &HashSet::from([id]));
        }
    }

    return (factory.try_remove_component_any(entity, id), groups);
}

pub fn try_remove_component<T: AnyComponent + 'static>(components: &mut HashMap<Entity, HashSet<Component>>, packed: &mut PackedEntities, factory: &mut Factory, entity: &Entity) -> (bool, HashSet<Group>) {
    let mut groups = HashSet::<Group>::new();

    if try_remove(components, entity, T::id()) {
        if let Some(components) = components.get(entity) {
            groups = packed.process_remove(entity, components, &HashSet::from([T::id()]));
        }
    }

    return (factory.try_remove_component::<T>(entity), groups);
}

pub fn try_add(components: &mut HashMap<Entity, HashSet<Component>>, entity: &Entity, id: Component) -> bool {
    if let Some(components) = components.get_mut(entity) {
        if components.contains(&id) {
            return false;
        }

        components.insert(id);
    } else {
        let mut set = HashSet::new();
        set.insert(id);

        components.insert(entity.clone(), set);
    }

    return true;
}

pub fn try_remove(components: &mut HashMap<Entity, HashSet<Component>>, entity: &Entity, id: Component) -> bool {
    if let Some(components) = components.get_mut(entity) {
        if !components.contains(&id) {
            return false;
        }

        components.remove(&id);

        return true;
    }

    return false;
}