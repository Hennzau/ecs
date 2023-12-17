pub mod pool;

pub use ecs_macros::Component as ComponentBuilder;

pub type Component = u64;
pub type Group = u128;

/*
    Trait needed for creating a Component (note: the user doesn't have to manipulate this trait
    everything is included in the macro "derive(Component)"
*/

pub trait AnyComponent {
    fn id() -> Component where Self: Sized;
}

pub fn components_to_group(components: &Vec<Component>) -> Group {
    let mut result = 0 as Group;

    for component in components {
        result += component.clone() as Group;
    }

    return result;
}