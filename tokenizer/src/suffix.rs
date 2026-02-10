use tokenizer_trait::SrcIterator;

use crate::IdentifierOrKeyword;



#[derive(Debug)]
pub struct Suffix {
    parsed: IdentifierOrKeyword,
}

impl tokenizer_trait::Token for Suffix {
    fn parse_token(data: SrcIterator) -> Option<(Self, SrcIterator)> {
        let inner = IdentifierOrKeyword::parse_token(data)?;
        if inner.0.parsed() == "_" {
            return None;
        }
        Some((Self{parsed: inner.0}, inner.1))
    }
}

impl Suffix {
    pub fn parsed(&self) -> &str {
        &self.parsed.parsed()
    }

    pub fn into_no_e_suffix(self) -> Option<SuffixNoE> {
        if self.parsed().starts_with('e') || self.parsed().starts_with('E') {
            return None;
        }
        Some(SuffixNoE{parsed: self})
    }
}

#[derive(Debug)]
pub struct SuffixNoE {
    parsed: Suffix,
}

impl tokenizer_trait::Token for SuffixNoE {
    fn parse_token(data: SrcIterator) -> Option<(Self, SrcIterator)> {
        let inner = Suffix::parse_token(data)?;
        if inner.0.parsed().starts_with('e') || inner.0.parsed().starts_with('E') {
            return None;
        }
        Some((Self{parsed: inner.0}, inner.1))
    }
}

impl SuffixNoE {
    pub fn parsed(&self) -> &str {
        self.parsed.parsed()
    }

    pub fn into_generic_suffix(self) -> Suffix {
        self.parsed
    }
}
