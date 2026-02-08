use tokenizer_trait::ParseIterator;

use crate::tokenizer::{string_escapes::UnicodeEscape, suffix::Suffix};

#[derive(Debug)]
pub(in crate::tokenizer) struct CharLiteral {
    value: char,
    suffix: Option<Suffix>,
}

impl tokenizer_trait::Token for CharLiteral {
    fn parse_token(mut data: ParseIterator) -> Option<(Self, ParseIterator)> {
        if data.next()? != '\'' {
            return None;
        }

        let end_chr: char;
        if *data.peek()? == '\\' {
            //byte escape
            if let Some(byte_escape) =
                crate::tokenizer::string_escapes::ByteEscape::parse_token(data.clone())
            {
                if byte_escape.0.value() >= 0x80 {
                    return None;
                }
                end_chr = byte_escape.0.value() as char;
                data = byte_escape.1;
            } else if let Some(unicode_escape) = UnicodeEscape::parse_token(data.clone()) {
                if unicode_escape.0.is_null() {
                    return None;
                }
                data = unicode_escape.1;
                end_chr = unicode_escape.0.to_char()?;
            } else {
                data.next()?; //pop the backslash
                let next = data.next()?;
                if next == '\'' || next == '"' {
                    end_chr = next;
                } else {
                    return None;
                }
            }
        } else {
            let chr = data.next()?;
            if !chr.is_ascii() {
                return None;
            }
            if chr == '\r' || chr == '\n' || chr == '\t' || chr == '\'' {
                return None;
            }
            end_chr = chr;
        }

        if data.next()? != '\'' {
            return None;
        }
        if let Some(suffix) = Suffix::parse_token(data.clone()) {
            return Some((
                Self {
                    value: end_chr,
                    suffix: Some(suffix.0),
                },
                suffix.1,
            ));
        }
        Some((
            Self {
                value: end_chr,
                suffix: None,
            },
            data,
        ))
    }
}
