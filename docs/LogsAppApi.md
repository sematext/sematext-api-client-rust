# \LogsAppApi

All URIs are relative to *http://localhost*

| Method                                                                     | HTTP request                          | Description     |
| -------------------------------------------------------------------------- | ------------------------------------- | --------------- |
| [**create_logsene_application**](LogsAppApi.md#create_logsene_application) | **Post** /logsene-reports/api/v3/apps | Create Logs App |



## create_logsene_application

> crate::models::GenericApiResponse create_logsene_application(application_details)
Create Logs App

### Parameters


| Name                    | Type                                  | Description                              | Required   | Notes |
| ----------------------- | ------------------------------------- | ---------------------------------------- | ---------- | ----- |
| **application_details** | [**CreateAppInfo**](CreateAppInfo.md) | Details of the application to be created | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
