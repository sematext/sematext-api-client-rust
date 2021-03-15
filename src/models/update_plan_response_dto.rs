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
pub struct UpdatePlanResponseDto {
  #[serde(rename = "events")]
  events: Option<Vec<::models::EventDto>>,
  #[serde(rename = "planId")]
  plan_id: Option<i64>
}

impl UpdatePlanResponseDto {
  pub fn new() -> UpdatePlanResponseDto {
    UpdatePlanResponseDto {
      events: None,
      plan_id: None
    }
  }

  pub fn set_events(&mut self, events: Vec<::models::EventDto>) {
    self.events = Some(events);
  }

  pub fn with_events(mut self, events: Vec<::models::EventDto>) -> UpdatePlanResponseDto {
    self.events = Some(events);
    self
  }

  pub fn events(&self) -> Option<&Vec<::models::EventDto>> {
    self.events.as_ref()
  }

  pub fn reset_events(&mut self) {
    self.events = None;
  }

  pub fn set_plan_id(&mut self, plan_id: i64) {
    self.plan_id = Some(plan_id);
  }

  pub fn with_plan_id(mut self, plan_id: i64) -> UpdatePlanResponseDto {
    self.plan_id = Some(plan_id);
    self
  }

  pub fn plan_id(&self) -> Option<&i64> {
    self.plan_id.as_ref()
  }

  pub fn reset_plan_id(&mut self) {
    self.plan_id = None;
  }

}



