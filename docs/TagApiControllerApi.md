# \TagApiControllerApi

All URIs are relative to *http://localhost*

| Method                                                                        | HTTP request                                              | Description                                                                                             |
| ----------------------------------------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| [**get_tag_names_using_get**](TagApiControllerApi.md#get_tag_names_using_get) | **Get** /spm-reports/api/v3/apps/{appIds}/tagNames        | Gets tag names for the given application identifiers appearing in the given time frame.                 |
| [**get_using_get2**](TagApiControllerApi.md#get_using_get2)                   | **Get** /spm-reports/api/v3/apps/{appIds}/metrics/filters | Gets values for specified tags for the given application identifiers appearing in the given time frame. |
| [**get_using_get3**](TagApiControllerApi.md#get_using_get3)                   | **Get** /spm-reports/api/v3/apps/{appIds}/tags            | Gets values for specified tags for the given application identifiers appearing in the given time frame. |



## get_tag_names_using_get

> serde_json::Value get_tag_names_using_get(app_ids, from, to, metrics, logs, events, rum)
Gets tag names for the given application identifiers appearing in the given time frame.

### Parameters


| Name        | Type             | Description | Required   | Notes              |
| ----------- | ---------------- | ----------- | ---------- | ------------------ |
| **app_ids** | **String**       | appIds      | [required] |
| **from**    | Option<**i64**>  | from        |            |
| **to**      | Option<**i64**>  | to          |            |
| **metrics** | Option<**bool**> | metrics     |            | [default to true]  |
| **logs**    | Option<**bool**> | logs        |            | [default to true]  |
| **events**  | Option<**bool**> | events      |            | [default to false] |
| **rum**     | Option<**bool**> | rum         |            | [default to true]  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_using_get2

> serde_json::Value get_using_get2(app_ids, tag, from, to, metrics, logs, events, rum)
Gets values for specified tags for the given application identifiers appearing in the given time frame.

### Parameters


| Name        | Type                         | Description | Required   | Notes              |
| ----------- | ---------------------------- | ----------- | ---------- | ------------------ |
| **app_ids** | **String**                   | appIds      | [required] |
| **tag**     | [**Vec<String>**](String.md) | tag         | [required] |
| **from**    | Option<**i64**>              | from        |            |
| **to**      | Option<**i64**>              | to          |            |
| **metrics** | Option<**bool**>             | metrics     |            | [default to true]  |
| **logs**    | Option<**bool**>             | logs        |            | [default to true]  |
| **events**  | Option<**bool**>             | events      |            | [default to false] |
| **rum**     | Option<**bool**>             | rum         |            | [default to true]  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_using_get3

> serde_json::Value get_using_get3(app_ids, tag, from, to, metrics, logs, events, rum)
Gets values for specified tags for the given application identifiers appearing in the given time frame.

### Parameters


| Name        | Type                         | Description | Required   | Notes              |
| ----------- | ---------------------------- | ----------- | ---------- | ------------------ |
| **app_ids** | **String**                   | appIds      | [required] |
| **tag**     | [**Vec<String>**](String.md) | tag         | [required] |
| **from**    | Option<**i64**>              | from        |            |
| **to**      | Option<**i64**>              | to          |            |
| **metrics** | Option<**bool**>             | metrics     |            | [default to true]  |
| **logs**    | Option<**bool**>             | logs        |            | [default to true]  |
| **events**  | Option<**bool**>             | events      |            | [default to false] |
| **rum**     | Option<**bool**>             | rum         |            | [default to true]  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
