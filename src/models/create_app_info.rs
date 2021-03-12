/* 
 * Sematext Cloud API
 *
 * API Explorer provides access and documentation for Sematext REST API. The REST API requires the API Key to be sent as part of `Authorization` header. E.g.: `Authorization : apiKey e5f18450-205a-48eb-8589-7d49edaea813`.
 *
 * OpenAPI spec version: v3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppInfo {
  #[serde(rename = "appType")]
  app_type: Option<String>,
  #[serde(rename = "discountCode")]
  discount_code: Option<String>,
  #[serde(rename = "initialPlanId")]
  initial_plan_id: Option<i64>,
  /// AWS app meta data. Applicable only for `aws` appType
  #[serde(rename = "metaData")]
  meta_data: Option<::models::AppMetadata>,
  #[serde(rename = "name")]
  name: Option<String>
}

impl CreateAppInfo {
  pub fn new() -> CreateAppInfo {
    CreateAppInfo {
      app_type: None,
      discount_code: None,
      initial_plan_id: None,
      meta_data: None,
      name: None
    }
  }

  pub fn set_app_type(&mut self, app_type: String) {
    self.app_type = Some(app_type);
  }

  pub fn with_app_type(mut self, app_type: String) -> CreateAppInfo {
    self.app_type = Some(app_type);
    self
  }

  pub fn app_type(&self) -> Option<&String> {
    self.app_type.as_ref()
  }

  pub fn reset_app_type(&mut self) {
    self.app_type = None;
  }

  pub fn set_discount_code(&mut self, discount_code: String) {
    self.discount_code = Some(discount_code);
  }

  pub fn with_discount_code(mut self, discount_code: String) -> CreateAppInfo {
    self.discount_code = Some(discount_code);
    self
  }

  pub fn discount_code(&self) -> Option<&String> {
    self.discount_code.as_ref()
  }

  pub fn reset_discount_code(&mut self) {
    self.discount_code = None;
  }

  pub fn set_initial_plan_id(&mut self, initial_plan_id: i64) {
    self.initial_plan_id = Some(initial_plan_id);
  }

  pub fn with_initial_plan_id(mut self, initial_plan_id: i64) -> CreateAppInfo {
    self.initial_plan_id = Some(initial_plan_id);
    self
  }

  pub fn initial_plan_id(&self) -> Option<&i64> {
    self.initial_plan_id.as_ref()
  }

  pub fn reset_initial_plan_id(&mut self) {
    self.initial_plan_id = None;
  }

  pub fn set_meta_data(&mut self, meta_data: ::models::AppMetadata) {
    self.meta_data = Some(meta_data);
  }

  pub fn with_meta_data(mut self, meta_data: ::models::AppMetadata) -> CreateAppInfo {
    self.meta_data = Some(meta_data);
    self
  }

  pub fn meta_data(&self) -> Option<&::models::AppMetadata> {
    self.meta_data.as_ref()
  }

  pub fn reset_meta_data(&mut self) {
    self.meta_data = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> CreateAppInfo {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}



