/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.AppDescription;
import io.swagger.client.model.AppResponse;
import io.swagger.client.model.AppTypesResponse;
import io.swagger.client.model.AppsResponse;
import io.swagger.client.model.Generic Map Based Api Response;
import io.swagger.client.model.Invitation;
import io.swagger.client.model.UpdateAppInfo;
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
 * API tests for AppsApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> AppsApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    AppsApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * delete
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn delete_using_delete_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let any_state_app_id: i64 = serde_json::from_value(value).unwrap();
    let response: GenericMapBasedApiResponse = api_client.delete_using_delete(any_state_app_id).await.unwrap();
}
/**
 * Get all App types supported for the account identified with apiKey
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_app_types_using_get_test() {
    let api_client = get_client();
    let response: AppTypesResponse = api_client.get_app_types_using_get().await.unwrap();
}
/**
 * Gets defails for one particular App
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_using_get1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let any_state_app_id: i64 = serde_json::from_value(value).unwrap();
    let response: AppResponse = api_client.get_using_get1(any_state_app_id).await.unwrap();
}
/**
 * Invite guests to an app
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn invite_app_guests_using_post1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: Invitation = serde_json::from_value(value).unwrap();
    let response: GenericMapBasedApiResponse = api_client.invite_app_guests_using_post1(body).await.unwrap();
}
/**
 * Get all users of apps accessible to this account
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn list_apps_users_using_get_test() {
    let api_client = get_client();
    let response: AppsResponse = api_client.list_apps_users_using_get().await.unwrap();
}
/**
 * Get all apps accessible by account identified with apiKey
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn list_using_get1_test() {
    let api_client = get_client();
    let response: AppsResponse = api_client.list_using_get1().await.unwrap();
}
/**
 * Update description of the app
 *
 * App can be in any state
 *
 */
#[tokio::test(core_threads = 3)]
async fn update_description_using_put1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let any_state_app_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let body: AppDescription = serde_json::from_value(value).unwrap();
    let response: AppResponse = api_client.update_description_using_put1(any_state_app_id, body).await.unwrap();
}
/**
 * Update app
 *
 * App can be in any state
 *
 */
#[tokio::test(core_threads = 3)]
async fn update_using_put3_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: UpdateAppInfo = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let any_state_app_id: i64 = serde_json::from_value(value).unwrap();
    let response: AppResponse = api_client.update_using_put3(body, any_state_app_id).await.unwrap();
}
