use crate::parser::tll::Transaction;
use crate::{match_first_pop, parser::common::Token};

#[derive(Clone)]
pub enum Command {
    Activate(String),
    BuoyancyTarget(u32),
    Deactivate(String),
    Deregister(String),
    NewContract(String, String),
    NewPlayer(String, String),
    Nuke,
    Payday,
    Relevel(Option<u32>),
    Report,
    Revision,
    Transaction(Transaction),
}

impl Command {
    pub fn from_name_and_vec(name: String, mut tokens: Vec<Token>) -> Option<Command> {
        match &name.to_lowercase()[..] {
            "activate" => Some(Command::Activate(match_first_pop!(tokens {
                    Token::String(s) => { s },
                    Token::Identifier(s) => { s },
                } else { panic!("cannot activate no one") }))),
            "bt" => Some(Command::BuoyancyTarget(match_first_pop!(tokens {
                    Token::Integer(i) => { i },
                } else { panic!("setting buoyancy target requires an integer") }))),
            "deactivate" => Some(Command::Deactivate(match_first_pop!(tokens {
                    Token::String(s) => { s },
                    Token::Identifier(s) => { s },
                } else { panic!("cannot deactivate no one") }))),
            "delplayer" | "delcontract" | "deregister" => {
                Some(Command::Deregister(match_first_pop!(tokens {
                    Token::Identifier(s) => { s },
                } else { panic!("expected identifier in #deregister command") })))
            }
            "payday" => Some(Command::Payday),
            "newcontract" => {
                let identifier = match_first_pop!(tokens {
                    Token::Identifier(s) => { s },
                } else { panic!("expected identifier in #newcontract command") });

                let full_name = match_first_pop!(tokens {
                    Token::String(s) => { s },
                    Token::Identifier(s) => { s },
                } else { identifier.clone() });

                Some(Command::NewContract(identifier, full_name))
            }
            "newplayer" | "register" => {
                let identifier = match_first_pop!(tokens {
                    Token::Identifier(s) => { s },
                } else { panic!("expected identifier in #newplayer command") });

                let full_name = match_first_pop!(tokens {
                    Token::String(s) => { s },
                    Token::Identifier(s) => { s },
                } else { identifier.clone() });

                Some(Command::NewPlayer(identifier, full_name))
            }
            "nuke" => Some(Command::Nuke),
            "relevel" => Some(Command::Relevel(match_first_pop!(tokens {
                    Token::Integer(i) => { Some(i) },
                } else { None }))),
            "report" => Some(Command::Report),
            "revision" => Some(Command::Revision),
            "transaction" | "t" => Some(Command::Transaction(
                Transaction::from_vec(tokens).expect("no transaction specified"),
            )),
            _ => {
                eprintln!("no such command: {}", name);
                None
            }
        }
    }
}
