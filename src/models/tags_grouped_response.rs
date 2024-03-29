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
pub struct TagsGroupedResponse {
  #[serde(rename = "v")]
  v: Vec<TagsGrouped> 
}

impl TagsGroupedResponse {
  pub fn new(v: Vec<TagsGrouped>, ) -> TagsGroupedResponse {
    TagsGroupedResponse {
      v: v
    }
  }

  pub fn set_v(&mut self, v: Vec<TagsGrouped>) {
    self.v = v;
  }

  pub fn with_v(mut self, v: Vec<TagsGrouped>) -> TagsGroupedResponse {
    self.v = v;
    self
  }

  pub fn v(&self) -> &Vec<TagsGrouped> {
    &self.v
  }



  pub fn validate(&self) {
  }

}


