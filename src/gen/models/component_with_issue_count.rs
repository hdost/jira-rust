/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ComponentWithIssueCount : Details about a component with a count of the issues it contains.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentWithIssueCount {
    /// Count of issues for the component.
    #[serde(rename = "issueCount", skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<i64>,
    /// The description for the component.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL for this count of the issues contained in the component.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// Not used.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The key of the project to which the component is assigned.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The user assigned to issues created with this component, when `assigneeType` does not identify a valid assignee.
    #[serde(rename = "realAssignee", skip_serializing_if = "Option::is_none")]
    pub real_assignee: Option<crate::gen::models::User>,
    /// Whether a user is associated with `assigneeType`. For example, if the `assigneeType` is set to `COMPONENT_LEAD` but the component lead is not set, then `false` is returned.
    #[serde(
        rename = "isAssigneeTypeValid",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_assignee_type_valid: Option<bool>,
    /// The details of the user associated with `assigneeType`, if any. See `realAssignee` for details of the user assigned to issues created with this component.
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<crate::gen::models::User>,
    /// The nominal user type used to determine the assignee for issues created with this component. See `realAssigneeType` for details on how the type of the user, and hence the user, assigned to issues is determined. Takes the following values:   *  `PROJECT_LEAD` the assignee to any issues created with this component is nominally the lead for the project the component is in.  *  `COMPONENT_LEAD` the assignee to any issues created with this component is nominally the lead for the component.  *  `UNASSIGNED` an assignee is not set for issues created with this component.  *  `PROJECT_DEFAULT` the assignee to any issues created with this component is nominally the default assignee for the project that the component is in.
    #[serde(rename = "assigneeType", skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<AssigneeType>,
    /// The user details for the component's lead user.
    #[serde(rename = "lead", skip_serializing_if = "Option::is_none")]
    pub lead: Option<crate::gen::models::User>,
    /// The type of the assignee that is assigned to issues created with this component, when an assignee cannot be set from the `assigneeType`. For example, `assigneeType` is set to `COMPONENT_LEAD` but no component lead is set. This property is set to one of the following values:   *  `PROJECT_LEAD` when `assigneeType` is `PROJECT_LEAD` and the project lead has permission to be assigned issues in the project that the component is in.  *  `COMPONENT_LEAD` when `assignee`Type is `COMPONENT_LEAD` and the component lead has permission to be assigned issues in the project that the component is in.  *  `UNASSIGNED` when `assigneeType` is `UNASSIGNED` and Jira is configured to allow unassigned issues.  *  `PROJECT_DEFAULT` when none of the preceding cases are true.
    #[serde(rename = "realAssigneeType", skip_serializing_if = "Option::is_none")]
    pub real_assignee_type: Option<RealAssigneeType>,
    /// The name for the component.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique identifier for the component.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl ComponentWithIssueCount {
    /// Details about a component with a count of the issues it contains.
    pub fn new() -> ComponentWithIssueCount {
        ComponentWithIssueCount {
            issue_count: None,
            description: None,
            _self: None,
            project_id: None,
            project: None,
            real_assignee: None,
            is_assignee_type_valid: None,
            assignee: None,
            assignee_type: None,
            lead: None,
            real_assignee_type: None,
            name: None,
            id: None,
        }
    }
}

/// The nominal user type used to determine the assignee for issues created with this component. See `realAssigneeType` for details on how the type of the user, and hence the user, assigned to issues is determined. Takes the following values:   *  `PROJECT_LEAD` the assignee to any issues created with this component is nominally the lead for the project the component is in.  *  `COMPONENT_LEAD` the assignee to any issues created with this component is nominally the lead for the component.  *  `UNASSIGNED` an assignee is not set for issues created with this component.  *  `PROJECT_DEFAULT` the assignee to any issues created with this component is nominally the default assignee for the project that the component is in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssigneeType {
    #[serde(rename = "PROJECT_DEFAULT")]
    PROJECTDEFAULT,
    #[serde(rename = "COMPONENT_LEAD")]
    COMPONENTLEAD,
    #[serde(rename = "PROJECT_LEAD")]
    PROJECTLEAD,
    #[serde(rename = "UNASSIGNED")]
    UNASSIGNED,
}
/// The type of the assignee that is assigned to issues created with this component, when an assignee cannot be set from the `assigneeType`. For example, `assigneeType` is set to `COMPONENT_LEAD` but no component lead is set. This property is set to one of the following values:   *  `PROJECT_LEAD` when `assigneeType` is `PROJECT_LEAD` and the project lead has permission to be assigned issues in the project that the component is in.  *  `COMPONENT_LEAD` when `assignee`Type is `COMPONENT_LEAD` and the component lead has permission to be assigned issues in the project that the component is in.  *  `UNASSIGNED` when `assigneeType` is `UNASSIGNED` and Jira is configured to allow unassigned issues.  *  `PROJECT_DEFAULT` when none of the preceding cases are true.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RealAssigneeType {
    #[serde(rename = "PROJECT_DEFAULT")]
    PROJECTDEFAULT,
    #[serde(rename = "COMPONENT_LEAD")]
    COMPONENTLEAD,
    #[serde(rename = "PROJECT_LEAD")]
    PROJECTLEAD,
    #[serde(rename = "UNASSIGNED")]
    UNASSIGNED,
}