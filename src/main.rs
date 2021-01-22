use std::fs::File;
use std::io::BufReader;

use chrono::naive::MIN_DATE;

pub mod model;
pub mod parser;

use model::Context;
use parser::{
    ast::Operator,
    gsdl::{Parser as GsdParser},
    tll::{Parser as TlParser, Statement},
};

fn main() {
    let mut context = Context::new();

    let mut date = MIN_DATE;

    let mut tlparser = TlParser::from_reader(BufReader::new(
        File::open("data.txt").expect("data.txt not found"),
    ));

    let mut gsdparser = GsdParser::from_reader(BufReader::new(
        File::open("state.txt").expect("state.txt not found"),
    ));

    while let Some(d) = gsdparser.next_raw() {
        context.process(&d);
    }

    while let Some(lo) = tlparser.next_raw() {
        if lo.datetime().date() != date {
            date = lo.datetime().date();
            println!("\n *** {}", date.format("%a %-d %B %Y"));
        }

        match lo.action() {
            Statement::Transaction(t) => {
                context.apply(&lo);
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
            Statement::Command(c) => context.exec(c),
        }
    }

    println!("{}", context.display());
}
