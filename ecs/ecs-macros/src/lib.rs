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

                return hasher.finish();
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
            fn id(&self) -> u64 {
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

                return hasher.finish();
            }

            fn event_id() -> u64 {
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

                return hasher.finish();
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