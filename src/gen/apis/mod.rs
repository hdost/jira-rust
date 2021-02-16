use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod app_properties_api;
pub mod application_roles_api;
pub mod audit_records_api;
pub mod avatars_api;
pub mod dashboards_api;
pub mod dynamic_modules_api;
pub mod filter_sharing_api;
pub mod filters_api;
pub mod group_and_user_picker_api;
pub mod groups_api;
pub mod issue_attachments_api;
pub mod issue_comment_properties_api;
pub mod issue_comments_api;
pub mod issue_custom_field_contexts_api;
pub mod issue_custom_field_options_api;
pub mod issue_custom_field_options_apps_api;
pub mod issue_field_configurations_api;
pub mod issue_fields_api;
pub mod issue_link_types_api;
pub mod issue_links_api;
pub mod issue_navigator_settings_api;
pub mod issue_notification_schemes_api;
pub mod issue_priorities_api;
pub mod issue_properties_api;
pub mod issue_remote_links_api;
pub mod issue_resolutions_api;
pub mod issue_search_api;
pub mod issue_security_level_api;
pub mod issue_security_schemes_api;
pub mod issue_type_properties_api;
pub mod issue_type_schemes_api;
pub mod issue_type_screen_schemes_api;
pub mod issue_types_api;
pub mod issue_votes_api;
pub mod issue_watchers_api;
pub mod issue_worklog_properties_api;
pub mod issue_worklogs_api;
pub mod issues_api;
pub mod jira_expressions_api;
pub mod jira_settings_api;
pub mod jql_api;
pub mod labels_api;
pub mod myself_api;
pub mod permission_schemes_api;
pub mod permissions_api;
pub mod project_avatars_api;
pub mod project_categories_api;
pub mod project_components_api;
pub mod project_email_api;
pub mod project_key_and_name_validation_api;
pub mod project_permission_schemes_api;
pub mod project_properties_api;
pub mod project_role_actors_api;
pub mod project_roles_api;
pub mod project_types_api;
pub mod project_versions_api;
pub mod projects_api;
pub mod screen_schemes_api;
pub mod screen_tab_fields_api;
pub mod screen_tabs_api;
pub mod screens_api;
pub mod server_info_api;
pub mod tasks_api;
pub mod time_tracking_api;
pub mod user_properties_api;
pub mod user_search_api;
pub mod users_api;
pub mod webhooks_api;
pub mod workflow_scheme_drafts_api;
pub mod workflow_scheme_project_associations_api;
pub mod workflow_schemes_api;
pub mod workflow_status_categories_api;
pub mod workflow_statuses_api;
pub mod workflow_transition_properties_api;
pub mod workflow_transition_rules_api;
pub mod workflows_api;

pub mod configuration;
