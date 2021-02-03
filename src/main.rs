use std::fs::File;
use std::io::{self, BufReader, Read, Write};

pub mod model;
pub mod parser;

use model::{Context, Report};
use parser::{gsdl::Parser as GsdParser, tll::Parser as TlParser};

fn main() -> io::Result<()> {
    let mut context = Context::new();

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
    f.write(Report::with_context(&mut context).format(&format).as_bytes())?;
    Ok(())
}
