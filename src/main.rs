use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use chrono::naive::NaiveDate;

pub mod parser;
use parser::Line;
use parser::Statement;
use parser::Operator;

fn main() {
    let mut block_date: Option<NaiveDate> = None;

    for ln in read_lines("data.txt").expect("data.txt not found") {
        if let Ok(mut text) = ln {
            if let Some(current_date) = block_date {
                // i'm not sure why but token production only works if there is
                // a whitespace at the end. i tried to find a workaround but
                // i'm too tired for this so here you go.
                text.push('\n');
                let lo = match Line::with_date_from_str(&current_date, &mut text) {
                    Some(tr) => tr,
                    None => {
                        block_date = None;
                        continue;
                    }
                };

                match lo.get_action() {
                    Statement::Transaction(t) => {
                        for w in t.expand() {
                            let actstr = match w.get_operator() {
                                Operator::Plus => String::from("+"),
                                Operator::Minus => String::from("-"),
                                Operator::Transfer(m) => String::from(">") + m,
                            };

                            println!(
                                "{} {}: {} {} ({})",
                                lo.get_datetime().format("[%R]"),
                                w.get_agent(),
                                actstr,
                                w.get_amount().pretty(),
                                w.get_comment()
                                )
                        }
                    },
                    Statement::Command {..} => println!("a command occured here"),
                }
            } else {
                if text.is_empty() {
                    continue;
                }
                if let Ok(date) = NaiveDate::parse_from_str(&text, "%F") {
                    block_date = Some(date);
                    println!("\n *** {}", date.format("%a %-d %B %Y"));
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
