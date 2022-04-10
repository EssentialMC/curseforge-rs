use chrono::{DateTime, Utc};
use query_string::QueryString;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

use super::categories::Category;
use super::core::Pagination;
use super::file::{File, FileIndex};
use super::nullable_string;

/// <https://docs.curseforge.com/#search-mods>
#[derive(Clone, Debug, PartialEq, Serialize, QueryString)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    pub game_id: i32,
    pub class_id: Option<i32>,
    pub category_id: Option<i32>,
    pub game_version: Option<String>,
    pub search_filter: Option<String>,
    pub sort_field: Option<SearchSort>,
    pub sort_order: Option<SearchSortOrder>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub game_version_type_id: Option<i32>,
    pub slug: Option<String>,
    pub index: Option<i32>,
    pub page_size: Option<i32>,
}

impl SearchParams {
    pub fn game(game_id: i32) -> Self {
        Self {
            game_id,
            class_id: None,
            category_id: None,
            game_version: None,
            search_filter: None,
            sort_field: None,
            sort_order: None,
            mod_loader_type: None,
            game_version_type_id: None,
            slug: None,
            index: None,
            page_size: None,
        }
    }
}

/// <https://docs.curseforge.com/#search-mods>
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

/// <https://docs.curseforge.com/#tocS_ModLoaderType>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
}

/// <https://docs.curseforge.com/#tocS_Search%20Mods%20Response>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SearchProjectsResponse {
    pub data: Vec<Project>,
    pub pagination: Pagination,
}

/// <https://docs.curseforge.com/#tocS_Mod>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub links: ProjectLinks,
    pub summary: String,
    pub status: ProjectStatus,
    pub download_count: f64,
    pub is_featured: bool,
    pub primary_category_id: u32,
    pub categories: Vec<Category>,
    pub class_id: Option<i32>,
    pub authors: Vec<ProjectAuthor>,
    pub logo: Option<ProjectAsset>,
    pub screenshots: Vec<ProjectAsset>,
    pub main_file_id: i32,
    pub latest_files: Vec<File>,
    pub latest_files_indexes: Vec<FileIndex>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub date_released: DateTime<Utc>,
    pub allow_mod_distribution: Option<bool>,
    pub game_popularity_rank: i32,
    pub is_available: bool,
}

/// <https://docs.curseforge.com/#tocS_ModLinks>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ProjectLinks {
    pub website_url: String,
    #[serde(deserialize_with = "nullable_string")]
    pub wiki_url: Option<String>,
    #[serde(deserialize_with = "nullable_string")]
    pub issues_url: Option<String>,
    #[serde(deserialize_with = "nullable_string")]
    pub source_url: Option<String>,
}

/// <https://docs.curseforge.com/#tocS_ModLinks>
#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ProjectStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
}

/// <https://docs.curseforge.com/#tocS_ModAuthor>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ProjectAuthor {
    pub id: i32,
    pub name: String,
    pub url: String,
}

/// <https://docs.curseforge.com/#tocS_ModAsset>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ProjectAsset {
    pub id: i32,
    pub mod_id: i32,
    pub title: String,
    #[serde(deserialize_with = "nullable_string")]
    pub description: Option<String>,
    pub thumbnail_url: String,
    pub url: String,
}
