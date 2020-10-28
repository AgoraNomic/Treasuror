use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::Peekable;
//use std::collections::HashMap;

// use chrono::{Date, TimeZone, Utc};
use chrono::naive::{NaiveDate, NaiveTime};

fn main() {
    let mut block_date: Option<NaiveDate> = None;
//    let mut entities: HashMap<&str, HashMap<&str, u32>> = 

    for ln in read_lines("data.txt").expect("data.txt not found") {
        if let Ok(text) = ln {
            if let Some(current_date) = block_date {
                if text.is_empty() {
                    block_date = None;
                    continue;
                }

                let mut words = text.split_whitespace().peekable();
                let mut time_str = words.peek().unwrap();
                
                let time = match NaiveTime::parse_from_str(time_str, "[%R]") {
                    Ok(t) => {
                        words.next();
                        t
                    },
                    Err(_) => NaiveTime::from_hms(0, 0, 0),
                };
                
                println!("{} {}", time.format("[%R]"), words.collect::<Vec<&str>>().join("  "));
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
