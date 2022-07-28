# \ResetPasswordApi

All URIs are relative to *<https://localhost>*

| Method                                                                           | HTTP request                                      | Description    |
| -------------------------------------------------------------------------------- | ------------------------------------------------- | -------------- |
| [**reset_password_using_post1**](ResetPasswordApi.md#reset_password_using_post1) | **Post** /users-web/api/v3/account/password/reset | Reset Password |

# **reset_password_using_post1**

> ::models::GenericMapBasedApiResponse reset_password_using_post1(ctx, dto)
Reset Password

### Required Parameters

| Name    | Type                        | Description                           | Notes                    |
| ------- | --------------------------- | ------------------------------------- | ------------------------ |
| **ctx** | **context.Context**         | context containing the authentication | nil if no authentication |
| **dto** | [**UserInfo**](UserInfo.md) | dto                                   |

### Return type

[**::models::GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
