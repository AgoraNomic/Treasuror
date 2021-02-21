use crate::{match_first_pop, parser::ast::Token};

#[derive(Clone)]
pub enum Command {
    Relevel(Option<u32>),
    Report,
    NewPlayer(String, String),
    Deregister(String),
    Nuke,
    Payday,
}

impl Command {
    pub fn from_name_and_vec(name: String, mut tokens: Vec<Token>) -> Option<Command> {
        match &name.to_lowercase()[..] {
            "relevel" => Some(Command::Relevel(match_first_pop!(tokens {
                    Token::Integer(i) => { Some(i) },
                } else { None }))),
            "report" => Some(Command::Report),
            "nuke" => Some(Command::Nuke),
            "payday" => Some(Command::Payday),
            "newplayer" => {
                let identifier = match_first_pop!(tokens {
                    Token::Identifier(s) => { s },
                } else { panic!("expected identifier in #newplayer command") });

                let full_name = match_first_pop!(tokens {
                    Token::String(s) => { s },
                    Token::Identifier(s) => { s },
                } else { identifier.clone() });

                Some(Command::NewPlayer(identifier, full_name))
            }
            "deregister" => Some(Command::Deregister(match_first_pop!(tokens {
                    Token::Identifier(s) => { s },
                } else { panic!("expected identifier in #deregister command") }))),
            _ => {
                eprintln!("no such command: {}", name);
                None
            }
        }
    }
}
