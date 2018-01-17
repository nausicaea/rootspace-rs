//! Provides a custom derive `Component` to allow to name components more easily.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate ecs;

use proc_macro::TokenStream;
use ecs::ComponentTrait;

/// Implements the `Component` custom derive.
#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    // Parse the token stream
    let ast = syn::parse(input).expect("Could not parse the input token stream");

    // Build the impl
    let gen = impl_component(&ast);

    // Return the generated impl
    gen.into()
}

/// Implements the `ComponentTrait` for the input type.
fn impl_component(ast: &syn::DeriveInput) -> quote::Tokens {
    // Get the name of the type.
    let name = &ast.ident;

    // Generate the impl.
    quote! {
        impl ComponentTrait for #name {}
    }
}
