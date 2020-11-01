use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;

use chrono::naive::NaiveDate;

mod transaction;
use transaction::Transaction;

pub mod token;
use token::Operator;

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

                let actstr = match t.get_action() {
                    Operator::Plus => "+",
                    Operator::Minus => "-",
                    Operator::Transfer(_) => ">",
                };

                println!("{} {} {} {} {}", t.get_datetime().format("[%R]"), t.get_agent(), t.get_amount().pretty(), actstr, t.get_comment());
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
