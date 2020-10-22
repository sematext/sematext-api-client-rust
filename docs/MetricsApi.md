# \MetricsApi

All URIs are relative to *http://localhost*

| Method                                                                         | HTTP request                                              | Description                                     |
| ------------------------------------------------------------------------------ | --------------------------------------------------------- | ----------------------------------------------- |
| [**list_data_series_using_post1**](MetricsApi.md#list_data_series_using_post1) | **Post** /spm-reports/api/v3/apps/{appId}/metrics/data    | Get metrics data points for an app              |
| [**list_filters_using_post1**](MetricsApi.md#list_filters_using_post1)         | **Post** /spm-reports/api/v3/apps/{appId}/metrics/filters | Get metrics filters and their values for an app |
| [**list_metrics_keys_using_get1**](MetricsApi.md#list_metrics_keys_using_get1) | **Get** /spm-reports/api/v3/apps/{appId}/metrics/keys     | Get metrics keys for an app                     |
| [**list_metrics_using_get**](MetricsApi.md#list_metrics_using_get)             | **Get** /spm-reports/api/v3/apps/{appId}/metrics          | Get metrics info for an app                     |



## list_data_series_using_post1

> crate::models::GenericApiResponse list_data_series_using_post1(app_id, request_body)
Get metrics data points for an app

Default value of interval is 5m

### Parameters


| Name             | Type                                          | Description                | Required   | Notes |
| ---------------- | --------------------------------------------- | -------------------------- | ---------- | ----- |
| **app_id**       | **i64**                                       | appId                      | [required] |
| **request_body** | [**DataSeriesRequest**](DataSeriesRequest.md) | Metric data points request | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_filters_using_post1

> crate::models::GenericApiResponse list_filters_using_post1(app_id, request_body)
Get metrics filters and their values for an app

Default value of interval is 5m

### Parameters


| Name             | Type                                          | Description            | Required   | Notes |
| ---------------- | --------------------------------------------- | ---------------------- | ---------- | ----- |
| **app_id**       | **i64**                                       | appId                  | [required] |
| **request_body** | [**DataSeriesRequest**](DataSeriesRequest.md) | Metric filters request | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metrics_keys_using_get1

> crate::models::GenericApiResponse list_metrics_keys_using_get1(app_id)
Get metrics keys for an app

### Parameters


| Name       | Type    | Description | Required   | Notes |
| ---------- | ------- | ----------- | ---------- | ----- |
| **app_id** | **i64** | appId       | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metrics_using_get

> crate::models::GenericApiResponse list_metrics_using_get(app_id)
Get metrics info for an app

### Parameters


| Name       | Type    | Description | Required   | Notes |
| ---------- | ------- | ----------- | ---------- | ----- |
| **app_id** | **i64** | appId       | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
