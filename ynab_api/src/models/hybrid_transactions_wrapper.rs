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
pub struct HybridTransactionsWrapper {
  #[serde(rename = "transactions")]
  transactions: Vec<::models::HybridTransaction>
}

impl HybridTransactionsWrapper {
  pub fn new(transactions: Vec<::models::HybridTransaction>) -> HybridTransactionsWrapper {
    HybridTransactionsWrapper {
      transactions: transactions
    }
  }

  pub fn set_transactions(&mut self, transactions: Vec<::models::HybridTransaction>) {
    self.transactions = transactions;
  }

  pub fn with_transactions(mut self, transactions: Vec<::models::HybridTransaction>) -> HybridTransactionsWrapper {
    self.transactions = transactions;
    self
  }

  pub fn transactions(&self) -> &Vec<::models::HybridTransaction> {
    &self.transactions
  }


}


