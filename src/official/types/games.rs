use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::core::{CoreApiStatus, CoreStatus};
use super::fixes::nullable_string;

/// <https://docs.curseforge.com/#tocS_Game>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub date_modified: DateTime<Utc>,
    pub assets: GameAssets,
    pub status: CoreStatus,
    pub api_status: CoreApiStatus,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_GameAssets>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct GameAssets {
    #[serde(deserialize_with = "nullable_string")]
    pub icon_url: Option<String>,
    #[serde(deserialize_with = "nullable_string")]
    pub tile_url: Option<String>,
    #[serde(deserialize_with = "nullable_string")]
    pub cover_url: Option<String>,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_GameVersionsByType>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct GameVersions {
    pub r#type: i32,
    pub versions: Vec<String>,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}

/// <https://docs.curseforge.com/#tocS_GameVersionType>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(feature = "unknown-fields"), serde(deny_unknown_fields))]
pub struct GameVersionType {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    #[cfg(feature = "unknown-fields")]
    #[serde(flatten)]
    pub other_fields: serde_json::Value,
}
