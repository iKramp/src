#![allow(dead_code)]

pub mod byte_literal;
pub mod byte_string_literal;
pub mod c_string_literal;
pub mod char_literal;
pub mod float_literal;
pub mod identifier_or_keyword;
pub mod integer_literal;
pub mod lifetime_token;
pub mod punctuation;
pub mod raw_byte_string_literal;
pub mod raw_c_string_literal;
pub mod raw_identifier;
pub mod raw_string_literal;
pub mod reserved_token;
pub mod string_escapes;
pub mod string_literal;
pub mod suffix;

use std::path::Path;

use tokenizer_macro::ParseEnumToken;
use tokenizer_trait::{SrcIterator, Token as TokenTrait};

use crate::{
    byte_literal::ByteLiteral,
    byte_string_literal::ByteStringLiteral,
    c_string_literal::CStringLiteral,
    char_literal::CharLiteral,
    float_literal::FloatLiteral,
    identifier_or_keyword::{IdentifierOrKeyword, RawIdentifier},
    integer_literal::IntegerLiteral,
    lifetime_token::LifetimeToken,
    punctuation::Punctuation,
    raw_byte_string_literal::RawByteStringLiteral,
    raw_c_string_literal::RawCStringLiteral,
    raw_string_literal::RawStringLiteral,
    reserved_token::ReservedToken,
    string_literal::StringLiteral,
};

pub fn tokenize_file(filename: &Path) -> Box<[Token]> {
    let data = std::fs::read_to_string(filename).expect("Failed to read file");
    let mut data = &data[..];
    if data.starts_with('\u{FEFF}') {
        data = &data[3..];
    }
    if data.starts_with("#!") {
        let mut internal_data = data[3..].lines().next().unwrap_or("");
        internal_data = internal_data.trim();
        if !internal_data.starts_with('[') {
            let first_newline = data.find('\n').unwrap_or(data.len());
            data = &data[first_newline..];
        }
    }

    tokenize(data.chars().peekable(), filename)
}

pub fn tokenize(mut data: SrcIterator, filename: &Path) -> Box<[Token]> {
    let mut tokens = Vec::new();
    loop {
        while let Some(chr) = data.peek() {
            if chr.is_whitespace() {
                data.next();
            } else {
                break;
            }
        }
        let res = Token::parse_token(data.clone());
        match res {
            Some((token, remaining)) => {
                data = remaining;
                // println!("Parsed token: {:?}", token);
                if let Token::ReservedToken(a) = token {
                    panic!("Parsed reserved token: {:?}", a);
                }
                tokens.push(token);
            }
            None => {
                if data.peek().is_none() {
                    return tokens.into_boxed_slice();
                } else {
                    println!("Failed to parse token");
                    for _ in 0..1000 {
                        if let Some(chr) = data.next() {
                            print!("{}", chr);
                        } else {
                            break;
                        }
                    }

                    panic!("Failed to parse token in file {}", filename.to_str().unwrap());
                }
            }
        }
    }
}

#[derive(ParseEnumToken, Debug)]
pub enum Token {
    Comment(Comment),
    ReservedToken(ReservedToken),
    RawIdentifier(RawIdentifier),
    CharLiteral(CharLiteral),
    StringLiteral(StringLiteral),
    RawStringLiteral(RawStringLiteral),
    ByteLiteral(ByteLiteral),
    ByteStringLiteral(ByteStringLiteral),
    RawByteStringLiteral(RawByteStringLiteral),
    CStringLiteral(CStringLiteral),
    RawCStringLiteral(RawCStringLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    LifetimeToken(LifetimeToken),
    Punctuation(Punctuation),
    IdentifierOrKeyword(IdentifierOrKeyword),
}

#[derive(ParseEnumToken, Debug)]
pub enum Comment {
    LineComment(LineComment),
    BlockComment(BlockComment),
}

#[derive(Debug)]
pub struct LineComment;

impl tokenizer_trait::Token for LineComment {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '/' || data.next()? != '/' {
            return None;
        }
        while let Some(chr) = data.peek() {
            if *chr == '\n' {
                break;
            }
            data.next();
        }
        Some((Self, data))
    }
}

#[derive(Debug)]
pub struct BlockComment;

impl tokenizer_trait::Token for BlockComment {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '/' || data.next()? != '*' {
            return None;
        }
        loop {
            let chr = data.next()?;
            if chr == '*' && *data.peek()? == '/' {
                data.next();
                break;
            }
        }
        Some((Self, data))
    }
}
