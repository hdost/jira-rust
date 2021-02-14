/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::gen::apis::ResponseContent;
use crate::gen::models;

/// struct for passing parameters to the method `add_share_permission`
#[derive(Clone, Debug)]
pub struct AddSharePermissionParams {
    /// The ID of the filter.
    pub id: i64,
    pub share_permission_input_bean: models::SharePermissionInputBean,
}

/// struct for passing parameters to the method `delete_share_permission`
#[derive(Clone, Debug)]
pub struct DeleteSharePermissionParams {
    /// The ID of the filter.
    pub id: i64,
    /// The ID of the share permission.
    pub permission_id: i64,
}

/// struct for passing parameters to the method `get_share_permission`
#[derive(Clone, Debug)]
pub struct GetSharePermissionParams {
    /// The ID of the filter.
    pub id: i64,
    /// The ID of the share permission.
    pub permission_id: i64,
}

/// struct for passing parameters to the method `get_share_permissions`
#[derive(Clone, Debug)]
pub struct GetSharePermissionsParams {
    /// The ID of the filter.
    pub id: i64,
}

/// struct for passing parameters to the method `set_default_share_scope`
#[derive(Clone, Debug)]
pub struct SetDefaultShareScopeParams {
    pub default_share_scope: models::DefaultShareScope,
}

/// struct for typed errors of method `add_share_permission`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSharePermissionError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_share_permission`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSharePermissionError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_default_share_scope`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDefaultShareScopeError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_share_permission`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSharePermissionError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_share_permissions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSharePermissionsError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_default_share_scope`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetDefaultShareScopeError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// Add a share permissions to a filter. If you add a global share permission (one for all logged-in users or the public) it will overwrite all share permissions for the filter.  Be aware that this operation uses different objects for updating share permissions compared to [Update filter](#api-rest-api-3-filter-id-put).  **[Permissions](#permissions) required:** *Share dashboards and filters* [global permission](https://confluence.atlassian.com/x/x4dKLg) and the user must own the filter.
pub async fn add_share_permission(
    configuration: &configuration::Configuration,
    params: AddSharePermissionParams,
) -> Result<Vec<models::SharePermission>, Error<AddSharePermissionError>> {
    // unbox the parameters
    let id = params.id;
    let share_permission_input_bean = params.share_permission_input_bean;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/filter/{id}/permission",
        configuration.base_path,
        id = id
    );
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&share_permission_input_bean);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddSharePermissionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a share permission from a filter.  **[Permissions](#permissions) required:** Permission to access Jira and the user must own the filter.
pub async fn delete_share_permission(
    configuration: &configuration::Configuration,
    params: DeleteSharePermissionParams,
) -> Result<(), Error<DeleteSharePermissionError>> {
    // unbox the parameters
    let id = params.id;
    let permission_id = params.permission_id;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/filter/{id}/permission/{permissionId}",
        configuration.base_path,
        id = id,
        permissionId = permission_id
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteSharePermissionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the default sharing settings for new filters and dashboards for a user.  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn get_default_share_scope(
    configuration: &configuration::Configuration,
) -> Result<models::DefaultShareScope, Error<GetDefaultShareScopeError>> {
    // unbox the parameters

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/filter/defaultShareScope",
        configuration.base_path
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetDefaultShareScopeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a share permission for a filter. A filter can be shared with groups, projects, all logged-in users, or the public. Sharing with all logged-in users or the public is known as a global share permission.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None, however, a share permission is only returned for:   *  filters owned by the user.  *  filters shared with a group that the user is a member of.  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.  *  filters shared with a public project.  *  filters shared with the public.
pub async fn get_share_permission(
    configuration: &configuration::Configuration,
    params: GetSharePermissionParams,
) -> Result<models::SharePermission, Error<GetSharePermissionError>> {
    // unbox the parameters
    let id = params.id;
    let permission_id = params.permission_id;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/filter/{id}/permission/{permissionId}",
        configuration.base_path,
        id = id,
        permissionId = permission_id
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSharePermissionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the share permissions for a filter. A filter can be shared with groups, projects, all logged-in users, or the public. Sharing with all logged-in users or the public is known as a global share permission.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None, however, share permissions are only returned for:   *  filters owned by the user.  *  filters shared with a group that the user is a member of.  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.  *  filters shared with a public project.  *  filters shared with the public.
pub async fn get_share_permissions(
    configuration: &configuration::Configuration,
    params: GetSharePermissionsParams,
) -> Result<Vec<models::SharePermission>, Error<GetSharePermissionsError>> {
    // unbox the parameters
    let id = params.id;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/filter/{id}/permission",
        configuration.base_path,
        id = id
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSharePermissionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets the default sharing for new filters and dashboards for a user.  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn set_default_share_scope(
    configuration: &configuration::Configuration,
    params: SetDefaultShareScopeParams,
) -> Result<models::DefaultShareScope, Error<SetDefaultShareScopeError>> {
    // unbox the parameters
    let default_share_scope = params.default_share_scope;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/filter/defaultShareScope",
        configuration.base_path
    );
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&default_share_scope);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetDefaultShareScopeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
