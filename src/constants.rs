use prettytable::format::{FormatBuilder, TableFormat};
use strum::IntoEnumIterator;
use ynab_api::models;

use crate::types::*;

pub const GET_CMD: &str = "get";
pub const COMPLETIONS_CMD: &str = "completions";
pub const SHELL_ARG: &str = "shell";
pub const ACCESS_TOKEN_ARG: &str = "access-token";
pub const BUDGET_ID_ARG: &str = "budget-id";
pub const DEFAULT_BUDGET_ID_VAL: &str = "last-used";
pub const BUDGETS_CMD: &str = "budgets";
pub const HEADERS_ARG: &str = "headers";
pub const BORDERS_ARG: &str = "borders";
pub const OUTPUT_ARG: &str = "output";
pub const TABLE_COLUMNS_ARG: &str = "columns";
pub const BUDGET_CMD: &str = "budget";
pub const LAST_KNOWLEDGE_OF_SERVER_ARG: &str = "last-knowledge-of-server";
pub const CATEGORIES_CMD: &str = "categories";
// pub const INCLUDE_HIDDEN_ARG: &str = "include-hidden";
// pub const INCLUDE_CLOSED_ARG: &str = "include-closed";
pub const ALL_VAL: &str = "all";
pub const BUDGET_SETTINGS_CMD: &str = "budget-settings";
pub const CATEGORY_CMD: &str = "category";
pub const ID_ARG: &str = "id";
pub const MONTH_ARG: &str = "month";
pub const UPDATE_CMD: &str = "update";
pub const CURRENT_MONTH_VAL: &str = "current";
pub const SET_BUDGETED_ARG: &str = "set-budgeted";
pub const MONTH_FORMAT: &str = "%b %Y";
pub const MONTH_VAL_FORMATS: [&str; 4] = ["%d %b %Y", "%d %Y %b", "%d %Y %m", "%d %m %Y"];
pub const FILE_INPUT_ARG: &str = "file";
pub const TRANSACTIONS_CMD: &str = "transactions";
pub const SCHEDULED_TRANSACTIONS_CMD: &str = "scheduled-transactions";
pub const SINCE_DATE_ARG: &str = "since";
pub const TRANSACTION_TYPE_ARG: &str = "type";
pub const INCLUDE_SUBTRANSACTIONS_ARG: &str = "include-subtransactions";
pub const TRANSACTION_CMD: &str = "transaction";
pub const SCHEDULED_TRANSACTION_CMD: &str = "scheduled-transaction";
pub const SET_ACCOUNT_ID_ARG: &str = "set-account-id";
pub const SET_DATE_ARG: &str = "set-date";
pub const SET_AMOUNT_ARG: &str = "set-amount";
pub const SET_PAYEE_ID_ARG: &str = "set-payee-id";
pub const PAYEE_ID_ARG: &str = "payee-id";
pub const SET_PAYEE_NAME_ARG: &str = "set-payee-name";
pub const SET_CATEGORY_ID_ARG: &str = "set-category-id";
pub const SET_MEMO_ARG: &str = "set-memo";
pub const SET_CLEARED_ARG: &str = "set-cleared";
pub const SET_APPROVED_ARG: &str = "set-approved";
pub const SET_FLAG_COLOR_ARG: &str = "set-flag-color";
pub const SET_IMPORT_ID_ARG: &str = "set-import-id";
pub const NONE_VAL: &str = "none";
pub const CREATE_CMD: &str = "create";
pub const LIST_CMD: &str = "list";
pub const USER_CMD: &str = "user";
pub const ACCOUNTS_CMD: &str = "accounts";
pub const ACCOUNT_CMD: &str = "account";
pub const SUBTRANSACTION_COLS: [TransactionCol; 9] = [
    TransactionCol::Id,
    TransactionCol::Amount,
    TransactionCol::Memo,
    TransactionCol::PayeeId,
    TransactionCol::CategoryId,
    TransactionCol::TransferAccountId,
    TransactionCol::Deleted,
    TransactionCol::Type,
    TransactionCol::ParentTransactionId,
];
pub const SCHEDULED_SUBTRANSACTION_COLS: [ScheduledTransactionCol; 9] = [
    ScheduledTransactionCol::Id,
    ScheduledTransactionCol::Amount,
    ScheduledTransactionCol::Memo,
    ScheduledTransactionCol::PayeeId,
    ScheduledTransactionCol::CategoryId,
    ScheduledTransactionCol::TransferAccountId,
    ScheduledTransactionCol::Deleted,
    ScheduledTransactionCol::Type,
    ScheduledTransactionCol::ParentTransactionId,
];
pub const PAYEES_CMD: &str = "payees";
pub const PAYEE_CMD: &str = "payee";
pub const PAYEE_LOCATIONS_CMD: &str = "payee-locations";
pub const PAYEE_LOCATION_CMD: &str = "payee-location";
pub const MONTHS_CMD: &str = "months";
pub const MONTH_CMD: &str = "month";
pub const INCLUDE_CATEGORIES_ARG: &str = "include-categories";
pub const CATEGORY_NON_GROUP_COLS: [CategoryCol; 14] = [
    CategoryCol::Id,
    CategoryCol::Name,
    CategoryCol::Hidden,
    CategoryCol::OriginalCategoryGroupId,
    CategoryCol::Note,
    CategoryCol::Budgeted,
    CategoryCol::Activity,
    CategoryCol::Balance,
    CategoryCol::GoalType,
    CategoryCol::GoalCreationMonth,
    CategoryCol::GoalTarget,
    CategoryCol::GoalTargetMonth,
    CategoryCol::GoalPercentageComplete,
    CategoryCol::Deleted,
];
pub const ACCOUNT_ID_ARG: &str = "account-id";
pub const CATEGORY_ID_ARG: &str = "category-id";
pub const ACCESS_TOKEN_ENV: &str = "YNAB_ACCESS_TOKEN";

lazy_static! {
    pub static ref DEFAULT_HEADERS_VAL: String = true.to_string();
    pub static ref DEFAULT_BORDERS_VAL: String = false.to_string();
    pub static ref DEFAULT_OUTPUT_VAL: String = OutputFormat::Table.to_string();
    pub static ref BUDGET_COL_STRINGS: Vec<String> =
        BudgetCol::iter().map(|v| v.to_string()).collect();
    pub static ref CATEGORY_COL_STRINGS: Vec<String> =
        CategoryCol::iter()
            .map(|v| v.to_string())
            .collect();
    pub static ref TRANSACTION_COL_STRINGS: Vec<String> =
        TransactionCol::iter().map(|v| v.to_string()).collect();
    pub static ref SCHEDULED_TRANSACTION_COL_STRINGS: Vec<String> =
        ScheduledTransactionCol::iter().map(|v| v.to_string()).collect();
    pub static ref ACCOUNT_COL_STRINGS: Vec<String> =
        AccountCol::iter().map(|v| v.to_string()).collect();
    pub static ref PAYEE_COL_STRINGS: Vec<String> =
        PayeeCol::iter().map(|v| v.to_string()).collect();
    pub static ref PAYEE_LOCATION_COL_STRINGS: Vec<String> =
        PayeeLocationCol::iter().map(|v| v.to_string()).collect();
    pub static ref MONTH_COL_STRINGS: Vec<String> =
        MonthCol::iter().map(|v| v.to_string()).collect();
    pub static ref TABLE_NO_BORDERS_FORMAT: TableFormat =
        FormatBuilder::new().column_separator(' ').build();
    pub static ref OUTPUT_FORMAT_STRINGS: Vec<String> =
        OutputFormat::iter().map(|v| v.to_string()).collect();
    pub static ref TRUE_STRING: String = true.to_string();
    pub static ref FALSE_STRING: String = false.to_string();
    // @@@ THESE SHOULD END WITH '_DEFAULT_COL_VALS'
    pub static ref BUDGET_DEFAULT_COLS: String = join_cols(&[
        BudgetCol::Id,
        BudgetCol::Name,
        BudgetCol::LastModified,
        BudgetCol::CurrencyIsoCode,
    ]);
    pub static ref CATEGORY_DEFAULT_COLS: String = join_cols(&[
        CategoryCol::GroupName,
        CategoryCol::Id,
        CategoryCol::Name,
        CategoryCol::Budgeted,
        CategoryCol::Activity,
        CategoryCol::Balance,
        CategoryCol::Hidden,
    ]);
    pub static ref TRANSACTION_DEFAULT_COLS: String = join_cols(&[
        TransactionCol::Id,
        TransactionCol::AccountName,
        TransactionCol::Approved,
        TransactionCol::Date,
        TransactionCol::PayeeName,
        TransactionCol::CategoryName,
        TransactionCol::Memo,
        TransactionCol::Amount,
        TransactionCol::Cleared,
    ]);
    pub static ref SCHEDULED_TRANSACTION_DEFAULT_COLS: String = join_cols(&[
        ScheduledTransactionCol::Id,
        ScheduledTransactionCol::AccountName,
        ScheduledTransactionCol::DateNext,
        ScheduledTransactionCol::Frequency,
        ScheduledTransactionCol::PayeeName,
        ScheduledTransactionCol::CategoryName,
        ScheduledTransactionCol::Memo,
        ScheduledTransactionCol::Amount,
    ]);
    pub static ref ACCOUNT_DEFAULT_COLS: String = join_cols(&[
        AccountCol::Id,
        AccountCol::Name,
        AccountCol::Type,
        AccountCol::OnBudget,
        AccountCol::Balance,
        AccountCol::ClearedBalance,
        AccountCol::UnclearedBalance,
        AccountCol::Closed,
    ]);
    pub static ref PAYEE_DEFAULT_COLS: String = join_cols(&[
        PayeeCol::Id,
        PayeeCol::Name,
        PayeeCol::TransferAccountId,
    ]);
    pub static ref PAYEE_LOCATION_DEFAULT_COLS: String = join_cols(&[
        PayeeLocationCol::Id,
        PayeeLocationCol::PayeeId,
        PayeeLocationCol::Latitude,
        PayeeLocationCol::Longitude,
    ]);
    pub static ref MONTH_DEFAULT_COLS: String = join_cols(&[
        MonthCol::Month,
        MonthCol::Note,
        MonthCol::Income,
        MonthCol::Budgeted,
        MonthCol::Activity,
        MonthCol::ToBeBudgeted,
        MonthCol::AgeOfMoney,
    ]);
    pub static ref TABLE_WITH_BORDERS_FORMAT: TableFormat = *prettytable::format::consts::FORMAT_DEFAULT;
    // @@@     if atty::is(atty::Stream::Stdout) {
    // @@@         FormatBuilder::new()
    // @@@             .column_separator('│')
    // @@@             .borders('│')
    // @@@             .separators(
    // @@@                 &[LinePosition::Top],
    // @@@                 LineSeparator::new('─', '┬', '┌', '┐'),
    // @@@             ).separators(
    // @@@                 &[LinePosition::Title],
    // @@@                 LineSeparator::new('─', '┼', '├', '┤'),
    // @@@             ).separators(
    // @@@                 &[LinePosition::Bottom],
    // @@@                 LineSeparator::new('─', '┴', '└', '┘'),
    // @@@             ).padding(1, 1)
    // @@@             .build()
    // @@@     } else {
    // @@@         *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE
    // @@@     }
    // @@@ };
    pub static ref SENTENCE_TO_UPPERCASE_RE: regex::Regex = regex::Regex::new(r"(\b(id|iso)\b|^[a-z])").expect("Expected SENTENCE_TO_UPPER_CASE_RE to be valid");
    pub static ref TRANSACTION_TYPE_VALS: Vec<String> =
        models::TransactionType::iter().map(|v| v.to_string()).collect();
    pub static ref CLEARED_VALS: Vec<String> =
        models::Cleared::iter().map(|v| v.to_string()).collect();
    pub static ref FLAG_COLOR_VALS: Vec<String> =
        models::FlagColor::iter().map(|v| v.to_string()).collect();
}

pub fn join_cols<C>(cols: &[C]) -> String
where
    C: ToString,
{
    let vals: Vec<_> = cols.iter().map(|v| v.to_string()).collect();
    vals.join(",")
}
