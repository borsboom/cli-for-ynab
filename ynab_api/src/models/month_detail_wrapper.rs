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
pub struct MonthDetailWrapper {
  #[serde(rename = "month")]
  month: ::models::MonthDetail
}

impl MonthDetailWrapper {
  pub fn new(month: ::models::MonthDetail) -> MonthDetailWrapper {
    MonthDetailWrapper {
      month: month
    }
  }

  pub fn set_month(&mut self, month: ::models::MonthDetail) {
    self.month = month;
  }

  pub fn with_month(mut self, month: ::models::MonthDetail) -> MonthDetailWrapper {
    self.month = month;
    self
  }

  pub fn month(&self) -> &::models::MonthDetail {
    &self.month
  }


}



