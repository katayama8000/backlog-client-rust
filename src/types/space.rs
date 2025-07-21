use serde::{Deserialize, Serialize};

/// https://developer.nulab.com/docs/backlog/api/2/get-space/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    #[serde(rename = "spaceKey")]
    pub space_key: String,
    pub name: String,
    #[serde(rename = "ownerId")]
    pub owner_id: u32,
    pub lang: String,
    pub timezone: String,
    #[serde(rename = "reportSendTime")]
    pub report_send_time: String,
    #[serde(rename = "textFormattingRule")]
    pub text_formatting_rule: String,
    pub created: String,
    pub updated: String,
}
