# AlertRule

## Properties

| Name                                           | Type                                                                                          | Description | Notes                |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------- | ----------- | -------------------- |
| **account_email**                              | Option<**String**>                                                                            |             | [optional][readonly] |
| **allowed_app_types**                          | Option<**Vec<i64>**>                                                                          |             | [optional]           |
| **analyzing_time**                             | Option<**String**>                                                                            |             | [optional]           |
| **app_display_state**                          | Option<**String**>                                                                            |             | [optional][readonly] |
| **app_id**                                     | Option<**i64**>                                                                               |             | [optional]           |
| **app_name**                                   | Option<**String**>                                                                            |             | [optional][readonly] |
| **app_state**                                  | Option<**String**>                                                                            |             | [optional][readonly] |
| **app_token**                                  | Option<**String**>                                                                            |             | [optional][readonly] |
| **app_type**                                   | Option<**String**>                                                                            |             | [optional][readonly] |
| **back_to_normal_needed**                      | Option<**bool**>                                                                              |             | [optional]           |
| **chart_key**                                  | Option<**String**>                                                                            |             | [optional]           |
| **color**                                      | Option<**String**>                                                                            |             | [optional]           |
| **creator_email**                              | Option<**String**>                                                                            |             | [optional][readonly] |
| **default_agg_type**                           | Option<**String**>                                                                            |             | [optional][readonly] |
| **description**                                | Option<**String**>                                                                            |             | [optional]           |
| **disallowed_app_types**                       | Option<**Vec<i64>**>                                                                          |             | [optional]           |
| **enabled**                                    | Option<**bool**>                                                                              |             | [optional]           |
| **estimate_operation**                         | Option<**String**>                                                                            |             | [optional]           |
| **estimate_value**                             | Option<**f64**>                                                                               |             | [optional]           |
| **filter_values**                              | Option<**String**>                                                                            |             | [optional]           |
| **filter_values_obj**                          | Option<[**Vec<crate::models::FilterValue>**](FilterValue.md)>                                 |             | [optional]           |
| **ignore_regular_events_enabled**              | Option<**bool**>                                                                              |             | [optional]           |
| **integrations**                               | Option<**String**>                                                                            |             | [optional][readonly] |
| **last_data_received_date**                    | Option<**i64**>                                                                               |             | [optional][readonly] |
| **last_sent**                                  | Option<**i64**>                                                                               |             | [optional][readonly] |
| **last_triggered**                             | Option<**i64**>                                                                               |             | [optional][readonly] |
| **metadata**                                   | Option<[**serde_json::Value**](.md)>                                                          |             | [optional]           |
| **metric_key**                                 | Option<**String**>                                                                            |             | [optional][readonly] |
| **metric_label**                               | Option<**String**>                                                                            |             | [optional]           |
| **min_delay_between_notifications_in_minutes** | Option<**String**>                                                                            |             | [optional]           |
| **name**                                       | Option<**String**>                                                                            |             | [optional]           |
| **notification_emails**                        | Option<**Vec<String>**>                                                                       |             | [optional]           |
| **notification_integrations**                  | Option<[**Vec<crate::models::NotificationIntegration>**](NotificationIntegration.md)>         |             | [optional]           |
| **notifications_enabled**                      | Option<**bool**>                                                                              |             | [optional]           |
| **query**                                      | Option<**String**>                                                                            |             | [optional]           |
| **report_name**                                | Option<**String**>                                                                            |             | [optional]           |
| **rule_key**                                   | Option<**i64**>                                                                               |             | [optional][readonly] |
| **rule_type**                                  | Option<**String**>                                                                            |             | [optional]           |
| **runbook**                                    | Option<**String**>                                                                            |             | [optional]           |
| **saved_query_id**                             | Option<**i64**>                                                                               |             | [optional][readonly] |
| **schedule**                                   | Option<[**Vec<crate::models::AlertRuleScheduleWeekdayDto>**](AlertRuleScheduleWeekdayDto.md)> |             | [optional]           |
| **sematext_service**                           | Option<**String**>                                                                            |             | [optional][readonly] |
| **send_to_email**                              | Option<**String**>                                                                            |             | [optional]           |
| **timezone**                                   | Option<**String**>                                                                            |             | [optional]           |
| **use_only_alert_rule_integrations**           | Option<**bool**>                                                                              |             | [optional]           |
| **user_permissions**                           | Option<[**crate::models::UserPermissions**](UserPermissions.md)>                              |             | [optional]           |
| **value_column_name**                          | Option<**String**>                                                                            |             | [optional][readonly] |
| **value_name**                                 | Option<**String**>                                                                            |             | [optional][readonly] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
