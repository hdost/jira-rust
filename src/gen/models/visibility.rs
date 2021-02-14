/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Visibility : The group or role to which this item is visible.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Visibility {
    /// Whether visibility of this item is restricted to a group or role.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The name of the group or role to which visibility of this item is restricted.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Visibility {
    /// The group or role to which this item is visible.
    pub fn new() -> Visibility {
        Visibility {
            _type: None,
            value: None,
        }
    }
}

/// Whether visibility of this item is restricted to a group or role.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "role")]
    Role,
}
