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
    ScopeStart(String),
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
            Self::ScopeStart(arg0) => f.debug_tuple("ScopeStart").field(arg0).finish(),
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
                    if !a.starts_with('\"') || !a.ends_with('\"') {
                        panic!("keyword attribute must be a string literal");
                    }
                    //trim
                    let a = a[1..a.len() - 1].to_string();
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
                let meta = &attr.meta;
                if let syn::Meta::List(list) = meta {
                    let a = list.tokens.to_string();
                    tokens.push(TokenType::ScopeStart(a));
                } else {
                    panic!("scopestart attribute must be a list");
                }
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
    let mut generated = generate_sopes(&mut tokens);
    generated.extend(generate_branches(&mut tokens));

    let mut fields = Vec::new();
    let mut names = Vec::new();

    let mut matchers = Vec::new();

    let mut token_index = 0;
    while token_index < tokens.len() {
        match &tokens[token_index] {
            TokenType::Field(name, ty) => {
                fields.push(quote::quote! {
                    #name: #ty
                });
                names.push(name.clone());
                matchers.push(quote::quote! {
                    let (#name, mut data) = <#ty as AstNode>::parse_node(data)?;
                });
            }
            TokenType::Keyword(keyword) => {
                matchers.push(quote::quote! {
                    let tokenizer::Token::IdentifierOrKeyword(ident_or_keyword) = data.next()? else {
                        return None;
                    };
                    if ident_or_keyword.parsed() != #keyword {
                        return None;
                    }
                });
            }
            TokenType::Punctuation(punct) => {
                let punct_ident = syn::Ident::new(punct, Span::call_site().into());
                matchers.push(quote::quote! {
                    let tokenizer::Token::Punctuation(tokenizer::punctuation::Punctuation::#punct_ident) = data.next()? else {
                        return None;
                    };
                });
            }
            TokenType::Optional => {
                token_index += 1;
                let TokenType::Field(name, ty) = &tokens[token_index] else {
                    panic!("optional must be followed by a field");
                };
                fields.push(quote::quote! {
                    #name: Option<#ty>
                });
                names.push(name.clone());
                matchers.push(quote::quote! {
                    let (#name, mut data) = <Option<#ty> as AstNode>::parse_node(data)?;
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
                names.push(name.clone());
                matchers.push(quote::quote! {
                    let (#name, mut data) = <Vec<#ty> as AstNode>::parse_node(data)?;
                });
            }
            _ => panic!("invalid token found. Probably some syntax error"),
        }
        token_index += 1;
    }

    let struct_header: TokenStream = quote::quote! {
        struct #struct_type {
            #(#fields),*
        }

        impl ast_trait::AstNode for #struct_type {
            fn parse_node(mut data: ast_trait::TokenIterator) -> Option<(Self, ast_trait::TokenIterator)> {
                #(#matchers)*
                Some((Self {
                    #(#names),*
                }, data))
            }
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
            .rposition(|a| matches!(a, TokenType::ScopeStart(_)))
        else {
            panic!("Unmatched scope found");
        };
        let TokenType::ScopeStart(scope_name) = &tokens[start] else {
            panic!("Expected ScopeStart token");
        };

        let (type_token_stream, ty) = generate_scope(&tokens[start..=end], scope_name.clone());
        result.extend(type_token_stream);

        let combined_name = syn::Ident::new(scope_name, Span::call_site().into());

        tokens.splice(
            start..=end,
            [TokenType::Field(combined_name.clone(), ty.clone())],
        );
    }

    if let Some(_pos) = tokens
        .iter()
        .position(|a| matches!(a, TokenType::ScopeStart(_) | TokenType::ScopeEnd))
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

fn generate_scope(tokens: &[TokenType], field_name: String) -> (TokenStream, syn::Type) {
    eprintln!("Generating scope for tokens: {:?}", tokens);
    let tokens = tokens[1..tokens.len() - 1].to_vec(); // Remove the scope start and end tokens

    let combined_name = field_name.split("_").map(|s| {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }).collect::<String>() + "AutoGen";


    let new_struct_type = syn::Ident::new(&combined_name, Span::call_site().into());

    let stream = generate_from_tokens(tokens, new_struct_type.clone());

    (stream, syn::parse_quote!(#new_struct_type))
}

fn generate_branch(
    branches: Vec<(&[TokenType], String)>,
    enum_name: String,
) -> (TokenStream, syn::Type) {
    let mut cases = Vec::new();
    let mut types_and_names = Vec::new();

    for branch in branches {
        let tokens = branch.0;
        let case_name = &branch.1;
        let case_name = syn::Ident::new(case_name, Span::call_site().into());
        let mut field_found = false;

        let mut token_counter = 0;
        while token_counter < tokens.len() {
            let maybe_field = &tokens[token_counter];
            match maybe_field {
                TokenType::Field(_, field_type) => {
                    if field_found {
                        panic!("more han one field in a branch. Use scopes to group together");
                    }
                    cases.push(quote::quote! {
                        #case_name(#field_type),
                    });
                    types_and_names.push((field_type.clone(), case_name.clone()));
                    field_found = true;
                },
                TokenType::Optional => {
                    token_counter += 1;
                    let TokenType::Field(_, field_type) = &tokens[token_counter] else {
                        panic!("optional must be followed by a field");
                    };
                    if field_found {
                        panic!("more han one field in a branch. Use scopes to group together");
                    }
                    cases.push(quote::quote! {
                        #case_name(Option<#field_type>),
                    });
                    types_and_names.push((syn::parse_quote!(Option<#field_type>), case_name.clone()));
                    field_found = true;
                }
                TokenType::Repeating(_, _) => {
                    token_counter += 1;
                    let TokenType::Field(_, field_type) = &tokens[token_counter] else {
                        panic!("repeating must be followed by a field");
                    };
                    if field_found {
                        panic!("more han one field in a branch. Use scopes to group together");
                    }
                    cases.push(quote::quote! {
                        #case_name(Vec<#field_type>),
                    });
                    types_and_names.push((syn::parse_quote!(Vec<#field_type>), case_name.clone()));
                    field_found = true;
                },
                _ => {
                    panic!("only fields, optional, and repeating can be used in branches. Limit with scopes. Found: {:?}", maybe_field);
                }
            }
            token_counter += 1;
        }
        if !field_found {
            panic!("no field found in branch. Branches must have exactly one field. Use scopes to group together multiple fields");
        }
    }

    let enum_name = format!("{}AutoGen", enum_name);
    let enum_ident = syn::Ident::new(&enum_name, Span::call_site().into());
    let mut enum_def_stream: TokenStream = quote::quote! {
        enum #enum_ident {
            #(#cases)*
        }
    }
    .into();

    let mut matchers = Vec::new();

    for (case_type, case_name) in types_and_names {
        let case_match_stream = quote::quote! {
            'block: {
                let cloned_data = data.clone();
                if let Some((parsed_data, remaining)) = <#case_type as AstNode>::parse_node(cloned_data) {
                    return Some((Self::#case_name(parsed_data), remaining));
                } else {
                    break 'block;
                }
            }
        };
        matchers.push(case_match_stream);
    }

    let enum_match_stream: TokenStream = quote::quote! {
        impl ast_trait::AstNode for #enum_ident {
            fn parse_node(data: ast_trait::TokenIterator) -> Option<(Self, ast_trait::TokenIterator)> {
                #(#matchers)*
                None
            }
        }
    }.into();
    
    enum_def_stream.extend(enum_match_stream);

    (enum_def_stream, syn::parse_quote!(#enum_ident))
}
