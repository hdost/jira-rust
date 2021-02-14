/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldWasClause : A clause that asserts a previous value of a field. For example, `status WAS \"Resolved\" BY currentUser() BEFORE \"2019/02/02\"`. See [WAS](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-WASWAS) for more information about the WAS operator.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldWasClause {
    #[serde(rename = "field")]
    pub field: crate::gen::models::JqlQueryField,
    /// The operator between the field and operand.
    #[serde(rename = "operator")]
    pub operator: Operator,
    #[serde(rename = "operand")]
    pub operand: crate::gen::models::JqlQueryClauseOperand,
    /// The list of time predicates.
    #[serde(rename = "predicates")]
    pub predicates: Vec<crate::gen::models::JqlQueryClauseTimePredicate>,
}

impl FieldWasClause {
    /// A clause that asserts a previous value of a field. For example, `status WAS \"Resolved\" BY currentUser() BEFORE \"2019/02/02\"`. See [WAS](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-WASWAS) for more information about the WAS operator.
    pub fn new(
        field: crate::gen::models::JqlQueryField,
        operator: Operator,
        operand: crate::gen::models::JqlQueryClauseOperand,
        predicates: Vec<crate::gen::models::JqlQueryClauseTimePredicate>,
    ) -> FieldWasClause {
        FieldWasClause {
            field,
            operator,
            operand,
            predicates,
        }
    }
}

/// The operator between the field and operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "was")]
    Was,
    #[serde(rename = "was in")]
    WasIn,
    #[serde(rename = "was not in")]
    WasNotIn,
    #[serde(rename = "was not")]
    WasNot,
}
