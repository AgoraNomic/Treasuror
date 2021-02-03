use std::collections::HashMap;

use chrono::naive::{MIN_DATE, NaiveDate};

use tabular::{Row, Table};
use numeral::Cardinal;

use crate::{
    model::{Context, Entity, EntityKind, HistoryEntry},
    parser::ast::Currency,
};

pub struct Report {
    forbes: u32,
    date: NaiveDate,
    notes: Vec<String>,
    tables: Vec<String>,
    history: String,
}

impl Report {
    pub fn with_context(ctx: &mut Context) -> Report {
        let mut asset_tables: HashMap<EntityKind, Table> = HashMap::new();

        let mut entities = ctx.entities().values().collect::<Vec<&Entity>>();
        entities.sort_by(|a, b| a.identifier().to_lowercase().cmp(&b.identifier().to_lowercase()));

        for ent in entities {
            let mut row = Row::new().with_cell(ent.identifier().replace("_", " "));

            for curr in ctx.assets().iter() {
                row.add_cell(ent.balance(*curr));
            }

            asset_tables.entry(ent.kind()).or_insert(Report::asset_table(ctx.assets(), ent.kind())).add_row(row);
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
            notes: ctx.take_notes(),
            tables: asset_tables.values().map(|t| t.to_string()).collect(),
            history,
        }
    }
    
    fn asset_table(assets: &Vec<Currency>, kind: EntityKind) -> Table {
        let mut title_row = Row::new().with_cell(format!("{:^14}", kind));
        let mut bar_row = Row::new().with_cell("=".repeat(14));

        assets.iter().for_each(|c| {
            if *c == Currency::Coin {
                title_row.add_cell(format!("{:^6}", c.abbr()));
                bar_row.add_cell("======");
            } else {
                title_row.add_cell(format!("{:^4}", c.abbr()));
                bar_row.add_cell("====");
            }
        });

        Table::new(&format!("{}{}", " {:<}", "  {:>}".repeat(assets.len())))
            .with_row(title_row)
            .with_row(bar_row)
    }

    pub fn format(&self, fstr: &str) -> String {
        fstr
            .replace("{bar}", &"=".repeat(72))
            .replace("{forbes}", &format!("{:^72}", String::from("FORBES ") + &self.forbes.cardinal().to_uppercase()))
            .replace("{date}", &self.date.format("%d %B, %Y").to_string())
            .replace("{tables}", &self.tables.join("\n\n"))
            .replace("{history}", &self.history)
    }
}
