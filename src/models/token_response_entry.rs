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
pub struct TokenResponseEntry {
  #[serde(rename = "token")]
  #[serde(default)]
  token: Option<TokenDto> 
}

impl TokenResponseEntry {
  pub fn new() -> TokenResponseEntry {
    TokenResponseEntry {
      token: None
    }
  }

  pub fn set_token(&mut self, token: TokenDto) {
    self.token = Some(token);
  }

  pub fn with_token(mut self, token: TokenDto) -> TokenResponseEntry {
    self.token = Some(token);
    self
  }

  pub fn token(&self) -> Option<&TokenDto> {
    self.token.as_ref()
  }

  pub fn reset_token(&mut self) {
    self.token = None;
  }


  pub fn validate(&self) {
  }

}


