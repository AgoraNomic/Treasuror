use std::io::BufReader;
use std::fs::File;

use chrono::naive::MIN_DATE;

pub mod parser;
use parser::{Statement, Operator, Parser};

fn main() {
    let mut date = MIN_DATE;

    let mut parser = Parser::from_filename("data.txt").expect("data.txt not found");

    while let Some(lo) = parser.next_raw() {
        if lo.datetime().date() != date {
            date = lo.datetime().date();
            println!("\n *** {}", date.format("%a %-d %B %Y"));
        }

        match lo.action() {
            Statement::Transaction(t) => {
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
            },
            Statement::Command {..} => println!("a command occured here"),
        }
    }
}
