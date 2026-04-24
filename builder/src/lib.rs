use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let (idents, types, name, builder_name) = if let syn::DeriveInput {
        data:
            Data::Struct(syn::DataStruct {
                fields: Fields::Named(syn::FieldsNamed { ref named, .. }),
                ..
            }),
        ident,
        ..
    } = derive_input
    {
        (
            named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect::<Vec<_>>(),
            named.iter().map(|f| f.ty.clone()).collect::<Vec<_>>(),
            ident.clone(),
            format_ident!("{}Builder", ident.clone()),
        )
    } else {
        panic!("Builder can only be derived for structs with named fields");
    };

    let expanded = quote! {
        pub struct #builder_name {
            #(#idents: Option<#types>),*
        }

        impl #builder_name {

            #(pub fn #idents(&mut self, #idents: #types) -> &mut Self {
                self.#idents = Some(#idents);
                self
            })*

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#idents: self.#idents.clone().ok_or(concat!("Field ", stringify!(#idents), " is missing"))?,)*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#idents: None),*
                }
            }
        }
    };

    expanded.into()
}
