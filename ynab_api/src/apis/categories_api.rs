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
use serde_json;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct CategoriesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> CategoriesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> CategoriesApiClient<C> {
        CategoriesApiClient {
            configuration: configuration,
        }
    }
}

//@@@ A datatype that takes just a month or "current" would be good, INTEAD OF NAIVEDATE
pub trait CategoriesApi {
    fn get_categories(&self, budget_id: &str) -> Box<Future<Item = ::models::CategoriesResponse, Error = Error>>;
    fn get_category_by_id(&self, budget_id: &str, category_id: &str) -> Box<Future<Item = ::models::CategoryResponse, Error = Error>>;
    fn get_month_category_by_id(&self, budget_id: &str, month: &chrono::NaiveDate, category_id: &str) -> Box<Future<Item = ::models::CategoryResponse, Error = Error>>;
    fn update_month_category(&self, budget_id: &str, month: &chrono::NaiveDate, category_id: &str, month_category: ::models::SaveMonthCategoryWrapper) -> Box<Future<Item = ::models::CategoryResponse, Error = Error>>;
}


impl<C: hyper::client::Connect>CategoriesApi for CategoriesApiClient<C> {
    fn get_categories(&self, budget_id: &str) -> Box<Future<Item = ::models::CategoriesResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/categories", configuration.base_path, budget_id=budget_id);

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
                let parsed: Result<::models::CategoriesResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_category_by_id(&self, budget_id: &str, category_id: &str) -> Box<Future<Item = ::models::CategoryResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/categories/{category_id}", configuration.base_path, budget_id=budget_id, category_id=category_id);

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
                let parsed: Result<::models::CategoryResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_month_category_by_id(&self, budget_id: &str, month: &chrono::NaiveDate, category_id: &str) -> Box<Future<Item = ::models::CategoryResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/budgets/{budget_id}/months/{month}/categories/{category_id}", configuration.base_path, budget_id=budget_id, month=month, category_id=category_id);
        println!("@@@ uri_str={}", uri_str);

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
                let parsed: Result<::models::CategoryResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn update_month_category(&self, budget_id: &str, month: &chrono::NaiveDate, category_id: &str, month_category: ::models::SaveMonthCategoryWrapper) -> Box<Future<Item = ::models::CategoryResponse, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Patch;

        let uri_str = format!("{}/budgets/{budget_id}/months/{month}/categories/{category_id}", configuration.base_path, budget_id=budget_id, month=month, category_id=category_id);
        println!("@@@ uri_str={}", uri_str);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = configuration.new_request(method, uri.unwrap());


        let serialized = serde_json::to_string(&month_category).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::CategoryResponse, _> = super::json_from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}