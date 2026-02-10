#![allow(dead_code)]
use ast_macros::{AstNode, ast_node};

#[ast_node]
#[derive(AstNode)]
pub struct StructNode {
    #[keyword("struct")]

    identifier: Identifier,
    
    // #[optional]
    #[repeating("*")]
    #[scopestart]
    generics: GenericParams,
    
    #[optional]
    #[scopestart]
    where_clause: WhereClause,
    #[scopeend]
    #[scopeend]
    
    #[branch(fields,tst)]
    
    #[punctuation(";")]
    
    #[branch(fields, tst2)]
    
    #[punctuation("{")]
    
    #[optional]
    struct_fields: StructFields,
    
    #[punctuation("}")]
    
    #[endbranch(fields)]
    
    end: (),
}

struct Identifier;
struct GenericParams;
struct WhereClause;
struct StructFields;
