use crate::{
    IdentifierOrKeyword, StringLiteral,
    identifier_or_keyword::ReservedRawIdentifier,
    integer_literal::{BinLiteral, DecLiteral, HexLiteral, OctLiteral},
    lifetime_token::ReservedRawLifetime,
};

#[derive(Debug, tokenizer_macro::ParseEnumToken)]
pub enum ReservedToken {
    ReservedGuardedStringLiteral(ReservedGuardedStringLiteral),
    ReservedNumber(ReservedNumber),
    ReservedPounds(ReservedPounds),
    ReservedRawIdentifier(ReservedRawIdentifier),
    ReservedRawLifetime(ReservedRawLifetime),
    ReservedTokenDoubleQuote(ReservedTokenDoubleQuote),
    ReservedTokenLifetime(ReservedTokenLifetime),
    ReservedTokenPound(ReservedTokenPound),
    ReservedTokenSingleQuote(ReservedTokenSingleQuote),
}

#[derive(Debug)]
pub struct ReservedGuardedStringLiteral;

impl tokenizer_trait::Token for ReservedGuardedStringLiteral {
    fn parse_token(
        mut data: tokenizer_trait::SrcIterator,
    ) -> Option<(Self, tokenizer_trait::SrcIterator)> {
        if data.next()? != '#' {
            return None;
        }
        while let Some('#') = data.peek() {
            data.next();
        }
        let string_literal = StringLiteral::parse_token(data)?;
        Some((Self, string_literal.1))
    }
}

#[derive(Debug)]
pub struct ReservedNumber;

impl tokenizer_trait::Token for ReservedNumber {
    fn parse_token(
        data: tokenizer_trait::SrcIterator,
    ) -> Option<(Self, tokenizer_trait::SrcIterator)> {
        'block: {
            let temp_data = data.clone();
            let Some((_, mut temp_data)) = BinLiteral::parse_token(temp_data) else {
                break 'block;
            };
            let Some(next) = temp_data.next() else {
                break 'block;
            };
            if ('2'..='9').contains(&next) {
                return Some((Self, temp_data));
            }
        }
        'block: {
            let temp_data = data.clone();
            let Some((_, mut temp_data)) = OctLiteral::parse_token(temp_data) else {
                break 'block;
            };
            let Some(next) = temp_data.next() else {
                break 'block;
            };
            if ('8'..='9').contains(&next) {
                return Some((Self, temp_data));
            }
        }
        'block: {
            let temp_data = data.clone();
            let Some((_, mut temp_data)) = BinLiteral::parse_token(temp_data) else {
                break 'block;
            };
            let Some('.') = temp_data.next() else {
                break 'block;
            };
            let Some(after) = temp_data.peek() else {
                break 'block;
            };
            if *after == '.' || *after == '_' || unicode_ident::is_xid_start(*after) {
                break 'block;
            }
            return Some((Self, temp_data));
        }
        'block: {
            let temp_data = data.clone();
            let Some((_, mut temp_data)) = OctLiteral::parse_token(temp_data) else {
                break 'block;
            };
            let Some('.') = temp_data.next() else {
                break 'block;
            };
            let Some(after) = temp_data.peek() else {
                break 'block;
            };
            if *after == '.' || *after == '_' || unicode_ident::is_xid_start(*after) {
                break 'block;
            }
            return Some((Self, temp_data));
        }
        'block: {
            let temp_data = data.clone();
            let Some((_, mut temp_data)) = HexLiteral::parse_token(temp_data) else {
                break 'block;
            };
            let Some('.') = temp_data.next() else {
                break 'block;
            };
            let Some(after) = temp_data.peek() else {
                break 'block;
            };
            if *after == '.' || *after == '_' || unicode_ident::is_xid_start(*after) {
                break 'block;
            }
            return Some((Self, temp_data));
        }
        'block: {
            let temp_data = data.clone();
            let Some((_, mut temp_data)) = BinLiteral::parse_token(temp_data) else {
                break 'block;
            };
            let Some(next) = temp_data.next() else {
                break 'block;
            };
            if next == 'e' || next == 'E' {
                return Some((Self, temp_data));
            }
        }
        'block: {
            let temp_data = data.clone();
            let Some((_, mut temp_data)) = OctLiteral::parse_token(temp_data) else {
                break 'block;
            };
            let Some(next) = temp_data.next() else {
                break 'block;
            };
            if next == 'e' || next == 'E' {
                return Some((Self, temp_data));
            }
        }
        'block: {
            let mut temp_data = data.clone();
            let Some(next) = temp_data.next() else {
                break 'block;
            };
            if next != '0' {
                break 'block;
            }
            let Some(num_format) = temp_data.next() else {
                break 'block;
            };
            if num_format != 'b' && num_format != 'o' && num_format != 'x' {
                break 'block;
            }
            while let Some('_') = temp_data.peek() {
                temp_data.next();
            }
            let Some(next) = temp_data.next() else {
                return Some((Self, temp_data));
            };
            if num_format == 'b' && !('0'..='1').contains(&next) {
                return Some((Self, temp_data));
            }
            if num_format == 'o' && !('0'..='7').contains(&next) {
                return Some((Self, temp_data));
            }
            if num_format == 'x' && !next.is_ascii_hexdigit() {
                return Some((Self, temp_data));
            }
        }
        'block: {
            let temp_data = data.clone();
            let Some((_, mut temp_data)) = DecLiteral::parse_token(temp_data) else {
                break 'block;
            };
            if let Some('.') = temp_data.peek() {
                temp_data.next();
                let Some((_, _temp_data)) = DecLiteral::parse_token(temp_data) else {
                    break 'block;
                };
                temp_data = _temp_data;
            }
            let Some(next) = temp_data.next() else {
                break 'block;
            };
            if next != 'e' && next != 'E' {
                break 'block;
            }
            if let Some(next) = temp_data.peek()
                && (*next == '+' || *next == '-')
            {
                temp_data.next();
            }
            let Some(next) = temp_data.next() else {
                return Some((Self, temp_data));
            };
            if !next.is_ascii_digit() {
                return Some((Self, temp_data));
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct ReservedPounds;

impl tokenizer_trait::Token for ReservedPounds {
    fn parse_token(
        mut data: tokenizer_trait::SrcIterator,
    ) -> Option<(Self, tokenizer_trait::SrcIterator)> {
        if data.next()? != '#' {
            return None;
        }
        let next = data.peek()?;
        if *next == '#' {
            while let Some(&'#') = data.peek() {
                data.next();
            }
            return Some((Self, data));
        }
        None
    }
}

#[derive(Debug)]
pub struct ReservedTokenDoubleQuote;

impl tokenizer_trait::Token for ReservedTokenDoubleQuote {
    fn parse_token(
        data: tokenizer_trait::SrcIterator,
    ) -> Option<(Self, tokenizer_trait::SrcIterator)> {
        let mut inner = IdentifierOrKeyword::parse_token(data.clone())?;
        let parsed = inner.0.parsed();
        if ["b", "c", "r", "br", "cr"].contains(&parsed) {
            return None;
        }
        if let Some('"') = inner.1.peek() {
            inner.1.next();
            return Some((Self, inner.1));
        }
        None
    }
}

#[derive(Debug)]
pub struct ReservedTokenLifetime;

impl tokenizer_trait::Token for ReservedTokenLifetime {
    fn parse_token(
        mut data: tokenizer_trait::SrcIterator,
    ) -> Option<(Self, tokenizer_trait::SrcIterator)> {
        if data.next()? != '\'' {
            return None;
        }
        let mut inner = IdentifierOrKeyword::parse_token(data)?;
        if inner.0.parsed() == "r" {
            return None;
        }
        if let Some('#') = inner.1.peek() {
            inner.1.next();
            return Some((Self, inner.1));
        }
        None
    }
}

#[derive(Debug)]
pub struct ReservedTokenPound;

impl tokenizer_trait::Token for ReservedTokenPound {
    fn parse_token(
        data: tokenizer_trait::SrcIterator,
    ) -> Option<(Self, tokenizer_trait::SrcIterator)> {
        let mut inner = IdentifierOrKeyword::parse_token(data.clone())?;
        let parsed = inner.0.parsed();
        if ["r", "br", "cr"].contains(&parsed) {
            return None;
        }
        if let Some('#') = inner.1.peek() {
            inner.1.next();
            return Some((Self, inner.1));
        }
        None
    }
}

#[derive(Debug)]
pub struct ReservedTokenSingleQuote;

impl tokenizer_trait::Token for ReservedTokenSingleQuote {
    fn parse_token(
        data: tokenizer_trait::SrcIterator,
    ) -> Option<(Self, tokenizer_trait::SrcIterator)> {
        let mut inner = IdentifierOrKeyword::parse_token(data)?;
        if inner.0.parsed() == "b" {
            return None;
        }
        if let Some('\'') = inner.1.peek() {
            inner.1.next();
            return Some((Self, inner.1));
        }
        None
    }
}
