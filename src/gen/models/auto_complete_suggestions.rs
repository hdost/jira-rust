/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AutoCompleteSuggestions : The results from a JQL query.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoCompleteSuggestions {
    /// The list of suggested item.
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::gen::models::AutoCompleteSuggestion>>,
}

impl AutoCompleteSuggestions {
    /// The results from a JQL query.
    pub fn new() -> AutoCompleteSuggestions {
        AutoCompleteSuggestions { results: None }
    }
}
