use crate::{
    match_first_pop,
    model::{Currency, Entity},
    parser::{
        error::*,
        common::{Token, TokenIterator},
        tll::error::*,
    }
};

pub enum Directive {
    Assets(Vec<Currency>),
    Entity(Entity),
    Flotation(f32),
    Forbes(u32),
}

impl Directive {
    pub fn with_source(ln: &str) -> Result<Directive, AnyError<&str>> {
        if ln.trim().is_empty() {
            return Err(AnyError::Syntax(
                SyntaxError { message: "".to_string(), kind: ErrorKind::Empty }
            ));
        }

        let mut tokens = Vec::new();

        for tr in TokenIterator::with_source(ln) {
            tokens.push(tr?);
        }

        Ok(match_first_pop!(tokens {
            Token::Identifier(s) => { match &s.to_lowercase()[..] {
                "assets" => Directive::Assets(
                    tokens
                        .iter()
                        .map(|x| Currency::from_abbr(
                            x.extract_string()).expect("no such currency")
                        )
                        .collect::<Vec<Currency>>()
                ),
                "flotation" => Directive::Flotation(
                    tokens[0].extract_float()
                ),
                "ent" => Directive::Entity(Entity::from_vec(&mut tokens)),
                "forbes" => Directive::Forbes(tokens[0].extract_int()),
                s => panic!("No such directive: {}", s),
            }},
        } else { panic!("directive must start with identifier") }))
    }
}
