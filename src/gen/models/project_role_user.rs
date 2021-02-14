/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectRoleUser : Details of the user associated with the role.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRoleUser {
    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. Returns *unknown* if the record is deleted and corrupted, for example, as the result of a server import.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl ProjectRoleUser {
    /// Details of the user associated with the role.
    pub fn new() -> ProjectRoleUser {
        ProjectRoleUser { account_id: None }
    }
}
