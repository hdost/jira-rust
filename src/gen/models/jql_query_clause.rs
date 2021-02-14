/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQueryClause : A JQL query clause.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryClause {
    /// The list of nested clauses.
    #[serde(rename = "clauses")]
    pub clauses: Vec<crate::gen::models::JqlQueryClause>,
    /// The operator applied to the field.
    #[serde(rename = "operator")]
    pub operator: Operator,
    #[serde(rename = "field")]
    pub field: crate::gen::models::JqlQueryField,
    #[serde(rename = "operand")]
    pub operand: crate::gen::models::JqlQueryClauseOperand,
    /// The list of time predicates.
    #[serde(rename = "predicates")]
    pub predicates: Vec<crate::gen::models::JqlQueryClauseTimePredicate>,
}

impl JqlQueryClause {
    /// A JQL query clause.
    pub fn new(
        clauses: Vec<crate::gen::models::JqlQueryClause>,
        operator: Operator,
        field: crate::gen::models::JqlQueryField,
        operand: crate::gen::models::JqlQueryClauseOperand,
        predicates: Vec<crate::gen::models::JqlQueryClauseTimePredicate>,
    ) -> JqlQueryClause {
        JqlQueryClause {
            clauses,
            operator,
            field,
            operand,
            predicates,
        }
    }
}

/// The operator applied to the field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "changed")]
    Changed,
}
