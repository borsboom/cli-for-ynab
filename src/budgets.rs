use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use ynab_api::models;

use crate::args::*;
use crate::output::*;
use crate::types::*;
use crate::ynab_state::*;

pub fn list_budgets(state: &YnabState) -> Result<(), AnyError> {
    let response = state.run(&|c| c.budgets_api().get_budgets())?;
    htable_output(
        state,
        response.data(),
        response.data(),
        &make_budget_summary_table,
    )
}

pub fn get_budget(state: &YnabState) -> Result<(), AnyError> {
    match state.global.output_format {
        OutputFormat::Table => return Err(Box::new(Error::GetBudgetOnlyJson)),
        OutputFormat::Csv => return Err(Box::new(Error::GetBudgetOnlyJson)),
        OutputFormat::Json => (),
        // no catch-all to ensure that if we add a new output format we check here
    }
    let last_knowledge_of_server = opt_last_knowledge_of_server_value_of(state.matches);
    let response = state.run(&|c| {
        c.budgets_api()
            .get_budget_by_id(&state.global.budget_id, last_knowledge_of_server)
    })?;
    json_output(response.data())
}

pub fn get_budget_settings(state: &YnabState) -> Result<(), AnyError> {
    let response = state.run(&|c| {
        c.budgets_api()
            .get_budget_settings_by_id(&state.global.budget_id)
    })?;
    vtable_output(
        state,
        response.data(),
        response.data().settings(),
        &make_budget_settings_table,
    )
}

fn make_budget_summary_table(
    state: &YnabState,
    columns: &Vec<BudgetCol>,
    data: &models::BudgetSummaryWrapper,
) -> Result<Table, AnyError> {
    let mut table = make_htable(state, columns, &|_| Alignment::LEFT);
    for budget in data.budgets() {
        let last_modified_on_str = opt_date_time_str(budget.last_modified_on());
        let curfmt = budget.currency_format();
        let currency_decimal_digits_str = opt_to_str(curfmt.map(|f| f.decimal_digits()));
        let currency_symbol_first_str = opt_to_str(curfmt.map(|f| f.symbol_first()));
        let currency_display_symbol_str = opt_to_str(curfmt.map(|f| f.display_symbol()));
        let col_value = |col: &BudgetCol| match col {
            BudgetCol::Id => Cell::new(budget.id()),
            BudgetCol::Name => Cell::new(budget.name()),
            // @@@ TODO: show it as a delta if recent (e.g. 2 hours ago, 2 months ago)
            BudgetCol::LastModified => Cell::new(&last_modified_on_str),
            BudgetCol::DateFormat => {
                Cell::new(&opt_ref_str(budget.date_format().map(|f| f.format())))
            }
            BudgetCol::CurrencyIsoCode => Cell::new(&opt_ref_str(curfmt.map(|f| f.iso_code()))),
            BudgetCol::CurrencyExampleFormat => {
                Cell::new(&opt_ref_str(curfmt.map(|f| f.example_format())))
            }
            BudgetCol::CurrencyDecimalDigits => Cell::new(&currency_decimal_digits_str),
            BudgetCol::CurrencyDecimalSeparator => {
                Cell::new(&opt_ref_str(curfmt.map(|f| f.decimal_separator())))
            }
            BudgetCol::CurrencySymbolFirst => Cell::new(&currency_symbol_first_str),
            BudgetCol::CurrencyGroupSeparator => {
                Cell::new(&opt_ref_str(curfmt.map(|f| f.group_separator())))
            }
            BudgetCol::CurrencySymbol => {
                Cell::new(&opt_ref_str(curfmt.map(|f| f.currency_symbol())))
            }
            BudgetCol::CurrencyDisplaySymbol => Cell::new(&currency_display_symbol_str),
        };
        table.add_row(Row::new(columns.iter().map(|n| col_value(n)).collect()));
    }
    Ok(table)
}

fn make_budget_settings_table(
    state: &YnabState,
    settings: &models::BudgetSettings,
) -> Result<Table, AnyError> {
    let mut table = make_vtable(state);
    let datefmt = settings.date_format();
    let curfmt = settings.currency_format();
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::DateFormat.to_string()),
        Cell::new(datefmt.format()),
    ]));
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::CurrencyIsoCode.to_string()),
        Cell::new(curfmt.iso_code()),
    ]));
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::CurrencyExampleFormat.to_string()),
        Cell::new(curfmt.example_format()),
    ]));
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::CurrencyDecimalDigits.to_string()),
        Cell::new(&curfmt.decimal_digits().to_string()),
    ]));
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::CurrencyDecimalSeparator.to_string()),
        Cell::new(curfmt.decimal_separator()),
    ]));
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::CurrencySymbolFirst.to_string()),
        Cell::new(&curfmt.symbol_first().to_string()),
    ]));
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::CurrencyGroupSeparator.to_string()),
        Cell::new(curfmt.group_separator()),
    ]));
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::CurrencySymbol.to_string()),
        Cell::new(curfmt.currency_symbol()),
    ]));
    table.add_row(Row::new(vec![
        vfield_cell(&BudgetCol::CurrencyDisplaySymbol.to_string()),
        Cell::new(&curfmt.display_symbol().to_string()),
    ]));
    Ok(table)
}
