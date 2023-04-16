use std::future::Future;

use arcstr::ArcStr;
use genawaiter::{rc,yield_, Generator};

#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub enum Special {
    /// `=`
    EqSign,
    /// `:`
    Colon,
    /// `;`
    SemiColon
}
#[derive(Debug)]
pub enum Literal {
    String(ArcStr),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

/// A fragment of text, formated correctly
#[derive(Debug)]
pub enum Token {
    UcIdent(ArcStr), // +
    LcIdent(ArcStr), // +
    Special(Special), //+
    Literal(Literal),
    OperatorSign(OperatorSign), // +
    BraceRight(BraceKind), // +
    BraceLeft(BraceKind), // +
}

impl Token {
    // yet to fill
    fn from_single_char(ch: char) -> Option<Self> {
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

impl Token {
    pub fn is_ident(&self) -> bool {
        match self {
            Token::UcIdent(_) | Token::LcIdent(_) => true,
            _ => false
        }
    }

    pub fn from_input(input: &str) -> rc::Gen<Token,(), impl Future<Output = ()> + '_> {
        let parse_word = |window: &[char]| -> (ArcStr,usize) {
            let mut s = String::new();
            for (i,ch) in window.iter().enumerate() {
                if !(ch.is_alphabetic()) {
                    return (s.into(),i+1)
                };
                s.push(*ch)
            };
            let len = s.len();
            (s.into(),len - 1)
        };

        let lit_predicate =|ch: char| {
            match ch {
                '-' => true,
                '0'..='9' => true,
                '\"' => true,
                't' | 'f' => true,
                _ => false
            }
        };

        let parse_lit = |window: &[char]| -> (Literal,usize) {
            match window[0] {
                '"' => {
                    //todo: support escape char
                    let mut s = String::new();
                    for i in &window[1..] {
                        if *i == '"' {
                            let length =  s.len()+1;
                            return (Literal::String(s.into()),length)
                        };
                        s.push(*i);
                    };
                    panic!("Unclosed Literal");
                },
                '0'..='9' => {
                    let mut before_dot = 0i64;
                    let mut dot_position = None;
                    let mut after_dot = 0f64;

                    let mut iter = window.iter().enumerate();
                    'before_dot: while let Some((offset,ch)) = iter.next() {
                        match ch {
                            '0'..='9' => before_dot = before_dot * 10 + ch.to_digit(10).unwrap() as i64,
                            '.' => {
                                dot_position = Some(offset);
                                break 'before_dot
                            },
                            _ => return (Literal::Integer(before_dot), offset)
                        }
                    };
                    while let Some((offset,ch)) = iter.next() {
                        match ch {
                            '0'..='9' => after_dot = after_dot + (f64::powf(-10.0, (offset - dot_position.unwrap()) as f64)) * (ch.to_digit(10).unwrap() as f64),
                            _ => return (Literal::Float(before_dot as f64 + after_dot), offset)
                        }
                    };
                    match dot_position {
                        Some(_) => {
                            (Literal::Float(before_dot as f64 + after_dot), window.len())
                        },
                        None => {
                            (Literal::Integer(before_dot),window.len())
                        }
                    }
                },
                _ => {
                    match &window[..] {
                        ['t','r','u','e', ..] => return (Literal::Boolean(true),4),
                        ['f','a','l','s','e', ..] => return (Literal::Boolean(false),5),
                        _ => unreachable!()
                    }
                }
            }
        };

        rc::gen!({
            let mut chars: Vec<char> = input.chars().collect();
            let mut offset = 0;

            loop {
                let window = &chars[offset..];
                match window {
                    [w,..] if lit_predicate(*w) => {
                        let (lit,moved) = parse_lit(window);
                        offset += moved;
                        yield_!(Token::Literal(lit))
                    },
                    [w,..] if w.is_ascii_lowercase() && w.is_alphabetic()=> {
                        let (s,moved) = parse_word(window);
                        offset += moved;
                        yield_!(Token::LcIdent(ArcStr::from(s)))
                    },
                    [w,..] if w.is_ascii_uppercase() && w.is_alphabetic() => {
                        let (s,moved) = parse_word(window);
                        offset += moved;
                        yield_!(Token::UcIdent(ArcStr::from(s)))
                    },
                    [ch,..] if Self::from_single_char(*ch).is_some() => {
                        let tok = Self::from_single_char(*ch).unwrap();
                        offset += 1;
                        yield_!(tok);
                    },
                    [' ',..] => offset += 1,
                    [] => break,
                    _ => {
                        log::error!("omfg");
                        break
                    },
                }
            }
        })
    }
}


pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
    log::info!("lexing the {}", input);
    Token::from_input(input).into_iter()
}