pub type Entity = u64;

pub type EntityBatch = (Entity, Entity, Vec<Entity>);

pub type EntitySet = Vec<Entity>;

pub type EntityIndex = usize;

pub const NULL_ENTITY: usize = usize::MAX;

pub fn as_key(entity: Entity) -> usize {
    return entity as usize;
}