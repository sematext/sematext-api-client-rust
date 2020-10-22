# \AlertsApi

All URIs are relative to *http://localhost*

| Method                                                                                    | HTTP request                                                 | Description                |
| ----------------------------------------------------------------------------------------- | ------------------------------------------------------------ | -------------------------- |
| [**create_alert_using_post1**](AlertsApi.md#create_alert_using_post1)                     | **Post** /users-web/api/v3/alerts                            | Create alert rule          |
| [**delete_alert_rule_using_delete1**](AlertsApi.md#delete_alert_rule_using_delete1)       | **Delete** /users-web/api/v3/alerts/{updateableAlertId}      | Delete alert rule          |
| [**disable_alert_rule_using_put**](AlertsApi.md#disable_alert_rule_using_put)             | **Put** /users-web/api/v3/alerts/{updateableAlertId}/disable | Disable alert rule         |
| [**enable_alert_rule_using_put1**](AlertsApi.md#enable_alert_rule_using_put1)             | **Put** /users-web/api/v3/alerts/{updateableAlertId}/enable  | Enable alert rule          |
| [**get_alert_rules_for_app_using_get1**](AlertsApi.md#get_alert_rules_for_app_using_get1) | **Get** /users-web/api/v3/apps/{appId}/alerts                | Get alert rules for an app |



## create_alert_using_post1

> crate::models::GenericApiResponse create_alert_using_post1(dto)
Create alert rule

### Parameters


| Name    | Type                          | Description | Required   | Notes |
| ------- | ----------------------------- | ----------- | ---------- | ----- |
| **dto** | [**AlertRule**](AlertRule.md) | dto         | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert_rule_using_delete1

> crate::models::GenericApiResponse delete_alert_rule_using_delete1(updateable_alert_id)
Delete alert rule

### Parameters


| Name                    | Type    | Description       | Required   | Notes |
| ----------------------- | ------- | ----------------- | ---------- | ----- |
| **updateable_alert_id** | **i64** | updateableAlertId | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_alert_rule_using_put

> crate::models::GenericApiResponse disable_alert_rule_using_put(updateable_alert_id)
Disable alert rule

### Parameters


| Name                    | Type    | Description       | Required   | Notes |
| ----------------------- | ------- | ----------------- | ---------- | ----- |
| **updateable_alert_id** | **i64** | updateableAlertId | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_alert_rule_using_put1

> crate::models::GenericApiResponse enable_alert_rule_using_put1(updateable_alert_id)
Enable alert rule

### Parameters


| Name                    | Type    | Description       | Required   | Notes |
| ----------------------- | ------- | ----------------- | ---------- | ----- |
| **updateable_alert_id** | **i64** | updateableAlertId | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert_rules_for_app_using_get1

> crate::models::GenericApiResponse get_alert_rules_for_app_using_get1(app_id)
Get alert rules for an app

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
