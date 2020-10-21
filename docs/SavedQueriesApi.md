# \SavedQueriesApi

All URIs are relative to *https://localhost*

| Method                                                                                            | HTTP request                                                  | Description                  |
| ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- | ---------------------------- |
| [**delete_saved_query_using_delete**](SavedQueriesApi.md#delete_saved_query_using_delete)         | **Delete** /users-web/api/v3/savedQueries/{updateableQueryId} | Delete saved query           |
| [**get_saved_queries_for_app_using_get**](SavedQueriesApi.md#get_saved_queries_for_app_using_get) | **Get** /users-web/api/v3/apps/{appId}/savedQueries           | Get saved queries for an app |
| [**save_query_using_post**](SavedQueriesApi.md#save_query_using_post)                             | **Post** /users-web/api/v3/savedQueries                       | Create saved query           |
| [**save_query_using_put**](SavedQueriesApi.md#save_query_using_put)                               | **Put** /users-web/api/v3/savedQueries/{updateableQueryId}    | Update saved query           |


# **delete_saved_query_using_delete**
> ::models::GenericApiResponse delete_saved_query_using_delete(ctx, updateable_query_id)
Delete saved query

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_query_id** | **i64**             | updateableQueryId                     |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_saved_queries_for_app_using_get**
> ::models::GenericApiResponse get_saved_queries_for_app_using_get(ctx, app_id)
Get saved queries for an app

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

# **save_query_using_post**
> ::models::GenericApiResponse save_query_using_post(ctx, saved_query_dto)
Create saved query

### Required Parameters

| Name                | Type                            | Description                           | Notes                    |
| ------------------- | ------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**             | **context.Context**             | context containing the authentication | nil if no authentication |
| **saved_query_dto** | [**SavedQuery**](SavedQuery.md) | savedQueryDto                         |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **save_query_using_put**
> ::models::GenericApiResponse save_query_using_put(ctx, saved_query_dto, updateable_query_id)
Update saved query

### Required Parameters

| Name                    | Type                            | Description                           | Notes                    |
| ----------------------- | ------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context**             | context containing the authentication | nil if no authentication |
| **saved_query_dto**     | [**SavedQuery**](SavedQuery.md) | savedQueryDto                         |
| **updateable_query_id** | **i64**                         | updateableQueryId                     |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
