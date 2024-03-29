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
pub struct PlansResponseEntry {
  #[serde(rename = "availablePlans")]
  #[serde(default)]
  available_plans: Option<Vec<Plan>> 
}

impl PlansResponseEntry {
  pub fn new() -> PlansResponseEntry {
    PlansResponseEntry {
      available_plans: None
    }
  }

  pub fn set_available_plans(&mut self, available_plans: Vec<Plan>) {
    self.available_plans = Some(available_plans);
  }

  pub fn with_available_plans(mut self, available_plans: Vec<Plan>) -> PlansResponseEntry {
    self.available_plans = Some(available_plans);
    self
  }

  pub fn available_plans(&self) -> Option<&Vec<Plan>> {
    self.available_plans.as_ref()
  }

  pub fn reset_available_plans(&mut self) {
    self.available_plans = None;
  }


  pub fn validate(&self) {
  }

}


