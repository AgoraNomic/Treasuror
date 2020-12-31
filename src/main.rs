use chrono::naive::MIN_DATE;

pub mod model;
pub mod parser;

use model::Context;
use parser::{Operator, Parser, Statement};

fn main() {
    let mut context = Context::new();

    let mut date = MIN_DATE;

    let mut parser = Parser::from_filename("data.txt").expect("data.txt not found");

    while let Some(lo) = parser.next_raw() {
        if lo.datetime().date() != date {
            date = lo.datetime().date();
            println!("\n *** {}", date.format("%a %-d %B %Y"));
        }

        match lo.action() {
            Statement::Transaction(t) => {
                context.apply(t);
                for w in t.expand() {
                    let actstr = match w.operator() {
                        Operator::Plus => String::from("+"),
                        Operator::Minus => String::from("-"),
                        _ => String::from("what?"),
                    };

                    println!(
                        "{} {}: {} {} ({})",
                        lo.datetime().format("[%R]"),
                        w.agent(),
                        actstr,
                        w.amount().pretty(),
                        w.comment()
                    )
                }
            }
            Statement::Command(_) => println!("a command occured here"),
        }
    }

    println!("{}", context.display());
}
