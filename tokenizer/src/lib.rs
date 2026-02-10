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

pub fn tokenize(mut data: SrcIterator, filename: &str) {
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
                    println!("Parsed reserved token: {:?}", a);
                }
            }
            None => {
                if data.peek().is_none() {
                    return;
                } else {
                    println!("Failed to parse token");
                    for _ in 0..1000 {
                        if let Some(chr) = data.next() {
                            print!("{}", chr);
                        } else {
                            break;
                        }
                    }

                    panic!("Failed to parse token in file {}", filename);
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
