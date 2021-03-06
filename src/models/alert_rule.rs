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
pub struct AlertRule {
  #[serde(rename = "accountEmail")]
  account_email: Option<String>,
  #[serde(rename = "allowedAppTypes")]
  allowed_app_types: Option<Vec<i64>>,
  #[serde(rename = "analyzingTime")]
  analyzing_time: Option<String>,
  #[serde(rename = "appDisplayState")]
  app_display_state: Option<String>,
  #[serde(rename = "appId")]
  app_id: Option<i64>,
  #[serde(rename = "appName")]
  app_name: Option<String>,
  #[serde(rename = "appState")]
  app_state: Option<String>,
  #[serde(rename = "appToken")]
  app_token: Option<String>,
  #[serde(rename = "appType")]
  app_type: Option<String>,
  #[serde(rename = "backToNormalNeeded")]
  back_to_normal_needed: Option<bool>,
  #[serde(rename = "chartKey")]
  chart_key: Option<String>,
  #[serde(rename = "color")]
  color: Option<String>,
  #[serde(rename = "creatorEmail")]
  creator_email: Option<String>,
  #[serde(rename = "defaultAggType")]
  default_agg_type: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "disallowedAppTypes")]
  disallowed_app_types: Option<Vec<i64>>,
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  #[serde(rename = "estimateOperation")]
  estimate_operation: Option<String>,
  #[serde(rename = "estimateValue")]
  estimate_value: Option<f64>,
  #[serde(rename = "filterValues")]
  filter_values: Option<String>,
  #[serde(rename = "filterValuesObj")]
  filter_values_obj: Option<Vec<::models::FilterValue>>,
  #[serde(rename = "ignoreRegularEventsEnabled")]
  ignore_regular_events_enabled: Option<bool>,
  #[serde(rename = "integrations")]
  integrations: Option<String>,
  #[serde(rename = "lastDataReceivedDate")]
  last_data_received_date: Option<i64>,
  #[serde(rename = "lastSent")]
  last_sent: Option<i64>,
  #[serde(rename = "lastTriggered")]
  last_triggered: Option<i64>,
  #[serde(rename = "metadata")]
  metadata: Option<Value>,
  #[serde(rename = "metricKey")]
  metric_key: Option<String>,
  #[serde(rename = "metricLabel")]
  metric_label: Option<String>,
  #[serde(rename = "minDelayBetweenNotificationsInMinutes")]
  min_delay_between_notifications_in_minutes: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "notificationEmails")]
  notification_emails: Option<Vec<String>>,
  #[serde(rename = "notificationIntegrations")]
  notification_integrations: Option<Vec<::models::NotificationIntegration>>,
  #[serde(rename = "notificationsEnabled")]
  notifications_enabled: Option<bool>,
  #[serde(rename = "query")]
  query: Option<String>,
  #[serde(rename = "reportName")]
  report_name: Option<String>,
  #[serde(rename = "ruleKey")]
  rule_key: Option<i64>,
  #[serde(rename = "ruleType")]
  rule_type: Option<String>,
  #[serde(rename = "runbook")]
  runbook: Option<String>,
  #[serde(rename = "savedQueryId")]
  saved_query_id: Option<i64>,
  #[serde(rename = "schedule")]
  schedule: Option<Vec<::models::AlertRuleScheduleWeekdayDto>>,
  #[serde(rename = "sematextService")]
  sematext_service: Option<String>,
  #[serde(rename = "sendToEmail")]
  send_to_email: Option<String>,
  #[serde(rename = "timezone")]
  timezone: Option<String>,
  #[serde(rename = "useOnlyAlertRuleIntegrations")]
  use_only_alert_rule_integrations: Option<bool>,
  #[serde(rename = "userPermissions")]
  user_permissions: Option<::models::UserPermissions>,
  #[serde(rename = "valueColumnName")]
  value_column_name: Option<String>,
  #[serde(rename = "valueName")]
  value_name: Option<String>
}

impl AlertRule {
  pub fn new() -> AlertRule {
    AlertRule {
      account_email: None,
      allowed_app_types: None,
      analyzing_time: None,
      app_display_state: None,
      app_id: None,
      app_name: None,
      app_state: None,
      app_token: None,
      app_type: None,
      back_to_normal_needed: None,
      chart_key: None,
      color: None,
      creator_email: None,
      default_agg_type: None,
      description: None,
      disallowed_app_types: None,
      enabled: None,
      estimate_operation: None,
      estimate_value: None,
      filter_values: None,
      filter_values_obj: None,
      ignore_regular_events_enabled: None,
      integrations: None,
      last_data_received_date: None,
      last_sent: None,
      last_triggered: None,
      metadata: None,
      metric_key: None,
      metric_label: None,
      min_delay_between_notifications_in_minutes: None,
      name: None,
      notification_emails: None,
      notification_integrations: None,
      notifications_enabled: None,
      query: None,
      report_name: None,
      rule_key: None,
      rule_type: None,
      runbook: None,
      saved_query_id: None,
      schedule: None,
      sematext_service: None,
      send_to_email: None,
      timezone: None,
      use_only_alert_rule_integrations: None,
      user_permissions: None,
      value_column_name: None,
      value_name: None
    }
  }

  pub fn set_account_email(&mut self, account_email: String) {
    self.account_email = Some(account_email);
  }

  pub fn with_account_email(mut self, account_email: String) -> AlertRule {
    self.account_email = Some(account_email);
    self
  }

  pub fn account_email(&self) -> Option<&String> {
    self.account_email.as_ref()
  }

  pub fn reset_account_email(&mut self) {
    self.account_email = None;
  }

  pub fn set_allowed_app_types(&mut self, allowed_app_types: Vec<i64>) {
    self.allowed_app_types = Some(allowed_app_types);
  }

  pub fn with_allowed_app_types(mut self, allowed_app_types: Vec<i64>) -> AlertRule {
    self.allowed_app_types = Some(allowed_app_types);
    self
  }

  pub fn allowed_app_types(&self) -> Option<&Vec<i64>> {
    self.allowed_app_types.as_ref()
  }

  pub fn reset_allowed_app_types(&mut self) {
    self.allowed_app_types = None;
  }

  pub fn set_analyzing_time(&mut self, analyzing_time: String) {
    self.analyzing_time = Some(analyzing_time);
  }

  pub fn with_analyzing_time(mut self, analyzing_time: String) -> AlertRule {
    self.analyzing_time = Some(analyzing_time);
    self
  }

  pub fn analyzing_time(&self) -> Option<&String> {
    self.analyzing_time.as_ref()
  }

  pub fn reset_analyzing_time(&mut self) {
    self.analyzing_time = None;
  }

  pub fn set_app_display_state(&mut self, app_display_state: String) {
    self.app_display_state = Some(app_display_state);
  }

  pub fn with_app_display_state(mut self, app_display_state: String) -> AlertRule {
    self.app_display_state = Some(app_display_state);
    self
  }

  pub fn app_display_state(&self) -> Option<&String> {
    self.app_display_state.as_ref()
  }

  pub fn reset_app_display_state(&mut self) {
    self.app_display_state = None;
  }

  pub fn set_app_id(&mut self, app_id: i64) {
    self.app_id = Some(app_id);
  }

  pub fn with_app_id(mut self, app_id: i64) -> AlertRule {
    self.app_id = Some(app_id);
    self
  }

  pub fn app_id(&self) -> Option<&i64> {
    self.app_id.as_ref()
  }

  pub fn reset_app_id(&mut self) {
    self.app_id = None;
  }

  pub fn set_app_name(&mut self, app_name: String) {
    self.app_name = Some(app_name);
  }

  pub fn with_app_name(mut self, app_name: String) -> AlertRule {
    self.app_name = Some(app_name);
    self
  }

  pub fn app_name(&self) -> Option<&String> {
    self.app_name.as_ref()
  }

  pub fn reset_app_name(&mut self) {
    self.app_name = None;
  }

  pub fn set_app_state(&mut self, app_state: String) {
    self.app_state = Some(app_state);
  }

  pub fn with_app_state(mut self, app_state: String) -> AlertRule {
    self.app_state = Some(app_state);
    self
  }

  pub fn app_state(&self) -> Option<&String> {
    self.app_state.as_ref()
  }

  pub fn reset_app_state(&mut self) {
    self.app_state = None;
  }

  pub fn set_app_token(&mut self, app_token: String) {
    self.app_token = Some(app_token);
  }

  pub fn with_app_token(mut self, app_token: String) -> AlertRule {
    self.app_token = Some(app_token);
    self
  }

  pub fn app_token(&self) -> Option<&String> {
    self.app_token.as_ref()
  }

  pub fn reset_app_token(&mut self) {
    self.app_token = None;
  }

  pub fn set_app_type(&mut self, app_type: String) {
    self.app_type = Some(app_type);
  }

  pub fn with_app_type(mut self, app_type: String) -> AlertRule {
    self.app_type = Some(app_type);
    self
  }

  pub fn app_type(&self) -> Option<&String> {
    self.app_type.as_ref()
  }

  pub fn reset_app_type(&mut self) {
    self.app_type = None;
  }

  pub fn set_back_to_normal_needed(&mut self, back_to_normal_needed: bool) {
    self.back_to_normal_needed = Some(back_to_normal_needed);
  }

  pub fn with_back_to_normal_needed(mut self, back_to_normal_needed: bool) -> AlertRule {
    self.back_to_normal_needed = Some(back_to_normal_needed);
    self
  }

  pub fn back_to_normal_needed(&self) -> Option<&bool> {
    self.back_to_normal_needed.as_ref()
  }

  pub fn reset_back_to_normal_needed(&mut self) {
    self.back_to_normal_needed = None;
  }

  pub fn set_chart_key(&mut self, chart_key: String) {
    self.chart_key = Some(chart_key);
  }

  pub fn with_chart_key(mut self, chart_key: String) -> AlertRule {
    self.chart_key = Some(chart_key);
    self
  }

  pub fn chart_key(&self) -> Option<&String> {
    self.chart_key.as_ref()
  }

  pub fn reset_chart_key(&mut self) {
    self.chart_key = None;
  }

  pub fn set_color(&mut self, color: String) {
    self.color = Some(color);
  }

  pub fn with_color(mut self, color: String) -> AlertRule {
    self.color = Some(color);
    self
  }

  pub fn color(&self) -> Option<&String> {
    self.color.as_ref()
  }

  pub fn reset_color(&mut self) {
    self.color = None;
  }

  pub fn set_creator_email(&mut self, creator_email: String) {
    self.creator_email = Some(creator_email);
  }

  pub fn with_creator_email(mut self, creator_email: String) -> AlertRule {
    self.creator_email = Some(creator_email);
    self
  }

  pub fn creator_email(&self) -> Option<&String> {
    self.creator_email.as_ref()
  }

  pub fn reset_creator_email(&mut self) {
    self.creator_email = None;
  }

  pub fn set_default_agg_type(&mut self, default_agg_type: String) {
    self.default_agg_type = Some(default_agg_type);
  }

  pub fn with_default_agg_type(mut self, default_agg_type: String) -> AlertRule {
    self.default_agg_type = Some(default_agg_type);
    self
  }

  pub fn default_agg_type(&self) -> Option<&String> {
    self.default_agg_type.as_ref()
  }

  pub fn reset_default_agg_type(&mut self) {
    self.default_agg_type = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> AlertRule {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_disallowed_app_types(&mut self, disallowed_app_types: Vec<i64>) {
    self.disallowed_app_types = Some(disallowed_app_types);
  }

  pub fn with_disallowed_app_types(mut self, disallowed_app_types: Vec<i64>) -> AlertRule {
    self.disallowed_app_types = Some(disallowed_app_types);
    self
  }

  pub fn disallowed_app_types(&self) -> Option<&Vec<i64>> {
    self.disallowed_app_types.as_ref()
  }

  pub fn reset_disallowed_app_types(&mut self) {
    self.disallowed_app_types = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> AlertRule {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_estimate_operation(&mut self, estimate_operation: String) {
    self.estimate_operation = Some(estimate_operation);
  }

  pub fn with_estimate_operation(mut self, estimate_operation: String) -> AlertRule {
    self.estimate_operation = Some(estimate_operation);
    self
  }

  pub fn estimate_operation(&self) -> Option<&String> {
    self.estimate_operation.as_ref()
  }

  pub fn reset_estimate_operation(&mut self) {
    self.estimate_operation = None;
  }

  pub fn set_estimate_value(&mut self, estimate_value: f64) {
    self.estimate_value = Some(estimate_value);
  }

  pub fn with_estimate_value(mut self, estimate_value: f64) -> AlertRule {
    self.estimate_value = Some(estimate_value);
    self
  }

  pub fn estimate_value(&self) -> Option<&f64> {
    self.estimate_value.as_ref()
  }

  pub fn reset_estimate_value(&mut self) {
    self.estimate_value = None;
  }

  pub fn set_filter_values(&mut self, filter_values: String) {
    self.filter_values = Some(filter_values);
  }

  pub fn with_filter_values(mut self, filter_values: String) -> AlertRule {
    self.filter_values = Some(filter_values);
    self
  }

  pub fn filter_values(&self) -> Option<&String> {
    self.filter_values.as_ref()
  }

  pub fn reset_filter_values(&mut self) {
    self.filter_values = None;
  }

  pub fn set_filter_values_obj(&mut self, filter_values_obj: Vec<::models::FilterValue>) {
    self.filter_values_obj = Some(filter_values_obj);
  }

  pub fn with_filter_values_obj(mut self, filter_values_obj: Vec<::models::FilterValue>) -> AlertRule {
    self.filter_values_obj = Some(filter_values_obj);
    self
  }

  pub fn filter_values_obj(&self) -> Option<&Vec<::models::FilterValue>> {
    self.filter_values_obj.as_ref()
  }

  pub fn reset_filter_values_obj(&mut self) {
    self.filter_values_obj = None;
  }

  pub fn set_ignore_regular_events_enabled(&mut self, ignore_regular_events_enabled: bool) {
    self.ignore_regular_events_enabled = Some(ignore_regular_events_enabled);
  }

  pub fn with_ignore_regular_events_enabled(mut self, ignore_regular_events_enabled: bool) -> AlertRule {
    self.ignore_regular_events_enabled = Some(ignore_regular_events_enabled);
    self
  }

  pub fn ignore_regular_events_enabled(&self) -> Option<&bool> {
    self.ignore_regular_events_enabled.as_ref()
  }

  pub fn reset_ignore_regular_events_enabled(&mut self) {
    self.ignore_regular_events_enabled = None;
  }

  pub fn set_integrations(&mut self, integrations: String) {
    self.integrations = Some(integrations);
  }

  pub fn with_integrations(mut self, integrations: String) -> AlertRule {
    self.integrations = Some(integrations);
    self
  }

  pub fn integrations(&self) -> Option<&String> {
    self.integrations.as_ref()
  }

  pub fn reset_integrations(&mut self) {
    self.integrations = None;
  }

  pub fn set_last_data_received_date(&mut self, last_data_received_date: i64) {
    self.last_data_received_date = Some(last_data_received_date);
  }

  pub fn with_last_data_received_date(mut self, last_data_received_date: i64) -> AlertRule {
    self.last_data_received_date = Some(last_data_received_date);
    self
  }

  pub fn last_data_received_date(&self) -> Option<&i64> {
    self.last_data_received_date.as_ref()
  }

  pub fn reset_last_data_received_date(&mut self) {
    self.last_data_received_date = None;
  }

  pub fn set_last_sent(&mut self, last_sent: i64) {
    self.last_sent = Some(last_sent);
  }

  pub fn with_last_sent(mut self, last_sent: i64) -> AlertRule {
    self.last_sent = Some(last_sent);
    self
  }

  pub fn last_sent(&self) -> Option<&i64> {
    self.last_sent.as_ref()
  }

  pub fn reset_last_sent(&mut self) {
    self.last_sent = None;
  }

  pub fn set_last_triggered(&mut self, last_triggered: i64) {
    self.last_triggered = Some(last_triggered);
  }

  pub fn with_last_triggered(mut self, last_triggered: i64) -> AlertRule {
    self.last_triggered = Some(last_triggered);
    self
  }

  pub fn last_triggered(&self) -> Option<&i64> {
    self.last_triggered.as_ref()
  }

  pub fn reset_last_triggered(&mut self) {
    self.last_triggered = None;
  }

  pub fn set_metadata(&mut self, metadata: Value) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: Value) -> AlertRule {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&Value> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_metric_key(&mut self, metric_key: String) {
    self.metric_key = Some(metric_key);
  }

  pub fn with_metric_key(mut self, metric_key: String) -> AlertRule {
    self.metric_key = Some(metric_key);
    self
  }

  pub fn metric_key(&self) -> Option<&String> {
    self.metric_key.as_ref()
  }

  pub fn reset_metric_key(&mut self) {
    self.metric_key = None;
  }

  pub fn set_metric_label(&mut self, metric_label: String) {
    self.metric_label = Some(metric_label);
  }

  pub fn with_metric_label(mut self, metric_label: String) -> AlertRule {
    self.metric_label = Some(metric_label);
    self
  }

  pub fn metric_label(&self) -> Option<&String> {
    self.metric_label.as_ref()
  }

  pub fn reset_metric_label(&mut self) {
    self.metric_label = None;
  }

  pub fn set_min_delay_between_notifications_in_minutes(&mut self, min_delay_between_notifications_in_minutes: String) {
    self.min_delay_between_notifications_in_minutes = Some(min_delay_between_notifications_in_minutes);
  }

  pub fn with_min_delay_between_notifications_in_minutes(mut self, min_delay_between_notifications_in_minutes: String) -> AlertRule {
    self.min_delay_between_notifications_in_minutes = Some(min_delay_between_notifications_in_minutes);
    self
  }

  pub fn min_delay_between_notifications_in_minutes(&self) -> Option<&String> {
    self.min_delay_between_notifications_in_minutes.as_ref()
  }

  pub fn reset_min_delay_between_notifications_in_minutes(&mut self) {
    self.min_delay_between_notifications_in_minutes = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> AlertRule {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_notification_emails(&mut self, notification_emails: Vec<String>) {
    self.notification_emails = Some(notification_emails);
  }

  pub fn with_notification_emails(mut self, notification_emails: Vec<String>) -> AlertRule {
    self.notification_emails = Some(notification_emails);
    self
  }

  pub fn notification_emails(&self) -> Option<&Vec<String>> {
    self.notification_emails.as_ref()
  }

  pub fn reset_notification_emails(&mut self) {
    self.notification_emails = None;
  }

  pub fn set_notification_integrations(&mut self, notification_integrations: Vec<::models::NotificationIntegration>) {
    self.notification_integrations = Some(notification_integrations);
  }

  pub fn with_notification_integrations(mut self, notification_integrations: Vec<::models::NotificationIntegration>) -> AlertRule {
    self.notification_integrations = Some(notification_integrations);
    self
  }

  pub fn notification_integrations(&self) -> Option<&Vec<::models::NotificationIntegration>> {
    self.notification_integrations.as_ref()
  }

  pub fn reset_notification_integrations(&mut self) {
    self.notification_integrations = None;
  }

  pub fn set_notifications_enabled(&mut self, notifications_enabled: bool) {
    self.notifications_enabled = Some(notifications_enabled);
  }

  pub fn with_notifications_enabled(mut self, notifications_enabled: bool) -> AlertRule {
    self.notifications_enabled = Some(notifications_enabled);
    self
  }

  pub fn notifications_enabled(&self) -> Option<&bool> {
    self.notifications_enabled.as_ref()
  }

  pub fn reset_notifications_enabled(&mut self) {
    self.notifications_enabled = None;
  }

  pub fn set_query(&mut self, query: String) {
    self.query = Some(query);
  }

  pub fn with_query(mut self, query: String) -> AlertRule {
    self.query = Some(query);
    self
  }

  pub fn query(&self) -> Option<&String> {
    self.query.as_ref()
  }

  pub fn reset_query(&mut self) {
    self.query = None;
  }

  pub fn set_report_name(&mut self, report_name: String) {
    self.report_name = Some(report_name);
  }

  pub fn with_report_name(mut self, report_name: String) -> AlertRule {
    self.report_name = Some(report_name);
    self
  }

  pub fn report_name(&self) -> Option<&String> {
    self.report_name.as_ref()
  }

  pub fn reset_report_name(&mut self) {
    self.report_name = None;
  }

  pub fn set_rule_key(&mut self, rule_key: i64) {
    self.rule_key = Some(rule_key);
  }

  pub fn with_rule_key(mut self, rule_key: i64) -> AlertRule {
    self.rule_key = Some(rule_key);
    self
  }

  pub fn rule_key(&self) -> Option<&i64> {
    self.rule_key.as_ref()
  }

  pub fn reset_rule_key(&mut self) {
    self.rule_key = None;
  }

  pub fn set_rule_type(&mut self, rule_type: String) {
    self.rule_type = Some(rule_type);
  }

  pub fn with_rule_type(mut self, rule_type: String) -> AlertRule {
    self.rule_type = Some(rule_type);
    self
  }

  pub fn rule_type(&self) -> Option<&String> {
    self.rule_type.as_ref()
  }

  pub fn reset_rule_type(&mut self) {
    self.rule_type = None;
  }

  pub fn set_runbook(&mut self, runbook: String) {
    self.runbook = Some(runbook);
  }

  pub fn with_runbook(mut self, runbook: String) -> AlertRule {
    self.runbook = Some(runbook);
    self
  }

  pub fn runbook(&self) -> Option<&String> {
    self.runbook.as_ref()
  }

  pub fn reset_runbook(&mut self) {
    self.runbook = None;
  }

  pub fn set_saved_query_id(&mut self, saved_query_id: i64) {
    self.saved_query_id = Some(saved_query_id);
  }

  pub fn with_saved_query_id(mut self, saved_query_id: i64) -> AlertRule {
    self.saved_query_id = Some(saved_query_id);
    self
  }

  pub fn saved_query_id(&self) -> Option<&i64> {
    self.saved_query_id.as_ref()
  }

  pub fn reset_saved_query_id(&mut self) {
    self.saved_query_id = None;
  }

  pub fn set_schedule(&mut self, schedule: Vec<::models::AlertRuleScheduleWeekdayDto>) {
    self.schedule = Some(schedule);
  }

  pub fn with_schedule(mut self, schedule: Vec<::models::AlertRuleScheduleWeekdayDto>) -> AlertRule {
    self.schedule = Some(schedule);
    self
  }

  pub fn schedule(&self) -> Option<&Vec<::models::AlertRuleScheduleWeekdayDto>> {
    self.schedule.as_ref()
  }

  pub fn reset_schedule(&mut self) {
    self.schedule = None;
  }

  pub fn set_sematext_service(&mut self, sematext_service: String) {
    self.sematext_service = Some(sematext_service);
  }

  pub fn with_sematext_service(mut self, sematext_service: String) -> AlertRule {
    self.sematext_service = Some(sematext_service);
    self
  }

  pub fn sematext_service(&self) -> Option<&String> {
    self.sematext_service.as_ref()
  }

  pub fn reset_sematext_service(&mut self) {
    self.sematext_service = None;
  }

  pub fn set_send_to_email(&mut self, send_to_email: String) {
    self.send_to_email = Some(send_to_email);
  }

  pub fn with_send_to_email(mut self, send_to_email: String) -> AlertRule {
    self.send_to_email = Some(send_to_email);
    self
  }

  pub fn send_to_email(&self) -> Option<&String> {
    self.send_to_email.as_ref()
  }

  pub fn reset_send_to_email(&mut self) {
    self.send_to_email = None;
  }

  pub fn set_timezone(&mut self, timezone: String) {
    self.timezone = Some(timezone);
  }

  pub fn with_timezone(mut self, timezone: String) -> AlertRule {
    self.timezone = Some(timezone);
    self
  }

  pub fn timezone(&self) -> Option<&String> {
    self.timezone.as_ref()
  }

  pub fn reset_timezone(&mut self) {
    self.timezone = None;
  }

  pub fn set_use_only_alert_rule_integrations(&mut self, use_only_alert_rule_integrations: bool) {
    self.use_only_alert_rule_integrations = Some(use_only_alert_rule_integrations);
  }

  pub fn with_use_only_alert_rule_integrations(mut self, use_only_alert_rule_integrations: bool) -> AlertRule {
    self.use_only_alert_rule_integrations = Some(use_only_alert_rule_integrations);
    self
  }

  pub fn use_only_alert_rule_integrations(&self) -> Option<&bool> {
    self.use_only_alert_rule_integrations.as_ref()
  }

  pub fn reset_use_only_alert_rule_integrations(&mut self) {
    self.use_only_alert_rule_integrations = None;
  }

  pub fn set_user_permissions(&mut self, user_permissions: ::models::UserPermissions) {
    self.user_permissions = Some(user_permissions);
  }

  pub fn with_user_permissions(mut self, user_permissions: ::models::UserPermissions) -> AlertRule {
    self.user_permissions = Some(user_permissions);
    self
  }

  pub fn user_permissions(&self) -> Option<&::models::UserPermissions> {
    self.user_permissions.as_ref()
  }

  pub fn reset_user_permissions(&mut self) {
    self.user_permissions = None;
  }

  pub fn set_value_column_name(&mut self, value_column_name: String) {
    self.value_column_name = Some(value_column_name);
  }

  pub fn with_value_column_name(mut self, value_column_name: String) -> AlertRule {
    self.value_column_name = Some(value_column_name);
    self
  }

  pub fn value_column_name(&self) -> Option<&String> {
    self.value_column_name.as_ref()
  }

  pub fn reset_value_column_name(&mut self) {
    self.value_column_name = None;
  }

  pub fn set_value_name(&mut self, value_name: String) {
    self.value_name = Some(value_name);
  }

  pub fn with_value_name(mut self, value_name: String) -> AlertRule {
    self.value_name = Some(value_name);
    self
  }

  pub fn value_name(&self) -> Option<&String> {
    self.value_name.as_ref()
  }

  pub fn reset_value_name(&mut self) {
    self.value_name = None;
  }

}



