use crate::match_first_pop;
use crate::parser::ast::{Amount, Operator, Token};

#[derive(Clone)]
pub enum Statement {
    Transaction(Transaction),
    Command(Command),
}

impl Statement {
    pub fn from_vec(mut tokens: Vec<Token>) -> Option<Statement> {
        match tokens[0].clone() {
            Token::Identifier(i) => Some(Statement::Transaction(Transaction {
                agent: match_first_pop!(tokens {
                    Token::Identifier(_) => { String::from(i) },
                } else { return None }),
                amount: Amount::from_vec(&mut tokens),
                operator: match_first_pop!(tokens {
                    Token::Op(o) => { o },
                } else { return None }),
                comment: match_first_pop!(tokens {
                    Token::String(s) => { String::from(s) },
                } else { String::from("") }),
            })),
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
pub struct Transaction {
    agent: String,
    amount: Amount,
    operator: Operator,
    comment: String,
}

impl Transaction {
    pub fn expand(&self) -> Vec<Transaction> {
        match &self.operator {
            Operator::Transfer(s) => vec![
                Transaction {
                    agent: self.agent.clone(),
                    amount: self.amount,
                    operator: Operator::Minus,
                    comment: format!(
                        "Transfer {}{}",
                        s,
                        if self.comment == "" {
                            String::from("")
                        } else {
                            String::from(": ") + &self.comment
                        }
                    ),
                },
                Transaction {
                    agent: String::from(s),
                    amount: self.amount,
                    operator: Operator::Plus,
                    comment: format!(
                        "Transfer {}{}",
                        self.agent.clone(),
                        if self.comment == "" {
                            String::from("")
                        } else {
                            String::from(": ") + &self.comment
                        }
                    ),
                },
            ],
            _ => vec![self.clone()],
        }
    }

    pub fn agent(&self) -> &str {
        &self.agent
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }

    pub fn operator(&self) -> Operator {
        self.operator.clone()
    }

    pub fn comment(&self) -> &str {
        &self.comment
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
