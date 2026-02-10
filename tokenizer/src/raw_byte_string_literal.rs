use tokenizer_trait::SrcIterator;

use crate::suffix::Suffix;


#[derive(Debug)]
pub struct RawByteStringLiteral {
    value: Box<[u8]>,
    suffix: Option<Suffix>
}

impl tokenizer_trait::Token for RawByteStringLiteral {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != 'b' || data.next()? != 'r' {
            return None;
        }
        let mut num_hashes = 0;
        while let Some('#') = data.peek() && num_hashes < 256 {
            num_hashes += 1;
            data.next();
        }

        if num_hashes >= 256 {
            return None;
        }

        if data.next()? != '"' {
            return None;
        }

        let mut parsed = Vec::new();

        while let Some(chr) = data.next() {
            if chr == '\r' {
                return None;
            }
            if chr == '"' {
                let mut temp_peekable = data.clone();
                let mut num_after_hashes = 0;
                while let Some('#') = temp_peekable.peek() {
                    num_after_hashes += 1;
                    temp_peekable.next();
                }
                if num_after_hashes == num_hashes {
                    //found the end of the string literal
                    
                    if let Some(suffix) = Suffix::parse_token(temp_peekable.clone()) {
                        return Some((
                            Self { value: parsed.into_boxed_slice(), suffix: Some(suffix.0) },
                            suffix.1,
                        ));
                    }
                    
                    return Some((
                        Self { value: parsed.into_boxed_slice(), suffix: None },
                        temp_peekable,
                    ));
                }
            }
            if !chr.is_ascii() {
                return None;
            }
            parsed.push(chr as u8);
        }


        None
    }
}
