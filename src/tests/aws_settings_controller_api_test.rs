/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.CloudWatchSettings;
import io.swagger.client.model.CloudWatchSettingsResponse;
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
 * API tests for AwsSettingsControllerApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> AwsSettingsControllerApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    AwsSettingsControllerApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Update App&#x27;s AWS CloudWatch settings
 *
 * Applicable only for AWS Apps
 *
 */
#[tokio::test(core_threads = 3)]
async fn update_using_put_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: CloudWatchSettings = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: CloudWatchSettingsResponse = api_client.update_using_put(body, app_id).await.unwrap();
}
