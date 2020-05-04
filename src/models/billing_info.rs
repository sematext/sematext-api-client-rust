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
pub struct BillingInfo {
  #[serde(rename = "creditCardId")]
  credit_card_id: Option<i64>,
  #[serde(rename = "paymentMethod")]
  payment_method: Option<String>,
  #[serde(rename = "planId")]
  plan_id: Option<i64>
}

impl BillingInfo {
  pub fn new() -> BillingInfo {
    BillingInfo {
      credit_card_id: None,
      payment_method: None,
      plan_id: None
    }
  }

  pub fn set_credit_card_id(&mut self, credit_card_id: i64) {
    self.credit_card_id = Some(credit_card_id);
  }

  pub fn with_credit_card_id(mut self, credit_card_id: i64) -> BillingInfo {
    self.credit_card_id = Some(credit_card_id);
    self
  }

  pub fn credit_card_id(&self) -> Option<&i64> {
    self.credit_card_id.as_ref()
  }

  pub fn reset_credit_card_id(&mut self) {
    self.credit_card_id = None;
  }

  pub fn set_payment_method(&mut self, payment_method: String) {
    self.payment_method = Some(payment_method);
  }

  pub fn with_payment_method(mut self, payment_method: String) -> BillingInfo {
    self.payment_method = Some(payment_method);
    self
  }

  pub fn payment_method(&self) -> Option<&String> {
    self.payment_method.as_ref()
  }

  pub fn reset_payment_method(&mut self) {
    self.payment_method = None;
  }

  pub fn set_plan_id(&mut self, plan_id: i64) {
    self.plan_id = Some(plan_id);
  }

  pub fn with_plan_id(mut self, plan_id: i64) -> BillingInfo {
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



