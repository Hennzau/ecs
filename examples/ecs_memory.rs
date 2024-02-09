extern crate hnz;

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
            return SystemBuilder::track_components(&[components::A::component_id()]);
        }
    }

    pub struct B {}

    impl System for B {
        fn components(&self) -> AHashSet<ComponentID> {
            return SystemBuilder::track_components(&[components::B::component_id()]);
        }
    }

    pub struct C {}

    impl System for C {
        fn components(&self) -> AHashSet<ComponentID> {
            return SystemBuilder::track_components(&[components::C::component_id()]);
        }
    }

    pub struct AB {}

    impl System for AB {
        fn components(&self) -> AHashSet<ComponentID> {
            return SystemBuilder::track_components(&[components::A::component_id(), components::B::component_id()]);
        }
    }

    pub struct AC {}

    impl System for AC {
        fn components(&self) -> AHashSet<ComponentID> {
            return SystemBuilder::track_components(&[components::A::component_id(), components::C::component_id()]);
        }
    }

    pub struct BC {}

    impl System for BC {
        fn components(&self) -> AHashSet<ComponentID> {
            return SystemBuilder::track_components(&[components::B::component_id(), components::C::component_id()]);
        }
    }

    pub struct ABC {}

    impl System for ABC {
        fn components(&self) -> AHashSet<ComponentID> {
            return SystemBuilder::track_components(&[components::A::component_id(), components::B::component_id(), components::C::component_id()]);
        }
    }
}

fn main() {
    let mut builder = ApplicationBuilder::new();
    builder.add_systems(vec![
        SystemBuilder::create_system(systems::A {}),
        SystemBuilder::create_system(systems::B {}),
        SystemBuilder::create_system(systems::C {}),
        SystemBuilder::create_system(systems::AB {}),
        SystemBuilder::create_system(systems::AC {}),
        SystemBuilder::create_system(systems::BC {}),
        SystemBuilder::create_system(systems::ABC {}),
    ], SystemBuilder::mix_types(&[
        SystemType::JOIN
    ]));

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

    let _ = app.try_add_component_set_clone(&entities_2, components::A {});
    let _ = app.try_add_component_set_clone(&entities_3, components::B {});
    let _ = app.try_add_component_set_clone(&entities_4, components::C {});

    let _ = app.set_bundle(entities_8).remove_component::<components::A>().remove_component::<components::C>().try_build();

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
