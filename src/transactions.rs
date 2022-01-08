use chrono::NaiveDate;
use prettytable::format::Alignment;
use prettytable::{Cell, Row, Table};
use strum::IntoEnumIterator;
use ynab_api::models;

use crate::args::*;
use crate::constants::*;
use crate::output::*;
use crate::types::*;
use crate::ynab_state::*;

pub fn get_transaction(state: &YnabState) -> Result<(), AnyError> {
    let id = req_value_of(state.matches, ID_ARG);
    let include_subtransactions: bool =
        req_parse_value_of(state.matches, INCLUDE_SUBTRANSACTIONS_ARG);
    let response = state.run(&|c| {
        c.transactions_api()
            .get_transaction_by_id(&state.global.budget_id, id)
    })?;
    let mut wrapper = (*response.data()).clone();
    let mut tr: models::TransactionDetail = (*wrapper.transaction()).clone();
    if !include_subtransactions {
        tr.set_subtransactions(vec![]);
    }
    wrapper.set_transaction(tr.clone());
    vtable_output(state, &wrapper, &tr, &make_transaction_table)
}

fn make_save_transaction(
    state: &YnabState,
    old: models::TransactionDetail,
) -> Result<models::SaveTransaction, AnyError> {
    let settings = state.get_budget_settings()?;
    let mut new = models::SaveTransaction::new(
        state
            .matches
            .value_of(SET_ACCOUNT_ID_ARG)
            .unwrap_or_else(|| old.account_id())
            .to_string(),
        opt_date_value_of(&settings, state.matches, SET_DATE_ARG)?.unwrap_or(*old.date()),
        opt_milliunits_value_of(&settings, state.matches, SET_AMOUNT_ARG)?.unwrap_or(*old.amount()),
    );
    //@@@ RENAME/MOVE
    fn xxx<T>(
        newv: &Option<&str>,
        oldv: Option<&String>,
        new: &mut T,
        setter: &dyn Fn(&mut T, &str),
    ) {
        if let Some(v) = newv.map_or(oldv.map(|v| &v[..]), Some) {
            setter(new, v)
        };
    }
    if let new_payee_name @ Some(_) = state.matches.value_of(SET_PAYEE_NAME_ARG) {
        xxx(&new_payee_name, old.payee_name(), &mut new, &|new, v| {
            new.set_payee_name(v.to_string())
        });
    } else {
        xxx(
            &state.matches.value_of(SET_PAYEE_ID_ARG),
            old.payee_id(),
            &mut new,
            &|new, v| new.set_payee_id(v.to_string()),
        );
    }
    xxx(
        &state.matches.value_of(SET_CATEGORY_ID_ARG),
        old.category_id(),
        &mut new,
        &|new, v| new.set_category_id(v.to_string()),
    );
    //@@@ TODO: support looking up a category name
    xxx(
        &state.matches.value_of(SET_MEMO_ARG),
        old.memo(),
        &mut new,
        &|new, v| new.set_memo(v.to_string()),
    );
    // @@@ state.matches.value_of(SET_MEMO_ARG).map_or(old.memo().map(|v| &v[..]), |v| Some(v)).map(|v| new.set_memo(v.to_string()));
    new.set_cleared(opt_parse_value_of(state.matches, SET_CLEARED_ARG).unwrap_or(*old.cleared()));
    new.set_approved(
        opt_parse_value_of(state.matches, SET_APPROVED_ARG).unwrap_or(*old.approved()),
    );
    match opt_flag_color_value_of(state.matches, SET_FLAG_COLOR_ARG)
        .unwrap_or_else(|| old.flag_color().cloned())
    {
        None => {
            new.reset_flag_color();
        }
        Some(c) => {
            new.set_flag_color(c);
        }
    }
    xxx(
        &state.matches.value_of(SET_IMPORT_ID_ARG),
        old.import_id(),
        &mut new,
        &|new, v| new.set_import_id(v.to_string()),
    );
    // println!("@@@ savetransaction={:?}", new);
    Ok(new)
}

pub fn update_transaction(state: &YnabState) -> Result<(), AnyError> {
    let id = req_value_of(state.matches, ID_ARG);
    let model: models::SaveTransactionWrapper = if let Some(m) = file_input(state)? {
        m
    } else {
        let old = state
            .run(&|c| {
                c.transactions_api()
                    .get_transaction_by_id(&state.global.budget_id, id)
            })?
            .data()
            .transaction()
            .clone();
        let new = make_save_transaction(state, old)?;
        models::SaveTransactionWrapper::new(new)
    };
    let response: models::TransactionResponse = state.run(&|c| {
        c.transactions_api().update_transaction(
            &state.global.budget_id,
            id,
            model.clone(), // @@@ NOT SURE WHY WE NEED TO CLONE HERE
        )
    })?;
    // @@@ PRINT "UPDATED" MESSAGE (WITH LOGGING INFO LEVEL)
    vtable_output(
        state,
        response.data(),
        response.data().transaction(),
        &make_transaction_table,
    )
}

pub fn create_transaction(state: &YnabState) -> Result<(), AnyError> {
    let model: models::SaveTransactionsWrapper = if let Some(m) = file_input(state)? {
        m
    } else {
        let old = models::TransactionDetail::new(
            "".to_string(),
            NaiveDate::from_ymd(1970, 1, 1),
            models::Milliunits::new(0),
            models::Cleared::Uncleared,
            false,
            "".to_string(),
            false,
            "".to_string(),
            Vec::new(),
        );
        let new = make_save_transaction(state, old)?;
        let mut model = models::SaveTransactionsWrapper::new();
        model.set_transaction(new);
        model
    };
    let response: models::SaveTransactionsResponse = state.run(&|c| {
        c.transactions_api()
            .create_transaction(&state.global.budget_id, model.clone())
    })?;
    // @@@ PRINT "UPDATED" MESSAGE (WITH LOGGING INFO LEVEL)
    if let Some(transaction) = response.data().transaction() {
        vtable_output(state, response.data(), transaction, &make_transaction_table)
    } else if let Some(transactions) = response.data().transactions() {
        htable_output(
            state,
            response.data(),
            transactions,
            &make_transactions_table,
        )
    } else {
        Ok(())
    }
}

fn transaction_details_output(
    state: &YnabState,
    data: &models::TransactionsWrapper,
    include_subtransactions: bool,
) -> Result<(), AnyError> {
    let mut wrapper = data.clone();
    let mut transactions: Vec<models::TransactionDetail> = Vec::new();
    for tr in data.transactions() {
        let mut tr: models::TransactionDetail = (*tr).clone();
        if !include_subtransactions {
            tr.set_subtransactions(vec![]);
        }
        transactions.push(tr);
    }
    wrapper.set_transactions(transactions.clone());
    htable_output(state, &wrapper, &transactions, &make_transactions_table)
}

fn hybrid_transactions_output(
    state: &YnabState,
    data: &models::HybridTransactionsWrapper,
    include_subtransactions: bool,
) -> Result<(), AnyError> {
    let mut wrapper = data.clone();
    let mut transactions: Vec<models::HybridTransaction> = Vec::new();
    for tr in data.transactions() {
        if include_subtransactions || *tr._type() == models::HybridTransactionType::Transaction {
            transactions.push((*tr).clone());
        }
    }
    wrapper.set_transactions(transactions.clone());
    htable_output(
        state,
        &wrapper,
        &transactions,
        &make_hybrid_transaction_table,
    )
}

pub fn list_transactions(state: &YnabState) -> Result<(), AnyError> {
    let settings = state.get_budget_settings()?;
    // @@@ AN EMPTY VALUE SHOULD BE THE SAME AS THE OPTION NOT BEING PROVIDED, EVERYWHERE
    let since_date = opt_date_value_of(&settings, state.matches, SINCE_DATE_ARG)?;
    let transaction_type = req_transaction_type_value_of(state.matches, TRANSACTION_TYPE_ARG);
    let last_knowledge_of_server = opt_last_knowledge_of_server_value_of(state.matches);
    let include_subtransactions: bool =
        req_parse_value_of(state.matches, INCLUDE_SUBTRANSACTIONS_ARG);
    let opt_account_id = state.matches.value_of(ACCOUNT_ID_ARG);
    let opt_payee_id = state.matches.value_of(PAYEE_ID_ARG);
    let opt_category_id = state.matches.value_of(CATEGORY_ID_ARG);
    if let Some(account_id) = opt_account_id {
        let response = state.run(&|c| {
            c.transactions_api().get_transactions_by_account(
                &state.global.budget_id,
                account_id,
                since_date,
                transaction_type,
                last_knowledge_of_server,
            )
        })?;
        transaction_details_output(state, response.data(), include_subtransactions)
    } else if let Some(payee_id) = opt_payee_id {
        let response = state.run(&|c| {
            c.transactions_api().get_transactions_by_payee(
                &state.global.budget_id,
                payee_id,
                since_date,
                transaction_type,
                last_knowledge_of_server,
            )
        })?;
        hybrid_transactions_output(state, response.data(), include_subtransactions)
    } else if let Some(category_id) = opt_category_id {
        let response = state.run(&|c| {
            c.transactions_api().get_transactions_by_category(
                &state.global.budget_id,
                category_id,
                since_date,
                transaction_type,
                last_knowledge_of_server,
            )
        })?;
        hybrid_transactions_output(state, response.data(), include_subtransactions)
    } else {
        let response = state.run(&|c| {
            c.transactions_api().get_transactions(
                &state.global.budget_id,
                since_date,
                transaction_type,
                last_knowledge_of_server,
            )
        })?;
        transaction_details_output(state, response.data(), include_subtransactions)
    }
}

fn transaction_detail_cell(
    settings: &models::BudgetSettings,
    tr: &models::TransactionDetail,
    col: &TransactionCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        TransactionCol::Id => Cell::new(tr.id()),
        TransactionCol::Date => Cell::new(&date_str(settings, tr.date())),
        TransactionCol::Amount => {
            Cell::new_align(&milliunits_str(settings, tr.amount()), currency_alignment)
        }
        TransactionCol::Memo => Cell::new(&opt_ref_str(tr.memo())),
        TransactionCol::Cleared => Cell::new(&tr.cleared().to_string()),
        TransactionCol::Approved => Cell::new(&tr.approved().to_string()),
        TransactionCol::FlagColor => Cell::new(&opt_to_str(tr.flag_color())),
        TransactionCol::AccountId => Cell::new(tr.account_id()),
        TransactionCol::PayeeId => Cell::new(&opt_ref_str(tr.payee_id())),
        TransactionCol::CategoryId => Cell::new(&opt_ref_str(tr.category_id())),
        TransactionCol::TransferAccountId => Cell::new(&opt_ref_str(tr.transfer_account_id())),
        TransactionCol::TransferTransactionId => {
            Cell::new(&opt_ref_str(tr.transfer_transaction_id()))
        }
        TransactionCol::ImportId => Cell::new(&opt_ref_str(tr.import_id())),
        TransactionCol::Deleted => Cell::new(&tr.deleted().to_string()),
        TransactionCol::Type => Cell::new(&models::HybridTransactionType::Transaction.to_string()),
        TransactionCol::ParentTransactionId => Cell::new(""),
        TransactionCol::AccountName => Cell::new(tr.account_name()),
        TransactionCol::PayeeName => Cell::new(&opt_ref_str(tr.payee_name())),
        TransactionCol::CategoryName => Cell::new(&opt_ref_str(tr.category_name())),
    }
}

fn subtransaction_cell(
    settings: &models::BudgetSettings,
    parent: &models::TransactionDetail,
    sub: &models::SubTransaction,
    col: &TransactionCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        TransactionCol::Id => Cell::new(sub.id()),
        TransactionCol::Date => Cell::new(""),
        TransactionCol::Amount => {
            Cell::new_align(&milliunits_str(settings, sub.amount()), currency_alignment)
        }
        TransactionCol::Memo => Cell::new(&opt_ref_str(sub.memo())),
        TransactionCol::Cleared => Cell::new(""),
        TransactionCol::Approved => Cell::new(""),
        TransactionCol::FlagColor => Cell::new(""),
        TransactionCol::AccountId => Cell::new(""),
        TransactionCol::PayeeId => Cell::new(&opt_ref_str(sub.payee_id())),
        TransactionCol::CategoryId => Cell::new(&opt_ref_str(sub.category_id())),
        TransactionCol::TransferAccountId => Cell::new(&opt_ref_str(sub.transfer_account_id())),
        TransactionCol::TransferTransactionId => Cell::new(""),
        TransactionCol::ImportId => Cell::new(""),
        TransactionCol::Deleted => Cell::new(&sub.deleted().to_string()),
        TransactionCol::Type => {
            Cell::new(&models::HybridTransactionType::Subtransaction.to_string())
        }
        TransactionCol::ParentTransactionId => Cell::new(parent.id()),
        TransactionCol::AccountName => Cell::new(""),
        // @@@ TODO USE PAYEE/CATEGORY APIS TO LOOK UP PAYEE/CATEGORY NAMES (BUT NEED TO CACHE)
        TransactionCol::PayeeName => Cell::new(""),
        TransactionCol::CategoryName => Cell::new(""),
    }
}

fn hybrid_transaction_cell(
    settings: &models::BudgetSettings,
    tr: &models::HybridTransaction,
    col: &TransactionCol,
    currency_alignment: Alignment,
) -> Cell {
    match col {
        TransactionCol::Id => Cell::new(tr.id()),
        TransactionCol::Date => Cell::new(&date_str(settings, tr.date())),
        TransactionCol::Amount => {
            Cell::new_align(&milliunits_str(settings, tr.amount()), currency_alignment)
        }
        TransactionCol::Memo => Cell::new(&opt_ref_str(tr.memo())),
        TransactionCol::Cleared => Cell::new(&tr.cleared().to_string()),
        TransactionCol::Approved => Cell::new(&tr.approved().to_string()),
        TransactionCol::FlagColor => Cell::new(&opt_to_str(tr.flag_color())),
        TransactionCol::AccountId => Cell::new(tr.account_id()),
        TransactionCol::PayeeId => Cell::new(&opt_ref_str(tr.payee_id())),
        TransactionCol::CategoryId => Cell::new(&opt_ref_str(tr.category_id())),
        TransactionCol::TransferAccountId => Cell::new(&opt_ref_str(tr.transfer_account_id())),
        TransactionCol::TransferTransactionId => {
            Cell::new(&opt_ref_str(tr.transfer_transaction_id()))
        }
        TransactionCol::ImportId => Cell::new(&opt_ref_str(tr.import_id())),
        TransactionCol::Deleted => Cell::new(&tr.deleted().to_string()),
        TransactionCol::Type => Cell::new(&tr._type().to_string()),
        TransactionCol::ParentTransactionId => Cell::new(&opt_ref_str(tr.parent_transaction_id())),
        TransactionCol::AccountName => Cell::new(tr.account_name()),
        TransactionCol::PayeeName => Cell::new(&opt_ref_str(tr.payee_name())),
        TransactionCol::CategoryName => Cell::new(&opt_ref_str(tr.category_name())),
    }
}

fn make_transactions_table(
    state: &YnabState,
    columns: &Vec<TransactionCol>,
    transactions: &Vec<models::TransactionDetail>,
) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let alignment = &|col: &TransactionCol| match *col {
        TransactionCol::Amount => Alignment::RIGHT,
        _ => Alignment::LEFT,
    };
    let mut table = make_htable(state, columns, alignment);
    for tr in transactions {
        table.add_row(Row::new(
            columns
                .iter()
                .map(|col| transaction_detail_cell(&settings, tr, col, Alignment::RIGHT))
                .collect(),
        ));
        for sub in tr.subtransactions() {
            table.add_row(Row::new(
                columns
                    .iter()
                    .map(|col| subtransaction_cell(&settings, tr, sub, col, Alignment::RIGHT))
                    .collect(),
            ));
        }
    }
    Ok(table)
}

fn make_hybrid_transaction_table(
    state: &YnabState,
    columns: &Vec<TransactionCol>,
    transactions: &Vec<models::HybridTransaction>,
) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let alignment = &|col: &TransactionCol| match *col {
        TransactionCol::Amount => Alignment::RIGHT,
        _ => Alignment::LEFT,
    };
    let mut table = make_htable(state, columns, alignment);
    for tr in transactions {
        table.add_row(Row::new(
            columns
                .iter()
                .map(|col| hybrid_transaction_cell(&settings, tr, col, Alignment::RIGHT))
                .collect(),
        ));
    }
    Ok(table)
}

fn make_transaction_table(
    state: &YnabState,
    tr: &models::TransactionDetail,
) -> Result<Table, AnyError> {
    let settings = state.get_budget_settings()?;
    let mut table = make_vtable(state);
    for col in TransactionCol::iter() {
        table.add_row(Row::new(vec![
            vfield_cell(&col.to_string()),
            transaction_detail_cell(&settings, tr, &col, Alignment::LEFT),
        ]));
    }
    for sub in tr.subtransactions() {
        table.add_row(Row::new(vec![Cell::new(""), Cell::new("")]));
        table.add_row(Row::new(vec![Cell::new("SUBTRANSACTION"), Cell::new("")]));
        for col in SUBTRANSACTION_COLS.iter() {
            table.add_row(Row::new(vec![
                vfield_cell(&col.to_string()),
                subtransaction_cell(&settings, tr, sub, col, Alignment::LEFT),
            ]));
        }
    }
    Ok(table)
}
