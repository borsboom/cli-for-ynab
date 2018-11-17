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
pub struct BudgetDetail {
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "name")]
  name: String,
  /// The last time any changes were made to the budget from either a web or mobile client.
  #[serde(rename = "last_modified_on")]
  last_modified_on: Option<String>,
  #[serde(rename = "date_format")]
  date_format: Option<::models::DateFormat>,
  #[serde(rename = "currency_format")]
  currency_format: Option<::models::CurrencyFormat>,
  #[serde(rename = "accounts")]
  accounts: Option<Vec<::models::Account>>,
  #[serde(rename = "payees")]
  payees: Option<Vec<::models::Payee>>,
  #[serde(rename = "payee_locations")]
  payee_locations: Option<Vec<::models::PayeeLocation>>,
  #[serde(rename = "category_groups")]
  category_groups: Option<Vec<::models::CategoryGroup>>,
  #[serde(rename = "categories")]
  categories: Option<Vec<::models::Category>>,
  #[serde(rename = "months")]
  months: Option<Vec<::models::MonthDetail>>,
  #[serde(rename = "transactions")]
  transactions: Option<Vec<::models::TransactionSummary>>,
  #[serde(rename = "subtransactions")]
  subtransactions: Option<Vec<::models::SubTransaction>>,
  #[serde(rename = "scheduled_transactions")]
  scheduled_transactions: Option<Vec<::models::ScheduledTransactionSummary>>,
  #[serde(rename = "scheduled_subtransactions")]
  scheduled_subtransactions: Option<Vec<::models::ScheduledSubTransaction>>
}

impl BudgetDetail {
  pub fn new(id: String, name: String) -> BudgetDetail {
    BudgetDetail {
      id: id,
      name: name,
      last_modified_on: None,
      date_format: None,
      currency_format: None,
      accounts: None,
      payees: None,
      payee_locations: None,
      category_groups: None,
      categories: None,
      months: None,
      transactions: None,
      subtransactions: None,
      scheduled_transactions: None,
      scheduled_subtransactions: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> BudgetDetail {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> BudgetDetail {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_last_modified_on(&mut self, last_modified_on: String) {
    self.last_modified_on = Some(last_modified_on);
  }

  pub fn with_last_modified_on(mut self, last_modified_on: String) -> BudgetDetail {
    self.last_modified_on = Some(last_modified_on);
    self
  }

  pub fn last_modified_on(&self) -> Option<&String> {
    self.last_modified_on.as_ref()
  }

  pub fn reset_last_modified_on(&mut self) {
    self.last_modified_on = None;
  }

  pub fn set_date_format(&mut self, date_format: ::models::DateFormat) {
    self.date_format = Some(date_format);
  }

  pub fn with_date_format(mut self, date_format: ::models::DateFormat) -> BudgetDetail {
    self.date_format = Some(date_format);
    self
  }

  pub fn date_format(&self) -> Option<&::models::DateFormat> {
    self.date_format.as_ref()
  }

  pub fn reset_date_format(&mut self) {
    self.date_format = None;
  }

  pub fn set_currency_format(&mut self, currency_format: ::models::CurrencyFormat) {
    self.currency_format = Some(currency_format);
  }

  pub fn with_currency_format(mut self, currency_format: ::models::CurrencyFormat) -> BudgetDetail {
    self.currency_format = Some(currency_format);
    self
  }

  pub fn currency_format(&self) -> Option<&::models::CurrencyFormat> {
    self.currency_format.as_ref()
  }

  pub fn reset_currency_format(&mut self) {
    self.currency_format = None;
  }

  pub fn set_accounts(&mut self, accounts: Vec<::models::Account>) {
    self.accounts = Some(accounts);
  }

  pub fn with_accounts(mut self, accounts: Vec<::models::Account>) -> BudgetDetail {
    self.accounts = Some(accounts);
    self
  }

  pub fn accounts(&self) -> Option<&Vec<::models::Account>> {
    self.accounts.as_ref()
  }

  pub fn reset_accounts(&mut self) {
    self.accounts = None;
  }

  pub fn set_payees(&mut self, payees: Vec<::models::Payee>) {
    self.payees = Some(payees);
  }

  pub fn with_payees(mut self, payees: Vec<::models::Payee>) -> BudgetDetail {
    self.payees = Some(payees);
    self
  }

  pub fn payees(&self) -> Option<&Vec<::models::Payee>> {
    self.payees.as_ref()
  }

  pub fn reset_payees(&mut self) {
    self.payees = None;
  }

  pub fn set_payee_locations(&mut self, payee_locations: Vec<::models::PayeeLocation>) {
    self.payee_locations = Some(payee_locations);
  }

  pub fn with_payee_locations(mut self, payee_locations: Vec<::models::PayeeLocation>) -> BudgetDetail {
    self.payee_locations = Some(payee_locations);
    self
  }

  pub fn payee_locations(&self) -> Option<&Vec<::models::PayeeLocation>> {
    self.payee_locations.as_ref()
  }

  pub fn reset_payee_locations(&mut self) {
    self.payee_locations = None;
  }

  pub fn set_category_groups(&mut self, category_groups: Vec<::models::CategoryGroup>) {
    self.category_groups = Some(category_groups);
  }

  pub fn with_category_groups(mut self, category_groups: Vec<::models::CategoryGroup>) -> BudgetDetail {
    self.category_groups = Some(category_groups);
    self
  }

  pub fn category_groups(&self) -> Option<&Vec<::models::CategoryGroup>> {
    self.category_groups.as_ref()
  }

  pub fn reset_category_groups(&mut self) {
    self.category_groups = None;
  }

  pub fn set_categories(&mut self, categories: Vec<::models::Category>) {
    self.categories = Some(categories);
  }

  pub fn with_categories(mut self, categories: Vec<::models::Category>) -> BudgetDetail {
    self.categories = Some(categories);
    self
  }

  pub fn categories(&self) -> Option<&Vec<::models::Category>> {
    self.categories.as_ref()
  }

  pub fn reset_categories(&mut self) {
    self.categories = None;
  }

  pub fn set_months(&mut self, months: Vec<::models::MonthDetail>) {
    self.months = Some(months);
  }

  pub fn with_months(mut self, months: Vec<::models::MonthDetail>) -> BudgetDetail {
    self.months = Some(months);
    self
  }

  pub fn months(&self) -> Option<&Vec<::models::MonthDetail>> {
    self.months.as_ref()
  }

  pub fn reset_months(&mut self) {
    self.months = None;
  }

  pub fn set_transactions(&mut self, transactions: Vec<::models::TransactionSummary>) {
    self.transactions = Some(transactions);
  }

  pub fn with_transactions(mut self, transactions: Vec<::models::TransactionSummary>) -> BudgetDetail {
    self.transactions = Some(transactions);
    self
  }

  pub fn transactions(&self) -> Option<&Vec<::models::TransactionSummary>> {
    self.transactions.as_ref()
  }

  pub fn reset_transactions(&mut self) {
    self.transactions = None;
  }

  pub fn set_subtransactions(&mut self, subtransactions: Vec<::models::SubTransaction>) {
    self.subtransactions = Some(subtransactions);
  }

  pub fn with_subtransactions(mut self, subtransactions: Vec<::models::SubTransaction>) -> BudgetDetail {
    self.subtransactions = Some(subtransactions);
    self
  }

  pub fn subtransactions(&self) -> Option<&Vec<::models::SubTransaction>> {
    self.subtransactions.as_ref()
  }

  pub fn reset_subtransactions(&mut self) {
    self.subtransactions = None;
  }

  pub fn set_scheduled_transactions(&mut self, scheduled_transactions: Vec<::models::ScheduledTransactionSummary>) {
    self.scheduled_transactions = Some(scheduled_transactions);
  }

  pub fn with_scheduled_transactions(mut self, scheduled_transactions: Vec<::models::ScheduledTransactionSummary>) -> BudgetDetail {
    self.scheduled_transactions = Some(scheduled_transactions);
    self
  }

  pub fn scheduled_transactions(&self) -> Option<&Vec<::models::ScheduledTransactionSummary>> {
    self.scheduled_transactions.as_ref()
  }

  pub fn reset_scheduled_transactions(&mut self) {
    self.scheduled_transactions = None;
  }

  pub fn set_scheduled_subtransactions(&mut self, scheduled_subtransactions: Vec<::models::ScheduledSubTransaction>) {
    self.scheduled_subtransactions = Some(scheduled_subtransactions);
  }

  pub fn with_scheduled_subtransactions(mut self, scheduled_subtransactions: Vec<::models::ScheduledSubTransaction>) -> BudgetDetail {
    self.scheduled_subtransactions = Some(scheduled_subtransactions);
    self
  }

  pub fn scheduled_subtransactions(&self) -> Option<&Vec<::models::ScheduledSubTransaction>> {
    self.scheduled_subtransactions.as_ref()
  }

  pub fn reset_scheduled_subtransactions(&mut self) {
    self.scheduled_subtransactions = None;
  }

}



