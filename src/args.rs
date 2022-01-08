use chrono::prelude::*;
use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches, SubCommand};
use std::fmt;
use std::fs;
use std::io;
use std::str::FromStr;
use ynab_api::models;

use crate::constants::*;
use crate::types::*;
use crate::ynab_state::*;

// struct ArgsBuilder<'a> {
//     phantom: std::marker::PhantomData<&'a ()>, //@@@ NOT SURE IF USING RIGHT
//     BUDGET_COL_STRINGS: Vec<String>,
//     BUDGET_DEFAULT_COLS: String,
//
// }
//
// impl<'a> ArgsBuilder<'a> {
//     fn new() -> ArgsBuilder<'a> {
//         ArgsBuilder {
//             phantom: std::marker::PhantomData,
//             BUDGET_COL_STRINGS:
//                 BudgetCol::iter().map(|v| v.to_string()).collect(),
//             BUDGET_DEFAULT_COLS: join_cols(&[
//                 BudgetCol::Id.to_string(),
//                 BudgetCol::Name.to_string(),
//                 BudgetCol::LastModified.to_string(),
//                 BudgetCol::CurrencyIsoCode.to_string(),
//             ]),
//         }
//     }
//
//     fn clap_app(&'a self) -> App<'a, 'a> {
//         GlobalOptions::args(GlobalFlags::ALL, super_command_settings(app_from_crate!()))
//                         .arg(table_columns_arg(
//                             &self.BUDGET_COL_STRINGS,
//                             &self.BUDGET_DEFAULT_COLS,
//                         ))
//
//     }
// }

pub fn is_i64(v: String) -> Result<(), String> {
    map_parse_to_validator(i64::from_str(&v))
}

pub fn is_month(val: String) -> Result<(), String> {
    map_parse_to_validator(parse_month(&val))
}

fn map_parse_to_validator<T, E>(x: Result<T, E>) -> Result<(), String>
where
    E: ToString,
{
    x.map(|_| ()).map_err(|e| e.to_string())
}

pub fn req_value_of<'a>(matches: &'a ArgMatches<'static>, name: &str) -> &'a str {
    // Use of 'expect' is safe because clap has validated input
    matches
        .value_of(name)
        .unwrap_or_else(|| panic!("Expected {} argument to exist", name))
}

pub fn req_parse_value_of<T: FromStr>(matches: &ArgMatches<'static>, name: &str) -> T
where
    <T as FromStr>::Err: fmt::Debug,
{
    let val = req_value_of(matches, name);
    // Use of 'expect' is safe because clap has validated input
    T::from_str(val).unwrap_or_else(|_| panic!("Expected {} argument to be valid: {}", name, val))
}

pub fn opt_parse_value_of<T: FromStr>(matches: &ArgMatches<'static>, name: &str) -> Option<T>
where
    <T as FromStr>::Err: fmt::Debug,
{
    // Use of 'expect' is safe because clap has validated input
    matches.value_of(name).map(|val| {
        T::from_str(val)
            .unwrap_or_else(|_| panic!("Expected {} argument to be valid: {}", name, val))
    })
}

fn parse_milliunits(
    settings: &models::BudgetSettings,
    val: &str,
) -> Result<models::Milliunits, std::num::ParseFloatError> {
    let fmt = settings.currency_format();
    let f = f64::from_str(
        &val.replace(fmt.currency_symbol(), "")
            .replace(fmt.group_separator(), "")
            .replace(fmt.decimal_separator(), "."),
    )?;
    let i = (f * 1000.0).round() as i64;
    Ok(models::Milliunits::new(i))
}

fn parse_date(
    settings: &models::BudgetSettings,
    val: &str,
) -> Result<NaiveDate, chrono::ParseError> {
    // @@@ TODO: also support e.g. "today", yesterday", "last sunday", etc.
    // @@@ TODO: make year optional (default to current year)
    let fmt = settings.date_format().format();
    let fmts = [
        fmt.replace("YYYY", "%Y")
            .replace("MM", "%m")
            .replace("DD", "%d"),
        fmt.replace("YYYY", "%Y")
            .replace("MM", "%b")
            .replace("DD", "%d"),
        "%Y-%m-%d".to_string(),
    ];
    let mut last_err: Option<chrono::ParseError> = None;
    for f in fmts.iter() {
        match NaiveDate::parse_from_str(val, f) {
            Ok(d) => {
                return Ok(d);
            }
            Err(e) => {
                last_err = Some(e);
            }
        }
    }
    Err(last_err.expect("Expected last_err to be set"))
}

// Cannot be validated in command-line (since we need budget settings to parse), so this returns a Result instead of panicing on invalid.
pub fn req_milliunits_value_of(
    settings: &models::BudgetSettings,
    matches: &ArgMatches<'static>,
    name: &str,
) -> Result<models::Milliunits, Error> {
    let val = req_value_of(matches, name);
    parse_milliunits(settings, val)
        .map_err(|e| Error::InvalidCurrencyAmountArg(name.to_string(), val.to_string(), e))
}

// Cannot be validated in command-line (since we need budget settings to parse), so this returns a Result instead of panicing on invalid.
pub fn opt_milliunits_value_of(
    settings: &models::BudgetSettings,
    matches: &ArgMatches<'static>,
    name: &str,
) -> Result<Option<models::Milliunits>, Error> {
    match matches.value_of(name) {
        None => Ok(None),
        Some(val) => parse_milliunits(settings, val)
            .map(Some)
            .map_err(|e| Error::InvalidCurrencyAmountArg(name.to_string(), val.to_string(), e)),
    }
}

pub fn req_month_value_of(matches: &ArgMatches<'static>, name: &str) -> NaiveDate {
    let val = req_value_of(matches, name);
    parse_month(val).unwrap_or_else(|_| panic!("Expected valid month: {}", val))
}

pub fn opt_month_value_of(matches: &ArgMatches<'static>, name: &str) -> Option<NaiveDate> {
    matches
        .value_of(name)
        .map(|val| parse_month(val).unwrap_or_else(|_| panic!("Expected valid month: {}", val)))
}

// Cannot be validated in command-line (since we need budget settings to parse), so this returns a Result instead of panicing on invalid.
pub fn opt_date_value_of(
    settings: &models::BudgetSettings,
    matches: &ArgMatches<'static>,
    name: &str,
) -> Result<Option<NaiveDate>, Error> {
    match matches.value_of(name) {
        None => Ok(None),
        Some(val) => parse_date(settings, val)
            .map(Some)
            .map_err(|e| Error::InvalidDateArg(name.to_string(), val.to_string(), e)),
    }
}

pub fn opt_flag_color_value_of(
    matches: &ArgMatches<'static>,
    name: &str,
) -> Option<Option<models::FlagColor>> {
    matches.value_of(name).map(|val| {
        if val == NONE_VAL {
            None
        } else {
            Some(
                models::FlagColor::from_str(val)
                    .unwrap_or_else(|_| panic!("Expected valid flag color: {}", val)),
            )
        }
    })
}

pub fn req_transaction_type_value_of(
    matches: &ArgMatches<'static>,
    name: &str,
) -> Option<models::TransactionType> {
    let val = req_value_of(matches, name);
    if val == ALL_VAL {
        None
    } else {
        // Use of 'expect' is safe because clap has validated input
        Some(models::TransactionType::from_str(val).unwrap_or_else(|_| {
            panic!(
                "Expected {} argument to be valid transaction type: {}",
                name, val
            )
        }))
    }
}

pub fn opt_last_knowledge_of_server_value_of(matches: &ArgMatches<'static>) -> Option<i64> {
    matches.value_of(LAST_KNOWLEDGE_OF_SERVER_ARG).map(|v| {
        v.parse().unwrap_or_else(|_| {
            panic!(
                "Expected {} argument to be integer: {}",
                LAST_KNOWLEDGE_OF_SERVER_ARG, v
            )
        })
    })
}

// @@@ RENAME TO month_option
pub fn month_arg(name: &'static str) -> Arg<'static, 'static> {
    // @@@ TODO: also support e.g. "last month", etc.
    Arg::with_name(name)
        .value_name("MONTH")
        .takes_value(true)
        .validator(is_month)
}

// @@@ RENME TO bool_option
pub fn bool_arg(name: &'static str) -> Arg<'static, 'static> {
    Arg::with_name(name)
        .possible_values(&[&TRUE_STRING, &FALSE_STRING])
        .value_name("BOOL")
        .takes_value(true)
}

pub fn add_possible_values(
    values: &'static Vec<String>,
    arg: Arg<'static, 'static>,
) -> Arg<'static, 'static> {
    //@@@ THIS LOOKS LIKE A FOLD
    let mut arg = arg;
    for val in values {
        arg = arg.possible_value(val);
    }
    arg
}

pub fn parse_month(val: &str) -> Result<NaiveDate, chrono::ParseError> {
    // @@@ TODO: also support e.g. "last month", etc.
    if val == CURRENT_MONTH_VAL {
        let d: NaiveDate = Local::now().date().naive_local();
        return Ok(NaiveDate::from_ymd(d.year(), d.month(), 1));
    }
    let val = format!("1 {}", val.replace("-", " ").replace("/", " "));
    let mut last_err: Option<chrono::ParseError> = None;
    for f in MONTH_VAL_FORMATS.iter() {
        match NaiveDate::parse_from_str(&val, f) {
            Ok(d) => {
                return Ok(d);
            }
            Err(e) => {
                last_err = Some(e);
            }
        }
    }
    Err(last_err.expect("Expected last_err to be set"))
}

//@@@ RENAME?
pub struct GlobalFlags {
    budget_id: bool,
    ynab_api: bool,
    table_format: bool,
    htable_format: bool,
    output_format: bool,
}

impl GlobalFlags {
    pub const NONE: GlobalFlags = GlobalFlags {
        budget_id: false,
        ynab_api: false,
        table_format: false,
        htable_format: false,
        output_format: false,
    };
    pub const ALL: GlobalFlags = GlobalFlags {
        budget_id: true,
        ynab_api: true,
        table_format: true,
        htable_format: true,
        output_format: true,
    };
    // @@@ NOW HIDING GLOBAL FLAGS FOR ALL SUBCOMMANDS; DON'T NEED ALL THESE
    pub const HTABLE_NO_BUDGET_ID: GlobalFlags = GlobalFlags::NONE;
    pub const HTABLE_WITH_BUDGET_ID: GlobalFlags = GlobalFlags::NONE;
    pub const VTABLE_NO_BUDGET_ID: GlobalFlags = GlobalFlags::NONE;
    pub const VTABLE_WITH_BUDGET_ID: GlobalFlags = GlobalFlags::NONE;
    pub const JSON_WITH_BUDGET_ID: GlobalFlags = GlobalFlags::NONE;
    // pub const HTABLE_NO_BUDGET_ID: GlobalFlags = GlobalFlags {
    //     budget_id: false,
    //     ynab_api: true,
    //     table_format: true,
    //     htable_format: true,
    //     output_format: true,
    // };
    // pub const HTABLE_WITH_BUDGET_ID: GlobalFlags = GlobalFlags {
    //     budget_id: true,
    //     ..Self::HTABLE_NO_BUDGET_ID
    // };
    // pub const VTABLE_NO_BUDGET_ID: GlobalFlags = GlobalFlags {
    //     budget_id: false,
    //     ynab_api: true,
    //     table_format: true,
    //     htable_format: false,
    //     output_format: true,
    // };
    // pub const VTABLE_WITH_BUDGET_ID: GlobalFlags = GlobalFlags {
    //     budget_id: true,
    //     ..Self::VTABLE_NO_BUDGET_ID
    // };
    // pub const JSON_NO_BUDGET_ID: GlobalFlags = GlobalFlags {
    //     budget_id: false,
    //     ynab_api: true,
    //     table_format: false,
    //     htable_format: false,
    //     output_format: true,
    // };
    // pub const JSON_WITH_BUDGET_ID: GlobalFlags = GlobalFlags {
    //     budget_id: true,
    //     ..Self::JSON_NO_BUDGET_ID
    // };
}

pub struct GlobalOptions {
    pub access_token: Option<String>,
    pub budget_id: String,
    pub headers: bool,
    pub table_borders: bool,
    pub output_format: OutputFormat,
}

// @@@ RENAME TO GlobalArgs?
impl GlobalOptions {
    // pub fn default() -> GlobalOptions {
    //     GlobalOptions {
    //         access_token: None,
    //         budget_id: DEFAULT_BUDGET_ID.to_string(),
    //         headers: true,
    //         //@@@ table_borders: true,
    //         output_format: OutputFormat::Table,
    //         //@@@ TODO: make default columns for each table configurable
    //     }
    // }
    pub fn args(flags: GlobalFlags, app: App<'static, 'static>) -> App<'static, 'static> {
        let result =
            app.arg(
                Arg::with_name(ACCESS_TOKEN_ARG)
                    .long(ACCESS_TOKEN_ARG)
                    .value_name("TOKEN")
                    .env(ACCESS_TOKEN_ENV)
                    .help("Personal access token for YNAB API")
                    .long_help("Personal Access Token for YNAB API.  To obtain a Personal Access Token, sign in to your account, go to \"My Account\", scroll down and navigate to \"Developer Settings\" section. From the Developer Settings page, click \"New Token\" under the Personal Access Tokens section, enter your password and you will be presented with a new Personal Access Token. You will not be able to retrieve the token later so you should store it in a safe place. This new token will not expire but can be revoked at any time from this same screen.")
                    .takes_value(true)
                    .hidden(!flags.ynab_api),
            ).arg(
                uuid_arg(BUDGET_ID_ARG)
                    .long(BUDGET_ID_ARG)
                    .default_value(DEFAULT_BUDGET_ID_VAL)
                    .env("YNAB_BUDGET_ID")
                    .help("The id of the budget")
                    .long_help("The id of the budget.  `last-used` can also be used to specify the last used budget.  Use `ynab get budgets` to get a list of your budgets' IDs.")
                    .hidden(!flags.budget_id),
            ).arg(
                bool_arg(HEADERS_ARG)
                    .long(HEADERS_ARG)
                    .env("YNAB_HEADERS")
                    .default_value(&DEFAULT_HEADERS_VAL)
                    .help("Tables have a column headers row?")
                    .hidden(!flags.htable_format),
            ).arg(
                bool_arg(BORDERS_ARG)
                    .long(BORDERS_ARG)
                    .env("YNAB_BORDERS")
                    .default_value(&DEFAULT_BORDERS_VAL)
                    .help("Tables have ASCII borders?")
                    .hidden(!flags.table_format),
            ).arg(
                add_possible_values(&OUTPUT_FORMAT_STRINGS, Arg::with_name(OUTPUT_ARG))
                    .long(OUTPUT_ARG)
                    .short("o")
                    .value_name("FORMAT")
                    .env("YNAB_OUTPUT")
                    .default_value(&DEFAULT_OUTPUT_VAL)
                    .help("Output format")
                    .long_help("Output format.  If JSON, outputs the response received from the YNAB API directly.")
                    .takes_value(true)
                    .hidden(!flags.output_format),
            );
        // @@@ USING flags.ynab_api AS A PROXY FOR THIS SHOW ALL GLOBAL ARGS
        if flags.ynab_api {
            result
        } else {
            // @@@ ALSO FIND A WAY TO OMIT THE --HELP AND --VERSION FLAGS THAT CLAP ADDS AUTOMATICALLY
            result.after_help("(global arguments omitted; see `ynab --help`).")
        }
    }

    // pub fn from_envvars(
    //     defaults: GlobalOptions,
    //     vars: std::env::Vars,
    // ) -> Result<GlobalOptions,AnyError> {
    //     let m: HashMap<_, _> = vars.collect();
    //     // @@@ ERROR WHEN PARSING FAILS SHOULD INDICATE _WHAT_ FAILED TO PARSE
    //     fn from_str_option<T: FromStr>(
    //         m: &HashMap<String, String>,
    //         n: &str,
    //         def: T,
    //     ) -> Result<T, <T as FromStr>::Err> {
    //         m.get(n).map(|v| T::from_str(&v)).unwrap_or(Ok(def))
    //     }
    //     let headers = from_str_option(&m, "YNAB_HEADERS", defaults.headers)?;
    //     //@@@ let table_borders = from_str_option(&m, "YNAB_BORDERS", defaults.table_borders)?;
    //     let output_format = from_str_option(&m, "YNAB_OUTPUT", defaults.output_format)?;
    //     Ok(GlobalOptions {
    //         access_token: m
    //             .get("YNAB_ACCESS_TOKEN")
    //             .map(|v| Some(v.to_owned()))
    //             .unwrap_or(defaults.access_token),
    //         budget_id: m
    //             .get("YNAB_BUDGET_ID")
    //             .map(|v| v.to_owned())
    //             .unwrap_or(defaults.budget_id),
    //         headers,
    //         //@@@ table_borders,
    //         output_format,
    //     })
    // }

    pub fn from_matches(
        super_matches: &ArgMatches<'static>,
        matches: &ArgMatches<'static>,
    ) -> Result<GlobalOptions, AnyError> {
        fn opt_value_of(
            super_matches: &ArgMatches<'static>,
            matches: &ArgMatches<'static>,
            name: &str,
        ) -> Option<String> {
            let val = if matches.occurrences_of(name) > 0 {
                matches.value_of(name)
            } else {
                super_matches.value_of(name)
            };
            val.map(|v| v.to_string())
        }
        fn req_value_of(
            super_matches: &ArgMatches<'static>,
            matches: &ArgMatches<'static>,
            name: &str,
        ) -> String {
            // Use of 'expect' is safe because clap has validated input
            opt_value_of(super_matches, matches, name)
                .unwrap_or_else(|| panic!("Expected {} argument to exists", name))
        }
        fn req_parse_value_of<T: FromStr>(
            super_matches: &ArgMatches<'static>,
            matches: &ArgMatches<'static>,
            name: &str,
        ) -> T
        where
            <T as FromStr>::Err: fmt::Debug,
        {
            let val = req_value_of(super_matches, matches, name);
            // Use of 'expect' is safe because clap has validated input
            T::from_str(&val).unwrap_or_else(|_| panic!("{} argument is valid: {}", name, val))
        }
        let headers = req_parse_value_of(super_matches, matches, HEADERS_ARG);
        let table_borders = req_parse_value_of(super_matches, matches, BORDERS_ARG);
        let output_format = req_parse_value_of(super_matches, matches, OUTPUT_ARG);
        Ok(GlobalOptions {
            access_token: opt_value_of(super_matches, matches, ACCESS_TOKEN_ARG),
            budget_id: req_value_of(super_matches, matches, BUDGET_ID_ARG),
            headers,
            table_borders,
            output_format,
        })
    }
}

fn super_command_settings(app: App<'static, 'static>) -> App<'static, 'static> {
    app.setting(AppSettings::SubcommandRequired)
}

fn subcommand(name: &str, flags: GlobalFlags) -> App<'static, 'static> {
    GlobalOptions::args(flags, SubCommand::with_name(name))
}

fn super_subcommand(name: &str) -> App<'static, 'static> {
    super_command_settings(SubCommand::with_name(name))
}

fn table_columns_arg(
    all_cols: &'static Vec<String>,
    default_cols: &'static String,
) -> Arg<'static, 'static> {
    add_possible_values(
        all_cols,
        Arg::with_name(TABLE_COLUMNS_ARG)
            .long(TABLE_COLUMNS_ARG)
            .help("Columns to include in table/CSV output")
            .value_name("COLUMN")
            .takes_value(true)
            .possible_value(ALL_VAL)
            .default_value(default_cols)
            .multiple(true)
            .use_delimiter(true),
    )
}

fn last_knowledge_of_server_arg() -> Arg<'static, 'static> {
    Arg::with_name(LAST_KNOWLEDGE_OF_SERVER_ARG)
        .long(LAST_KNOWLEDGE_OF_SERVER_ARG)
        .value_name("INTEGER")
        .help("The starting server knowledge")
        .long_help("The starting server knowledge.  If provided, only entities that have changed since last_knowledge_of_server will be included.")
        .takes_value(true)
        .validator(is_i64)
}

fn include_subtransactions_arg() -> Arg<'static, 'static> {
    bool_arg(INCLUDE_SUBTRANSACTIONS_ARG)
        .long(INCLUDE_SUBTRANSACTIONS_ARG)
        .help("Include subtransaction in output?")
        .takes_value(true)
        .default_value(&TRUE_STRING)
}

fn file_input_arg() -> Arg<'static, 'static> {
    Arg::with_name(FILE_INPUT_ARG)
                                .long(FILE_INPUT_ARG)
                                .short("f")
                                .value_name("PATH")
                                .help("If specified, JSON-formatted input containing data to write")
                                .help("If specified, JSON-formatted input containing data to write.  The data must match the YNAB API specification (see https://api.youneedabudget.com/v1).  Specify `-` to read from standard input instead of a file.")
                                .takes_value(true)
}

// @@@ SHOULD BE date_option
fn date_arg(name: &'static str) -> Arg<'static, 'static> {
    Arg::with_name(name).value_name("DATE").takes_value(true)
}

// @@@ SHOULD BE milliunits_option
fn milliunits_arg(name: &'static str) -> Arg<'static, 'static> {
    Arg::with_name(name)
        .value_name("CURRENCY-AMOUNT")
        .takes_value(true)
}

fn uuid_arg(name: &'static str) -> Arg<'static, 'static> {
    Arg::with_name(name).value_name("UUID").takes_value(true)
}

fn id_arg() -> Arg<'static, 'static> {
    Arg::with_name(ID_ARG)
        .long(ID_ARG)
        .value_name("UUID")
        .takes_value(true)
}

// @@@ MOVE TO CONSTANTS?
const DATE_ARG_LONG_HELP_SUFFIX: &str =
    "[format: match your budget settings' date format, or ISO 8601 (`YYYY-MM-DD`) format]";
const SET_TRANSACTION_DATE_ARG_HELP: &str = "If specified, set the transaction date";
const MILLIUNITS_ARG_LONG_HELP_SUFFIX: &str = "[format: match your budget settings' number format (the currency symbol and group separators may be omitted)]";
const SET_TRANSACTION_AMOUNT_ARG_HELP: &str = "If specified, set the transaction amount";
const SET_BUDGETED_ARG_HELP: &str = "Budgeted amount";
const LIST_CATEGORIES_MONTH_ARG_HELP: &str = "List categories for the given month";
const MONTH_ARG_LONG_HELP_SUFFIX: &str = "[format: `Mon-YYYY` (e.g. `Nov-2018`), `YYYY-MM` (e.g. `2018-11`), or `current` (the current month)]";
const SINCE_DATE_ARG_HELP: &str =
    "If specified, only transactions on or after this date will be included";
const GET_CATEGORY_MONTH_ARG_HELP: &str =
    "If specified, returns a single category for a specific budget month";
const BUDGET_MONTH_HELP: &str = "The budget month";

lazy_static! {
    pub static ref SET_TRANSACTION_DATE_ARG_LONG_HELP: String = format!(
        "{}.  {}",
        SET_TRANSACTION_DATE_ARG_HELP, DATE_ARG_LONG_HELP_SUFFIX
    );
    pub static ref SET_TRANSACTION_AMOUNT_ARG_LONG_HELP: String = format!(
        "{}.  {}",
        SET_TRANSACTION_AMOUNT_ARG_HELP, MILLIUNITS_ARG_LONG_HELP_SUFFIX
    );
    pub static ref SET_BUDGETED_ARG_LONG_HELP: String = format!(
        "{}.  {}",
        SET_BUDGETED_ARG_HELP, MILLIUNITS_ARG_LONG_HELP_SUFFIX
    );
    pub static ref LIST_CATEGORIES_MONTH_ARG_LONG_HELP: String = format!(
        "{}.  {}",
        LIST_CATEGORIES_MONTH_ARG_HELP, MONTH_ARG_LONG_HELP_SUFFIX
    );
    pub static ref SINCE_DATE_ARG_LONG_HELP: String =
        format!("{}.  {}", SINCE_DATE_ARG_HELP, DATE_ARG_LONG_HELP_SUFFIX);
    pub static ref GET_CATEGORY_MONTH_ARG_LONG_HELP: String = format!(
        "{}. Amounts (budgeted, activity, balance, etc.) are specific to the specified month.  {}",
        GET_CATEGORY_MONTH_ARG_HELP, MONTH_ARG_LONG_HELP_SUFFIX
    );
    pub static ref BUDGET_MONTH_LONG_HELP: String =
        format!("{}.  {}", BUDGET_MONTH_HELP, MONTH_ARG_LONG_HELP_SUFFIX);
}

fn set_transaction_args(create: bool, cmd: App<'static, 'static>) -> App<'static, 'static> {
    cmd.arg(
        uuid_arg(SET_ACCOUNT_ID_ARG)
            .long(SET_ACCOUNT_ID_ARG)
            .help("Set the account ID")
            .conflicts_with(FILE_INPUT_ARG)
            .required(create),
    ).arg(
        date_arg(SET_DATE_ARG)
            .long(SET_DATE_ARG)
            .help(SET_TRANSACTION_DATE_ARG_HELP)
            .long_help(&SET_TRANSACTION_DATE_ARG_LONG_HELP)
            .conflicts_with(FILE_INPUT_ARG)
            .required(create),
    ).arg(
        milliunits_arg(SET_AMOUNT_ARG)
            .long(SET_AMOUNT_ARG)
            .help(SET_TRANSACTION_AMOUNT_ARG_HELP)
            .long_help(&SET_TRANSACTION_AMOUNT_ARG_LONG_HELP)
            .conflicts_with(FILE_INPUT_ARG)
            .required(create),
    ).arg(
        uuid_arg(SET_PAYEE_ID_ARG)
            .long(SET_PAYEE_ID_ARG)
            .help("If specified, set the the payee for the transaction")
    ).arg(
        Arg::with_name(SET_PAYEE_NAME_ARG)
            .long(SET_PAYEE_NAME_ARG)
            .value_name("TEXT")
            .help("If specified, set the payee name.")
            .long_help("If specified, set the payee name.  This will be used to resolve the payee by either (1) a matching payee rename rule (only if --set-import-id is also specified) or (2) a payee with the same name or (3) creation of a new payee.")
            .takes_value(true)
    ).group(
        ArgGroup::with_name("PAYEE_GROUP")
            .args(&[SET_PAYEE_ID_ARG, SET_PAYEE_NAME_ARG])
            .conflicts_with(FILE_INPUT_ARG)
    ).arg(
        uuid_arg(SET_CATEGORY_ID_ARG)
            .long(SET_CATEGORY_ID_ARG)
            .help("If specified, set the category for the transaction")
            .long_help("If specified, set the category for the transaction. Split and Credit Card Payment categories are not permitted and will be ignored if supplied. If an existing transaction has a Split category it cannot be changed.")
            .conflicts_with(FILE_INPUT_ARG),
    ).arg(
        Arg::with_name(SET_MEMO_ARG)
            .long(SET_MEMO_ARG)
            .value_name("TEXT")
            .help("If specified, set the memo of the transaction")
            .takes_value(true)
            .conflicts_with(FILE_INPUT_ARG),
    ).arg(add_possible_values(
        &CLEARED_VALS,
        Arg::with_name(SET_CLEARED_ARG)
            .long(SET_CLEARED_ARG)
            .value_name("STATUS")
            .help("If specified, set the cleared status of the transaction")
            .takes_value(true)
            .conflicts_with(FILE_INPUT_ARG),
    )).arg(
        bool_arg(SET_APPROVED_ARG)
            .long(SET_APPROVED_ARG)
            .help("If specified, set whether or not the transaction is approved")
            .long_help("If specified, set whether or not the transaction is approved.  If not supplied, transaction will be unapproved by default.")
            .conflicts_with(FILE_INPUT_ARG),
    ).arg(add_possible_values(
        &FLAG_COLOR_VALS,
        Arg::with_name(SET_FLAG_COLOR_ARG)
            .long(SET_FLAG_COLOR_ARG)
            .value_name("COLOR")
            .help("If specified, set the transaction flag")
            .takes_value(true)
            .conflicts_with(FILE_INPUT_ARG)
            .possible_value(NONE_VAL),
    )).arg(
        uuid_arg(SET_IMPORT_ID_ARG)
            .long(SET_IMPORT_ID_ARG)
            .help("If specified, set the import ID")
            .long_help("If specified, set the import ID.  If specified for a new transaction, the transaction will be treated as Imported and assigned this import_id. If another transaction on the same account with this same import_id is later attempted to be created, it will be skipped to prevent duplication. Transactions imported through File Based Import or Direct Import and not through the API, are assigned an import_id in the format: `YNAB:[milliunit_amount]:[iso_date]:[occurrence]`. For example, a transaction dated 2015-12-30 in the amount of -$294.23 USD would have an import_id of `YNAB:-294230:2015-12-30:1`. If a second transaction on the same account was imported and had the same date and same amount, its import_id would be `YNAB:-294230:2015-12-30:2`. Using a consistent format will prevent duplicates through Direct Import and File Based Import. If import_id is specified as null, the transaction will be treated as a user entered transaction.")
            .conflicts_with(FILE_INPUT_ARG),
    ).arg(file_input_arg())
}

pub fn build_clap_app() -> App<'static, 'static> {
    GlobalOptions::args(GlobalFlags::ALL, super_command_settings(app_from_crate!()))
        .global_setting(AppSettings::AllArgsOverrideSelf)
        //@@@ .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::GlobalVersion)
        //@@@ .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .global_setting(AppSettings::NextLineHelp)
        .global_setting(AppSettings::AllowLeadingHyphen)
        //@@@ TODO: IMPLEMENT SOME NICE LOGGING
        // .arg(Arg::with_name("v")
        //      .short("v")
        //      .multiple(true)
        //      .help("Sets the level of verbosity"))
        .subcommand(
            super_subcommand(LIST_CMD)
                .about("Commands to list multiple items")
                .subcommand(
                    subcommand(ACCOUNTS_CMD, GlobalFlags::HTABLE_WITH_BUDGET_ID)
                        .about("List accounts")
                        .arg(table_columns_arg(
                            &ACCOUNT_COL_STRINGS,
                            &ACCOUNT_DEFAULT_COLS,
                        ))
                        // .arg(bool_arg(INCLUDE_CLOSED_ARG)
                        //     .long(INCLUDE_CLOSED_ARG)
                        //     .help("@@@")
                        //     .takes_value(true)
                        //     .default_value(&TRUE_STRING)),
                ).subcommand(
                    subcommand(PAYEES_CMD, GlobalFlags::HTABLE_WITH_BUDGET_ID)
                        .about("List payees")
                        .arg(table_columns_arg(
                            &PAYEE_COL_STRINGS,
                            &PAYEE_DEFAULT_COLS,
                        ))
                ).subcommand(
                    subcommand(PAYEE_LOCATIONS_CMD, GlobalFlags::HTABLE_WITH_BUDGET_ID)
                        .about("List payee locations")
                        .long_about("List payee locations\n\nWhen you enter a transaction and specify a payee on the YNAB mobile apps, the GPS coordinates for that location are stored, with your permission, so that the next time you are in the same place (like the Grocery store) we can pre-populate nearby payees for you! It's handy and saves you time. This resource makes these locations available. Locations will not be available for all payees.")
                        .arg(table_columns_arg(
                            &PAYEE_LOCATION_COL_STRINGS,
                            &PAYEE_LOCATION_DEFAULT_COLS,
                        )).arg(
                            uuid_arg(PAYEE_ID_ARG)
                                .long(PAYEE_ID_ARG)
                                .help("List locations for payee ID")
                        )
                ).subcommand(
                    subcommand(MONTHS_CMD, GlobalFlags::HTABLE_WITH_BUDGET_ID)
                        .about("List budget months")
                        .long_about("List budget months\n\nEach budget contains one or more months, which is where To be Budgeted, Age of Money and category (budgeted / activity / balances) amounts are available.")
                        .arg(table_columns_arg(
                            &MONTH_COL_STRINGS,
                            &MONTH_DEFAULT_COLS,
                        ))
                ).subcommand(
                    subcommand(BUDGETS_CMD, GlobalFlags::HTABLE_NO_BUDGET_ID)
                        .about("List budgets")
                        .arg(table_columns_arg(
                            &BUDGET_COL_STRINGS,
                            &BUDGET_DEFAULT_COLS,
                        ))
                ).subcommand(
                    subcommand(CATEGORIES_CMD, GlobalFlags::HTABLE_WITH_BUDGET_ID)
                        .about("List categories")
                        .arg(table_columns_arg(
                            &CATEGORY_COL_STRINGS,
                            &CATEGORY_DEFAULT_COLS,
                        )).arg(
                            month_arg(MONTH_ARG)
                                .long(MONTH_ARG)
                                .help(LIST_CATEGORIES_MONTH_ARG_HELP)
                                .long_help(&LIST_CATEGORIES_MONTH_ARG_LONG_HELP)
                            )
                        // .arg(bool_arg(INCLUDE_HIDDEN_ARG)
                        //     .long(INCLUDE_HIDDEN_ARG)
                        //     .help("@@@")
                        //     .takes_value(true)
                        //     .default_value(&TRUE_STRING))
                ).subcommand(
                    subcommand(TRANSACTIONS_CMD, GlobalFlags::HTABLE_WITH_BUDGET_ID)
                        .about("List transactions")
                        .arg(table_columns_arg(
                            &TRANSACTION_COL_STRINGS,
                            &TRANSACTION_DEFAULT_COLS,
                        )).arg(
                            date_arg(SINCE_DATE_ARG)
                                .long(SINCE_DATE_ARG)
                                .help(SINCE_DATE_ARG_HELP)
                                .long_help(&SINCE_DATE_ARG_LONG_HELP)
                        ).arg(
                            add_possible_values(&TRANSACTION_TYPE_VALS, Arg::with_name(TRANSACTION_TYPE_ARG)
                                .long(TRANSACTION_TYPE_ARG)
                                .value_name("TYPE")
                                .help("If specified, only transactions of the specified type will be included")
                                .takes_value(true)
                                .default_value(ALL_VAL)
                                .possible_value(ALL_VAL))
                        ).arg(last_knowledge_of_server_arg()
                        ).arg(include_subtransactions_arg()
                        ).arg(
                            uuid_arg(ACCOUNT_ID_ARG)
                                .long(ACCOUNT_ID_ARG)
                                .help("If specified, list transactions in the specified account will be included")                                .takes_value(true)
                        ).arg(
                            uuid_arg(CATEGORY_ID_ARG)
                                .long(CATEGORY_ID_ARG)
                                .help("If specified, list transactions for the specified category will be included")
                        ).arg(
                            uuid_arg(PAYEE_ID_ARG)
                                .long(PAYEE_ID_ARG)
                                .help("If specified, list transactions for the specified payee will be included")
                        ).group(
                            ArgGroup::with_name("ACCOUNT_CATEGORY_PAYEE_GROUP")
                                .args(&[ACCOUNT_ID_ARG, CATEGORY_ID_ARG, PAYEE_ID_ARG])
                        )
                ).subcommand(
                    subcommand(SCHEDULED_TRANSACTIONS_CMD, GlobalFlags::HTABLE_WITH_BUDGET_ID)
                        .about("List scheduled transactions")
                        .arg(table_columns_arg(
                            &SCHEDULED_TRANSACTION_COL_STRINGS,
                            &SCHEDULED_TRANSACTION_DEFAULT_COLS,
                        )).arg(include_subtransactions_arg()
                        )
                )
        ).subcommand(
            super_subcommand(GET_CMD)
                .about("Commands to get details for single items")
                .subcommand(
                    subcommand(USER_CMD, GlobalFlags::VTABLE_NO_BUDGET_ID)
                        .about("Returns authenticated user information")
                ).subcommand(
                    subcommand(BUDGET_CMD, GlobalFlags::JSON_WITH_BUDGET_ID)
                        .about("Returns a single budget with all related entities")
                        .long_about("Returns a single budget with all related entities\n\nThis is effectively a full budget export, and may output a huge amount of data.  Only JSON output is supported.")
                        .arg(last_knowledge_of_server_arg())
                ).subcommand(
                    subcommand(BUDGET_SETTINGS_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Returns settings for a budget")
                ).subcommand(
                    subcommand(CATEGORY_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Returns a single category")
                        .long_about("Returns a single category\n\nBy default, amounts (budgeted, activity, balance, etc.) are specific to the current budget month (UTC).")
                        .arg(
                            id_arg()
                                .help("The ID of the category")
                                .required(true)
                        ).arg(
                            month_arg(MONTH_ARG)
                                .long(MONTH_ARG)
                                .help(GET_CATEGORY_MONTH_ARG_HELP)
                                .long_help(&GET_CATEGORY_MONTH_ARG_LONG_HELP)
                                .default_value(CURRENT_MONTH_VAL)
                                .required(true)
                            )
                ).subcommand(
                    subcommand(ACCOUNT_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Returns a single account")
                        .arg(
                            id_arg()
                                .help("The ID of the account")
                                .required(true)
                            )
                ).subcommand(
                    subcommand(PAYEE_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Returns single payee")
                        .arg(
                            id_arg()
                                .help("The ID of the payee")
                                .required(true)
                            )
                ).subcommand(
                    subcommand(PAYEE_LOCATION_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Returns a single payee location")
                        .arg(
                            id_arg()
                                .help("ID of payee location")
                                .required(true)
                        )
                ).subcommand(
                    subcommand(MONTH_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Returns a single budget month")
                        .arg(
                            month_arg(MONTH_ARG)
                                .long(MONTH_ARG)
                                .help(BUDGET_MONTH_HELP)
                                .long_help(&BUDGET_MONTH_LONG_HELP)
                                .default_value(CURRENT_MONTH_VAL)
                                .required(true)
                        ).arg(bool_arg(INCLUDE_CATEGORIES_ARG)
                                .long(INCLUDE_CATEGORIES_ARG)
                                .help("Include categories in output?")
                                .takes_value(true)
                                .default_value(&TRUE_STRING)
                        )
                ).subcommand(
                    subcommand(TRANSACTION_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Returns a single transaction")
                        .arg(
                            id_arg()
                                .help("The ID of the transaction")
                                .required(true)
                        ).arg(include_subtransactions_arg())
                ).subcommand(
                    subcommand(SCHEDULED_TRANSACTION_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Returns a single scheduled transaction")
                        .arg(
                            id_arg()
                                .help("The ID of the scheduled transaction")
                                .required(true)
                        ).arg(include_subtransactions_arg())
                ),
        ).subcommand(
            // @@@ TODO: --quiet (or something) global argument to suppress printing update/create response
            super_subcommand(UPDATE_CMD)
                .about("Commands to update existing items")
                .subcommand(
                    subcommand(CATEGORY_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Update an existing month category")
                        .arg(
                            id_arg()
                                .help("The id of the category")
                                .required(true)
                        ).arg(
                            month_arg(MONTH_ARG)
                                .long(MONTH_ARG)
                                .help(BUDGET_MONTH_HELP)
                                .long_help(&BUDGET_MONTH_LONG_HELP)
                                .default_value(CURRENT_MONTH_VAL)
                                .required(true)
                        ).arg(
                            milliunits_arg(SET_BUDGETED_ARG)
                                .long(SET_BUDGETED_ARG)
                                .help(SET_BUDGETED_ARG_HELP)
                                .long_help(&SET_BUDGETED_ARG_LONG_HELP)
                                .required_unless(FILE_INPUT_ARG)
                                .conflicts_with(FILE_INPUT_ARG)
                        ).arg(file_input_arg())
                ).subcommand(
                    set_transaction_args(false, subcommand(TRANSACTION_CMD, GlobalFlags::VTABLE_WITH_BUDGET_ID)
                        .about("Update an existing transaction")
                        .arg(
                            id_arg()
                                .help("The id of the transaction")
                                .required(true)
                        ))
                ),
        ).subcommand(
            super_subcommand(CREATE_CMD)
                .about("Commands to create new items")
                .subcommand(
                    set_transaction_args(true, subcommand(TRANSACTION_CMD, GlobalFlags::HTABLE_WITH_BUDGET_ID)
                        .about("Creates a single transaction or multiple transactions")
                        .long_about("Creates a single transaction or multiple transactions\n\nTo create multiple transactions, you must use the `--file` option.  If you provide a body containing a 'transaction' object, a single transaction will be created and if you provide a body containing a `transactions` array, multiple transactions will be created.")
                        .arg(table_columns_arg(
                            &TRANSACTION_COL_STRINGS,
                            &TRANSACTION_DEFAULT_COLS,
                        )))
                ),
        ).subcommand(
            subcommand(COMPLETIONS_CMD, GlobalFlags::NONE)
                .about("Generates completion scripts for your shell")
                .setting(AppSettings::Hidden)
                .arg(
                    Arg::with_name(SHELL_ARG)
                        .required(true)
                        .possible_values(&clap::Shell::variants())
                        .help("The shell to generate the script for")
                )
        )
}

pub fn file_input<T>(state: &YnabState) -> Result<Option<T>, AnyError>
where
    for<'de> T: serde::Deserialize<'de>,
{
    if let Some(path) = state.matches.value_of(FILE_INPUT_ARG) {
        // @@@ ERROR MESSAGES HERE SHOULD INCLUDE THE FILENAME
        let file: Box<dyn io::Read> = if path == "-" {
            Box::new(std::io::stdin())
        } else {
            Box::new(fs::File::open(path)?)
        };
        let d: T = serde_json::from_reader(file)?;
        Ok(Some(d))
    } else {
        Ok(None)
    }
}
