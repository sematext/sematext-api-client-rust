# \MetricsApi

All URIs are relative to *https://localhost*

| Method                                                                         | HTTP request                                              | Description                                     |
| ------------------------------------------------------------------------------ | --------------------------------------------------------- | ----------------------------------------------- |
| [**list_data_series_using_post1**](MetricsApi.md#list_data_series_using_post1) | **Post** /spm-reports/api/v3/apps/{appId}/metrics/data    | Get metrics data points for an app              |
| [**list_filters_using_post**](MetricsApi.md#list_filters_using_post)           | **Post** /spm-reports/api/v3/apps/{appId}/metrics/filters | Get metrics filters and their values for an app |
| [**list_metrics_keys_using_get1**](MetricsApi.md#list_metrics_keys_using_get1) | **Get** /spm-reports/api/v3/apps/{appId}/metrics/keys     | Get metrics keys for an app                     |
| [**list_metrics_using_get1**](MetricsApi.md#list_metrics_using_get1)           | **Get** /spm-reports/api/v3/apps/{appId}/metrics          | Get metrics info for an app                     |


# **list_data_series_using_post1**
> ::models::GenericApiResponse list_data_series_using_post1(ctx, app_id, request_body)
Get metrics data points for an app

Default value of interval is 5m

### Required Parameters

| Name             | Type                                          | Description                           | Notes                    |
| ---------------- | --------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**          | **context.Context**                           | context containing the authentication | nil if no authentication |
| **app_id**       | **i64**                                       | appId                                 |
| **request_body** | [**DataSeriesRequest**](DataSeriesRequest.md) | Metric data points request            |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_filters_using_post**
> ::models::GenericApiResponse list_filters_using_post(ctx, app_id, request_body)
Get metrics filters and their values for an app

Default value of interval is 5m

### Required Parameters

| Name             | Type                                          | Description                           | Notes                    |
| ---------------- | --------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**          | **context.Context**                           | context containing the authentication | nil if no authentication |
| **app_id**       | **i64**                                       | appId                                 |
| **request_body** | [**DataSeriesRequest**](DataSeriesRequest.md) | Metric filters request                |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_metrics_keys_using_get1**
> ::models::GenericApiResponse list_metrics_keys_using_get1(ctx, app_id)
Get metrics keys for an app

### Required Parameters

| Name       | Type                | Description                           | Notes                    |
| ---------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context** | context containing the authentication | nil if no authentication |
| **app_id** | **i64**             | appId                                 |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_metrics_using_get1**
> ::models::GenericApiResponse list_metrics_using_get1(ctx, app_id)
Get metrics info for an app

### Required Parameters

| Name       | Type                | Description                           | Notes                    |
| ---------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context** | context containing the authentication | nil if no authentication |
| **app_id** | **i64**             | appId                                 |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
