use crate::{lexer_types::Token, parser_types::{AstNode, TypeDef}};

fn parse_next_type_def(content: &[Token]) -> (TypeDef,usize) {
    unimplemented!()
}

pub fn parse(mut toks: impl Iterator<Item = Token>) -> AstNode {
    use Token::*;
    let mut program = Vec::new();

    // windowing
    let tokens: Vec<_> = toks.collect();
    let mut offset = 0;
    loop {
        match &tokens[offset..] {
            [LcIdent(ident), rest @ ..] if ident == "typedef" => {
                let (def,step) = parse_next_type_def(rest);
                program.push(AstNode::TypeDef(def));
                offset += step;
            },
            
            [] => break,
            _ => unimplemented!(),
        }
    };

    AstNode::Many(program)
}