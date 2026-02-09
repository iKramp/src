mod identifier_or_keyword;
mod punctuation;
mod lifetime_token;
mod float_literal;
mod integer_literal;
mod suffix;
mod raw_c_string_literal;
mod c_string_literal;
mod string_escapes;
mod raw_byte_string_literal;
mod byte_string_literal;
mod byte_literal;
mod raw_string_literal;
mod string_literal;
mod char_literal;
mod raw_identifier;
mod reserved_token;

use tokenizer_macro::ParseEnumToken;
use tokenizer_trait::{ParseIterator, Token as TokenTrait};

use crate::tokenizer::{byte_literal::ByteLiteral, byte_string_literal::ByteStringLiteral, c_string_literal::CStringLiteral, char_literal::CharLiteral, float_literal::FloatLiteral, identifier_or_keyword::{IdentifierOrKeyword, RawIdentifier}, integer_literal::IntegerLiteral, lifetime_token::LifetimeToken, punctuation::Punctuation, raw_byte_string_literal::RawByteStringLiteral, raw_c_string_literal::RawCStringLiteral, raw_string_literal::RawStringLiteral, reserved_token::ReservedToken, string_literal::StringLiteral};

pub fn tokenize(mut data: ParseIterator, filename: &str) {
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

#[derive(Debug)]
struct TempToken;

impl tokenizer_trait::Token for TempToken {
    fn parse_token(data: ParseIterator) -> Option<(Self, ParseIterator)> {
        None
    }
}

#[derive(ParseEnumToken, Debug)]
enum Token {
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
enum Comment {
    LineComment(LineComment),
    BlockComment(BlockComment),
}

#[derive(Debug)]
struct LineComment;

impl tokenizer_trait::Token for LineComment {
    fn parse_token(mut data: ParseIterator) -> Option<(Self, ParseIterator)> {
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
struct BlockComment;

impl tokenizer_trait::Token for BlockComment {
    fn parse_token(mut data: ParseIterator) -> Option<(Self, ParseIterator)> {
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
