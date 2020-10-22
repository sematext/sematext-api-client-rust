# \SavedQueriesApi

All URIs are relative to *http://localhost*

| Method                                                                                            | HTTP request                                                  | Description                  |
| ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- | ---------------------------- |
| [**delete_saved_query_using_delete**](SavedQueriesApi.md#delete_saved_query_using_delete)         | **Delete** /users-web/api/v3/savedQueries/{updateableQueryId} | Delete saved query           |
| [**get_saved_queries_for_app_using_get**](SavedQueriesApi.md#get_saved_queries_for_app_using_get) | **Get** /users-web/api/v3/apps/{appId}/savedQueries           | Get saved queries for an app |
| [**save_query_using_post**](SavedQueriesApi.md#save_query_using_post)                             | **Post** /users-web/api/v3/savedQueries                       | Create saved query           |
| [**save_query_using_put**](SavedQueriesApi.md#save_query_using_put)                               | **Put** /users-web/api/v3/savedQueries/{updateableQueryId}    | Update saved query           |



## delete_saved_query_using_delete

> crate::models::GenericApiResponse delete_saved_query_using_delete(updateable_query_id)
Delete saved query

### Parameters


| Name                    | Type    | Description       | Required   | Notes |
| ----------------------- | ------- | ----------------- | ---------- | ----- |
| **updateable_query_id** | **i64** | updateableQueryId | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saved_queries_for_app_using_get

> crate::models::GenericApiResponse get_saved_queries_for_app_using_get(app_id)
Get saved queries for an app

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


## save_query_using_post

> crate::models::GenericApiResponse save_query_using_post(saved_query_dto)
Create saved query

### Parameters


| Name                | Type                            | Description   | Required   | Notes |
| ------------------- | ------------------------------- | ------------- | ---------- | ----- |
| **saved_query_dto** | [**SavedQuery**](SavedQuery.md) | savedQueryDto | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_query_using_put

> crate::models::GenericApiResponse save_query_using_put(updateable_query_id, saved_query_dto)
Update saved query

### Parameters


| Name                    | Type                            | Description       | Required   | Notes |
| ----------------------- | ------------------------------- | ----------------- | ---------- | ----- |
| **updateable_query_id** | **i64**                         | updateableQueryId | [required] |
| **saved_query_dto**     | [**SavedQuery**](SavedQuery.md) | savedQueryDto     | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
