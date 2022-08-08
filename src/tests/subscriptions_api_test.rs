/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.Generic Map Based Api Response;
import io.swagger.client.model.MailReportResponse;
import io.swagger.client.model.ReportInfo;
import io.swagger.client.model.SubscriptionDashboardDto;
import io.swagger.client.model.SubscriptionDto;
import io.swagger.client.model.SubscriptionResponse;
import io.swagger.client.model.SubscriptionsResponse;
import io.swagger.client.model.UpdateSubscriptionDto;
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
 * API tests for SubscriptionsApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> SubscriptionsApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    SubscriptionsApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Create App subscription
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn create_for_app_using_post_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: SubscriptionDto = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: SubscriptionResponse = api_client.create_for_app_using_post(body, app_id).await.unwrap();
}
/**
 * Create dashboard subscription
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn create_for_dash_using_post1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: SubscriptionDashboardDto = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let dash_id: i64 = serde_json::from_value(value).unwrap();
    let response: SubscriptionResponse = api_client.create_for_dash_using_post1(body, dash_id).await.unwrap();
}
/**
 * Delete subscription
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn delete_using_delete3_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let updateable_subscription_id: i64 = serde_json::from_value(value).unwrap();
    let response: GenericMapBasedApiResponse = api_client.delete_using_delete3(updateable_subscription_id).await.unwrap();
}
/**
 * Get subscriptions for an App
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn list_using_get2_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: SubscriptionsResponse = api_client.list_using_get2(app_id).await.unwrap();
}
/**
 * Get current account&#x27;s subscriptions
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn list_using_get5_test() {
    let api_client = get_client();
    let response: SubscriptionsResponse = api_client.list_using_get5().await.unwrap();
}
/**
 * Email an App report
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn send_app_report_using_post1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: ReportInfo = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: MailReportResponse = api_client.send_app_report_using_post1(body, app_id).await.unwrap();
}
/**
 * Email a dashboard report
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn send_dash_report_using_post1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: ReportInfo = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let dash_id: i64 = serde_json::from_value(value).unwrap();
    let response: MailReportResponse = api_client.send_dash_report_using_post1(body, dash_id).await.unwrap();
}
/**
 * Toggle subscription status
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn toggle_enabled_using_put_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: UpdateSubscriptionDto = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let updateable_subscription_id: i64 = serde_json::from_value(value).unwrap();
    let response: SubscriptionResponse = api_client.toggle_enabled_using_put(body, updateable_subscription_id).await.unwrap();
}
/**
 * Update App subscription
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn update_for_app_using_put1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: SubscriptionDto = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: SubscriptionResponse = api_client.update_for_app_using_put1(body, app_id).await.unwrap();
}
/**
 * Update dashboard subscription
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn update_for_dash_using_put_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: SubscriptionDashboardDto = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let dash_id: i64 = serde_json::from_value(value).unwrap();
    let response: SubscriptionResponse = api_client.update_for_dash_using_put(body, dash_id).await.unwrap();
}
