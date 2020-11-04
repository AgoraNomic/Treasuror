use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;

use chrono::naive::NaiveDate;

pub mod parser;
use parser::transaction::Transaction;
use parser::ast::operator::Operator;

fn main() {
    let mut block_date: Option<NaiveDate> = None;
//    let mut entities: HashMap<&str, HashMap<&str, u32>> = 

    for ln in read_lines("data.txt").expect("data.txt not found") {
        if let Ok(mut text) = ln {
            if let Some(current_date) = block_date {
                text.push('\n');
                let t = match Transaction::with_date_from_str(&current_date, &mut text) {
                    Some(tr) => tr,
                    None => {
                        block_date = None;
                        continue;
                    }
                };

                for w in t.expand_transfer() {
                    let actstr = match w.get_action() {
                        Operator::Plus => String::from("+"),
                        Operator::Minus => String::from("-"),
                        Operator::Transfer(w) => String::from(">") + w,
                    };

                    println!(
                        "{} {}: {} {} ({})",
                        w.get_datetime().format("[%R]"),
                        w.get_agent(),
                        actstr,
                        w.get_amount().pretty(),
                        w.get_comment()
                        );
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
