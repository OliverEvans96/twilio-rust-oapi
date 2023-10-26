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
pub struct ApiPeriodV2010PeriodAccountPeriodShortCode {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this ShortCode resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The API version used to start a new TwiML session when an SMS message is sent to this short code.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// The date and time in GMT that this resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that this resource was last updated, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// A string that you assigned to describe this resource. By default, the `FriendlyName` is the short code.
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// The short code. e.g., 894546.
    #[serde(rename = "short_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub short_code: Option<Option<String>>,
    /// The unique string that that we created to identify this ShortCode resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The HTTP method we use to call the `sms_fallback_url`. Can be: `GET` or `POST`.
    #[serde(rename = "sms_fallback_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_fallback_method: Option<Option<SmsFallbackMethod>>,
    /// The URL that we call if an error occurs while retrieving or executing the TwiML from `sms_url`.
    #[serde(rename = "sms_fallback_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_fallback_url: Option<Option<String>>,
    /// The HTTP method we use to call the `sms_url`. Can be: `GET` or `POST`.
    #[serde(rename = "sms_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_method: Option<Option<SmsMethod>>,
    /// The URL we call when receiving an incoming SMS message to this short code.
    #[serde(rename = "sms_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_url: Option<Option<String>>,
    /// The URI of this resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodShortCode {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodShortCode {
        ApiPeriodV2010PeriodAccountPeriodShortCode {
            account_sid: None,
            api_version: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            short_code: None,
            sid: None,
            sms_fallback_method: None,
            sms_fallback_url: None,
            sms_method: None,
            sms_url: None,
            uri: None,
        }
    }
}

/// The HTTP method we use to call the `sms_fallback_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsFallbackMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for SmsFallbackMethod {
    fn default() -> SmsFallbackMethod {
        Self::Head
    }
}
/// The HTTP method we use to call the `sms_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for SmsMethod {
    fn default() -> SmsMethod {
        Self::Head
    }
}

