use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use chrono::{Date, TimeZone, Utc};

fn main() {
    let mut block_date: Option<Date<Utc>> = None;

    for ln in read_lines("data.txt").expect("data.txt not found") {
        if let Ok(text) = ln {
            if let Some(current_date) = block_date {
                if text.is_empty() {
                    block_date = None;
                    continue;
                }
                
                if text.starts_with("[") && text[..7].ends_with("]") { 
                    let hour: u32 = text[1..3].parse().expect("invalid hour");
                    let minute: u32 = text[4..6].parse().expect("invalid minute");
                    let date_and_time = current_date.and_hms(hour, minute, 0);

                    println!("datetime: {}", date_and_time.format("%F %H:%M"));
                }
            } else {
                println!("no block here");
                if text.is_empty() {
                    continue;
                }
                block_date = extract_date(&text);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_date(s: &str) -> Option<Date<Utc>> {
    let mut split = s.split('-');
    let year = split.next();
    let month = split.next();
    let day = split.next();
    
    if year.is_some() && month.is_some() && day.is_some() {
        let year: i32 = year.unwrap().parse().expect("invalid year");
        let month: u32 = month.unwrap().parse().expect("invalid month");
        let day: u32 = day.unwrap().parse().expect("invalid day");
        Some(Utc.ymd(year, month, day))
    } else {
        None
    }
}
