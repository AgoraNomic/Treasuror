use std::collections::HashMap;
use std::env::args;
use std::fs::{self, File};
use std::io::{BufReader, Write};

use chrono::{
    naive::{MIN_DATE, NaiveDate, NaiveTime},
    Utc,
    Duration,
    Datelike,
    Weekday,
};

use plotters::prelude::*;

use assetlib::{
    model::{Context, Currency},
    parser::{gsdl::{Directive, Parser as GsdParser}, tll::{Command, Parser as TlParser}},
};

struct EntHist {
    active: bool,
    registered: bool,
    notable: bool,
    history: Vec<(NaiveDate, u32)>,
}

impl EntHist {
    fn new() -> EntHist {
        EntHist {
            active: true,
            registered: true,
            notable: false,
            history: Vec::new(),
        }
    }
}

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

    let mut start_date = MIN_DATE;

    let mut context = Context::from_datetime(end_date);

    let mut coinhist = HashMap::new();

    let mut gsdparser = GsdParser::from_reader(BufReader::new(
        File::open("data/state.txt").expect("data/state.txt not found"),
    ));

    while let Some(d) = gsdparser.next_raw() {
        match d {
            Directive::Date(d) => {
                start_date = d;
            }
            Directive::Entity(ref e) => {
                coinhist.entry(e.identifier().clone()).or_insert(EntHist::new());
            }
            _ => {}
        }
        context.process(&d);
    }

    let mut files = fs::read_dir("data/tll")?
        .map(|f| f.unwrap())
        .collect::<Vec<_>>();
    files.sort_by_key(|f| f.file_name());

    for f in files.iter() {
        let mut tlparser = TlParser::from_reader(BufReader::new(
            File::open(f.path())
                .unwrap_or_else(|_| panic!("could not open tll file {:?}", f.file_name())),
        ));

        while let Some(lo) = tlparser.next_raw() {
            let before = context.datetime().date();

            match lo.action() {
                Command::NewContract(s1, _) => {
                    coinhist.entry(s1.clone()).or_insert(EntHist::new());
                }
                Command::NewPlayer(s1, _) => {
                    coinhist.entry(s1.clone()).or_insert(EntHist::new());
                }
                Command::Deregister(s1) => {
                    coinhist.get_mut(&s1[..]).unwrap().registered = false;
                }
                Command::Deactivate(s1) => {
                    coinhist.get_mut(&s1[..]).unwrap().active = false;
                }
                Command::Activate(s1) => {
                    coinhist.get_mut(&s1[..]).unwrap().active = true;
                }
                _ => {}
            }

            context.enter(lo);
            let after = context.datetime().date();

            for d in before.iter_days() {
                if d >= after {
                    break;
                }

                for (e, hist) in coinhist.iter_mut() {
                    if !hist.registered {
                        continue;
                    }
                    
                    let balance = context.entities().get(e).unwrap().balance(Currency::Coin);

                    if balance > 1800 {
                        hist.notable = true;
                    }

                    hist.history.push((
                        d, //+ Duration::days(1),
                        balance
                    ));
                }
            }
        }
    }

    for d in context.datetime().date().iter_days() {
        if d > end_date.date() {
            break;
        }

        for (e, hist) in coinhist.iter_mut() {
            if !hist.registered {
                continue;
            }

            hist.history.push((
                d, // + Duration::days(1),
                context.entities().get(e).unwrap().balance(Currency::Coin)
            ));
        }
    }

    // ---UNCOMMENT FOR CSV GENERATION--- //
    //let mut table = Vec::new();

    //let mut hists = coinhist.iter().map(|(e, hist)| {
    //    let mut v = Vec::new();
    //    v.push(e.clone());
    //    table.push(v);
    //    hist.history.iter().peekable()
    //}).collect::<Vec<_>>();

    //for d in start_date.iter_days() {
    //    if d >= end_date.date() {
    //        break;
    //    }

    //    for (i, col) in table.iter_mut().enumerate() {
    //        let hist = &mut hists[i];
    //        if let Some((hd, entry)) = hist.peek() {
    //            if d != *hd {
    //                col.push("".to_string());
    //            } else {
    //                col.push(entry.to_string());
    //                hist.next();
    //            }
    //        }
    //    }
    //}

    //let table_out = table.iter().map(|c| c.join(",")).fold(String::new(), |acc, item| {
    //    acc + &item[..] + "\n"
    //});

    //let mut f = File::create("table.txt")?;
    //f.write_all(table_out.as_bytes())?;

    let root = BitMapBackend::new("chart.png", (1350, 800)).into_drawing_area();
    root.fill(&RGBColor(180, 180, 180))?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Coins over time for entities who at one time owned over 1800 coins", ("sans serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(
            (NaiveDate::from_ymd(2021, 1, 18)..end_date.date() + Duration::days(30))
                .monthly(),
            0u32..15000u32
        )?;

    chart.configure_mesh()
        .y_labels(30)
        .draw()?;

    let mut sorted = coinhist.iter()
        .filter(|(_, h)| {
            /* h.active && h.registered && */ h.notable
        })
        .filter(|(_, h)| {
            h.history.iter()
                .filter(|(_, n)| n > &0)
                .next()
                .is_some()
        })
        //.filter(|(e, _)| {
        //    context.entities().get(e).unwrap().kind().is_player()
        //})
        .collect::<Vec<_>>();

    sorted.sort_by(|(e1, _), (e2, _)| {
        e1.to_lowercase().cmp(&e2.to_lowercase())
    });

    for (idx, (e, hist)) in sorted.iter().enumerate() {
        let color = Palette99::pick(idx);

        chart.draw_series(LineSeries::new(
            hist.history.iter().copied(),
            color.mix(0.9).stroke_width(3)
        ))?
            .label(e.replace('_', " "))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(idx).stroke_width(3)));

        chart.draw_series(PointSeries::of_element(
            hist.history.iter()
                .filter(|(d, _)| d.weekday() == Weekday::Mon)
                .copied(),
            5,
            color.filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
            },
        ))?;
    }

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE)
        .draw()?;

    // let mut format = String::new();
    // File::open("data/format.txt")?.read_to_string(&mut format)?;

    // let mut f = File::create("out.txt")?;
    // f.write_all(
    //     Report::with_context(&mut context)
    //         .format(&format)
    //         .as_bytes(),
    // )?;

    eprintln!(
        "total coins upon completion: {}",
        context.entities().currency_total(Currency::Coin),
    );
    Ok(())
}
