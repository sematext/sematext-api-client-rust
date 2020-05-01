# \SubscriptionsApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_using_get1**](SubscriptionsApi.md#list_using_get1) | **Get** /users-web/api/v3/apps/{appId}/subscriptions | Get subscriptions for an app
[**send_report_using_post**](SubscriptionsApi.md#send_report_using_post) | **Post** /users-web/api/v3/apps/{appId}/report/send | Trigger emailing of report for an app


# **list_using_get1**
> ::models::GenericApiResponse list_using_get1(ctx, app_id)
Get subscriptions for an app

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **i64**| appId | 

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **send_report_using_post**
> ::models::GenericApiResponse send_report_using_post(ctx, app_id, email_dto)
Trigger emailing of report for an app

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **i64**| appId | 
  **email_dto** | [**ReportInfo**](ReportInfo.md)| emailDto | 

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

