use chrono::naive::NaiveDate;

use crate::{
    model::{Currency, Entity},
    parser::{
        common::{token_com::*, TokenIterator},
        error::{
            any::AnyError,
            syntax::{ErrorKind, SyntaxError},
        },
    },
};

pub enum Directive {
    Assets(Vec<Currency>),
    Date(NaiveDate),
    Entity(Entity),
    Flotation(f32),
    Forbes(u32),
}

impl Directive {
    pub fn with_source(ln: &str) -> Result<Directive, AnyError<&str>> {
        if ln.trim().is_empty() {
            return Err(AnyError::Syntax(SyntaxError {
                message: "".to_string(),
                kind: ErrorKind::Empty,
            }));
        }

        let mut tokens = Vec::new();

        for tr in TokenIterator::with_source(ln) {
            tokens.push(tr?);
        }

        let s = expect_identifier(&mut tokens, "need an identifier to begin an identifier")?;

        match &s.to_lowercase()[..] {
            "assets" => {
                let mut result = Vec::new();

                while !tokens.is_empty() {
                    result.push(try_into_currency(&expect_identifier(
                        &mut tokens,
                        "parameters to CURRENCY must be identifiers",
                    )?)?)
                }

                Ok(Directive::Assets(result))
            }
            "flotation" => Ok(Directive::Flotation(expect_float(
                &mut tokens,
                "FLOTATION requires a float argument",
            )?)),
            "date" => Ok(Directive::Date(expect_date(
                &mut tokens,
                "DATE requires a date argument",
            )?)),
            "ent" => Ok(Directive::Entity(parse(&mut tokens)?)),
            "forbes" => Ok(Directive::Forbes(expect_integer(
                &mut tokens,
                "FORBES requires an integer argunent",
            )?)),
            s => Err(SyntaxError::from(
                &format!("no such directive: {}", s),
                ErrorKind::UnrecognizedDirective,
            )
            .into()),
        }
    }
}
