use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

use crate::official::types::projects::ModLoaderType;

/// <https://docs.curseforge.com/#get-games>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamesParams {
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

/// <https://docs.curseforge.com/#get-categories>
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoriesParams {
    pub game_id: i32,
    pub class_id: Option<i32>,
}

impl CategoriesParams {
    /// Instantiate this structure with a `game_id` and no `class_id`.
    pub fn game(game_id: i32) -> Self {
        Self {
            game_id,
            class_id: None,
        }
    }
}

/// <https://docs.curseforge.com/#search-mods>
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSearchParams {
    pub game_id: i32,
    pub class_id: Option<i32>,
    pub category_id: Option<i32>,
    pub game_version: Option<String>,
    pub search_filter: Option<String>,
    pub sort_field: Option<SearchSort>,
    pub sort_order: Option<SearchSortOrder>,
    #[serde(rename = "modLoaderType")]
    pub mod_loader: Option<ModLoaderType>,
    pub game_version_type_id: Option<i32>,
    pub slug: Option<String>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

impl ProjectSearchParams {
    pub fn game(game_id: i32) -> Self {
        Self {
            game_id,
            class_id: None,
            category_id: None,
            game_version: None,
            search_filter: None,
            sort_field: None,
            sort_order: None,
            mod_loader: None,
            game_version_type_id: None,
            slug: None,
            index: None,
            page_size: None,
        }
    }
}

/// <https://docs.curseforge.com/#tocS_ModsSearchSortField>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SearchSort {
    Featured = 1,
    Popularity = 2,
    LastUpdated = 3,
    Name = 4,
    Author = 5,
    TotalDownloads = 6,
    Category = 7,
    GameVersion = 8,
}

/// <https://docs.curseforge.com/#tocS_SortOrder>
#[derive(Clone, Debug, PartialEq, EnumString, Display, SerializeDisplay, DeserializeFromStr)]
pub enum SearchSortOrder {
    #[strum(serialize = "asc")]
    Ascending,
    #[strum(serialize = "desc")]
    Descending,
}

/// <https://docs.curseforge.com/#get-mod-files>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectFilesParams {
    pub game_version: Option<String>,
    #[serde(rename = "modLoaderType")]
    pub mod_loader: Option<ModLoaderType>,
    pub game_version_type_id: Option<i32>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

macro_rules! several_body {
    ($field:literal, $field_type:ty, $iter:expr) => {{
        use serde::Serialize;

        #[derive(Serialize)]
        struct __RequestBody {
            #[serde(rename = $field)]
            __field: Vec<$field_type>,
        }

        __RequestBody {
            __field: $iter.collect(),
        }
    }};
}

pub(crate) use several_body;

/// <https://docs.curseforge.com/#tocS_GetFeaturedModsRequestBody>
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedProjectsBody {
    pub game_id: i32,
    pub excluded_mod_ids: Vec<i32>,
    pub game_version_type_id: Option<i32>,
}

impl FeaturedProjectsBody {
    pub fn game(game_id: i32) -> Self {
        Self {
            game_id,
            excluded_mod_ids: Vec::new(),
            game_version_type_id: None,
        }
    }
}
