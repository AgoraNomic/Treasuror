use crate::{
    model::Currency,
    parser::{
        common::{token_com::*, Token},
        error::syntax::{ErrorKind, SyntaxError, SyntaxResult},
    }
};

use super::Transaction;

#[derive(Clone)]
pub enum Command {
    Activate(String),
    BuoyancyTarget(u32),
    Deactivate(String),
    Deregister(String),
    Message(String),
    NewContract(String, String),
    NewPlayer(String, String),
    NoRecord(Box<Command>),
    Nuke,
    Payday,
    Relevel(Option<u32>),
    Rename(Currency, Currency),
    Report,
    Revision,
    Transaction(Transaction),
}

impl Command {
    pub fn from_name_and_vec(name: String, mut tokens: Vec<Token>) -> SyntaxResult<Command> {
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
            "message" | "msg" => {
                Ok(Command::Message(expect_stringlike(&mut tokens, "expected string for message")?))
            }
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
            "norecord" | "nr" => {
                let next_name = 
                    expect_identifier(&mut tokens, "expected subcommand in #norecord")?;

                Ok(Command::NoRecord(Box::new(Command::from_name_and_vec(next_name, tokens)?)))
            }
            "nuke" => Ok(Command::Nuke),
            "payday" => Ok(Command::Payday),
            "relevel" => Ok(Command::Relevel(expect_integer(&mut tokens, "").ok())),
            "rename" => {
                let first =
                    try_into_currency(&expect_identifier(&mut tokens, "expected identifier in #rename")?)?;
                let second =
                    try_into_currency(&expect_identifier(&mut tokens, "expected identifier in #rename")?)?;

                Ok(Command::Rename(first, second))
            }
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
