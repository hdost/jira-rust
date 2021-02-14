/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JexpIssues : The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JexpIssues {
    /// The JQL query that specifies the set of issues available in the Jira expression.
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<crate::gen::models::JexpJqlIssues>,
}

impl JexpIssues {
    /// The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable.
    pub fn new() -> JexpIssues {
        JexpIssues { jql: None }
    }
}
