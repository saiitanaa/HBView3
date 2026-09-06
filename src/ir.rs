pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub body: Vec<Statement>,
}

pub enum Statement {
    Call(String),
    Return,
}
