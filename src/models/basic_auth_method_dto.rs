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
pub struct BasicAuthMethodDto {
  #[serde(rename = "authType")]
  #[serde(default)]
  auth_type: Option<String>, 
  #[serde(rename = "uuid")]
  #[serde(default)]
  uuid: Option<String> 
}

impl BasicAuthMethodDto {
  pub fn new() -> BasicAuthMethodDto {
    BasicAuthMethodDto {
      auth_type: None,
      uuid: None
    }
  }

  pub fn set_auth_type(&mut self, auth_type: String) {
    self.auth_type = Some(auth_type);
  }

  pub fn with_auth_type(mut self, auth_type: String) -> BasicAuthMethodDto {
    self.auth_type = Some(auth_type);
    self
  }

  pub fn auth_type(&self) -> Option<&String> {
    self.auth_type.as_ref()
  }

  pub fn reset_auth_type(&mut self) {
    self.auth_type = None;
  }

  pub fn set_uuid(&mut self, uuid: String) {
    self.uuid = Some(uuid);
  }

  pub fn with_uuid(mut self, uuid: String) -> BasicAuthMethodDto {
    self.uuid = Some(uuid);
    self
  }

  pub fn uuid(&self) -> Option<&String> {
    self.uuid.as_ref()
  }

  pub fn reset_uuid(&mut self) {
    self.uuid = None;
  }


  pub fn validate(&self) {
  }

}


