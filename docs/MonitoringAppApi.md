# \MonitoringAppApi

All URIs are relative to *https://localhost*

| Method                                                                   | HTTP request                      | Description           |
| ------------------------------------------------------------------------ | --------------------------------- | --------------------- |
| [**create_spm_application**](MonitoringAppApi.md#create_spm_application) | **Post** /spm-reports/api/v3/apps | Create Monitoring App |


# **create_spm_application**
> ::models::GenericApiResponse create_spm_application(ctx, application_details)
Create Monitoring App

### Required Parameters

| Name                    | Type                                  | Description                              | Notes                    |
| ----------------------- | ------------------------------------- | ---------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context**                   | context containing the authentication    | nil if no authentication |
| **application_details** | [**CreateAppInfo**](CreateAppInfo.md) | Details of the application to be created |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
