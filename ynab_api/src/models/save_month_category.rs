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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaveMonthCategory {
  /// Budgeted amount in milliunits format
  #[serde(rename = "budgeted")]
  budgeted: ::models::Milliunits
}

impl SaveMonthCategory {
  pub fn new(budgeted: ::models::Milliunits) -> SaveMonthCategory {
    SaveMonthCategory {
      budgeted: budgeted
    }
  }

  pub fn set_budgeted(&mut self, budgeted: ::models::Milliunits) {
    self.budgeted = budgeted;
  }

  pub fn with_budgeted(mut self, budgeted: ::models::Milliunits) -> SaveMonthCategory {
    self.budgeted = budgeted;
    self
  }

  pub fn budgeted(&self) -> &::models::Milliunits {
    &self.budgeted
  }


}



