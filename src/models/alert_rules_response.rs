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
pub struct AlertRulesResponse {
  #[serde(rename = "data")]
  #[serde(default)]
  data: Option<AlertRulesResponseEntry>, 
  #[serde(rename = "errors")]
  #[serde(default)]
  errors: Option<Vec<Error>>, 
  #[serde(rename = "message")]
  #[serde(default)]
  message: Option<String>, 
  #[serde(rename = "success")]
  #[serde(default)]
  success: Option<bool> 
}

impl AlertRulesResponse {
  pub fn new() -> AlertRulesResponse {
    AlertRulesResponse {
      data: None,
      errors: None,
      message: None,
      success: None
    }
  }

  pub fn set_data(&mut self, data: AlertRulesResponseEntry) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: AlertRulesResponseEntry) -> AlertRulesResponse {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&AlertRulesResponseEntry> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_errors(&mut self, errors: Vec<Error>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<Error>) -> AlertRulesResponse {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> Option<&Vec<Error>> {
    self.errors.as_ref()
  }

  pub fn reset_errors(&mut self) {
    self.errors = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> AlertRulesResponse {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = Some(success);
  }

  pub fn with_success(mut self, success: bool) -> AlertRulesResponse {
    self.success = Some(success);
    self
  }

  pub fn success(&self) -> Option<&bool> {
    self.success.as_ref()
  }

  pub fn reset_success(&mut self) {
    self.success = None;
  }


  pub fn validate(&self) {
  }

}


