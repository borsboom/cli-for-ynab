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
pub struct SaveMonthCategoryWrapper {
  #[serde(rename = "month_category")]
  month_category: ::models::SaveMonthCategory
}

impl SaveMonthCategoryWrapper {
  pub fn new(month_category: ::models::SaveMonthCategory) -> SaveMonthCategoryWrapper {
    SaveMonthCategoryWrapper {
      month_category: month_category
    }
  }

  pub fn set_month_category(&mut self, month_category: ::models::SaveMonthCategory) {
    self.month_category = month_category;
  }

  pub fn with_month_category(mut self, month_category: ::models::SaveMonthCategory) -> SaveMonthCategoryWrapper {
    self.month_category = month_category;
    self
  }

  pub fn month_category(&self) -> &::models::SaveMonthCategory {
    &self.month_category
  }


}



