/* 
 * Sematext Cloud API
 *
 * API Explorer provides access and documentation for Sematext REST API. The REST API requires the API Key to be sent as part of `Authorization` header. E.g.: `Authorization : apiKey e5f18450-205a-48eb-8589-7d49edaea813`.
 *
 * OpenAPI spec version: v3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */
#![allow(unused_imports)]

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{Date, NaiveDateTime, NaiveDate, DateTime, FixedOffset, Utc};

use crate::models::*;
use crate::date_serializer;
use crate::date_serializer_opt;
use crate::serialize_quoted_numbers;
use crate::serialize_quoted_numbers_opt;
//Uncomment this to deal with limited rfc support on server side
//use crate::datetime_serializer::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AlertNotificationRequest {
  #[serde(rename = "defaultInterval")]
  #[serde(default)]
  default_interval: Option<i64>, 
  #[serde(rename = "end")]
  #[serde(default)]
  end: Option<String>, 
  #[serde(rename = "interval")]
  #[serde(default)]
  interval: Option<String>, 
  #[serde(rename = "start")]
  #[serde(default)]
  start: Option<String> 
}

impl AlertNotificationRequest {
  pub fn new() -> AlertNotificationRequest {
    AlertNotificationRequest {
      default_interval: None,
      end: None,
      interval: None,
      start: None
    }
  }

  pub fn set_default_interval(&mut self, default_interval: i64) {
    self.default_interval = Some(default_interval);
  }

  pub fn with_default_interval(mut self, default_interval: i64) -> AlertNotificationRequest {
    self.default_interval = Some(default_interval);
    self
  }

  pub fn default_interval(&self) -> Option<&i64> {
    self.default_interval.as_ref()
  }

  pub fn reset_default_interval(&mut self) {
    self.default_interval = None;
  }

  pub fn set_end(&mut self, end: String) {
    self.end = Some(end);
  }

  pub fn with_end(mut self, end: String) -> AlertNotificationRequest {
    self.end = Some(end);
    self
  }

  pub fn end(&self) -> Option<&String> {
    self.end.as_ref()
  }

  pub fn reset_end(&mut self) {
    self.end = None;
  }

  pub fn set_interval(&mut self, interval: String) {
    self.interval = Some(interval);
  }

  pub fn with_interval(mut self, interval: String) -> AlertNotificationRequest {
    self.interval = Some(interval);
    self
  }

  pub fn interval(&self) -> Option<&String> {
    self.interval.as_ref()
  }

  pub fn reset_interval(&mut self) {
    self.interval = None;
  }

  pub fn set_start(&mut self, start: String) {
    self.start = Some(start);
  }

  pub fn with_start(mut self, start: String) -> AlertNotificationRequest {
    self.start = Some(start);
    self
  }

  pub fn start(&self) -> Option<&String> {
    self.start.as_ref()
  }

  pub fn reset_start(&mut self) {
    self.start = None;
  }


  pub fn validate(&self) {
  }

}


