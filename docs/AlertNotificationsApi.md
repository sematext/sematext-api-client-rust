# \AlertNotificationsApi

All URIs are relative to *http://localhost*

| Method                                                                                                                  | HTTP request                                                 | Description                        |
| ----------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ | ---------------------------------- |
| [**get_alert_notifications_for_app_using_post1**](AlertNotificationsApi.md#get_alert_notifications_for_app_using_post1) | **Post** /users-web/api/v3/apps/{appId}/notifications/alerts | Get alert notifications for an app |
| [**get_alert_notifications_for_user_using_post**](AlertNotificationsApi.md#get_alert_notifications_for_user_using_post) | **Post** /users-web/api/v3/notifications/alerts              | Get alert notifications for a user |



## get_alert_notifications_for_app_using_post1

> crate::models::GenericApiResponse get_alert_notifications_for_app_using_post1(app_id, time_interval)
Get alert notifications for an app

Default value of interval is 1d

### Parameters


| Name              | Type                                                        | Description   | Required   | Notes |
| ----------------- | ----------------------------------------------------------- | ------------- | ---------- | ----- |
| **app_id**        | **i64**                                                     | appId         | [required] |
| **time_interval** | [**AlertNotificationRequest**](AlertNotificationRequest.md) | Time Interval | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert_notifications_for_user_using_post

> crate::models::GenericApiResponse get_alert_notifications_for_user_using_post(time_interval)
Get alert notifications for a user

Default value of interval is 1d

### Parameters


| Name              | Type                                                        | Description   | Required   | Notes |
| ----------------- | ----------------------------------------------------------- | ------------- | ---------- | ----- |
| **time_interval** | [**AlertNotificationRequest**](AlertNotificationRequest.md) | Time Interval | [required] |

### Return type

[**crate::models::GenericApiResponse**](Generic Api Response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
