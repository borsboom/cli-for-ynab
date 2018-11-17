/* 
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DateFormat : The date format setting for the budget.  In some cases the format will not be available and will be specified as null.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateFormat {
  #[serde(rename = "format")]
  format: String
}

impl DateFormat {
  /// The date format setting for the budget.  In some cases the format will not be available and will be specified as null.
  pub fn new(format: String) -> DateFormat {
    DateFormat {
      format: format
    }
  }

  pub fn set_format(&mut self, format: String) {
    self.format = format;
  }

  pub fn with_format(mut self, format: String) -> DateFormat {
    self.format = format;
    self
  }

  pub fn format(&self) -> &String {
    &self.format
  }


}



