# \SubscriptionsApi

All URIs are relative to *https://localhost*

| Method                                                                               | HTTP request                                                          | Description                             |
| ------------------------------------------------------------------------------------ | --------------------------------------------------------------------- | --------------------------------------- |
| [**create_for_app_using_post**](SubscriptionsApi.md#create_for_app_using_post)       | **Post** /users-web/api/v3/apps/{appId}/subscription                  | Create App subscription                 |
| [**create_for_dash_using_post1**](SubscriptionsApi.md#create_for_dash_using_post1)   | **Post** /users-web/api/v3/dashboards/{dashId}/subscription           | Create dashboard subscription           |
| [**delete_using_delete2**](SubscriptionsApi.md#delete_using_delete2)                 | **Delete** /users-web/api/v3/subscriptions/{updateableSubscriptionId} | Delete subscription                     |
| [**list_using_get2**](SubscriptionsApi.md#list_using_get2)                           | **Get** /users-web/api/v3/apps/{appId}/subscriptions                  | Get subscriptions for an App            |
| [**list_using_get5**](SubscriptionsApi.md#list_using_get5)                           | **Get** /users-web/api/v3/subscriptions                               | Get current account&#39;s subscriptions |
| [**send_app_report_using_post1**](SubscriptionsApi.md#send_app_report_using_post1)   | **Post** /users-web/api/v3/apps/{appId}/report/send                   | Email an App report                     |
| [**send_dash_report_using_post1**](SubscriptionsApi.md#send_dash_report_using_post1) | **Post** /users-web/api/v3/dashboards/{dashId}/report/send            | Email a dashboard report                |
| [**toggle_enabled_using_put1**](SubscriptionsApi.md#toggle_enabled_using_put1)       | **Put** /users-web/api/v3/subscriptions/{updateableSubscriptionId}    | Toggle subscription status              |
| [**update_for_app_using_put1**](SubscriptionsApi.md#update_for_app_using_put1)       | **Put** /users-web/api/v3/apps/{appId}/subscription                   | Update App subscription                 |
| [**update_for_dash_using_put**](SubscriptionsApi.md#update_for_dash_using_put)       | **Put** /users-web/api/v3/dashboards/{dashId}/subscription            | Update dashboard subscription           |


# **create_for_app_using_post**
> ::models::GenericApiResponse create_for_app_using_post(ctx, app_id, subscription)
Create App subscription

### Required Parameters

| Name             | Type                                      | Description                           | Notes                    |
| ---------------- | ----------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**          | **context.Context**                       | context containing the authentication | nil if no authentication |
| **app_id**       | **i64**                                   | appId                                 |
| **subscription** | [**SubscriptionDto**](SubscriptionDto.md) | subscription                          |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_for_dash_using_post1**
> ::models::GenericApiResponse create_for_dash_using_post1(ctx, dash_id, subscription)
Create dashboard subscription

### Required Parameters

| Name             | Type                                                        | Description                           | Notes                    |
| ---------------- | ----------------------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**          | **context.Context**                                         | context containing the authentication | nil if no authentication |
| **dash_id**      | **i64**                                                     | dashId                                |
| **subscription** | [**SubscriptionDashboardDto**](SubscriptionDashboardDto.md) | subscription                          |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_using_delete2**
> ::models::GenericApiResponse delete_using_delete2(ctx, updateable_subscription_id)
Delete subscription

### Required Parameters

| Name                           | Type                | Description                           | Notes                    |
| ------------------------------ | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                        | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_subscription_id** | **i64**             | updateableSubscriptionId              |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_using_get2**
> ::models::GenericApiResponse list_using_get2(ctx, app_id)
Get subscriptions for an App

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

# **list_using_get5**
> ::models::GenericApiResponse list_using_get5(ctx, )
Get current account's subscriptions

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **send_app_report_using_post1**
> ::models::GenericApiResponse send_app_report_using_post1(ctx, app_id, email_dto)
Email an App report

### Required Parameters

| Name          | Type                            | Description                           | Notes                    |
| ------------- | ------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**       | **context.Context**             | context containing the authentication | nil if no authentication |
| **app_id**    | **i64**                         | appId                                 |
| **email_dto** | [**ReportInfo**](ReportInfo.md) | emailDto                              |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **send_dash_report_using_post1**
> ::models::GenericApiResponse send_dash_report_using_post1(ctx, dash_id, email_dto)
Email a dashboard report

### Required Parameters

| Name          | Type                            | Description                           | Notes                    |
| ------------- | ------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**       | **context.Context**             | context containing the authentication | nil if no authentication |
| **dash_id**   | **i64**                         | dashId                                |
| **email_dto** | [**ReportInfo**](ReportInfo.md) | emailDto                              |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **toggle_enabled_using_put1**
> ::models::GenericApiResponse toggle_enabled_using_put1(ctx, updateable_subscription_id, dto)
Toggle subscription status

### Required Parameters

| Name                           | Type                                                  | Description                           | Notes                    |
| ------------------------------ | ----------------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**                        | **context.Context**                                   | context containing the authentication | nil if no authentication |
| **updateable_subscription_id** | **i64**                                               | updateableSubscriptionId              |
| **dto**                        | [**UpdateSubscriptionDto**](UpdateSubscriptionDto.md) | dto                                   |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_for_app_using_put1**
> ::models::GenericApiResponse update_for_app_using_put1(ctx, app_id, subscription)
Update App subscription

### Required Parameters

| Name             | Type                                      | Description                           | Notes                    |
| ---------------- | ----------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**          | **context.Context**                       | context containing the authentication | nil if no authentication |
| **app_id**       | **i64**                                   | appId                                 |
| **subscription** | [**SubscriptionDto**](SubscriptionDto.md) | subscription                          |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_for_dash_using_put**
> ::models::GenericApiResponse update_for_dash_using_put(ctx, dash_id, subscription)
Update dashboard subscription

### Required Parameters

| Name             | Type                                                        | Description                           | Notes                    |
| ---------------- | ----------------------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**          | **context.Context**                                         | context containing the authentication | nil if no authentication |
| **dash_id**      | **i64**                                                     | dashId                                |
| **subscription** | [**SubscriptionDashboardDto**](SubscriptionDashboardDto.md) | subscription                          |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
