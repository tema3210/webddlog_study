use arcstr::ArcStr;

#[derive(Debug,Clone)]
pub enum BraceKind {
    /// {}
    Brace,
    // []
    Braket,
    // ()
    Parethesis,
    // <>
    AngleBrace
}
#[derive(Debug,Clone)]
pub enum OperatorSign {
    And,
    Plus,
    Minus,
    /// `|`
    Pipe,
    Mul,
    Div
}

/// A special character encountered in text
#[derive(Debug,Clone)]
pub enum Special {
    /// `=`
    EqSign,
    /// `:`
    Colon,
    /// `;`
    SemiColon
}
#[derive(Debug,Clone)]
pub enum Literal {
    String(ArcStr),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

/// A fragment of text, formated correctly
#[derive(Debug,Clone)]
pub enum Token {
    UcIdent(ArcStr),
    LcIdent(ArcStr),
    Special(Special),
    Literal(Literal),
    OperatorSign(OperatorSign),
    BraceRight(BraceKind),
    BraceLeft(BraceKind),
}

impl Token {
    pub fn from_single_char(ch: char) -> Option<Self> {
        Some(match ch {
            '{' => Token::BraceLeft(BraceKind::Brace),
            '[' => Token::BraceLeft(BraceKind::Braket),
            '(' => Token::BraceLeft(BraceKind::Parethesis),
            '<' => Token::BraceLeft(BraceKind::AngleBrace),

            '}' => Token::BraceRight(BraceKind::Brace),
            ']' => Token::BraceRight(BraceKind::Braket),
            ')' => Token::BraceRight(BraceKind::Parethesis),
            '>' => Token::BraceRight(BraceKind::AngleBrace),
            
            '=' => Token::Special(Special::EqSign),
            ':' => Token::Special(Special::Colon),
            ';' => Token::Special(Special::SemiColon),

            '&' => Token::OperatorSign(OperatorSign::And),
            '+' => Token::OperatorSign(OperatorSign::Plus),
            '-' => Token::OperatorSign(OperatorSign::Minus),
            '|' => Token::OperatorSign(OperatorSign::Pipe),
            '*' => Token::OperatorSign(OperatorSign::Mul),
            '/' => Token::OperatorSign(OperatorSign::Div),

            _ => return None
        })
    }
}
