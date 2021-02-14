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

/// struct for passing parameters to the method `get_field_auto_complete_for_query_string`
#[derive(Clone, Debug)]
pub struct GetFieldAutoCompleteForQueryStringParams {
    /// The name of the field.
    pub field_name: Option<String>,
    /// The partial field item name entered by the user.
    pub field_value: Option<String>,
    /// The name of the [ CHANGED operator predicate](https://confluence.atlassian.com/x/hQORLQ#Advancedsearching-operatorsreference-CHANGEDCHANGED) for which the suggestions are generated. The valid predicate operators are *by*, *from*, and *to*.
    pub predicate_name: Option<String>,
    /// The partial predicate item name entered by the user.
    pub predicate_value: Option<String>,
}

/// struct for passing parameters to the method `migrate_queries`
#[derive(Clone, Debug)]
pub struct MigrateQueriesParams {
    pub jql_personal_data_migration_request: models::JqlPersonalDataMigrationRequest,
}

/// struct for passing parameters to the method `parse_jql_queries`
#[derive(Clone, Debug)]
pub struct ParseJqlQueriesParams {
    pub jql_queries_to_parse: models::JqlQueriesToParse,
    /// How to validate the JQL query and treat the validation results. Validation options include:   *  `strict` Returns all errors. If validation fails, the query structure is not returned.  *  `warn` Returns all errors. If validation fails but the JQL query is correctly formed, the query structure is returned.  *  `none` No validation is performed. If JQL query is correctly formed, the query structure is returned.
    pub validation: Option<String>,
}

/// struct for typed errors of method `get_auto_complete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAutoCompleteError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_field_auto_complete_for_query_string`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFieldAutoCompleteForQueryStringError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `migrate_queries`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MigrateQueriesError {
    Status400(),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `parse_jql_queries`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParseJqlQueriesError {
    Status400(models::ErrorCollection),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// Returns reference data for JQL searches. This is a downloadable version of the documentation provided in [Advanced searching - fields reference](https://confluence.atlassian.com/x/gwORLQ) and [Advanced searching - functions reference](https://confluence.atlassian.com/x/hgORLQ), along with a list of JQL-reserved words. Use this information to assist with the programmatic creation of JQL queries or the validation of queries built in a custom query builder.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.
pub async fn get_auto_complete(
    configuration: &configuration::Configuration,
) -> Result<models::JqlReferenceData, Error<GetAutoCompleteError>> {
    // unbox the parameters

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/jql/autocompletedata",
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
        let local_var_entity: Option<GetAutoCompleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the JQL search auto complete suggestions for a field.  Suggestions can be obtained by providing:   *  `fieldName` to get a list of all values for the field.  *  `fieldName` and `fieldValue` to get a list of values containing the text in `fieldValue`.  *  `fieldName` and `predicateName` to get a list of all predicate values for the field.  *  `fieldName`, `predicateName`, and `predicateValue` to get a list of predicate values containing the text in `predicateValue`.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.
pub async fn get_field_auto_complete_for_query_string(
    configuration: &configuration::Configuration,
    params: GetFieldAutoCompleteForQueryStringParams,
) -> Result<models::AutoCompleteSuggestions, Error<GetFieldAutoCompleteForQueryStringError>> {
    // unbox the parameters
    let field_name = params.field_name;
    let field_value = params.field_value;
    let predicate_name = params.predicate_name;
    let predicate_value = params.predicate_value;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/rest/api/3/jql/autocompletedata/suggestions",
        configuration.base_path
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = field_name {
        local_var_req_builder =
            local_var_req_builder.query(&[("fieldName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_value {
        local_var_req_builder =
            local_var_req_builder.query(&[("fieldValue", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = predicate_name {
        local_var_req_builder =
            local_var_req_builder.query(&[("predicateName", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = predicate_value {
        local_var_req_builder =
            local_var_req_builder.query(&[("predicateValue", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetFieldAutoCompleteForQueryStringError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Converts one or more JQL queries with user identifiers (username or user key) to equivalent JQL queries with account IDs.  You may wish to use this operation if your system stores JQL queries and you want to make them GDPR-compliant. For more information about GDPR-related changes, see the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/).  **[Permissions](#permissions) required:** Permission to access Jira.
pub async fn migrate_queries(
    configuration: &configuration::Configuration,
    params: MigrateQueriesParams,
) -> Result<models::ConvertedJqlQueries, Error<MigrateQueriesError>> {
    // unbox the parameters
    let jql_personal_data_migration_request = params.jql_personal_data_migration_request;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/jql/pdcleaner", configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&jql_personal_data_migration_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MigrateQueriesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Parses and validates JQL queries.  Validation is performed in context of the current user.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.
pub async fn parse_jql_queries(
    configuration: &configuration::Configuration,
    params: ParseJqlQueriesParams,
) -> Result<models::ParsedJqlQueries, Error<ParseJqlQueriesError>> {
    // unbox the parameters
    let jql_queries_to_parse = params.jql_queries_to_parse;
    let validation = params.validation;

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rest/api/3/jql/parse", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = validation {
        local_var_req_builder =
            local_var_req_builder.query(&[("validation", &local_var_str.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&jql_queries_to_parse);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ParseJqlQueriesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
