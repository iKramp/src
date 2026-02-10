use tokenizer_trait::{SrcIterator, Token};

use crate::{
    integer_literal::DecLiteral,
    suffix::{Suffix, SuffixNoE},
};

#[derive(Debug)]
pub struct FloatLiteral {
    whole_part: DecLiteral,
    fractional_part: Option<DecLiteral>,
    exponent_part: Option<ExponentPart>,
    suffix: Option<Suffix>,
}

impl tokenizer_trait::Token for FloatLiteral {
    fn parse_token(data: SrcIterator) -> Option<(Self, SrcIterator)> {
        let (whole_part, data) = DecLiteral::parse_token(data)?;

        if let Some(parsed) = parse_third_form(data.clone()) {
            return Some((
                Self {
                    whole_part,
                    fractional_part: parsed.1,
                    exponent_part: Some(parsed.2),
                    suffix: parsed.3,
                },
                parsed.0,
            ));
        }

        if let Some(parsed) = parse_second_form(data.clone()) {
            return Some((
                Self {
                    whole_part,
                    fractional_part: Some(parsed.1),
                    exponent_part: None,
                    suffix: parsed.2.map(|s| s.into_generic_suffix()),
                },
                parsed.0,
            ));
        }

        if let Some(parsed) = parse_first_form(data.clone()) {
            return Some((
                Self {
                    whole_part,
                    fractional_part: None,
                    exponent_part: None,
                    suffix: None,
                },
                parsed,
            ));
        }

        None
    }
}

fn parse_first_form(mut data: SrcIterator) -> Option<SrcIterator> {
    if data.next()? != '.' {
        return None;
    }
    let Some(next) = data.peek() else {
        return Some(data);
    };
    if *next == '.' || *next == '_' || unicode_ident::is_xid_start(*next) {
        return None;
    }
    Some(data)
}

fn parse_second_form(
    mut data: SrcIterator,
) -> Option<(SrcIterator, DecLiteral, Option<SuffixNoE>)> {
    if data.next()? != '.' {
        return None;
    }
    let (fractional_part, data) = DecLiteral::parse_token(data)?;
    if let Some(a) = SuffixNoE::parse_token(data.clone()) {
        return Some((a.1, fractional_part, Some(a.0)));
    }
    Some((data, fractional_part, None))
}

fn parse_third_form(
    mut data: SrcIterator,
) -> Option<(
    SrcIterator,
    Option<DecLiteral>,
    ExponentPart,
    Option<Suffix>,
)> {
    let data_cont;
    let fractional_part;
    if *data.peek()? == '.' {
        //parse fractional part
        data.next();
        let (_fractional_part, data) = DecLiteral::parse_token(data)?;
        data_cont = data;
        fractional_part = Some(_fractional_part);
    } else {
        data_cont = data;
        fractional_part = None;
    }

    let exponent_part = ExponentPart::parse_token(data_cont)?;

    if let Some(suffix) = Suffix::parse_token(exponent_part.1.clone()) {
        return Some((suffix.1, fractional_part, exponent_part.0, Some(suffix.0)));
    }
    Some((exponent_part.1, fractional_part, exponent_part.0, None))
}

#[derive(Debug)]
struct ExponentPart {
    sign: Option<bool>, //true for +, false for -
    exponent: DecLiteral,
}

impl tokenizer_trait::Token for ExponentPart {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        let marker = data.next()?;
        let sign;
        if marker != 'e' && marker != 'E' {
            return None;
        }
        let second = *data.peek()?;
        if second == '+' {
            data.next();
            sign = Some(true);
        } else if second == '-' {
            data.next();
            sign = Some(false);
        } else {
            sign = None;
        }

        while let Some('_') = data.peek() {
            data.next();
        }

        let exponent = DecLiteral::parse_token(data)?;
        Some((
            Self {
                sign,
                exponent: exponent.0,
            },
            exponent.1,
        ))
    }
}
