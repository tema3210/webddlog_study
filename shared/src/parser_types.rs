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