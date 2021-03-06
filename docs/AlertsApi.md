# \AlertsApi

All URIs are relative to *https://localhost*

| Method                                                                                  | HTTP request                                                 | Description                |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------ | -------------------------- |
| [**create_alert_using_post1**](AlertsApi.md#create_alert_using_post1)                   | **Post** /users-web/api/v3/alerts                            | Create alert rule          |
| [**delete_alert_rule_using_delete1**](AlertsApi.md#delete_alert_rule_using_delete1)     | **Delete** /users-web/api/v3/alerts/{updateableAlertId}      | Delete alert rule          |
| [**disable_alert_rule_using_put1**](AlertsApi.md#disable_alert_rule_using_put1)         | **Put** /users-web/api/v3/alerts/{updateableAlertId}/disable | Disable alert rule         |
| [**enable_alert_rule_using_put1**](AlertsApi.md#enable_alert_rule_using_put1)           | **Put** /users-web/api/v3/alerts/{updateableAlertId}/enable  | Enable alert rule          |
| [**get_alert_rules_for_app_using_get**](AlertsApi.md#get_alert_rules_for_app_using_get) | **Get** /users-web/api/v3/apps/{appId}/alerts                | Get alert rules for an app |


# **create_alert_using_post1**
> ::models::AlertRuleResponse create_alert_using_post1(ctx, dto)
Create alert rule

### Required Parameters

| Name    | Type                          | Description                           | Notes                    |
| ------- | ----------------------------- | ------------------------------------- | ------------------------ |
| **ctx** | **context.Context**           | context containing the authentication | nil if no authentication |
| **dto** | [**AlertRule**](AlertRule.md) | dto                                   |

### Return type

[**::models::AlertRuleResponse**](AlertRuleResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_alert_rule_using_delete1**
> ::models::GenericMapBasedApiResponse delete_alert_rule_using_delete1(ctx, updateable_alert_id)
Delete alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**::models::GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **disable_alert_rule_using_put1**
> ::models::GenericMapBasedApiResponse disable_alert_rule_using_put1(ctx, updateable_alert_id)
Disable alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**::models::GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **enable_alert_rule_using_put1**
> ::models::GenericMapBasedApiResponse enable_alert_rule_using_put1(ctx, updateable_alert_id)
Enable alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**::models::GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_alert_rules_for_app_using_get**
> ::models::AlertRulesResponse get_alert_rules_for_app_using_get(ctx, app_id)
Get alert rules for an app

### Required Parameters

| Name       | Type                | Description                           | Notes                    |
| ---------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context** | context containing the authentication | nil if no authentication |
| **app_id** | **i64**             | appId                                 |

### Return type

[**::models::AlertRulesResponse**](AlertRulesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
