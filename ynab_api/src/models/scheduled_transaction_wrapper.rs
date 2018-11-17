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
pub struct ScheduledTransactionWrapper {
  #[serde(rename = "scheduled_transaction")]
  scheduled_transaction: ::models::ScheduledTransactionDetail
}

impl ScheduledTransactionWrapper {
  pub fn new(scheduled_transaction: ::models::ScheduledTransactionDetail) -> ScheduledTransactionWrapper {
    ScheduledTransactionWrapper {
      scheduled_transaction: scheduled_transaction
    }
  }

  pub fn set_scheduled_transaction(&mut self, scheduled_transaction: ::models::ScheduledTransactionDetail) {
    self.scheduled_transaction = scheduled_transaction;
  }

  pub fn with_scheduled_transaction(mut self, scheduled_transaction: ::models::ScheduledTransactionDetail) -> ScheduledTransactionWrapper {
    self.scheduled_transaction = scheduled_transaction;
    self
  }

  pub fn scheduled_transaction(&self) -> &::models::ScheduledTransactionDetail {
    &self.scheduled_transaction
  }


}



