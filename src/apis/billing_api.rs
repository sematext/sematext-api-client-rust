/* 
 * Sematext Cloud API
 *
 * API Explorer provides access and documentation for Sematext REST API. The REST API requires the API Key to be sent as part of `Authorization` header. E.g.: `Authorization : apiKey e5f18450-205a-48eb-8589-7d49edaea813`.
 *
 * OpenAPI spec version: v3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */
#![allow(unused_imports)]
use std::sync::Arc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use serde_json::Value;
use tokio::runtime::Runtime;
use futures;
use futures::{Future, Stream};
use bigdecimal::BigDecimal;

use hyper::Body;
use hyper::body::Bytes;
use hyper::body::HttpBody;
use std::str::FromStr;
use chrono::{Date, NaiveDateTime, DateTime, FixedOffset, Utc, SecondsFormat};
use crate::{OutlinePrint};
use crate::models::*;
use super::{Error, configuration};
use headers::{Authorization, Header};
use headers::authorization::Credentials;

pub struct BillingApiClient<C: hyper::client::connect::Connect + Clone + Send + Sync> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> BillingApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> BillingApiClient<C> {
        BillingApiClient {
            configuration: configuration,
        }
    }
}

#[async_trait::async_trait]
pub trait BillingApi {
    async fn get_detailed_invoice_using_get1(&self, service: &str, year: i32, month: i32) -> Result<InvoiceResponse, Error<serde_json::Value>>;
    async fn list_available_plans_using_get1(&self, integration_id: i64, app_type: &str) -> Result<PlansResponse, Error<serde_json::Value>>;
    async fn update_plan_using_put1(&self, body: crate::models::BillingInfo, app_id: i64) -> Result<UpdatePlanResponse, Error<serde_json::Value>>;
}

#[async_trait::async_trait]
impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>BillingApi for BillingApiClient<C> {
 ///
 /// Get invoice details
 ///
 /// 
 ///
 /// # Arguments
 ///
 /// * `service` - Get invoice details
 /// 
 ///
 /// * `year` - Get invoice details
 /// 
 ///
 /// * `month` - Get invoice details
 /// 
 ///
    async fn get_detailed_invoice_using_get1(&self, service: &str, year: i32, month: i32) -> Result<InvoiceResponse, Error<serde_json::Value>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_headers.insert("Authorization".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            let has_query_params = false;
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            if has_query_params || auth_query.len()>0  {
                format!("/?{}", query.finish())
            } else {
                "".to_string()
            }
        };
        let uri_str = format!("{}users-web/api/v3/billing/invoice/{service}/{year}/{month}{}", configuration.base_path, query_string, service=service, year=year, month=month);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);

        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req =
            hyper::Request::builder()
                .method(method)
                .uri(uri);

        let headers = req.headers_mut().unwrap();

        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(
                hyper::header::HeaderName::from_str(key.as_ref()).unwrap(),
                val.parse().unwrap(),
            );
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let res = configuration
            .client.request(req)
            .await
            .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

        let mut res = res?;

        let status = res.status();
        let mut res_body: Vec<u8> = vec![];

        while let Some(chunk) = res.body_mut().data().await {
            let mut chunk_vec = chunk.unwrap().to_vec();
            res_body.append(chunk_vec.as_mut());
        }

        //Uncomment to see what went wrong
/*
        let string_result = std::str::from_utf8(&res_body).unwrap();
        let value_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&string_result);
        if let Ok(json_value) = value_result {
            //Valid json, invalid structure, pretty-printed output
            eprintln!("{}", serde_json::to_string_pretty(&json_value).unwrap());
        } else {
            //Invalid json, raw output
            dbg!(&string_result);
        }
*/
        let res_body =
            if status.is_success() {
                Ok(res_body)
            } else {
                Err(Error::from((status, res_body.borrow())))
            };

        let mut res_body = res_body?;

        let res_body =
            serde_json::from_slice(res_body.borrow())
            .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

        res_body
    }

 ///
 /// Get available plans
 ///
 /// 
 ///
 /// # Arguments
 ///
 /// * `integration_id` - Get available plans
 /// 
 ///
 /// * `app_type` - Get available plans
 /// 
 ///
    async fn list_available_plans_using_get1(&self, integration_id: i64, app_type: &str) -> Result<PlansResponse, Error<serde_json::Value>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_headers.insert("Authorization".to_owned(), val);
        };
        let method = hyper::Method::GET;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            let has_query_params = true;
            query.append_pair("integrationId", &integration_id.outline_print() );
            query.append_pair("appType", &app_type.outline_print() );
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            if has_query_params || auth_query.len()>0  {
                format!("/?{}", query.finish())
            } else {
                "".to_string()
            }
        };
        let uri_str = format!("{}users-web/api/v3/billing/availablePlans{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);

        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req =
            hyper::Request::builder()
                .method(method)
                .uri(uri);

        let headers = req.headers_mut().unwrap();

        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(
                hyper::header::HeaderName::from_str(key.as_ref()).unwrap(),
                val.parse().unwrap(),
            );
        }

        let somebody = Body::empty();

        let req = req.body(somebody).unwrap();

        let res = configuration
            .client.request(req)
            .await
            .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

        let mut res = res?;

        let status = res.status();
        let mut res_body: Vec<u8> = vec![];

        while let Some(chunk) = res.body_mut().data().await {
            let mut chunk_vec = chunk.unwrap().to_vec();
            res_body.append(chunk_vec.as_mut());
        }

        //Uncomment to see what went wrong
/*
        let string_result = std::str::from_utf8(&res_body).unwrap();
        let value_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&string_result);
        if let Ok(json_value) = value_result {
            //Valid json, invalid structure, pretty-printed output
            eprintln!("{}", serde_json::to_string_pretty(&json_value).unwrap());
        } else {
            //Invalid json, raw output
            dbg!(&string_result);
        }
*/
        let res_body =
            if status.is_success() {
                Ok(res_body)
            } else {
                Err(Error::from((status, res_body.borrow())))
            };

        let mut res_body = res_body?;

        let res_body =
            serde_json::from_slice(res_body.borrow())
            .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

        res_body
    }

 ///
 /// Update plan for an app
 ///
 /// 
 ///
 /// # Arguments
 ///
 /// * `body` - Update plan for an app
 /// 
 ///
 /// * `app_id` - Update plan for an app
 /// 
 ///
    async fn update_plan_using_put1(&self, body: crate::models::BillingInfo, app_id: i64) -> Result<UpdatePlanResponse, Error<serde_json::Value>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            auth_headers.insert("Authorization".to_owned(), val);
        };
        let method = hyper::Method::PUT;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            let has_query_params = false;
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            if has_query_params || auth_query.len()>0  {
                format!("/?{}", query.finish())
            } else {
                "".to_string()
            }
        };
        let uri_str = format!("{}users-web/api/v3/billing/info/{appId}{}", configuration.base_path, query_string, appId=app_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        //dbg!(&uri_str);

        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req =
            hyper::Request::builder()
                .method(method)
                .uri(uri);

        let headers = req.headers_mut().unwrap();

        if let Some(ref user_agent) = configuration.user_agent {
            headers.insert(hyper::header::USER_AGENT, user_agent.parse().unwrap());
        }


        for (key, val) in auth_headers {
            headers.insert(
                hyper::header::HeaderName::from_str(key.as_ref()).unwrap(),
                val.parse().unwrap(),
            );
        }

        let somebody = Body::empty();
        let serialized = serde_json::to_string(&body).unwrap();
        //Pretty print request body if needed for some dbg reasons
        //let value_result: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        //eprintln!("\n{}\n", serde_json::to_string_pretty(&value_result).unwrap());
        headers.insert(hyper::header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(hyper::header::CONTENT_LENGTH, format!("{}", serialized.len()).parse().unwrap());
        let somebody = Body::from(serialized);

        let req = req.body(somebody).unwrap();

        let res = configuration
            .client.request(req)
            .await
            .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

        let mut res = res?;

        let status = res.status();
        let mut res_body: Vec<u8> = vec![];

        while let Some(chunk) = res.body_mut().data().await {
            let mut chunk_vec = chunk.unwrap().to_vec();
            res_body.append(chunk_vec.as_mut());
        }

        //Uncomment to see what went wrong
/*
        let string_result = std::str::from_utf8(&res_body).unwrap();
        let value_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&string_result);
        if let Ok(json_value) = value_result {
            //Valid json, invalid structure, pretty-printed output
            eprintln!("{}", serde_json::to_string_pretty(&json_value).unwrap());
        } else {
            //Invalid json, raw output
            dbg!(&string_result);
        }
*/
        let res_body =
            if status.is_success() {
                Ok(res_body)
            } else {
                Err(Error::from((status, res_body.borrow())))
            };

        let mut res_body = res_body?;

        let res_body =
            serde_json::from_slice(res_body.borrow())
            .map_err(|e| -> Error<serde_json::Value> { Error::from(e) });

        res_body
    }

}
