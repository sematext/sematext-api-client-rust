use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  alert_notifications_api: Box<dyn (::apis::AlertNotificationsApi)>,
  alerts_api: Box<dyn (::apis::AlertsApi)>,
  apps_api: Box<dyn (::apis::AppsApi)>,
  aws_settings_controller_api: Box<dyn (::apis::AwsSettingsControllerApi)>,
  billing_api: Box<dyn (::apis::BillingApi)>,
  logs_app_api: Box<dyn (::apis::LogsAppApi)>,
  metrics_api: Box<dyn (::apis::MetricsApi)>,
  monitoring_app_api: Box<dyn (::apis::MonitoringAppApi)>,
  reset_password_api: Box<dyn (::apis::ResetPasswordApi)>,
  saved_queries_api: Box<dyn (::apis::SavedQueriesApi)>,
  subscriptions_api: Box<dyn (::apis::SubscriptionsApi)>,
  tag_api_controller_api: Box<dyn (::apis::TagApiControllerApi)>,
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
      metrics_api: Box::new(::apis::MetricsApiClient::new(rc.clone())),
      monitoring_app_api: Box::new(::apis::MonitoringAppApiClient::new(rc.clone())),
      reset_password_api: Box::new(::apis::ResetPasswordApiClient::new(rc.clone())),
      saved_queries_api: Box::new(::apis::SavedQueriesApiClient::new(rc.clone())),
      subscriptions_api: Box::new(::apis::SubscriptionsApiClient::new(rc.clone())),
      tag_api_controller_api: Box::new(::apis::TagApiControllerApiClient::new(rc.clone())),
    }
  }

  pub fn alert_notifications_api(&self) -> &dyn (::apis::AlertNotificationsApi){
    self.alert_notifications_api.as_ref()
  }

  pub fn alerts_api(&self) -> &dyn (::apis::AlertsApi){
    self.alerts_api.as_ref()
  }

  pub fn apps_api(&self) -> &dyn(::apis::AppsApi){
    self.apps_api.as_ref()
  }

  pub fn aws_settings_controller_api(&self) -> &dyn (::apis::AwsSettingsControllerApi){
    self.aws_settings_controller_api.as_ref()
  }

  pub fn billing_api(&self) -> &dyn (::apis::BillingApi){
    self.billing_api.as_ref()
  }

  pub fn logs_app_api(&self) -> &dyn (::apis::LogsAppApi){
    self.logs_app_api.as_ref()
  }

  pub fn metrics_api(&self) -> &dyn (::apis::MetricsApi){
    self.metrics_api.as_ref()
  }

  pub fn monitoring_app_api(&self) -> &dyn (::apis::MonitoringAppApi){
    self.monitoring_app_api.as_ref()
  }

  pub fn reset_password_api(&self) -> &dyn (::apis::ResetPasswordApi){
    self.reset_password_api.as_ref()
  }

  pub fn saved_queries_api(&self) -> &dyn (::apis::SavedQueriesApi){
    self.saved_queries_api.as_ref()
  }

  pub fn subscriptions_api(&self) -> &dyn (::apis::SubscriptionsApi){
    self.subscriptions_api.as_ref()
  }

  pub fn tag_api_controller_api(&self) -> &dyn (::apis::TagApiControllerApi){
    self.tag_api_controller_api.as_ref()
  }


}
