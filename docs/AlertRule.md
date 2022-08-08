# AlertRule

## Properties

| Name                                           | Type                                                                   | Description | Notes                        |
| ---------------------------------------------- | ---------------------------------------------------------------------- | ----------- | ---------------------------- |
| **account_email**                              | **String**                                                             |             | [optional] [default to null] |
| **allowed_app_types**                          | **Vec<i64>**                                                           |             | [optional] [default to null] |
| **analyzing_time**                             | **String**                                                             |             | [optional] [default to null] |
| **app_display_state**                          | **String**                                                             |             | [optional] [default to null] |
| **app_id**                                     | **i64**                                                                |             | [optional] [default to null] |
| **app_name**                                   | **String**                                                             |             | [optional] [default to null] |
| **app_state**                                  | **String**                                                             |             | [optional] [default to null] |
| **app_token**                                  | **String**                                                             |             | [optional] [default to null] |
| **app_type**                                   | **String**                                                             |             | [optional] [default to null] |
| **back_to_normal_needed**                      | **bool**                                                               |             | [optional] [default to null] |
| **chart_key**                                  | **String**                                                             |             | [optional] [default to null] |
| **color**                                      | **String**                                                             |             | [optional] [default to null] |
| **creator_email**                              | **String**                                                             |             | [optional] [default to null] |
| **default_agg_type**                           | **String**                                                             |             | [optional] [default to null] |
| **description**                                | **String**                                                             |             | [optional] [default to null] |
| **disallowed_app_types**                       | **Vec<i64>**                                                           |             | [optional] [default to null] |
| **enabled**                                    | **bool**                                                               |             | [optional] [default to null] |
| **estimate_operation**                         | **String**                                                             |             | [optional] [default to null] |
| **estimate_value**                             | **f64**                                                                |             | [optional] [default to null] |
| **filter_values**                              | **String**                                                             |             | [optional] [default to null] |
| **filter_values_obj**                          | [**Vec<FilterValue>**](FilterValue.md)                                 |             | [optional] [default to null] |
| **ignore_regular_events_enabled**              | **bool**                                                               |             | [optional] [default to null] |
| **integrations**                               | **String**                                                             |             | [optional] [default to null] |
| **last_data_received_date**                    | **i64**                                                                |             | [optional] [default to null] |
| **last_sent**                                  | **i64**                                                                |             | [optional] [default to null] |
| **last_triggered**                             | **i64**                                                                |             | [optional] [default to null] |
| **metadata**                                   | [***Value**](Value.md)                                                 |             | [optional] [default to null] |
| **metric_key**                                 | **String**                                                             |             | [optional] [default to null] |
| **metric_label**                               | **String**                                                             |             | [optional] [default to null] |
| **min_delay_between_notifications_in_minutes** | **String**                                                             |             | [optional] [default to null] |
| **name**                                       | **String**                                                             |             | [optional] [default to null] |
| **notification_emails**                        | **Vec<String>**                                                        |             | [optional] [default to null] |
| **notification_integrations**                  | [**Vec<NotificationIntegration>**](NotificationIntegration.md)         |             | [optional] [default to null] |
| **notifications_enabled**                      | **bool**                                                               |             | [optional] [default to null] |
| **priority**                                   | **String**                                                             |             | [optional] [default to null] |
| **query**                                      | **String**                                                             |             | [optional] [default to null] |
| **report_name**                                | **String**                                                             |             | [optional] [default to null] |
| **rule_key**                                   | **i64**                                                                |             | [optional] [default to null] |
| **rule_type**                                  | **String**                                                             |             | [optional] [default to null] |
| **runbook**                                    | **String**                                                             |             | [optional] [default to null] |
| **saved_query_id**                             | **i64**                                                                |             | [optional] [default to null] |
| **schedule**                                   | [**Vec<AlertRuleScheduleWeekdayDto>**](AlertRuleScheduleWeekdayDto.md) |             | [optional] [default to null] |
| **sematext_service**                           | **String**                                                             |             | [optional] [default to null] |
| **send_to_email**                              | **String**                                                             |             | [optional] [default to null] |
| **timezone**                                   | **String**                                                             |             | [optional] [default to null] |
| **use_only_alert_rule_integrations**           | **bool**                                                               |             | [optional] [default to null] |
| **user_permissions**                           | [***UserPermissions**](UserPermissions.md)                             |             | [optional] [default to null] |
| **value_column_name**                          | **String**                                                             |             | [optional] [default to null] |
| **value_name**                                 | **String**                                                             |             | [optional] [default to null] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
