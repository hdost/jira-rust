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
pub struct IssueCommentListRequestBean {
    /// The list of comment IDs. A maximum of 1000 IDs can be specified.
    #[serde(rename = "ids")]
    pub ids: Vec<i64>,
}

impl IssueCommentListRequestBean {
    pub fn new(ids: Vec<i64>) -> IssueCommentListRequestBean {
        IssueCommentListRequestBean { ids }
    }
}
