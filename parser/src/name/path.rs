use tokenizer::identifier_or_keyword::Identifier;
use ast_trait::AstNode;

#[ast_macros::ast_node]
#[derive(AstNode)]
struct SimplePath {
    #[optional]
    #[scopestart(start_colon)]
    #[punctuation(DoubleColon)]
    #[scopeend]
    #[repeating(1, 9999999, DoubleColon)]
    segments: SimplePathSegment,
}

#[derive(Debug)]
pub enum SimplePathSegment {
    Identifier(Identifier),
    Super,
    SelfValue,
    Crate,
    DollarCrate,
}

impl AstNode for SimplePathSegment {
    fn parse_node(data: ast_trait::TokenIterator) -> Option<(Self, ast_trait::TokenIterator)> {
        let 
    }
}
