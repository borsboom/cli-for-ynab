use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use strum::IntoEnumIterator;
use ynab_api::models;

use args::*;
use categories::*;
use constants::*;
use output::*;
use types::*;
use ynab_state::*;

pub fn list_months(state: &YnabState) -> Result<(), AnyError> {
    let response = state.run(&|c| c.months_api().get_budget_months(&state.global.budget_id))?;
    htable_output(
        state,
        response.data(),
        response.data().months(),
        &make_months_table,
    )
}

pub fn get_month(state: &YnabState) -> Result<(), AnyError> {
    // @@@ TODO: SHOULD HAVE A WAY TO GET A TABLE OF THE MONTH'S CATEGORIES
    let month = req_month_value_of(state.matches, MONTH_ARG);
    let include_categories: bool = req_parse_value_of(state.matches, INCLUDE_CATEGORIES_ARG);
    let response = state.run(&|c| {
        c.months_api()
            .get_budget_month(&state.global.budget_id, &month)
    })?;
    let mut wrapper = (*response.data()).clone();
    let mut month = (*wrapper.month()).clone();
    if !include_categories {
        month.set_categories(vec![]);
    }
    wrapper.set_month(month.clone());
    vtable_output(state, &wrapper, &month, &make_month_table)
}

fn month_summary_cell(
    settings: &models::BudgetSettings,
    month: &models::MonthSummary,
    col: &MonthCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        MonthCol::Month => Cell::new(&month_str(month.month())),
        MonthCol::Note => Cell::new(&opt_ref_str(month.note())),
        MonthCol::Income => Cell::new_align(
            &opt_milliunits_str(settings, month.income()),
            currency_alignment,
        ),
        MonthCol::Budgeted => Cell::new_align(
            &opt_milliunits_str(settings, month.budgeted()),
            currency_alignment,
        ),
        MonthCol::Activity => Cell::new_align(
            &opt_milliunits_str(settings, month.activity()),
            currency_alignment,
        ),
        MonthCol::ToBeBudgeted => Cell::new_align(
            &opt_milliunits_str(settings, month.to_be_budgeted()),
            currency_alignment,
        ),
        MonthCol::AgeOfMoney => Cell::new(&opt_to_str(month.age_of_money())),
    }
}

fn make_months_table(
    state: &YnabState,
    columns: &Vec<MonthCol>,
    months: &Vec<models::MonthSummary>,
) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let alignment = &|c: &MonthCol| match c {
        MonthCol::Income => Alignment::RIGHT,
        MonthCol::Budgeted => Alignment::RIGHT,
        MonthCol::Activity => Alignment::RIGHT,
        MonthCol::ToBeBudgeted => Alignment::RIGHT,
        _ => Alignment::LEFT,
    };
    let mut table = make_htable(state, columns, alignment);
    for month in months {
        table.add_row(Row::new(
            columns
                .iter()
                .map(|c| month_summary_cell(&settings, &month, c, Alignment::RIGHT))
                .collect(),
        ));
    }
    Ok(table)
}

fn month_detail_cell(
    settings: &models::BudgetSettings,
    month: &models::MonthDetail,
    col: &MonthCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        MonthCol::Month => Cell::new(&month_str(month.month())),
        MonthCol::Note => Cell::new(&opt_ref_str(month.note())),
        MonthCol::Income => Cell::new_align(
            &opt_milliunits_str(settings, month.income()),
            currency_alignment,
        ),
        MonthCol::Budgeted => Cell::new_align(
            &opt_milliunits_str(settings, month.budgeted()),
            currency_alignment,
        ),
        MonthCol::Activity => Cell::new_align(
            &opt_milliunits_str(settings, month.activity()),
            currency_alignment,
        ),
        MonthCol::ToBeBudgeted => Cell::new_align(
            &opt_milliunits_str(settings, month.to_be_budgeted()),
            currency_alignment,
        ),
        MonthCol::AgeOfMoney => Cell::new(&opt_to_str(month.age_of_money())),
    }
}

fn make_month_table(state: &YnabState, month: &models::MonthDetail) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let mut table = make_vtable(state);
    for col in MonthCol::iter() {
        table.add_row(Row::new(vec![
            vfield_cell(&col.to_string()),
            month_detail_cell(&settings, month, &col, Alignment::LEFT),
        ]));
    }
    for cat in month.categories() {
        table.add_row(Row::new(vec![Cell::new(""), Cell::new("")]));
        table.add_row(Row::new(vec![Cell::new("CATEGORY"), Cell::new("")]));
        for col in CATEGORY_NON_GROUP_COLS.iter() {
            table.add_row(Row::new(vec![
                vfield_cell(&col.to_string()),
                category_cell(&settings, None, cat, &col, Alignment::LEFT),
            ]));
        }
    }
    Ok(table)
}
