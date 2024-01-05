/// This sub-crate provides algorithms and functions to smartly sort entities, components and systems
/// in order to provide fast-access to them, without the need of massive iterate-testing.

pub mod entities;
mod mapping;
mod flat;