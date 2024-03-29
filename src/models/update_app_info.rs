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
pub struct UpdateAppInfo {
  #[serde(rename = "description")]
  #[serde(default)]
  description: Option<String>,  // New Description of App 
  #[serde(rename = "ignorePercentage")]
  #[serde(default)]
  ignore_percentage: Option<f64>,  // 1.0 
  #[serde(rename = "maxEvents")]
  #[serde(default)]
  max_events: Option<i64>,  // 1000 
  #[serde(rename = "maxLimitMB")]
  #[serde(default)]
  max_limit_mb: Option<i64>,  // 10 
  #[serde(rename = "name")]
  #[serde(default)]
  name: Option<String>,  // New Name 
  #[serde(rename = "sampling")]
  #[serde(default)]
  sampling: Option<bool>,  // false 
  #[serde(rename = "samplingPercentage")]
  #[serde(default)]
  sampling_percentage: Option<i32>,  // 10 
  #[serde(rename = "staggering")]
  #[serde(default)]
  staggering: Option<bool>,  // false 
  #[serde(rename = "status")]
  #[serde(default)]
  status: Option<String>  // ACTIVE 
}

impl UpdateAppInfo {
  pub fn new() -> UpdateAppInfo {
    UpdateAppInfo {
      description: None,
      ignore_percentage: None,
      max_events: None,
      max_limit_mb: None,
      name: None,
      sampling: None,
      sampling_percentage: None,
      staggering: None,
      status: None
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> UpdateAppInfo {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_ignore_percentage(&mut self, ignore_percentage: f64) {
    self.ignore_percentage = Some(ignore_percentage);
  }

  pub fn with_ignore_percentage(mut self, ignore_percentage: f64) -> UpdateAppInfo {
    self.ignore_percentage = Some(ignore_percentage);
    self
  }

  pub fn ignore_percentage(&self) -> Option<&f64> {
    self.ignore_percentage.as_ref()
  }

  pub fn reset_ignore_percentage(&mut self) {
    self.ignore_percentage = None;
  }

  pub fn set_max_events(&mut self, max_events: i64) {
    self.max_events = Some(max_events);
  }

  pub fn with_max_events(mut self, max_events: i64) -> UpdateAppInfo {
    self.max_events = Some(max_events);
    self
  }

  pub fn max_events(&self) -> Option<&i64> {
    self.max_events.as_ref()
  }

  pub fn reset_max_events(&mut self) {
    self.max_events = None;
  }

  pub fn set_max_limit_mb(&mut self, max_limit_mb: i64) {
    self.max_limit_mb = Some(max_limit_mb);
  }

  pub fn with_max_limit_mb(mut self, max_limit_mb: i64) -> UpdateAppInfo {
    self.max_limit_mb = Some(max_limit_mb);
    self
  }

  pub fn max_limit_mb(&self) -> Option<&i64> {
    self.max_limit_mb.as_ref()
  }

  pub fn reset_max_limit_mb(&mut self) {
    self.max_limit_mb = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> UpdateAppInfo {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_sampling(&mut self, sampling: bool) {
    self.sampling = Some(sampling);
  }

  pub fn with_sampling(mut self, sampling: bool) -> UpdateAppInfo {
    self.sampling = Some(sampling);
    self
  }

  pub fn sampling(&self) -> Option<&bool> {
    self.sampling.as_ref()
  }

  pub fn reset_sampling(&mut self) {
    self.sampling = None;
  }

  pub fn set_sampling_percentage(&mut self, sampling_percentage: i32) {
    self.sampling_percentage = Some(sampling_percentage);
  }

  pub fn with_sampling_percentage(mut self, sampling_percentage: i32) -> UpdateAppInfo {
    self.sampling_percentage = Some(sampling_percentage);
    self
  }

  pub fn sampling_percentage(&self) -> Option<&i32> {
    self.sampling_percentage.as_ref()
  }

  pub fn reset_sampling_percentage(&mut self) {
    self.sampling_percentage = None;
  }

  pub fn set_staggering(&mut self, staggering: bool) {
    self.staggering = Some(staggering);
  }

  pub fn with_staggering(mut self, staggering: bool) -> UpdateAppInfo {
    self.staggering = Some(staggering);
    self
  }

  pub fn staggering(&self) -> Option<&bool> {
    self.staggering.as_ref()
  }

  pub fn reset_staggering(&mut self) {
    self.staggering = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> UpdateAppInfo {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }


  pub fn validate(&self) {
  }

}


