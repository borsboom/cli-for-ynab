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
pub struct MonthSummary {
  #[serde(rename = "month")]
  // @@@ PROPER MONTH TYPE
  month: chrono::NaiveDate,
  #[serde(rename = "note")]
  note: Option<String>,
  /// The total amount in transactions categorized to 'Inflow: To be Budgeted' in the month
  #[serde(rename = "income")]
  income: Option<::models::Milliunits>,
  /// The total amount budgeted in the month
  #[serde(rename = "budgeted")]
  budgeted: Option<::models::Milliunits>,
  /// The total amount in transactions in the month, excluding those categorized to 'Inflow: To be Budgeted'
  #[serde(rename = "activity")]
  activity: Option<::models::Milliunits>,
  /// The available amount for 'To be Budgeted'
  #[serde(rename = "to_be_budgeted")]
  to_be_budgeted: Option<::models::Milliunits>,
  /// The Age of Money as of the month
  #[serde(rename = "age_of_money")]
  age_of_money: Option<i32>
}

impl MonthSummary {
  pub fn new(month: chrono::NaiveDate) -> MonthSummary {
    MonthSummary {
      month: month,
      note: None,
      income: None,
      budgeted: None,
      activity: None,
      to_be_budgeted: None,
      age_of_money: None
    }
  }

  pub fn set_month(&mut self, month: chrono::NaiveDate) {
    self.month = month;
  }

  pub fn with_month(mut self, month: chrono::NaiveDate) -> MonthSummary {
    self.month = month;
    self
  }

  pub fn month(&self) -> &chrono::NaiveDate {
    &self.month
  }


  pub fn set_note(&mut self, note: Option<String>) {
    self.note = note;
  }

  pub fn with_note(mut self, note: Option<String>) -> MonthSummary {
    self.note = note;
    self
  }

  pub fn note(&self) -> Option<&String> {
    self.note.as_ref()
  }


  pub fn set_income(&mut self, income: Option<::models::Milliunits>) {
    self.income = income;
  }

  pub fn with_income(mut self, income: Option<::models::Milliunits>) -> MonthSummary {
    self.income = income;
    self
  }

  pub fn income(&self) -> Option<&::models::Milliunits> {
    self.income.as_ref()
  }


  pub fn set_budgeted(&mut self, budgeted: Option<::models::Milliunits>) {
    self.budgeted = budgeted;
  }

  pub fn with_budgeted(mut self, budgeted: Option<::models::Milliunits>) -> MonthSummary {
    self.budgeted = budgeted;
    self
  }

  pub fn budgeted(&self) -> Option<&::models::Milliunits> {
    self.budgeted.as_ref()
  }


  pub fn set_activity(&mut self, activity: Option<::models::Milliunits>) {
    self.activity = activity;
  }

  pub fn with_activity(mut self, activity: Option<::models::Milliunits>) -> MonthSummary {
    self.activity = activity;
    self
  }

  pub fn activity(&self) -> Option<&::models::Milliunits> {
    self.activity.as_ref()
  }


  pub fn set_to_be_budgeted(&mut self, to_be_budgeted: Option<::models::Milliunits>) {
    self.to_be_budgeted = to_be_budgeted;
  }

  pub fn with_to_be_budgeted(mut self, to_be_budgeted: Option<::models::Milliunits>) -> MonthSummary {
    self.to_be_budgeted = to_be_budgeted;
    self
  }

  pub fn to_be_budgeted(&self) -> Option<&::models::Milliunits> {
    self.to_be_budgeted.as_ref()
  }


  pub fn set_age_of_money(&mut self, age_of_money: Option<i32>) {
    self.age_of_money = age_of_money;
  }

  pub fn with_age_of_money(mut self, age_of_money: Option<i32>) -> MonthSummary {
    self.age_of_money = age_of_money;
    self
  }

  pub fn age_of_money(&self) -> Option<&i32> {
    self.age_of_money.as_ref()
  }


}



