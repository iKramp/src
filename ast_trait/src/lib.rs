use core::marker::Sized;
use core::option::Option;
use std::fmt::Debug;

pub type TokenIterator<'a> = std::iter::Peekable<core::slice::Iter<'a, tokenizer::Token>>;

pub trait AstNode: Sized + Debug {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)>;
}

impl<T: AstNode + Debug> AstNode for Box<T> {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)> {
        let ret_data = T::parse_node(data);
        ret_data.map(|(data, new_iter)| (Box::new(data), new_iter))
    }
}

impl<T: AstNode + Debug> AstNode for Vec<T> {
    fn parse_node(mut data_iterator: TokenIterator) -> Option<(Self, TokenIterator)> {
        let mut ret_vec = Vec::new();
        while let Some((data, new_iter)) = T::parse_node(data_iterator.clone()) {
            ret_vec.push(data);
            data_iterator = new_iter;
        }
        Some((ret_vec, data_iterator))
    }
}

impl<T: AstNode + Debug> AstNode for Option<T> {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)> {
        match T::parse_node(data.clone()) {
            Some((data, new_iter)) => Some((Some(data), new_iter)),
            None => Some((None, data)),
        }
    }
}

impl AstNode for () {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)> {
        Some(((), data))
    }
}
