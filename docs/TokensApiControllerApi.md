# \TokensApiControllerApi

All URIs are relative to *http://localhost*

| Method                                                                     | HTTP request                                                        | Description                       |
| -------------------------------------------------------------------------- | ------------------------------------------------------------------- | --------------------------------- |
| [**create_app_token**](TokensApiControllerApi.md#create_app_token)         | **Post** /users-web/api/v3/apps/{appId}/tokens                      | Create new app token              |
| [**delete_app_token1**](TokensApiControllerApi.md#delete_app_token1)       | **Delete** /users-web/api/v3/apps/{appId}/tokens/{tokenId}          | Delete app token                  |
| [**get_app_tokens1**](TokensApiControllerApi.md#get_app_tokens1)           | **Get** /users-web/api/v3/apps/{appId}/tokens                       | Get app available tokens          |
| [**regenerate_app_token**](TokensApiControllerApi.md#regenerate_app_token) | **Post** /users-web/api/v3/apps/{appId}/tokens/{tokenId}/regenerate | Regenerate app token)             |
| [**update_app_token1**](TokensApiControllerApi.md#update_app_token1)       | **Put** /users-web/api/v3/apps/{appId}/tokens/{tokenId}             | Update app token (enable/disable) |



## create_app_token

> crate::models::GenericApiResponse create_app_token(app_id, dto)
Create new app token

### Parameters


| Name       | Type                                    | Description | Required   | Notes |
| ---------- | --------------------------------------- | ----------- | ---------- | ----- |
| **app_id** | **i64**                                 | appId       | [required] |
| **dto**    | [**CreateTokenDto**](CreateTokenDto.md) | dto         | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app_token1

> crate::models::GenericApiResponse delete_app_token1(app_id, token_id)
Delete app token

### Parameters


| Name         | Type    | Description | Required   | Notes |
| ------------ | ------- | ----------- | ---------- | ----- |
| **app_id**   | **i64** | appId       | [required] |
| **token_id** | **i64** | tokenId     | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_tokens1

> crate::models::GenericApiResponse get_app_tokens1(app_id)
Get app available tokens

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


## regenerate_app_token

> crate::models::GenericApiResponse regenerate_app_token(app_id, token_id)
Regenerate app token)

### Parameters


| Name         | Type    | Description | Required   | Notes |
| ------------ | ------- | ----------- | ---------- | ----- |
| **app_id**   | **i64** | appId       | [required] |
| **token_id** | **i64** | tokenId     | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app_token1

> crate::models::GenericApiResponse update_app_token1(app_id, token_id, dto)
Update app token (enable/disable)

### Parameters


| Name         | Type                                    | Description | Required   | Notes |
| ------------ | --------------------------------------- | ----------- | ---------- | ----- |
| **app_id**   | **i64**                                 | appId       | [required] |
| **token_id** | **i64**                                 | tokenId     | [required] |
| **dto**      | [**UpdateTokenDto**](UpdateTokenDto.md) | dto         | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
