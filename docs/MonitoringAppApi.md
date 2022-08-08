# {{classname}}

All URIs are relative to */*

| Method                                                                     | HTTP request                     | Description           |
| -------------------------------------------------------------------------- | -------------------------------- | --------------------- |
| [**create_spm_application1**](MonitoringAppApi.md#create_spm_application1) | **POST** spm-reports/api/v3/apps | Create Monitoring App |

# **create_spm_application1**

> AppsResponse create_spm_application1(ctx, body)
Create Monitoring App

### Required Parameters

| Name     | Type                                  | Description                              | Notes                    |
| -------- | ------------------------------------- | ---------------------------------------- | ------------------------ |
| **ctx**  | **context.Context**                   | context containing the authentication    | nil if no authentication |
| **body** | [**CreateAppInfo**](CreateAppInfo.md) | Details of the application to be created |

### Return type

[**AppsResponse**](AppsResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
