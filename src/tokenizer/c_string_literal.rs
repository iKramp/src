use tokenizer_trait::ParseIterator;

use crate::tokenizer::{string_escapes::{ByteEscape, UnicodeEscape}, suffix::Suffix};


#[derive(Debug)]
pub(in crate::tokenizer) struct CStringLiteral {
    value: Vec<u8>,
    suffix: Option<Suffix>
}

impl tokenizer_trait::Token for CStringLiteral {
    fn parse_token(mut data: ParseIterator) -> Option<(Self, ParseIterator)> {
        if data.next()? != 'c' || data.next()? != '"' {
            return None;
        }

        let mut content = Vec::new();

        while let Some(chr) = data.peek() {
            if *chr == '\\' {
                //handle escapes
                if let Some(byte_escape) = ByteEscape::parse_token(data.clone()) {
                    if byte_escape.0.value() == 0 {
                        return None;
                    }
                    content.push(byte_escape.0.value());
                    data = byte_escape.1;
                    continue;
                }
                if let Some(unicode_escape) = UnicodeEscape::parse_token(data.clone()) {
                    if unicode_escape.0.is_null() {
                        return None;
                    }
                    data = unicode_escape.1;
                    content.extend_from_slice(&unicode_escape.0.to_utf8_bytes());

                    continue;
                }
                data.next()?;
                if data.next()? == '\n' { // \LF string continue
                    while let Some(chr) = data.peek() {
                        if *chr == ' ' || *chr == '\t' || *chr == '\n' || *chr == '\r' {
                            data.next();
                        } else {
                            break;
                        }
                    }
                    continue;
                }

                return None;
            }

            let chr = data.next()?;
            if chr == '\r' || chr == '\0' {
                return None;
            }
            if chr == '"' {
                if let Some(suffix) = Suffix::parse_token(data.clone()) {
                    return Some((
                        Self { value: content, suffix: Some(suffix.0) },
                        suffix.1,
                    ));
                }
                return Some((
                    Self { value: content, suffix: None },
                    data,
                ));
            }
            content.extend_from_slice(chr.to_string().as_bytes());
        }
        None
    }
}
