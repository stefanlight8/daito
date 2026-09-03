use {
    serde::{Deserialize, Serialize},
    serde_repr::{Deserialize_repr, Serialize_repr},
};

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone)]
#[repr(u8)]
pub enum ActivityType {
    #[default]
    Playing = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone)]
#[repr(u8)]
pub enum StatusDisplayType {
    #[default]
    Name = 0,
    State = 1,
    Details = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityTimestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct ActivityAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Activity {
    pub name: String,
    pub activity_type: ActivityType,
    pub created_at: i64,
    pub status_display_type: StatusDisplayType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<ActivityButton>>,
}
