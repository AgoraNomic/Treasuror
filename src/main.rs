use std::env::args;
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};

use chrono::{
    naive::{NaiveDate, NaiveTime},
    Utc,
};

use assetlib::{
    model::{Context, Currency, Report},
    parser::{gsdl::Parser as GsdParser, tll::Parser as TlParser},
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
        let date = NaiveDate::parse_from_str(&(args[1])[..], "%F")
            .unwrap_or_else(|_| panic!("invalid date format: {}", args[1]));

        let time = if args.len() == 2 {
            NaiveTime::from_hms(0, 0, 0)
        } else {
            NaiveTime::parse_from_str(&(args[2])[..], "%R")
                .unwrap_or_else(|_| panic!("invalid time format: {}", args[2]))
        };

        Context::from_datetime(date.and_time(time))
    } else {
        panic!("unknown number of args: {}", args.len())
    };

    let mut gsdparser = GsdParser::from_reader(BufReader::new(
        File::open("data/state.txt").expect("data/state.txt not found"),
    ));

    while let Some(d) = gsdparser.next_raw() {
        context.process(&d);
    }

    let mut files = fs::read_dir("data/tll")?
        .map(|f| f.unwrap())
        .collect::<Vec<_>>();
    files.sort_by_key(|f| f.file_name());

    for f in files.iter() {
        let mut tlparser = TlParser::from_reader(BufReader::new(
            File::open(f.path())
                .unwrap_or_else(|_| panic!("could not open tll file {:?}", f.file_name()))
        ));

        while let Some(lo) = tlparser.next_raw() {
            context.enter(lo);
        }
    }

    let mut format = String::new();
    File::open("data/format.txt")?.read_to_string(&mut format)?;

    let mut f = File::create("out.txt")?;
    f.write_all(
        Report::with_context(&mut context)
            .format(&format)
            .as_bytes(),
    )?;

    eprintln!(
        "total coins upon completion: {}",
        context.entities().currency_total(Currency::Coin)
    );
    Ok(())
}
