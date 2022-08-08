# {{classname}}

All URIs are relative to */*

| Method                                                                                            | HTTP request                                             | Description                                                                                             |
| ------------------------------------------------------------------------------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| [**get_tag_names_using_get**](TagApiControllerApi.md#get_tag_names_using_get)                     | **GET** spm-reports/api/v3/apps/{appIds}/tagNames        | Gets tag names for the given application identifiers appearing in the given time frame.                 |
| [**get_tags_grouped_by_id_using_get1**](TagApiControllerApi.md#get_tags_grouped_by_id_using_get1) | **GET** spm-reports/api/v3/apps/{appIds}/grouped         | Gets tag names grouped by application id.                                                               |
| [**get_using_get**](TagApiControllerApi.md#get_using_get)                                         | **GET** spm-reports/api/v3/apps/{appIds}/metrics/filters | Gets values for specified tags for the given application identifiers appearing in the given time frame. |
| [**get_using_get1**](TagApiControllerApi.md#get_using_get1)                                       | **GET** spm-reports/api/v3/apps/{appIds}/tags            | Gets values for specified tags for the given application identifiers appearing in the given time frame. |

# **get_tag_names_using_get**

> TagNamesResponse get_tag_names_using_get(ctx, app_ids, optional)
Gets tag names for the given application identifiers appearing in the given time frame.

### Required Parameters

| Name         | Type                       | Description                           | Notes                    |
| ------------ | -------------------------- | ------------------------------------- | ------------------------ |
| **ctx**      | **context.Context**        | context containing the authentication | nil if no authentication |
| **app_ids**  | **String**                 | appIds                                |
| **optional** | **map[string]interface{}** | optional parameters                   | nil if no parameters     |

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.
| Name        | Type       | Description | Notes              |
| ----------- | ---------- | ----------- | ------------------ |
| **app_ids** | **String** | appIds      |
| **from**    | **i64**    | from        |
| **to**      | **i64**    | to          |
| **metrics** | **bool**   | metrics     | [default to true]  |
| **logs**    | **bool**   | logs        | [default to true]  |
| **events**  | **bool**   | events      | [default to false] |
| **rum**     | **bool**   | rum         | [default to true]  |

### Return type

[**TagNamesResponse**](TagNamesResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_tags_grouped_by_id_using_get1**

> TagsGroupedResponse get_tags_grouped_by_id_using_get1(ctx, app_ids, tag, optional)
Gets tag names grouped by application id.

### Required Parameters

| Name         | Type                               | Description                           | Notes                    |
| ------------ | ---------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**      | **context.Context**                | context containing the authentication | nil if no authentication |
| **app_ids**  | **String**                         | appIds                                |
| **tag**      | [**Vec&lt;String&gt;**](String.md) | tag                                   |
| **optional** | **map[string]interface{}**         | optional parameters                   | nil if no parameters     |

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.
| Name        | Type                               | Description | Notes              |
| ----------- | ---------------------------------- | ----------- | ------------------ |
| **app_ids** | **String**                         | appIds      |
| **tag**     | [**Vec&lt;String&gt;**](String.md) | tag         |
| **from**    | **i64**                            | from        |
| **to**      | **i64**                            | to          |
| **metrics** | **bool**                           | metrics     | [default to true]  |
| **logs**    | **bool**                           | logs        | [default to true]  |
| **events**  | **bool**                           | events      | [default to false] |
| **rum**     | **bool**                           | rum         | [default to true]  |

### Return type

[**TagsGroupedResponse**](TagsGroupedResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_using_get**

> ::std::collections::HashMap<String, Dimension> get_using_get(ctx, app_ids, tag, optional)
Gets values for specified tags for the given application identifiers appearing in the given time frame.

### Required Parameters

| Name         | Type                               | Description                           | Notes                    |
| ------------ | ---------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**      | **context.Context**                | context containing the authentication | nil if no authentication |
| **app_ids**  | **String**                         | appIds                                |
| **tag**      | [**Vec&lt;String&gt;**](String.md) | tag                                   |
| **optional** | **map[string]interface{}**         | optional parameters                   | nil if no parameters     |

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.
| Name        | Type                               | Description | Notes              |
| ----------- | ---------------------------------- | ----------- | ------------------ |
| **app_ids** | **String**                         | appIds      |
| **tag**     | [**Vec&lt;String&gt;**](String.md) | tag         |
| **from**    | **i64**                            | from        |
| **to**      | **i64**                            | to          |
| **metrics** | **bool**                           | metrics     | [default to true]  |
| **logs**    | **bool**                           | logs        | [default to true]  |
| **events**  | **bool**                           | events      | [default to false] |
| **rum**     | **bool**                           | rum         | [default to true]  |

### Return type

[**::std::collections::HashMap<String, Dimension>**](Dimension.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_using_get1**

> ::std::collections::HashMap<String, Dimension> get_using_get1(ctx, app_ids, tag, optional)
Gets values for specified tags for the given application identifiers appearing in the given time frame.

### Required Parameters

| Name         | Type                               | Description                           | Notes                    |
| ------------ | ---------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**      | **context.Context**                | context containing the authentication | nil if no authentication |
| **app_ids**  | **String**                         | appIds                                |
| **tag**      | [**Vec&lt;String&gt;**](String.md) | tag                                   |
| **optional** | **map[string]interface{}**         | optional parameters                   | nil if no parameters     |

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.
| Name        | Type                               | Description | Notes              |
| ----------- | ---------------------------------- | ----------- | ------------------ |
| **app_ids** | **String**                         | appIds      |
| **tag**     | [**Vec&lt;String&gt;**](String.md) | tag         |
| **from**    | **i64**                            | from        |
| **to**      | **i64**                            | to          |
| **metrics** | **bool**                           | metrics     | [default to true]  |
| **logs**    | **bool**                           | logs        | [default to true]  |
| **events**  | **bool**                           | events      | [default to false] |
| **rum**     | **bool**                           | rum         | [default to true]  |

### Return type

[**::std::collections::HashMap<String, Dimension>**](Dimension.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
