#![allow(dead_code)]
use ast_macros::{AstNode, ast_node};

#[ast_node]
#[derive(AstNode)]
pub struct StructNode {
    #[keyword("struct")]

    identifier: Identifier,
    
    #[repeating("*")]
    generics: GenericParams,
    
    #[optional]
    where_clause: WhereClause,
    
    #[branch(fields,Fieldless)]
    
    #[punctuation(Punctuation::Semicolon)]
    
    #[branch(fields,WithFields)]
    
    #[punctuation(Punctuation::OpenBrace)]

    #[optional]
    struct_fields: StructFields,
    
    #[punctuation(Punctuation::CloseBrace)]
    
    #[endbranch(fields)]
    
    end: (),
}

struct Identifier;
struct GenericParams;
struct WhereClause;
struct StructFields;
