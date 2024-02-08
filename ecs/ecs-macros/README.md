
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

impl AnyComponent for Position3Df32 {
    fn id(&self) -> ComponentID {
        let hasher = RandomState::with_seed(0);
    
        let id_str = std::any::type_name::<Self>();
    
        return hasher.hash_one(id_str);
    }
    
    fn component_id() -> ComponentID {
        let hasher = RandomState::with_seed(0);
    
        let id_str = std::any::type_name::<Self>();
    
        return hasher.hash_one(id_str);
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        return self as &mut dyn std::any::Any;
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        return self as &dyn std::any::Any;
    }
    
    fn into_any (self: Box<Self>) -> Box<dyn std::any::Any> {
        return self;
    }
}
```
