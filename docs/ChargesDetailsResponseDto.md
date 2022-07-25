# ChargesDetailsResponseDto

## Properties

| Name                   | Type                                                           | Description | Notes                        |
| ---------------------- | -------------------------------------------------------------- | ----------- | ---------------------------- |
| **app**                | [***::models::App**](App.md)                                   |             | [optional] [default to null] |
| **charge_base**        | **String**                                                     |             | [optional] [default to null] |
| **day_usage_data**     | [**Vec<::models::DayUsageData>**](DayUsageData.md)             |             | [optional] [default to null] |
| **monthly_fee_amount** | **f32**                                                        |             | [optional] [default to null] |
| **period_fee_periods** | [**Vec<::models::MinPeriodFeePeriod>**](MinPeriodFeePeriod.md) |             | [optional] [default to null] |
| **total_amount**       | **f32**                                                        |             | [optional] [default to null] |
| **usage_amount**       | **f32**                                                        |             | [optional] [default to null] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
