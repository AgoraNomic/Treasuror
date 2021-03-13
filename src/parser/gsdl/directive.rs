use crate::{
    match_first_pop,
    model::Entity,
    parser::ast::{Currency, Token, TokenIterator},
};

pub enum Directive {
    Assets(Vec<Currency>),
    Entity(Entity),
    Flotation(f32),
    Forbes(u32),
}

impl Directive {
    pub fn with_source(ln: &str) -> Option<Directive> {
        if ln.trim().is_empty() {
            return None;
        }
        let mut tokens: Vec<Token> = TokenIterator::with_source(ln).collect();

        Some(match_first_pop!(tokens {
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
