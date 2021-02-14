/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextProjectMapping : Details of context to project associations.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextProjectMapping {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the project.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Whether context is global.
    #[serde(rename = "isGlobalContext", skip_serializing_if = "Option::is_none")]
    pub is_global_context: Option<bool>,
}

impl CustomFieldContextProjectMapping {
    /// Details of context to project associations.
    pub fn new(context_id: String) -> CustomFieldContextProjectMapping {
        CustomFieldContextProjectMapping {
            context_id,
            project_id: None,
            is_global_context: None,
        }
    }
}
