/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueMatchesForJql : A list of the issues matched to a JQL query or details of errors encountered during matching.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueMatchesForJql {
    /// A list of issue IDs.
    #[serde(rename = "matchedIssues")]
    pub matched_issues: Vec<i64>,
    /// A list of errors.
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
}

impl IssueMatchesForJql {
    /// A list of the issues matched to a JQL query or details of errors encountered during matching.
    pub fn new(matched_issues: Vec<i64>, errors: Vec<String>) -> IssueMatchesForJql {
        IssueMatchesForJql {
            matched_issues,
            errors,
        }
    }
}
