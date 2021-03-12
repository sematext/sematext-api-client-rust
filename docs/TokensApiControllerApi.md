# \TokensApiControllerApi

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_app_token**](TokensApiControllerApi.md#create_app_token) | **Post** /users-web/api/v3/apps/{appId}/tokens | Create new app token
[**delete_app_token1**](TokensApiControllerApi.md#delete_app_token1) | **Delete** /users-web/api/v3/apps/{appId}/tokens/{tokenId} | Delete app token
[**get_app_tokens1**](TokensApiControllerApi.md#get_app_tokens1) | **Get** /users-web/api/v3/apps/{appId}/tokens | Get app available tokens
[**regenerate_app_token1**](TokensApiControllerApi.md#regenerate_app_token1) | **Post** /users-web/api/v3/apps/{appId}/tokens/{tokenId}/regenerate | Regenerate app token)
[**update_app_token**](TokensApiControllerApi.md#update_app_token) | **Put** /users-web/api/v3/apps/{appId}/tokens/{tokenId} | Update app token (enable/disable or name)


# **create_app_token**
> ::models::TokenResponse create_app_token(ctx, app_id, dto)
Create new app token

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **i64**| appId | 
  **dto** | [**CreateTokenDto**](CreateTokenDto.md)| dto | 

### Return type

[**::models::TokenResponse**](TokenResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_app_token1**
> ::models::GenericMapBasedApiResponse delete_app_token1(ctx, app_id, token_id)
Delete app token

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **i64**| appId | 
  **token_id** | **i64**| tokenId | 

### Return type

[**::models::GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_app_tokens1**
> ::models::TokensResponse get_app_tokens1(ctx, app_id)
Get app available tokens

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **i64**| appId | 

### Return type

[**::models::TokensResponse**](TokensResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **regenerate_app_token1**
> ::models::TokenResponse regenerate_app_token1(ctx, app_id, token_id)
Regenerate app token)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **i64**| appId | 
  **token_id** | **i64**| tokenId | 

### Return type

[**::models::TokenResponse**](TokenResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_app_token**
> ::models::TokenResponse update_app_token(ctx, app_id, token_id, dto)
Update app token (enable/disable or name)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **i64**| appId | 
  **token_id** | **i64**| tokenId | 
  **dto** | [**UpdateTokenDto**](UpdateTokenDto.md)| dto | 

### Return type

[**::models::TokenResponse**](TokenResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

