use core::marker::Sized;
use core::option::Option;
use std::fmt::Debug;

pub type TokenIterator<'a> = std::iter::Peekable<core::slice::Iter<'a, tokenizer::Token>>;

pub trait AstNode: Sized + Debug {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)>;
    fn parse_optional_node(data: TokenIterator) -> (Option<Self>, TokenIterator) {
        let ret_data = Self::parse_node(data.clone());
        match ret_data {
            Some((data, new_iter)) => (Some(data), new_iter),
            None => (None, data),
        }
    }
    fn parse_star_node(data: TokenIterator) -> (Vec<Self>, TokenIterator) {
        let mut ret_vec = Vec::new();
        let mut iter = data;
        while let Some((data, new_iter)) = Self::parse_node(iter.clone()) {
            ret_vec.push(data);
            iter = new_iter;
        }
        (ret_vec, iter)
    }
}

impl<T: AstNode + Debug> AstNode for Box<T> {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)> {
        let ret_data = T::parse_node(data);
        ret_data.map(|(data, new_iter)| (Box::new(data), new_iter))
    }
}

impl AstNode for () {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)> {
        Some(((), data))
    }
}
