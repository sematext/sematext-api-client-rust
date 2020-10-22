# \SubscriptionsApi

All URIs are relative to *http://localhost*

| Method                                                                               | HTTP request                                                          | Description                         |
| ------------------------------------------------------------------------------------ | --------------------------------------------------------------------- | ----------------------------------- |
| [**create_for_app_using_post**](SubscriptionsApi.md#create_for_app_using_post)       | **Post** /users-web/api/v3/apps/{appId}/subscription                  | Create App subscription             |
| [**create_for_dash_using_post1**](SubscriptionsApi.md#create_for_dash_using_post1)   | **Post** /users-web/api/v3/dashboards/{dashId}/subscription           | Create dashboard subscription       |
| [**delete_using_delete2**](SubscriptionsApi.md#delete_using_delete2)                 | **Delete** /users-web/api/v3/subscriptions/{updateableSubscriptionId} | Delete subscription                 |
| [**list_using_get2**](SubscriptionsApi.md#list_using_get2)                           | **Get** /users-web/api/v3/apps/{appId}/subscriptions                  | Get subscriptions for an App        |
| [**list_using_get5**](SubscriptionsApi.md#list_using_get5)                           | **Get** /users-web/api/v3/subscriptions                               | Get current account's subscriptions |
| [**send_app_report_using_post1**](SubscriptionsApi.md#send_app_report_using_post1)   | **Post** /users-web/api/v3/apps/{appId}/report/send                   | Email an App report                 |
| [**send_dash_report_using_post1**](SubscriptionsApi.md#send_dash_report_using_post1) | **Post** /users-web/api/v3/dashboards/{dashId}/report/send            | Email a dashboard report            |
| [**toggle_enabled_using_put1**](SubscriptionsApi.md#toggle_enabled_using_put1)       | **Put** /users-web/api/v3/subscriptions/{updateableSubscriptionId}    | Toggle subscription status          |
| [**update_for_app_using_put1**](SubscriptionsApi.md#update_for_app_using_put1)       | **Put** /users-web/api/v3/apps/{appId}/subscription                   | Update App subscription             |
| [**update_for_dash_using_put**](SubscriptionsApi.md#update_for_dash_using_put)       | **Put** /users-web/api/v3/dashboards/{dashId}/subscription            | Update dashboard subscription       |



## create_for_app_using_post

> crate::models::GenericApiResponse create_for_app_using_post(app_id, subscription)
Create App subscription

### Parameters


| Name             | Type                                      | Description  | Required   | Notes |
| ---------------- | ----------------------------------------- | ------------ | ---------- | ----- |
| **app_id**       | **i64**                                   | appId        | [required] |
| **subscription** | [**SubscriptionDto**](SubscriptionDto.md) | subscription | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_for_dash_using_post1

> crate::models::GenericApiResponse create_for_dash_using_post1(dash_id, subscription)
Create dashboard subscription

### Parameters


| Name             | Type                                                        | Description  | Required   | Notes |
| ---------------- | ----------------------------------------------------------- | ------------ | ---------- | ----- |
| **dash_id**      | **i64**                                                     | dashId       | [required] |
| **subscription** | [**SubscriptionDashboardDto**](SubscriptionDashboardDto.md) | subscription | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_using_delete2

> crate::models::GenericApiResponse delete_using_delete2(updateable_subscription_id)
Delete subscription

### Parameters


| Name                           | Type    | Description              | Required   | Notes |
| ------------------------------ | ------- | ------------------------ | ---------- | ----- |
| **updateable_subscription_id** | **i64** | updateableSubscriptionId | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_using_get2

> crate::models::GenericApiResponse list_using_get2(app_id)
Get subscriptions for an App

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


## list_using_get5

> crate::models::GenericApiResponse list_using_get5()
Get current account's subscriptions

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_app_report_using_post1

> crate::models::GenericApiResponse send_app_report_using_post1(app_id, email_dto)
Email an App report

### Parameters


| Name          | Type                            | Description | Required   | Notes |
| ------------- | ------------------------------- | ----------- | ---------- | ----- |
| **app_id**    | **i64**                         | appId       | [required] |
| **email_dto** | [**ReportInfo**](ReportInfo.md) | emailDto    | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_dash_report_using_post1

> crate::models::GenericApiResponse send_dash_report_using_post1(dash_id, email_dto)
Email a dashboard report

### Parameters


| Name          | Type                            | Description | Required   | Notes |
| ------------- | ------------------------------- | ----------- | ---------- | ----- |
| **dash_id**   | **i64**                         | dashId      | [required] |
| **email_dto** | [**ReportInfo**](ReportInfo.md) | emailDto    | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_enabled_using_put1

> crate::models::GenericApiResponse toggle_enabled_using_put1(updateable_subscription_id, dto)
Toggle subscription status

### Parameters


| Name                           | Type                                                  | Description              | Required   | Notes |
| ------------------------------ | ----------------------------------------------------- | ------------------------ | ---------- | ----- |
| **updateable_subscription_id** | **i64**                                               | updateableSubscriptionId | [required] |
| **dto**                        | [**UpdateSubscriptionDto**](UpdateSubscriptionDto.md) | dto                      | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_for_app_using_put1

> crate::models::GenericApiResponse update_for_app_using_put1(app_id, subscription)
Update App subscription

### Parameters


| Name             | Type                                      | Description  | Required   | Notes |
| ---------------- | ----------------------------------------- | ------------ | ---------- | ----- |
| **app_id**       | **i64**                                   | appId        | [required] |
| **subscription** | [**SubscriptionDto**](SubscriptionDto.md) | subscription | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_for_dash_using_put

> crate::models::GenericApiResponse update_for_dash_using_put(dash_id, subscription)
Update dashboard subscription

### Parameters


| Name             | Type                                                        | Description  | Required   | Notes |
| ---------------- | ----------------------------------------------------------- | ------------ | ---------- | ----- |
| **dash_id**      | **i64**                                                     | dashId       | [required] |
| **subscription** | [**SubscriptionDashboardDto**](SubscriptionDashboardDto.md) | subscription | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
