# {{classname}}

All URIs are relative to */*

| Method                                                                                                                    | HTTP request                                                | Description                        |
| ------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- | ---------------------------------- |
| [**get_alert_notifications_for_app_using_post1**](AlertNotificationsApi.md#get_alert_notifications_for_app_using_post1)   | **POST** users-web/api/v3/apps/{appId}/notifications/alerts | Get alert notifications for an app |
| [**get_alert_notifications_for_user_using_post1**](AlertNotificationsApi.md#get_alert_notifications_for_user_using_post1) | **POST** users-web/api/v3/notifications/alerts              | Get alert notifications for a user |

# **get_alert_notifications_for_app_using_post1**

> NotificationsResponse get_alert_notifications_for_app_using_post1(ctx, body, app_id)
Get alert notifications for an app

Default value of interval is 1d

### Required Parameters

| Name       | Type                                                        | Description                           | Notes                    |
| ---------- | ----------------------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**    | **context.Context**                                         | context containing the authentication | nil if no authentication |
| **body**   | [**AlertNotificationRequest**](AlertNotificationRequest.md) | Time Interval                         |
| **app_id** | **i64**                                                     | appId                                 |

### Return type

[**NotificationsResponse**](NotificationsResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_alert_notifications_for_user_using_post1**

> NotificationsResponse get_alert_notifications_for_user_using_post1(ctx, body)
Get alert notifications for a user

Default value of interval is 1d

### Required Parameters

| Name     | Type                                                        | Description                           | Notes                    |
| -------- | ----------------------------------------------------------- | ------------------------------------- | ------------------------ |
| **ctx**  | **context.Context**                                         | context containing the authentication | nil if no authentication |
| **body** | [**AlertNotificationRequest**](AlertNotificationRequest.md) | Time Interval                         |

### Return type

[**NotificationsResponse**](NotificationsResponse.md)

### Authorization

[api_key](../README.md#api_key),

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
