# {{classname}}

All URIs are relative to */*

| Method                                                               | HTTP request                              | Description                               |
| -------------------------------------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| [**update_using_put**](AwsSettingsControllerApi.md#update_using_put) | **PUT** users-web/api/v3/apps/{appId}/aws | Update App&#x27;s AWS CloudWatch settings |

# **update_using_put**

> CloudWatchSettingsResponse update_using_put(ctx, body, app_id)
Update App's AWS CloudWatch settings

Applicable only for AWS Apps

### Required Parameters

| Name       | Type                                            | Description                           | Notes                    |
| ---------- | ----------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context**                             | context containing the authentication | nil if no authentication |
| **body**   | [**CloudWatchSettings**](CloudWatchSettings.md) | dto                                   |
| **app_id** | **i64**                                         | appId                                 |

### Return type

[**CloudWatchSettingsResponse**](CloudWatchSettingsResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
