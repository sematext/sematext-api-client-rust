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
pub struct AlertRuleResponseEntry {
  #[serde(rename = "alertRule")]
  alert_rule: Option<::models::AlertRule>
}

impl AlertRuleResponseEntry {
  pub fn new() -> AlertRuleResponseEntry {
    AlertRuleResponseEntry {
      alert_rule: None
    }
  }

  pub fn set_alert_rule(&mut self, alert_rule: ::models::AlertRule) {
    self.alert_rule = Some(alert_rule);
  }

  pub fn with_alert_rule(mut self, alert_rule: ::models::AlertRule) -> AlertRuleResponseEntry {
    self.alert_rule = Some(alert_rule);
    self
  }

  pub fn alert_rule(&self) -> Option<&::models::AlertRule> {
    self.alert_rule.as_ref()
  }

  pub fn reset_alert_rule(&mut self) {
    self.alert_rule = None;
  }

}



