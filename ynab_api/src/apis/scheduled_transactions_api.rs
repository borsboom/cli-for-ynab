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

pub struct ScheduledTransactionsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ScheduledTransactionsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ScheduledTransactionsApiClient<C> {
        ScheduledTransactionsApiClient {
            configuration: configuration,
        }
    }
}

pub trait ScheduledTransactionsApi {
    fn get_scheduled_transaction_by_id(&self, budget_id: &str, scheduled_transaction_id: &str) -> Box<Future<Item = ::models::ScheduledTransactionResponse, Error = Error>>;
    fn get_scheduled_transactions(&self, budget_id: &str) -> Box<Future<Item = ::models::ScheduledTransactionsResponse, Error = Error>>;
}


impl<C: hyper::client::Connect>ScheduledTransactionsApi for ScheduledTransactionsApiClient<C> {
    fn get_scheduled_transaction_by_id(&self, budget_id: &str, scheduled_transaction_id: &str) -> Box<Future<Item = ::models::ScheduledTransactionResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/scheduled_transactions/{scheduled_transaction_id}", configuration.base_path, budget_id=budget_id, scheduled_transaction_id=scheduled_transaction_id);

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
                let parsed: Result<::models::ScheduledTransactionResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_scheduled_transactions(&self, budget_id: &str) -> Box<Future<Item = ::models::ScheduledTransactionsResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/scheduled_transactions", configuration.base_path, budget_id=budget_id);

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
                let parsed: Result<::models::ScheduledTransactionsResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}