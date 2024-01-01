use std::future::Future;

use arcstr::ArcStr;
use genawaiter::{rc,yield_};
use shared::lexer_types::*;


fn parse_lit(window: &[char]) -> (Option<Literal>,usize) {
    log::debug!("get next literal from {:?}", &window);
    match window[0] {
        '"' => {
            //todo: support escape char
            let mut s = String::new();
            for i in &window[1..] {
                if *i == '"' {
                    let length =  s.len()+1;
                    return (Some(Literal::String(s.into())),length)
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
                // log::debug!("before dot: {}, {:?}, {}",before_dot,dot_position,after_dot);
                match ch {
                    '0'..='9' => before_dot = before_dot * 10 + ch.to_digit(10).unwrap() as i64,
                    '.' => {
                        dot_position = Some(offset);
                        break 'before_dot
                    },
                    _ => return (Some(Literal::Integer(before_dot)), offset)
                }
            };
            while let Some((offset,ch)) = iter.next() {
                let factor = (offset - dot_position.unwrap()) as f64;
                // log::debug!("after dot: {}, {:?}, {}, {}",before_dot,dot_position,after_dot, factor);
                match ch {
                    '0'..='9' => after_dot = after_dot + (f64::powf(0.1, factor)) * (ch.to_digit(10).unwrap() as f64),
                    _ => return (Some(Literal::Float(before_dot as f64 + after_dot)), offset)
                }
            };
            match dot_position {
                Some(_) => {
                    (Some(Literal::Float(before_dot as f64 + after_dot)), window.len())
                },
                None => {
                    (Some(Literal::Integer(before_dot)),window.len())
                }
            }
        },
        _ => {
            match &window[..] {
                ['t','r','u','e', ..] => return (Some(Literal::Boolean(true)),4),
                ['f','a','l','s','e', ..] => return (Some(Literal::Boolean(false)),5),
                _ => unreachable!()
            }
        }
    }
}

fn parse_word(window: &[char]) -> (ArcStr,usize) {
    log::debug!("get next word from {:?}", window);
    let mut s = String::new();
    for (i,ch) in window.iter().enumerate() {
        if !(ch.is_alphabetic() || *ch == '_') {
            return (s.into(),i)
        };
        s.push(*ch)
    };
    let len = s.len();
    (s.into(),len)
}

fn from_input(input: &str) -> rc::Gen<Token,(), impl Future<Output = ()> + '_> {
    
    let lit_predicate =|ch: char| {
        match ch {
            '0'..='9' => true,
            '\"' => true,
            't' | 'f' => true,
            _ => false
        }
    };

    rc::gen!({
        let mut chars: Vec<char> = input.chars().collect();
        let mut offset = 0;

        loop {
            let window = &chars[offset..];
            match window {
                ['-',dig,..] if ('0'..='9').contains(dig) => {
                    if let (Some(lit),moved) = parse_lit(&window[1..]) {
                        offset += moved + 1;
                        match lit {
                            Literal::Integer(i) => yield_!(Token::Literal(Literal::Integer(-i))),
                            Literal::Float(f) => yield_!(Token::Literal(Literal::Float(-f))),
                            _ => panic!("oh no"),
                        }
                    }
                }
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
                [w,..] if lit_predicate(*w) => {
                    if let (Some(lit),moved) = parse_lit(window) {
                        offset += moved;
                        yield_!(Token::Literal(lit))
                    }
                },
                [ch,..] if Token::from_single_char(*ch).is_some() => {
                    let tok = Token::from_single_char(*ch).unwrap();
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


pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
    log::info!("lexing the {}", input);
    from_input(input).into_iter()
}