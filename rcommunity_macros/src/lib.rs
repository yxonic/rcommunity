extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Unique)]
pub fn unique_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_unique(input.into()).into()
}

fn derive_unique(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse2(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl ::rcommunity_core::Unique for #name {}
    };
    gen.into()
}

#[proc_macro_derive(UserType)]
pub fn user_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_user(input.into()).into()
}

fn derive_user(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse2(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl ::rcommunity_core::UserType for #name {}
    };
    gen.into()
}

#[proc_macro_derive(ReactionType)]
pub fn reaction_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_reaction(input.into()).into()
}

fn derive_reaction(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse2(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl ::rcommunity_core::ReactionType for #name {}
    };
    gen.into()
}
