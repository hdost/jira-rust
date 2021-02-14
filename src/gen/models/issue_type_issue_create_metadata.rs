/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeIssueCreateMetadata : Details of the issue creation metadata for an issue type.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeIssueCreateMetadata {
    /// The URL of these issue type details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The ID of the issue type.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The description of the issue type.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of the issue type's avatar.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The name of the issue type.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this issue type is used to create subtasks.
    #[serde(rename = "subtask", skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
    /// The ID of the issue type's avatar.
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// Unique ID for next-gen projects.
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Hierarchy level of the issue type.
    #[serde(rename = "hierarchyLevel", skip_serializing_if = "Option::is_none")]
    pub hierarchy_level: Option<i32>,
    /// Details of the next-gen projects the issue type is available in.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<crate::gen::models::Scope>,
    /// Expand options that include additional issue type metadata details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// List of the fields available when creating an issue for the issue type.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<::std::collections::HashMap<String, crate::gen::models::FieldMetadata>>,
}

impl IssueTypeIssueCreateMetadata {
    /// Details of the issue creation metadata for an issue type.
    pub fn new() -> IssueTypeIssueCreateMetadata {
        IssueTypeIssueCreateMetadata {
            _self: None,
            id: None,
            description: None,
            icon_url: None,
            name: None,
            subtask: None,
            avatar_id: None,
            entity_id: None,
            hierarchy_level: None,
            scope: None,
            expand: None,
            fields: None,
        }
    }
}
