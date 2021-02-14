/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ScreenDetails : Details of a screen.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenDetails {
    /// The name of the screen. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the screen. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ScreenDetails {
    /// Details of a screen.
    pub fn new(name: String) -> ScreenDetails {
        ScreenDetails {
            name,
            description: None,
        }
    }
}
