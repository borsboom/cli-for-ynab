use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: std::marker::PhantomData<C>,
  accounts_api: Box<dyn (::apis::AccountsApi)>,
  budgets_api: Box<dyn (::apis::BudgetsApi)>,
  categories_api: Box<dyn (::apis::CategoriesApi)>,
  deprecated_api: Box<dyn (::apis::DeprecatedApi)>,
  months_api: Box<dyn (::apis::MonthsApi)>,
  payee_locations_api: Box<dyn (::apis::PayeeLocationsApi)>,
  payees_api: Box<dyn (::apis::PayeesApi)>,
  scheduled_transactions_api: Box<dyn (::apis::ScheduledTransactionsApi)>,
  transactions_api: Box<dyn (::apis::TransactionsApi)>,
  user_api: Box<dyn (::apis::UserApi)>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: std::marker::PhantomData,
      accounts_api: Box::new(::apis::AccountsApiClient::new(rc.clone())),
      budgets_api: Box::new(::apis::BudgetsApiClient::new(rc.clone())),
      categories_api: Box::new(::apis::CategoriesApiClient::new(rc.clone())),
      deprecated_api: Box::new(::apis::DeprecatedApiClient::new(rc.clone())),
      months_api: Box::new(::apis::MonthsApiClient::new(rc.clone())),
      payee_locations_api: Box::new(::apis::PayeeLocationsApiClient::new(rc.clone())),
      payees_api: Box::new(::apis::PayeesApiClient::new(rc.clone())),
      scheduled_transactions_api: Box::new(::apis::ScheduledTransactionsApiClient::new(rc.clone())),
      transactions_api: Box::new(::apis::TransactionsApiClient::new(rc.clone())),
      user_api: Box::new(::apis::UserApiClient::new(rc.clone())),
    }
  }

  pub fn accounts_api(&self) -> &dyn (::apis::AccountsApi){
    self.accounts_api.as_ref()
  }

  pub fn budgets_api(&self) -> &dyn (::apis::BudgetsApi){
    self.budgets_api.as_ref()
  }

  pub fn categories_api(&self) -> &dyn (::apis::CategoriesApi){
    self.categories_api.as_ref()
  }

  pub fn deprecated_api(&self) -> &dyn (::apis::DeprecatedApi){
    self.deprecated_api.as_ref()
  }

  pub fn months_api(&self) -> &dyn (::apis::MonthsApi){
    self.months_api.as_ref()
  }

  pub fn payee_locations_api(&self) -> &dyn (::apis::PayeeLocationsApi){
    self.payee_locations_api.as_ref()
  }

  pub fn payees_api(&self) -> &dyn (::apis::PayeesApi){
    self.payees_api.as_ref()
  }

  pub fn scheduled_transactions_api(&self) -> &dyn (::apis::ScheduledTransactionsApi){
    self.scheduled_transactions_api.as_ref()
  }

  pub fn transactions_api(&self) -> &dyn (::apis::TransactionsApi){
    self.transactions_api.as_ref()
  }

  pub fn user_api(&self) -> &dyn (::apis::UserApi){
    self.user_api.as_ref()
  }


}
