/* 
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;
use chrono::{DateTime,Local};

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetSummary {
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "name")]
  name: String,
  /// The last time any changes were made to the budget from either a web or mobile client.
  #[serde(rename = "last_modified_on")]
  last_modified_on: Option<DateTime<Local>>,
  #[serde(rename = "date_format")]
  date_format: Option<::models::DateFormat>,
  #[serde(rename = "currency_format")]
  currency_format: Option<::models::CurrencyFormat>
}

impl BudgetSummary {
  pub fn new(id: String, name: String) -> BudgetSummary {
    BudgetSummary {
      id: id,
      name: name,
      last_modified_on: None,
      date_format: None,
      currency_format: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> BudgetSummary {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> BudgetSummary {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_last_modified_on(&mut self, last_modified_on: DateTime<Local>) {
    self.last_modified_on = Some(last_modified_on);
  }

  pub fn with_last_modified_on(mut self, last_modified_on: DateTime<Local>) -> BudgetSummary {
    self.last_modified_on = Some(last_modified_on);
    self
  }

  pub fn last_modified_on(&self) -> Option<&DateTime<Local>> {
    self.last_modified_on.as_ref()
  }

  pub fn reset_last_modified_on(&mut self) {
    self.last_modified_on = None;
  }

  pub fn set_date_format(&mut self, date_format: ::models::DateFormat) {
    self.date_format = Some(date_format);
  }

  pub fn with_date_format(mut self, date_format: ::models::DateFormat) -> BudgetSummary {
    self.date_format = Some(date_format);
    self
  }

  pub fn date_format(&self) -> Option<&::models::DateFormat> {
    self.date_format.as_ref()
  }

  pub fn reset_date_format(&mut self) {
    self.date_format = None;
  }

  pub fn set_currency_format(&mut self, currency_format: ::models::CurrencyFormat) {
    self.currency_format = Some(currency_format);
  }

  pub fn with_currency_format(mut self, currency_format: ::models::CurrencyFormat) -> BudgetSummary {
    self.currency_format = Some(currency_format);
    self
  }

  pub fn currency_format(&self) -> Option<&::models::CurrencyFormat> {
    self.currency_format.as_ref()
  }

  pub fn reset_currency_format(&mut self) {
    self.currency_format = None;
  }

}



