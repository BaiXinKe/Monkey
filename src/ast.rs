use std::rc::Rc;

use crate::token::Token;

// trait sets
pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

// --------------------------------------------------------------
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        match self.statements.first() {
            Some(first) => first.token_literal(),
            None => "".to_string(),
        }
    }
}

// ------------------------------------------------------------------
pub struct LetStatement {
    pub token: Token,
    pub name: Rc<Identifier>,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
