use core::marker::Sized;
use core::option::Option;
use std::fmt::Debug;

pub type SrcIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub trait Token: Sized + Debug {
    fn parse_token(data: SrcIterator) -> Option<(Self, SrcIterator)>;
}

impl<T: Token + Debug> Token for Box<T> {
    fn parse_token(data: SrcIterator) -> Option<(Self, SrcIterator)> {
        let ret_data = T::parse_token(data);
        ret_data.map(|(data, new_iter)| (Box::new(data), new_iter))
    }
}
