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
pub struct BasicOrganizationDto {
    #[serde(rename = "authMethods", skip_serializing_if = "Option::is_none")]
    pub auth_methods: Option<Vec<crate::models::BasicAuthMethodDto>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl BasicOrganizationDto {
    pub fn new() -> BasicOrganizationDto {
        BasicOrganizationDto {
            auth_methods: None,
            name: None,
            status: None,
            uuid: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    ACTIVE,
    #[serde(rename = "IN_REGISTRATION")]
    INREGISTRATION,
    #[serde(rename = "DISABLED")]
    DISABLED,
    #[serde(rename = "EXPIRED")]
    EXPIRED,
    #[serde(rename = "INVITED")]
    INVITED,
    #[serde(rename = "DEMO")]
    DEMO,
}

