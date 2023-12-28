/*
This is the main module for stocking entities in different memory, in order to iterate over them efficiently
 */

mod graph;
mod mapping;
mod pool;
pub mod factory;
pub mod storage;
pub mod helper;

pub use mapping::MemoryMapping;
pub use mapping::MemoryMappingDescriptor;