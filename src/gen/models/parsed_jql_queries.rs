/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ParsedJqlQueries : A list of parsed JQL queries.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParsedJqlQueries {
    /// A list of parsed JQL queries.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::gen::models::ParsedJqlQuery>,
}

impl ParsedJqlQueries {
    /// A list of parsed JQL queries.
    pub fn new(queries: Vec<crate::gen::models::ParsedJqlQuery>) -> ParsedJqlQueries {
        ParsedJqlQueries { queries }
    }
}
