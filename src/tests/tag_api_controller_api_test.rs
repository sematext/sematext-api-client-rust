/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.Dimension;
import io.swagger.client.model.TagNamesResponse;
import io.swagger.client.model.TagsGroupedResponse;
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
 * API tests for TagApiControllerApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> TagApiControllerApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    TagApiControllerApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Gets tag names for the given application identifiers appearing in the given time frame.
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_tag_names_using_get_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_ids: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let metrics: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let logs: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let events: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let rum: bool = serde_json::from_value(value).unwrap();
    let response: TagNamesResponse = api_client.get_tag_names_using_get(app_ids, from, to, metrics, logs, events, rum).await.unwrap();
}
/**
 * Gets tag names grouped by application id.
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_tags_grouped_by_id_using_get1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_ids: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let tag: Vec<String> = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let metrics: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let logs: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let events: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let rum: bool = serde_json::from_value(value).unwrap();
    let response: TagsGroupedResponse = api_client.get_tags_grouped_by_id_using_get1(app_ids, tag, from, to, metrics, logs, events, rum).await.unwrap();
}
/**
 * Gets values for specified tags for the given application identifiers appearing in the given time frame.
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_using_get_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_ids: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let tag: Vec<String> = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let metrics: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let logs: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let events: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let rum: bool = serde_json::from_value(value).unwrap();
    let response: ::std::collections::HashMap<String, Dimension> = api_client.get_using_get(app_ids, tag, from, to, metrics, logs, events, rum).await.unwrap();
}
/**
 * Gets values for specified tags for the given application identifiers appearing in the given time frame.
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_using_get1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_ids: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let tag: Vec<String> = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let from: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let to: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let metrics: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let logs: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let events: bool = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let rum: bool = serde_json::from_value(value).unwrap();
    let response: ::std::collections::HashMap<String, Dimension> = api_client.get_using_get1(app_ids, tag, from, to, metrics, logs, events, rum).await.unwrap();
}
