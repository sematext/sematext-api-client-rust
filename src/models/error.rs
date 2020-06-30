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
pub struct Error {
  #[serde(rename = "code")]
  code: Option<String>,
  #[serde(rename = "message")]
  message: Option<String>
}

impl Error {
  pub fn new() -> Error {
    Error {
      code: None,
      message: None
    }
  }

  pub fn set_code(&mut self, code: String) {
    self.code = Some(code);
  }

  pub fn with_code(mut self, code: String) -> Error {
    self.code = Some(code);
    self
  }

  pub fn code(&self) -> Option<&String> {
    self.code.as_ref()
  }

  pub fn reset_code(&mut self) {
    self.code = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> Error {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

}


