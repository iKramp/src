use crate::IdentifierOrKeyword;


#[derive(Debug)]
struct RawIdentifier {
    inner: IdentifierOrKeyword,
}

impl tokenizer_trait::Token for RawIdentifier {
    fn parse_token(mut data: tokenizer_trait::SrcIterator) -> Option<(Self, tokenizer_trait::SrcIterator)> {
        if data.next()? == 'r' && data.next()? == '#' {
            let inner = IdentifierOrKeyword::parse_token(data)?;
            return Some((Self { inner: inner.0 }, inner.1));
        }
        None
    }
}
