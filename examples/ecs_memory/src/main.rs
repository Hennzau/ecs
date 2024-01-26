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

    #[derive(Component)]
    pub struct A {}

    #[derive(Component)]
    pub struct B {}

    #[derive(Component)]
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
    builder.add_join_system(Box::new(systems::A {}));
    builder.add_join_system(Box::new(systems::B {}));
    builder.add_join_system(Box::new(systems::C {}));
    builder.add_join_system(Box::new(systems::AB {}));
    builder.add_join_system(Box::new(systems::AC {}));
    builder.add_join_system(Box::new(systems::BC {}));
    builder.add_join_system(Box::new(systems::ABC {}));

    let mut app = builder.build();

    let mut entities = Vec::new();

    for i in 0..25 {
        let entity = app.spawn();

        if i % 2 == 0 {
            let _ = app.try_add_component(&entity, components::A {});
        }

        if i % 3 == 0 {
            let _ = app.try_add_component(&entity, components::B {});
        }

        if i % 4 == 0 {
            let _ = app.try_add_component(&entity, components::C {});
        }

        if i % 8 == 0 {
            let _ = app.try_remove_component::<components::A>(&entity);
            let _ = app.try_remove_component::<components::C>(&entity);
        }

        entities.push(entity);
    }

    println!("A: {}", components::A::component_id());
    println!("B: {}", components::B::component_id());
    println!("C: {}", components::C::component_id());
    println!("AB: {}", components_to_group(&vec![components::A::component_id(), components::B::component_id()].into_iter().collect()));
    println!("AC: {}", components_to_group(&vec![components::A::component_id(), components::C::component_id()].into_iter().collect()));
    println!("BC: {}", components_to_group(&vec![components::B::component_id(), components::C::component_id()].into_iter().collect()));
    println!("ABC: {}", components_to_group(&vec![components::A::component_id(), components::B::component_id(), components::C::component_id()].into_iter().collect()));

    println!("Entities:\n====================");

    for entities in app.entities() {
        println!("{:?}", entities);
    }

    println!("Views:\n====================");

    println!("A: {:?}", app.try_view(components_to_group(&vec![components::A::component_id()].into_iter().collect())));
    println!("B: {:?}", app.try_view(components_to_group(&vec![components::B::component_id()].into_iter().collect())));
    println!("C: {:?}", app.try_view(components_to_group(&vec![components::C::component_id()].into_iter().collect())));
    println!("AB: {:?}", app.try_view(components_to_group(&vec![components::A::component_id(), components::B::component_id()].into_iter().collect())));
    println!("AC: {:?}", app.try_view(components_to_group(&vec![components::A::component_id(), components::C::component_id()].into_iter().collect())));
    println!("BC: {:?}", app.try_view(components_to_group(&vec![components::B::component_id(), components::C::component_id()].into_iter().collect())));
    println!("ABC: {:?}", app.try_view(components_to_group(&vec![components::A::component_id(), components::B::component_id(), components::C::component_id()].into_iter().collect())));
}
