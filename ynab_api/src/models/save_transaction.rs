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
pub struct SaveTransaction {
  #[serde(rename = "account_id")]
  account_id: String,
  #[serde(rename = "date")]
  date: chrono::NaiveDate,
  /// The transaction amount in milliunits format
  #[serde(rename = "amount")]
  amount: ::models::Milliunits,
  /// The payee for the transaction
  #[serde(rename = "payee_id")]
  payee_id: Option<String>,
  /// The payee name.  If a payee_name value is provided and payee_id has a null value, the payee_name value will be used to resolve the payee by either (1) a matching payee rename rule (only if import_id is also specified) or (2) a payee with the same name or (3) creation of a new payee.
  #[serde(rename = "payee_name")]
  payee_name: Option<String>,
  /// The category for the transaction.  Split and Credit Card Payment categories are not permitted and will be ignored if supplied.  If an existing transaction has a Split category it cannot be changed.
  #[serde(rename = "category_id")]
  category_id: Option<String>,
  #[serde(rename = "memo")]
  memo: Option<String>,
  /// The cleared status of the transaction
  // @@@ MAKE ENUM
  #[serde(rename = "cleared")]
  cleared: Option<::models::Cleared>,
  /// Whether or not the transaction is approved.  If not supplied, transaction will be unapproved by default.
  #[serde(rename = "approved")]
  approved: Option<bool>,
  /// The transaction flag
  // @@@ MAKE ENUM
  #[serde(rename = "flag_color")]
  flag_color: Option<::models::FlagColor>,
  /// If specified for a new transaction, the transaction will be treated as Imported and assigned this import_id.  If another transaction on the same account with this same import_id is later attempted to be created, it will be skipped to prevent duplication.  Transactions imported through File Based Import or Direct Import and not through the API, are assigned an import_id in the format: 'YNAB:[milliunit_amount]:[iso_date]:[occurrence]'.  For example, a transaction dated 2015-12-30 in the amount of -$294.23 USD would have an import_id of 'YNAB:-294230:2015-12-30:1'.  If a second transaction on the same account was imported and had the same date and same amount, its import_id would be 'YNAB:-294230:2015-12-30:2'.  Using a consistent format will prevent duplicates through Direct Import and File Based Import.  If import_id is specified as null, the transaction will be treated as a user entered transaction.
  #[serde(rename = "import_id")]
  import_id: Option<String>
}

impl SaveTransaction {
  pub fn new(account_id: String, date: chrono::NaiveDate, amount: ::models::Milliunits) -> SaveTransaction {
    SaveTransaction {
      account_id: account_id,
      date: date,
      amount: amount,
      payee_id: None,
      payee_name: None,
      category_id: None,
      memo: None,
      cleared: None,
      approved: None,
      flag_color: None,
      import_id: None
    }
  }

  pub fn set_account_id(&mut self, account_id: String) {
    self.account_id = account_id;
  }

  pub fn with_account_id(mut self, account_id: String) -> SaveTransaction {
    self.account_id = account_id;
    self
  }

  pub fn account_id(&self) -> &String {
    &self.account_id
  }


  pub fn set_date(&mut self, date: chrono::NaiveDate) {
    self.date = date;
  }

  pub fn with_date(mut self, date: chrono::NaiveDate) -> SaveTransaction {
    self.date = date;
    self
  }

  pub fn date(&self) -> &chrono::NaiveDate {
    &self.date
  }


  pub fn set_amount(&mut self, amount: ::models::Milliunits) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: ::models::Milliunits) -> SaveTransaction {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &::models::Milliunits {
    &self.amount
  }


  pub fn set_payee_id(&mut self, payee_id: String) {
    self.payee_id = Some(payee_id);
  }

  pub fn with_payee_id(mut self, payee_id: String) -> SaveTransaction {
    self.payee_id = Some(payee_id);
    self
  }

  pub fn payee_id(&self) -> Option<&String> {
    self.payee_id.as_ref()
  }

  pub fn reset_payee_id(&mut self) {
    self.payee_id = None;
  }

  pub fn set_payee_name(&mut self, payee_name: String) {
    self.payee_name = Some(payee_name);
  }

  pub fn with_payee_name(mut self, payee_name: String) -> SaveTransaction {
    self.payee_name = Some(payee_name);
    self
  }

  pub fn payee_name(&self) -> Option<&String> {
    self.payee_name.as_ref()
  }

  pub fn reset_payee_name(&mut self) {
    self.payee_name = None;
  }

  pub fn set_category_id(&mut self, category_id: String) {
    self.category_id = Some(category_id);
  }

  pub fn with_category_id(mut self, category_id: String) -> SaveTransaction {
    self.category_id = Some(category_id);
    self
  }

  pub fn category_id(&self) -> Option<&String> {
    self.category_id.as_ref()
  }

  pub fn reset_category_id(&mut self) {
    self.category_id = None;
  }

  pub fn set_memo(&mut self, memo: String) {
    self.memo = Some(memo);
  }

  pub fn with_memo(mut self, memo: String) -> SaveTransaction {
    self.memo = Some(memo);
    self
  }

  pub fn memo(&self) -> Option<&String> {
    self.memo.as_ref()
  }

  pub fn reset_memo(&mut self) {
    self.memo = None;
  }

  pub fn set_cleared(&mut self, cleared: ::models::Cleared) {
    self.cleared = Some(cleared);
  }

  pub fn with_cleared(mut self, cleared: ::models::Cleared) -> SaveTransaction {
    self.cleared = Some(cleared);
    self
  }

  pub fn cleared(&self) -> Option<&::models::Cleared> {
    self.cleared.as_ref()
  }

  pub fn reset_cleared(&mut self) {
    self.cleared = None;
  }

  pub fn set_approved(&mut self, approved: bool) {
    self.approved = Some(approved);
  }

  pub fn with_approved(mut self, approved: bool) -> SaveTransaction {
    self.approved = Some(approved);
    self
  }

  pub fn approved(&self) -> Option<&bool> {
    self.approved.as_ref()
  }

  pub fn reset_approved(&mut self) {
    self.approved = None;
  }

  pub fn set_flag_color(&mut self, flag_color: ::models::FlagColor) {
    self.flag_color = Some(flag_color);
  }

  pub fn with_flag_color(mut self, flag_color: ::models::FlagColor) -> SaveTransaction {
    self.flag_color = Some(flag_color);
    self
  }

  pub fn flag_color(&self) -> Option<&::models::FlagColor> {
    self.flag_color.as_ref()
  }

  pub fn reset_flag_color(&mut self) {
    self.flag_color = None;
  }

  pub fn set_import_id(&mut self, import_id: String) {
    self.import_id = Some(import_id);
  }

  pub fn with_import_id(mut self, import_id: String) -> SaveTransaction {
    self.import_id = Some(import_id);
    self
  }

  pub fn import_id(&self) -> Option<&String> {
    self.import_id.as_ref()
  }

  pub fn reset_import_id(&mut self) {
    self.import_id = None;
  }

}


