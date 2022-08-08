/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.AppsResponse;
import io.swagger.client.model.CreateAppInfo;
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
 * API tests for MonitoringAppApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> MonitoringAppApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    MonitoringAppApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Create Monitoring App
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn create_spm_application1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: CreateAppInfo = serde_json::from_value(value).unwrap();
    let response: AppsResponse = api_client.create_spm_application1(body).await.unwrap();
}
