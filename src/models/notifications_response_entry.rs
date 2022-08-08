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
pub struct NotificationsResponseEntry {
  #[serde(rename = "end")]
  #[serde(default)]
  end: Option<String>, 
  #[serde(rename = "notifications")]
  #[serde(default)]
  notifications: Option<Vec<AlertNotification>>, 
  #[serde(rename = "start")]
  #[serde(default)]
  start: Option<String> 
}

impl NotificationsResponseEntry {
  pub fn new() -> NotificationsResponseEntry {
    NotificationsResponseEntry {
      end: None,
      notifications: None,
      start: None
    }
  }

  pub fn set_end(&mut self, end: String) {
    self.end = Some(end);
  }

  pub fn with_end(mut self, end: String) -> NotificationsResponseEntry {
    self.end = Some(end);
    self
  }

  pub fn end(&self) -> Option<&String> {
    self.end.as_ref()
  }

  pub fn reset_end(&mut self) {
    self.end = None;
  }

  pub fn set_notifications(&mut self, notifications: Vec<AlertNotification>) {
    self.notifications = Some(notifications);
  }

  pub fn with_notifications(mut self, notifications: Vec<AlertNotification>) -> NotificationsResponseEntry {
    self.notifications = Some(notifications);
    self
  }

  pub fn notifications(&self) -> Option<&Vec<AlertNotification>> {
    self.notifications.as_ref()
  }

  pub fn reset_notifications(&mut self) {
    self.notifications = None;
  }

  pub fn set_start(&mut self, start: String) {
    self.start = Some(start);
  }

  pub fn with_start(mut self, start: String) -> NotificationsResponseEntry {
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


