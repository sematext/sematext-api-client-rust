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
pub struct EventDto {
  #[serde(rename = "from")]
  #[serde(default)]
  from: Option<Value>, 
  #[serde(rename = "to")]
  #[serde(default)]
  to: Option<Value>, 
  #[serde(rename = "type")]
  #[serde(default)]
  _type: Option<String> 
}

impl EventDto {
  pub fn new() -> EventDto {
    EventDto {
      from: None,
      to: None,
      _type: None
    }
  }

  pub fn set_from(&mut self, from: Value) {
    self.from = Some(from);
  }

  pub fn with_from(mut self, from: Value) -> EventDto {
    self.from = Some(from);
    self
  }

  pub fn from(&self) -> Option<&Value> {
    self.from.as_ref()
  }

  pub fn reset_from(&mut self) {
    self.from = None;
  }

  pub fn set_to(&mut self, to: Value) {
    self.to = Some(to);
  }

  pub fn with_to(mut self, to: Value) -> EventDto {
    self.to = Some(to);
    self
  }

  pub fn to(&self) -> Option<&Value> {
    self.to.as_ref()
  }

  pub fn reset_to(&mut self) {
    self.to = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> EventDto {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }


  pub fn validate(&self) {
  }

}


