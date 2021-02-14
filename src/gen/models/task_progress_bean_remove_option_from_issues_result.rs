/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// TaskProgressBeanRemoveOptionFromIssuesResult : Details about a task.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskProgressBeanRemoveOptionFromIssuesResult {
    /// The URL of the task.
    #[serde(rename = "self")]
    pub _self: String,
    /// The ID of the task.
    #[serde(rename = "id")]
    pub id: String,
    /// The description of the task.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The status of the task.
    #[serde(rename = "status")]
    pub status: Status,
    /// Information about the progress of the task.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The result of the task execution.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::gen::models::RemoveOptionFromIssuesResult>,
    /// The ID of the user who submitted the task.
    #[serde(rename = "submittedBy")]
    pub submitted_by: i64,
    /// The progress of the task, as a percentage complete.
    #[serde(rename = "progress")]
    pub progress: i64,
    /// The execution time of the task, in milliseconds.
    #[serde(rename = "elapsedRuntime")]
    pub elapsed_runtime: i64,
    /// A timestamp recording when the task was submitted.
    #[serde(rename = "submitted")]
    pub submitted: i64,
    /// A timestamp recording when the task was started.
    #[serde(rename = "started", skip_serializing_if = "Option::is_none")]
    pub started: Option<i64>,
    /// A timestamp recording when the task was finished.
    #[serde(rename = "finished", skip_serializing_if = "Option::is_none")]
    pub finished: Option<i64>,
    /// A timestamp recording when the task progress was last updated.
    #[serde(rename = "lastUpdate")]
    pub last_update: i64,
}

impl TaskProgressBeanRemoveOptionFromIssuesResult {
    /// Details about a task.
    pub fn new(
        _self: String,
        id: String,
        status: Status,
        submitted_by: i64,
        progress: i64,
        elapsed_runtime: i64,
        submitted: i64,
        last_update: i64,
    ) -> TaskProgressBeanRemoveOptionFromIssuesResult {
        TaskProgressBeanRemoveOptionFromIssuesResult {
            _self,
            id,
            description: None,
            status,
            message: None,
            result: None,
            submitted_by,
            progress,
            elapsed_runtime,
            submitted,
            started: None,
            finished: None,
            last_update,
        }
    }
}

/// The status of the task.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ENQUEUED")]
    ENQUEUED,
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "COMPLETE")]
    COMPLETE,
    #[serde(rename = "FAILED")]
    FAILED,
    #[serde(rename = "CANCEL_REQUESTED")]
    CANCELREQUESTED,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
    #[serde(rename = "DEAD")]
    DEAD,
}
