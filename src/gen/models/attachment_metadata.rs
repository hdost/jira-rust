/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AttachmentMetadata : Metadata for an issue attachment.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentMetadata {
    /// The ID of the attachment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The URL of the attachment metadata details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The name of the attachment file.
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Details of the user who attached the file.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<crate::gen::models::User>,
    /// The datetime the attachment was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The size of the attachment.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The MIME type of the attachment.
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Additional properties of the attachment.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The URL of the attachment.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The URL of a thumbnail representing the attachment.
    #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}

impl AttachmentMetadata {
    /// Metadata for an issue attachment.
    pub fn new() -> AttachmentMetadata {
        AttachmentMetadata {
            id: None,
            _self: None,
            filename: None,
            author: None,
            created: None,
            size: None,
            mime_type: None,
            properties: None,
            content: None,
            thumbnail: None,
        }
    }
}
