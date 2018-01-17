//! Provides a custom derive `Component` to allow to name components more easily.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// Implements the `Component` custom derive.
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    // Parse the token stream
    let ast: syn::DeriveInput = syn::parse(input).expect("Could not parse the input token stream");

    // Get the name of the type.
    let name = &ast.ident;

    // Generate the impl.
    let gen = quote! {
        use ecs::ComponentTrait;

        impl ComponentTrait for #name {}
    };

    // Return the generated impl
    gen.into()
}
