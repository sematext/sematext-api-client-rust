/* 
 * Sematext Cloud API
 *
 * API Explorer provides access and documentation for Sematext REST API. The REST API requires the API Key to be sent as part of `Authorization` header. E.g.: `Authorization : apiKey e5f18450-205a-48eb-8589-7d49edaea813`.
 *
 * OpenAPI spec version: v3
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */
#![allow(unused_imports)]

use serde_json::Value;
use bigdecimal::BigDecimal;
use chrono::{Date, NaiveDateTime, NaiveDate, DateTime, FixedOffset, Utc};

use crate::models::*;
use crate::date_serializer;
use crate::date_serializer_opt;
use crate::serialize_quoted_numbers;
use crate::serialize_quoted_numbers_opt;
//Uncomment this to deal with limited rfc support on server side
//use crate::datetime_serializer::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct App {
  #[serde(rename = "ajaxThreshold")]
  #[serde(default)]
  ajax_threshold: Option<i64>, 
  #[serde(rename = "appType")]
  #[serde(default)]
  app_type: Option<String>, 
  #[serde(rename = "appTypeId")]
  #[serde(default)]
  app_type_id: Option<i64>, 
  #[serde(rename = "creatorEmail")]
  #[serde(default)]
  creator_email: Option<String>, 
  #[serde(rename = "creditCardExpiry")]
  #[serde(default)]
  credit_card_expiry: Option<String>, 
  #[serde(rename = "creditCardNumber")]
  #[serde(default)]
  credit_card_number: Option<String>, 
  #[serde(rename = "description")]
  #[serde(default)]
  description: Option<String>, 
  #[serde(rename = "displayStatus")]
  #[serde(default)]
  display_status: Option<String>, 
  #[serde(rename = "firstDataSavedDate")]
  #[serde(default)]
  first_data_saved_date: Option<i64>, 
  #[serde(rename = "id")]
  #[serde(default)]
  id: Option<i64>, 
  #[serde(rename = "integration")]
  #[serde(default)]
  integration: Option<ServiceIntegration>, 
  #[serde(rename = "lastDataReceivedDate")]
  #[serde(default)]
  last_data_received_date: Option<i64>, 
  #[serde(rename = "lastDataSavedDate")]
  #[serde(default)]
  last_data_saved_date: Option<i64>, 
  #[serde(rename = "loggedInUserAppRole")]
  #[serde(default)]
  logged_in_user_app_role: Option<String>, 
  #[serde(rename = "monthlyInvoiceAccount")]
  #[serde(default)]
  monthly_invoice_account: Option<bool>, 
  #[serde(rename = "name")]
  #[serde(default)]
  name: Option<String>, 
  #[serde(rename = "ownerEmail")]
  #[serde(default)]
  owner_email: Option<String>, 
  #[serde(rename = "owningOrganization")]
  #[serde(default)]
  owning_organization: Option<BasicOrganizationDto>, 
  #[serde(rename = "pageLoadThreshold")]
  #[serde(default)]
  page_load_threshold: Option<i64>, 
  #[serde(rename = "paymentMethodId")]
  #[serde(default)]
  payment_method_id: Option<i64>, 
  #[serde(rename = "plan")]
  #[serde(default)]
  plan: Option<Plan>, 
  #[serde(rename = "prepaidAccount")]
  #[serde(default)]
  prepaid_account: Option<bool>, 
  #[serde(rename = "readToken")]
  #[serde(default)]
  read_token: Option<String>, 
  #[serde(rename = "readWriteToken")]
  #[serde(default)]
  read_write_token: Option<String>, 
  #[serde(rename = "registrationDate")]
  #[serde(default)]
  registration_date: Option<i64>, 
  #[serde(rename = "status")]
  #[serde(default)]
  status: Option<String>, 
  #[serde(rename = "token")]
  #[serde(default)]
  token: Option<String>, 
  #[serde(rename = "tokens")]
  #[serde(default)]
  tokens: Option<Vec<String>>, 
  #[serde(rename = "trialEndDate")]
  #[serde(default)]
  trial_end_date: Option<i64>, 
  #[serde(rename = "urlGroupLimit")]
  #[serde(default)]
  url_group_limit: Option<i32>, 
  #[serde(rename = "userRoles")]
  #[serde(default)]
  user_roles: Option<Vec<UserRole>>, 
  #[serde(rename = "writeToken")]
  #[serde(default)]
  write_token: Option<String> 
}

impl App {
  pub fn new() -> App {
    App {
      ajax_threshold: None,
      app_type: None,
      app_type_id: None,
      creator_email: None,
      credit_card_expiry: None,
      credit_card_number: None,
      description: None,
      display_status: None,
      first_data_saved_date: None,
      id: None,
      integration: None,
      last_data_received_date: None,
      last_data_saved_date: None,
      logged_in_user_app_role: None,
      monthly_invoice_account: None,
      name: None,
      owner_email: None,
      owning_organization: None,
      page_load_threshold: None,
      payment_method_id: None,
      plan: None,
      prepaid_account: None,
      read_token: None,
      read_write_token: None,
      registration_date: None,
      status: None,
      token: None,
      tokens: None,
      trial_end_date: None,
      url_group_limit: None,
      user_roles: None,
      write_token: None
    }
  }

  pub fn set_ajax_threshold(&mut self, ajax_threshold: i64) {
    self.ajax_threshold = Some(ajax_threshold);
  }

  pub fn with_ajax_threshold(mut self, ajax_threshold: i64) -> App {
    self.ajax_threshold = Some(ajax_threshold);
    self
  }

  pub fn ajax_threshold(&self) -> Option<&i64> {
    self.ajax_threshold.as_ref()
  }

  pub fn reset_ajax_threshold(&mut self) {
    self.ajax_threshold = None;
  }

  pub fn set_app_type(&mut self, app_type: String) {
    self.app_type = Some(app_type);
  }

  pub fn with_app_type(mut self, app_type: String) -> App {
    self.app_type = Some(app_type);
    self
  }

  pub fn app_type(&self) -> Option<&String> {
    self.app_type.as_ref()
  }

  pub fn reset_app_type(&mut self) {
    self.app_type = None;
  }

  pub fn set_app_type_id(&mut self, app_type_id: i64) {
    self.app_type_id = Some(app_type_id);
  }

  pub fn with_app_type_id(mut self, app_type_id: i64) -> App {
    self.app_type_id = Some(app_type_id);
    self
  }

  pub fn app_type_id(&self) -> Option<&i64> {
    self.app_type_id.as_ref()
  }

  pub fn reset_app_type_id(&mut self) {
    self.app_type_id = None;
  }

  pub fn set_creator_email(&mut self, creator_email: String) {
    self.creator_email = Some(creator_email);
  }

  pub fn with_creator_email(mut self, creator_email: String) -> App {
    self.creator_email = Some(creator_email);
    self
  }

  pub fn creator_email(&self) -> Option<&String> {
    self.creator_email.as_ref()
  }

  pub fn reset_creator_email(&mut self) {
    self.creator_email = None;
  }

  pub fn set_credit_card_expiry(&mut self, credit_card_expiry: String) {
    self.credit_card_expiry = Some(credit_card_expiry);
  }

  pub fn with_credit_card_expiry(mut self, credit_card_expiry: String) -> App {
    self.credit_card_expiry = Some(credit_card_expiry);
    self
  }

  pub fn credit_card_expiry(&self) -> Option<&String> {
    self.credit_card_expiry.as_ref()
  }

  pub fn reset_credit_card_expiry(&mut self) {
    self.credit_card_expiry = None;
  }

  pub fn set_credit_card_number(&mut self, credit_card_number: String) {
    self.credit_card_number = Some(credit_card_number);
  }

  pub fn with_credit_card_number(mut self, credit_card_number: String) -> App {
    self.credit_card_number = Some(credit_card_number);
    self
  }

  pub fn credit_card_number(&self) -> Option<&String> {
    self.credit_card_number.as_ref()
  }

  pub fn reset_credit_card_number(&mut self) {
    self.credit_card_number = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> App {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_display_status(&mut self, display_status: String) {
    self.display_status = Some(display_status);
  }

  pub fn with_display_status(mut self, display_status: String) -> App {
    self.display_status = Some(display_status);
    self
  }

  pub fn display_status(&self) -> Option<&String> {
    self.display_status.as_ref()
  }

  pub fn reset_display_status(&mut self) {
    self.display_status = None;
  }

  pub fn set_first_data_saved_date(&mut self, first_data_saved_date: i64) {
    self.first_data_saved_date = Some(first_data_saved_date);
  }

  pub fn with_first_data_saved_date(mut self, first_data_saved_date: i64) -> App {
    self.first_data_saved_date = Some(first_data_saved_date);
    self
  }

  pub fn first_data_saved_date(&self) -> Option<&i64> {
    self.first_data_saved_date.as_ref()
  }

  pub fn reset_first_data_saved_date(&mut self) {
    self.first_data_saved_date = None;
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i64) -> App {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i64> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_integration(&mut self, integration: ServiceIntegration) {
    self.integration = Some(integration);
  }

  pub fn with_integration(mut self, integration: ServiceIntegration) -> App {
    self.integration = Some(integration);
    self
  }

  pub fn integration(&self) -> Option<&ServiceIntegration> {
    self.integration.as_ref()
  }

  pub fn reset_integration(&mut self) {
    self.integration = None;
  }

  pub fn set_last_data_received_date(&mut self, last_data_received_date: i64) {
    self.last_data_received_date = Some(last_data_received_date);
  }

  pub fn with_last_data_received_date(mut self, last_data_received_date: i64) -> App {
    self.last_data_received_date = Some(last_data_received_date);
    self
  }

  pub fn last_data_received_date(&self) -> Option<&i64> {
    self.last_data_received_date.as_ref()
  }

  pub fn reset_last_data_received_date(&mut self) {
    self.last_data_received_date = None;
  }

  pub fn set_last_data_saved_date(&mut self, last_data_saved_date: i64) {
    self.last_data_saved_date = Some(last_data_saved_date);
  }

  pub fn with_last_data_saved_date(mut self, last_data_saved_date: i64) -> App {
    self.last_data_saved_date = Some(last_data_saved_date);
    self
  }

  pub fn last_data_saved_date(&self) -> Option<&i64> {
    self.last_data_saved_date.as_ref()
  }

  pub fn reset_last_data_saved_date(&mut self) {
    self.last_data_saved_date = None;
  }

  pub fn set_logged_in_user_app_role(&mut self, logged_in_user_app_role: String) {
    self.logged_in_user_app_role = Some(logged_in_user_app_role);
  }

  pub fn with_logged_in_user_app_role(mut self, logged_in_user_app_role: String) -> App {
    self.logged_in_user_app_role = Some(logged_in_user_app_role);
    self
  }

  pub fn logged_in_user_app_role(&self) -> Option<&String> {
    self.logged_in_user_app_role.as_ref()
  }

  pub fn reset_logged_in_user_app_role(&mut self) {
    self.logged_in_user_app_role = None;
  }

  pub fn set_monthly_invoice_account(&mut self, monthly_invoice_account: bool) {
    self.monthly_invoice_account = Some(monthly_invoice_account);
  }

  pub fn with_monthly_invoice_account(mut self, monthly_invoice_account: bool) -> App {
    self.monthly_invoice_account = Some(monthly_invoice_account);
    self
  }

  pub fn monthly_invoice_account(&self) -> Option<&bool> {
    self.monthly_invoice_account.as_ref()
  }

  pub fn reset_monthly_invoice_account(&mut self) {
    self.monthly_invoice_account = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> App {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_owner_email(&mut self, owner_email: String) {
    self.owner_email = Some(owner_email);
  }

  pub fn with_owner_email(mut self, owner_email: String) -> App {
    self.owner_email = Some(owner_email);
    self
  }

  pub fn owner_email(&self) -> Option<&String> {
    self.owner_email.as_ref()
  }

  pub fn reset_owner_email(&mut self) {
    self.owner_email = None;
  }

  pub fn set_owning_organization(&mut self, owning_organization: BasicOrganizationDto) {
    self.owning_organization = Some(owning_organization);
  }

  pub fn with_owning_organization(mut self, owning_organization: BasicOrganizationDto) -> App {
    self.owning_organization = Some(owning_organization);
    self
  }

  pub fn owning_organization(&self) -> Option<&BasicOrganizationDto> {
    self.owning_organization.as_ref()
  }

  pub fn reset_owning_organization(&mut self) {
    self.owning_organization = None;
  }

  pub fn set_page_load_threshold(&mut self, page_load_threshold: i64) {
    self.page_load_threshold = Some(page_load_threshold);
  }

  pub fn with_page_load_threshold(mut self, page_load_threshold: i64) -> App {
    self.page_load_threshold = Some(page_load_threshold);
    self
  }

  pub fn page_load_threshold(&self) -> Option<&i64> {
    self.page_load_threshold.as_ref()
  }

  pub fn reset_page_load_threshold(&mut self) {
    self.page_load_threshold = None;
  }

  pub fn set_payment_method_id(&mut self, payment_method_id: i64) {
    self.payment_method_id = Some(payment_method_id);
  }

  pub fn with_payment_method_id(mut self, payment_method_id: i64) -> App {
    self.payment_method_id = Some(payment_method_id);
    self
  }

  pub fn payment_method_id(&self) -> Option<&i64> {
    self.payment_method_id.as_ref()
  }

  pub fn reset_payment_method_id(&mut self) {
    self.payment_method_id = None;
  }

  pub fn set_plan(&mut self, plan: Plan) {
    self.plan = Some(plan);
  }

  pub fn with_plan(mut self, plan: Plan) -> App {
    self.plan = Some(plan);
    self
  }

  pub fn plan(&self) -> Option<&Plan> {
    self.plan.as_ref()
  }

  pub fn reset_plan(&mut self) {
    self.plan = None;
  }

  pub fn set_prepaid_account(&mut self, prepaid_account: bool) {
    self.prepaid_account = Some(prepaid_account);
  }

  pub fn with_prepaid_account(mut self, prepaid_account: bool) -> App {
    self.prepaid_account = Some(prepaid_account);
    self
  }

  pub fn prepaid_account(&self) -> Option<&bool> {
    self.prepaid_account.as_ref()
  }

  pub fn reset_prepaid_account(&mut self) {
    self.prepaid_account = None;
  }

  pub fn set_read_token(&mut self, read_token: String) {
    self.read_token = Some(read_token);
  }

  pub fn with_read_token(mut self, read_token: String) -> App {
    self.read_token = Some(read_token);
    self
  }

  pub fn read_token(&self) -> Option<&String> {
    self.read_token.as_ref()
  }

  pub fn reset_read_token(&mut self) {
    self.read_token = None;
  }

  pub fn set_read_write_token(&mut self, read_write_token: String) {
    self.read_write_token = Some(read_write_token);
  }

  pub fn with_read_write_token(mut self, read_write_token: String) -> App {
    self.read_write_token = Some(read_write_token);
    self
  }

  pub fn read_write_token(&self) -> Option<&String> {
    self.read_write_token.as_ref()
  }

  pub fn reset_read_write_token(&mut self) {
    self.read_write_token = None;
  }

  pub fn set_registration_date(&mut self, registration_date: i64) {
    self.registration_date = Some(registration_date);
  }

  pub fn with_registration_date(mut self, registration_date: i64) -> App {
    self.registration_date = Some(registration_date);
    self
  }

  pub fn registration_date(&self) -> Option<&i64> {
    self.registration_date.as_ref()
  }

  pub fn reset_registration_date(&mut self) {
    self.registration_date = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> App {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_token(&mut self, token: String) {
    self.token = Some(token);
  }

  pub fn with_token(mut self, token: String) -> App {
    self.token = Some(token);
    self
  }

  pub fn token(&self) -> Option<&String> {
    self.token.as_ref()
  }

  pub fn reset_token(&mut self) {
    self.token = None;
  }

  pub fn set_tokens(&mut self, tokens: Vec<String>) {
    self.tokens = Some(tokens);
  }

  pub fn with_tokens(mut self, tokens: Vec<String>) -> App {
    self.tokens = Some(tokens);
    self
  }

  pub fn tokens(&self) -> Option<&Vec<String>> {
    self.tokens.as_ref()
  }

  pub fn reset_tokens(&mut self) {
    self.tokens = None;
  }

  pub fn set_trial_end_date(&mut self, trial_end_date: i64) {
    self.trial_end_date = Some(trial_end_date);
  }

  pub fn with_trial_end_date(mut self, trial_end_date: i64) -> App {
    self.trial_end_date = Some(trial_end_date);
    self
  }

  pub fn trial_end_date(&self) -> Option<&i64> {
    self.trial_end_date.as_ref()
  }

  pub fn reset_trial_end_date(&mut self) {
    self.trial_end_date = None;
  }

  pub fn set_url_group_limit(&mut self, url_group_limit: i32) {
    self.url_group_limit = Some(url_group_limit);
  }

  pub fn with_url_group_limit(mut self, url_group_limit: i32) -> App {
    self.url_group_limit = Some(url_group_limit);
    self
  }

  pub fn url_group_limit(&self) -> Option<&i32> {
    self.url_group_limit.as_ref()
  }

  pub fn reset_url_group_limit(&mut self) {
    self.url_group_limit = None;
  }

  pub fn set_user_roles(&mut self, user_roles: Vec<UserRole>) {
    self.user_roles = Some(user_roles);
  }

  pub fn with_user_roles(mut self, user_roles: Vec<UserRole>) -> App {
    self.user_roles = Some(user_roles);
    self
  }

  pub fn user_roles(&self) -> Option<&Vec<UserRole>> {
    self.user_roles.as_ref()
  }

  pub fn reset_user_roles(&mut self) {
    self.user_roles = None;
  }

  pub fn set_write_token(&mut self, write_token: String) {
    self.write_token = Some(write_token);
  }

  pub fn with_write_token(mut self, write_token: String) -> App {
    self.write_token = Some(write_token);
    self
  }

  pub fn write_token(&self) -> Option<&String> {
    self.write_token.as_ref()
  }

  pub fn reset_write_token(&mut self) {
    self.write_token = None;
  }


  pub fn validate(&self) {
  }

}


