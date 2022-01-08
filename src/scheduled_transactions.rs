use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use strum::IntoEnumIterator;
use ynab_api::models;

use crate::args::*;
use crate::constants::*;
use crate::output::*;
use crate::types::*;
use crate::ynab_state::*;

pub fn get_scheduled_transaction(state: &YnabState) -> Result<(), AnyError> {
    let id = req_value_of(state.matches, ID_ARG);
    let include_subtransactions: bool =
        req_parse_value_of(state.matches, INCLUDE_SUBTRANSACTIONS_ARG);
    let response = state.run(&|c| {
        c.scheduled_transactions_api()
            .get_scheduled_transaction_by_id(&state.global.budget_id, id)
    })?;
    let mut wrapper = (*response.data()).clone();
    let mut tr: models::ScheduledTransactionDetail = (*wrapper.scheduled_transaction()).clone();
    if !include_subtransactions {
        tr.set_subtransactions(vec![]);
    }
    wrapper.set_scheduled_transaction(tr.clone());
    vtable_output(state, &wrapper, &tr, &make_scheduled_transaction_table)
}

pub fn list_scheduled_transactions(state: &YnabState) -> Result<(), AnyError> {
    let include_subtransactions: bool =
        req_parse_value_of(state.matches, INCLUDE_SUBTRANSACTIONS_ARG);
    let response = state.run(&|c| {
        c.scheduled_transactions_api()
            .get_scheduled_transactions(&state.global.budget_id)
    })?;
    let mut wrapper = (*response.data()).clone();
    let mut transactions: Vec<models::ScheduledTransactionDetail> = Vec::new();
    for tr in response.data().scheduled_transactions() {
        let mut tr: models::ScheduledTransactionDetail = (*tr).clone();
        if !include_subtransactions {
            tr.set_subtransactions(vec![]);
        }
        transactions.push(tr);
    }
    wrapper.set_scheduled_transactions(transactions.clone());
    htable_output(
        state,
        &wrapper,
        &transactions,
        &make_scheduled_transactions_table,
    )
}

fn scheduled_transaction_cell(
    settings: &models::BudgetSettings,
    tr: &models::ScheduledTransactionDetail,
    col: &ScheduledTransactionCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        ScheduledTransactionCol::Id => Cell::new(tr.id()),
        ScheduledTransactionCol::DateFirst => Cell::new(&date_str(settings, tr.date_first())),
        ScheduledTransactionCol::DateNext => Cell::new(&date_str(settings, tr.date_next())),
        ScheduledTransactionCol::Frequency => Cell::new(tr.frequency()),
        ScheduledTransactionCol::Amount => {
            Cell::new_align(&milliunits_str(settings, tr.amount()), currency_alignment)
        }
        ScheduledTransactionCol::Memo => Cell::new(&opt_ref_str(tr.memo())),
        ScheduledTransactionCol::FlagColor => Cell::new(&opt_to_str(tr.flag_color())),
        ScheduledTransactionCol::AccountId => Cell::new(tr.account_id()),
        ScheduledTransactionCol::PayeeId => Cell::new(&opt_ref_str(tr.payee_id())),
        ScheduledTransactionCol::CategoryId => Cell::new(&opt_ref_str(tr.category_id())),
        ScheduledTransactionCol::TransferAccountId => {
            Cell::new(&opt_ref_str(tr.transfer_account_id()))
        }
        ScheduledTransactionCol::Deleted => Cell::new(&tr.deleted().to_string()),
        ScheduledTransactionCol::Type => {
            Cell::new(&models::HybridTransactionType::Transaction.to_string())
        }
        ScheduledTransactionCol::ParentTransactionId => Cell::new(""),
        ScheduledTransactionCol::AccountName => Cell::new(tr.account_name()),
        ScheduledTransactionCol::PayeeName => Cell::new(&opt_ref_str(tr.payee_name())),
        ScheduledTransactionCol::CategoryName => Cell::new(&opt_ref_str(tr.category_name())),
    }
}

fn scheduled_subtransaction_cell(
    settings: &models::BudgetSettings,
    parent: &models::ScheduledTransactionDetail,
    sub: &models::ScheduledSubTransaction,
    col: &ScheduledTransactionCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        ScheduledTransactionCol::Id => Cell::new(sub.id()),
        ScheduledTransactionCol::DateFirst => Cell::new(""),
        ScheduledTransactionCol::DateNext => Cell::new(""),
        ScheduledTransactionCol::Frequency => Cell::new(""),
        ScheduledTransactionCol::Amount => {
            Cell::new_align(&milliunits_str(settings, sub.amount()), currency_alignment)
        }
        ScheduledTransactionCol::Memo => Cell::new(&opt_ref_str(sub.memo())),
        ScheduledTransactionCol::FlagColor => Cell::new(""),
        ScheduledTransactionCol::AccountId => Cell::new(""),
        ScheduledTransactionCol::PayeeId => Cell::new(&opt_ref_str(sub.payee_id())),
        ScheduledTransactionCol::CategoryId => Cell::new(&opt_ref_str(sub.category_id())),
        ScheduledTransactionCol::TransferAccountId => {
            Cell::new(&opt_ref_str(sub.transfer_account_id()))
        }
        ScheduledTransactionCol::Deleted => Cell::new(&sub.deleted().to_string()),
        ScheduledTransactionCol::Type => {
            Cell::new(&models::HybridTransactionType::Subtransaction.to_string())
        }
        ScheduledTransactionCol::ParentTransactionId => Cell::new(parent.id()),
        ScheduledTransactionCol::AccountName => Cell::new(""),
        // @@@ TODO USE PAYEE/CATEGORY APIS TO LOOK UP PAYEE/CATEGORY NAMES (BUT NEED TO CACHE)
        ScheduledTransactionCol::PayeeName => Cell::new(""),
        ScheduledTransactionCol::CategoryName => Cell::new(""),
    }
}

fn make_scheduled_transactions_table(
    state: &YnabState,
    columns: &Vec<ScheduledTransactionCol>,
    transactions: &Vec<models::ScheduledTransactionDetail>,
) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let alignment = &|col: &ScheduledTransactionCol| match *col {
        ScheduledTransactionCol::Amount => Alignment::RIGHT,
        _ => Alignment::LEFT,
    };
    let mut table = make_htable(state, columns, alignment);
    for tr in transactions {
        table.add_row(Row::new(
            columns
                .iter()
                .map(|col| scheduled_transaction_cell(&settings, tr, col, Alignment::RIGHT))
                .collect(),
        ));
        for sub in tr.subtransactions() {
            table.add_row(Row::new(
                columns
                    .iter()
                    .map(|col| {
                        scheduled_subtransaction_cell(&settings, tr, sub, col, Alignment::RIGHT)
                    })
                    .collect(),
            ));
        }
    }
    Ok(table)
}

fn make_scheduled_transaction_table(
    state: &YnabState,
    tr: &models::ScheduledTransactionDetail,
) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let mut table = make_vtable(state);
    for col in ScheduledTransactionCol::iter() {
        table.add_row(Row::new(vec![
            vfield_cell(&col.to_string()),
            scheduled_transaction_cell(&settings, tr, &col, Alignment::LEFT),
        ]));
    }
    for sub in tr.subtransactions() {
        table.add_row(Row::new(vec![Cell::new(""), Cell::new("")]));
        table.add_row(Row::new(vec![Cell::new("SUBTRANSACTION"), Cell::new("")]));
        for col in SCHEDULED_SUBTRANSACTION_COLS.iter() {
            table.add_row(Row::new(vec![
                vfield_cell(&col.to_string()),
                scheduled_subtransaction_cell(&settings, tr, sub, col, Alignment::LEFT),
            ]));
        }
    }
    Ok(table)
}
