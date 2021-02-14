/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeWithStatus : Status details for an issue type.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeWithStatus {
    /// The URL of the issue type's status details.
    #[serde(rename = "self")]
    pub _self: String,
    /// The ID of the issue type.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the issue type.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether this issue type represents subtasks.
    #[serde(rename = "subtask")]
    pub subtask: bool,
    /// List of status details for the issue type.
    #[serde(rename = "statuses")]
    pub statuses: Vec<crate::gen::models::StatusDetails>,
}

impl IssueTypeWithStatus {
    /// Status details for an issue type.
    pub fn new(
        _self: String,
        id: String,
        name: String,
        subtask: bool,
        statuses: Vec<crate::gen::models::StatusDetails>,
    ) -> IssueTypeWithStatus {
        IssueTypeWithStatus {
            _self,
            id,
            name,
            subtask,
            statuses,
        }
    }
}