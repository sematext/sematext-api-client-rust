# \ResetPasswordApi

All URIs are relative to *http://localhost*

| Method                                                                           | HTTP request                                      | Description    |
| -------------------------------------------------------------------------------- | ------------------------------------------------- | -------------- |
| [**reset_password_using_post1**](ResetPasswordApi.md#reset_password_using_post1) | **Post** /users-web/api/v3/account/password/reset | Reset Password |



## reset_password_using_post1

> crate::models::GenericApiResponse reset_password_using_post1(dto)
Reset Password

### Parameters


| Name    | Type                        | Description | Required   | Notes |
| ------- | --------------------------- | ----------- | ---------- | ----- |
| **dto** | [**UserInfo**](UserInfo.md) | dto         | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
