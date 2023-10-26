/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.51.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`delete_recording_add_on_result_payload`]
#[derive(Clone, Debug, Default)]
pub struct DeleteRecordingAddOnResultPayloadParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resources to delete.
    pub account_sid: String,
    /// The SID of the recording to which the AddOnResult resource that contains the payloads to delete belongs.
    pub reference_sid: String,
    /// The SID of the AddOnResult to which the payloads to delete belongs.
    pub add_on_result_sid: String,
    /// The Twilio-provided string that uniquely identifies the Recording AddOnResult Payload resource to delete.
    pub sid: String
}

/// struct for passing parameters to the method [`fetch_recording_add_on_result_payload`]
#[derive(Clone, Debug, Default)]
pub struct FetchRecordingAddOnResultPayloadParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resource to fetch.
    pub account_sid: String,
    /// The SID of the recording to which the AddOnResult resource that contains the payload to fetch belongs.
    pub reference_sid: String,
    /// The SID of the AddOnResult to which the payload to fetch belongs.
    pub add_on_result_sid: String,
    /// The Twilio-provided string that uniquely identifies the Recording AddOnResult Payload resource to fetch.
    pub sid: String
}

/// struct for passing parameters to the method [`list_recording_add_on_result_payload`]
#[derive(Clone, Debug, Default)]
pub struct ListRecordingAddOnResultPayloadParams {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resources to read.
    pub account_sid: String,
    /// The SID of the recording to which the AddOnResult resource that contains the payloads to read belongs.
    pub reference_sid: String,
    /// The SID of the AddOnResult to which the payloads to read belongs.
    pub add_on_result_sid: String,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
    /// The page index. This value is simply for client state.
    pub page: Option<i32>,
    /// The page token. This is provided by the API.
    pub page_token: Option<String>
}


/// struct for typed errors of method [`delete_recording_add_on_result_payload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRecordingAddOnResultPayloadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_recording_add_on_result_payload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchRecordingAddOnResultPayloadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_recording_add_on_result_payload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRecordingAddOnResultPayloadError {
    UnknownValue(serde_json::Value),
}


/// Delete a payload from the result along with all associated Data
pub async fn delete_recording_add_on_result_payload(configuration: &configuration::Configuration, params: DeleteRecordingAddOnResultPayloadParams) -> Result<(), Error<DeleteRecordingAddOnResultPayloadError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let reference_sid = params.reference_sid;
    let add_on_result_sid = params.add_on_result_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ReferenceSid=crate::apis::urlencode(reference_sid), AddOnResultSid=crate::apis::urlencode(add_on_result_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteRecordingAddOnResultPayloadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch an instance of a result payload
pub async fn fetch_recording_add_on_result_payload(configuration: &configuration::Configuration, params: FetchRecordingAddOnResultPayloadParams) -> Result<crate::models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayload, Error<FetchRecordingAddOnResultPayloadError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let reference_sid = params.reference_sid;
    let add_on_result_sid = params.add_on_result_sid;
    let sid = params.sid;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads/{Sid}.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ReferenceSid=crate::apis::urlencode(reference_sid), AddOnResultSid=crate::apis::urlencode(add_on_result_sid), Sid=crate::apis::urlencode(sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FetchRecordingAddOnResultPayloadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of payloads belonging to the AddOnResult
pub async fn list_recording_add_on_result_payload(configuration: &configuration::Configuration, params: ListRecordingAddOnResultPayloadParams) -> Result<crate::models::ListRecordingAddOnResultPayloadResponse, Error<ListRecordingAddOnResultPayloadError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_sid = params.account_sid;
    let reference_sid = params.reference_sid;
    let add_on_result_sid = params.add_on_result_sid;
    let page_size = params.page_size;
    let page = params.page;
    let page_token = params.page_token;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads.json", local_var_configuration.base_path, AccountSid=crate::apis::urlencode(account_sid), ReferenceSid=crate::apis::urlencode(reference_sid), AddOnResultSid=crate::apis::urlencode(add_on_result_sid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("Page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("PageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListRecordingAddOnResultPayloadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
