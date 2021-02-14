/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveOptionFromIssuesResult {
    /// The IDs of the modified issues.
    #[serde(rename = "modifiedIssues", skip_serializing_if = "Option::is_none")]
    pub modified_issues: Option<Vec<i64>>,
    /// The IDs of the unchanged issues, those issues where errors prevent modification.
    #[serde(rename = "unmodifiedIssues", skip_serializing_if = "Option::is_none")]
    pub unmodified_issues: Option<Vec<i64>>,
    /// A collection of errors related to unchanged issues. The collection size is limited, which means not all errors may be returned.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<crate::gen::models::SimpleErrorCollection>,
}

impl RemoveOptionFromIssuesResult {
    pub fn new() -> RemoveOptionFromIssuesResult {
        RemoveOptionFromIssuesResult {
            modified_issues: None,
            unmodified_issues: None,
            errors: None,
        }
    }
}