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
pub struct LimitChangeEventDto {
  #[serde(rename = "eventTime")]
  event_time: Option<String>,
  #[serde(rename = "newLimit")]
  new_limit: Option<i64>,
  #[serde(rename = "oldLimit")]
  old_limit: Option<i64>
}

impl LimitChangeEventDto {
  pub fn new() -> LimitChangeEventDto {
    LimitChangeEventDto {
      event_time: None,
      new_limit: None,
      old_limit: None
    }
  }

  pub fn set_event_time(&mut self, event_time: String) {
    self.event_time = Some(event_time);
  }

  pub fn with_event_time(mut self, event_time: String) -> LimitChangeEventDto {
    self.event_time = Some(event_time);
    self
  }

  pub fn event_time(&self) -> Option<&String> {
    self.event_time.as_ref()
  }

  pub fn reset_event_time(&mut self) {
    self.event_time = None;
  }

  pub fn set_new_limit(&mut self, new_limit: i64) {
    self.new_limit = Some(new_limit);
  }

  pub fn with_new_limit(mut self, new_limit: i64) -> LimitChangeEventDto {
    self.new_limit = Some(new_limit);
    self
  }

  pub fn new_limit(&self) -> Option<&i64> {
    self.new_limit.as_ref()
  }

  pub fn reset_new_limit(&mut self) {
    self.new_limit = None;
  }

  pub fn set_old_limit(&mut self, old_limit: i64) {
    self.old_limit = Some(old_limit);
  }

  pub fn with_old_limit(mut self, old_limit: i64) -> LimitChangeEventDto {
    self.old_limit = Some(old_limit);
    self
  }

  pub fn old_limit(&self) -> Option<&i64> {
    self.old_limit.as_ref()
  }

  pub fn reset_old_limit(&mut self) {
    self.old_limit = None;
  }

}



