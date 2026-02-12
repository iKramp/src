use ast_trait::AstNode;


#[ast_macros::ast_node]
#[derive(ast_macro::AstNode)]
pub struct Test {
    #[repeating(2, 10, DoubleColon)]
    attr: Attr,
}

// #[ast_macros::ast_node]
// #[derive(ast_macro::AstNode)]
// struct InnerAttribute {
//     #[punctuation(Hash)]
//     #[punctuation(Bang)]
//     #[punctuation(LeftSquare)]
//     attr: Attr,
//     #[punctuation(RightSquare)]
//     end: (),
// }

// #[ast_macros::ast_node]
// #[derive(ast_macro::AstNode)]
// struct OuterAttribute {
//     #[punctuation(Hash)]
//     #[punctuation(LeftSquare)]
//     inner: Attr,
//     #[punctuation(RightSquare)]
//     end: (),
// }

struct Attr;

impl AstNode for Attr {
    fn parse_node(data: ast_trait::TokenIterator) -> Option<(Self, ast_trait::TokenIterator)> {
        todo!()
    }
}
