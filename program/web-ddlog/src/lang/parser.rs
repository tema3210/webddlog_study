use super::lexer::Token;

pub enum TypeSpec {
    //Primitives
    Boolean,
    String,
    BitVec,
    Integer,
    Float,
    //UDTs
    Tuple(Vec<TypeSpec>),
    Union(Vec<TypeSpec>),
    TypeVar(String),
    Func(Box<TypeSpec>,Box<TypeSpec>)
}


pub enum TypeDef {
    Tuple {
        name: String,
        generics: Vec<String>,
        members: Vec<TypeSpec>
    },
    Union {
        name: String,
        generics: Vec<String>,
        members: Vec<TypeSpec>
    },
}

pub struct Expression();

pub enum AstNode {
    Many(Vec<AstNode>),
    TypeDef(TypeDef),
    Expression(Expression),
    Function{
        name: String,
        type_: TypeSpec
    }
}

impl Default for AstNode {
    fn default() -> Self {
        AstNode::Many(vec![])
    }
}

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