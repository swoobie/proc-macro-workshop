use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _derive_input = parse_macro_input!(input as syn::DeriveInput);
    let expanded = quote! {
        // The generated impl will go here.
    };
    TokenStream::from(expanded)
}
