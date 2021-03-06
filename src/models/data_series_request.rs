/*
 * Sematext Cloud API
 *
 * API Explorer provides access and documentation for Sematext REST API. The REST API requires the API Key to be sent as part of `Authorization` header. E.g.: `Authorization : apiKey e5f18450-205a-48eb-8589-7d49edaea813`.
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSeriesRequest {
    #[serde(rename = "defaultInterval", skip_serializing_if = "Option::is_none")]
    pub default_interval: Option<i64>,
    /// End time of interval. Can be expressed as timestamp in milliseconds or UTC date in yyyy-MM-dd HH:mm:ss format
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Map of allowed filter values and aggregation strategy. List of available filter values can be fetched using metric filters endpoint and default aggregation strategy depends on metric
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, crate::models::DataSeriesFilter>>,
    /// Data points interval granularity between two data points.Default value is \"AUTO\" - calculated based on selected time span. Not required while getting filters.
    #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Granularity>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Metric name or metric group prefix
    #[serde(rename = "metric")]
    pub metric: String,
    /// Start time of interval. Can be expressed as timestamp in milliseconds or UTC date in yyyy-MM-dd HH:mm:ss format
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

impl DataSeriesRequest {
    pub fn new(metric: String) -> DataSeriesRequest {
        DataSeriesRequest {
            default_interval: None,
            end: None,
            filters: None,
            granularity: None,
            interval: None,
            metric,
            start: None,
        }
    }
}

/// Data points interval granularity between two data points.Default value is \"AUTO\" - calculated based on selected time span. Not required while getting filters.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Granularity {
    #[serde(rename = "AUTO")]
    AUTO,
    #[serde(rename = "ONE_MINUTE")]
    ONEMINUTE,
    #[serde(rename = "FIVE_MINUTES")]
    FIVEMINUTES,
    #[serde(rename = "HOUR")]
    HOUR,
    #[serde(rename = "DAY")]
    DAY,
    #[serde(rename = "WEEK")]
    WEEK,
    #[serde(rename = "MONTH")]
    MONTH,
}

