use prettytable::{Cell, Row, Table};
use ynab_api::models;

use output::*;
use types::*;
use ynab_state::*;

pub fn get_user(state: &YnabState) -> Result<(), AnyError> {
    let response = state.run(&|c| c.user_api().get_user())?;
    vtable_output(
        state,
        response.data(),
        response.data().user(),
        &make_user_table,
    )
}

fn make_user_table(state: &YnabState, user: &models::User) -> Result<Table, AnyError> {
    let mut table = make_vtable(state);
    table.add_row(Row::new(vec![vfield_cell("id"), Cell::new(user.id())]));
    Ok(table)
}
