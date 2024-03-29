use std::fmt::{self, Display};

use chrono::{
    naive::{NaiveDate, MIN_DATE},
    Datelike,
};

use numeral::Cardinal;
use tabular::{Row, Table};
use textwrap::{wrap, Options};

use crate::model::{Context, Currency, Entity, EntityKind, HistoryEntry};

pub struct Report<'a> {
    forbes: u32,
    total_buoyancy: u32,
    buoyancy_target: u32,
    flotation: f32,
    date: NaiveDate,
    // notes: Vec<String>,
    tables: Vec<AssetTable<'a>>,
    history: String,
}

impl<'a> Report<'a> {
    pub fn with_context(ctx: &'a mut Context) -> Report<'a> {
        // let notes = ctx.take_notes();

        let entities_grouped = ctx.entities().as_grouped_vec();
        let mut asset_tables = Vec::new();

        for (kind, entities) in entities_grouped.iter() {
            let mut at = AssetTable::new(&ctx.assets(), *kind);
            for e in entities.iter() {
                at.add_entity(e);
            }
            asset_tables.push(at);
        }

        let mut date = MIN_DATE;
        let mut history = String::from("");

        let tstr = "[{:<}] {:<} {:<}{:>}{:<} {:<}";
        let history_table = &mut Table::new(tstr);

        let threshold = 2;
        let end_month = ctx.max_datetime().month0();
        let before_month = (end_month + 11 - threshold) % 12;

        for entry in ctx.history().iter().rev() {
            let now_month = entry.datetime().month0();

            if now_month == before_month {
                break;
            }

            if entry.datetime().date() != date {
                history.push_str(&history_table.to_string()[..]);
                *history_table = Table::new(tstr)
                    .with_heading("")
                    .with_heading(entry.datetime().format("*** %a %-d %B %Y").to_string());

                date = entry.datetime().date();
            }

            match entry.entry() {
                HistoryEntry::Transaction(t) => history_table.add_row(
                    Row::new()
                        .with_cell(entry.datetime().format("%R"))
                        .with_cell(format!("{}:", t.agent().replace("_", " ")))
                        .with_cell(if t.amount() > 0 { "+" } else { "-" })
                        .with_cell(t.amount().abs())
                        .with_cell(t.currency().abbr())
                        .with_cell(format!("({})", t.comment())),
                ),
                HistoryEntry::Event(s) => {
                    history_table.add_heading(format!("[{}] {}", entry.datetime().format("%R"), s))
                }
            };
        }
        history.push_str(&history_table.to_string()[..]);

        Report {
            forbes: ctx.forbes(),
            buoyancy_target: ctx.buoyancy_target(),
            total_buoyancy: ctx.total_buoyancy(),
            flotation: ctx.flotation(),
            date: ctx.max_datetime().date(),
            // notes,
            tables: asset_tables,
            history,
        }
    }

    pub fn format(&self, fstr: &str) -> String {
        fstr.replace("{bar}", &"=".repeat(72))
            .replace(
                "{forbes}",
                &format!(
                    "{:^72}",
                    String::from("FORBES ") + &self.forbes.cardinal().to_uppercase()
                ),
            )
            .replace("{date}", &self.date.format("%d %B %Y").to_string())
            .replace(
                "{tables}",
                &self
                    .tables
                    .iter()
                    .fold(String::new(), |acc, x| acc + &x.to_string() + "\n")
                    .trim_end(),
            )
            .replace("{history}", self.history.trim())
            .replace(
                "{buoyancy}",
                Table::new("{:<} {:<}")
                    .with_row(
                        Row::new()
                            .with_cell("Total Buoyancy:")
                            .with_cell(self.total_buoyancy),
                    )
                    .with_row(
                        Row::new()
                            .with_cell("Buoyancy Target:")
                            .with_cell(self.buoyancy_target),
                    )
                    .with_row(
                        Row::new()
                            .with_cell("Unit of Flotation:")
                            .with_cell(format!("{:.4}", self.flotation)),
                    )
                    .to_string()
                    .trim_end(),
            )
    }
}

struct AssetTable<'a> {
    assets: &'a [Currency],
    table: Table,
    footnotes: Vec<&'a str>,
}

impl<'a> AssetTable<'a> {
    pub fn new(assets: &'a [Currency], kind: EntityKind) -> AssetTable<'a> {
        let mut title_row = Row::new().with_cell(format!("{:^14}", kind));
        let mut bar_row = Row::new().with_cell("=".repeat(14));

        for c in assets.iter() {
            if *c == Currency::Coin {
                title_row.add_cell(format!("{:^6}", c.abbr()));
                bar_row.add_cell("======");
            } else {
                title_row.add_cell(format!("{:^4}", c.abbr()));
                bar_row.add_cell("====");
            }
        }

        AssetTable {
            assets,
            table: Table::new(&format!("{}{}", " {:<}", "  {:>}".repeat(assets.len())))
                .with_row(title_row)
                .with_row(bar_row),
            footnotes: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, ent: &'a Entity) {
        let mut row = Row::new().with_cell(format!(
            "{:<10}{:>4}",
            ent.identifier().replace("_", " "),
            if ent.has_full_name() {
                format!("[{}]", self.footnotes.len())
            } else {
                String::from("   ")
            }
        ));

        if ent.has_full_name() {
            self.footnotes.push(ent.full_name());
        }

        for curr in self.assets.iter() {
            row.add_cell(ent.balance(*curr));
        }

        self.table.add_row(row);
    }
}

impl<'a> Display for AssetTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.footnotes.is_empty() {
            write!(
                f,
                "{}\n{}",
                self.table,
                self.footnotes
                    .iter()
                    .enumerate()
                    .fold(String::new(), |acc, (i, name)| acc
                        + &wrap(
                            name,
                            Options::new(72)
                                .initial_indent(&format!("{}. ", i))
                                .subsequent_indent("   ")
                        )
                        .join("\n")
                        + "\n")
            )
        } else {
            f.write_str(&self.table.to_string())
        }
    }
}
