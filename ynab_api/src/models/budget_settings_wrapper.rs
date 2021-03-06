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
pub struct BudgetSettingsWrapper {
  #[serde(rename = "settings")]
  settings: ::models::BudgetSettings
}

impl BudgetSettingsWrapper {
  pub fn new(settings: ::models::BudgetSettings) -> BudgetSettingsWrapper {
    BudgetSettingsWrapper {
      settings: settings
    }
  }

  pub fn set_settings(&mut self, settings: ::models::BudgetSettings) {
    self.settings = settings;
  }

  pub fn with_settings(mut self, settings: ::models::BudgetSettings) -> BudgetSettingsWrapper {
    self.settings = settings;
    self
  }

  pub fn settings(&self) -> &::models::BudgetSettings {
    &self.settings
  }


}



