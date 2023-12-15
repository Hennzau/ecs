
# hnz

## The macro crate for ecs

This crate gives the user the ability to easily create Components data structures.

It basically implement the ComponentTrait on the data structure affected by the **derive** macro :
```rust
#[derive(Component)]
```

An example is :
```rust
#[derive(Component)]
struct Position3Df32 {
	x: f32,
	y: f32,
	z: f32
}
```
You can then use this fresh new Component everywhere HNZ needs a ComponentTrait type.

## Explanation :

The above example became, after deriving it :
```rust
struct Position3Df32 {
	x: f32,
	y: f32,
	z: f32
}

impl hnz::ecs::component::ComponentTrait for Position3Df32 {
	fn id() -> u64 {  
		use std::{  
			collections::hash_map::DefaultHasher,  
			hash::{  
				Hash,  
				Hasher  
			}  
		};  

		let id_str = std::any::type_name::<Self>();  
		let mut hasher = DefaultHasher::new();  

		id_str.hash(&mut hasher);  

		hasher.finish()  
	}
}
```
