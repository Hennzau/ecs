pub mod archetype_graph;

/// This module represents the archetype graph for entities and components.
/// This is the base concept of Archetypes Based ECS, but here we use this graph to only represent the relationships between archetypes.
/// It does not own any data for components.
///
/// In fact, this graph also stores the current archetype of an entity.
pub mod graph;