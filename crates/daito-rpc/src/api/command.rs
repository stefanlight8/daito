use daito_api::activity::Activity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandKind {
    Dispatch,
    SetActivity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CommandArgs {
    SetActivity {
        pid: u32,
        activity: Option<Activity>,
    },
}
