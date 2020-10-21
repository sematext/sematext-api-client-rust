use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T>
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod alert_notifications_api;
pub use self::alert_notifications_api::{ AlertNotificationsApi, AlertNotificationsApiClient };
mod alerts_api;
pub use self::alerts_api::{ AlertsApi, AlertsApiClient };
mod apps_api;
pub use self::apps_api::{ AppsApi, AppsApiClient };
mod aws_settings_controller_api;
pub use self::aws_settings_controller_api::{ AwsSettingsControllerApi, AwsSettingsControllerApiClient };
mod billing_api;
pub use self::billing_api::{ BillingApi, BillingApiClient };
mod logs_app_api;
pub use self::logs_app_api::{ LogsAppApi, LogsAppApiClient };
mod metrics_api;
pub use self::metrics_api::{ MetricsApi, MetricsApiClient };
mod monitoring_app_api;
pub use self::monitoring_app_api::{ MonitoringAppApi, MonitoringAppApiClient };
mod reset_password_api;
pub use self::reset_password_api::{ ResetPasswordApi, ResetPasswordApiClient };
mod saved_queries_api;
pub use self::saved_queries_api::{ SavedQueriesApi, SavedQueriesApiClient };
mod subscriptions_api;
pub use self::subscriptions_api::{ SubscriptionsApi, SubscriptionsApiClient };
mod tag_api_controller_api;
pub use self::tag_api_controller_api::{ TagApiControllerApi, TagApiControllerApiClient };
mod tokens_api_controller_api;
pub use self::tokens_api_controller_api::{ TokensApiControllerApi, TokensApiControllerApiClient };

pub mod configuration;
pub mod client;
