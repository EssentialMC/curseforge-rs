use chrono::{DateTime, Utc};
use query_string::QueryString;
use serde::{Deserialize, Serialize};

/// <https://docs.curseforge.com/#get-categories>
#[derive(Clone, Debug, PartialEq, Serialize, QueryString)]
#[serde(rename_all = "camelCase")]
pub struct CategoriesParams {
    pub game_id: i32,
    pub class_id: Option<i32>,
}

impl CategoriesParams {
    pub fn game(game_id: i32) -> Self {
        Self {
            game_id,
            class_id: None,
        }
    }
}

/// <https://docs.curseforge.com/#tocS_Get%20Categories%20Response>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CategoriesResponse {
    pub data: Vec<Category>,
}

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
    pub date_modified: DateTime<Utc>,
    pub is_class: Option<bool>,
    #[serde(default)]
    pub class_id: Option<i32>,
    pub parent_category_id: Option<i32>,
}
