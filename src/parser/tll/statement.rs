use crate::parser::{ast::Token, tll::Transaction};

#[derive(Clone)]
pub enum Statement {
    Transaction(Transaction),
    Command(Command),
}

impl Statement {
    pub fn from_vec(tokens: Vec<Token>) -> Option<Statement> {
        match tokens[0].clone() {
            Token::Identifier(_) => {
                Transaction::from_vec(tokens).map(|t| Statement::Transaction(t))
            }
            Token::Command(c) => Some(Statement::Command(Command {
                cmd: String::from(c),
                args: tokens,
            })),
            _ => None,
        }
    }

    pub fn transaction(&self) -> Option<Transaction> {
        if let Statement::Transaction(t) = self {
            Some(t.clone())
        } else {
            None
        }
    }

    pub fn command(&self) -> Option<Command> {
        if let Statement::Command(c) = self {
            Some(c.clone())
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Command {
    cmd: String,
    args: Vec<Token>,
}

impl Command {
    pub fn command(&self) -> &str {
        &self.cmd
    }

    pub fn args(&self) -> &Vec<Token> {
        &self.args
    }
}
