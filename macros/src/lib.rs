use proc_macro::TokenStream;
use quote::{format_ident, quote};

use syn::{
    parse_macro_input,
    DeriveInput,
};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let component = &input.ident;
    let pool = format_ident!("{}Pool", component);

    let expanded = quote! {
        impl AnyComponent for #component {
            fn id(&self) -> ComponentID {
                let hasher = RandomState::with_seed(0);

                let id_str = std::any::type_name::<Self>();

                return hasher.hash_one(id_str);
            }

            fn type_id() -> ComponentID {
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