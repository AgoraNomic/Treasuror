use std::cmp::max;
use std::env::args;
use std::fs::{self, File};
use std::io::BufReader;

use chrono::{
    naive::{NaiveDate, NaiveTime},
    Utc,
    Duration,
    Datelike,
    Weekday,
};

use plotters::prelude::*;

use assetlib::{
    model::{Context, Currency},
    parser::{gsdl::Parser as GsdParser, tll::Parser as TlParser},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().collect::<Vec<String>>();

    let end_date = if args.len() == 1 {
        eprintln!(
            "no argument given; generating report as of {} UTC",
            Utc::now().format("%c")
        );
        Utc::now().naive_utc()
    } else if args.len() > 1 {
        let date = NaiveDate::parse_from_str(&(args[1])[..], "%F")
            .unwrap_or_else(|_| panic!("invalid date format: {}", args[1]));

        let time = if args.len() == 2 {
            NaiveTime::from_hms(0, 0, 0)
        } else {
            NaiveTime::parse_from_str(&(args[2])[..], "%R")
                .unwrap_or_else(|_| panic!("invalid time format: {}", args[2]))
        };

        date.and_time(time)
    } else {
        panic!("unknown number of args: {}", args.len())
    };

    let mut context = Context::from_datetime(end_date);

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

    let mut max_coins = 0;
    let mut coinhist = Vec::new();

    for f in files.iter() {
        let mut tlparser = TlParser::from_reader(BufReader::new(
            File::open(f.path())
                .unwrap_or_else(|_| panic!("could not open tll file {:?}", f.file_name())),
        ));

        while let Some(lo) = tlparser.next_raw() {
            let before = context.datetime().date();
            context.enter(lo);
            let after = context.datetime().date();
            let coins = context.entities().currency_total(Currency::Coin);
            max_coins = max(max_coins, coins);
            for d in before.iter_days() {
                if d >= after {
                    break;
                }

                coinhist.push((d + Duration::days(1), coins));
            }
        }
    }

    let final_count = context.entities().currency_total(Currency::Coin);
    
    for d in context.datetime().date().iter_days() {
        if d > end_date.date() {
            break;
        }

        coinhist.push((d + Duration::days(1), final_count));
    }

    let root = BitMapBackend::new("chart.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Total coins over time (Mondays marked)", ("sans serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(
            (NaiveDate::from_ymd(2021, 1, 18)..end_date.date())
                .monthly(),
            10000..max_coins
        )?;

    chart.configure_mesh()
        .y_labels(30)
        .draw()?;

    chart.draw_series(LineSeries::new(
        coinhist.iter().copied(),
        RED.stroke_width(3))
    )?;

    chart.draw_series(PointSeries::of_element(
        coinhist.iter()
            .filter(|(d, _)| d.weekday() == Weekday::Mon)
            .copied(),
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
        },
    ))?;

    // let mut format = String::new();
    // File::open("data/format.txt")?.read_to_string(&mut format)?;

    // let mut f = File::create("out.txt")?;
    // f.write_all(
    //     Report::with_context(&mut context)
    //         .format(&format)
    //         .as_bytes(),
    // )?;

    eprintln!(
        "total coins upon completion: {}, max coins: {}",
        context.entities().currency_total(Currency::Coin),
        max_coins
    );
    Ok(())
}
