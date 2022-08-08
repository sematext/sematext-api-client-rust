# UsageDto

## Properties

| Name                     | Type                                                   | Description | Notes                        |
| ------------------------ | ------------------------------------------------------ | ----------- | ---------------------------- |
| **daily_usage**          | [**Vec<DailyDto>**](DailyDto.md)                       |             | [optional] [default to null] |
| **daily_volume_mb**      | **i64**                                                |             | [optional] [default to null] |
| **end**                  | **DateTime<Utc>**                                      |             | [optional] [default to null] |
| **failed_count**         | **i64**                                                |             | [optional] [default to null] |
| **ingested_count**       | **i64**                                                |             | [optional] [default to null] |
| **ingested_volume**      | **i64**                                                |             | [optional] [default to null] |
| **limit_change_events**  | [**Vec<LimitChangeEventDto>**](LimitChangeEventDTO.md) |             | [optional] [default to null] |
| **max_allowed_mb**       | **i64**                                                |             | [optional] [default to null] |
| **max_limit_mb**         | **i64**                                                |             | [optional] [default to null] |
| **start**                | **DateTime<Utc>**                                      |             | [optional] [default to null] |
| **stored_count**         | **i64**                                                |             | [optional] [default to null] |
| **stored_volume**        | **i64**                                                |             | [optional] [default to null] |
| **volume_change_events** | [**Vec<LimitChangeEventDto>**](LimitChangeEventDTO.md) |             | [optional] [default to null] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
