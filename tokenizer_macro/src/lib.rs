use core::panic;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(ParseEnumToken)]
pub fn derive_enum_new(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();


    let variants = match &input.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("ParseEnumToken can only be derived for enums"),
    };

    let variant_arms = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let syn::Fields::Unnamed(field) = &variant.fields else {
                panic!("ParseEnumToken can only be derived for enums with unnamed fields");
            };
            let field = field
                .unnamed
                .first()
                .expect("ParseEnumToken can only be derived for enums with at least one field");

            let field_type = &field.ty;

            quote::quote! {
                if let Some((data, new_iter)) = <#field_type as tokenizer_trait::Token>::parse_token(data.clone()) {
                    return Some((#name::#ident(data), new_iter));
                }
            }
        })
        .collect::<Vec<_>>();

    let r#gen = quote::quote! {
        impl #impl_generics tokenizer_trait::Token for #name #ty_generics #where_clause {
            fn parse_token(data: tokenizer_trait::ParseIterator) -> Option<(Self, tokenizer_trait::ParseIterator)> {
                #(#variant_arms)*
                None
            }
        }
    };
    r#gen.into()
}

#[proc_macro_derive(ParseStructToken)]
pub fn derive_struct_new(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("ParseStructToken can only be derived for structs"),
    };

    let field_arms = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = field.ty.clone();

        quote::quote! {
            let (#ident, new_iter) = #ty::parse_token(iter)?;
            iter = new_iter;
        }
    });

    let field_packing = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote::quote! {
            #ident
        }
    });

    let r#gen = quote::quote! {
        impl #impl_generics tokenizer_trait::Token for #name #ty_generics #where_clause {
            fn parse_token(data: tokenizer_trait::ParseIterator) -> Option<(Self, TokenizerTrait::ParseIterator)> {
                let mut iter = data;

                #(#field_arms)*

                Some((
                    Self {
                        #(#field_packing),*
                    },
                    iter
                ))
            }
        }
    };
    r#gen.into()
}

