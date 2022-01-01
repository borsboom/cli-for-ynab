use chrono::prelude::*;
use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use std::fmt;
use std::io;
use std::str::FromStr;
use strum::IntoEnumIterator;
use ynab_api::models;

use constants::*;
use types::*;
use ynab_state::*;

fn get_columns<C, I>(matches: &clap::ArgMatches<'static>) -> Vec<C>
where
    C: FromStr,
    C: IntoEnumIterator<Iterator = I>,
    I: Iterator<Item = C>,
    C: Clone,
    <C as FromStr>::Err: fmt::Debug,
{
    // expect() below is safe since CLI parser has validated the column name already
    let vals: Vec<_> = matches
        .values_of(TABLE_COLUMNS_ARG)
        .unwrap_or_else(|| panic!("Expected {} value to be non-empty", TABLE_COLUMNS_ARG))
        .collect();
    if vals.iter().any(|v| *v == ALL_VAL) {
        C::iter().collect()
    } else {
        vals.iter()
            .map(|c| {
                C::from_str(c).unwrap_or_else(|_| {
                    panic!(
                        "Expected {} column value to be valid: {}",
                        TABLE_COLUMNS_ARG, c
                    )
                })
            })
            .collect()
    }
}

pub fn json_output<D>(data: &D) -> Result<(), AnyError>
where
    D: serde::Serialize,
{
    let json = serde_json::to_string(data)?;
    println!("{}", json);
    Ok(())
}

pub fn htable_output<C, D, I, W>(
    state: &YnabState,
    wrapper: &W,
    data: &D,
    make_table: &dyn Fn(&YnabState, &Vec<C>, &D) -> Result<Table, AnyError>,
) -> Result<(), AnyError>
where
    C: FromStr,
    C: IntoEnumIterator<Iterator = I>,
    I: Iterator<Item = C>,
    C: Clone,
    <C as FromStr>::Err: fmt::Debug,
    W: serde::ser::Serialize,
{
    match state.global.output_format {
        OutputFormat::Table => {
            let columns = get_columns(state.matches);
            make_table(state, &columns, data)?.printstd();
        }
        OutputFormat::Csv => {
            let columns = get_columns(state.matches);
            make_table(state, &columns, data)?.to_csv(io::stdout())?;
        }
        OutputFormat::Json => {
            json_output(wrapper)?;
        }
    }
    Ok(())
}

pub fn vtable_output<D, W>(
    state: &YnabState,
    wrapper: &W,
    data: &D,
    make_table: &dyn Fn(&YnabState, &D) -> Result<Table, AnyError>,
) -> Result<(), AnyError>
where
    W: serde::ser::Serialize,
{
    match state.global.output_format {
        OutputFormat::Table => {
            make_table(state, data)?.printstd();
        }
        OutputFormat::Csv => {
            make_table(state, data)?.to_csv(io::stdout())?;
        }
        OutputFormat::Json => {
            json_output(wrapper)?;
        }
    }
    Ok(())
}

pub fn opt_date_time_str(dt: Option<&DateTime<Local>>) -> String {
    dt.map(|dt| dt.format("%c").to_string())
        .unwrap_or_else(|| "".to_string())
}

pub fn opt_month_str(d: Option<&NaiveDate>) -> String {
    d.map(|d| d.format(MONTH_FORMAT).to_string())
        .unwrap_or_else(|| "".to_string())
}

pub fn month_str(d: &NaiveDate) -> String {
    d.format(MONTH_FORMAT).to_string()
}

pub fn opt_str(v: Option<String>) -> String {
    v.unwrap_or_else(|| "".to_string())
}

pub fn opt_ref_str(v: Option<&String>) -> String {
    v.map(|v| v.to_owned()).unwrap_or_else(|| "".to_string())
}

pub fn opt_to_str<T>(v: Option<T>) -> String
where
    T: ToString,
{
    opt_str(v.map(|u| u.to_string()))
}

pub fn opt_milliunits_str(
    settings: &models::BudgetSettings,
    opt_mu: Option<&models::Milliunits>,
) -> String {
    opt_mu
        .map(|mu| milliunits_str(settings, mu))
        .unwrap_or_else(|| "".to_string())
}

pub fn milliunits_str(settings: &models::BudgetSettings, mu: &models::Milliunits) -> String {
    //@@@ TODO SUPPORT GROUP SEPARATORS
    //@@@ TODO ADD A --CURRENCY CLI ARG TO SELECT BETWEEN PLAIN, FORMATTED, OR MILLIUNITS (IN WHICH CASE, SHOUDN'T NEED TO RETRIEVE BUDGET SETTINGS)
    let fmt = settings.currency_format();
    let dd = fmt.decimal_digits();
    let mu = mu.to_int();
    let f: f64 = ((mu as f64) / 1000.0).abs();
    let s = format!("{:.*}", *dd as usize, f).replacen(".", fmt.decimal_separator(), 1);
    let s = if *fmt.display_symbol() {
        if *fmt.symbol_first() {
            format!("{}{}", fmt.currency_symbol(), s)
        } else {
            format!("{}{}", s, fmt.currency_symbol())
        }
    } else {
        s
    };
    if mu < 0 {
        format!("-{}", s)
    } else {
        s
    }
}

pub fn date_str(settings: &models::BudgetSettings, d: &NaiveDate) -> String {
    let fmt = settings
        .date_format()
        .format()
        .replace("YYYY", "%Y")
        .replace("MM", "%m")
        .replace("DD", "%d");
    d.format(&fmt).to_string()
}

pub fn make_table(state: &YnabState) -> Table {
    let mut table = Table::new();
    if state.global.table_borders {
        table.set_format(*TABLE_WITH_BORDERS_FORMAT);
    } else {
        table.set_format(*TABLE_NO_BORDERS_FORMAT);
    }
    table
}

pub fn header_cell(name: &str, alignment: Alignment) -> Cell {
    Cell::new_align(&name.replace("-", " ").to_uppercase(), alignment)
}

pub fn make_htable<C>(
    state: &YnabState,
    columns: &Vec<C>,
    alignment: &dyn Fn(&C) -> Alignment,
) -> Table
where
    C: ToString,
{
    let mut table = make_table(state);
    if state.global.headers {
        table.set_titles(Row::new(
            columns
                .iter()
                .map(|col| header_cell(&col.to_string(), alignment(col)))
                .collect(),
        ));
    }
    table
}

pub fn make_vtable(state: &YnabState) -> Table {
    make_table(state)
    // if state.global.headers {
    //     table.set_titles(Row::new(
    //         vec![header_cell("field:", Alignment::RIGHT), header_cell("value", Alignment::LEFT)]));
    // }
}

pub fn to_sentence_case(s: &str) -> String {
    SENTENCE_TO_UPPERCASE_RE
        .replace_all(s, |caps: &regex::Captures| caps[1].to_uppercase())
        .to_string()
}

pub fn vfield_cell(name: &str) -> Cell {
    Cell::new_align(
        &format!("{}:", to_sentence_case(&name.replace("-", " "))),
        Alignment::RIGHT,
    )
}
