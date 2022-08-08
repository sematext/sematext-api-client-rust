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
pub struct TagsGrouped {
  #[serde(rename = "appId")]
  app_id: i64, 
  #[serde(rename = "tags")]
  tags: ::std::collections::HashMap<String, Dimension> 
}

impl TagsGrouped {
  pub fn new(app_id: i64, tags: ::std::collections::HashMap<String, Dimension>, ) -> TagsGrouped {
    TagsGrouped {
      app_id: app_id,
      tags: tags
    }
  }

  pub fn set_app_id(&mut self, app_id: i64) {
    self.app_id = app_id;
  }

  pub fn with_app_id(mut self, app_id: i64) -> TagsGrouped {
    self.app_id = app_id;
    self
  }

  pub fn app_id(&self) -> &i64 {
    &self.app_id
  }


  pub fn set_tags(&mut self, tags: ::std::collections::HashMap<String, Dimension>) {
    self.tags = tags;
  }

  pub fn with_tags(mut self, tags: ::std::collections::HashMap<String, Dimension>) -> TagsGrouped {
    self.tags = tags;
    self
  }

  pub fn tags(&self) -> &::std::collections::HashMap<String, Dimension> {
    &self.tags
  }



  pub fn validate(&self) {
  }

}


