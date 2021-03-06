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

pub struct MonthsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> MonthsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> MonthsApiClient<C> {
        MonthsApiClient {
            configuration: configuration,
        }
    }
}

pub trait MonthsApi {
    // @@@ USE A REAL MONTH TYPE THAT ALSO HAS A 'current' ENUM VALUE
    fn get_budget_month(&self, budget_id: &str, month: &chrono::NaiveDate) -> Box<Future<Item = ::models::MonthDetailResponse, Error = Error>>;
    fn get_budget_months(&self, budget_id: &str) -> Box<Future<Item = ::models::MonthSummariesResponse, Error = Error>>;
}


impl<C: hyper::client::Connect>MonthsApi for MonthsApiClient<C> {
    fn get_budget_month(&self, budget_id: &str, month: &chrono::NaiveDate) -> Box<Future<Item = ::models::MonthDetailResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/months/{month}", configuration.base_path, budget_id=budget_id, month=month);

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
                let parsed: Result<::models::MonthDetailResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_budget_months(&self, budget_id: &str) -> Box<Future<Item = ::models::MonthSummariesResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/months", configuration.base_path, budget_id=budget_id);

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
                let parsed: Result<::models::MonthSummariesResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
