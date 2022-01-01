use std::error;
use std::fmt;

use constants::*;

#[derive(Debug)]
pub enum Error {
    MissingAccessToken,
    GetBudgetOnlyJson,
    InvalidCurrencyAmountArg(String, String, std::num::ParseFloatError),
    InvalidDateArg(String, String, chrono::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingAccessToken => write!(f, "Missing Personal Access Token for YNAB API.  Use --{} or {} to set (see --help for more information).", ACCESS_TOKEN_ARG, ACCESS_TOKEN_ENV),
            Error::GetBudgetOnlyJson => write!(f, "'get budget' only supports JSON output (use '--output=json' argument).\n\tWARNING: This command may output a huge amount of data!"),
            Error::InvalidCurrencyAmountArg(name, val, err) => write!(f, "Invalid currency amount for '--{}' (\"{}\"): {}", name, val, err.to_string()),
            Error::InvalidDateArg(name, val, err) => write!(f, "Invalid date for '--{}' (\"{}\"): {}", name, val, err.to_string()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "YNAB CLI error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

//@@@ WE SHOULD PROBABLY NOT USE THIS, INSTEAD MOVE ALL ERRORS INTO 'enum Error'
pub type AnyError = Box<dyn error::Error>;

#[derive(EnumString, Display, EnumIter, Debug)]
pub enum OutputFormat {
    #[strum(serialize = "table")]
    Table,
    #[strum(serialize = "csv")]
    Csv,
    #[strum(serialize = "json")]
    Json,
}

#[derive(Clone, EnumIter, EnumString, Display, Debug)]
pub enum AccountCol {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "on-budget")]
    OnBudget,
    #[strum(serialize = "closed")]
    Closed,
    #[strum(serialize = "note")]
    Note,
    #[strum(serialize = "balance")]
    Balance,
    #[strum(serialize = "cleared-balance")]
    ClearedBalance,
    #[strum(serialize = "uncleared-balance")]
    UnclearedBalance,
    #[strum(serialize = "transfer-payee-id")]
    TransferPayeeId,
    #[strum(serialize = "deleted")]
    Deleted,
}

#[derive(Clone, EnumIter, EnumString, Display, Debug)]
pub enum PayeeLocationCol {
    #[strum(serialize = "id")]
    Id,
    // @@@ TODO: able to lookup payee name from ID in UI
    #[strum(serialize = "payee-id")]
    PayeeId,
    #[strum(serialize = "latitude")]
    Latitude,
    #[strum(serialize = "longitude")]
    Longitude,
    #[strum(serialize = "deleted")]
    Deleted,
}

#[derive(Clone, EnumIter, EnumString, Display, Debug)]
pub enum PayeeCol {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "transfer-account-id")]
    TransferAccountId,
    #[strum(serialize = "deleted")]
    Deleted,
}

// @@@ CHANGE 'Col' TO 'Field' IN MOST PLACES
#[derive(Clone, EnumIter, EnumString, Display, Debug)]
pub enum BudgetCol {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "last-modified")]
    LastModified,
    #[strum(serialize = "date-format")]
    DateFormat,
    #[strum(serialize = "currency-iso-code")]
    CurrencyIsoCode,
    #[strum(serialize = "currency-example-format")]
    CurrencyExampleFormat,
    #[strum(serialize = "currency-decimal-digits")]
    CurrencyDecimalDigits,
    #[strum(serialize = "currency-decimal-separator")]
    CurrencyDecimalSeparator,
    #[strum(serialize = "currency-symbol-first")]
    CurrencySymbolFirst,
    #[strum(serialize = "currency-group-separator")]
    CurrencyGroupSeparator,
    #[strum(serialize = "currency-symbol")]
    CurrencySymbol,
    #[strum(serialize = "currency-display-symbol")]
    CurrencyDisplaySymbol,
}

#[derive(Clone, EnumIter, EnumString, Display, Debug)]
pub enum CategoryCol {
    #[strum(serialize = "group-id")]
    GroupId,
    #[strum(serialize = "group-name")]
    GroupName,
    #[strum(serialize = "group-hidden")]
    GroupHidden,
    #[strum(serialize = "group-deleted")]
    GroupDeleted,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "hidden")]
    Hidden,
    #[strum(serialize = "original-category-group-id")]
    OriginalCategoryGroupId,
    #[strum(serialize = "note")]
    Note,
    #[strum(serialize = "budgeted")]
    Budgeted,
    #[strum(serialize = "activity")]
    Activity,
    #[strum(serialize = "balance")]
    Balance,
    #[strum(serialize = "goal-type")]
    GoalType,
    #[strum(serialize = "goal-creation-month")]
    GoalCreationMonth,
    #[strum(serialize = "goal-target")]
    GoalTarget,
    #[strum(serialize = "goal-target-month")]
    GoalTargetMonth,
    #[strum(serialize = "goal-percentage-complete")]
    GoalPercentageComplete,
    #[strum(serialize = "deleted")]
    Deleted,
}

#[derive(Clone, EnumIter, EnumString, Display, Debug)]
pub enum TransactionCol {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "date")]
    Date,
    #[strum(serialize = "amount")]
    Amount,
    #[strum(serialize = "memo")]
    Memo,
    #[strum(serialize = "cleared")]
    Cleared,
    #[strum(serialize = "approved")]
    Approved,
    #[strum(serialize = "flag-color")]
    FlagColor,
    #[strum(serialize = "account-id")]
    AccountId,
    #[strum(serialize = "payee-id")]
    PayeeId,
    #[strum(serialize = "category-id")]
    CategoryId,
    #[strum(serialize = "transfer-account-id")]
    TransferAccountId,
    #[strum(serialize = "transfer-transaction-id")]
    TransferTransactionId,
    #[strum(serialize = "import-id")]
    ImportId,
    #[strum(serialize = "deleted")]
    Deleted,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "parent-transaction-id")]
    ParentTransactionId,
    #[strum(serialize = "account-name")]
    AccountName,
    #[strum(serialize = "payee-name")]
    PayeeName,
    #[strum(serialize = "category-name")]
    CategoryName,
}

#[derive(Clone, EnumIter, EnumString, Display, Debug)]
pub enum ScheduledTransactionCol {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "date-first")]
    DateFirst,
    #[strum(serialize = "date-next")]
    DateNext,
    #[strum(serialize = "frequency")]
    Frequency,
    #[strum(serialize = "amount")]
    Amount,
    #[strum(serialize = "memo")]
    Memo,
    #[strum(serialize = "flag-color")]
    FlagColor,
    #[strum(serialize = "account-id")]
    AccountId,
    #[strum(serialize = "payee-id")]
    PayeeId,
    #[strum(serialize = "category-id")]
    CategoryId,
    #[strum(serialize = "transfer-account-id")]
    TransferAccountId,
    #[strum(serialize = "deleted")]
    Deleted,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "parent-transaction-id")]
    ParentTransactionId,
    #[strum(serialize = "account-name")]
    AccountName,
    #[strum(serialize = "payee-name")]
    PayeeName,
    #[strum(serialize = "category_name")]
    CategoryName,
}

#[derive(Clone, EnumIter, EnumString, Display, Debug)]
pub enum MonthCol {
    #[strum(serialize = "month")]
    Month,
    #[strum(serialize = "note")]
    Note,
    #[strum(serialize = "income")]
    Income,
    #[strum(serialize = "budgeted")]
    Budgeted,
    #[strum(serialize = "activity")]
    Activity,
    #[strum(serialize = "to-be-budgeted")]
    ToBeBudgeted,
    #[strum(serialize = "age-of-money")]
    AgeOfMoney,
}
