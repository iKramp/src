use tokenizer_trait::{SrcIterator, Token};

const STRICT_KEYWORD_LIST: &[&str] = &[
    "_", "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
    "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
    "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true",
    "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
];

const RESERVED_KEYWORD_LIST: &[&str] = &[
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try", "gen",
];

#[derive(Debug)]
pub struct IdentifierOrKeyword {
    parsed: String,
}

impl IdentifierOrKeyword {
    pub fn parsed(&self) -> &str {
        &self.parsed
    }
}

impl Token for IdentifierOrKeyword {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)>
    where
        Self: Sized,
    {
        let first = data.next()?;
        if first != '_' && !unicode_ident::is_xid_start(first) {
            return None;
        }

        let mut parsed = String::new();
        parsed.push(first);
        while let Some(chr) = data.peek() {
            let is_xid_continue = unicode_ident::is_xid_continue(*chr);
            if !is_xid_continue {
                break;
            }
            parsed.push(*chr);
            data.next();
        }

        Some((Self { parsed }, data))
    }
}

#[derive(Debug)]
pub struct RawIdentifier {
    inner: IdentifierOrKeyword,
}

impl Token for RawIdentifier {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)>
    where
        Self: Sized,
    {
        if data.next()? == 'r' && data.next()? == '#' {
            let inner = IdentifierOrKeyword::parse_token(data)?;
            return Some((Self { inner: inner.0 }, inner.1));
        }
        None
    }
}

#[derive(Debug)]
pub struct NonKeywordIdentifier {
    inner: IdentifierOrKeyword,
}

impl Token for NonKeywordIdentifier {
    fn parse_token(data: SrcIterator) -> Option<(Self, SrcIterator)>
    where
        Self: Sized,
    {
        let inner = IdentifierOrKeyword::parse_token(data)?;
        if STRICT_KEYWORD_LIST.contains(&inner.0.parsed.as_str())
            || RESERVED_KEYWORD_LIST.contains(&inner.0.parsed.as_str())
        {
            return None;
        }
        Some((Self { inner: inner.0 }, inner.1))
    }
}

#[derive(tokenizer_macro::ParseEnumToken, Debug)]
pub enum Identifier {
    NonKeywordIdentifier(NonKeywordIdentifier),
    RawIdentifier(RawIdentifier),
}

#[derive(Debug)]
pub struct ReservedRawIdentifier;

impl Token for ReservedRawIdentifier {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)>
    where
        Self: Sized,
    {
        if data.next()? != 'r' || data.next()? != '#' {
            return None;
        }
        const RESERVED_RAW_IDENT_TABLE: &[&str] = &["_", "crate", "self", "Self", "super"];
        for reserved_raw_ident in RESERVED_RAW_IDENT_TABLE {
            let zipped = reserved_raw_ident.chars().zip(data.clone());
            if zipped.clone().all(|(a, b)| a == b) && zipped.count() == reserved_raw_ident.len() {
                (0..reserved_raw_ident.len()).for_each(|_| {
                    data.next();
                });
                return Some((
                    Self,
                    data,
                ));
            }
        }
        None
    }
}
