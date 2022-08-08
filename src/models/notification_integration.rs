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
pub struct NotificationIntegration {
  #[serde(rename = "applicability")]
  #[serde(default)]
  applicability: Option<String>, 
  #[serde(rename = "createDate")]
  #[serde(default)]
  //Uncomment this also to deal with limited rfc support on server side
  //#[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
  create_date: Option<DateTime<Utc>>, 
  #[serde(rename = "createdByOwner")]
  #[serde(default)]
  created_by_owner: Option<bool>, 
  #[serde(rename = "creatorId")]
  #[serde(default)]
  creator_id: Option<i64>, 
  #[serde(rename = "id")]
  #[serde(default)]
  id: Option<i64>, 
  #[serde(rename = "integrationType")]
  #[serde(default)]
  integration_type: Option<String>, 
  #[serde(rename = "name")]
  #[serde(default)]
  name: Option<String>, 
  #[serde(rename = "params")]
  #[serde(default)]
  params: Option<::std::collections::HashMap<String, String>>, 
  #[serde(rename = "state")]
  #[serde(default)]
  state: Option<String>, 
  #[serde(rename = "userId")]
  #[serde(default)]
  user_id: Option<i64> 
}

impl NotificationIntegration {
  pub fn new() -> NotificationIntegration {
    NotificationIntegration {
      applicability: None,
      create_date: None,
      created_by_owner: None,
      creator_id: None,
      id: None,
      integration_type: None,
      name: None,
      params: None,
      state: None,
      user_id: None
    }
  }

  pub fn set_applicability(&mut self, applicability: String) {
    self.applicability = Some(applicability);
  }

  pub fn with_applicability(mut self, applicability: String) -> NotificationIntegration {
    self.applicability = Some(applicability);
    self
  }

  pub fn applicability(&self) -> Option<&String> {
    self.applicability.as_ref()
  }

  pub fn reset_applicability(&mut self) {
    self.applicability = None;
  }

  pub fn set_create_date(&mut self, create_date: DateTime<Utc>) {
    self.create_date = Some(create_date);
  }

  pub fn with_create_date(mut self, create_date: DateTime<Utc>) -> NotificationIntegration {
    self.create_date = Some(create_date);
    self
  }

  pub fn create_date(&self) -> Option<&DateTime<Utc>> {
    self.create_date.as_ref()
  }

  pub fn reset_create_date(&mut self) {
    self.create_date = None;
  }

  pub fn set_created_by_owner(&mut self, created_by_owner: bool) {
    self.created_by_owner = Some(created_by_owner);
  }

  pub fn with_created_by_owner(mut self, created_by_owner: bool) -> NotificationIntegration {
    self.created_by_owner = Some(created_by_owner);
    self
  }

  pub fn created_by_owner(&self) -> Option<&bool> {
    self.created_by_owner.as_ref()
  }

  pub fn reset_created_by_owner(&mut self) {
    self.created_by_owner = None;
  }

  pub fn set_creator_id(&mut self, creator_id: i64) {
    self.creator_id = Some(creator_id);
  }

  pub fn with_creator_id(mut self, creator_id: i64) -> NotificationIntegration {
    self.creator_id = Some(creator_id);
    self
  }

  pub fn creator_id(&self) -> Option<&i64> {
    self.creator_id.as_ref()
  }

  pub fn reset_creator_id(&mut self) {
    self.creator_id = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> NotificationIntegration {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_integration_type(&mut self, integration_type: String) {
    self.integration_type = Some(integration_type);
  }

  pub fn with_integration_type(mut self, integration_type: String) -> NotificationIntegration {
    self.integration_type = Some(integration_type);
    self
  }

  pub fn integration_type(&self) -> Option<&String> {
    self.integration_type.as_ref()
  }

  pub fn reset_integration_type(&mut self) {
    self.integration_type = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> NotificationIntegration {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_params(&mut self, params: ::std::collections::HashMap<String, String>) {
    self.params = Some(params);
  }

  pub fn with_params(mut self, params: ::std::collections::HashMap<String, String>) -> NotificationIntegration {
    self.params = Some(params);
    self
  }

  pub fn params(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.params.as_ref()
  }

  pub fn reset_params(&mut self) {
    self.params = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> NotificationIntegration {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_user_id(&mut self, user_id: i64) {
    self.user_id = Some(user_id);
  }

  pub fn with_user_id(mut self, user_id: i64) -> NotificationIntegration {
    self.user_id = Some(user_id);
    self
  }

  pub fn user_id(&self) -> Option<&i64> {
    self.user_id.as_ref()
  }

  pub fn reset_user_id(&mut self) {
    self.user_id = None;
  }


  pub fn validate(&self) {
  }

}


