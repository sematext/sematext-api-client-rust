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
pub struct TokenDto {
  #[serde(rename = "createdAt")]
  created_at: Option<String>,
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  #[serde(rename = "id")]
  id: Option<i64>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "readable")]
  readable: Option<bool>,
  #[serde(rename = "token")]
  token: Option<String>,
  #[serde(rename = "writeable")]
  writeable: Option<bool>
}

impl TokenDto {
  pub fn new() -> TokenDto {
    TokenDto {
      created_at: None,
      enabled: None,
      id: None,
      name: None,
      readable: None,
      token: None,
      writeable: None
    }
  }

  pub fn set_created_at(&mut self, created_at: String) {
    self.created_at = Some(created_at);
  }

  pub fn with_created_at(mut self, created_at: String) -> TokenDto {
    self.created_at = Some(created_at);
    self
  }

  pub fn created_at(&self) -> Option<&String> {
    self.created_at.as_ref()
  }

  pub fn reset_created_at(&mut self) {
    self.created_at = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> TokenDto {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> TokenDto {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> TokenDto {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_readable(&mut self, readable: bool) {
    self.readable = Some(readable);
  }

  pub fn with_readable(mut self, readable: bool) -> TokenDto {
    self.readable = Some(readable);
    self
  }

  pub fn readable(&self) -> Option<&bool> {
    self.readable.as_ref()
  }

  pub fn reset_readable(&mut self) {
    self.readable = None;
  }

  pub fn set_token(&mut self, token: String) {
    self.token = Some(token);
  }

  pub fn with_token(mut self, token: String) -> TokenDto {
    self.token = Some(token);
    self
  }

  pub fn token(&self) -> Option<&String> {
    self.token.as_ref()
  }

  pub fn reset_token(&mut self) {
    self.token = None;
  }

  pub fn set_writeable(&mut self, writeable: bool) {
    self.writeable = Some(writeable);
  }

  pub fn with_writeable(mut self, writeable: bool) -> TokenDto {
    self.writeable = Some(writeable);
    self
  }

  pub fn writeable(&self) -> Option<&bool> {
    self.writeable.as_ref()
  }

  pub fn reset_writeable(&mut self) {
    self.writeable = None;
  }

}



