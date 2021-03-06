/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Workflow : Details about a workflow.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(rename = "id")]
    pub id: crate::gen::models::PublishedWorkflowId,
    /// The description of the workflow.
    #[serde(rename = "description")]
    pub description: String,
    /// The transitions of the workflow.
    #[serde(rename = "transitions", skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<crate::gen::models::Transition>>,
    /// The statuses of the workflow.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<crate::gen::models::WorkflowStatus>>,
}

impl Workflow {
    /// Details about a workflow.
    pub fn new(id: crate::gen::models::PublishedWorkflowId, description: String) -> Workflow {
        Workflow {
            id,
            description,
            transitions: None,
            statuses: None,
        }
    }
}
