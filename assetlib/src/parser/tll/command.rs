use crate::parser::common::{token_com::*, Token};

use super::{error::*, Transaction};

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
    pub fn from_name_and_vec(name: String, mut tokens: Vec<Token>) -> Result<Command, SyntaxError> {
        match &name.to_lowercase()[..] {
            "activate" => Ok(Command::Activate(expect_stringlike(
                &mut tokens,
                "cannot activate no one",
            )?)),
            "bt" => Ok(Command::BuoyancyTarget(expect_integer(
                &mut tokens,
                "setting buoyancy target requires an integer",
            )?)),
            "deactivate" => Ok(Command::Deactivate(expect_stringlike(
                &mut tokens,
                "cannot deactivate no one",
            )?)),
            "delplayer" | "delcontract" | "deregister" => Ok(Command::Deregister(
                expect_identifier(&mut tokens, "expected identifier in #deregister")?,
            )),
            "payday" => Ok(Command::Payday),
            "newcontract" => {
                let identifier =
                    expect_identifier(&mut tokens, "expected identifier in #newcontract")?;
                let full_name =
                    expect_stringlike(&mut tokens, "").unwrap_or_else(|_| identifier.clone());

                Ok(Command::NewContract(identifier, full_name))
            }
            "newplayer" | "register" => {
                let identifier =
                    expect_identifier(&mut tokens, "expected identifier in #newplayer")?;
                let full_name =
                    expect_stringlike(&mut tokens, "").unwrap_or_else(|_| identifier.clone());

                Ok(Command::NewPlayer(identifier, full_name))
            }
            "nuke" => Ok(Command::Nuke),
            "relevel" => Ok(Command::Relevel(expect_integer(&mut tokens, "").ok())),
            "report" => Ok(Command::Report),
            "revision" => Ok(Command::Revision),
            "transaction" | "t" => Ok(Command::Transaction(Transaction::from_vec(tokens)?)),
            _ => Err(SyntaxError::from(
                &format!("no such command: {}", name),
                ErrorKind::UnrecognizedCommand,
            )),
        }
    }
}
