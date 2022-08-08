/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.CreateTokenDto;
import io.swagger.client.model.Generic Map Based Api Response;
import io.swagger.client.model.TokenResponse;
import io.swagger.client.model.TokensResponse;
import io.swagger.client.model.UpdateTokenDto;
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
 * API tests for TokensApiControllerApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> TokensApiControllerApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    TokensApiControllerApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Create new app token
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn create_app_token1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: CreateTokenDto = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: TokenResponse = api_client.create_app_token1(body, app_id).await.unwrap();
}
/**
 * Delete app token
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn delete_app_token1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let token_id: i64 = serde_json::from_value(value).unwrap();
    let response: GenericMapBasedApiResponse = api_client.delete_app_token1(app_id, token_id).await.unwrap();
}
/**
 * Get app available tokens
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_app_tokens1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: TokensResponse = api_client.get_app_tokens1(app_id).await.unwrap();
}
/**
 * Regenerate app token)
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn regenerate_app_token_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let token_id: i64 = serde_json::from_value(value).unwrap();
    let response: TokenResponse = api_client.regenerate_app_token(app_id, token_id).await.unwrap();
}
/**
 * Update app token (enable/disable or name)
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn update_app_token1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: UpdateTokenDto = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let token_id: i64 = serde_json::from_value(value).unwrap();
    let response: TokenResponse = api_client.update_app_token1(body, app_id, token_id).await.unwrap();
}
