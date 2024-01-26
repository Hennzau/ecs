use proc_macro::TokenStream;
use quote::quote;

use syn::{
    parse_macro_input,
    DeriveInput,
};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl AnyComponent for #name {
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
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl AnyEvent for #name {
            fn id(&self) -> EventID {
                let hasher = RandomState::with_seed(0);

                let id_str = std::any::type_name::<Self>();

                return hasher.hash_one(id_str);
            }

            fn event_id() -> EventID {
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
        }
    };

    TokenStream::from(expanded)
}