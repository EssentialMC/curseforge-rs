use chrono::{DateTime, Utc};
use query_string::QueryString;
use serde::{Deserialize, Deserializer, Serialize};

use super::core::{CoreApiStatus, CoreStatus, Pagination};

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

fn nullable_str<'de, D: Deserializer<'de>>(deser: D) -> Result<Option<String>, D::Error> {
    let maybe: Option<String> = Option::deserialize(deser)?;
    Ok(maybe.filter(|string| !string.is_empty()))
}
