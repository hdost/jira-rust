/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PageOfComments : A page of comments.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageOfComments {
    /// The index of the first item returned.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The maximum number of items that could be returned.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The number of items returned.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// The list of comments.
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<crate::gen::models::Comment>>,
}

impl PageOfComments {
    /// A page of comments.
    pub fn new() -> PageOfComments {
        PageOfComments {
            start_at: None,
            max_results: None,
            total: None,
            comments: None,
        }
    }
}
