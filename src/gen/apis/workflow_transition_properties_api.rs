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

/// struct for passing parameters to the method `create_workflow_transition_property`
#[derive(Clone, Debug)]
pub struct CreateWorkflowTransitionPropertyParams {
    /// The ID of the transition. To get the ID, view the workflow in text mode in the Jira admin settings. The ID is shown next to the transition.
    pub transition_id: i64,
    /// The key of the property being added, also known as the name of the property. Set this to the same value as the `key` defined in the request body.
    pub key: String,
    /// The name of the workflow that the transition belongs to.
    pub workflow_name: String,
    pub request_body: ::std::collections::HashMap<String, serde_json::Value>,
    /// The workflow status. Set to *live* for inactive workflows or *draft* for draft workflows. Active workflows cannot be edited.
    pub workflow_mode: Option<String>,
}

/// struct for passing parameters to the method `delete_workflow_transition_property`
#[derive(Clone, Debug)]
pub struct DeleteWorkflowTransitionPropertyParams {
    /// The ID of the transition. To get the ID, view the workflow in text mode in the Jira admin settings. The ID is shown next to the transition.
    pub transition_id: i64,
    /// The name of the transition property to delete, also known as the name of the property.
    pub key: String,
    /// The name of the workflow that the transition belongs to.
    pub workflow_name: String,
    /// The workflow status. Set to `live` for inactive workflows or `draft` for draft workflows. Active workflows cannot be edited.
    pub workflow_mode: Option<String>,
}

/// struct for passing parameters to the method `get_workflow_transition_properties`
#[derive(Clone, Debug)]
pub struct GetWorkflowTransitionPropertiesParams {
    /// The ID of the transition. To get the ID, view the workflow in text mode in the Jira administration console. The ID is shown next to the transition.
    pub transition_id: i64,
    /// The name of the workflow that the transition belongs to.
    pub workflow_name: String,
    /// Some properties with keys that have the *jira.* prefix are reserved, which means they are not editable. To include these properties in the results, set this parameter to *true*.
    pub include_reserved_keys: Option<bool>,
    /// The key of the property being returned, also known as the name of the property. If this parameter is not specified, all properties on the transition are returned.
    pub key: Option<String>,
    /// The workflow status. Set to *live* for active and inactive workflows, or *draft* for draft workflows.
    pub workflow_mode: Option<String>,
}

/// struct for passing parameters to the method `update_workflow_transition_property`
#[derive(Clone, Debug)]
pub struct UpdateWorkflowTransitionPropertyParams {
    /// The ID of the transition. To get the ID, view the workflow in text mode in the Jira admin settings. The ID is shown next to the transition.
    pub transition_id: i64,
    /// The key of the property being updated, also known as the name of the property. Set this to the same value as the `key` defined in the request body.
    pub key: String,
    /// The name of the workflow that the transition belongs to.
    pub workflow_name: String,
    pub request_body: ::std::collections::HashMap<String, serde_json::Value>,
    /// The workflow status. Set to `live` for inactive workflows or `draft` for draft workflows. Active workflows cannot be edited.
    pub workflow_mode: Option<String>,
}

/// struct for typed errors of method `create_workflow_transition_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateWorkflowTransitionPropertyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_workflow_transition_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteWorkflowTransitionPropertyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_workflow_transition_properties`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorkflowTransitionPropertiesError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_workflow_transition_property`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateWorkflowTransitionPropertyError {
    Status400(),
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// Adds a property to a workflow transition. Transition properties are used to change the behavior of a transition. For more information, see [Transition properties](https://confluence.atlassian.com/x/zIhKLg#Advancedworkflowconfiguration-transitionproperties) and [Workflow properties](https://confluence.atlassian.com/x/JYlKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn create_workflow_transition_property(
    configuration: &configuration::Configuration,
    params: CreateWorkflowTransitionPropertyParams,
) -> Result<models::WorkflowTransitionProperty, Error<CreateWorkflowTransitionPropertyError>> {
    // unbox the parameters
    let transition_id = params.transition_id;
    let key = params.key;
    let workflow_name = params.workflow_name;
    let request_body = params.request_body;
    let workflow_mode = params.workflow_mode;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/workflow/transitions/{transitionId}/properties",
        configuration.base_path,
        transitionId = transition_id
    );
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    local_var_req_builder =
        local_var_req_builder.query(&[("workflowName", &workflow_name.to_string())]);
    if let Some(ref local_var_str) = workflow_mode {
        local_var_req_builder =
            local_var_req_builder.query(&[("workflowMode", &local_var_str.to_string())]);
    }
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
    local_var_req_builder = local_var_req_builder.json(&request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateWorkflowTransitionPropertyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a property from a workflow transition. Transition properties are used to change the behavior of a transition. For more information, see [Transition properties](https://confluence.atlassian.com/x/zIhKLg#Advancedworkflowconfiguration-transitionproperties) and [Workflow properties](https://confluence.atlassian.com/x/JYlKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn delete_workflow_transition_property(
    configuration: &configuration::Configuration,
    params: DeleteWorkflowTransitionPropertyParams,
) -> Result<(), Error<DeleteWorkflowTransitionPropertyError>> {
    // unbox the parameters
    let transition_id = params.transition_id;
    let key = params.key;
    let workflow_name = params.workflow_name;
    let workflow_mode = params.workflow_mode;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/workflow/transitions/{transitionId}/properties",
        configuration.base_path,
        transitionId = transition_id
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    local_var_req_builder =
        local_var_req_builder.query(&[("workflowName", &workflow_name.to_string())]);
    if let Some(ref local_var_str) = workflow_mode {
        local_var_req_builder =
            local_var_req_builder.query(&[("workflowMode", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<DeleteWorkflowTransitionPropertyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the properties on a workflow transition. Transition properties are used to change the behavior of a transition. For more information, see [Transition properties](https://confluence.atlassian.com/x/zIhKLg#Advancedworkflowconfiguration-transitionproperties) and [Workflow properties](https://confluence.atlassian.com/x/JYlKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn get_workflow_transition_properties(
    configuration: &configuration::Configuration,
    params: GetWorkflowTransitionPropertiesParams,
) -> Result<models::WorkflowTransitionProperty, Error<GetWorkflowTransitionPropertiesError>> {
    // unbox the parameters
    let transition_id = params.transition_id;
    let workflow_name = params.workflow_name;
    let include_reserved_keys = params.include_reserved_keys;
    let key = params.key;
    let workflow_mode = params.workflow_mode;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/workflow/transitions/{transitionId}/properties",
        configuration.base_path,
        transitionId = transition_id
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_reserved_keys {
        local_var_req_builder =
            local_var_req_builder.query(&[("includeReservedKeys", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    local_var_req_builder =
        local_var_req_builder.query(&[("workflowName", &workflow_name.to_string())]);
    if let Some(ref local_var_str) = workflow_mode {
        local_var_req_builder =
            local_var_req_builder.query(&[("workflowMode", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetWorkflowTransitionPropertiesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a workflow transition by changing the property value. Trying to update a property that does not exist results in a new property being added to the transition. Transition properties are used to change the behavior of a transition. For more information, see [Transition properties](https://confluence.atlassian.com/x/zIhKLg#Advancedworkflowconfiguration-transitionproperties) and [Workflow properties](https://confluence.atlassian.com/x/JYlKLg).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub async fn update_workflow_transition_property(
    configuration: &configuration::Configuration,
    params: UpdateWorkflowTransitionPropertyParams,
) -> Result<models::WorkflowTransitionProperty, Error<UpdateWorkflowTransitionPropertyError>> {
    // unbox the parameters
    let transition_id = params.transition_id;
    let key = params.key;
    let workflow_name = params.workflow_name;
    let request_body = params.request_body;
    let workflow_mode = params.workflow_mode;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/workflow/transitions/{transitionId}/properties",
        configuration.base_path,
        transitionId = transition_id
    );
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("key", &key.to_string())]);
    local_var_req_builder =
        local_var_req_builder.query(&[("workflowName", &workflow_name.to_string())]);
    if let Some(ref local_var_str) = workflow_mode {
        local_var_req_builder =
            local_var_req_builder.query(&[("workflowMode", &local_var_str.to_string())]);
    }
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
    local_var_req_builder = local_var_req_builder.json(&request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateWorkflowTransitionPropertyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
