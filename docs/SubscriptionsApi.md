# {{classname}}

All URIs are relative to */*

| Method                                                                               | HTTP request                                                         | Description                              |
| ------------------------------------------------------------------------------------ | -------------------------------------------------------------------- | ---------------------------------------- |
| [**create_for_app_using_post**](SubscriptionsApi.md#create_for_app_using_post)       | **POST** users-web/api/v3/apps/{appId}/subscription                  | Create App subscription                  |
| [**create_for_dash_using_post1**](SubscriptionsApi.md#create_for_dash_using_post1)   | **POST** users-web/api/v3/dashboards/{dashId}/subscription           | Create dashboard subscription            |
| [**delete_using_delete3**](SubscriptionsApi.md#delete_using_delete3)                 | **DELETE** users-web/api/v3/subscriptions/{updateableSubscriptionId} | Delete subscription                      |
| [**list_using_get2**](SubscriptionsApi.md#list_using_get2)                           | **GET** users-web/api/v3/apps/{appId}/subscriptions                  | Get subscriptions for an App             |
| [**list_using_get5**](SubscriptionsApi.md#list_using_get5)                           | **GET** users-web/api/v3/subscriptions                               | Get current account&#x27;s subscriptions |
| [**send_app_report_using_post1**](SubscriptionsApi.md#send_app_report_using_post1)   | **POST** users-web/api/v3/apps/{appId}/report/send                   | Email an App report                      |
| [**send_dash_report_using_post1**](SubscriptionsApi.md#send_dash_report_using_post1) | **POST** users-web/api/v3/dashboards/{dashId}/report/send            | Email a dashboard report                 |
| [**toggle_enabled_using_put**](SubscriptionsApi.md#toggle_enabled_using_put)         | **PUT** users-web/api/v3/subscriptions/{updateableSubscriptionId}    | Toggle subscription status               |
| [**update_for_app_using_put1**](SubscriptionsApi.md#update_for_app_using_put1)       | **PUT** users-web/api/v3/apps/{appId}/subscription                   | Update App subscription                  |
| [**update_for_dash_using_put**](SubscriptionsApi.md#update_for_dash_using_put)       | **PUT** users-web/api/v3/dashboards/{dashId}/subscription            | Update dashboard subscription            |

# **create_for_app_using_post**

> SubscriptionResponse create_for_app_using_post(ctx, body, app_id)
Create App subscription

### Required Parameters

| Name       | Type                                      | Description                           | Notes                    |
| ---------- | ----------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context**                       | context containing the authentication | nil if no authentication |
| **body**   | [**SubscriptionDto**](SubscriptionDto.md) | subscription                          |
| **app_id** | **i64**                                   | appId                                 |

### Return type

[**SubscriptionResponse**](SubscriptionResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_for_dash_using_post1**

> SubscriptionResponse create_for_dash_using_post1(ctx, body, dash_id)
Create dashboard subscription

### Required Parameters

| Name        | Type                                                        | Description                           | Notes                    |
| ----------- | ----------------------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**     | **context.Context**                                         | context containing the authentication | nil if no authentication |
| **body**    | [**SubscriptionDashboardDto**](SubscriptionDashboardDto.md) | subscription                          |
| **dash_id** | **i64**                                                     | dashId                                |

### Return type

[**SubscriptionResponse**](SubscriptionResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_using_delete3**

> GenericMapBasedApiResponse delete_using_delete3(ctx, updateable_subscription_id)
Delete subscription

### Required Parameters

| Name                           | Type                | Description                           | Notes                    |
| ------------------------------ | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                        | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_subscription_id** | **i64**             | updateableSubscriptionId              |

### Return type

[**GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_using_get2**

> SubscriptionsResponse list_using_get2(ctx, app_id)
Get subscriptions for an App

### Required Parameters

| Name       | Type                | Description                           | Notes                    |
| ---------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context** | context containing the authentication | nil if no authentication |
| **app_id** | **i64**             | appId                                 |

### Return type

[**SubscriptionsResponse**](SubscriptionsResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_using_get5**

> SubscriptionsResponse list_using_get5(ctx, )
Get current account's subscriptions

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**SubscriptionsResponse**](SubscriptionsResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **send_app_report_using_post1**

> MailReportResponse send_app_report_using_post1(ctx, body, app_id)
Email an App report

### Required Parameters

| Name       | Type                            | Description                           | Notes                    |
| ---------- | ------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context**             | context containing the authentication | nil if no authentication |
| **body**   | [**ReportInfo**](ReportInfo.md) | emailDto                              |
| **app_id** | **i64**                         | appId                                 |

### Return type

[**MailReportResponse**](MailReportResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **send_dash_report_using_post1**

> MailReportResponse send_dash_report_using_post1(ctx, body, dash_id)
Email a dashboard report

### Required Parameters

| Name        | Type                            | Description                           | Notes                    |
| ----------- | ------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**     | **context.Context**             | context containing the authentication | nil if no authentication |
| **body**    | [**ReportInfo**](ReportInfo.md) | emailDto                              |
| **dash_id** | **i64**                         | dashId                                |

### Return type

[**MailReportResponse**](MailReportResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **toggle_enabled_using_put**

> SubscriptionResponse toggle_enabled_using_put(ctx, body, updateable_subscription_id)
Toggle subscription status

### Required Parameters

| Name                           | Type                                                  | Description                           | Notes                    |
| ------------------------------ | ----------------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**                        | **context.Context**                                   | context containing the authentication | nil if no authentication |
| **body**                       | [**UpdateSubscriptionDto**](UpdateSubscriptionDto.md) | dto                                   |
| **updateable_subscription_id** | **i64**                                               | updateableSubscriptionId              |

### Return type

[**SubscriptionResponse**](SubscriptionResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_for_app_using_put1**

> SubscriptionResponse update_for_app_using_put1(ctx, body, app_id)
Update App subscription

### Required Parameters

| Name       | Type                                      | Description                           | Notes                    |
| ---------- | ----------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context**                       | context containing the authentication | nil if no authentication |
| **body**   | [**SubscriptionDto**](SubscriptionDto.md) | subscription                          |
| **app_id** | **i64**                                   | appId                                 |

### Return type

[**SubscriptionResponse**](SubscriptionResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_for_dash_using_put**

> SubscriptionResponse update_for_dash_using_put(ctx, body, dash_id)
Update dashboard subscription

### Required Parameters

| Name        | Type                                                        | Description                           | Notes                    |
| ----------- | ----------------------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**     | **context.Context**                                         | context containing the authentication | nil if no authentication |
| **body**    | [**SubscriptionDashboardDto**](SubscriptionDashboardDto.md) | subscription                          |
| **dash_id** | **i64**                                                     | dashId                                |

### Return type

[**SubscriptionResponse**](SubscriptionResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
