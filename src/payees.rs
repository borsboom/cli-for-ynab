use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use strum::IntoEnumIterator;
use ynab_api::models;

use args::*;
use constants::*;
use output::*;
use types::*;
use ynab_state::*;

pub fn list_payees(state: &YnabState) -> Result<(), AnyError> {
    let response = state.run(&|c| c.payees_api().get_payees(&state.global.budget_id))?;
    htable_output(
        state,
        response.data(),
        response.data().payees(),
        &make_payees_table,
    )
}

pub fn list_payee_locations(state: &YnabState) -> Result<(), AnyError> {
    let opt_payee_id = state.matches.value_of(PAYEE_ID_ARG);
    let response = state.run(&|c| {
        if let Some(payee_id) = opt_payee_id {
            c.payee_locations_api()
                .get_payee_locations_by_payee(&state.global.budget_id, payee_id)
        } else {
            c.payee_locations_api()
                .get_payee_locations(&state.global.budget_id)
        }
    })?;
    htable_output(
        state,
        response.data(),
        response.data().payee_locations(),
        &make_payee_locations_table,
    )
}

pub fn get_payee(state: &YnabState) -> Result<(), AnyError> {
    let id = req_value_of(state.matches, ID_ARG);
    let response = state.run(&|c| c.payees_api().get_payee_by_id(&state.global.budget_id, id))?;
    vtable_output(
        state,
        response.data(),
        response.data().payee(),
        &make_payee_table,
    )
}

pub fn get_payee_location(state: &YnabState) -> Result<(), AnyError> {
    let id = req_value_of(state.matches, ID_ARG);
    let response = state.run(&|c| {
        c.payee_locations_api()
            .get_payee_location_by_id(&state.global.budget_id, id)
    })?;
    vtable_output(
        state,
        response.data(),
        response.data().payee_location(),
        &make_payee_location_table,
    )
}

fn payee_cell(pay: &models::Payee, col: &PayeeCol) -> Cell {
    match col {
        PayeeCol::Id => Cell::new(pay.id()),
        PayeeCol::Name => Cell::new(pay.name()),
        PayeeCol::TransferAccountId => Cell::new(&opt_ref_str(pay.transfer_account_id())),
        PayeeCol::Deleted => Cell::new(&pay.deleted().to_string()),
    }
}

fn make_payees_table(
    state: &YnabState,
    columns: &Vec<PayeeCol>,
    payees: &Vec<models::Payee>,
) -> Result<Table, AnyError> {
    let alignment = &|_: &PayeeCol| Alignment::LEFT;
    let mut table = make_htable(state, columns, alignment);
    for pay in payees {
        table.add_row(Row::new(
            columns.iter().map(|c| payee_cell(pay, c)).collect(),
        ));
    }
    Ok(table)
}

fn make_payee_table(state: &YnabState, payee: &models::Payee) -> Result<Table, AnyError> {
    let mut table = make_vtable(state);
    for col in PayeeCol::iter() {
        table.add_row(Row::new(vec![
            vfield_cell(&col.to_string()),
            payee_cell(payee, &col),
        ]));
    }
    Ok(table)
}

fn payee_location_cell(loc: &models::PayeeLocation, col: &PayeeLocationCol) -> Cell {
    match col {
        PayeeLocationCol::Id => Cell::new(loc.id()),
        PayeeLocationCol::PayeeId => Cell::new(loc.payee_id()),
        PayeeLocationCol::Latitude => Cell::new(loc.latitude()),
        PayeeLocationCol::Longitude => Cell::new(loc.longitude()),
        PayeeLocationCol::Deleted => Cell::new(&loc.deleted().to_string()),
    }
}

fn make_payee_locations_table(
    state: &YnabState,
    columns: &Vec<PayeeLocationCol>,
    payee_locations: &Vec<models::PayeeLocation>,
) -> Result<Table, AnyError> {
    // @@@ TODO: WOULD BE NICE TO INCLUDE PAYEE LOCATION INFO IN 'get payee' and 'get payees' INSTEAD OF SEPARATE COMMAND
    let alignment = &|_: &PayeeLocationCol| Alignment::LEFT;
    let mut table = make_htable(state, columns, alignment);
    for pay in payee_locations {
        table.add_row(Row::new(
            columns
                .iter()
                .map(|c| payee_location_cell(pay, c))
                .collect(),
        ));
    }
    Ok(table)
}

fn make_payee_location_table(
    state: &YnabState,
    loc: &models::PayeeLocation,
) -> Result<Table, AnyError> {
    let mut table = make_vtable(state);
    for col in PayeeLocationCol::iter() {
        table.add_row(Row::new(vec![
            vfield_cell(&col.to_string()),
            payee_location_cell(loc, &col),
        ]));
    }
    Ok(table)
}
