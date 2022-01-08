use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    Ynab(ErrorResponse),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

impl From<ErrorResponse> for Error {
    fn from(e: ErrorResponse) -> Self {
        return Error::Ynab(e)
    }
}


impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
        Error::Hyper(e) => { write!(f, "Error connecting to YNAB API: {}", e) }
        Error::Serde(e) => { write!(f, "Error parsing data from YNAB API: {}", e) }
        Error::Ynab(e) => { write!(f, "Error from YNAB API: {} ({} {})", e.error().detail(), e.error().id(), e.error().name()) }
    }
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
  	// @@@ CHANGE DEPENDING ON ENUM?
    "YNAB API error"
  }

  fn cause(&self) -> Option<&(dyn std::error::Error)> {
    None
  }
}


use super::models::*;

mod accounts_api;
pub use self::accounts_api::{ AccountsApi, AccountsApiClient };
mod budgets_api;
pub use self::budgets_api::{ BudgetsApi, BudgetsApiClient };
mod categories_api;
pub use self::categories_api::{ CategoriesApi, CategoriesApiClient };
mod deprecated_api;
pub use self::deprecated_api::{ DeprecatedApi, DeprecatedApiClient };
mod months_api;
pub use self::months_api::{ MonthsApi, MonthsApiClient };
mod payee_locations_api;
pub use self::payee_locations_api::{ PayeeLocationsApi, PayeeLocationsApiClient };
mod payees_api;
pub use self::payees_api::{ PayeesApi, PayeesApiClient };
mod scheduled_transactions_api;
pub use self::scheduled_transactions_api::{ ScheduledTransactionsApi, ScheduledTransactionsApiClient };
mod transactions_api;
pub use self::transactions_api::{ TransactionsApi, TransactionsApiClient };
mod user_api;
pub use self::user_api::{ UserApi, UserApiClient };

pub mod configuration;
pub mod client;



//@@@ RIGHT PLACE?
fn json_from_slice<'a, T>(body: &'a hyper::Chunk) -> Result<T, Error>
where T: serde::Deserialize<'a>
{
	// println!("@@@ BODY:\n{}", String::from_utf8(body.to_vec()).unwrap());
	// @@@ WOULD BE BETTER TO CHECK STATUS FOR NON-200 RATHER THAN USE `starts_with`
    if body.starts_with(b"{\"error\":{") {
        // println!("@@@ LOOKS LIKE AN ERROR");
        let parsed: Result<::models::ErrorResponse, _> = serde_json::from_slice(&body);
        // println!("@@@ PARSED? {:?}", parsed);
        match parsed {
            Ok(e) => { Err(Error::from(e)) }
            Err(e) => { Err(Error::from(e)) }
        }
    } else {
        let parsed: Result<T, _> = serde_json::from_slice(&body);
        match parsed {
            Ok(r) => { Ok(r) }
            Err(e) => { Err(Error::from(e)) }
        }
    }
}
