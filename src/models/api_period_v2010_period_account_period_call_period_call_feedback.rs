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
pub struct ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback {
    /// The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The date that this resource was created, given in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date that this resource was last updated, given in [RFC 2822](https://www.php.net/manual/en/class.datetime.php#datetime.constants.rfc2822) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// A list of issues experienced during the call. The issues can be: `imperfect-audio`, `dropped-call`, `incorrect-caller-id`, `post-dial-delay`, `digits-not-captured`, `audio-latency`, `unsolicited-call`, or `one-way-audio`.
    #[serde(rename = "issues", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Option<Vec<crate::models::CallFeedbackEnumIssues>>>,
    /// `1` to `5` quality score where `1` represents imperfect experience and `5` represents a perfect call.
    #[serde(rename = "quality_score", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub quality_score: Option<Option<i32>>,
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback {
        ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback {
            account_sid: None,
            date_created: None,
            date_updated: None,
            issues: None,
            quality_score: None,
            sid: None,
        }
    }
}


