use tokenizer_trait::SrcIterator;

use crate::{string_escapes::ByteEscape, suffix::Suffix};


#[derive(Debug)]
pub struct ByteStringLiteral {
    value: Vec<u8>,
    suffix: Option<Suffix>
}

impl tokenizer_trait::Token for ByteStringLiteral {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != 'b' || data.next()? != '"' {
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
            }
            let chr = data.next()?;
            if chr == '\r' {
                return None;
            }
            if chr == '"' {
                let temp_peekable = data.clone();
                if let Some(suffix) = Suffix::parse_token(temp_peekable.clone()) {
                    return Some((
                        Self { value: content, suffix: Some(suffix.0) },
                        suffix.1,
                    ));
                }
                return Some((
                    Self { value: content, suffix: None },
                    temp_peekable,
                ));
            }
            if !chr.is_ascii() {
                return None;
            }
            content.push(chr as u8);
        }

        None
    }
}
