/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// StatusDetails : A status.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusDetails {
    /// The URL of the status.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The description of the status.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of the icon used to represent the status.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The name of the status.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the status.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The category assigned to the status.
    #[serde(rename = "statusCategory", skip_serializing_if = "Option::is_none")]
    pub status_category: Option<crate::gen::models::StatusCategory>,
}

impl StatusDetails {
    /// A status.
    pub fn new() -> StatusDetails {
        StatusDetails {
            _self: None,
            description: None,
            icon_url: None,
            name: None,
            id: None,
            status_category: None,
        }
    }
}