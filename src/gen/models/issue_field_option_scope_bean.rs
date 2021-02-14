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
pub struct IssueFieldOptionScopeBean {
    /// DEPRECATED
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<i64>>,
    /// Defines the projects in which the option is available and the behavior of the option within each project. Specify one object per project. The behavior of the option in a project context overrides the behavior in the global context.
    #[serde(rename = "projects2", skip_serializing_if = "Option::is_none")]
    pub projects2: Option<Vec<crate::gen::models::ProjectScopeBean>>,
    /// Defines the behavior of the option within the global context. If this property is set, even if set to an empty object, then the option is available in all projects.
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<crate::gen::models::GlobalScopeBean>,
}

impl IssueFieldOptionScopeBean {
    pub fn new() -> IssueFieldOptionScopeBean {
        IssueFieldOptionScopeBean {
            projects: None,
            projects2: None,
            global: None,
        }
    }
}
