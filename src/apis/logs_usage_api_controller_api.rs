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

pub struct LogsUsageApiControllerApiClient<C: hyper::client::connect::Connect + Clone + Send + Sync> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static> LogsUsageApiControllerApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> LogsUsageApiControllerApiClient<C> {
        LogsUsageApiControllerApiClient {
            configuration: configuration,
        }
    }
}

#[async_trait::async_trait]
pub trait LogsUsageApiControllerApi {
    async fn get_for_range_using_get(&self, app_id: i64, from: i64, to: i64) -> Result<UsageResponse, Error<serde_json::Value>>;
}

#[async_trait::async_trait]
impl<C: hyper::client::connect::Connect + Clone + Send + Sync + 'static>LogsUsageApiControllerApi for LogsUsageApiControllerApiClient<C> {
 ///
 /// getForRange
 ///
 /// 
 ///
 /// # Arguments
 ///
 /// * `app_id` - getForRange
 /// 
 ///
 /// * `from` - getForRange
 /// 
 ///
 /// * `to` - getForRange
 /// 
 ///
    async fn get_for_range_using_get(&self, app_id: i64, from: i64, to: i64) -> Result<UsageResponse, Error<serde_json::Value>> {
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
        let uri_str = format!("{}logsene-reports/api/v3/apps/{appId}/usage/{from}/{to}{}", configuration.base_path, query_string, appId=app_id, from=from, to=to);

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

}
