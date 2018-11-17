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
pub struct PayeeWrapper {
  #[serde(rename = "payee")]
  payee: ::models::Payee
}

impl PayeeWrapper {
  pub fn new(payee: ::models::Payee) -> PayeeWrapper {
    PayeeWrapper {
      payee: payee
    }
  }

  pub fn set_payee(&mut self, payee: ::models::Payee) {
    self.payee = payee;
  }

  pub fn with_payee(mut self, payee: ::models::Payee) -> PayeeWrapper {
    self.payee = payee;
    self
  }

  pub fn payee(&self) -> &::models::Payee {
    &self.payee
  }


}



