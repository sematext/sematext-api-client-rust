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
pub struct ServiceIntegration {
  #[serde(rename = "appTypeId")]
  #[serde(default)]
  app_type_id: Option<i64>, 
  #[serde(rename = "appTypeName")]
  #[serde(default)]
  app_type_name: Option<String>, 
  #[serde(rename = "displayName")]
  #[serde(default)]
  display_name: Option<String>, 
  #[serde(rename = "enabled")]
  #[serde(default)]
  enabled: Option<bool>, 
  #[serde(rename = "externalProductId")]
  #[serde(default)]
  external_product_id: Option<i64>, 
  #[serde(rename = "externalProductName")]
  #[serde(default)]
  external_product_name: Option<String>, 
  #[serde(rename = "id")]
  #[serde(default)]
  id: Option<i64>, 
  #[serde(rename = "integrationType")]
  #[serde(default)]
  integration_type: Option<String>, 
  #[serde(rename = "ordinal")]
  #[serde(default)]
  ordinal: Option<i32>, 
  #[serde(rename = "parentIntegrationId")]
  #[serde(default)]
  parent_integration_id: Option<i64>, 
  #[serde(rename = "sematextService")]
  #[serde(default)]
  sematext_service: Option<String>, 
  #[serde(rename = "visible")]
  #[serde(default)]
  visible: Option<bool> 
}

impl ServiceIntegration {
  pub fn new() -> ServiceIntegration {
    ServiceIntegration {
      app_type_id: None,
      app_type_name: None,
      display_name: None,
      enabled: None,
      external_product_id: None,
      external_product_name: None,
      id: None,
      integration_type: None,
      ordinal: None,
      parent_integration_id: None,
      sematext_service: None,
      visible: None
    }
  }

  pub fn set_app_type_id(&mut self, app_type_id: i64) {
    self.app_type_id = Some(app_type_id);
  }

  pub fn with_app_type_id(mut self, app_type_id: i64) -> ServiceIntegration {
    self.app_type_id = Some(app_type_id);
    self
  }

  pub fn app_type_id(&self) -> Option<&i64> {
    self.app_type_id.as_ref()
  }

  pub fn reset_app_type_id(&mut self) {
    self.app_type_id = None;
  }

  pub fn set_app_type_name(&mut self, app_type_name: String) {
    self.app_type_name = Some(app_type_name);
  }

  pub fn with_app_type_name(mut self, app_type_name: String) -> ServiceIntegration {
    self.app_type_name = Some(app_type_name);
    self
  }

  pub fn app_type_name(&self) -> Option<&String> {
    self.app_type_name.as_ref()
  }

  pub fn reset_app_type_name(&mut self) {
    self.app_type_name = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> ServiceIntegration {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> ServiceIntegration {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_external_product_id(&mut self, external_product_id: i64) {
    self.external_product_id = Some(external_product_id);
  }

  pub fn with_external_product_id(mut self, external_product_id: i64) -> ServiceIntegration {
    self.external_product_id = Some(external_product_id);
    self
  }

  pub fn external_product_id(&self) -> Option<&i64> {
    self.external_product_id.as_ref()
  }

  pub fn reset_external_product_id(&mut self) {
    self.external_product_id = None;
  }

  pub fn set_external_product_name(&mut self, external_product_name: String) {
    self.external_product_name = Some(external_product_name);
  }

  pub fn with_external_product_name(mut self, external_product_name: String) -> ServiceIntegration {
    self.external_product_name = Some(external_product_name);
    self
  }

  pub fn external_product_name(&self) -> Option<&String> {
    self.external_product_name.as_ref()
  }

  pub fn reset_external_product_name(&mut self) {
    self.external_product_name = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> ServiceIntegration {
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

  pub fn with_integration_type(mut self, integration_type: String) -> ServiceIntegration {
    self.integration_type = Some(integration_type);
    self
  }

  pub fn integration_type(&self) -> Option<&String> {
    self.integration_type.as_ref()
  }

  pub fn reset_integration_type(&mut self) {
    self.integration_type = None;
  }

  pub fn set_ordinal(&mut self, ordinal: i32) {
    self.ordinal = Some(ordinal);
  }

  pub fn with_ordinal(mut self, ordinal: i32) -> ServiceIntegration {
    self.ordinal = Some(ordinal);
    self
  }

  pub fn ordinal(&self) -> Option<&i32> {
    self.ordinal.as_ref()
  }

  pub fn reset_ordinal(&mut self) {
    self.ordinal = None;
  }

  pub fn set_parent_integration_id(&mut self, parent_integration_id: i64) {
    self.parent_integration_id = Some(parent_integration_id);
  }

  pub fn with_parent_integration_id(mut self, parent_integration_id: i64) -> ServiceIntegration {
    self.parent_integration_id = Some(parent_integration_id);
    self
  }

  pub fn parent_integration_id(&self) -> Option<&i64> {
    self.parent_integration_id.as_ref()
  }

  pub fn reset_parent_integration_id(&mut self) {
    self.parent_integration_id = None;
  }

  pub fn set_sematext_service(&mut self, sematext_service: String) {
    self.sematext_service = Some(sematext_service);
  }

  pub fn with_sematext_service(mut self, sematext_service: String) -> ServiceIntegration {
    self.sematext_service = Some(sematext_service);
    self
  }

  pub fn sematext_service(&self) -> Option<&String> {
    self.sematext_service.as_ref()
  }

  pub fn reset_sematext_service(&mut self) {
    self.sematext_service = None;
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.visible = Some(visible);
  }

  pub fn with_visible(mut self, visible: bool) -> ServiceIntegration {
    self.visible = Some(visible);
    self
  }

  pub fn visible(&self) -> Option<&bool> {
    self.visible.as_ref()
  }

  pub fn reset_visible(&mut self) {
    self.visible = None;
  }


  pub fn validate(&self) {
  }

}


