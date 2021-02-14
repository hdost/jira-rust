/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PageOfWorklogs : Paginated list of worklog details

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageOfWorklogs {
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The number of results on the page.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// List of worklogs.
    #[serde(rename = "worklogs", skip_serializing_if = "Option::is_none")]
    pub worklogs: Option<Vec<crate::gen::models::Worklog>>,
}

impl PageOfWorklogs {
    /// Paginated list of worklog details
    pub fn new() -> PageOfWorklogs {
        PageOfWorklogs {
            start_at: None,
            max_results: None,
            total: None,
            worklogs: None,
        }
    }
}