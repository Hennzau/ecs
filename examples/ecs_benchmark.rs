use std::time;
use hnz::ecs::prelude::*;

#[derive(Clone, Component)]
struct NameString {
    name: String,
}

#[derive(Clone, Component)]
struct ValueF32 {
    value: f32,
}

fn main() {
    let builder = ApplicationBuilder::new();

    let mut app = builder.build();

    // Create 16*1024 entities with the same 2 components
    // instances are cloned from a basic value

    let starting_time = time::Instant::now();
    let batch = app.spawn_batch(16 * 1024);

    let _ = app.batch_bundle(batch).add_component_clone(NameString { name: "entity".to_string() }).add_component_clone(ValueF32 { value: 0.0f32 }).try_build();

    println!("Benchmark for 16*1024 entities with 2 cloned components {:?}", starting_time.elapsed());

    // Create 16*1024 entities with the same 2 components
    // instances are already created when added

    let mut components_1 = Vec::new();
    let mut components_2 = Vec::new();

    for _ in 0..16 * 1024 {
        components_1.push(NameString { name: "entity".to_string() });
        components_2.push(ValueF32 { value: 0.0f32 });
    }

    let starting_time = time::Instant::now();

    let batch = app.spawn_batch(16 * 1024);
    let _ = app.batch_bundle(batch).add_component(components_1).add_component(components_2);

    println!("Benchmark for 16*1024 entities with 2 components already created {:?}", starting_time.elapsed());

}