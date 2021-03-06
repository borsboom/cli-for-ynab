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

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthSummariesWrapper {
  #[serde(rename = "months")]
  months: Vec<::models::MonthSummary>
}

impl MonthSummariesWrapper {
  pub fn new(months: Vec<::models::MonthSummary>) -> MonthSummariesWrapper {
    MonthSummariesWrapper {
      months: months
    }
  }

  pub fn set_months(&mut self, months: Vec<::models::MonthSummary>) {
    self.months = months;
  }

  pub fn with_months(mut self, months: Vec<::models::MonthSummary>) -> MonthSummariesWrapper {
    self.months = months;
    self
  }

  pub fn months(&self) -> &Vec<::models::MonthSummary> {
    &self.months
  }


}



