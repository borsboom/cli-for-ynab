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
pub struct UserResponse {
  #[serde(rename = "data")]
  data: ::models::UserWrapper
}

impl UserResponse {
  pub fn new(data: ::models::UserWrapper) -> UserResponse {
    UserResponse {
      data: data
    }
  }

  pub fn set_data(&mut self, data: ::models::UserWrapper) {
    self.data = data;
  }

  pub fn with_data(mut self, data: ::models::UserWrapper) -> UserResponse {
    self.data = data;
    self
  }

  pub fn data(&self) -> &::models::UserWrapper {
    &self.data
  }


}


