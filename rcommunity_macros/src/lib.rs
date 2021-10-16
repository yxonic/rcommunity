extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(UserType)]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive(input.into()).into()
}

fn derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse2(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl UserType for #name {}
    };
    gen.into()
}
