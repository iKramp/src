use tokenizer_trait::ParseIterator;

use crate::tokenizer::suffix::Suffix;



#[derive(Debug)]
pub(in crate::tokenizer) struct ByteLiteral {
    value: u8,
    suffix: Option<Suffix>
}

impl tokenizer_trait::Token for ByteLiteral {
    fn parse_token(mut data: ParseIterator) -> Option<(Self, ParseIterator)> {
        if data.next()? != 'b' || data.next()? != '\'' {
            return None;
        }

        let byte;
        if *data.peek()? == '\\' {
            //byte escape
            if let Some(byte_escape) = crate::tokenizer::string_escapes::ByteEscape::parse_token(data.clone()) {
                if byte_escape.0.value() == 0 {
                    return None;
                }
                byte = byte_escape.0.value();
                data = byte_escape.1;
            } else {
                return None;
            }
        } else {
            let chr = data.next()?;
            if !chr.is_ascii() {
                return None;
            }
            if chr == '\r' || chr == '\n' || chr == '\t' || chr == '\'' {
                return None;
            }
            byte = chr as u8;
        }

        if data.next()? != '\'' {
            return None;
        }
        if let Some(suffix) = Suffix::parse_token(data.clone()) {
            return Some((
                Self { value: byte, suffix: Some(suffix.0) },
                suffix.1,
            ));
        }
        Some((
            Self { value: byte, suffix: None },
            data,
        ))
    }
}
