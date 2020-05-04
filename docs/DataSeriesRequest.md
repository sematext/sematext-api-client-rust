# DataSeriesRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_interval** | **i64** |  | [optional] [default to null]
**end** | **String** | End time of interval. Can be expressed as timestamp in milliseconds or UTC date in yyyy-MM-dd HH:mm:ss format | [optional] [default to null]
**filters** | [**::std::collections::HashMap<String, ::models::DataSeriesFilter>**](DataSeriesFilter.md) | Map of allowed filter values and aggregation strategy. List of available filter values can be fetched using metric filters endpoint and default aggregation strategy depends on metric | [optional] [default to null]
**granularity** | **String** | Data points interval granularity between two data points.Default value is \&quot;AUTO\&quot; - calculated based on selected time span. Not required while getting filters. | [optional] [default to null]
**interval** | **String** |  | [optional] [default to null]
**metric** | **String** | Metric name or metric group prefix | [default to null]
**start** | **String** | Start time of interval. Can be expressed as timestamp in milliseconds or UTC date in yyyy-MM-dd HH:mm:ss format | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


