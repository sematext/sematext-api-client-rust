/*
 * Sematext Cloud API
 *
 * API Explorer provides access and documentation for Sematext REST API. The REST API requires the API Key to be sent as part of `Authorization` header. E.g.: `Authorization : apiKey e5f18450-205a-48eb-8589-7d49edaea813`.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct MetricsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> MetricsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> MetricsApiClient<C> {
        MetricsApiClient {
            configuration,
        }
    }
}

pub trait MetricsApi {
    fn list_data_series_using_post1(&self, app_id: i64, request_body: crate::models::DataSeriesRequest) -> Box<dyn Future<Item = crate::models::GenericApiResponse, Error = Error<serde_json::Value>>>;
    fn list_filters_using_post1(&self, app_id: i64, request_body: crate::models::DataSeriesRequest) -> Box<dyn Future<Item = crate::models::GenericApiResponse, Error = Error<serde_json::Value>>>;
    fn list_metrics_keys_using_get1(&self, app_id: i64) -> Box<dyn Future<Item = crate::models::GenericApiResponse, Error = Error<serde_json::Value>>>;
    fn list_metrics_using_get(&self, app_id: i64) -> Box<dyn Future<Item = crate::models::GenericApiResponse, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>MetricsApi for MetricsApiClient<C> {
    fn list_data_series_using_post1(&self, app_id: i64, request_body: crate::models::DataSeriesRequest) -> Box<dyn Future<Item = crate::models::GenericApiResponse, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/spm-reports/api/v3/apps/{appId}/metrics/data".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        req = req.with_path_param("appId".to_string(), app_id.to_string());
        req = req.with_body_param(request_body);

        req.execute(self.configuration.borrow())
    }

    fn list_filters_using_post1(&self, app_id: i64, request_body: crate::models::DataSeriesRequest) -> Box<dyn Future<Item = crate::models::GenericApiResponse, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/spm-reports/api/v3/apps/{appId}/metrics/filters".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        req = req.with_path_param("appId".to_string(), app_id.to_string());
        req = req.with_body_param(request_body);

        req.execute(self.configuration.borrow())
    }

    fn list_metrics_keys_using_get1(&self, app_id: i64) -> Box<dyn Future<Item = crate::models::GenericApiResponse, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/spm-reports/api/v3/apps/{appId}/metrics/keys".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        req = req.with_path_param("appId".to_string(), app_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn list_metrics_using_get(&self, app_id: i64) -> Box<dyn Future<Item = crate::models::GenericApiResponse, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/spm-reports/api/v3/apps/{appId}/metrics".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        req = req.with_path_param("appId".to_string(), app_id.to_string());

        req.execute(self.configuration.borrow())
    }

}
