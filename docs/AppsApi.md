# \AppsApi

All URIs are relative to *http://localhost*

| Method                                                                      | HTTP request                                               | Description                                                        |
| --------------------------------------------------------------------------- | ---------------------------------------------------------- | ------------------------------------------------------------------ |
| [**delete_using_delete**](AppsApi.md#delete_using_delete)                   | **Delete** /users-web/api/v3/apps/{anyStateAppId}          | delete                                                             |
| [**get_app_types_using_get1**](AppsApi.md#get_app_types_using_get1)         | **Get** /users-web/api/v3/apps/types                       | Get all App types supported for the account identified with apiKey |
| [**get_using_get**](AppsApi.md#get_using_get)                               | **Get** /users-web/api/v3/apps/{anyStateAppId}             | Gets defails for one particular App                                |
| [**invite_app_guests_using_post**](AppsApi.md#invite_app_guests_using_post) | **Post** /users-web/api/v3/apps/guests                     | Invite guests to an app                                            |
| [**list_apps_users_using_get**](AppsApi.md#list_apps_users_using_get)       | **Get** /users-web/api/v3/apps/users                       | Get all users of apps accessible to this account                   |
| [**list_using_get1**](AppsApi.md#list_using_get1)                           | **Get** /users-web/api/v3/apps                             | Get all apps accessible by account identified with apiKey          |
| [**update_description_using_put**](AppsApi.md#update_description_using_put) | **Put** /users-web/api/v3/apps/{anyStateAppId}/description | Update description of the app                                      |
| [**update_using_put3**](AppsApi.md#update_using_put3)                       | **Put** /users-web/api/v3/apps/{anyStateAppId}             | Update app                                                         |



## delete_using_delete

> crate::models::GenericApiResponse delete_using_delete(any_state_app_id)
delete

### Parameters


| Name                 | Type    | Description   | Required   | Notes |
| -------------------- | ------- | ------------- | ---------- | ----- |
| **any_state_app_id** | **i64** | anyStateAppId | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_types_using_get1

> crate::models::GenericApiResponse get_app_types_using_get1()
Get all App types supported for the account identified with apiKey

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_using_get

> crate::models::GenericApiResponse get_using_get(any_state_app_id)
Gets defails for one particular App

### Parameters


| Name                 | Type    | Description   | Required   | Notes |
| -------------------- | ------- | ------------- | ---------- | ----- |
| **any_state_app_id** | **i64** | anyStateAppId | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_app_guests_using_post

> crate::models::GenericApiResponse invite_app_guests_using_post(invitation)
Invite guests to an app

### Parameters


| Name           | Type                            | Description                                                                                                       | Required   | Notes |
| -------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ---------- | ----- |
| **invitation** | [**Invitation**](Invitation.md) | For `app` and `apps` fields only `id` needs to be populated.Other fields can be left empty or with default values | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_apps_users_using_get

> crate::models::GenericApiResponse list_apps_users_using_get()
Get all users of apps accessible to this account

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_using_get1

> crate::models::GenericApiResponse list_using_get1()
Get all apps accessible by account identified with apiKey

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_description_using_put

> crate::models::GenericApiResponse update_description_using_put(any_state_app_id, update_details)
Update description of the app

App can be in any state

### Parameters


| Name                 | Type                                            | Description    | Required   | Notes |
| -------------------- | ----------------------------------------------- | -------------- | ---------- | ----- |
| **any_state_app_id** | **i64**                                         | App Id         | [required] |
| **update_details**   | Option<[**AppDescription**](AppDescription.md)> | Update Details |            |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_using_put3

> crate::models::GenericApiResponse update_using_put3(any_state_app_id, dto)
Update app

App can be in any state

### Parameters


| Name                 | Type                                  | Description | Required   | Notes |
| -------------------- | ------------------------------------- | ----------- | ---------- | ----- |
| **any_state_app_id** | **i64**                               | App Id      | [required] |
| **dto**              | [**UpdateAppInfo**](UpdateAppInfo.md) | dto         | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
