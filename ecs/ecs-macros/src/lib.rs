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
        }
    };

    TokenStream::from(expanded)
}