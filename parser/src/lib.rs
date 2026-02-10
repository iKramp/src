#![allow(dead_code)]
use ast_macros::{AstNode, ast_node};
use ast_trait::AstNode;

#[ast_node]
#[derive(AstNode)]
pub struct StructNode {
    #[keyword("struct")]

    identifier: Identifier,
    
    #[optional]
    generics: GenericParams,
    
    #[optional]
    where_clause: WhereClause,
    
    #[branch(fields,Fieldless)]

    #[scopestart(fieldless)]
    #[punctuation(Semicolon)]
    #[scopeend]
    
    #[branch(fields,WithFields)]
    
    #[scopestart(fields)]
    #[punctuation(LeftCurly)]

    #[optional]
    struct_fields: StructFields,
    
    #[punctuation(RightCurly)]
    #[scopeend]
    
    #[endbranch(fields)]
    
    end: (),
}

#[derive(Debug)]
struct Identifier;
impl AstNode for Identifier {
    fn parse_node(data: ast_trait::TokenIterator) -> Option<(Self, ast_trait::TokenIterator)> {
        todo!()
    }
}
#[derive(Debug)]
struct GenericParams;
impl AstNode for GenericParams {
    fn parse_node(data: ast_trait::TokenIterator) -> Option<(Self, ast_trait::TokenIterator)> {
        todo!()
    }
}
#[derive(Debug)]
struct WhereClause;
impl AstNode for WhereClause {
    fn parse_node(data: ast_trait::TokenIterator) -> Option<(Self, ast_trait::TokenIterator)> {
        todo!()
    }
}
#[derive(Debug)]
struct StructFields;
impl AstNode for StructFields {
    fn parse_node(data: ast_trait::TokenIterator) -> Option<(Self, ast_trait::TokenIterator)> {
        todo!()
    }
}
