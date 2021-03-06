/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowRules : A collection of transition rules.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRules {
    /// The workflow conditions.
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::gen::models::WorkflowTransitionRule>,
    /// The workflow validators.
    #[serde(rename = "validators")]
    pub validators: Vec<crate::gen::models::WorkflowTransitionRule>,
    /// The workflow post functions.
    #[serde(rename = "postFunctions")]
    pub post_functions: Vec<crate::gen::models::WorkflowTransitionRule>,
}

impl WorkflowRules {
    /// A collection of transition rules.
    pub fn new(
        conditions: Vec<crate::gen::models::WorkflowTransitionRule>,
        validators: Vec<crate::gen::models::WorkflowTransitionRule>,
        post_functions: Vec<crate::gen::models::WorkflowTransitionRule>,
    ) -> WorkflowRules {
        WorkflowRules {
            conditions,
            validators,
            post_functions,
        }
    }
}
