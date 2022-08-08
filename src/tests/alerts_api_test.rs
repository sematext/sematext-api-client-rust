/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.AlertRule;
import io.swagger.client.model.AlertRuleResponse;
import io.swagger.client.model.AlertRulesResponse;
import io.swagger.client.model.Generic Map Based Api Response;
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
 * API tests for AlertsApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> AlertsApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    AlertsApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Create alert rule
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn create_alert_using_post_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: AlertRule = serde_json::from_value(value).unwrap();
    let response: AlertRuleResponse = api_client.create_alert_using_post(body).await.unwrap();
}
/**
 * Delete alert rule
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn delete_alert_rule_using_delete_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let updateable_alert_id: i64 = serde_json::from_value(value).unwrap();
    let response: GenericMapBasedApiResponse = api_client.delete_alert_rule_using_delete(updateable_alert_id).await.unwrap();
}
/**
 * Disable alert rule
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn disable_alert_rule_using_put_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let updateable_alert_id: i64 = serde_json::from_value(value).unwrap();
    let response: GenericMapBasedApiResponse = api_client.disable_alert_rule_using_put(updateable_alert_id).await.unwrap();
}
/**
 * Enable alert rule
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn enable_alert_rule_using_put_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let updateable_alert_id: i64 = serde_json::from_value(value).unwrap();
    let response: GenericMapBasedApiResponse = api_client.enable_alert_rule_using_put(updateable_alert_id).await.unwrap();
}
/**
 * Get alert rules for an app
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_alert_rules_for_app_using_get_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: AlertRulesResponse = api_client.get_alert_rules_for_app_using_get(app_id).await.unwrap();
}
