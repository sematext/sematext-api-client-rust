# \AlertsApi

All URIs are relative to *https://localhost*

| Method                                                                                  | HTTP request                                                 | Description                |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------ | -------------------------- |
| [**create_alert_using_post**](AlertsApi.md#create_alert_using_post)                     | **Post** /users-web/api/v3/alerts                            | Create alert rule          |
| [**delete_alert_rule_using_delete**](AlertsApi.md#delete_alert_rule_using_delete)       | **Delete** /users-web/api/v3/alerts/{updateableAlertId}      | Delete alert rule          |
| [**disable_alert_rule_using_put**](AlertsApi.md#disable_alert_rule_using_put)           | **Put** /users-web/api/v3/alerts/{updateableAlertId}/disable | Disable alert rule         |
| [**enable_alert_rule_using_put**](AlertsApi.md#enable_alert_rule_using_put)             | **Put** /users-web/api/v3/alerts/{updateableAlertId}/enable  | Enable alert rule          |
| [**get_alert_rules_for_app_using_get**](AlertsApi.md#get_alert_rules_for_app_using_get) | **Get** /users-web/api/v3/apps/{appId}/alerts                | Get alert rules for an app |


# **create_alert_using_post**
> ::models::GenericApiResponse create_alert_using_post(ctx, dto)
Create alert rule

### Required Parameters

| Name    | Type                          | Description                           | Notes                    |
| ------- | ----------------------------- | ------------------------------------- | ------------------------ |
| **ctx** | **context.Context**           | context containing the authentication | nil if no authentication |
| **dto** | [**AlertRule**](AlertRule.md) | dto                                   |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_alert_rule_using_delete**
> ::models::GenericApiResponse delete_alert_rule_using_delete(ctx, updateable_alert_id)
Delete alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **disable_alert_rule_using_put**
> ::models::GenericApiResponse disable_alert_rule_using_put(ctx, updateable_alert_id)
Disable alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **enable_alert_rule_using_put**
> ::models::GenericApiResponse enable_alert_rule_using_put(ctx, updateable_alert_id)
Enable alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_alert_rules_for_app_using_get**
> ::models::GenericApiResponse get_alert_rules_for_app_using_get(ctx, app_id)
Get alert rules for an app

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
