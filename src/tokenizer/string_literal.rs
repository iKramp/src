use tokenizer_trait::ParseIterator;

use crate::tokenizer::{string_escapes::{ByteEscape, UnicodeEscape}, suffix::Suffix};



#[derive(Debug)]
pub(in crate::tokenizer) struct StringLiteral {
    value: String,
    suffix: Option<Suffix>
}

impl tokenizer_trait::Token for StringLiteral {
    fn parse_token(mut data: ParseIterator) -> Option<(Self, ParseIterator)> {
        if data.next()? != '"' {
            return None;
        }

        let mut content = String::new();

        while let Some(chr) = data.peek() {
            if *chr == '\\' {
                //handle escapes
                if let Some(byte_escape) = ByteEscape::parse_token(data.clone()) {
                    if byte_escape.0.value() >= 0x80 {
                        return None;
                    }
                    content.push(byte_escape.0.value() as char);
                    data = byte_escape.1;
                    continue;
                }
                if let Some(unicode_escape) = UnicodeEscape::parse_token(data.clone()) {
                    data = unicode_escape.1;
                    content.push(unicode_escape.0.to_char()?);

                    continue;
                }
                data.next()?;
                if *data.peek()? == '\'' {
                    content.push('\'');
                    data.next();
                    continue;
                }
                if *data.peek()? == '"' {
                    content.push('"');
                    data.next();
                    continue;
                }


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
            content.push(chr);
        }
        None
    }
}
