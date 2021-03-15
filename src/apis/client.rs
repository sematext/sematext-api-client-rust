use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  alert_notifications_api: Box<::apis::AlertNotificationsApi>,
  alerts_api: Box<::apis::AlertsApi>,
  apps_api: Box<::apis::AppsApi>,
  aws_settings_controller_api: Box<::apis::AwsSettingsControllerApi>,
  billing_api: Box<::apis::BillingApi>,
  logs_app_api: Box<::apis::LogsAppApi>,
  logs_usage_api_controller_api: Box<::apis::LogsUsageApiControllerApi>,
  monitoring_app_api: Box<::apis::MonitoringAppApi>,
  reset_password_api: Box<::apis::ResetPasswordApi>,
  subscriptions_api: Box<::apis::SubscriptionsApi>,
  tag_api_controller_api: Box<::apis::TagApiControllerApi>,
  tokens_api_controller_api: Box<::apis::TokensApiControllerApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      alert_notifications_api: Box::new(::apis::AlertNotificationsApiClient::new(rc.clone())),
      alerts_api: Box::new(::apis::AlertsApiClient::new(rc.clone())),
      apps_api: Box::new(::apis::AppsApiClient::new(rc.clone())),
      aws_settings_controller_api: Box::new(::apis::AwsSettingsControllerApiClient::new(rc.clone())),
      billing_api: Box::new(::apis::BillingApiClient::new(rc.clone())),
      logs_app_api: Box::new(::apis::LogsAppApiClient::new(rc.clone())),
      logs_usage_api_controller_api: Box::new(::apis::LogsUsageApiControllerApiClient::new(rc.clone())),
      monitoring_app_api: Box::new(::apis::MonitoringAppApiClient::new(rc.clone())),
      reset_password_api: Box::new(::apis::ResetPasswordApiClient::new(rc.clone())),
      subscriptions_api: Box::new(::apis::SubscriptionsApiClient::new(rc.clone())),
      tag_api_controller_api: Box::new(::apis::TagApiControllerApiClient::new(rc.clone())),
      tokens_api_controller_api: Box::new(::apis::TokensApiControllerApiClient::new(rc.clone())),
    }
  }

  pub fn alert_notifications_api(&self) -> &::apis::AlertNotificationsApi{
    self.alert_notifications_api.as_ref()
  }

  pub fn alerts_api(&self) -> &::apis::AlertsApi{
    self.alerts_api.as_ref()
  }

  pub fn apps_api(&self) -> &::apis::AppsApi{
    self.apps_api.as_ref()
  }

  pub fn aws_settings_controller_api(&self) -> &::apis::AwsSettingsControllerApi{
    self.aws_settings_controller_api.as_ref()
  }

  pub fn billing_api(&self) -> &::apis::BillingApi{
    self.billing_api.as_ref()
  }

  pub fn logs_app_api(&self) -> &::apis::LogsAppApi{
    self.logs_app_api.as_ref()
  }

  pub fn logs_usage_api_controller_api(&self) -> &::apis::LogsUsageApiControllerApi{
    self.logs_usage_api_controller_api.as_ref()
  }

  pub fn monitoring_app_api(&self) -> &::apis::MonitoringAppApi{
    self.monitoring_app_api.as_ref()
  }

  pub fn reset_password_api(&self) -> &::apis::ResetPasswordApi{
    self.reset_password_api.as_ref()
  }

  pub fn subscriptions_api(&self) -> &::apis::SubscriptionsApi{
    self.subscriptions_api.as_ref()
  }

  pub fn tag_api_controller_api(&self) -> &::apis::TagApiControllerApi{
    self.tag_api_controller_api.as_ref()
  }

  pub fn tokens_api_controller_api(&self) -> &::apis::TokensApiControllerApi{
    self.tokens_api_controller_api.as_ref()
  }


}
