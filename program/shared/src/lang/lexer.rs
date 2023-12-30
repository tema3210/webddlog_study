use crate::lexer_types::Token;

pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
    log::info!("lexing the {}", input);
    
    [].into_iter()
}