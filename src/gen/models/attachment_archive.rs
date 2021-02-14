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
pub struct AttachmentArchive {
    #[serde(rename = "moreAvailable", skip_serializing_if = "Option::is_none")]
    pub more_available: Option<bool>,
    #[serde(
        rename = "totalNumberOfEntriesAvailable",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_number_of_entries_available: Option<i32>,
    #[serde(rename = "totalEntryCount", skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i32>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::gen::models::AttachmentArchiveEntry>>,
}

impl AttachmentArchive {
    pub fn new() -> AttachmentArchive {
        AttachmentArchive {
            more_available: None,
            total_number_of_entries_available: None,
            total_entry_count: None,
            entries: None,
        }
    }
}