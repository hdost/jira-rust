/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQueryClauseOperand : Details of an operand in a JQL clause.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryClauseOperand {
    /// The list of operand values.
    #[serde(rename = "values")]
    pub values: Vec<crate::gen::models::JqlQueryUnitaryOperand>,
    /// The operand value.
    #[serde(rename = "value")]
    pub value: String,
    /// The name of the function.
    #[serde(rename = "function")]
    pub function: String,
    /// The list of function arguments.
    #[serde(rename = "arguments")]
    pub arguments: Vec<String>,
    /// The keyword that is the operand value.
    #[serde(rename = "keyword")]
    pub keyword: Keyword,
}

impl JqlQueryClauseOperand {
    /// Details of an operand in a JQL clause.
    pub fn new(
        values: Vec<crate::gen::models::JqlQueryUnitaryOperand>,
        value: String,
        function: String,
        arguments: Vec<String>,
        keyword: Keyword,
    ) -> JqlQueryClauseOperand {
        JqlQueryClauseOperand {
            values,
            value,
            function,
            arguments,
            keyword,
        }
    }
}

/// The keyword that is the operand value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Keyword {
    #[serde(rename = "empty")]
    Empty,
}