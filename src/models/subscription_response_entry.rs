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
pub struct SubscriptionResponseEntry {
  #[serde(rename = "subscription")]
  subscription: Option<::models::Subscription>
}

impl SubscriptionResponseEntry {
  pub fn new() -> SubscriptionResponseEntry {
    SubscriptionResponseEntry {
      subscription: None
    }
  }

  pub fn set_subscription(&mut self, subscription: ::models::Subscription) {
    self.subscription = Some(subscription);
  }

  pub fn with_subscription(mut self, subscription: ::models::Subscription) -> SubscriptionResponseEntry {
    self.subscription = Some(subscription);
    self
  }

  pub fn subscription(&self) -> Option<&::models::Subscription> {
    self.subscription.as_ref()
  }

  pub fn reset_subscription(&mut self) {
    self.subscription = None;
  }

}


