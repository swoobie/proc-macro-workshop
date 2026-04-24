use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let fields = match derive_input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Builder can only be derived for structs with named fields"),
        },
        _ => panic!("Builder can only be derived for structs"),
    };

    let name = &derive_input.ident;
    let builder_ident = format_ident!("{}Builder", name);

    let builder_setters = fields.iter().map(map_builder_setters);
    let builder_fields = fields.iter().map(map_builder_fields);
    let field_idents = fields.iter().map(|f| f.ident.as_ref().unwrap());

    let expanded = quote! {
        pub struct #builder_ident {
            #(#builder_fields),*
        }

        impl #builder_ident {
            #(#builder_setters)*
        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#field_idents: None),*
                }
            }
        }
    };

    eprintln!("Expanded code:\n{}", expanded);

    TokenStream::from(expanded)
}

// -- Helper functions --

fn map_builder_setters(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! {
        pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
            self.#ident = Some(#ident);
            self
        }
    }
}

fn map_builder_fields(field: &syn::Field) -> proc_macro2::TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! {
        #ident: Option<#ty>
    }
}
