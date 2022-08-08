# {{classname}}

All URIs are relative to */*

| Method                                                                                  | HTTP request                                                | Description                |
| --------------------------------------------------------------------------------------- | ----------------------------------------------------------- | -------------------------- |
| [**create_alert_using_post**](AlertsApi.md#create_alert_using_post)                     | **POST** users-web/api/v3/alerts                            | Create alert rule          |
| [**delete_alert_rule_using_delete**](AlertsApi.md#delete_alert_rule_using_delete)       | **DELETE** users-web/api/v3/alerts/{updateableAlertId}      | Delete alert rule          |
| [**disable_alert_rule_using_put**](AlertsApi.md#disable_alert_rule_using_put)           | **PUT** users-web/api/v3/alerts/{updateableAlertId}/disable | Disable alert rule         |
| [**enable_alert_rule_using_put**](AlertsApi.md#enable_alert_rule_using_put)             | **PUT** users-web/api/v3/alerts/{updateableAlertId}/enable  | Enable alert rule          |
| [**get_alert_rules_for_app_using_get**](AlertsApi.md#get_alert_rules_for_app_using_get) | **GET** users-web/api/v3/apps/{appId}/alerts                | Get alert rules for an app |

# **create_alert_using_post**

> AlertRuleResponse create_alert_using_post(ctx, body)
Create alert rule

### Required Parameters

| Name     | Type                          | Description                           | Notes                    |
| -------- | ----------------------------- | ------------------------------------- | ------------------------ |
| **ctx**  | **context.Context**           | context containing the authentication | nil if no authentication |
| **body** | [**AlertRule**](AlertRule.md) | dto                                   |

### Return type

[**AlertRuleResponse**](AlertRuleResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_alert_rule_using_delete**

> GenericMapBasedApiResponse delete_alert_rule_using_delete(ctx, updateable_alert_id)
Delete alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **disable_alert_rule_using_put**

> GenericMapBasedApiResponse disable_alert_rule_using_put(ctx, updateable_alert_id)
Disable alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **enable_alert_rule_using_put**

> GenericMapBasedApiResponse enable_alert_rule_using_put(ctx, updateable_alert_id)
Enable alert rule

### Required Parameters

| Name                    | Type                | Description                           | Notes                    |
| ----------------------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**                 | **context.Context** | context containing the authentication | nil if no authentication |
| **updateable_alert_id** | **i64**             | updateableAlertId                     |

### Return type

[**GenericMapBasedApiResponse**](Generic Map Based Api Response.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_alert_rules_for_app_using_get**

> AlertRulesResponse get_alert_rules_for_app_using_get(ctx, app_id)
Get alert rules for an app

### Required Parameters

| Name       | Type                | Description                           | Notes                    |
| ---------- | ------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context** | context containing the authentication | nil if no authentication |
| **app_id** | **i64**             | appId                                 |

### Return type

[**AlertRulesResponse**](AlertRulesResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
