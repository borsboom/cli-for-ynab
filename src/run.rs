use std::io;
use std::str::FromStr;

use accounts::*;
use args::*;
use budgets::*;
use categories::*;
use constants::*;
use months::*;
use payees::*;
use scheduled_transactions::*;
use transactions::*;
use types::*;
use user::*;
use ynab_state::*;

pub fn run(
    prog_name: &str,
    // default_options: GlobalOptions,
    super_matches: clap::ArgMatches<'static>,
) -> Result<(), AnyError> {
    match super_matches.subcommand() {
        (COMPLETIONS_CMD, Some(matches)) => {
            let shell = matches
                .value_of(SHELL_ARG)
                .expect(&format!("Expected {} argument to exist", SHELL_ARG));
            build_clap_app().gen_completions_to(
                prog_name,
                clap::Shell::from_str(shell).expect(&format!(
                    "Expected {} argument to be valid: {}",
                    SHELL_ARG, shell
                )),
                &mut io::stdout(),
            );
        }
        (LIST_CMD, Some(matches)) => {
            match matches.subcommand() {
                (ACCOUNTS_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    list_accounts(&state)?;
                }
                (PAYEES_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    list_payees(&state)?;
                }
                (PAYEE_LOCATIONS_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    list_payee_locations(&state)?;
                }
                (MONTHS_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    list_months(&state)?;
                }
                (BUDGETS_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    list_budgets(&state)?;
                }
                (CATEGORIES_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    list_categories(&state)?;
                }
                (TRANSACTIONS_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    list_transactions(&state)?;
                }
                (SCHEDULED_TRANSACTIONS_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    list_scheduled_transactions(&state)?;
                }
                // panic!() is safe because clap enforces a subcommand
                (c, _) => panic!("Expected valid subcommand: {}", c),
            }
        }
        (GET_CMD, Some(matches)) => {
            match matches.subcommand() {
                (USER_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_user(&state)?;
                }
                (BUDGET_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_budget(&state)?;
                }
                (BUDGET_SETTINGS_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_budget_settings(&state)?;
                }
                (CATEGORY_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_category(&state)?;
                }
                (ACCOUNT_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_account(&state)?;
                }
                (PAYEE_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_payee(&state)?;
                }
                (PAYEE_LOCATION_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_payee_location(&state)?;
                }
                (MONTH_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_month(&state)?;
                }
                (TRANSACTION_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_transaction(&state)?;
                }
                (SCHEDULED_TRANSACTION_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    get_scheduled_transaction(&state)?;
                }
                // panic!() is safe because clap enforces a subcommand
                (c, _) => panic!("Expected valid subcommand: {}", c),
            }
        }
        (UPDATE_CMD, Some(matches)) => {
            match matches.subcommand() {
                (CATEGORY_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    update_category(&state)?;
                }
                (TRANSACTION_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    update_transaction(&state)?;
                }
                // panic!() is safe because clap enforces a subcommand
                (c, _) => panic!("Expected valid subcommand: {}", c),
            }
        }
        (CREATE_CMD, Some(matches)) => {
            match matches.subcommand() {
                (TRANSACTION_CMD, Some(matches)) => {
                    let state = YnabState::new(&super_matches, matches)?;
                    create_transaction(&state)?;
                }
                // panic!() is safe because clap enforces a subcommand
                (c, _) => panic!("Expected valid subcommand: {}", c),
            }
        }
        // (VERSION_CMD, Some(_)) => {
        //     let out = io::stdout();
        //     let mut buf_w = io::BufWriter::new(out.lock());
        //     build_clap_app().write_long_version(&mut buf_w)?;
        //     buf_w.write(b"\n")?;
        // }
        // panic!() is safe because clap enforces a subcommand
        (c, _) => panic!("Expected valid subcommand: {}", c),
    }

    // let accounts = core.run(ynab.accounts_api().get_accounts(&budget_id))?;
    // println!("@@@ accounts={:?}", accounts);
    // let transactions = make_zeroing_transactions(&accounts.data().accounts());
    // if transactions.len() > 0 {
    //     let response = core.run(ynab.transactions_api().create_transaction(&budget_id, models::SaveTransactionsWrapper::new().with_transactions(transactions)))?;
    //     println!("Wrote transaction IDs: {:?}", response.data().transaction_ids());
    // } else {
    //     println!("Nothing to do!");
    // }

    Ok(())
}

// fn zeroing_transaction(account: &models::Account, today: NaiveDate) -> models::SaveTransaction {
//     println!("Zeroing {}", account.name());
//     models::SaveTransaction::new (
//         account.id().clone(), //account_id
//         today, //date
//         -account.balance())
//       .with_payee_name("Manual Balance Adjustment".to_string())
//       .with_memo("[1900] Zeroing transaction".to_string())
//       .with_approved(true)
// }

// fn make_zeroing_transactions(accounts: &Vec<models::Account>) -> Vec<models::SaveTransaction> {
//     let today = Local::today().naive_local();
//     accounts
//         .iter()
//         .filter(|a| a.name().ends_with("|") && a.balance() != &0)
//         .map(move |a| zeroing_transaction(a, today))
//         .collect()
// }
