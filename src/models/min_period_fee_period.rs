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
pub struct MinPeriodFeePeriod {
  #[serde(rename = "browserMonitors")]
  #[serde(default)]
  browser_monitors: Option<i32>, 
  #[serde(rename = "discount")]
  #[serde(default)]
  discount: Option<f32>, 
  #[serde(rename = "fromDate")]
  #[serde(default)]
  //Uncomment this also to deal with limited rfc support on server side
  //#[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
  from_date: Option<DateTime<Utc>>, 
  #[serde(rename = "httpMonitors")]
  #[serde(default)]
  http_monitors: Option<i32>, 
  #[serde(rename = "id")]
  #[serde(default)]
  id: Option<i64>, 
  #[serde(rename = "ingestionPeriodFeeAmount")]
  #[serde(default)]
  ingestion_period_fee_amount: Option<f32>, 
  #[serde(rename = "minPeriodFeeAmount")]
  #[serde(default)]
  min_period_fee_amount: Option<f32>, 
  #[serde(rename = "overagePercentage")]
  #[serde(default)]
  overage_percentage: Option<f32>, 
  #[serde(rename = "planDataLimit")]
  #[serde(default)]
  plan_data_limit: Option<i64>, 
  #[serde(rename = "storagePeriodFeeAmount")]
  #[serde(default)]
  storage_period_fee_amount: Option<f32>, 
  #[serde(rename = "toDate")]
  #[serde(default)]
  //Uncomment this also to deal with limited rfc support on server side
  //#[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
  to_date: Option<DateTime<Utc>>, 
  #[serde(rename = "usedPlan")]
  #[serde(default)]
  used_plan: Option<String>, 
  #[serde(rename = "usedPlanPeriodFee")]
  #[serde(default)]
  used_plan_period_fee: Option<f32> 
}

impl MinPeriodFeePeriod {
  pub fn new() -> MinPeriodFeePeriod {
    MinPeriodFeePeriod {
      browser_monitors: None,
      discount: None,
      from_date: None,
      http_monitors: None,
      id: None,
      ingestion_period_fee_amount: None,
      min_period_fee_amount: None,
      overage_percentage: None,
      plan_data_limit: None,
      storage_period_fee_amount: None,
      to_date: None,
      used_plan: None,
      used_plan_period_fee: None
    }
  }

  pub fn set_browser_monitors(&mut self, browser_monitors: i32) {
    self.browser_monitors = Some(browser_monitors);
  }

  pub fn with_browser_monitors(mut self, browser_monitors: i32) -> MinPeriodFeePeriod {
    self.browser_monitors = Some(browser_monitors);
    self
  }

  pub fn browser_monitors(&self) -> Option<&i32> {
    self.browser_monitors.as_ref()
  }

  pub fn reset_browser_monitors(&mut self) {
    self.browser_monitors = None;
  }

  pub fn set_discount(&mut self, discount: f32) {
    self.discount = Some(discount);
  }

  pub fn with_discount(mut self, discount: f32) -> MinPeriodFeePeriod {
    self.discount = Some(discount);
    self
  }

  pub fn discount(&self) -> Option<&f32> {
    self.discount.as_ref()
  }

  pub fn reset_discount(&mut self) {
    self.discount = None;
  }

  pub fn set_from_date(&mut self, from_date: DateTime<Utc>) {
    self.from_date = Some(from_date);
  }

  pub fn with_from_date(mut self, from_date: DateTime<Utc>) -> MinPeriodFeePeriod {
    self.from_date = Some(from_date);
    self
  }

  pub fn from_date(&self) -> Option<&DateTime<Utc>> {
    self.from_date.as_ref()
  }

  pub fn reset_from_date(&mut self) {
    self.from_date = None;
  }

  pub fn set_http_monitors(&mut self, http_monitors: i32) {
    self.http_monitors = Some(http_monitors);
  }

  pub fn with_http_monitors(mut self, http_monitors: i32) -> MinPeriodFeePeriod {
    self.http_monitors = Some(http_monitors);
    self
  }

  pub fn http_monitors(&self) -> Option<&i32> {
    self.http_monitors.as_ref()
  }

  pub fn reset_http_monitors(&mut self) {
    self.http_monitors = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> MinPeriodFeePeriod {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_ingestion_period_fee_amount(&mut self, ingestion_period_fee_amount: f32) {
    self.ingestion_period_fee_amount = Some(ingestion_period_fee_amount);
  }

  pub fn with_ingestion_period_fee_amount(mut self, ingestion_period_fee_amount: f32) -> MinPeriodFeePeriod {
    self.ingestion_period_fee_amount = Some(ingestion_period_fee_amount);
    self
  }

  pub fn ingestion_period_fee_amount(&self) -> Option<&f32> {
    self.ingestion_period_fee_amount.as_ref()
  }

  pub fn reset_ingestion_period_fee_amount(&mut self) {
    self.ingestion_period_fee_amount = None;
  }

  pub fn set_min_period_fee_amount(&mut self, min_period_fee_amount: f32) {
    self.min_period_fee_amount = Some(min_period_fee_amount);
  }

  pub fn with_min_period_fee_amount(mut self, min_period_fee_amount: f32) -> MinPeriodFeePeriod {
    self.min_period_fee_amount = Some(min_period_fee_amount);
    self
  }

  pub fn min_period_fee_amount(&self) -> Option<&f32> {
    self.min_period_fee_amount.as_ref()
  }

  pub fn reset_min_period_fee_amount(&mut self) {
    self.min_period_fee_amount = None;
  }

  pub fn set_overage_percentage(&mut self, overage_percentage: f32) {
    self.overage_percentage = Some(overage_percentage);
  }

  pub fn with_overage_percentage(mut self, overage_percentage: f32) -> MinPeriodFeePeriod {
    self.overage_percentage = Some(overage_percentage);
    self
  }

  pub fn overage_percentage(&self) -> Option<&f32> {
    self.overage_percentage.as_ref()
  }

  pub fn reset_overage_percentage(&mut self) {
    self.overage_percentage = None;
  }

  pub fn set_plan_data_limit(&mut self, plan_data_limit: i64) {
    self.plan_data_limit = Some(plan_data_limit);
  }

  pub fn with_plan_data_limit(mut self, plan_data_limit: i64) -> MinPeriodFeePeriod {
    self.plan_data_limit = Some(plan_data_limit);
    self
  }

  pub fn plan_data_limit(&self) -> Option<&i64> {
    self.plan_data_limit.as_ref()
  }

  pub fn reset_plan_data_limit(&mut self) {
    self.plan_data_limit = None;
  }

  pub fn set_storage_period_fee_amount(&mut self, storage_period_fee_amount: f32) {
    self.storage_period_fee_amount = Some(storage_period_fee_amount);
  }

  pub fn with_storage_period_fee_amount(mut self, storage_period_fee_amount: f32) -> MinPeriodFeePeriod {
    self.storage_period_fee_amount = Some(storage_period_fee_amount);
    self
  }

  pub fn storage_period_fee_amount(&self) -> Option<&f32> {
    self.storage_period_fee_amount.as_ref()
  }

  pub fn reset_storage_period_fee_amount(&mut self) {
    self.storage_period_fee_amount = None;
  }

  pub fn set_to_date(&mut self, to_date: DateTime<Utc>) {
    self.to_date = Some(to_date);
  }

  pub fn with_to_date(mut self, to_date: DateTime<Utc>) -> MinPeriodFeePeriod {
    self.to_date = Some(to_date);
    self
  }

  pub fn to_date(&self) -> Option<&DateTime<Utc>> {
    self.to_date.as_ref()
  }

  pub fn reset_to_date(&mut self) {
    self.to_date = None;
  }

  pub fn set_used_plan(&mut self, used_plan: String) {
    self.used_plan = Some(used_plan);
  }

  pub fn with_used_plan(mut self, used_plan: String) -> MinPeriodFeePeriod {
    self.used_plan = Some(used_plan);
    self
  }

  pub fn used_plan(&self) -> Option<&String> {
    self.used_plan.as_ref()
  }

  pub fn reset_used_plan(&mut self) {
    self.used_plan = None;
  }

  pub fn set_used_plan_period_fee(&mut self, used_plan_period_fee: f32) {
    self.used_plan_period_fee = Some(used_plan_period_fee);
  }

  pub fn with_used_plan_period_fee(mut self, used_plan_period_fee: f32) -> MinPeriodFeePeriod {
    self.used_plan_period_fee = Some(used_plan_period_fee);
    self
  }

  pub fn used_plan_period_fee(&self) -> Option<&f32> {
    self.used_plan_period_fee.as_ref()
  }

  pub fn reset_used_plan_period_fee(&mut self) {
    self.used_plan_period_fee = None;
  }


  pub fn validate(&self) {
  }

}


