/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.51.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodQueue {
    /// The date and time in GMT that this resource was last updated, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The number of calls currently in the queue.
    #[serde(rename = "current_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub current_size: Option<Option<i32>>,
    /// A string that you assigned to describe this resource.
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// The URI of this resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Queue resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    ///  The average wait time in seconds of the members in this queue. This is calculated at the time of the request.
    #[serde(rename = "average_wait_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub average_wait_time: Option<Option<i32>>,
    /// The unique string that that we created to identify this Queue resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The date and time in GMT that this resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    ///  The maximum number of calls that can be in the queue. The default is 1000 and the maximum is 5000.
    #[serde(rename = "max_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_size: Option<Option<i32>>,
}

impl ApiPeriodV2010PeriodAccountPeriodQueue {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodQueue {
        ApiPeriodV2010PeriodAccountPeriodQueue {
            date_updated: None,
            current_size: None,
            friendly_name: None,
            uri: None,
            account_sid: None,
            average_wait_time: None,
            sid: None,
            date_created: None,
            max_size: None,
        }
    }
}


