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
pub struct AlertRuleResponse {
  #[serde(rename = "data")]
  data: Option<::models::AlertRuleResponseEntry>,
  #[serde(rename = "errors")]
  errors: Option<Vec<::models::Error>>,
  #[serde(rename = "message")]
  message: Option<String>,
  #[serde(rename = "success")]
  success: Option<bool>
}

impl AlertRuleResponse {
  pub fn new() -> AlertRuleResponse {
    AlertRuleResponse {
      data: None,
      errors: None,
      message: None,
      success: None
    }
  }

  pub fn set_data(&mut self, data: ::models::AlertRuleResponseEntry) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::models::AlertRuleResponseEntry) -> AlertRuleResponse {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::models::AlertRuleResponseEntry> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_errors(&mut self, errors: Vec<::models::Error>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<::models::Error>) -> AlertRuleResponse {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> Option<&Vec<::models::Error>> {
    self.errors.as_ref()
  }

  pub fn reset_errors(&mut self) {
    self.errors = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> AlertRuleResponse {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = Some(success);
  }

  pub fn with_success(mut self, success: bool) -> AlertRuleResponse {
    self.success = Some(success);
    self
  }

  pub fn success(&self) -> Option<&bool> {
    self.success.as_ref()
  }

  pub fn reset_success(&mut self) {
    self.success = None;
  }

}



