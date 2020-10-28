use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// use chrono::{Date, TimeZone, Utc};
use chrono::naive::{NaiveDate, NaiveTime};

fn main() {
    let mut block_date: Option<NaiveDate> = None;

    for ln in read_lines("data.txt").expect("data.txt not found") {
        if let Ok(text) = ln {
            if let Some(current_date) = block_date {
                if text.is_empty() {
                    block_date = None;
                    continue;
                }
                
                if let Ok(time) = NaiveTime::parse_from_str(&text[..7], "[%H:%M]") {   
                    println!("datetime: {}", current_date.and_time(time).format("%F %H:%M"));
                }
            } else {
                println!("no block here");
                if text.is_empty() {
                    continue;
                }
                if let Ok(date) = NaiveDate::parse_from_str(&text, "%F") {
                    block_date = Some(date);
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
