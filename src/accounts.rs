use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use strum::IntoEnumIterator;
use ynab_api::models;

use args::*;
use constants::*;
use output::*;
use types::*;
use ynab_state::*;

pub fn list_accounts(state: &YnabState) -> Result<(), AnyError> {
    //@@@ TODO: allow filtering by type and on-budget
    //@@@ TODO: generally allow sorting htables
    // let include_closed: bool = req_parse_value_of(state.matches, INCLUDE_CLOSED_ARG);
    let response = state.run(&|c| c.accounts_api().get_accounts(&state.global.budget_id))?;
    // let mut accounts: Vec<models::Account> = Vec::new();
    // for acc in response.data().accounts() {
    //     if include_closed || !acc.closed() {
    //         accounts.push(acc.clone());
    //     }
    // }
    // htable_output(state, &accounts, &make_accounts_table)
    htable_output(
        state,
        response.data(),
        response.data().accounts(),
        &make_accounts_table,
    )
}

pub fn get_account(state: &YnabState) -> Result<(), AnyError> {
    let id = req_value_of(state.matches, ID_ARG);
    let response = state.run(&|c| {
        c.accounts_api()
            .get_account_by_id(&state.global.budget_id, &id)
    })?;
    vtable_output(
        state,
        response.data(),
        response.data().account(),
        &make_account_table,
    )
}

fn account_cell(
    settings: &models::BudgetSettings,
    acc: &models::Account,
    col: &AccountCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        AccountCol::Id => Cell::new(acc.id()),
        AccountCol::Name => Cell::new(acc.name()),
        AccountCol::Type => Cell::new(acc._type()),
        AccountCol::OnBudget => Cell::new(&acc.on_budget().to_string()),
        AccountCol::Closed => Cell::new(&acc.closed().to_string()),
        AccountCol::Note => Cell::new(&opt_ref_str(acc.note())),
        AccountCol::Balance => Cell::new_align(
            &milliunits_str(&settings, acc.balance()),
            currency_alignment,
        ),
        AccountCol::ClearedBalance => Cell::new_align(
            &milliunits_str(&settings, acc.cleared_balance()),
            currency_alignment,
        ),
        AccountCol::UnclearedBalance => Cell::new_align(
            &milliunits_str(&settings, acc.uncleared_balance()),
            currency_alignment,
        ),
        AccountCol::TransferPayeeId => Cell::new(acc.transfer_payee_id()),
        AccountCol::Deleted => Cell::new(&acc.deleted().to_string()),
    }
}

fn make_accounts_table(
    state: &YnabState,
    columns: &Vec<AccountCol>,
    accounts: &Vec<models::Account>,
) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let alignment = &|col: &AccountCol| match *col {
        AccountCol::Balance => Alignment::RIGHT,
        AccountCol::ClearedBalance => Alignment::RIGHT,
        AccountCol::UnclearedBalance => Alignment::RIGHT,
        _ => Alignment::LEFT,
    };
    let mut table = make_htable(state, columns, alignment);
    for acc in accounts {
        table.add_row(Row::new(
            columns
                .iter()
                .map(|c| account_cell(&settings, &acc, c, Alignment::RIGHT))
                .collect(),
        ));
    }
    Ok(table)
}

fn make_account_table(state: &YnabState, account: &models::Account) -> Result<Table, AnyError> {
    // @@@ THIS SHOULD ONLY BE CALLED WHEN SETTINGS *ACTUALLY* NEEDED, HERE AND EVERYWHERE (E.G. GET THE FORMATTING FUNCTION TO DO IT)
    let settings = state.get_budget_settings()?;
    let mut table = make_vtable(state);
    for col in AccountCol::iter() {
        table.add_row(Row::new(vec![
            vfield_cell(&col.to_string()),
            account_cell(&settings, account, &col, Alignment::LEFT),
        ]));
    }
    Ok(table)
}
