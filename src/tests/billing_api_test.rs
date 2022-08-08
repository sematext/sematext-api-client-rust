/*
package io.swagger.client.api;

import .ApiException;
import io.swagger.client.model.BillingInfo;
import io.swagger.client.model.InvoiceResponse;
import io.swagger.client.model.PlansResponse;
import io.swagger.client.model.UpdatePlanResponse;
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
 * API tests for BillingApi
 */

fn get_token() -> String {
    "your_token".to_string()
}

fn get_client() -> BillingApiClient<HttpConnector<GaiResolver>> {
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);
    let mut conf = Configuration::new(client);
    conf.oauth_access_token = Some(get_token()); //Must be provided

    BillingApiClient::new(Arc::new(conf))
}
/*
*/

/**
 * Get invoice details
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn get_detailed_invoice_using_get1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let service: String = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let year: i32 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let month: i32 = serde_json::from_value(value).unwrap();
    let response: InvoiceResponse = api_client.get_detailed_invoice_using_get1(service, year, month).await.unwrap();
}
/**
 * Get available plans
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn list_available_plans_using_get1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let integration_id: i64 = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_type: String = serde_json::from_value(value).unwrap();
    let response: PlansResponse = api_client.list_available_plans_using_get1(integration_id, app_type).await.unwrap();
}
/**
 * Update plan for an app
 *
 * 
 *
 */
#[tokio::test(core_threads = 3)]
async fn update_plan_using_put1_test() {
    let api_client = get_client();
    let value = json!(/*Put test json here*/);
    let body: BillingInfo = serde_json::from_value(value).unwrap();
    let value = json!(/*Put test json here*/);
    let app_id: i64 = serde_json::from_value(value).unwrap();
    let response: UpdatePlanResponse = api_client.update_plan_using_put1(body, app_id).await.unwrap();
}
