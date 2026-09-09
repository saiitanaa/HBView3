pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub body: Vec<Statement>,
}

pub enum Statement {
    Call {
        name: String,
        arguments: Vec<Expression>,
    },
    Variable {
        name: String,
        value: Expression,
    },
    Return,
}

pub enum Expression {
    String(String),
    Integer(i64),
    Identifier(String),
}