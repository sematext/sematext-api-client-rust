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
pub struct DataSeriesFilter {
    #[serde(rename = "aggregation", skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<Aggregation>,
    #[serde(rename = "multiValue", skip_serializing_if = "Option::is_none")]
    pub multi_value: Option<bool>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl DataSeriesFilter {
    pub fn new() -> DataSeriesFilter {
        DataSeriesFilter {
            aggregation: None,
            multi_value: None,
            values: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Aggregation {
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "SUM")]
    SUM,
    #[serde(rename = "AVG")]
    AVG,
    #[serde(rename = "MIN")]
    MIN,
    #[serde(rename = "MAX")]
    MAX,
}

