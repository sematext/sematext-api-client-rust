/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.Generic Map Based Api Response;
import io.swagger.client.model.UserInfo;
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
 * API tests for ResetPasswordApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> ResetPasswordApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    ResetPasswordApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Reset Password
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn reset_password_using_post_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: UserInfo = serde_json::from_value(value).unwrap();
    let response: GenericMapBasedApiResponse = api_client.reset_password_using_post(body).await.unwrap();
}
