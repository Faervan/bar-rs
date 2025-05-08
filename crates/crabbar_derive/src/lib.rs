use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let default = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Unit => Some(quote! {#ident}),
            _ => None,
        },
        _ => None,
    }
    .unwrap_or_else(|| quote! {#ident::default()});
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics crate::registry::Builder for #ident #ty_generics #where_clause {
            type Output = Self;
            fn build() -> Self::Output {
                #default
            }
        }
    }
    .into()
}
