/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.UsageResponse;
*/
use crate::models::*;
use crate::apis::*;
use crate::apis::configuration::Configuration;
use hyper::{
    client::connect::dns::GaiResolver,
    client::{Client, HttpConnector}
};
use std::sync::Arc;
use serde_json::json;

/**
 * API tests for LogsUsageApiControllerApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> LogsUsageApiControllerApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    LogsUsageApiControllerApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * getForRange
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_for_range_using_get_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to: i64 = serde_json::from_value(value).unwrap();
    let response: UsageResponse = api_client.get_for_range_using_get(app_id, from, to).await.unwrap();
}
