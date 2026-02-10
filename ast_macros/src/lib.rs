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
    Optional,                // Field name and type
    Repeating(usize, usize), // Min, max, field name and type
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
            Self::Field(arg0, arg1) => f
                .debug_tuple("Field")
                .field(arg0)
                .field(&arg1.to_token_stream().to_string())
                .finish(),
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
pub fn derive_ast_node(_stream_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

//(attributes(keyword, punctuation, branch, endbranch, optional))
#[proc_macro_attribute]
pub fn ast_node(_attr: TokenStream, code: TokenStream) -> TokenStream {
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
            && a.elems.is_empty()
        {
            continue; // Skip unit fields
        }
        tokens.push(TokenType::Field(field_name, field_type));
    }

    generate_from_tokens(tokens, input.ident.clone())
}

fn generate_from_tokens(mut tokens: Vec<TokenType>, struct_type: syn::Ident) -> TokenStream {
    eprintln!("Generating from tokens for type {:?}", struct_type);
    let mut generated = generate_sopes(&mut tokens);
    generated.extend(generate_branches(&mut tokens));

    let mut fields = Vec::new();

    let mut token_index = 0;
    while token_index < tokens.len() {
        match &tokens[token_index] {
            TokenType::Field(name, ty) => {
                fields.push(quote::quote! {
                    #name: #ty
                });
            }
            TokenType::Keyword(_) => {}
            TokenType::Punctuation(_) => {}
            TokenType::Optional => {
                token_index += 1;
                let TokenType::Field(name, ty) = &tokens[token_index] else {
                    panic!("optional must be followed by a field");
                };
                fields.push(quote::quote! {
                    #name: Option<#ty>
                });
            }
            TokenType::Repeating(_, _) => {
                token_index += 1;
                let TokenType::Field(name, ty) = &tokens[token_index] else {
                    panic!(
                        "repeating must be followed by a field, start of branch, or start of scope"
                    );
                };
                fields.push(quote::quote! {
                    #name: Vec<#ty>
                });
            }
            _ => panic!("invalid token found. Probably some syntax error"),
        }
        token_index += 1;
    }

    eprintln!(
        "Generated fields for struct {:?}: {:?}",
        struct_type, fields
    );

    let struct_header: TokenStream = quote::quote! {
        struct #struct_type {
            #(#fields),*
        }
    }
    .into();
    generated.extend(struct_header);

    generated
}

fn generate_sopes(tokens: &mut Vec<TokenType>) -> TokenStream {
    let mut result = TokenStream::new();
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

        let (type_token_stream, ty) = generate_scope(&tokens[start..=end]);
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

fn generate_branches(tokens: &mut Vec<TokenType>) -> TokenStream {
    let mut result = TokenStream::new();
    loop {
        let mut branches = Vec::new();

        let Some(mut end) = tokens
            .iter()
            .position(|a| matches!(a, TokenType::EndBranch(_)))
        else {
            break;
        };
        let og_end = end;
        let mut real_start = 0;
        let TokenType::EndBranch(branch_name) = &tokens[end] else {
            panic!("Expected EndBranch token");
        };

        while let Some(start) = tokens[..end]
            .iter()
            .rposition(|a| matches!(a, TokenType::Branch(name, _) if name == branch_name))
        {
            let TokenType::Branch(_, case_name) = &tokens[start] else {
                panic!("Expected Branch token");
            };
            branches.push((&tokens[(start + 1)..end], case_name.clone()));
            end = start;
            real_start = start;
        }
        if branches.is_empty() {
            panic!("No matching Branch token found for EndBranch");
        }
        let (branch_token_stream, ty) = generate_branch(branches, branch_name.clone());
        result.extend(branch_token_stream);

        let combined_name = syn::Ident::new(branch_name, Span::call_site().into());

        tokens.splice(
            real_start..=og_end,
            [TokenType::Field(combined_name.clone(), ty.clone())],
        );
    }
    result
}

fn generate_scope(tokens: &[TokenType]) -> (TokenStream, syn::Type) {
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
    let mut combined_name = String::new();
    for field in &fields {
        combined_name.push_str(&field.1.to_token_stream().to_string());
    }
    combined_name.push_str("AutoGen");

    let new_struct_type = syn::Ident::new(&combined_name, Span::call_site().into());

    let stream = generate_from_tokens(tokens, new_struct_type.clone());

    (stream, syn::parse_quote!(#new_struct_type))
}

fn generate_branch(
    branches: Vec<(&[TokenType], String)>,
    enum_name: String,
) -> (TokenStream, syn::Type) {
    let mut cases = Vec::new();

    for branch in branches {
        let tokens = branch.0;
        let case_name = &branch.1;
        let case_name = syn::Ident::new(case_name, Span::call_site().into());
        let mut field_found = false;

        let mut token_counter = 0;
        while token_counter < tokens.len() {
            let maybe_field = &tokens[token_counter];
            if let TokenType::Field(_, field_type) = maybe_field {
                if field_found {
                    panic!("more han one field in a branch. Use scopes to group together");
                }
                cases.push(quote::quote! {
                    #case_name(#field_type),
                });
                field_found = true;
            } else if let TokenType::Optional = maybe_field {
                let TokenType::Field(_, field_type) = &tokens[token_counter + 1] else {
                    panic!("optional must be followed by a field");
                };
                if field_found {
                    panic!("more han one field in a branch. Use scopes to group together");
                }
                cases.push(quote::quote! {
                    #case_name(Option<#field_type>),
                });
                field_found = true;
                token_counter += 1; // Skip the field token
            } else if let TokenType::Repeating(_, _) = maybe_field {
                let TokenType::Field(_, field_type) = &tokens[token_counter + 1] else {
                    panic!("repeating must be followed by a field");
                };
                if field_found {
                    panic!("more han one field in a branch. Use scopes to group together");
                }
                cases.push(quote::quote! {
                    #case_name(Vec<#field_type>),
                });
                field_found = true;
                token_counter += 1; // Skip the field token
            } else if let TokenType::Branch(_, _) = maybe_field {
                panic!("nested branches might have to be wrapped in a scope")
            }
            token_counter += 1;
        }
        if !field_found {
            cases.push(quote::quote! {
                #case_name,
            });
        }
    }

    let enum_name = format!("{}AutoGen", enum_name);
    let enum_ident = syn::Ident::new(&enum_name, Span::call_site().into());
    let stream: TokenStream = quote::quote! {
        enum #enum_ident {
            #(#cases)*
        }
    }
    .into();
    (stream, syn::parse_quote!(#enum_ident))
}
