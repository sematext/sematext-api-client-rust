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
pub struct Dimension {
  #[serde(rename = "name")]
  name: String, 
  #[serde(rename = "values")]
  values: Vec<String> 
}

impl Dimension {
  pub fn new(name: String, values: Vec<String>, ) -> Dimension {
    Dimension {
      name: name,
      values: values
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Dimension {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_values(&mut self, values: Vec<String>) {
    self.values = values;
  }

  pub fn with_values(mut self, values: Vec<String>) -> Dimension {
    self.values = values;
    self
  }

  pub fn values(&self) -> &Vec<String> {
    &self.values
  }



  pub fn validate(&self) {
  }

}


