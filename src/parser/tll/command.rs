use crate::{match_first_pop, parser::ast::Token};

#[derive(Clone)]
pub enum Command {
    Relevel,
    Report,
    NewPlayer(String, String),
    Nuke,
}

impl Command {
    pub fn from_name_and_vec(name: String, mut tokens: Vec<Token>) -> Option<Command> {
        match &name.to_lowercase()[..] {
            "relevel" => Some(Command::Relevel),
            "report" => Some(Command::Report),
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
            "nuke" => Some(Command::Nuke),
            _ => {
                eprintln!("no such command: {}", name);
                None
            }
        }
    }
}
