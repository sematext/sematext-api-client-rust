# \LogsUsageApiControllerApi

All URIs are relative to *<https://localhost>*

| Method                                                                              | HTTP request                                                   | Description |
| ----------------------------------------------------------------------------------- | -------------------------------------------------------------- | ----------- |
| [**get_for_range_using_get**](LogsUsageApiControllerApi.md#get_for_range_using_get) | **Get** /logsene-reports/api/v3/apps/{appId}/usage/{from}/{to} | getForRange |

# **get_for_range_using_get**

> ::models::UsageResponse get_for_range_using_get(ctx, app_id, from, to)
getForRange

### Required Parameters

| Name       | Type                | Description                           | Notes                    |
| ---------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context** | context containing the authentication | nil if no authentication |
| **app_id** | **i64**             | appId                                 |
| **from**   | **i64**             | from                                  |
| **to**     | **i64**             | to                                    |

### Return type

[**::models::UsageResponse**](UsageResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
