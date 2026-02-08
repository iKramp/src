use core::marker::Sized;
use core::option::Option;
use std::fmt::Debug;

pub type ParseIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub trait Token{
    fn parse_token(data: ParseIterator) -> Option<(Self, ParseIterator)>
    where
        Self: Sized + Debug;
}

impl<T: Token + Debug> Token for Box<T> {
    fn parse_token(data: ParseIterator) -> Option<(Self, ParseIterator)> {
        let ret_data = T::parse_token(data);
        ret_data.map(|(data, new_iter)| (Box::new(data), new_iter))
    }
}
