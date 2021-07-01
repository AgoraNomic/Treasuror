use std::env::args;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use chrono::{
    naive::{NaiveDate, NaiveTime},
    Utc,
};

use treasuror::{
    model::{Context, Currency, Report},
    parser::{gsdl::Parser as GsdParser, tll::Parser as TlParser}
};

fn main() -> io::Result<()> {
    let args = args().collect::<Vec<String>>();

    let mut context = if args.len() == 1 {
        eprintln!(
            "no argument given; generating report as of {} UTC",
            Utc::now().format("%c")
        );
        Context::from_datetime(Utc::now().naive_utc())
    } else if args.len() > 1 {
        Context::from_datetime(
            NaiveDate::parse_from_str(&(args[1])[..], "%F")
                .unwrap_or_else(|_| panic!("invalid date format: {}", args[1]))
                .and_time(if args.len() == 2 {
                    NaiveTime::from_hms(0, 0, 0)
                } else {
                    NaiveTime::parse_from_str(&(args[2])[..], "%R")
                        .unwrap_or_else(|_| panic!("invalid time format: {}", args[2]))
                }),
        )
    } else {
        panic!("unknown number of args: {}", args.len())
    };

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
        context.enter(lo);
    }

    let mut format = String::new();
    File::open("format.txt")?.read_to_string(&mut format)?;

    let mut f = File::create("out.txt")?;
    f.write_all(
        Report::with_context(&mut context)
            .format(&format)
            .as_bytes(),
    )?;

    eprintln!(
        "total coins upon completion: {}",
        context.currency_total(Currency::Coin)
    );
    Ok(())
}
