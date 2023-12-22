use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = quote! {
        impl hnz::ecs::core::component::AnyComponent for #name {
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
    };

    TokenStream::from(expanded)
}