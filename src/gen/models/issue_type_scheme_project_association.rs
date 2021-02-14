/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeSchemeProjectAssociation : Details of the association between an issue type scheme and project.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeSchemeProjectAssociation {
    /// The ID of the issue type scheme.
    #[serde(rename = "issueTypeSchemeId")]
    pub issue_type_scheme_id: String,
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
}

impl IssueTypeSchemeProjectAssociation {
    /// Details of the association between an issue type scheme and project.
    pub fn new(
        issue_type_scheme_id: String,
        project_id: String,
    ) -> IssueTypeSchemeProjectAssociation {
        IssueTypeSchemeProjectAssociation {
            issue_type_scheme_id,
            project_id,
        }
    }
}
