use std::fmt::Debug;

use proc_macro::{Span, TokenStream};
use quote::ToTokens;
use syn::Ident;

#[derive(Clone)]
enum TokenType {
    Keyword(String),
    Punctuation(String),
    Branch(String, String), // Branch name and case name
    EndBranch(String),
    Field(Ident, syn::Type), // Field name and type
    Optional,                 // Field name and type
    Repeating(usize, usize),  // Min, max, field name and type
    ScopeStart,
    ScopeEnd,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(arg0) => f.debug_tuple("Keyword").field(arg0).finish(),
            Self::Punctuation(arg0) => f.debug_tuple("Punctuation").field(arg0).finish(),
            Self::Branch(arg0, arg1) => f.debug_tuple("Branch").field(arg0).field(arg1).finish(),
            Self::EndBranch(arg0) => f.debug_tuple("EndBranch").field(arg0).finish(),
            Self::Field(arg0, arg1) => f.debug_tuple("Field").field(arg0).field(&arg1.to_token_stream().to_string()).finish(),
            Self::Optional => f.debug_tuple("Optional").finish(),
            Self::Repeating(arg0, arg1) => {
                f.debug_tuple("Repeating").field(arg0).field(arg1).finish()
            }
            Self::ScopeStart => f.debug_tuple("ScopeStart").finish(),
            Self::ScopeEnd => f.debug_tuple("ScopeEnd").finish(),
        }
    }
}

#[proc_macro_derive(
    AstNode,
    attributes(
        keyword,
        punctuation,
        branch,
        endbranch,
        optional,
        repeating,
        scopestart,
        scopeend
    )
)]
pub fn derive_ast_node(stream_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

//(attributes(keyword, punctuation, branch, endbranch, optional))
#[proc_macro_attribute]
pub fn ast_node(attr: TokenStream, code: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(code as syn::DeriveInput);
    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("AstNode can only be derived for structs"),
    };

    let mut tokens = Vec::new();

    for field in fields {
        for attr in &field.attrs {
            if attr.path().is_ident("keyword") {
                let meta = &attr.meta;
                if let syn::Meta::List(list) = meta {
                    let a = list.tokens.to_string();
                    tokens.push(TokenType::Keyword(a));
                } else {
                    panic!("keyword attribute must be a list");
                }
            }
            if attr.path().is_ident("punctuation") {
                let meta = &attr.meta;
                if let syn::Meta::List(list) = meta {
                    let a = list.tokens.to_string();
                    tokens.push(TokenType::Punctuation(a));
                } else {
                    panic!("keyword attribute must be a list");
                }
            }
            if attr.path().is_ident("branch") {
                let meta = &attr.meta;
                if let syn::Meta::List(list) = meta {
                    let a = list.tokens.to_string();
                    let (mut branch_name, mut case_name) = a
                        .split_once(',')
                        .expect("branch attribute must be in the format branch(name, other)");
                    branch_name = branch_name.trim();
                    case_name = case_name.trim();
                    tokens.push(TokenType::Branch(
                        branch_name.to_string(),
                        case_name.to_string(),
                    ));
                } else {
                    panic!("branch attribute must be a list");
                }
            }
            if attr.path().is_ident("endbranch") {
                let meta = &attr.meta;
                if let syn::Meta::List(list) = meta {
                    let a = list.tokens.to_string();
                    tokens.push(TokenType::EndBranch(a));
                } else {
                    panic!("endbranch attribute must be a list");
                }
            }
            if attr.path().is_ident("optional") {
                tokens.push(TokenType::Optional);
            }
            if attr.path().is_ident("repeating") {
                tokens.push(TokenType::Repeating(0, usize::MAX)); // For simplicity, we can assume
                // repeating means 0 to infinity
            }
            if attr.path().is_ident("scopestart") {
                tokens.push(TokenType::ScopeStart);
            }
            if attr.path().is_ident("scopeend") {
                tokens.push(TokenType::ScopeEnd);
            }
        }
        let field_name = field.ident.clone().expect("Fields must be named");
        let field_type = field.ty.clone();
        if let syn::Type::Tuple(a) = &field_type
            && a.elems.is_empty() {
                continue; // Skip unit fields
            }
        tokens.push(TokenType::Field(field_name, field_type));
    }

    generate_from_tokens(tokens, input.ident.clone())
}

fn generate_from_tokens(mut tokens: Vec<TokenType>, struct_type: syn::Ident) -> TokenStream {
    eprintln!("Generating from tokens for type {:?}", struct_type);
    let mut generated = generate_sopes(&mut tokens, &struct_type);

    let mut fields = Vec::new();

    for token in tokens {
        if let TokenType::Field(name, ty) = token {
            fields.push(quote::quote! {
                #name: #ty
            });
        }
    }

    eprintln!("Generated fields for struct {:?}: {:?}", struct_type, fields);

    let struct_header: TokenStream = quote::quote! {
        struct #struct_type {
            #(#fields),*
        }
    }.into();
    generated.extend(struct_header);

    generated
}

fn generate_sopes(tokens: &mut Vec<TokenType>, type_name: &syn::Ident) -> TokenStream {
    let mut result = TokenStream::new();
    #[allow(clippy::never_loop)]
    loop {
        let Some(end) = tokens.iter().position(|a| matches!(a, TokenType::ScopeEnd)) else {
            break;
        };
        let Some(start) = tokens[..end]
            .iter()
            .rposition(|a| matches!(a, TokenType::ScopeStart))
        else {
            panic!("Unmatched scope found");
        };
        let (type_token_stream, ty) = generate_scope(&tokens[start..=end], type_name);
        result.extend(type_token_stream);
        let fields = tokens[(start + 1)..end]
            .iter()
            .filter_map(|a| {
                if let TokenType::Field(name, ty) = a {
                    Some((name.clone(), ty.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut combined_name = String::new();
        for field in &fields {
            combined_name.push_str(&field.0.to_string());
            combined_name.push('_');
        }
        combined_name.pop(); // Remove the last underscore
        let combined_name = syn::Ident::new(&combined_name, Span::call_site().into());

        tokens.splice(
            start..=end,
            [TokenType::Field(combined_name.clone(), ty.clone())],
        );
    }

    if let Some(_pos) = tokens
        .iter()
        .position(|a| matches!(a, TokenType::ScopeStart | TokenType::ScopeEnd))
    {
        panic!("Unmatched scope found");
    }

    result
}

fn generate_scope(tokens: &[TokenType], parent_type_name: &syn::Ident) -> (TokenStream, syn::Type) {
    eprintln!("Generating scope for tokens: {:?}", tokens);
    let tokens = tokens[1..tokens.len() - 1].to_vec(); // Remove the scope start and end tokens

    let fields = tokens
        .iter()
        .filter_map(|a| {
            if let TokenType::Field(name, ty) = a {
                Some((name.clone(), ty.clone()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut combined_name = parent_type_name.to_string();
    for field in &fields {
        combined_name.push_str(&field.1.to_token_stream().to_string());
    }

    let new_struct_type = syn::Ident::new(&combined_name, Span::call_site().into());

    let stream = generate_from_tokens(tokens, new_struct_type.clone());

    (stream, syn::parse_quote!(#new_struct_type))
}
