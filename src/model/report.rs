use std::collections::HashMap;
use std::fmt::{self, Display};

use chrono::naive::{NaiveDate, MIN_DATE};

use numeral::Cardinal;
use tabular::{Row, Table};

use crate::{
    model::{Context, Entity, EntityKind, HistoryEntry},
    parser::ast::Currency,
};

pub struct Report<'a> {
    forbes: u32,
    date: NaiveDate,
    notes: Vec<String>,
    tables: Vec<AssetTable<'a>>,
    history: String,
}

impl<'a> Report<'a> {
    pub fn with_context(ctx: &'a mut Context) -> Report<'a> {
        let notes = ctx.take_notes();

        let mut asset_tables: HashMap<EntityKind, AssetTable> = HashMap::new();

        for ent in ctx.entities_vec_sorted() {
            asset_tables
                .entry(ent.kind())
                .or_insert(AssetTable::new(ctx.assets(), ent.kind()))
                .add_entity(ent)
        }

        let mut date = MIN_DATE;
        let mut history = String::from("");

        let tstr = "[{:<}] {:<} {:<}{:>}{:<} {:<}";
        let history_table = &mut Table::new(tstr);

        for entry in ctx.history().iter().rev() {
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
            date,
            notes,
            tables: asset_tables.into_iter().map(|(_, e)| e).collect(),
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
            .replace("{date}", &self.date.format("%d %B, %Y").to_string())
            .replace(
                "{tables}",
                &self
                    .tables
                    .iter()
                    .fold(String::new(), |acc, x| acc + "\n" + &x.to_string()),
            )
            .replace("{history}", &self.history)
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
        if self.footnotes.len() != 0 {
            write!(
                f,
                "{}\n{}",
                self.table,
                self.footnotes
                    .iter()
                    .enumerate()
                    .fold(String::new(), |acc, (i, name)| acc
                        + &format!("{}. {}\n", i, name))
            )
        } else {
            f.write_str(&self.table.to_string())
        }
    }
}
