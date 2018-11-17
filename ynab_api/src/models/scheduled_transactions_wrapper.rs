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
pub struct ScheduledTransactionsWrapper {
  #[serde(rename = "scheduled_transactions")]
  scheduled_transactions: Vec<::models::ScheduledTransactionDetail>
}

impl ScheduledTransactionsWrapper {
  pub fn new(scheduled_transactions: Vec<::models::ScheduledTransactionDetail>) -> ScheduledTransactionsWrapper {
    ScheduledTransactionsWrapper {
      scheduled_transactions: scheduled_transactions
    }
  }

  pub fn set_scheduled_transactions(&mut self, scheduled_transactions: Vec<::models::ScheduledTransactionDetail>) {
    self.scheduled_transactions = scheduled_transactions;
  }

  pub fn with_scheduled_transactions(mut self, scheduled_transactions: Vec<::models::ScheduledTransactionDetail>) -> ScheduledTransactionsWrapper {
    self.scheduled_transactions = scheduled_transactions;
    self
  }

  pub fn scheduled_transactions(&self) -> &Vec<::models::ScheduledTransactionDetail> {
    &self.scheduled_transactions
  }


}



