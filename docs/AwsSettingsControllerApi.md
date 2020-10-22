# \AwsSettingsControllerApi

All URIs are relative to *http://localhost*

| Method                                                               | HTTP request                               | Description                          |
| -------------------------------------------------------------------- | ------------------------------------------ | ------------------------------------ |
| [**update_using_put**](AwsSettingsControllerApi.md#update_using_put) | **Put** /users-web/api/v3/apps/{appId}/aws | Update App's AWS CloudWatch settings |



## update_using_put

> crate::models::GenericApiResponse update_using_put(app_id, dto)
Update App's AWS CloudWatch settings

Applicable only for AWS Apps

### Parameters


| Name       | Type                                            | Description | Required   | Notes |
| ---------- | ----------------------------------------------- | ----------- | ---------- | ----- |
| **app_id** | **i64**                                         | appId       | [required] |
| **dto**    | [**CloudWatchSettings**](CloudWatchSettings.md) | dto         | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
