use tokenizer_trait::SrcIterator;

use crate::suffix::SuffixNoE;

#[derive(Debug)]
pub struct IntegerLiteral {
    kind: IntegerLiteralKind,
    suffix: Option<SuffixNoE>,
}

impl tokenizer_trait::Token for IntegerLiteral {
    fn parse_token(data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if let Some((bin_literal, data)) = BinLiteral::parse_token(data.clone()) {
            let suffix = SuffixNoE::parse_token(data.clone());
            let new_iter = suffix.as_ref().map_or(data.clone(), |s| s.1.clone());
            Some((
                Self {
                    kind: IntegerLiteralKind::Bin(bin_literal),
                    suffix: suffix.map(|s| s.0),
                },
                new_iter,
            ))
        } else if let Some((oct_literal, data)) = OctLiteral::parse_token(data.clone()) {
            let suffix = SuffixNoE::parse_token(data.clone());
            let new_iter = suffix.as_ref().map_or(data.clone(), |s| s.1.clone());
            Some((
                Self {
                    kind: IntegerLiteralKind::Oct(oct_literal),
                    suffix: suffix.map(|s| s.0),
                },
                new_iter,
            ))
        } else if let Some((hex_literal, data)) = HexLiteral::parse_token(data.clone()) {
            let suffix = SuffixNoE::parse_token(data.clone());
            let new_iter = suffix.as_ref().map_or(data.clone(), |s| s.1.clone());
            Some((
                Self {
                    kind: IntegerLiteralKind::Hex(hex_literal),
                    suffix: suffix.map(|s| s.0),
                },
                new_iter,
            ))
        } else {
            let (dec_literal, data) = DecLiteral::parse_token(data)?;
            let suffix = SuffixNoE::parse_token(data.clone());
            let new_iter = suffix.as_ref().map_or(data.clone(), |s| s.1.clone());
            Some((
                Self {
                    kind: IntegerLiteralKind::Dec(dec_literal),
                    suffix: suffix.map(|s| s.0),
                },
                new_iter,
            ))
        }
    }
}

#[derive(Debug)]
enum IntegerLiteralKind {
    Bin(BinLiteral),
    Oct(OctLiteral),
    Dec(DecLiteral),
    Hex(HexLiteral),
}

#[derive(Debug)]
enum BinDigit {
    Zero,
    One,
}

#[derive(Debug)]
enum OctDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[derive(Debug)]
enum DecDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug)]
pub enum HexDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(Debug)]
pub struct DecLiteral {
    content: Box<[DecDigit]>,
}

impl tokenizer_trait::Token for DecLiteral {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        let mut content = Vec::new();
        while let Some(chr) = data.peek() {
            let digit = match chr {
                '0' => DecDigit::Zero,
                '1' => DecDigit::One,
                '2' => DecDigit::Two,
                '3' => DecDigit::Three,
                '4' => DecDigit::Four,
                '5' => DecDigit::Five,
                '6' => DecDigit::Six,
                '7' => DecDigit::Seven,
                '8' => DecDigit::Eight,
                '9' => DecDigit::Nine,
                '_' => {
                    data.next();
                    continue;
                },
                _ => break,
            };
            content.push(digit);
            data.next();
        }
        if content.is_empty() {
            return None;
        }
        Some((
            Self {
                content: content.into_boxed_slice(),
            },
            data,
        ))
    }
}

#[derive(Debug)]
pub struct BinLiteral {
    content: Box<[BinDigit]>,
}

impl tokenizer_trait::Token for BinLiteral {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '0' || data.next()? != 'b' {
            return None;
        }
        let mut content = Vec::new();
        while let Some(chr) = data.peek() {
            let digit = match chr {
                '0' => BinDigit::Zero,
                '1' => BinDigit::One,
                '_' => {
                    data.next();
                    continue;
                },
                _ => break,
            };
            content.push(digit);
            data.next();
        }
        if content.is_empty() {
            return None;
        }
        Some((
            Self {
                content: content.into_boxed_slice(),
            },
            data,
        ))
    }
}

#[derive(Debug)]
pub struct OctLiteral {
    content: Box<[OctDigit]>,
}

impl tokenizer_trait::Token for OctLiteral {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '0' || data.next()? != 'o' {
            return None;
        }
        let mut content = Vec::new();
        while let Some(chr) = data.peek() {
            let digit = match chr {
                '0' => OctDigit::Zero,
                '1' => OctDigit::One,
                '2' => OctDigit::Two,
                '3' => OctDigit::Three,
                '4' => OctDigit::Four,
                '5' => OctDigit::Five,
                '6' => OctDigit::Six,
                '7' => OctDigit::Seven,
                '_' => {
                    data.next();
                    continue;
                },
                _ => break,
            };
            content.push(digit);
            data.next();
        }
        if content.is_empty() {
            return None;
        }
        Some((
            Self {
                content: content.into_boxed_slice(),
            },
            data,
        ))
    }
}

#[derive(Debug)]
pub struct HexLiteral {
    content: Box<[HexDigit]>,
}

impl tokenizer_trait::Token for HexLiteral {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        if data.next()? != '0' || data.next()? != 'x' {
            return None;
        }
        let mut content = Vec::new();
        while let Some(chr) = data.peek() {
            let digit = match chr {
                '0' => HexDigit::Zero,
                '1' => HexDigit::One,
                '2' => HexDigit::Two,
                '3' => HexDigit::Three,
                '4' => HexDigit::Four,
                '5' => HexDigit::Five,
                '6' => HexDigit::Six,
                '7' => HexDigit::Seven,
                '8' => HexDigit::Eight,
                '9' => HexDigit::Nine,
                'a' | 'A' => HexDigit::A,
                'b' | 'B' => HexDigit::B,
                'c' | 'C' => HexDigit::C,
                'd' | 'D' => HexDigit::D,
                'e' | 'E' => HexDigit::E,
                'f' | 'F' => HexDigit::F,
                '_' => {
                    data.next();
                    continue;
                },
                _ => break,
            };
            content.push(digit);
            data.next();
        }
        if content.is_empty() {
            return None;
        }
        Some((
            Self {
                content: content.into_boxed_slice(),
            },
            data,
        ))
    }
}
