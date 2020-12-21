use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;

use chrono::naive::NaiveDate;

pub mod parser;
use parser::Line;
use parser::Statement;
use parser::Operator;

fn main() {
    let mut block_date: Option<NaiveDate> = None;
//    let mut entities: HashMap<&str, HashMap<&str, u32>> = 

    for ln in read_lines("data.txt").expect("data.txt not found") {
        if let Ok(mut text) = ln {
            if let Some(current_date) = block_date {
                // i'm not sure why but token production only works if there is
                // a whitespace at the end. i tried to find a workaround but
                // i'm too tired for this so here you go.
                text.push('\n');
                let t = match Line::with_date_from_str(&current_date, &mut text) {
                    Some(tr) => tr,
                    None => {
                        block_date = None;
                        continue;
                    }
                };

                match t.get_action() {
                    Statement::Transaction {..} => {
                        for w in t.expand() {
                            let act = w.get_action();
                            let actstr = match act.get_operator() {
                                Some(Operator::Plus) => String::from("+"),
                                Some(Operator::Minus) => String::from("-"),
                                Some(Operator::Transfer(m)) => String::from(">") + m,
                                None => String::from("command"),
                            };

                            println!(
                                "{} {}: {} {} ({})",
                                w.get_datetime().format("[%R]"),
                                act.get_agent().unwrap(),
                                actstr,
                                act.get_amount().unwrap().pretty(),
                                act.get_comment().unwrap()
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
