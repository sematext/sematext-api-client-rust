use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    alert_notifications_api: Box<dyn crate::apis::AlertNotificationsApi>,
    alerts_api: Box<dyn crate::apis::AlertsApi>,
    apps_api: Box<dyn crate::apis::AppsApi>,
    aws_settings_controller_api: Box<dyn crate::apis::AwsSettingsControllerApi>,
    billing_api: Box<dyn crate::apis::BillingApi>,
    logs_app_api: Box<dyn crate::apis::LogsAppApi>,
    metrics_api: Box<dyn crate::apis::MetricsApi>,
    monitoring_app_api: Box<dyn crate::apis::MonitoringAppApi>,
    reset_password_api: Box<dyn crate::apis::ResetPasswordApi>,
    saved_queries_api: Box<dyn crate::apis::SavedQueriesApi>,
    subscriptions_api: Box<dyn crate::apis::SubscriptionsApi>,
    tag_api_controller_api: Box<dyn crate::apis::TagApiControllerApi>,
    tokens_api_controller_api: Box<dyn crate::apis::TokensApiControllerApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            alert_notifications_api: Box::new(crate::apis::AlertNotificationsApiClient::new(rc.clone())),
            alerts_api: Box::new(crate::apis::AlertsApiClient::new(rc.clone())),
            apps_api: Box::new(crate::apis::AppsApiClient::new(rc.clone())),
            aws_settings_controller_api: Box::new(crate::apis::AwsSettingsControllerApiClient::new(rc.clone())),
            billing_api: Box::new(crate::apis::BillingApiClient::new(rc.clone())),
            logs_app_api: Box::new(crate::apis::LogsAppApiClient::new(rc.clone())),
            metrics_api: Box::new(crate::apis::MetricsApiClient::new(rc.clone())),
            monitoring_app_api: Box::new(crate::apis::MonitoringAppApiClient::new(rc.clone())),
            reset_password_api: Box::new(crate::apis::ResetPasswordApiClient::new(rc.clone())),
            saved_queries_api: Box::new(crate::apis::SavedQueriesApiClient::new(rc.clone())),
            subscriptions_api: Box::new(crate::apis::SubscriptionsApiClient::new(rc.clone())),
            tag_api_controller_api: Box::new(crate::apis::TagApiControllerApiClient::new(rc.clone())),
            tokens_api_controller_api: Box::new(crate::apis::TokensApiControllerApiClient::new(rc.clone())),
        }
    }

    pub fn alert_notifications_api(&self) -> &dyn crate::apis::AlertNotificationsApi{
        self.alert_notifications_api.as_ref()
    }

    pub fn alerts_api(&self) -> &dyn crate::apis::AlertsApi{
        self.alerts_api.as_ref()
    }

    pub fn apps_api(&self) -> &dyn crate::apis::AppsApi{
        self.apps_api.as_ref()
    }

    pub fn aws_settings_controller_api(&self) -> &dyn crate::apis::AwsSettingsControllerApi{
        self.aws_settings_controller_api.as_ref()
    }

    pub fn billing_api(&self) -> &dyn crate::apis::BillingApi{
        self.billing_api.as_ref()
    }

    pub fn logs_app_api(&self) -> &dyn crate::apis::LogsAppApi{
        self.logs_app_api.as_ref()
    }

    pub fn metrics_api(&self) -> &dyn crate::apis::MetricsApi{
        self.metrics_api.as_ref()
    }

    pub fn monitoring_app_api(&self) -> &dyn crate::apis::MonitoringAppApi{
        self.monitoring_app_api.as_ref()
    }

    pub fn reset_password_api(&self) -> &dyn crate::apis::ResetPasswordApi{
        self.reset_password_api.as_ref()
    }

    pub fn saved_queries_api(&self) -> &dyn crate::apis::SavedQueriesApi{
        self.saved_queries_api.as_ref()
    }

    pub fn subscriptions_api(&self) -> &dyn crate::apis::SubscriptionsApi{
        self.subscriptions_api.as_ref()
    }

    pub fn tag_api_controller_api(&self) -> &dyn crate::apis::TagApiControllerApi{
        self.tag_api_controller_api.as_ref()
    }

    pub fn tokens_api_controller_api(&self) -> &dyn crate::apis::TokensApiControllerApi{
        self.tokens_api_controller_api.as_ref()
    }

}
