use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);

    let data_struct = match derive_input.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("Builder can only be derived for structs"),
    };

    let fields = match data_struct.fields {
        Fields::Named(fields) => fields.named,
        _ => panic!("Builder can only be derived for structs with named fields"),
    };

    let field_idents = fields.iter().filter_map(|field| field.ident.as_ref());
    let field_types = fields.iter().map(|field| &field.ty);

    let builder_struct_fields = field_idents
        .clone()
        .zip(field_types)
        .map(|(ident, ty)| quote! { #ident: Option<#ty> });

    let name = &derive_input.ident;
    let builder_ident = format_ident!("{}Builder", name);
    let expanded = quote! {
        pub struct #builder_ident {
            #(#builder_struct_fields),*
        }

        impl #builder_ident {
            // Builder methods will go here in the next step
        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#field_idents: None),*
                }
            }
        }

    };

    TokenStream::from(expanded)
}
