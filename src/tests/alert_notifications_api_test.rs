/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.AlertNotificationRequest;
import io.swagger.client.model.NotificationsResponse;
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
 * API tests for AlertNotificationsApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> AlertNotificationsApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    AlertNotificationsApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Get alert notifications for an app
 *
 * Default value of interval is 1d
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_alert_notifications_for_app_using_post1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: AlertNotificationRequest = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: NotificationsResponse = api_client.get_alert_notifications_for_app_using_post1(body, app_id).await.unwrap();
}
/**
 * Get alert notifications for a user
 *
 * Default value of interval is 1d
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_alert_notifications_for_user_using_post1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: AlertNotificationRequest = serde_json::from_value(value).unwrap();
    let response: NotificationsResponse = api_client.get_alert_notifications_for_user_using_post1(body).await.unwrap();
}
