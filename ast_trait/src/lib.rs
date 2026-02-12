use core::marker::Sized;
use core::option::Option;
use std::fmt::Debug;

use tokenizer::Token;
use tokenizer::punctuation::Punctuation;

pub type TokenIterator<'a> = std::iter::Peekable<core::slice::Iter<'a, tokenizer::Token>>;

pub trait AstNode: Sized {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)>;
    fn parse_repeating(
        data: TokenIterator,
        separator: Option<Punctuation>,
        min: usize,
        max: usize,
    ) -> Option<(Vec<Self>, TokenIterator)> {
        let mut ret_vec = Vec::new();
        let mut data_iterator = data;
        let Some((first, new_iter)) = Self::parse_node(data_iterator.clone()) else {
            if min == 0 {
                return Some((ret_vec, data_iterator));
            } else {
                return None;
            }
        };
        ret_vec.push(first);
        data_iterator = new_iter;

        loop {
            if let Some(sep) = &separator {
                if let Some(Token::Punctuation(other_sep)) = data_iterator.peek()
                    && other_sep == sep
                {
                } else {
                    break;
                }
                data_iterator.next();
            }
            match Self::parse_node(data_iterator.clone()) {
                Some((data, new_iter)) => {
                    ret_vec.push(data);
                    data_iterator = new_iter;
                }
                None => break,
            }
        }

        if ret_vec.len() < min || ret_vec.len() > max {
            return None;
        }
        Some((ret_vec, data_iterator))
    }
}

impl<T: AstNode> AstNode for Box<T> {
    fn parse_node(data: TokenIterator) -> Option<(Self, TokenIterator)> {
        let ret_data = T::parse_node(data);
        ret_data.map(|(data, new_iter)| (Box::new(data), new_iter))
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
