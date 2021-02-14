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

/// struct for passing parameters to the method `delete_issue_type_property`
#[derive(Clone, Debug)]
pub struct DeleteIssueTypePropertyParams {
    /// The ID of the issue type.
    pub issue_type_id: String,
    /// The key of the property. Use [Get issue type property keys](#api-rest-api-3-issuetype-issueTypeId-properties-get) to get a list of all issue type property keys.
    pub property_key: String,
}

/// struct for passing parameters to the method `get_issue_type_property`
#[derive(Clone, Debug)]
pub struct GetIssueTypePropertyParams {
    /// The ID of the issue type.
    pub issue_type_id: String,
    /// The key of the property. Use [Get issue type property keys](#api-rest-api-3-issuetype-issueTypeId-properties-get) to get a list of all issue type property keys.
    pub property_key: String,
}

/// struct for passing parameters to the method `get_issue_type_property_keys`
#[derive(Clone, Debug)]
pub struct GetIssueTypePropertyKeysParams {
    /// The ID of the issue type.
    pub issue_type_id: String,
}

/// struct for passing parameters to the method `set_issue_type_property`
#[derive(Clone, Debug)]
pub struct SetIssueTypePropertyParams {
    /// The ID of the issue type.
    pub issue_type_id: String,
    /// The key of the issue type property. The maximum length is 255 characters.
    pub property_key: String,
    pub body: Option<serde_json::Value>,
}

/// struct for typed errors of method `delete_issue_type_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIssueTypePropertyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_issue_type_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIssueTypePropertyError {
    Status400(),
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_issue_type_property_keys`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIssueTypePropertyKeysError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_issue_type_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetIssueTypePropertyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// Deletes the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn delete_issue_type_property(
    configuration: &configuration::Configuration,
    params: DeleteIssueTypePropertyParams,
) -> Result<(), Error<DeleteIssueTypePropertyError>> {
    // unbox the parameters
    let issue_type_id = params.issue_type_id;
    let property_key = params.property_key;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/issuetype/{issueTypeId}/properties/{propertyKey}",
        configuration.base_path,
        issueTypeId = crate::gen::apis::urlencode(issue_type_id),
        propertyKey = crate::gen::apis::urlencode(property_key)
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
        let local_var_entity: Option<DeleteIssueTypePropertyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the key and value of the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) to get the details of any issue type.  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) to get the details of any issue types associated with the projects the user has permission to browse.
pub async fn get_issue_type_property(
    configuration: &configuration::Configuration,
    params: GetIssueTypePropertyParams,
) -> Result<models::EntityProperty, Error<GetIssueTypePropertyError>> {
    // unbox the parameters
    let issue_type_id = params.issue_type_id;
    let property_key = params.property_key;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/issuetype/{issueTypeId}/properties/{propertyKey}",
        configuration.base_path,
        issueTypeId = crate::gen::apis::urlencode(issue_type_id),
        propertyKey = crate::gen::apis::urlencode(property_key)
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
        let local_var_entity: Option<GetIssueTypePropertyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties) keys of the issue type.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) to get the property keys of any issue type.  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) to get the property keys of any issue types associated with the projects the user has permission to browse.
pub async fn get_issue_type_property_keys(
    configuration: &configuration::Configuration,
    params: GetIssueTypePropertyKeysParams,
) -> Result<models::PropertyKeys, Error<GetIssueTypePropertyKeysError>> {
    // unbox the parameters
    let issue_type_id = params.issue_type_id;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/issuetype/{issueTypeId}/properties",
        configuration.base_path,
        issueTypeId = crate::gen::apis::urlencode(issue_type_id)
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
        let local_var_entity: Option<GetIssueTypePropertyKeysError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates or updates the value of the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties). Use this resource to store and update data against an issue type.  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn set_issue_type_property(
    configuration: &configuration::Configuration,
    params: SetIssueTypePropertyParams,
) -> Result<serde_json::Value, Error<SetIssueTypePropertyError>> {
    // unbox the parameters
    let issue_type_id = params.issue_type_id;
    let property_key = params.property_key;
    let body = params.body;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/issuetype/{issueTypeId}/properties/{propertyKey}",
        configuration.base_path,
        issueTypeId = crate::gen::apis::urlencode(issue_type_id),
        propertyKey = crate::gen::apis::urlencode(property_key)
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
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetIssueTypePropertyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
