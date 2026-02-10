use tokenizer_trait::SrcIterator;


#[derive(Debug)]
pub struct ByteEscape {
    byte: u8,
}

impl ByteEscape {
    pub fn value(&self) -> u8 {
        self.byte
    }
}

impl tokenizer_trait::Token for ByteEscape {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '\\' {
            return None;
        }

        let next_char = data.next()?;
        match next_char {
            'n' => Some((Self { byte: b'\n' }, data)),
            'r' => Some((Self { byte: b'\r' }, data)),
            't' => Some((Self { byte: b'\t' }, data)),
            '\\' => Some((Self { byte: b'\\' }, data)),
            '0' => Some((Self { byte: b'\0' }, data)),
            '\'' => Some((Self { byte: b'\'' }, data)),
            '\"' => Some((Self { byte: b'\"' }, data)),
            'x' => {
                let high = data.next()?;
                let low = data.next()?;
                let hex_str = format!("{}{}", high, low);
                if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                    Some((Self { byte }, data))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct UnicodeEscape {
    bytes: Box<[u8]>,
}

impl tokenizer_trait::Token for UnicodeEscape {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '\\' || data.next()? != 'u' || data.next()? != '{' {
            return None;
        }
        println!("Parsing Unicode escape sequence...");

        let mut content = Vec::new();

        for i in 0..6 {
            if let Some('}') = data.peek() {
                if i == 0 {
                    println!("No hex digits found in Unicode escape sequence");
                    return None; // No hex digits found
                }
                data.next(); // Consume the closing '}'
                println!("Finished parsing Unicode escape sequence: {:?}", content);
                return Some((
                    Self::from_ascii_sequence(&content)?,
                    data,
                ));
            }
            let next_char = data.next()?;
            println!("Read character '{}' in Unicode escape sequence", next_char);
            let digit = match next_char {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' | 'A' => 0xa,
                'b' | 'B' => 0xb,
                'c' | 'C' => 0xc,
                'd' | 'D' => 0xd,
                'e' | 'E' => 0xe,
                'f' | 'F' => 0xf,
                _ => return None,
            };
            content.push(digit);
            while data.peek() == Some(&'_') {
                data.next(); // Skip underscores
            }
            
        }
        None
    }
}

impl UnicodeEscape {
    pub fn from_ascii_sequence(data: &[u8]) -> Option<Self> {
        let code_point = data.iter().fold(0u32, |acc, &b| (acc << 4) | b as u32);
        if code_point > 0x10FFFF {
            return None; // Invalid Unicode code point
        }
        if data.len() > 6 || data.is_empty() {
            return None;
        }
        Some(Self { bytes: data.to_vec().into_boxed_slice() })
    }

    pub fn is_null(&self) -> bool {
        self.bytes.iter().all(|&b| b == 0)
    }

    pub fn to_utf8_bytes(&self) -> Box<[u8]> {
        let code_point = self.bytes.iter().fold(0u32, |acc, &b| (acc << 4) | b as u32);
        let mut utf8_bytes = Vec::new();
        if code_point <= 0x7F {
            utf8_bytes.push(code_point as u8);
        } else if code_point <= 0x7FF {
            utf8_bytes.push(0b11000000 | ((code_point >> 6) as u8));
            utf8_bytes.push(0b10000000 | ((code_point & 0b00111111) as u8));
        } else if code_point <= 0xFFFF {
            utf8_bytes.push(0b11100000 | ((code_point >> 12) as u8));
            utf8_bytes.push(0b10000000 | (((code_point >> 6) & 0b00111111) as u8));
            utf8_bytes.push(0b10000000 | ((code_point & 0b00111111) as u8));
        } else {
            utf8_bytes.push(0b11110000 | ((code_point >> 18) as u8));
            utf8_bytes.push(0b10000000 | (((code_point >> 12) & 0b00111111) as u8));
            utf8_bytes.push(0b10000000 | (((code_point >> 6) & 0b00111111) as u8));
            utf8_bytes.push(0b10000000 | ((code_point & 0b00111111) as u8));
        }
        utf8_bytes.into_boxed_slice()
    }

    pub fn to_char(&self) -> Option<char> {
        let code_point = self.bytes.iter().fold(0u32, |acc, &b| (acc << 4) | b as u32);
        std::char::from_u32(code_point)
    }
}
