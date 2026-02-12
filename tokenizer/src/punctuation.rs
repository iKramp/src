use tokenizer_trait::SrcIterator;


#[derive(Debug, PartialEq, Eq)]
pub enum Punctuation {
    SingleEqual,
    Less,
    LessEqual,
    DobuleEqual,
    NotEqual,
    GreaterEqual,
    Greater,
    DobuleAnd,
    DobuleOr,
    Bang,
    Tilde,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    SingleAnd,
    SingleOr,
    DoubleLess,
    DoubleGreater,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    CaretEqual,
    SingleAndEqual,
    SingleOrEqual,
    DoubleLessEqual,
    DoubleGreaterEqual,
    At,
    DoubleDotEqual,
    TripleDot,
    DoubleDot,
    SingleDot,
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    DashGreater,
    LessDash,
    EqualGreater,
    Hash,
    Dollar,
    Question,
    LeftCurly,
    RightCurly,
    LeftSquare,
    RightSquare,
    LeftParen,
    RightParen,
}

impl tokenizer_trait::Token for Punctuation {
    fn parse_token(mut data: SrcIterator) -> Option<(Self, SrcIterator)> {
        //first try match len 3
        let mut data_3_taken = data.clone();
        (0..3).for_each(|_| {
            data_3_taken.next();
        });
        let first_three = data.clone().take(3).collect::<String>();
        match first_three.as_str() {
            "<<=" => return Some((Self::DoubleLessEqual, data_3_taken)),
            ">>=" => return Some((Self::DoubleGreaterEqual, data_3_taken)),
            "..." => return Some((Self::TripleDot, data_3_taken)),
            "..=" => return Some((Self::DoubleDotEqual, data_3_taken)),
            _ => {}
        }
        //then try match len 2
        let mut data_2_taken = data.clone();
        (0..2).for_each(|_| {
            data_2_taken.next();
        });
        let first_two = data.clone().take(2).collect::<String>();
        match first_two.as_str() {
            "<=" => return Some((Self::LessEqual, data_2_taken)),
            "==" => return Some((Self::DobuleEqual, data_2_taken)),
            "!=" => return Some((Self::NotEqual, data_2_taken)),
            ">=" => return Some((Self::GreaterEqual, data_2_taken)),
            "&&" => return Some((Self::DobuleAnd, data_2_taken)),
            "||" => return Some((Self::DobuleOr, data_2_taken)),
            "<<" => return Some((Self::DoubleLess, data_2_taken)),
            ">>" => return Some((Self::DoubleGreater, data_2_taken)),
            "+=" => return Some((Self::PlusEqual, data_2_taken)),
            "-=" => return Some((Self::MinusEqual, data_2_taken)),
            "*=" => return Some((Self::StarEqual, data_2_taken)),
            "/=" => return Some((Self::SlashEqual, data_2_taken)),
            "%=" => return Some((Self::PercentEqual, data_2_taken)),
            "^=" => return Some((Self::CaretEqual, data_2_taken)),
            "&=" => return Some((Self::SingleAndEqual, data_2_taken)),
            "|=" => return Some((Self::SingleOrEqual, data_2_taken)),
            ".." => return Some((Self::DoubleDot, data_2_taken)),
            "::" => return Some((Self::DoubleColon, data_2_taken)),
            "->" => return Some((Self::DashGreater, data_2_taken)),
            "<-" => return Some((Self::LessDash, data_2_taken)),
            "=>" => return Some((Self::EqualGreater, data_2_taken)),
            _ => {}
        }
        //then try match len 1
        let mut data_1_taken = data.clone();
        data_1_taken.next();
        let first_char = *data.peek()?;
        match first_char {
            '=' => return Some((Self::SingleEqual, data_1_taken)),
            '<' => return Some((Self::Less, data_1_taken)),
            '>' => return Some((Self::Greater, data_1_taken)),
            '!' => return Some((Self::Bang, data_1_taken)),
            '~' => return Some((Self::Tilde, data_1_taken)),
            '+' => return Some((Self::Plus, data_1_taken)),
            '-' => return Some((Self::Minus, data_1_taken)),
            '*' => return Some((Self::Star, data_1_taken)),
            '/' => return Some((Self::Slash, data_1_taken)),
            '%' => return Some((Self::Percent, data_1_taken)),
            '^' => return Some((Self::Caret, data_1_taken)),
            '&' => return Some((Self::SingleAnd, data_1_taken)),
            '|' => return Some((Self::SingleOr, data_1_taken)),
            '@' => return Some((Self::At, data_1_taken)),
            '.' => return Some((Self::SingleDot, data_1_taken)),
            ',' => return Some((Self::Comma, data_1_taken)),
            ';' => return Some((Self::Semicolon, data_1_taken)),
            ':' => return Some((Self::Colon, data_1_taken)),
            '#' => return Some((Self::Hash, data_1_taken)),
            '$' => return Some((Self::Dollar, data_1_taken)),
            '?' => return Some((Self::Question, data_1_taken)),
            '{' => return Some((Self::LeftCurly, data_1_taken)),
            '}' => return Some((Self::RightCurly, data_1_taken)),
            '[' => return Some((Self::LeftSquare, data_1_taken)),
            ']' => return Some((Self::RightSquare, data_1_taken)),
            '(' => return Some((Self::LeftParen, data_1_taken)),
            ')' => return Some((Self::RightParen, data_1_taken)),
            _ => {}
        }

        None
    }
}
