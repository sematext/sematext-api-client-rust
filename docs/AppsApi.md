# {{classname}}

All URIs are relative to */*

| Method                                                                        | HTTP request                                              | Description                                                        |
| ----------------------------------------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------ |
| [**delete_using_delete**](AppsApi.md#delete_using_delete)                     | **DELETE** users-web/api/v3/apps/{anyStateAppId}          | delete                                                             |
| [**get_app_types_using_get**](AppsApi.md#get_app_types_using_get)             | **GET** users-web/api/v3/apps/types                       | Get all App types supported for the account identified with apiKey |
| [**get_using_get1**](AppsApi.md#get_using_get1)                               | **GET** users-web/api/v3/apps/{anyStateAppId}             | Gets defails for one particular App                                |
| [**invite_app_guests_using_post1**](AppsApi.md#invite_app_guests_using_post1) | **POST** users-web/api/v3/apps/guests                     | Invite guests to an app                                            |
| [**list_apps_users_using_get**](AppsApi.md#list_apps_users_using_get)         | **GET** users-web/api/v3/apps/users                       | Get all users of apps accessible to this account                   |
| [**list_using_get1**](AppsApi.md#list_using_get1)                             | **GET** users-web/api/v3/apps                             | Get all apps accessible by account identified with apiKey          |
| [**update_description_using_put1**](AppsApi.md#update_description_using_put1) | **PUT** users-web/api/v3/apps/{anyStateAppId}/description | Update description of the app                                      |
| [**update_using_put3**](AppsApi.md#update_using_put3)                         | **PUT** users-web/api/v3/apps/{anyStateAppId}             | Update app                                                         |

# **delete_using_delete**

> GenericMapBasedApiResponse delete_using_delete(ctx, any_state_app_id)
delete

### Required Parameters

| Name                 | Type                | Description                           | Notes                    |
| -------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**              | **context.Context** | context containing the authentication | nil if no authentication |
| **any_state_app_id** | **i64**             | anyStateAppId                         |

### Return type

[**GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_app_types_using_get**

> AppTypesResponse get_app_types_using_get(ctx, )
Get all App types supported for the account identified with apiKey

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**AppTypesResponse**](AppTypesResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_using_get1**

> AppResponse get_using_get1(ctx, any_state_app_id)
Gets defails for one particular App

### Required Parameters

| Name                 | Type                | Description                           | Notes                    |
| -------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**              | **context.Context** | context containing the authentication | nil if no authentication |
| **any_state_app_id** | **i64**             | anyStateAppId                         |

### Return type

[**AppResponse**](AppResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **invite_app_guests_using_post1**

> GenericMapBasedApiResponse invite_app_guests_using_post1(ctx, body)
Invite guests to an app

### Required Parameters

| Name     | Type                            | Description                                                                                                                                     | Notes                    |
| -------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| **ctx**  | **context.Context**             | context containing the authentication                                                                                                           | nil if no authentication |
| **body** | [**Invitation**](Invitation.md) | For &#x60;app&#x60; and &#x60;apps&#x60; fields only &#x60;id&#x60; needs to be populated.Other fields can be left empty or with default values |

### Return type

[**GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_apps_users_using_get**

> AppsResponse list_apps_users_using_get(ctx, )
Get all users of apps accessible to this account

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**AppsResponse**](AppsResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_using_get1**

> AppsResponse list_using_get1(ctx, )
Get all apps accessible by account identified with apiKey

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**AppsResponse**](AppsResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_description_using_put1**

> AppResponse update_description_using_put1(ctx, any_state_app_id, optional)
Update description of the app

App can be in any state

### Required Parameters

| Name                 | Type                       | Description                           | Notes                    |
| -------------------- | -------------------------- | ------------------------------------- | ------------------------ |
| **ctx**              | **context.Context**        | context containing the authentication | nil if no authentication |
| **any_state_app_id** | **i64**                    | App Id                                |
| **optional**         | **map[string]interface{}** | optional parameters                   | nil if no parameters     |

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.
| Name                 | Type                                    | Description    | Notes |
| -------------------- | --------------------------------------- | -------------- | ----- |
| **any_state_app_id** | **i64**                                 | App Id         |
| **body**             | [**AppDescription**](AppDescription.md) | Update Details |

### Return type

[**AppResponse**](AppResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_using_put3**

> AppResponse update_using_put3(ctx, body, any_state_app_id)
Update app

App can be in any state

### Required Parameters

| Name                 | Type                                  | Description                           | Notes                    |
| -------------------- | ------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**              | **context.Context**                   | context containing the authentication | nil if no authentication |
| **body**             | [**UpdateAppInfo**](UpdateAppInfo.md) | dto                                   |
| **any_state_app_id** | **i64**                               | App Id                                |

### Return type

[**AppResponse**](AppResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
