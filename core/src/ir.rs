use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Statement {
    #[serde(rename = "call")]
    Call {
        name: String,
        arguments: Vec<Expression>,
    },

    #[serde(rename = "variable")]
    Variable {
        name: String,
        value: Expression,
    },

    #[serde(rename = "return")]
    Return,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    #[serde(rename = "integer")]
    Integer {
        value: i64,
    },

    #[serde(rename = "identifier")]
    Identifier {
        name: String,
    },

    #[serde(rename = "string")]
    String {
        value: String,
    },
}