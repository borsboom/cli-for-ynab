/* 
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct AccountsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AccountsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AccountsApiClient<C> {
        AccountsApiClient {
            configuration: configuration,
        }
    }
}

pub trait AccountsApi {
    fn get_account_by_id(&self, budget_id: &str, account_id: &str) -> Box<Future<Item = ::models::AccountResponse, Error = Error>>;
    fn get_accounts(&self, budget_id: &str) -> Box<Future<Item = ::models::AccountsResponse, Error = Error>>;
}


impl<C: hyper::client::Connect>AccountsApi for AccountsApiClient<C> {
    fn get_account_by_id(&self, budget_id: &str, account_id: &str) -> Box<Future<Item = ::models::AccountResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/accounts/{account_id}", configuration.base_path, budget_id=budget_id, account_id=account_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let req = configuration.new_request(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AccountResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_accounts(&self, budget_id: &str) -> Box<Future<Item = ::models::AccountsResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/accounts", configuration.base_path, budget_id=budget_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let req = configuration.new_request(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AccountsResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
