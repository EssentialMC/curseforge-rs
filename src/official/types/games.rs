use chrono::{DateTime, Utc};
use query_string::QueryString;
use serde::{Deserialize, Serialize};

use super::core::{CoreApiStatus, CoreStatus, Pagination};
use super::nullable_str;

/// <https://docs.curseforge.com/#get-games>
#[derive(Clone, Debug, Default, PartialEq, Serialize, QueryString)]
#[serde(rename_all = "camelCase")]
pub struct GamesParams {
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

/// <https://docs.curseforge.com/#tocS_Get%20Games%20Response>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GamesResponse {
    pub data: Vec<Game>,
    pub pagination: Pagination,
}

/// <https://docs.curseforge.com/#tocS_Get%20Game%20Response>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GameResponse {
    pub data: Game,
}

/// <https://docs.curseforge.com/#tocS_Game>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub date_modified: DateTime<Utc>,
    pub assets: GameAssets,
    pub status: CoreStatus,
    pub api_status: CoreApiStatus,
}

/// <https://docs.curseforge.com/#tocS_GameAssets>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GameAssets {
    #[serde(deserialize_with = "nullable_str")]
    pub icon_url: Option<String>,
    #[serde(deserialize_with = "nullable_str")]
    pub tile_url: Option<String>,
    #[serde(deserialize_with = "nullable_str")]
    pub cover_url: Option<String>,
}

/// <https://docs.curseforge.com/#tocS_Get%20Versions%20Response>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GameVersionsResponse {
    pub data: Vec<GameVersions>,
}

/// <https://docs.curseforge.com/#tocS_GameVersionsByType>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GameVersions {
    #[serde(rename = "type")]
    pub kind: i32,
    pub versions: Vec<String>,
}

/// <https://docs.curseforge.com/#tocS_Get%20Version%20Types%20Response>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GameVersionTypesResponse {
    pub data: Vec<GameVersionType>,
}

/// <https://docs.curseforge.com/#tocS_GameVersionType>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GameVersionType {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
}
