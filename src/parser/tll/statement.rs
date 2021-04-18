use crate::parser::{
    common::Token,
    tll::{Command, Transaction},
};

#[derive(Clone)]
pub enum Statement {
    Transaction(Transaction),
    Command(Command),
}

impl Statement {
    pub fn from_vec(mut tokens: Vec<Token>) -> Option<Statement> {
        match tokens[0].clone() {
            Token::Identifier(_) => Transaction::from_vec(tokens).map(Statement::Transaction),
            Token::Command(s) => {
                tokens.remove(0);
                Command::from_name_and_vec(s, tokens).map(Statement::Command)
            }
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
