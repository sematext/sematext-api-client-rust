
# <img src="./assets/octi-blue.png" valign="bottom" width="60px"/>**&nbsp;&nbsp;sematext-api-client-rust**

<br>

>*A [Sematext Cloud](https://sematext.com/cloud/) API client, for interaction with Sematext Cloud solution monitoring, alerting and log shipping.*

<br>

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

<br>
One of a family of clients in following flavours:
<br>
<br>

* [sematext-api-client-javascript](https://github.com/sematext/sematext-api-client-javascript "Javascript")
* [sematext-api-client-rust](https://github.com/sematext/sematext-api-client-rust "Rust")
* [sematext-api-client-ruby](https://github.com/sematext/sematext-api-client-ruby "Ruby")
* [sematext-api-client-python](https://github.com/sematext/sematext-api-client-python "Python")
* [sematext-api-client-php](https://github.com/sematext/sematext-api-client-php "PHP")
* [sematext-api-client-java](https://github.com/sematext/sematext-api-client-java "Java")
* [sematext-api-client-go](https://github.com/sematext/sematext-api-client-go "Go/Golang")

<br>
Refer to below link for deeper information on the API itself.
<br>
<br>

* [Sematext Cloud API Reference](https://sematext.com/docs/api/ "API Reference")

<br>

## Contents

- [&nbsp;&nbsp;sematext-api-client-rust](#sematext-api-client-rust)
  - [Contents](#contents)
  - [Getting Started](#getting-started)
  - [Installation](#installation)
  - [Authentication](#authentication)
  - [Reference](#reference)
  - [Documentation for Models](#documentation-for-models)

<br>

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

## Authentication

This client code requires a Sematext API Access token to function. You can find this by logging into your [Sematext Cloud Account](https://apps.sematext.com/ui/account/api)


## Installation
Put the package under your project folder and add the following in import:
```
    "./stcloud"
```

## Reference

All URIs are relative to *https://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AlertNotificationsApi* | [**get_alert_notifications_for_app_using_post**](docs/AlertNotificationsApi.md#get_alert_notifications_for_app_using_post) | **Post** /users-web/api/v3/apps/{appId}/notifications/alerts | Get alert notifications for an app
*AlertNotificationsApi* | [**get_alert_notifications_for_user_using_post**](docs/AlertNotificationsApi.md#get_alert_notifications_for_user_using_post) | **Post** /users-web/api/v3/notifications/alerts | Get alert notifications for a user
*AlertsApi* | [**create_alert_using_post**](docs/AlertsApi.md#create_alert_using_post) | **Post** /users-web/api/v3/alerts | Create alert rule
*AlertsApi* | [**delete_alert_rule_using_delete**](docs/AlertsApi.md#delete_alert_rule_using_delete) | **Delete** /users-web/api/v3/alerts/{updateableAlertId} | Delete alert rule
*AlertsApi* | [**disable_alert_rule_using_put**](docs/AlertsApi.md#disable_alert_rule_using_put) | **Put** /users-web/api/v3/alerts/{updateableAlertId}/disable | Disable alert rule
*AlertsApi* | [**enable_alert_rule_using_put**](docs/AlertsApi.md#enable_alert_rule_using_put) | **Put** /users-web/api/v3/alerts/{updateableAlertId}/enable | Enable alert rule
*AlertsApi* | [**get_alert_rules_for_app_using_get**](docs/AlertsApi.md#get_alert_rules_for_app_using_get) | **Get** /users-web/api/v3/apps/{appId}/alerts | Get alert rules for an app
*AppsApi* | [**get_app_types_using_get**](docs/AppsApi.md#get_app_types_using_get) | **Get** /users-web/api/v3/apps/types | Get all App types supported for the account identified with apiKey
*AppsApi* | [**get_using_get**](docs/AppsApi.md#get_using_get) | **Get** /users-web/api/v3/apps/{anyStateAppId} | Gets defails for one particular App
*AppsApi* | [**invite_app_guests_using_post**](docs/AppsApi.md#invite_app_guests_using_post) | **Post** /users-web/api/v3/apps/guests | Invite guests to an app
*AppsApi* | [**list_apps_users_using_get**](docs/AppsApi.md#list_apps_users_using_get) | **Get** /users-web/api/v3/apps/users | Get all users of apps accessible to this account
*AppsApi* | [**list_using_get**](docs/AppsApi.md#list_using_get) | **Get** /users-web/api/v3/apps | Get all apps accessible by account identified with apiKey
*AppsApi* | [**update_description_using_put**](docs/AppsApi.md#update_description_using_put) | **Put** /users-web/api/v3/apps/{anyStateAppId}/description | Update description of the app
*AppsApi* | [**update_using_put1**](docs/AppsApi.md#update_using_put1) | **Put** /users-web/api/v3/apps/{anyStateAppId} | Update app
*AwsSettingsControllerApi* | [**update_using_put**](docs/AwsSettingsControllerApi.md#update_using_put) | **Put** /users-web/api/v3/apps/{appId}/aws | Update App&#39;s AWS CloudWatch settings
*BillingApi* | [**get_detailed_invoice_using_get**](docs/BillingApi.md#get_detailed_invoice_using_get) | **Get** /users-web/api/v3/billing/invoice/{service}/{year}/{month} | Get invoice details
*BillingApi* | [**list_available_plans_using_get**](docs/BillingApi.md#list_available_plans_using_get) | **Get** /users-web/api/v3/billing/availablePlans | Get available plans
*BillingApi* | [**update_plan_using_put**](docs/BillingApi.md#update_plan_using_put) | **Put** /users-web/api/v3/billing/info/{appId} | Update plan for an app
*LogsAppApi* | [**create_logsene_application**](docs/LogsAppApi.md#create_logsene_application) | **Post** /logsene-reports/api/v3/apps | Create Logs App
*MetricsApi* | [**list_data_series_using_post1**](docs/MetricsApi.md#list_data_series_using_post1) | **Post** /spm-reports/api/v3/apps/{appId}/metrics/data | Get metrics data points for an app
*MetricsApi* | [**list_filters_using_post**](docs/MetricsApi.md#list_filters_using_post) | **Post** /spm-reports/api/v3/apps/{appId}/metrics/filters | Get metrics filters and their values for an app
*MetricsApi* | [**list_metrics_keys_using_get1**](docs/MetricsApi.md#list_metrics_keys_using_get1) | **Get** /spm-reports/api/v3/apps/{appId}/metrics/keys | Get metrics keys for an app
*MetricsApi* | [**list_metrics_using_get1**](docs/MetricsApi.md#list_metrics_using_get1) | **Get** /spm-reports/api/v3/apps/{appId}/metrics | Get metrics info for an app
*MonitoringAppApi* | [**create_spm_application**](docs/MonitoringAppApi.md#create_spm_application) | **Post** /spm-reports/api/v3/apps | Create Monitoring App
*ResetPasswordApi* | [**reset_password_using_post**](docs/ResetPasswordApi.md#reset_password_using_post) | **Post** /users-web/api/v3/account/password/reset | Reset Password
*SavedQueriesApi* | [**delete_saved_query_using_delete**](docs/SavedQueriesApi.md#delete_saved_query_using_delete) | **Delete** /users-web/api/v3/savedQueries/{updateableQueryId} | Delete saved query
*SavedQueriesApi* | [**get_saved_queries_for_app_using_get**](docs/SavedQueriesApi.md#get_saved_queries_for_app_using_get) | **Get** /users-web/api/v3/apps/{appId}/savedQueries | Get saved queries for an app
*SavedQueriesApi* | [**save_query_using_post**](docs/SavedQueriesApi.md#save_query_using_post) | **Post** /users-web/api/v3/savedQueries | Create saved query
*SavedQueriesApi* | [**save_query_using_put**](docs/SavedQueriesApi.md#save_query_using_put) | **Put** /users-web/api/v3/savedQueries/{updateableQueryId} | Update saved query
*SubscriptionsApi* | [**list_using_get1**](docs/SubscriptionsApi.md#list_using_get1) | **Get** /users-web/api/v3/apps/{appId}/subscriptions | Get subscriptions for an app
*SubscriptionsApi* | [**send_report_using_post**](docs/SubscriptionsApi.md#send_report_using_post) | **Post** /users-web/api/v3/apps/{appId}/report/send | Trigger emailing of report for an app
*TagApiControllerApi* | [**get_tag_names_using_get**](docs/TagApiControllerApi.md#get_tag_names_using_get) | **Get** /spm-reports/api/v3/apps/{appIds}/tagNames | Gets tag names for the given application identifiers appearing in the given time frame.
*TagApiControllerApi* | [**get_using_get2**](docs/TagApiControllerApi.md#get_using_get2) | **Get** /spm-reports/api/v3/apps/{appIds}/metrics/filters | Gets values for specified tags for the given application identifiers appearing in the given time frame.
*TagApiControllerApi* | [**get_using_get3**](docs/TagApiControllerApi.md#get_using_get3) | **Get** /spm-reports/api/v3/apps/{appIds}/tags | Gets values for specified tags for the given application identifiers appearing in the given time frame.


## Documentation For Models

 - [AlertNotificationRequest](docs/AlertNotificationRequest.md)
 - [AlertRule](docs/AlertRule.md)
 - [AlertRuleScheduleTimeRangeDto](docs/AlertRuleScheduleTimeRangeDto.md)
 - [AlertRuleScheduleWeekdayDto](docs/AlertRuleScheduleWeekdayDto.md)
 - [App](docs/App.md)
 - [AppDescription](docs/AppDescription.md)
 - [AppMetadata](docs/AppMetadata.md)
 - [BasicAuthMethodDto](docs/BasicAuthMethodDto.md)
 - [BasicOrganizationDto](docs/BasicOrganizationDto.md)
 - [BillingInfo](docs/BillingInfo.md)
 - [CloudWatchSettings](docs/CloudWatchSettings.md)
 - [CreateAppInfo](docs/CreateAppInfo.md)
 - [DataSeriesFilter](docs/DataSeriesFilter.md)
 - [DataSeriesRequest](docs/DataSeriesRequest.md)
 - [Error](docs/Error.md)
 - [FilterValue](docs/FilterValue.md)
 - [GenericApiResponse](docs/GenericApiResponse.md)
 - [Invitation](docs/Invitation.md)
 - [NotificationIntegration](docs/NotificationIntegration.md)
 - [Plan](docs/Plan.md)
 - [ReportInfo](docs/ReportInfo.md)
 - [SavedQuery](docs/SavedQuery.md)
 - [ServiceIntegration](docs/ServiceIntegration.md)
 - [UpdateAppInfo](docs/UpdateAppInfo.md)
 - [UserInfo](docs/UserInfo.md)
 - [UserPermissions](docs/UserPermissions.md)
 - [UserRole](docs/UserRole.md)



