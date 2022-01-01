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

pub struct PayeeLocationsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PayeeLocationsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PayeeLocationsApiClient<C> {
        PayeeLocationsApiClient {
            configuration: configuration,
        }
    }
}

pub trait PayeeLocationsApi {
    fn get_payee_location_by_id(&self, budget_id: &str, payee_location_id: &str) -> Box<dyn (Future<Item = ::models::PayeeLocationResponse, Error = Error>)>;
    fn get_payee_locations(&self, budget_id: &str) -> Box<dyn (Future<Item = ::models::PayeeLocationsResponse, Error = Error>)>;
    fn get_payee_locations_by_payee(&self, budget_id: &str, payee_id: &str) -> Box<dyn (Future<Item = ::models::PayeeLocationsResponse, Error = Error>)>;
}


impl<C: hyper::client::Connect>PayeeLocationsApi for PayeeLocationsApiClient<C> {
    fn get_payee_location_by_id(&self, budget_id: &str, payee_location_id: &str) -> Box<dyn (Future<Item = ::models::PayeeLocationResponse, Error = Error>)> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/payee_locations/{payee_location_id}", configuration.base_path, budget_id=budget_id, payee_location_id=payee_location_id);

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
                let parsed: Result<::models::PayeeLocationResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_payee_locations(&self, budget_id: &str) -> Box<dyn (Future<Item = ::models::PayeeLocationsResponse, Error = Error>)> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/payee_locations", configuration.base_path, budget_id=budget_id);

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
                let parsed: Result<::models::PayeeLocationsResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_payee_locations_by_payee(&self, budget_id: &str, payee_id: &str) -> Box<dyn (Future<Item = ::models::PayeeLocationsResponse, Error = Error>)> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/payees/{payee_id}/payee_locations", configuration.base_path, budget_id=budget_id, payee_id=payee_id);

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
                let parsed: Result<::models::PayeeLocationsResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
