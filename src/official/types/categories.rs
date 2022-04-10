use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::fixes::nullable_datetime;

/// <https://docs.curseforge.com/#tocS_Category>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Category {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: Option<String>,
    pub url: Option<String>,
    pub icon_url: String,
    #[serde(deserialize_with = "nullable_datetime")]
    pub date_modified: Option<DateTime<Utc>>,
    pub is_class: Option<bool>,
    #[serde(default)]
    pub class_id: Option<i32>,
    pub parent_category_id: Option<i32>,
}
