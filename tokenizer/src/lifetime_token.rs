use tokenizer_trait::SrcIterator;

use crate::{IdentifierOrKeyword, identifier_or_keyword::NonKeywordIdentifier};

#[derive(Debug)]
pub enum LifetimeToken {
    Regular(IdentifierOrKeyword),
    Raw(RawLifetime),
}

impl tokenizer_trait::Token for LifetimeToken {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        //try raw
        if let Some(raw_lifetime) = RawLifetime::parse_token(data.clone()) {
            return Some((Self::Raw(raw_lifetime.0), raw_lifetime.1));
        }
        //try regular
        let first_chr = data.next()?;
        if first_chr != '\'' {
            return None;
        }
        let mut regular_lifetime = IdentifierOrKeyword::parse_token(data)?;
        if let Some('\'') = regular_lifetime.1.peek() {
            return None;
        }
        Some((Self::Regular(regular_lifetime.0), regular_lifetime.1))
    }
}

#[derive(Debug)]
pub enum LifetimeOrLabel {
    Regular(NonKeywordIdentifier),
    Raw(RawLifetime),
}

impl tokenizer_trait::Token for LifetimeOrLabel {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        //try raw
        if let Some(raw_lifetime) = RawLifetime::parse_token(data.clone()) {
            return Some((Self::Raw(raw_lifetime.0), raw_lifetime.1));
        }
        //try regular
        let first_chr = data.next()?;
        if first_chr != '\'' {
            return None;
        }
        let mut regular_lifetime = NonKeywordIdentifier::parse_token(data)?;
        if let Some('\'') = regular_lifetime.1.peek() {
            return None;
        }
        Some((Self::Regular(regular_lifetime.0), regular_lifetime.1))
    }
}

#[derive(Debug)]
pub struct RawLifetime {
    inner: IdentifierOrKeyword,
}

impl tokenizer_trait::Token for RawLifetime {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '\'' || data.next()? != 'r' || data.next()? != '#' {
            return None;
        }

        let mut inner = IdentifierOrKeyword::parse_token(data)?;
        if let Some('\'') = inner.1.peek() {
            return None;
        }
        Some((Self { inner: inner.0 }, inner.1))
    }
}

#[derive(Debug)]
pub struct ReservedRawLifetime;

impl tokenizer_trait::Token for ReservedRawLifetime {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '\'' || data.next()? != 'r' || data.next()? != '#' {
            return None;
        }
        const RESERVED_RAW_LIFETIME_TABLE: &[&str] = &["_", "crate", "self", "Self", "super"];
        for reserved_raw_lifetime in RESERVED_RAW_LIFETIME_TABLE {
            let zipped = reserved_raw_lifetime.chars().zip(data.clone());
            if zipped.clone().all(|(a, b)| a == b) && zipped.count() == reserved_raw_lifetime.len() {
                (0..reserved_raw_lifetime.len()).for_each(|_| {
                    data.next();
                });
                return Some((
                    Self,
                    data,
                ));
            }
        }
        None
    }
}
