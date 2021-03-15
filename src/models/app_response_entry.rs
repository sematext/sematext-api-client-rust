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
pub struct AppResponseEntry {
  #[serde(rename = "app")]
  app: Option<::models::App>
}

impl AppResponseEntry {
  pub fn new() -> AppResponseEntry {
    AppResponseEntry {
      app: None
    }
  }

  pub fn set_app(&mut self, app: ::models::App) {
    self.app = Some(app);
  }

  pub fn with_app(mut self, app: ::models::App) -> AppResponseEntry {
    self.app = Some(app);
    self
  }

  pub fn app(&self) -> Option<&::models::App> {
    self.app.as_ref()
  }

  pub fn reset_app(&mut self) {
    self.app = None;
  }

}



