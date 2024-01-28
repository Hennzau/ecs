/// This example shows how to use the ECS and how memory is mapped.
///
/// The principle of this example is to understand how memory can be mapped. The example consists in adding
/// component A to every entity that have a even index, component B to every entity that have a multiple of 3 and
/// component C to every entity that have a multiple of 4. Then, every entity that have a multiple of 8 will be removed
/// from component A and C.

use simple_logger::SimpleLogger;

use hnz::ecs::prelude::*;

pub mod components {
    use hnz::ecs::prelude::*;

    #[derive(Clone, Component)]
    pub struct A {}

    #[derive(Clone, Component)]
    pub struct B {}

    #[derive(Clone, Component)]
    pub struct C {}
}

pub mod systems {
    use hnz::ecs::prelude::*;

    use crate::components;

    pub struct A {}

    impl System for A {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![components::A::component_id()].into_iter().collect();
        }
    }

    pub struct B {}

    impl System for B {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![components::B::component_id()].into_iter().collect();
        }
    }

    pub struct C {}

    impl System for C {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![components::C::component_id()].into_iter().collect();
        }
    }

    pub struct AB {}

    impl System for AB {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![components::A::component_id(), components::B::component_id()].into_iter().collect();
        }
    }

    pub struct AC {}

    impl System for AC {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![components::A::component_id(), components::C::component_id()].into_iter().collect();
        }
    }

    pub struct BC {}

    impl System for BC {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![components::B::component_id(), components::C::component_id()].into_iter().collect();
        }
    }

    pub struct ABC {}

    impl System for ABC {
        fn components(&self) -> AHashSet<ComponentID> {
            return vec![components::A::component_id(), components::B::component_id(), components::C::component_id()].into_iter().collect();
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let mut builder = ApplicationBuilder::new();
    builder.add_systems(vec![
        SystemBuilder::new(systems::A {}),
        SystemBuilder::new(systems::B {}),
        SystemBuilder::new(systems::C {}),
        SystemBuilder::new(systems::AB {}),
        SystemBuilder::new(systems::AC {}),
        SystemBuilder::new(systems::BC {}),
        SystemBuilder::new(systems::ABC {}),
    ], vec![
        SystemType::JOIN
    ].into_iter().collect());

    let mut app = builder.build();

    let mut entities_2 = Vec::new();
    let mut entities_3 = Vec::new();
    let mut entities_4 = Vec::new();
    let mut entities_8 = Vec::new();

    for i in 0..25 {
        let entity = app.spawn();

        if i % 2 == 0 {
            entities_2.push(entity);
        }

        if i % 3 == 0 {
            entities_3.push(entity);
        }

        if i % 4 == 0 {
            entities_4.push(entity);
        }

        if i % 8 == 0 {
            entities_8.push(entity);
        }
    }

    let _ = app.try_multiple_add_component(&entities_2, components::A {});
    let _ = app.try_multiple_add_component(&entities_3, components::B {});
    let _ = app.try_multiple_add_component(&entities_4, components::C {});

    let _ = app.try_multiple_remove_component::<components::A>(&entities_8);
    let _ = app.try_multiple_remove_component::<components::C>(&entities_8);

    println!("A: {}", components::A::component_id());
    println!("B: {}", components::B::component_id());
    println!("C: {}", components::C::component_id());
    println!("AB: {}", group_id(&vec![components::A::component_id(), components::B::component_id()].into_iter().collect()));
    println!("AC: {}", group_id(&vec![components::A::component_id(), components::C::component_id()].into_iter().collect()));
    println!("BC: {}", group_id(&vec![components::B::component_id(), components::C::component_id()].into_iter().collect()));
    println!("ABC: {}", group_id(&vec![components::A::component_id(), components::B::component_id(), components::C::component_id()].into_iter().collect()));

    println!("Entities:\n====================");

    for entities in app.entities() {
        println!("{:?}", entities);
    }

    println!("Views:\n====================");

    println!("A: {:?}", app.try_view(group_id(&vec![components::A::component_id()].into_iter().collect())));
    println!("B: {:?}", app.try_view(group_id(&vec![components::B::component_id()].into_iter().collect())));
    println!("C: {:?}", app.try_view(group_id(&vec![components::C::component_id()].into_iter().collect())));
    println!("AB: {:?}", app.try_view(group_id(&vec![components::A::component_id(), components::B::component_id()].into_iter().collect())));
    println!("AC: {:?}", app.try_view(group_id(&vec![components::A::component_id(), components::C::component_id()].into_iter().collect())));
    println!("BC: {:?}", app.try_view(group_id(&vec![components::B::component_id(), components::C::component_id()].into_iter().collect())));
    println!("ABC: {:?}", app.try_view(group_id(&vec![components::A::component_id(), components::B::component_id(), components::C::component_id()].into_iter().collect())));
}
